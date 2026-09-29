# Core API（核心接口）

> 覆盖 WIT 模块：`plugin`（world 入口）、`context`、`metadata`、`logging`、`scheduler`、`i18n`、`ipc`、`uuid`、`common`、`permission`
> 包来源：`@pumpkinmc/pumpkin-api-ts`（`wit/v0.1/*.wit`）

---

## 1. 插件入口（`dist/index.ts` 封装类）

TypeScript 插件直接继承 `Plugin`，然后 `registerPlugin(new MyPlugin())`。

### `abstract class Plugin`

| 成员 | 签名 | 说明 | 返回 |
|---|---|---|---|
| `metadata` | `abstract metadata(): PluginMetadata` | **必须实现**，向宿主返回插件元数据 | 元数据对象 |
| `onLoad` | `onLoad(ctx: Context): void` | 插件加载时调用，注册事件/命令等 | 无 |
| `onUnload` | `onUnload(ctx: Context): void` | 插件卸载时调用，保存数据/清理资源 | 无 |
| `handleIpcMessage` | `handleIpcMessage(sender: PluginId, message: IpcMessage): IpcMessage` | 处理其他插件的 IPC 消息，默认抛异常表示不支持 | 回复消息 |
| `registerEvent` | `registerEvent(ctx, eventType, handler, priority="normal", blocking=true): void` | 注册事件监听 | 无 |
| `registerCommand` | `registerCommand(ctx, cmd, handler, permission=""): void` | 注册命令及执行处理器 | 无 |
| `registerCommandSuggestionHandler` | `registerCommandSuggestionHandler(handler): number` | 注册命令补全处理器，返回 handlerId | handlerId |
| `scheduleDelayedTask` | `scheduleDelayedTask(delayTicks, handler): number` | 延时任务（20 tick = 1 秒） | taskId |
| `scheduleRepeatingTask` | `scheduleRepeatingTask(delayTicks, periodTicks, handler): number` | 重复任务 | taskId |
| `registerAiGoal` | `registerAiGoal(goal: AiGoal): number` | 注册自定义 AI 目标 | goalId |
| `registerChunkGenerator` | `registerChunkGenerator(generator: ChunkGenerator): number` | 注册区块生成器 | generatorId |

### 顶层函数与类型

- `registerPlugin(plugin: Plugin): void` — 把插件实例注册给宿主（必须调用一次）。
- `type EventHandler<T> = (srv: Server, evt: T) => T | void`
- `type CommandHandler = (sender: CommandSender, srv: Server, args: ConsumedArgs) => number` — 返回命令执行结果码。
- `type CommandSuggestionHandler = (sender, srv, request) => CommandSuggestions`
- `type TaskHandler = (srv: Server) => void`
- `type ChunkGenerator = (phase: GenerationPhase, chunk: ChunkBuffer) => void`
- `interface AiGoal { canStart; shouldContinue; start; tick; stop }`（均由宿主回调）
- 导出给宿主的函数：`initPlugin`、`onLoad`、`onUnload`、`handleEvent`、`handleCommand`、`handleCommandSuggestion`、`handleTask`、`handleIpcMessage`、`handleAiGoalCanStart/ShouldContinue/Start/Tick/Stop`、`handleGeneratePhase`、`metadata.getMetadata`、`common`。
  > 这些**不是给插件调用的**，是 WASM 宿主反向调用插件的入口，普通插件无需关心。

---

## 2. 插件上下文 `context`

插件运行时句柄，通过 `onLoad(ctx)`、事件处理器参数等获得。

### 资源 `context`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `register-event` | `handler-id: u32, event-type, event-priority, blocking: bool` | 无 | 注册事件处理器 |
| `register-command` | `command: Command, permission: string` | 无 | 注册命令，`permission` 为所需权限节点 |
| `register-permission` | `permission: Permission` | `Result<_, string>` | 注册权限节点，失败返回错误串 |
| `get-data-folder` | 无 | `string` | 插件私有数据目录路径 |
| `get-server` | 无 | `Server` | 全局服务器实例 |
| `get-marketplace-metadata` | 无 | `Option<MarketplaceMetadata>` | 已签名插件返回市场元数据，未签名返回 none |

### 记录 `marketplace-metadata`

字段：`marketplace-url`、`plugin-id: s64`、`plugin-name`、`version`、`dev-id: s64`、`dev-name`、`is-paid: bool`、`user-id: s64`、`license-key: Option<string>`、`issued-at`（ISO-8601）。

---

## 3. 插件元数据 `metadata`

### 记录 `plugin-metadata`（TS 中即 `PluginMetadata`）

| 字段 | 类型 | 说明 |
|---|---|---|
| `name` | string | 插件名 |
| `version` | string | 版本（semver） |
| `authors` | string[] | 作者列表 |
| `description` | string | 简介 |
| `dependencies` | string[] | 依赖插件 |
| `permissions` | string[] | 申请的权限 |

### 函数

- `get-metadata(): plugin-metadata` — 宿主获取元数据。**插件一般通过实现 `metadata()` 间接提供，不需直接调用。**

---

## 4. 日志 `logging`

### 枚举 `level`：`trace` / `debug` / `info` / `warn` / `error`

| 函数 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `log(level, message)` | 级别 + 文本 | 无 | 输出一条日志（TS 中 `logging.log("info", "...")`） |
| `log-tracing(event: list<u8>)` | tracing 序列化事件字节 | 无 | 供 tracing crate 使用，普通插件不用 |

---

## 5. 任务调度 `scheduler`

| 函数 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `schedule-delayed-task(handler-id: u32, delay-ticks: u64)` | 处理器 ID、延时 tick | `u32` taskId | 延时执行一次（20 tick = 1 秒） |
| `schedule-repeating-task(handler-id, delay-ticks, period-ticks)` | 处理器 ID、首次延时、周期 tick | `u32` taskId | 周期性重复执行 |
| `cancel-task(task-id: u32)` | taskId | 无 | 取消未完成的任务 |

> 通常用 `Plugin.scheduleDelayedTask/scheduleRepeatingTask` 包装，无需手写 handler-id。

---

## 6. 国际化 `i18n`

| 函数 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `translate(key: string, locale: locale)` | 翻译键（如 `pumpkin:my_key`）、语言 | `string` | 取翻译文本，缺失回退英文或键本身 |
| `load-translations(namespace, json, locale)` | 命名空间、JSON 扁平映射、语言 | 无 | 加载自定义翻译 |

`locale` 为 `common` 中的枚举（`zh-cn`、`en-us` 等，共 100+ 种）。

---

## 7. 插件间通信 `ipc`

- `type plugin-id = string`
- `type ipc-message = list<u8>`

| 函数 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `send-ipc-message(recipient: plugin-id, message: ipc-message)` | 目标插件 ID、字节消息 | `Result<Result<ipc-message, string>>` | 同步发送并等待回复；外层错误为发送失败，内层为对端处理错误 |

接收方通过 `Plugin.handleIpcMessage` 处理。

---

## 8. UUID `uuid`

- `record uuid { high: u64, low: u64 }`

| 函数 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `generate()` | 无 | `uuid` | 生成随机 v4 UUID |
| `parse(s: string)` | 字符串 | `Option<uuid>` | 解析，非法返回 none |
| `to-string(id: uuid)` | uuid | `string` | 转标准字符串形式 |

---

## 9. 公共类型 `common`

纯类型模块，无函数。

### 类型别名
- `raw-text-component = list<u8>`（**已废弃**，请改用 `text` 模块的 `text-component`）
- `position = tuple<f64, f64, f64>`

### 记录
- `nbt-entry { key: string, value: u32 }`
- `nbt-tree { root: u32, tags: list<nbt-tag> }`
- `nbt-tag`（variant）：byte/short/int/long/float/double/byte-array/string-tag/list-tag/compound/int-array/long-array
- `block-pos { x: s32, y: s32, z: s32 }`
- `rgb-color { r,g,b: u8 }`
- `argb-color { a,r,g,b: u8 }`

### 枚举
- `hand`：left / right
- `game-mode`：survival / creative / adventure / spectator
- `named-color`：Minecraft 16 种基础颜色（black、dark-blue … white）
- `click-type`：left / right / shift-left / shift-right / middle / drop / control-drop / double-click / number-key / unknown
- `entity-pose`：standing、fall-flying、sleeping、swimming … inhaling（18 种）
- `locale`：全部 Minecraft 支持语言（af-za … zh-cn、zh-hk、zh-tw 等）

---

## 10. 权限 `permission`

### 枚举 `permission-level`
| 值 | 对应角色 | 含义 |
|---|---|---|
| `zero` | normal | 普通玩家，基础命令 |
| `one` | moderator | 可越过出生点保护 |
| `two` | gamemaster | 更多命令，可用命令方块 |
| `three` | admin | 多人管理相关命令 |
| `four` | owner | 全部命令，含服务器管理 |

### 数据
- `permission-default`（variant）：`deny`（默认拒绝）/ `allow`（默认允许）/ `op(permission-level)`（默认仅 OP 允许）
- `permission-child { node: string, value: bool }`
- `permission { node: string, description: string, default: permission-default, children: list<permission-child> }`

> `permission` 记录通过 `context.register-permission` 注册。
