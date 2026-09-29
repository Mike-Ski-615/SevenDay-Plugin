//! 主题曲播放。
//!
//! 用原生 `Player::play_sound`（TS 版的 `player.playSound` 在本机构建上会 wasm trap，
//! 被迫改用 `/playsound` 命令；Rust 侧没有这个问题）。

use pumpkin_plugin_api::wit::pumpkin::plugin::sounds::SoundCategory;
use pumpkin_plugin_api::Player;

use crate::config::day_config;

/// 给单个玩家换歌：
/// 1. 先停掉 `music` 分类下**全部**声音（含原版自动播放的背景音乐与旧主题曲），避免重叠；
/// 2. 再播放当天主题曲。
pub fn play_theme(player: &Player, day: u32) {
    player.stop_sound(None, Some(SoundCategory::Music));
    player.play_sound(day_config(day).sound, SoundCategory::Music, 1.0, 1.0);
}
