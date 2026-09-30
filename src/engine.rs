use crate::announce::announce;
use crate::config::LOG_PREFIX;
use crate::state::DayState;
use crate::time::{format_local, next_boundary_ms, now_ms};
use pumpkin_plugin_api::Server;
use std::sync::Mutex;
use tracing::info;
static STATE: Mutex<DayState> = Mutex::new(DayState {
    day: 1,
    round: 1,
    boundary_at: 0,
});
fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
pub fn get_state() -> DayState {
    *lock(&STATE)
}
fn set_state(next: DayState) {
    *lock(&STATE) = next;
}
fn commit(previous: DayState, next: DayState, server: Option<&Server>) {
    set_state(next);
    if let Some(srv) = server {
        announce(srv, &next);
    }
    info!(
        "{} 切换：第 {} 天 → 第 {} 天（第 {} 轮），下一日边界 {}",
        LOG_PREFIX,
        previous.day,
        next.day,
        next.round,
        format_local(next.boundary_at)
    );
}
pub fn init() -> DayState {
    let next = DayState::fresh(next_boundary_ms(now_ms()));
    set_state(next);
    info!(
        "{} 初始化：第 {} 天（第 {} 轮），切换时刻 {}",
        LOG_PREFIX,
        next.day,
        next.round,
        format_local(next.boundary_at)
    );
    next
}
pub fn tick(server: &Server) {
    let current = get_state();
    if now_ms() < current.boundary_at {
        return;
    }
    let next = current.advance(next_boundary_ms(now_ms()));
    commit(current, next, Some(server));
}
pub fn set_day(server: &Server, day: u32) -> DayState {
    let current = get_state();
    let next = current.set_day(day, next_boundary_ms(now_ms()));
    commit(current, next, Some(server));
    next
}
pub fn next_day(server: &Server) -> DayState {
    let current = get_state();
    let next = current.advance(next_boundary_ms(now_ms()));
    commit(current, next, Some(server));
    next
}
