use pumpkin_plugin_api::wit::pumpkin::plugin::sounds::SoundCategory;
use pumpkin_plugin_api::Player;
use crate::config::day_config;
pub fn play_theme(player: &Player, day: u32) {
    player.stop_sound(None, Some(SoundCategory::Music));
    player.play_custom_sound(day_config(day).sound, SoundCategory::Music, 1.0, 1.0);
}
