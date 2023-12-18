use rusty_fusion::{
    defines::ID_TIME_MACHINE_WARP, enums::TransportationType, error::catch_fail,
    tabledata::tdata_get, Combatant, Item,
};

use super::*;

pub fn regist_transportation_location(
    client: &mut FFClient,
    state: &mut ShardServerState,
) -> FFResult<()> {
    let pkt: sP_CL2FE_REQ_REGIST_TRANSPORTATION_LOCATION =
        *client.get_packet(P_CL2FE_REQ_REGIST_TRANSPORTATION_LOCATION)?;
    catch_fail(
        (|| {
            // TODO NPC proximity check

            let transport_type: TransportationType = pkt.eTT.try_into()?;
            let player = state.get_player_mut(client.get_player_id()?)?;
            match transport_type {
                TransportationType::Warp => {
                    player.update_scamper_flags(pkt.iLocationID)?;
                }
                TransportationType::Wyvern => {
                    player.update_skyway_flags(pkt.iLocationID)?;
                }
                TransportationType::Bus => {
                    return Err(FFError::build(
                        Severity::Warning,
                        "Bus reg invalid".to_string(),
                    ));
                }
            }

            let resp = sP_FE2CL_REP_PC_REGIST_TRANSPORTATION_LOCATION_SUCC {
                eTT: pkt.eTT,
                iLocationID: pkt.iLocationID,
                iWarpLocationFlag: player.get_scamper_flags(),
                aWyvernLocationFlag: player.get_skyway_flags(),
            };
            client.send_packet(P_FE2CL_REP_PC_REGIST_TRANSPORTATION_LOCATION_SUCC, &resp)
        })(),
        || {
            let resp = sP_FE2CL_REP_PC_REGIST_TRANSPORTATION_LOCATION_FAIL {
                eTT: pkt.eTT,
                iLocationID: pkt.iLocationID,
                iErrorCode: unused!(),
            };
            client.send_packet(P_FE2CL_REP_PC_REGIST_TRANSPORTATION_LOCATION_FAIL, &resp)
        },
    )
}

pub fn warp_use_transportation(
    clients: &mut ClientMap,
    state: &mut ShardServerState,
) -> FFResult<()> {
    let pkt: sP_CL2FE_REQ_PC_WARP_USE_TRANSPORTATION = *clients
        .get_self()
        .get_packet(P_CL2FE_REQ_PC_WARP_USE_TRANSPORTATION)?;
    catch_fail(
        (|| {
            let client = clients.get_self();
            let pc_id = client.get_player_id()?;
            let player = state.get_player_mut(pc_id)?;

            // TODO NPC proximity check

            let trip = tdata_get().get_trip_data(pkt.iTransporationID)?;
            if player.get_taros() < trip.cost {
                return Err(FFError::build(
                    Severity::Warning,
                    format!(
                        "Player {} doesn't have enough taros to warp",
                        player.get_player_id()
                    ),
                ));
            }

            match trip.transportation_type {
                TransportationType::Warp => {
                    let dest_data = tdata_get().get_scamper_data(trip.end_location)?;
                    player.set_position(dest_data.pos);
                }
                TransportationType::Wyvern => {}
                TransportationType::Bus => {
                    return Err(FFError::build(
                        Severity::Warning,
                        "Bus warp invalid".to_string(),
                    ));
                }
            }

            let player = state.get_player_mut(pc_id)?;
            let new_pos = player.get_position();
            let taros_left = player.set_taros(player.get_taros() - trip.cost);
            let resp = sP_FE2CL_REP_PC_WARP_USE_TRANSPORTATION_SUCC {
                eTT: trip.transportation_type as i32,
                iX: new_pos.x,
                iY: new_pos.y,
                iZ: new_pos.z,
                iCandy: taros_left as i32,
            };
            clients
                .get_self()
                .send_packet(P_FE2CL_REP_PC_WARP_USE_TRANSPORTATION_SUCC, &resp)
        })(),
        || {
            let resp = sP_FE2CL_REP_PC_WARP_USE_TRANSPORTATION_FAIL {
                iTransportationID: pkt.iTransporationID,
                iErrorCode: unused!(),
            };
            clients
                .get_self()
                .send_packet(P_FE2CL_REP_PC_WARP_USE_TRANSPORTATION_FAIL, &resp)
        },
    )
}

pub fn warp_use_npc(clients: &mut ClientMap, state: &mut ShardServerState) -> FFResult<()> {
    let pkt: sP_CL2FE_REQ_PC_WARP_USE_NPC =
        *clients.get_self().get_packet(P_CL2FE_REQ_PC_WARP_USE_NPC)?;
    catch_fail(
        (|| {
            let item_remaining = helpers::do_warp(
                clients,
                state,
                pkt.iNPC_ID,
                pkt.iWarpID,
                pkt.eIL1,
                pkt.iItemSlot1 as usize,
                pkt.eIL2,
                pkt.iItemSlot2 as usize,
            )?;

            let client = clients.get_self();
            let player = state.get_player(client.get_player_id().unwrap()).unwrap();
            let pos = player.get_position();
            let taros_left = player.get_taros();
            let resp = sP_FE2CL_REP_PC_WARP_USE_NPC_SUCC {
                iX: pos.x,
                iY: pos.y,
                iZ: pos.z,
                eIL: pkt.eIL2,
                iItemSlotNum: pkt.iItemSlot2,
                Item: item_remaining.into(),
                iCandy: taros_left as i32,
            };
            client.send_packet(P_FE2CL_REP_PC_WARP_USE_NPC_SUCC, &resp)
        })(),
        || {
            let resp = sP_FE2CL_REP_PC_WARP_USE_NPC_FAIL {
                iErrorCode: unused!(),
            };
            clients
                .get_self()
                .send_packet(P_FE2CL_REP_PC_WARP_USE_NPC_FAIL, &resp)
        },
    )
}

pub fn time_to_go_warp(clients: &mut ClientMap, state: &mut ShardServerState) -> FFResult<()> {
    let pkt: sP_CL2FE_REQ_PC_TIME_TO_GO_WARP = *clients
        .get_self()
        .get_packet(P_CL2FE_REQ_PC_TIME_TO_GO_WARP)?;
    catch_fail(
        (|| {
            let player = state.get_player(clients.get_self().get_player_id()?)?;
            if player.get_payzone_flag() {
                return Err(FFError::build(
                    Severity::Warning,
                    format!("Player {} is in the past", player.get_player_id()),
                ));
            }

            let item_remaining = helpers::do_warp(
                clients,
                state,
                pkt.iNPC_ID,
                ID_TIME_MACHINE_WARP,
                pkt.eIL1,
                pkt.iItemSlot1 as usize,
                pkt.eIL2,
                pkt.iItemSlot2 as usize,
            )?;

            let client = clients.get_self();
            let player = state.get_player(client.get_player_id().unwrap()).unwrap();
            let pos = player.get_position();
            let taros_left = player.get_taros();
            let resp = sP_FE2CL_REP_PC_WARP_USE_NPC_SUCC {
                iX: pos.x,
                iY: pos.y,
                iZ: pos.z,
                eIL: pkt.eIL2,
                iItemSlotNum: pkt.iItemSlot2,
                Item: item_remaining.into(),
                iCandy: taros_left as i32,
            };
            client.send_packet(P_FE2CL_REP_PC_WARP_USE_NPC_SUCC, &resp)
        })(),
        || {
            let resp = sP_FE2CL_REP_PC_WARP_USE_NPC_FAIL {
                iErrorCode: unused!(),
            };
            clients
                .get_self()
                .send_packet(P_FE2CL_REP_PC_WARP_USE_NPC_FAIL, &resp)
        },
    )
}

mod helpers {
    #![allow(clippy::too_many_arguments)]

    use super::*;

    pub fn do_warp(
        clients: &mut ClientMap,
        state: &mut ShardServerState,
        _npc_id: i32,
        warp_id: i32,
        req_item_location_ord: i32,
        req_item_slot: usize,
        req_item_consume_location_ord: i32,
        req_item_consume_slot: usize,
    ) -> FFResult<Option<Item>> {
        // TODO NPC proximity check

        let warp_data = tdata_get().get_warp_data(warp_id)?;
        let client = clients.get_self();
        let pc_id = client.get_player_id()?;
        let player = state.get_player_mut(pc_id)?;

        // TODO group proximity check

        if player.get_taros() < warp_data.cost {
            return Err(FFError::build(
                Severity::Warning,
                format!(
                    "Player {} doesn't have enough taros to warp",
                    player.get_player_id()
                ),
            ));
        }

        if player.get_level() < warp_data.min_level {
            return Err(FFError::build(
                Severity::Warning,
                format!(
                    "Player {} isn't a high enough level to warp ({} < {})",
                    player.get_player_id(),
                    player.get_level(),
                    warp_data.min_level
                ),
            ));
        }

        if let Some((item_type, item_id)) = warp_data.req_item {
            let item = player
                .get_item(req_item_location_ord.try_into()?, req_item_slot)?
                .as_ref();
            if !item.is_some_and(|item| item.get_type() == item_type && item.get_id() == item_id) {
                return Err(FFError::build(
                    Severity::Warning,
                    format!(
                        "Player {} doesn't have the required item ({:?}) to warp",
                        player.get_player_id(),
                        warp_data.req_item
                    ),
                ));
            }
        }

        let mut item_consumed = None;
        if let Some((item_type, item_id)) = warp_data.req_item_consumed {
            let item = player.get_item_mut(
                req_item_consume_location_ord.try_into()?,
                req_item_consume_slot,
            )?;
            if !item
                .as_mut()
                .is_some_and(|item| item.get_type() == item_type && item.get_id() == item_id)
            {
                return Err(FFError::build(
                    Severity::Warning,
                    format!(
                        "Player {} doesn't have the required consumed item ({:?}) to warp",
                        player.get_player_id(),
                        warp_data.req_item_consumed
                    ),
                ));
            }
            Item::split_items(item, 1); // consume item
            item_consumed = *item;
        }

        player.set_taros(player.get_taros() - warp_data.cost);
        let chunk = player.set_position(warp_data.pos);
        // TODO instancing
        state
            .entity_map
            .update(EntityID::Player(pc_id), Some(chunk), Some(clients));

        Ok(item_consumed)
    }
}
