use crate::config::{day_config, DAYS, DAY_PERMISSION};
use crate::engine;
use crate::time::format_local;
use pumpkin_plugin_api::command::{CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::{Command, CommandHandler};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};
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
pub fn register(context: &Context) {
    let mut command = Command::new(&["day".to_string()], "查看或控制七日战争天数（仅服主）")
        .execute(StatusHandler);
    command = command.then(CommandNode::literal("status").execute(StatusHandler));
    command = command.then(CommandNode::literal("next").execute(NextHandler));
    for (index, cfg) in DAYS.iter().enumerate() {
        let name = cfg.weekday.to_lowercase();
        command =
            command.then(CommandNode::literal(&name).execute(SetDayHandler(index as u32 + 1)));
    }
    context.register_command(command, DAY_PERMISSION);
}
