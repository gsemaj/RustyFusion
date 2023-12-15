#![allow(non_upper_case_globals)]

pub const SIZEOF_NANO_SKILLS: usize = 3;
pub const NANO_STAMINA_MAX: i16 = 150;

/* Constants ripped from the client */

pub const SUCC: u32 = 1;
pub const FAIL: u32 = 0;
pub const SIZEOF_BYTE: u32 = 1;
pub const SIZEOF_DWORD: u32 = 4;
pub const SIZEOF_INT: u32 = 4;
pub const SIZEOF_FLOAT: u32 = 4;
pub const SIZEOF_SHORT: u32 = 2;
pub const SIZEOF_ULONG: u32 = 4;
pub const SIZEOF_UINT64: u32 = 8;
pub const SIZEOF_IP_STRING: u32 = 16;
pub const SIZEOF_CN_UID_STRING: u32 = 50;
pub const SIZEOF_ACCOUNT_STRING: u32 = 33;
pub const SIZEOF_PASSWORD_STRING: u32 = 33;
pub const SIZEOF_AUTH_ID_STRING: u32 = 255;
pub const CN_MAX_COUNT_GROUP_MEMBER: u32 = 5;
pub const CN_MAX_COUNT_PC_GROUP_MEMBER: u32 = 4;
pub const CN_MAX_COUNT_NPC_GROUP_MEMBER: u32 = 5;
pub const CHAT_MAX_STRING: u32 = 128;
pub const PC_START_LOCATION_RANDOM_RANGE: u32 = 10000;
pub const SIZEOF_ANNOUNCE_STRING: u32 = 512;
pub const SERVER_COUNT_SHARD_CLIENT: u32 = 25;
pub const EXIT_CODE_DISCONNECT: u32 = 0;
pub const EXIT_CODE_REQ_BY_PC: u32 = 1;
pub const EXIT_CODE_REQ_BY_SVR: u32 = 2;
pub const EXIT_CODE_REQ_BY_GM: u32 = 3;
pub const EXIT_CODE_HACK: u32 = 4;
pub const EXIT_CODE_ERROR: u32 = 5;
pub const EXIT_CODE_LIVE_CHECK: u32 = 6;
pub const EXIT_CODE_REQ_BY_PC_DUPE_LOGIN: u32 = 7;
pub const EXIT_CODE_SERVER_ERROR: u32 = 99;
pub const SIZEOF_USER_ID: u32 = 32;
pub const SIZEOF_USER_PW: u32 = 32;
pub const SIZEOF_PC_SLOT: u32 = 4;
pub const SIZEOF_PC_NAME: u32 = 16;
pub const SIZEOF_PC_FIRST_NAME: u32 = 9;
pub const SIZEOF_PC_LAST_NAME: u32 = 17;
pub const SIZEOF_PC_NAME_FLAG: u32 = 8;
pub const GENDER_NONE: u32 = 0;
pub const GENDER_MALE: u32 = 1;
pub const GENDER_FEMALE: u32 = 2;
pub const MENTOR_CHANGE_BASE_COST: u32 = 100;
pub const REPEAT_MISSION_RESET_TIME: u32 = 9;
pub const SIZEOF_REPEAT_QUESTFLAG_NUMBER: u32 = 8;
pub const FATIGUE_RESET_TIME: u32 = 0;
pub const PC_FATIGUE_KILL_UNIT: u32 = 7;
pub const PC_FATIGUE_1_LEVEL: u32 = 11420;
pub const PC_FATIGUE_2_LEVEL: u32 = 6480;
pub const PC_FATIGUE_MAX_LEVEL: u32 = 2;
pub const PC_FUSIONMATTER_MAX: u32 = 999999999;
pub const PC_CANDY_MAX: u32 = 999999999;
pub const PC_BATTERY_MAX: u32 = 9999;
pub const PC_LEVEL_MAX: u32 = 36;
pub const SIZEOF_PC_BULLET_SLOT: u32 = 3;
pub const PC_TICK_TIME: u32 = 5000;
pub const SIZEOF_EQUIP_SLOT: u32 = 9;
pub const EQUIP_SLOT_HAND: u32 = 0;
pub const EQUIP_SLOT_UPPERBODY: u32 = 1;
pub const EQUIP_SLOT_LOWERBODY: u32 = 2;
pub const EQUIP_SLOT_FOOT: u32 = 3;
pub const EQUIP_SLOT_HEAD: u32 = 4;
pub const EQUIP_SLOT_FACE: u32 = 5;
pub const EQUIP_SLOT_BACK: u32 = 6;
pub const EQUIP_SLOT_END: u32 = 6;
pub const EQUIP_SLOT_HAND_EX: u32 = 7;
pub const EQUIP_SLOT_VEHICLE: u32 = 8;
pub const WPN_EQUIP_TYPE_NONE: u32 = 0;
pub const WPN_EQUIP_TYPE_OH_BLADE: u32 = 1;
pub const WPN_EQUIP_TYPE_OH_CLUB: u32 = 2;
pub const WPN_EQUIP_TYPE_OH_PISTOL: u32 = 3;
pub const WPN_EQUIP_TYPE_OH_RIPLE: u32 = 4;
pub const WPN_EQUIP_TYPE_OH_THROW: u32 = 5;
pub const WPN_EQUIP_TYPE_DH_BLADE: u32 = 6;
pub const WPN_EQUIP_TYPE_DH_CLUB: u32 = 7;
pub const WPN_EQUIP_TYPE_DH_DPISTOL: u32 = 8;
pub const WPN_EQUIP_TYPE_DH_RIPLE: u32 = 9;
pub const WPN_EQUIP_TYPE_DH_THROW: u32 = 10;
pub const WPN_EQUIP_TYPE_DH_ROCKET: u32 = 11;
pub const SIZEOF_INVEN_SLOT: u32 = 50;
pub const SIZEOF_QINVEN_SLOT: u32 = 50;
pub const SIZEOF_BANK_SLOT: u32 = 119;
pub const SIZEOF_RESTORE_SLOT: u32 = 5;
pub const SIZEOF_NANO_BANK_SLOT: u32 = 37;
pub const SIZEOF_QUEST_SLOT: u32 = 1024;
pub const NANO_QUEST_INDEX: u32 = 0;
pub const SIZEOF_RQUEST_SLOT: u32 = 9;
pub const SIZEOF_QUESTFLAG_NUMBER: u32 = 32;
pub const SIZEOF_EP_RECORD_SLOT: u32 = 51;
pub const SIZEOF_TRADE_SLOT: u32 = 12;
pub const SIZEOF_VENDOR_TABLE_SLOT: u32 = 20;
pub const SIZEOF_VENDOR_RESTORE_SLOT: u32 = 5;
pub const SIZEOF_QUEST_NPC_SLOT: u32 = 3;
pub const SIZEOF_QUEST_ITEM_SLOT: u32 = 3;
pub const SIZEOF_MAX_ITEM_STACK: u32 = 100;
pub const SIZEOF_PC_SKILL_SLOT: u32 = 33;
pub const SIZEOF_QUICK_SLOT: u32 = 8;
pub const ENCHANT_WEAPON_MATERIAL_ID: u32 = 101;
pub const ENCHANT_DEFENCE_MATERIAL_ID: u32 = 102;
pub const SIZEOF_NANO_CARRY_SLOT: u32 = 3;
pub const COUNTOF_NANO_PER_SET: u32 = 3;
pub const SIZEOF_NANO_SET: u32 = 13;
pub const SIZEOF_NANO_STYLE: u32 = 3;
pub const NANO_STYLE_NONE: u32 = 1;
pub const NANO_STYLE_CRYSTAL: u32 = 0;
pub const NANO_STYLE_ENERGY: u32 = 1;
pub const NANO_STYLE_FLUID: u32 = 2;
pub const SIZEOF_NANO_TYPE: u32 = 4;
pub const NANO_TYPE_POWER: u32 = 0;
pub const NANO_TYPE_ACCURACY: u32 = 1;
pub const NANO_TYPE_PROTECT: u32 = 2;
pub const NANO_TYPE_DODGE: u32 = 3;
pub const SIZEOF_NANO_TUNE_NEED_ITEM_SLOT: u32 = 10;
pub const VALUE_ATTACK_MISS: u32 = 1;
pub const VALUE_BATTERY_EMPTY_PENALTY: f32 = 0.5;
pub const MSG_ONLINE: u32 = 1;
pub const MSG_BUSY: u32 = 2;
pub const MSG_OFFLINE: u32 = 0;
pub const SIZEOF_FREE_CHAT_STRING: u32 = 128;
pub const SIZEOF_MENU_CHAT_STRING: u32 = 128;
pub const SIZEOF_BUDDYLIST_SLOT: u32 = 50;
pub const SIZEOF_EMAIL_SUBJECT_STRING: u32 = 32;
pub const SIZEOF_EMAIL_CONTENT_STRING: u32 = 512;
pub const SIZEOF_EMAIL_PAGE_SIZE: u32 = 5;
pub const SIZEOF_EMAIL_ITEM_CNT: u32 = 4;
pub const EMAIL_AND_MONEY_COST: u32 = 50;
pub const EMAIL_ITEM_COST: u32 = 20;
pub const BUDDYWARP_INTERVAL: u32 = 60;
pub const EMAILSEND_TIME_DELAY: u32 = 60;
pub const DB_ERROR_INVALID_DATA: u32 = 1;
pub const DB_ERROR_HACK_ATTEMPT: u32 = 2;
pub const DB_ERROR_ACCESS_FAIL: u32 = 3;
pub const DB_ERROR_PC_INSERT_FAIL: u32 = 4;
pub const CALL_NPC_MAX_CNT: u32 = 2048;
pub const CN_EP_RING_MAX_CNT: u32 = 999;
pub const CN_EP_RANK_1: f32 = 0.8;
pub const CN_EP_RANK_2: f32 = 0.7;
pub const CN_EP_RANK_3: f32 = 0.5;
pub const CN_EP_RANK_4: f32 = 0.3;
pub const CN_EP_RANK_5: f32 = 0.29;
pub const HF_BIT_NONE: u32 = 0;
pub const HF_BIT_NORMAL: u32 = 1;
pub const HF_BIT_CRITICAL: u32 = 2;
pub const HF_BIT_STYLE_WIN: u32 = 4;
pub const HF_BIT_STYLE_TIE: u32 = 8;
pub const HF_BIT_STYLE_LOSE: u32 = 16;
pub const SKIN_COLOR_MAX: u32 = 12;
pub const HAIR_COLOR_MAX: u32 = 18;
pub const EYE_COLOR_MAX: u32 = 5;
pub const BODY_TYPE_MAX: u32 = 3;
pub const HEIGHT_TYPE_MAX: u32 = 5;
pub const CLASS_TYPE_MAX: u32 = 4;
pub const CN_EP_RACE_MODE_PRACTICE: u32 = 0;
pub const CN_EP_RACE_MODE_RECORD: u32 = 1;
pub const CN_EP_SECOM_NPC_TYPE_NUM: u32 = 13;
pub const CN_EP_EECOM_NPC_TYPE_NUM: u32 = 14;
pub const CN_EP_SIZE_SMALL: u32 = 0;
pub const CN_EP_SIZE_MIDDLE: u32 = 1;
pub const CN_EP_SIZE_BIG: u32 = 2;
pub const CN_EP_TICKET_ITEM_ID_SMALL: u32 = 115;
pub const CN_EP_TICKET_ITEM_ID_MIDDLE: u32 = 116;
pub const CN_EP_TICKET_ITEM_ID_BIG: u32 = 117;
pub const CN_EP_TICKET_ITEM_ID_FREE: u32 = 118;
pub const CN_EP_DISTANCE_ERROR_SAFE_RANGE: u32 = 1200;
pub const CN_ACCOUNT_LEVEL__MASTER: u32 = 1;
pub const CN_ACCOUNT_LEVEL__POWER_DEVELOPER: u32 = 10;
pub const CN_ACCOUNT_LEVEL__QA: u32 = 20;
pub const CN_ACCOUNT_LEVEL__GM: u32 = 30;
pub const CN_ACCOUNT_LEVEL__CS: u32 = 40;
pub const CN_ACCOUNT_LEVEL__FREE_USER: u32 = 48;
pub const CN_ACCOUNT_LEVEL__PAY_USER: u32 = 49;
pub const CN_ACCOUNT_LEVEL__DEVELOPER: u32 = 50;
pub const CN_ACCOUNT_LEVEL__CLOSEBETA_USER: u32 = 80;
pub const CN_ACCOUNT_LEVEL__OPENBETA_USER: u32 = 85;
pub const CN_ACCOUNT_LEVEL__USER: u32 = 99;
pub const CN_SPECIAL_STATE_FLAG__PRINT_GM: u32 = 1;
pub const CN_SPECIAL_STATE_FLAG__INVISIBLE: u32 = 2;
pub const CN_SPECIAL_STATE_FLAG__INVULNERABLE: u32 = 4;
pub const CN_SPECIAL_STATE_FLAG__FULL_UI: u32 = 16;
pub const CN_SPECIAL_STATE_FLAG__COMBAT: u32 = 32;
pub const CN_SPECIAL_STATE_FLAG__MUTE_FREECHAT: u32 = 64;
pub const CN_GM_SET_VALUE_TYPE__HP: u32 = 1;
pub const CN_GM_SET_VALUE_TYPE__WEAPON_BATTERY: u32 = 2;
pub const CN_GM_SET_VALUE_TYPE__NANO_BATTERY: u32 = 3;
pub const CN_GM_SET_VALUE_TYPE__FUSION_MATTER: u32 = 4;
pub const CN_GM_SET_VALUE_TYPE__CANDY: u32 = 5;
pub const CN_GM_SET_VALUE_TYPE__SPEED: u32 = 6;
pub const CN_GM_SET_VALUE_TYPE__JUMP: u32 = 7;
pub const CN_GM_SET_VALUE_TYPE__END: u32 = 8;
pub const HEIGHT_CLIMBABLE: u32 = 150;
pub const CN_GROUP_WARP_CHECK_RANGE: u32 = 1000;
pub const WYVERN_LOCATION_FLAG_SIZE: u32 = 2;
pub const CN_PC_EVENT_ID_GET_NANO_QUEST: u32 = 1;
pub const CN_PC_EVENT_ID_DEFEAT_FUSE_AND_GET_NANO: u32 = 2;
pub const _dCN_STREETSTALL__ITEMLIST_COUNT_MAX: u32 = 5;
pub const CSB_BIT_NONE: u32 = 0;
pub const CSB_BIT_UP_MOVE_SPEED: u32 = 1;
pub const CSB_BIT_UP_SWIM_SPEED: u32 = 2;
pub const CSB_BIT_UP_JUMP_HEIGHT: u32 = 4;
pub const CSB_BIT_UP_STEALTH: u32 = 8;
pub const CSB_BIT_PHOENIX: u32 = 16;
pub const CSB_BIT_PROTECT_BATTERY: u32 = 32;
pub const CSB_BIT_PROTECT_INFECTION: u32 = 64;
pub const CSB_BIT_DN_MOVE_SPEED: u32 = 128;
pub const CSB_BIT_DN_ATTACK_SPEED: u32 = 256;
pub const CSB_BIT_STUN: u32 = 512;
pub const CSB_BIT_MEZ: u32 = 1024;
pub const CSB_BIT_KNOCKDOWN: u32 = 2048;
pub const CSB_BIT_MINIMAP_ENEMY: u32 = 4096;
pub const CSB_BIT_MINIMAP_TRESURE: u32 = 8192;
pub const CSB_BIT_REWARD_BLOB: u32 = 16384;
pub const CSB_BIT_REWARD_CASH: u32 = 32768;
pub const CSB_BIT_INFECTION: u32 = 65536;
pub const CSB_BIT_FREEDOM: u32 = 131072;
pub const CSB_BIT_BOUNDINGBALL: u32 = 262144;
pub const CSB_BIT_INVULNERABLE: u32 = 524288;
pub const CSB_BIT_STIMPAKSLOT1: u32 = 1048576;
pub const CSB_BIT_STIMPAKSLOT2: u32 = 2097152;
pub const CSB_BIT_STIMPAKSLOT3: u32 = 4194304;
pub const CSB_BIT_HEAL: u32 = 8388608;
pub const CSB_BIT_EXTRABANK: u32 = 16777216;
pub const TIME_BUFF_CONFIRM_KEY_MAX: u32 = 2000000000;
pub const READPACKET_SUCC: u32 = 0;
pub const READPACKET_FAIL: u32 = 1;
pub const READPACKET_RETURN: u32 = 2;
pub const BITMASK_FROM2TO: u32 = 4278190080;
pub const BITMASK_FROM: u32 = 4026531840;
pub const BITMASK_TO: u32 = 251658240;
pub const BITMASK_SENDBLOCK: u32 = 8388608;
pub const BITMASK_AUTHED: u32 = 4194304;
pub const BITMASK_U_ID: u32 = 4095;
pub const CL2LS: u32 = 301989888;
pub const CL2FE: u32 = 318767104;
pub const LS2CL: u32 = 553648128;
pub const LS2LS: u32 = 570425344;
pub const LS2DBA: u32 = 654311424;
pub const FE2CL: u32 = 822083584;
pub const FE2FE: u32 = 855638016;
pub const FE2GS: u32 = 872415232;
pub const FE2EP: u32 = 905969664;
pub const FE2MSG: u32 = 939524096;
pub const GS2FE: u32 = 1124073472;
pub const GS2GS: u32 = 1140850688;
pub const GS2AI: u32 = 1157627904;
pub const GS2EP: u32 = 1174405120;
pub const GS2DBA: u32 = 1191182336;
pub const GS2MSG: u32 = 1207959552;
pub const GS2MGR: u32 = 1241513984;
pub const AI2GS: u32 = 1409286144;
pub const EP2FE: u32 = 1660944384;
pub const EP2GS: u32 = 1677721600;
pub const DBA2GS: u32 = 1946157056;
pub const DBA2EP: u32 = 1962934272;
pub const MSG2FE: u32 = 2197815296;
pub const MSG2GS: u32 = 2214592512;
pub const MSG2CMSG: u32 = 2298478592;
pub const CMSG2MSG: u32 = 2550136832;
pub const MGR2SPY: u32 = 3003121664;
pub const SPY2MGR: u32 = 3019898880;
pub const MGR2SA: u32 = 3036676096;
pub const SA2MGR: u32 = 3053453312;
pub const SA2SPY: u32 = 3070230528;
pub const SPY2SA: u32 = 3087007744;
pub const SPY2SVR: u32 = 3103784960;
pub const SVR2SPY: u32 = 3120562176;
pub const SCH2SVR: u32 = 3221225472;
pub const SCH2LS: u32 = 3254779904;
pub const SCH2FE: u32 = 3271557120;
pub const SCH2GS: u32 = 3288334336;
pub const SCH2AI: u32 = 3305111552;
pub const SCH2EP: u32 = 3321888768;
pub const SCH2DBA: u32 = 3338665984;
pub const SCH2MSG: u32 = 3355443200;
pub const SCH2CMSG: u32 = 3372220416;
pub const CL2CDR: u32 = 520093696;
pub const SENDBLOCK: u32 = 8388608;
pub const AUTHED_X: u32 = 0;
pub const AUTHED_O: u32 = 4194304;
pub const SEND_SVR_FE: u32 = 1;
pub const SEND_SVR_FE_ANY: u32 = 2;
pub const SEND_SVR_FE_ALL: u32 = 3;
pub const SEND_SVR_AI: u32 = 4;
pub const SEND_SVR_AI_ANY: u32 = 5;
pub const SEND_SVR_AI_ALL: u32 = 6;
pub const SEND_SVR_FE_AI_ALL: u32 = 7;
pub const SEND_SVR_DBA: u32 = 8;
pub const SEND_SVR_GS: u32 = 9;
pub const SEND_SVR_MSG: u32 = 10;
pub const SEND_SVR_MSG_ANY: u32 = 11;
pub const SEND_SVR_MSG_ALL: u32 = 12;
pub const SEND_UNICAST: u32 = 1;
pub const SEND_ANYCAST: u32 = 2;
pub const SEND_ANYCAST_NEW: u32 = 3;
pub const SEND_BROADCAST: u32 = 4;
