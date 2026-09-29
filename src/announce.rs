use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::Server;
use crate::config::{day_config, LOG_PREFIX};
use crate::state::DayState;
pub fn announce(server: &Server, state: &DayState) {
    let cfg = day_config(state.day);
    server.broadcast(&format!(
        "{} 第 {} 轮 — 第 {} 天（{} · {}）",
        LOG_PREFIX, state.round, state.day, cfg.weekday, cfg.name
    ));
    let title = format!("第 {} 天", state.day);
    let subtitle = format!("{} · 第 {} 轮", cfg.name, state.round);
    for player in server.get_all_players() {
        player.send_title_animation(10, 60, 20);
        player.show_title(TextComponent::text(&title));
        player.show_subtitle(TextComponent::text(&subtitle));
    }
}
