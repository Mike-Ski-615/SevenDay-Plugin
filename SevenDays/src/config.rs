//! 天数配置：每天的名称、星期缩写、主题曲。

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
    /// 当日主题曲（原版音效资源 ID）。
    ///
    /// 用字符串 + `play_custom_sound` 而不是 `Sound` 枚举：
    /// 宿主的 `play_sound` 用 `format!("{sound:?}").to_lowercase().replace('_', ".")`
    /// 拼名字（如 `MusicOverworldForest` → `musicoverworldforest`），分隔符全丢，
    /// 导致 `from_name` 查不到而返回 Err；WIT 签名没有 result，Err 在 guest 侧就是 trap。
    /// `play_custom_sound` 直接收名字，无查表、无失败路径。
    pub sound: &'static str,
}

/// 7 天的配置表，下标 0 = 第 1 天（周一）。
pub const DAYS: [DayConfig; 7] = [
    DayConfig {
        name: "自由发育日",
        weekday: "Mon",
        sound: "minecraft:music.overworld.forest",
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Tue",
        sound: "minecraft:music.overworld.meadow",
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Wed",
        sound: "minecraft:music.overworld.grove",
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Thu",
        sound: "minecraft:music.overworld.jungle",
    },
    DayConfig {
        name: "幸运交易日",
        weekday: "Fri",
        sound: "minecraft:music.overworld.cherry_grove",
    },
    DayConfig {
        name: "自由发育日",
        weekday: "Sat",
        sound: "minecraft:music.overworld.lush_caves",
    },
    DayConfig {
        name: "决战 PvP 日",
        weekday: "Sun",
        sound: "minecraft:music.dragon",
    },
];

/// 取某天的配置（越界时回退到第 1 天）。
pub fn day_config(day: u32) -> &'static DayConfig {
    &DAYS[(clamp_day(day) - 1) as usize]
}
