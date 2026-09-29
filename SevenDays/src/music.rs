//! 主题曲播放。
//!
//! 用 `Player::play_custom_sound`（直接传音效名字）而**不是** `play_sound`：
//!
//! 宿主的 `play_sound` 实现是
//! ```text
//! let name = format!("{sound:?}").to_lowercase().replace('_', ".");
//! let data = Sound::from_name(&name).ok_or_else(|| Error::msg(...))?;
//! ```
//! 枚举变体 `MusicOverworldForest` 会被拼成 `musicoverworldforest`（分隔符全丢），
//! `from_name` 必然查不到 → 返回 `Err`。而 WIT 的 `play-sound` 签名没有 `result`，
//! 宿主返回 Err 在 guest 侧表现为 **wasm trap**，会直接中止整个插件实例。
//!
//! `play_custom_sound` 把字符串原样发成 `SoundEvent`，无查表、无失败路径。

use pumpkin_plugin_api::wit::pumpkin::plugin::sounds::SoundCategory;
use pumpkin_plugin_api::Player;

use crate::config::day_config;

/// 给单个玩家换歌：
/// 1. 先停掉 `music` 分类下**全部**声音（含原版自动播放的背景音乐与旧主题曲），避免重叠；
///    传 `None` 给 sound 表示"该分类下全部"，宿主不做查表，安全。
/// 2. 再播放当天主题曲。
pub fn play_theme(player: &Player, day: u32) {
    player.stop_sound(None, Some(SoundCategory::Music));
    player.play_custom_sound(day_config(day).sound, SoundCategory::Music, 1.0, 1.0);
}
