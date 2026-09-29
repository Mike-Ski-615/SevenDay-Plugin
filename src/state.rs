use crate::time::next_boundary_ms;

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

pub fn resume(mut saved: DayState, now: i64) -> DayState {
    while saved.boundary_at <= now {
        saved = saved.advance(next_boundary_ms(saved.boundary_at));
    }
    saved
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::next_boundary_ms;

    #[test]
    fn resume_skips_elapsed_boundaries() {
        let b0 = next_boundary_ms(0);
        let day = 86_400_000_i64;
        let saved = DayState {
            day: 6,
            round: 1,
            boundary_at: b0,
        };
        assert_eq!(
            resume(saved, b0 + 3 * day + 1),
            DayState {
                day: 3,
                round: 2,
                boundary_at: b0 + 4 * day,
            }
        );
    }
}
