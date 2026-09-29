//! `/day` 命令树（仅服主）。

use pumpkin_plugin_api::command::{CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::{Command, CommandHandler};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};

use crate::config::{day_config, DAYS, DAY_PERMISSION};
use crate::engine;
use crate::time::format_local;

fn status_text() -> String {
    let state = engine::get_state();
    let cfg = day_config(state.day);
    format!(
        "第 {} 轮 · 第 {} 天（{} · {}），下一日 {}",
        state.round,
        state.day,
        cfg.weekday,
        cfg.name,
        format_local(state.boundary_at)
    )
}

/// `/day` 与 `/day status`：查看当前状态。
pub struct StatusHandler;

impl CommandHandler for StatusHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        sender.send_message(TextComponent::text(&status_text()));
        Ok(1)
    }
}

/// `/day next`
pub struct NextHandler;

impl CommandHandler for NextHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let state = engine::next_day(&server);
        let cfg = day_config(state.day);
        sender.send_message(TextComponent::text(&format!(
            "已进入第 {} 天（{} · {}），第 {} 轮",
            state.day, cfg.weekday, cfg.name, state.round
        )));
        Ok(1)
    }
}

/// `/day <weekday>`：跳到指定天。
pub struct SetDayHandler(pub u32);

impl CommandHandler for SetDayHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let state = engine::set_day(&server, self.0);
        let cfg = day_config(state.day);
        sender.send_message(TextComponent::text(&format!(
            "已跳到第 {} 天（{} · {}）",
            state.day, cfg.weekday, cfg.name
        )));
        Ok(1)
    }
}

/// 构建并注册 `/day` 命令树。
pub fn register(context: &Context) {
    let mut command = Command::new(
        &["day".to_string()],
        "查看或控制七日战争天数（仅服主）",
    )
    .execute(StatusHandler);

    command = command.then(CommandNode::literal("status").execute(StatusHandler));
    command = command.then(CommandNode::literal("next").execute(NextHandler));

    // 每周一天一个字面量节点：/day mon … /day sun
    for (index, cfg) in DAYS.iter().enumerate() {
        let name = cfg.weekday.to_lowercase();
        command = command.then(CommandNode::literal(&name).execute(SetDayHandler(index as u32 + 1)));
    }

    context.register_command(command, DAY_PERMISSION);
}
