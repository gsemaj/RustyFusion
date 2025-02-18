use std::{sync::LazyLock, time::Duration};

use postgres::{tls, types::ToSql, GenericClient, Row};
use regex::Regex;

use crate::{
    database::*,
    defines::*,
    entity::{BuddyListEntry, Combatant, Entity, PlayerFlags, PlayerStyle},
    enums::PlayerGuide,
    item::Item,
    mission::Task,
    nano::Nano,
    net::packet::*,
    tabledata::tdata_get,
    util::{self, Bitfield},
    Position,
};

impl FFError {
    fn from_db_err(e: postgres::Error) -> Self {
        FFError::build(Severity::Warning, format!("Database error: {}", e))
    }
}

pub struct PostgresDatabase {
    client: postgres::Client,
    config: postgres::Config,
}
impl std::fmt::Debug for PostgresDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Postgres Database ({:?})", self.config)
    }
}
impl PostgresDatabase {
    pub fn connect(config: &GeneralConfig) -> FFResult<Box<dyn Database>> {
        let mut db_config = postgres::Client::configure();
        db_config
            .host(&config.db_host.get())
            .port(config.db_port.get())
            .user(&config.db_username.get())
            .password(config.db_password.get())
            .dbname(DB_NAME)
            .connect_timeout(Duration::from_secs(5));
        let mut db_client = db_config
            .connect(tls::NoTls)
            .map_err(FFError::from_db_err)?;

        let meta_table_exists: bool =
            Self::query(&mut db_client, "meta_table_exists", &[])?[0].get(0);
        if !meta_table_exists {
            log(
                Severity::Info,
                "Meta table missing; initializing database...",
            );
            Self::exec(
                &mut db_client,
                "create_tables",
                &[&PROTOCOL_VERSION, &DB_VERSION],
            )?;
        }

        Ok(Box::new(Self {
            client: db_client,
            config: db_config,
        }))
    }

    fn read_sql(name: &str) -> String {
        let path = format!("sql/{}.sql", name);
        match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                panic_log(&format!("Couldn't read SQL file {}: {}", path, e));
            }
        }
    }

    fn query(
        client: &mut impl GenericClient,
        name: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> FFResult<Vec<Row>> {
        let query = Self::read_sql(name);
        client.query(&query, params).map_err(FFError::from_db_err)
    }

    fn prep(client: &mut impl GenericClient, name: &str) -> FFResult<postgres::Statement> {
        let query = Self::read_sql(name);
        client.prepare(&query).map_err(FFError::from_db_err)
    }

    fn exec(
        client: &mut impl GenericClient,
        name: &str,
        mut params: &[&(dyn ToSql + Sync)],
    ) -> FFResult<u64> {
        static SQL_PARAMETER_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\$[0-9]+").unwrap());
        let calc_num_params = |s: &str| {
            // we can use the parameter with the highest number to determine the number of parameters
            let max_param = SQL_PARAMETER_REGEX
                .find_iter(s)
                .map(|m| m.as_str()[1..].parse::<usize>().unwrap())
                .max();
            max_param.unwrap_or(0)
        };

        let queries = Self::read_sql(name);
        let queries: Vec<&str> = queries.split(';').collect();

        // implicit transaction
        let mut tsct = client.transaction().map_err(FFError::from_db_err)?;

        let mut num_updated = 0;
        for query in queries {
            if query.trim().is_empty() {
                continue;
            }
            //println!("{}", query);
            let num_params = calc_num_params(query);
            match tsct.execute(query, &params[..num_params]) {
                Ok(r) => {
                    num_updated += r;
                    params = &params[num_params..];
                }
                Err(e) => {
                    panic_log(&format!("DB error: {}", e));
                }
            };
        }

        tsct.commit().map_err(FFError::from_db_err)?;
        Ok(num_updated)
    }

    fn save_player_internal(client: &mut impl GenericClient, player: &Player) -> FFResult<()> {
        let mut tsct = client.transaction().map_err(FFError::from_db_err)?;
        let client = &mut tsct;
        let save_item = Self::prep(client, "save_item")?;
        let save_quest_item = Self::prep(client, "save_quest_item")?;
        let save_nano = Self::prep(client, "save_nano")?;
        let save_running_quest = Self::prep(client, "save_running_quest")?;
        let pc_uid = player.get_uid();

        let position = if player.instance_id.instance_num.is_some() {
            player.get_pre_warp().position
        } else {
            player.get_position()
        };

        Self::exec(
            client,
            "save_player",
            &[
                &pc_uid,
                &(player.get_level() as Int),
                &(player.get_equipped_nano_ids()[0] as Int),
                &(player.get_equipped_nano_ids()[1] as Int),
                &(player.get_equipped_nano_ids()[2] as Int),
                &(player.flags.tutorial_flag as Int),
                &(player.flags.payzone_flag as Int),
                &position.x,
                &position.y,
                &position.z,
                &player.get_rotation(),
                &player.get_hp(),
                &(player.get_fusion_matter() as Int),
                &(player.get_taros() as Int),
                &(player.get_weapon_boosts() as Int),
                &(player.get_nano_potions() as Int),
                &((player.get_guide() as i16) as Int),
                &player.mission_journal.get_active_mission_id().unwrap_or(0),
                &player.flags.scamper_flags.get_chunk(0).unwrap(),
                &player.flags.skyway_flags.to_bytes().as_slice(),
                &player.flags.tip_flags.to_bytes().as_slice(),
                &player
                    .mission_journal
                    .completed_mission_flags
                    .to_bytes()
                    .as_slice(),
            ],
        )?;

        Self::exec(client, "clear_nanos", &[&pc_uid])?;
        for nano in player.get_nano_iter() {
            let nano_raw: sNano = Some(nano.clone()).into();
            client
                .execute(
                    &save_nano,
                    &[
                        &pc_uid,
                        &(nano_raw.iID as Int),
                        &(nano_raw.iSkillID as Int),
                        &(nano_raw.iStamina as Int),
                    ],
                )
                .map_err(FFError::from_db_err)?;
        }

        Self::exec(client, "clear_items", &[&pc_uid])?;
        for (slot_num, item) in player.get_item_iter() {
            let item_raw: sItemBase = Some(*item).into();
            client
                .execute(
                    &save_item,
                    &[
                        &pc_uid,
                        &(slot_num as Int),
                        &(item_raw.iID as Int),
                        &(item_raw.iType as Int),
                        &item_raw.iOpt,
                        &item_raw.iTimeLimit,
                    ],
                )
                .map_err(FFError::from_db_err)?;
        }

        Self::exec(client, "clear_quest_items", &[&pc_uid])?;
        for (virtual_slot, (item_id, count)) in player.get_quest_item_iter().enumerate() {
            client
                .execute(
                    &save_quest_item,
                    &[
                        &pc_uid,
                        &(item_id as Int),
                        &(count as Int),
                        &(virtual_slot as Int),
                    ],
                )
                .map_err(FFError::from_db_err)?;
        }

        Self::exec(client, "clear_running_quests", &[&pc_uid])?;
        for task in player.mission_journal.get_running_quests() {
            if task.m_aCurrTaskID == 0 {
                continue;
            }

            client
                .execute(
                    &save_running_quest,
                    &[
                        &pc_uid,
                        &(task.m_aCurrTaskID as Int),
                        &(task.m_aKillNPCCount[0] as Int),
                        &(task.m_aKillNPCCount[1] as Int),
                        &(task.m_aKillNPCCount[2] as Int),
                    ],
                )
                .map_err(FFError::from_db_err)?;
        }

        Self::exec(client, "clear_buddies", &[&pc_uid])?;
        for buddy_uid in player.get_buddy_uids() {
            Self::exec(client, "save_buddy", &[&pc_uid, &buddy_uid])?;
        }

        Self::exec(client, "clear_blocks", &[&pc_uid])?;
        for blocked_uid in player.get_blocked_uids() {
            Self::exec(client, "save_block", &[&pc_uid, &blocked_uid])?;
        }

        tsct.commit().map_err(FFError::from_db_err)?;
        Ok(())
    }

    fn load_player_internal(
        client: &mut impl GenericClient,
        row: &Row,
        load_buddies: bool,
    ) -> FFResult<Player> {
        let pc_uid = row.get("PlayerId");
        let slot_num: Int = row.get("Slot");
        let mut player = Player::new(pc_uid, slot_num as usize);
        let appearance_flag: Int = row.get("AppearanceFlag");
        player.style = if appearance_flag != 0 {
            Some(PlayerStyle {
                gender: row.get::<_, Int>("Gender") as i8,
                face_style: row.get::<_, Int>("FaceStyle") as i8,
                hair_style: row.get::<_, Int>("HairStyle") as i8,
                hair_color: row.get::<_, Int>("HairColor") as i8,
                skin_color: row.get::<_, Int>("SkinColor") as i8,
                eye_color: row.get::<_, Int>("EyeColor") as i8,
                height: row.get::<_, Int>("Height") as i8,
                body: row.get::<_, Int>("Body") as i8,
            })
        } else {
            None
        };

        let first_name: String = row.get("FirstName");
        let last_name: String = row.get("LastName");
        player.first_name = first_name;
        player.last_name = last_name;

        player.set_position(Position {
            x: row.get("XCoordinate"),
            y: row.get("YCoordinate"),
            z: row.get("ZCoordinate"),
        });
        player.set_rotation(row.get("Angle"));

        player.set_taros(row.get::<_, Int>("Taros") as u32);
        player.set_level(row.get::<_, Int>("Level") as i16)?;
        // fusion matter must be set after level
        player.set_fusion_matter(row.get::<_, Int>("FusionMatter") as u32, None);
        player.set_hp(row.get("HP"));
        player.set_weapon_boosts(row.get::<_, Int>("BatteryW") as u32);
        player.set_nano_potions(row.get::<_, Int>("BatteryN") as u32);

        let nano_col_names = ["Nano1", "Nano2", "Nano3"];
        for (slot, col_name) in nano_col_names.iter().enumerate() {
            let nano_id = row.get::<_, Int>(col_name) as i16;
            player.change_nano(slot, if nano_id == 0 { None } else { Some(nano_id) })?;
        }
        let nanos = Self::query(client, "load_nanos", &[&pc_uid])?;
        for nano in nanos {
            let nano_raw = sNano {
                iID: nano.get::<_, Int>("ID") as i16,
                iSkillID: nano.get::<_, Int>("Skill") as i16,
                iStamina: nano.get::<_, Int>("Stamina") as i16,
            };
            let nano: Option<Nano> = nano_raw.into();
            if let Some(nano) = nano {
                player.set_nano(nano);
            }
        }

        let mut player_flags = PlayerFlags::default();
        let first_use_bytes: &[u8] = row.get("FirstUseFlag");
        player_flags.tip_flags = Bitfield::from_bytes(first_use_bytes, SIZEOF_TIP_FLAGS)?;
        player_flags.tutorial_flag = row.get::<_, Int>("TutorialFlag") != 0;
        player_flags.name_check = (row.get::<_, Int>("NameCheck") as i8).try_into()?;
        player.flags = player_flags;

        let guide: PlayerGuide = (row.get::<_, Int>("Mentor") as i16).try_into()?;
        // TODO get total number of guides from DB (currently not stored)
        if guide != PlayerGuide::Computress {
            player.update_guide(guide);
        }

        let skyway_bytes: &[u8] = row.get("SkywayLocationFlag");
        player.flags.skyway_flags =
            Bitfield::from_bytes(skyway_bytes, WYVERN_LOCATION_FLAG_SIZE as usize)?;

        player
            .flags
            .scamper_flags
            .set_chunk(0, row.get("WarpLocationFlag"))
            .unwrap();

        let quest_bytes: &[u8] = row.get("Quests");
        player.mission_journal.completed_mission_flags =
            Bitfield::from_bytes(quest_bytes, SIZEOF_QUESTFLAG_NUMBER as usize)?;

        let running_quests = Self::query(client, "load_running_quests", &[&pc_uid])?;
        for quest in running_quests {
            let task_id: Int = quest.get("TaskID");
            let task_def = tdata_get().get_task_definition(task_id)?;
            let npc_count_1: Int = quest.get("RemainingNPCCount1");
            let npc_count_2: Int = quest.get("RemainingNPCCount2");
            let npc_count_3: Int = quest.get("RemainingNPCCount3");
            let mut task: Task = task_def.into();
            task.fail_time = None;
            task.set_remaining_enemy_defeats([
                npc_count_1 as usize,
                npc_count_2 as usize,
                npc_count_3 as usize,
            ]);
            player.mission_journal.start_task(task)?;
        }

        let active_mission_id: Int = row.get("CurrentMissionID");
        if active_mission_id != 0 {
            log_if_failed(
                player
                    .mission_journal
                    .set_active_mission_id(row.get("CurrentMissionID")),
            );
        }

        let items = Self::query(client, "load_items", &[&pc_uid])?;
        for item in items {
            let slot_num = item.get::<_, Int>("Slot") as usize;
            let item_raw = sItemBase {
                iType: item.get::<_, Int>("Type") as i16,
                iID: item.get::<_, Int>("ID") as i16,
                iOpt: item.get::<_, Int>("Opt"),
                iTimeLimit: item.get::<_, Int>("TimeLimit"),
            };

            let item: Option<Item> = item_raw.try_into()?;
            if item.is_some_and(|item| {
                item.get_expiry_time()
                    .is_some_and(|et| et < SystemTime::now())
            }) {
                // item is expired; skip it
                continue;
            }

            let (loc, slot_num) = util::slot_num_to_loc_and_slot_num(slot_num)?;
            player.set_item(loc, slot_num, item)?;
        }

        let quest_items = Self::query(client, "load_quest_items", &[&pc_uid])?;
        for quest_item in quest_items {
            let item_id: Int = quest_item.get("ID");
            let count: Int = quest_item.get("Opt");
            player.set_quest_item_count(item_id as i16, count as usize)?;
        }

        if load_buddies {
            log_if_failed(Self::load_buddies(client, &mut player));
            log_if_failed(Self::load_blocks(client, &mut player));
        }

        let perms: Int = row.get("AccountLevel");
        player.perms = perms as i16;

        Ok(player)
    }

    fn load_buddies(client: &mut impl GenericClient, player: &mut Player) -> FFResult<()> {
        let rows = Self::query(client, "load_buddy_ids", &[&player.get_uid()])?;
        for row in rows {
            let buddy_uid: BigInt = row.get("PlayerBId");
            let buddy_load_result = Self::query(client, "load_player", &[&buddy_uid]);
            match buddy_load_result {
                Ok(buddy_rows) => {
                    if let Some(buddy_row) = buddy_rows.first() {
                        let buddy = Self::load_player_internal(client, buddy_row, false)?;
                        let buddy_info = BuddyListEntry::new(&buddy);
                        log_if_failed(player.add_buddy(buddy_info));
                    } else {
                        log(
                            Severity::Warning,
                            &format!("Buddy with UID {} not found", buddy_uid),
                        );
                    }
                }
                Err(e) => {
                    log(
                        Severity::Warning,
                        &format!(
                            "Failed to load buddy with UID {}: {}",
                            buddy_uid,
                            e.get_msg()
                        ),
                    );
                }
            }
        }
        Ok(())
    }

    fn load_blocks(client: &mut impl GenericClient, player: &mut Player) -> FFResult<()> {
        let rows = Self::query(client, "load_blocked_ids", &[&player.get_uid()])?;
        for row in rows {
            let blocked_uid: BigInt = row.get("BlockedPlayerId");
            log_if_failed(player.block_player(blocked_uid));
        }
        Ok(())
    }
}
impl Database for PostgresDatabase {
    fn init_player(&mut self, acc_id: BigInt, player: &Player) -> FFResult<()> {
        let client = &mut self.client;
        let updated = Self::exec(
            client,
            "init_player",
            &[
                &player.get_uid(),
                &acc_id,
                &player.first_name,
                &player.last_name,
                &(player.get_style().iNameCheck as Int),
                &(player.get_slot_num() as Int),
                &player.get_position().x,
                &player.get_position().y,
                &player.get_position().z,
                &player.get_rotation(),
                &player.get_hp(),
                &player.flags.skyway_flags.to_bytes().as_slice(),
                &player.flags.tip_flags.to_bytes().as_slice(),
                &player
                    .mission_journal
                    .completed_mission_flags
                    .to_bytes()
                    .as_slice(),
                //
                &player.get_uid(),
            ],
        )?;
        assert_eq!(updated, 1 + 1);
        Ok(())
    }

    fn update_player_appearance(&mut self, player: &Player) -> FFResult<()> {
        let client = &mut self.client;
        let style = player.style.unwrap_or_default();
        let apperance_flag: Int = if player.style.is_some() { 1 } else { 0 };
        let updated = Self::exec(
            client,
            "update_appearance",
            &[
                &player.get_uid(),
                &(style.body as Int),
                &(style.eye_color as Int),
                &(style.face_style as Int),
                &(style.gender as Int),
                &(style.hair_color as Int),
                &(style.hair_style as Int),
                &(style.height as Int),
                &(style.skin_color as Int),
                //
                &player.get_uid(),
                &apperance_flag,
            ],
        )?;
        assert_eq!(updated, 1 + 1);
        Ok(())
    }

    fn find_account_from_username(&mut self, username: &Text) -> FFResult<Option<Account>> {
        let client = &mut self.client;
        let rows = Self::query(client, "find_account", &[username])?;
        assert!(rows.len() <= 1);
        Ok(rows.first().map(|row| Account {
            id: row.get("AccountId"),
            username: username.clone(),
            password_hashed: row.get("Password"),
            selected_slot: row.get::<_, Int>("Selected") as u8,
            account_level: row.get::<_, Int>("AccountLevel") as i16,
            banned_until: util::get_systime_from_sec(row.get::<_, Int>("BannedUntil") as u64),
            ban_reason: row.get("BanReason"),
        }))
    }

    fn find_account_from_player(&mut self, pc_uid: BigInt) -> FFResult<Account> {
        let client = &mut self.client;
        let rows = Self::query(client, "find_account_from_player", &[&pc_uid])?;
        if rows.is_empty() {
            return Err(FFError::build(
                Severity::Warning,
                format!("Account not found for player with UID {}", pc_uid),
            ));
        }
        let row = &rows[0];
        Ok(Account {
            id: row.get("AccountId"),
            username: row.get("Login"),
            password_hashed: row.get("Password"),
            selected_slot: row.get::<_, Int>("Selected") as u8,
            account_level: row.get::<_, Int>("AccountLevel") as i16,
            banned_until: util::get_systime_from_sec(row.get::<_, Int>("BannedUntil") as u64),
            ban_reason: row.get("BanReason"),
        })
    }

    fn create_account(&mut self, username: &Text, password_hashed: &Text) -> FFResult<Account> {
        let client = &mut self.client;

        let acc_level =
            if Self::query(client, "enum_account_ids", &[]).is_ok_and(|rows| rows.is_empty()) {
                CN_ACCOUNT_LEVEL__MASTER
            } else {
                config_get().login.default_account_level.get()
            } as Int;

        let updated = Self::exec(
            client,
            "create_account",
            &[username, password_hashed, &acc_level],
        )?;
        assert_eq!(updated, 1);
        let new_acc = self.find_account_from_username(username)?.unwrap();
        Ok(new_acc)
    }

    fn change_account_level(&mut self, acc_id: BigInt, new_level: Int) -> FFResult<()> {
        let client = &mut self.client;
        let updated = Self::exec(client, "change_account_level", &[&acc_id, &new_level])?;
        if updated == 0 {
            return Err(FFError::build(
                Severity::Warning,
                format!(
                    "Failed to change account level for account with ID {}",
                    acc_id
                ),
            ));
        }
        Ok(())
    }

    fn ban_account(
        &mut self,
        acc_id: BigInt,
        banned_until: SystemTime,
        ban_reason: Text,
    ) -> FFResult<()> {
        let client = &mut self.client;
        let banned_since = util::get_timestamp_sec(SystemTime::now()) as Int;
        let banned_until = util::get_timestamp_sec(banned_until) as Int;
        let updated = Self::exec(
            client,
            "ban_account",
            &[&acc_id, &banned_since, &banned_until, &ban_reason],
        )?;
        if updated == 0 {
            return Err(FFError::build(
                Severity::Warning,
                format!("Failed to ban account with ID {}", acc_id),
            ));
        }
        Ok(())
    }

    fn unban_account(&mut self, acc_id: BigInt) -> FFResult<()> {
        let client = &mut self.client;
        let updated = Self::exec(client, "unban_account", &[&acc_id])?;
        if updated == 0 {
            return Err(FFError::build(
                Severity::Warning,
                format!("Failed to unban account with ID {}", acc_id),
            ));
        }
        Ok(())
    }

    fn update_selected_player(&mut self, acc_id: BigInt, slot_num: Int) -> FFResult<()> {
        let client = &mut self.client;
        let timestamp_now = util::get_timestamp_sec(SystemTime::now()) as Int;
        let updated = Self::exec(
            client,
            "update_selected",
            &[&acc_id, &slot_num, &timestamp_now],
        )?;
        assert_eq!(updated, 1);
        Ok(())
    }

    fn load_player(&mut self, acc_id: BigInt, pc_uid: BigInt) -> FFResult<Player> {
        let client = &mut self.client;
        let rows = Self::query(client, "load_players", &[&acc_id])?;
        for row in &rows {
            if row.get::<_, BigInt>("PlayerId") == pc_uid {
                return Self::load_player_internal(client, row, true);
            }
        }
        Err(FFError::build(
            Severity::Warning,
            format!(
                "Player with UID {} not found for account with ID {}",
                pc_uid, acc_id
            ),
        ))
    }

    fn load_players(&mut self, acc_id: BigInt) -> FFResult<Vec<Player>> {
        let client = &mut self.client;
        let chars = Self::query(client, "load_players", &[&acc_id])?;
        let mut players = Vec::with_capacity(chars.len());
        for row in chars {
            match Self::load_player_internal(client, &row, true) {
                Ok(p) => players.push(p),
                Err(e) => {
                    let pc_uid: BigInt = row.get("PlayerId");
                    log(
                        Severity::Warning,
                        &format!("Failed to load player {}: {}", pc_uid, e.get_msg()),
                    );
                }
            }
        }
        Ok(players)
    }

    fn save_player(&mut self, player: &Player) -> FFResult<()> {
        Self::save_player_internal(&mut self.client, player)
    }

    fn save_players(&mut self, players: &[&Player]) -> FFResult<()> {
        let mut tsct = self.client.transaction().map_err(FFError::from_db_err)?;
        for player in players {
            Self::save_player_internal(&mut tsct, player)?;
        }
        tsct.commit().map_err(FFError::from_db_err)?;
        Ok(())
    }

    fn delete_player(&mut self, pc_uid: BigInt) -> FFResult<()> {
        let client = &mut self.client;
        let updated: u64 = Self::exec(client, "delete_player", &[&pc_uid])?;
        assert_eq!(updated, 1);
        Ok(())
    }
}
