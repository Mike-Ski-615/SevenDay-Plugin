//! 天数配置：每天的名称、星期缩写、主题曲。

use pumpkin_plugin_api::wit::pumpkin::plugin::sounds::Sound;

use crate::state::clamp_day;

/// 命令权限节点；命名空间须与插件名一致。
pub const DAY_PERMISSION: &str = "seven_day_war:day";

/// 日志统一前缀。
pub const LOG_PREFIX: &str = "[七日战争]";

/// 一天的名字与主题曲。
pub struct DayConfig {
    /// 当日名称。
    pub name: &'static str,
    /// 星期缩写（命令里也用这个）。
    pub weekday: &'static str,
    /// 当日主题曲（原版音效，用原生 `play_sound` 播放，无需 `/playsound` 命令）。
    pub sound: Sound,
}

/// 7 天的配置表，下标 0 = 第 1 天（周一）。
pub const DAYS: [DayConfig; 7] = [
    DayConfig {
        name: "自由发育日",
        weekday: "Mon",
        sound: Sound::MusicOverworldForest,
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Tue",
        sound: Sound::MusicOverworldMeadow,
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Wed",
        sound: Sound::MusicOverworldGrove,
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Thu",
        sound: Sound::MusicOverworldJungle,
    },
    DayConfig {
        name: "幸运交易日",
        weekday: "Fri",
        sound: Sound::MusicOverworldCherryGrove,
    },
    DayConfig {
        name: "自由发育日",
        weekday: "Sat",
        sound: Sound::MusicOverworldLushCaves,
    },
    DayConfig {
        name: "决战 PvP 日",
        weekday: "Sun",
        sound: Sound::MusicDragon,
    },
];

/// 取某天的配置（越界时回退到第 1 天）。
pub fn day_config(day: u32) -> &'static DayConfig {
    &DAYS[(clamp_day(day) - 1) as usize]
}
