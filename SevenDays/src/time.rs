//! 时间计算。
//!
//! 全部基于**真实墙钟**（epoch 毫秒），与游戏 tick 完全无关：
//! 切天的绝对时刻由 `next_boundary_ms` 决定，tick 只负责"多久检查一次"。
//!
//! 本模块刻意不依赖插件 API，纯函数，方便单测。

const MS_PER_MINUTE: i64 = 60_000;
const MS_PER_DAY: i64 = 86_400_000;

/// 每天切换到下一天的时刻（当地时间的整点）。
pub const SWITCH_HOUR: i64 = 8;

/// 世界时区偏移（分钟）。默认 UTC+8。
pub const TZ_OFFSET_MINUTES: i64 = 8 * 60;

/// 当前真实时间（epoch 毫秒）。WASI 提供真实时钟。
pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// 返回 `from_ms` 之后（不含）的下一个"当地 08:00"的 epoch 毫秒。
///
/// 时区按 [`TZ_OFFSET_MINUTES`] 计算，不依赖运行环境的 TZ 数据库。
pub fn next_boundary_ms(from_ms: i64) -> i64 {
    let offset = TZ_OFFSET_MINUTES * MS_PER_MINUTE;
    let local = from_ms + offset;
    let mut boundary_local = local.div_euclid(MS_PER_DAY) * MS_PER_DAY + SWITCH_HOUR * 3_600_000;
    if boundary_local <= local {
        boundary_local += MS_PER_DAY;
    }
    boundary_local - offset
}

/// 把 epoch 毫秒按配置时区格式化成 `YYYY-MM-DD HH:MM`。
pub fn format_local(ms: i64) -> String {
    let shifted = ms + TZ_OFFSET_MINUTES * MS_PER_MINUTE;
    let days = shifted.div_euclid(MS_PER_DAY);
    let secs = shifted.rem_euclid(MS_PER_DAY) / 1000;
    let (hour, minute) = (secs / 3600, (secs % 3600) / 60);
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}")
}

/// Howard Hinnant 的 `civil_from_days`：把「1970-01-01 起的天数」转成 (年, 月, 日)。
fn civil_from_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let year = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let day = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let month = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    (if month <= 2 { year + 1 } else { year }, month, day)
}
