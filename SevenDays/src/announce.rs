//! 天数切换时的广播与屏幕大标题。

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::Server;

use crate::config::{day_config, LOG_PREFIX};
use crate::state::DayState;

/// 广播 + 给每个在线玩家显示大标题。
pub fn announce(server: &Server, state: &DayState) {
    let cfg = day_config(state.day);
    let title = format!(
        "第 {} 天 · {} · {}",
        state.day, cfg.weekday, cfg.name
    );
    server.broadcast(&format!("{} 第 {} 轮 — {title}", LOG_PREFIX, state.round));

    let subtitle = format!("第 {} 轮", state.round);
    for player in server.get_all_players() {
        player.send_title_animation(10, 60, 20);
        player.show_title(TextComponent::text(&title));
        player.show_subtitle(TextComponent::text(&subtitle));
    }
}
