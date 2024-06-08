use rusty_fusion::{
    defines::*,
    entity::{Entity, EntityID},
    error::*,
    net::{
        packet::{PacketID::*, *},
        ClientMap,
    },
    state::ShardServerState,
    util,
};

const ERROR_CODE_BUDDY_DENY: i32 = 6;

pub fn request_make_buddy(clients: &mut ClientMap, state: &mut ShardServerState) -> FFResult<()> {
    let client = clients.get_self();
    let pkt: sP_CL2FE_REQ_REQUEST_MAKE_BUDDY =
        *client.get_packet(P_CL2FE_REQ_REQUEST_MAKE_BUDDY)?;

    let pc_id = client.get_player_id()?;
    let buddy_id = pkt.iBuddyID;
    let buddy_uid = pkt.iBuddyPCUID;

    state.entity_map.validate_proximity(
        &[EntityID::Player(pc_id), EntityID::Player(buddy_id)],
        RANGE_INTERACT,
    )?;

    let player = state.get_player(pc_id)?;
    if player.is_buddies_with(buddy_uid) {
        return Err(FFError::build(
            Severity::Warning,
            format!("{} is already buddies with player {}", player, buddy_uid),
        ));
    }

    if player.get_num_buddies() >= SIZEOF_BUDDYLIST_SLOT as usize {
        return Err(FFError::build(
            Severity::Warning,
            format!("{} has too many buddies", player),
        ));
    }

    let req_pkt = sP_FE2CL_REP_REQUEST_MAKE_BUDDY_SUCC_TO_ACCEPTER {
        iRequestID: pc_id,
        iBuddyID: buddy_id,
        szFirstName: util::encode_utf16(&player.first_name),
        szLastName: util::encode_utf16(&player.last_name),
    };

    let buddy = state.get_player(buddy_id)?;
    if buddy.get_uid() != buddy_uid {
        return Err(FFError::build(
            Severity::Warning,
            format!(
                "Buddy UID mismatch (client: {}, server: {})",
                buddy_uid,
                buddy.get_uid()
            ),
        ));
    }
    if buddy.get_num_buddies() >= SIZEOF_BUDDYLIST_SLOT as usize {
        // instant deny
        let deny_pkt = sP_FE2CL_REP_ACCEPT_MAKE_BUDDY_FAIL {
            iBuddyID: buddy_id,
            iBuddyPCUID: buddy_uid,
            iErrorCode: ERROR_CODE_BUDDY_DENY,
        };
        return client.send_packet(P_FE2CL_REP_ACCEPT_MAKE_BUDDY_FAIL, &deny_pkt);
    }

    let buddy_client = buddy.get_client(clients).unwrap();
    if buddy_client
        .send_packet(P_FE2CL_REP_REQUEST_MAKE_BUDDY_SUCC_TO_ACCEPTER, &req_pkt)
        .is_ok()
    {
        let player = state.get_player_mut(pc_id).unwrap();
        player.buddy_offered_to = Some(buddy_uid);
    }

    Ok(())
}
