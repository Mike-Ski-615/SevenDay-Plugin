use crate::config::day_config;
use pumpkin_plugin_api::wit::pumpkin::plugin::sounds::SoundCategory;
use pumpkin_plugin_api::Player;
pub const THEME_CATEGORY: SoundCategory = SoundCategory::Records;
pub fn play_theme(player: &Player, day: u32) {
    player.stop_sound(None, Some(SoundCategory::Music));
    player.stop_sound(None, Some(THEME_CATEGORY));
    player.play_custom_sound(day_config(day).sound, THEME_CATEGORY, 1.0, 1.0);
}
pub fn silence_vanilla_music(player: &Player) {
    player.stop_sound(None, Some(SoundCategory::Music));
}
