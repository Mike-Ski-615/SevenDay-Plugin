use crate::state::clamp_day;
pub const DAY_PERMISSION: &str = "seven_day_war:day";
pub const LOG_PREFIX: &str = "[七日战争]";
pub struct DayConfig {
    pub name: &'static str,
    pub weekday: &'static str,
}
pub const DAYS: [DayConfig; 7] = [
    DayConfig {
        name: "自由发育日",
        weekday: "Mon",
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Tue",
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Wed",
    },
    DayConfig {
        name: "阵营形成日",
        weekday: "Thu",
    },
    DayConfig {
        name: "幸运交易日",
        weekday: "Fri",
    },
    DayConfig {
        name: "自由发育日",
        weekday: "Sat",
    },
    DayConfig {
        name: "决战 PvP 日",
        weekday: "Sun",
    },
];
pub fn day_config(day: u32) -> &'static DayConfig {
    &DAYS[(clamp_day(day) - 1) as usize]
}
