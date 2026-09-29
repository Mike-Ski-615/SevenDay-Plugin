use crate::state::clamp_day;
pub const DAY_PERMISSION: &str = "seven_day_war:day";
pub const LOG_PREFIX: &str = "[七日战争]";
pub struct DayConfig {
    pub name: &'static str,
    pub weekday: &'static str,
    pub sound: &'static str,
}
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
pub fn day_config(day: u32) -> &'static DayConfig {
    &DAYS[(clamp_day(day) - 1) as usize]
}
