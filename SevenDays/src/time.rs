const MS_PER_MINUTE: i64 = 60_000;
const MS_PER_DAY: i64 = 86_400_000;
pub const SWITCH_HOUR: i64 = 8;
pub const TZ_OFFSET_MINUTES: i64 = 8 * 60;
pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
pub fn next_boundary_ms(from_ms: i64) -> i64 {
    let offset = TZ_OFFSET_MINUTES * MS_PER_MINUTE;
    let local = from_ms + offset;
    let mut boundary_local = local.div_euclid(MS_PER_DAY) * MS_PER_DAY + SWITCH_HOUR * 3_600_000;
    if boundary_local <= local {
        boundary_local += MS_PER_DAY;
    }
    boundary_local - offset
}
pub fn format_local(ms: i64) -> String {
    let shifted = ms + TZ_OFFSET_MINUTES * MS_PER_MINUTE;
    let days = shifted.div_euclid(MS_PER_DAY);
    let secs = shifted.rem_euclid(MS_PER_DAY) / 1000;
    let (hour, minute) = (secs / 3600, (secs % 3600) / 60);
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}")
}
fn civil_from_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}
