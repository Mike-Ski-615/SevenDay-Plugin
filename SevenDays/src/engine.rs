use std::sync::Mutex;
use pumpkin_plugin_api::{Context, Server};
use tracing::info;
use crate::announce::announce;
use crate::config::LOG_PREFIX;
use crate::music::play_theme;
use crate::state::DayState;
use crate::store;
use crate::time::{format_local, next_boundary_ms, now_ms};
static STATE: Mutex<DayState> = Mutex::new(DayState {
    day: 1,
    round: 1,
    boundary_at: 0,
});
static DATA_DIR: Mutex<String> = Mutex::new(String::new());
static PENDING_MUSIC: Mutex<Vec<String>> = Mutex::new(Vec::new());
fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
pub fn get_state() -> DayState {
    *lock(&STATE)
}
fn set_state(next: DayState) {
    *lock(&STATE) = next;
}
fn data_dir() -> String {
    lock(&DATA_DIR).clone()
}
fn persist() {
    let dir = data_dir();
    if dir.is_empty() {
        return;
    }
    if let Err(e) = store::write_file(&dir, &get_state()) {
        info!("{} 状态写入失败: {e}", LOG_PREFIX);
    }
}
fn commit(previous: DayState, next: DayState, server: Option<&Server>) {
    set_state(next);
    persist();
    if let Some(srv) = server {
        announce(srv, &next);
        for player in srv.get_all_players() {
            play_theme(&player, next.day);
        }
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
pub fn init(ctx: &Context) -> DayState {
    let dir = ctx.get_data_folder();
    *lock(&DATA_DIR) = dir.clone();
    let previous = store::read_file(&dir);
    let next = DayState::fresh(next_boundary_ms(now_ms()));
    set_state(next);
    persist();
    info!(
        "{} 初始化：第 1 天（第 1 轮），切换时刻 {}（读取到旧存档：{}，已按规则重置）",
        LOG_PREFIX,
        format_local(next.boundary_at),
        previous.map_or_else(|| "无".to_string(), |p| format!("第 {} 天", p.day))
    );
    next
}
pub fn enqueue_music(player_name: String) {
    lock(&PENDING_MUSIC).push(player_name);
}
fn drain_pending_music(server: &Server) {
    let names: Vec<String> = {
        let mut pending = lock(&PENDING_MUSIC);
        std::mem::take(&mut *pending)
    };
    for name in names {
        if let Some(player) = server.get_player_by_name(&name) {
            play_theme(&player, get_state().day);
        }
    }
}
pub fn tick(server: &Server) {
    drain_pending_music(server);
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
