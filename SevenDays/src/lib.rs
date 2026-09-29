//! 七日阵营战 —— Rust 版天数系统。

mod announce;
mod commands;
mod config;
mod engine;
mod music;
mod state;
mod store;
mod time;

use pumpkin_plugin_api::events::EventPriority;
use pumpkin_plugin_api::events::player::player_join::PlayerJoinEvent;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use pumpkin_plugin_api::permissions;
use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::wit::pumpkin::plugin::event::PlayerJoinEventData;
use pumpkin_plugin_api::{Context, EventHandler, Plugin, PluginMetadata, Server};
use tracing::info;

use crate::config::{day_config, DAY_PERMISSION, LOG_PREFIX};
use crate::music::play_theme;

/// 每 20 tick（= 1 秒）检查一次时间边界。
const CHECK_PERIOD_TICKS: u64 = 20;

/// 玩家加入时播放当天主题曲。
struct JoinHandler;

impl EventHandler<PlayerJoinEvent> for JoinHandler {
    fn handle(&self, _server: Server, event: PlayerJoinEventData) -> PlayerJoinEventData {
        play_theme(&event.player, engine::get_state().day);
        event
    }
}

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
            // 状态存到插件私有数据目录，需要读写权限。
            permissions: vec![
                permissions::FS_READ_DATA.to_string(),
                permissions::FS_WRITE_DATA.to_string(),
            ],
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

        let state = engine::init(&context);
        commands::register(&context);

        // 官方调度 API（u64 原生，没有 TS 版那个 BigInt panic）。
        context.schedule_repeating_task(CHECK_PERIOD_TICKS, CHECK_PERIOD_TICKS, |server| {
            engine::tick(&server);
        });

        // 玩家加入时播放当天主题曲。
        context.register_event_handler(JoinHandler, EventPriority::Normal, true)?;

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
