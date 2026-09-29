use crate::announce::announce;
use crate::config::LOG_PREFIX;
use crate::music::{play_theme, silence_vanilla_music};
use crate::state::{resume, DayState};
use crate::store;
use crate::time::{format_local, next_boundary_ms, now_ms};
use pumpkin_plugin_api::{Context, Server};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Mutex;
use tracing::info;
static STATE: Mutex<DayState> = Mutex::new(DayState {
    day: 1,
    round: 1,
    boundary_at: 0,
});
static DATA_DIR: Mutex<String> = Mutex::new(String::new());
static PENDING_MUSIC: Mutex<Vec<String>> = Mutex::new(Vec::new());
static MUSIC_SILENCE_UNTIL: AtomicI64 = AtomicI64::new(0);
const THEME_SILENCE_MS: i64 = 360_000;
fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
fn arm_music_silence() {
    MUSIC_SILENCE_UNTIL.store(now_ms() + THEME_SILENCE_MS, Ordering::Relaxed);
}
fn enforce_music_silence(server: &Server) {
    if now_ms() >= MUSIC_SILENCE_UNTIL.load(Ordering::Relaxed) {
        return;
    }
    for player in server.get_all_players() {
        silence_vanilla_music(&player);
    }
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
        arm_music_silence();
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
    let now = now_ms();
    let next = match store::read_file(&dir) {
        Some(saved) => resume(saved, now),
        None => DayState::fresh(next_boundary_ms(now)),
    };
    set_state(next);
    persist();
    info!(
        "{} 初始化：第 {} 天（第 {} 轮），切换时刻 {}",
        LOG_PREFIX,
        next.day,
        next.round,
        format_local(next.boundary_at)
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
    if names.is_empty() {
        return;
    }
    for name in names {
        if let Some(player) = server.get_player_by_name(&name) {
            play_theme(&player, get_state().day);
        }
    }
    arm_music_silence();
}
pub fn tick(server: &Server) {
    drain_pending_music(server);
    enforce_music_silence(server);
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
