mod announce;
mod commands;
mod config;
mod engine;
mod state;
mod time;
use crate::config::{DAY_PERMISSION, LOG_PREFIX, day_config};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::info;
const CHECK_PERIOD_TICKS: u64 = 20;
struct SevenDayWar;
impl Plugin for SevenDayWar {
    fn new() -> Self {
        SevenDayWar
    }
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "seven_day_war".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["SevenDay".into()],
            description: "七日阵营战 —— 天数系统（Rust 版）".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }
    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("{} Rust 版加载中…", LOG_PREFIX);
        context.register_permission(&Permission {
            node: DAY_PERMISSION.to_string(),
            description: "允许使用 /day 查看与控制七日战争天数".to_string(),
            default: PermissionDefault::Op(PermissionLevel::Four),
            children: Vec::new(),
        })?;
        let state = engine::init();
        commands::register(&context);
        context.schedule_repeating_task(CHECK_PERIOD_TICKS, CHECK_PERIOD_TICKS, |server| {
            engine::tick(&server);
        });
        let cfg = day_config(state.day);
        info!(
            "{} 就绪：第 {} 天（{} · {}），下一日边界 {}",
            LOG_PREFIX,
            state.day,
            cfg.weekday,
            cfg.name,
            time::format_local(state.boundary_at)
        );
        Ok(())
    }
}
pumpkin_plugin_api::register_plugin!(SevenDayWar);
