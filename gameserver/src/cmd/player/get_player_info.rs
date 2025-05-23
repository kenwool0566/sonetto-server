use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::{DAY_MS, YEAR_MS, send_message, send_raw_buffer, time_ms_u64};
use common::USER_ID;
use sonettobuf::{CmdId, GetPlayerInfoReply, HeroSimpleInfo, PlayerInfo};
// use sonettobuf::OpenInfo;
use tokio::net::TcpStream;

pub async fn on_get_player_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let cur_time = time_ms_u64();

    let show_heros = vec![HeroSimpleInfo {
        hero_id: 3023,
        level: Some(1),
        rank: Some(1),
        ex_skill_level: Some(1),
        skin: None,
    }];

    let player_info = PlayerInfo {
        user_id: Some(USER_ID),
        name: Some(String::from("kenwool")),
        portrait: Some(170001),
        level: Some(1),
        exp: Some(0),
        signature: Some(String::from("I alone am the honored one.")),
        birthday: Some(String::from("")),
        show_heros,
        register_time: Some((cur_time - YEAR_MS) as i64),
        hero_rare_nn_count: Some(0),
        hero_rare_n_count: Some(0),
        hero_rare_r_count: Some(0),
        hero_rare_sr_count: Some(1),
        hero_rare_ssr_count: Some(0),
        last_episode_id: Some(1),
        last_login_time: Some((cur_time - DAY_MS) as i64),
        last_logout_time: Some((cur_time - (DAY_MS / 2)) as i64),
        character_age: Vec::new(),
        show_achievement: None,
        bg: None,
        total_login_days: Some(1),
    };

    let open_infos = Vec::new();

    let data = GetPlayerInfoReply {
        player_info: Some(player_info),
        openinfos: open_infos,
        can_rename: Some(true),
        main_thumbnail: Some(false),
        ext_rename: Some(0),
    };

    send_message(socket, cmd_id, data, 0).await?;

    Ok(())
}
