pub const CYCLE_DAYS: u32 = 7;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DayState {
    pub day: u32,
    pub round: u32,
    pub boundary_at: i64,
}
impl DayState {
    pub fn fresh(boundary_at: i64) -> Self {
        Self {
            day: 1,
            round: 1,
            boundary_at,
        }
    }
    pub fn advance(&self, next_boundary: i64) -> Self {
        let wrapped = self.day >= CYCLE_DAYS;
        Self {
            day: if wrapped { 1 } else { self.day + 1 },
            round: if wrapped { self.round + 1 } else { self.round },
            boundary_at: next_boundary,
        }
    }
    pub fn set_day(&self, day: u32, next_boundary: i64) -> Self {
        Self {
            day: clamp_day(day),
            round: self.round,
            boundary_at: next_boundary,
        }
    }
}
pub fn clamp_day(day: u32) -> u32 {
    day.clamp(1, CYCLE_DAYS)
}
