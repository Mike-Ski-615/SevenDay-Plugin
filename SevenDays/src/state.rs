//! 天数状态与推进规则。纯数据 + 纯函数，不依赖插件 API。

/// 一轮的天数。
pub const CYCLE_DAYS: u32 = 7;

/// 当前天数 / 轮次 / 下一次切换的 epoch 毫秒。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DayState {
    /// 当前天数，1..CYCLE_DAYS。
    pub day: u32,
    /// 当前轮次，从 1 开始。
    pub round: u32,
    /// 下一次切换发生的 epoch 毫秒。
    pub boundary_at: i64,
}

impl DayState {
    /// 新一轮的开局状态（第 1 天第 1 轮）。
    pub fn fresh(boundary_at: i64) -> Self {
        Self {
            day: 1,
            round: 1,
            boundary_at,
        }
    }

    /// 前进一天；第 [`CYCLE_DAYS`] 天之后回到第 1 天并开启新一轮。
    pub fn advance(&self, next_boundary: i64) -> Self {
        let wrapped = self.day >= CYCLE_DAYS;
        Self {
            day: if wrapped { 1 } else { self.day + 1 },
            round: if wrapped { self.round + 1 } else { self.round },
            boundary_at: next_boundary,
        }
    }

    /// 跳到指定天（轮次不变，边界重置）。
    pub fn set_day(&self, day: u32, next_boundary: i64) -> Self {
        Self {
            day: clamp_day(day),
            round: self.round,
            boundary_at: next_boundary,
        }
    }
}

/// 把天数规整到 1..CYCLE_DAYS。
pub fn clamp_day(day: u32) -> u32 {
    day.clamp(1, CYCLE_DAYS)
}
