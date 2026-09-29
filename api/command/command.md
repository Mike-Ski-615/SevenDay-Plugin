# Command API（命令接口）

> WIT 模块：`command`（`wit/v0.1/command.wit`）
> 命名说明：WIT kebab-case 名在 TS 绑定中为 camelCase。

---

## 1. 注册命令的流程

```ts
const cmd = new Command(["home", "h"], "传送回出生点");
cmd.then(CommandNode.literal("set"));           // 子命令
cmd.executeWithHandlerId(handlerId);
ctx.registerCommand(cmd, "myplugin.home");      // 需要权限节点可传，不需要传 ""
```

- 用 `Plugin.registerCommand(ctx, cmd, handler, permission)` 注册：内部会 `cmd.executeWithHandlerId(id)` 并调用 `ctx.registerCommand`。
- 命令树用 `command-node.then(node)` 挂子节点。

---

## 2. 资源 `command`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `constructor(names: string[], description: string)` | 名称列表（首个为主名，其余为别名）、描述 | — | 创建命令 |
| `then(node)` | command-node | 无 | 添加子节点 |
| `execute-with-handler-id(handler-id)` | u32 | 无 | 绑定执行处理器 |

---

## 3. 命令节点 `command-node`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `literal(name)` | 名称 | command-node | **静态**，字面量子命令 |
| `argument(name, type)` | 参数名、argument-type | command-node | **静态**，参数节点 |
| `then(node)` | 子节点 | 无 | 挂载子节点 |
| `execute-with-handler-id(handler-id)` | u32 | 无 | 该节点执行处理器 |
| `suggest-with-handler-id(handler-id)` | u32 | 无 | 补全处理器 |
| `require-with-handler-id(handler-id)` | u32 | 无 | 条件处理器 |

---

## 4. 命令发送者 `command-sender`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-command-sender-type()` | 无 | command-sender-type | 发送者类型 |
| `get-name()` | 无 | string | 名称 |
| `send-message(text)` | 文本 | 无 | 发送消息 |
| `send-system-message(text)` | 文本 | 无 | 系统消息 |
| `send-error(text)` | 文本 | 无 | 错误消息 |
| `set-success-count(count)` | s32 | 无 | 设置成功计数 |
| `is-player()` / `is-console()` | 无 | bool | 类型判断 |
| `as-player()` | 无 | Option<player> | 转为玩家 |
| `permission-level()` | 无 | permission-level | 权限等级 |
| `has-permission-level(level)` | 等级 | bool | 权限等级是否足够 |
| `has-permission(server, node)` | server、节点 | bool | 是否有权限节点 |
| `position()` / `%world()` | 无 | Option<position> / Option<%world> | 位置/世界 |
| `get-locale()` | 无 | locale | 语言 |
| `should-receive-feedback()` / `should-broadcast-console-to-ops()` / `should-track-output()` | 无 | bool | 反馈相关 |

### variant `command-sender-type`
`rcon`、`console`、`player(player)`、`command-block(tuple<command-block-entity, %world>)`、`dummy`

---

## 5. 命令参数 `consumed-args`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-value(key)` | 参数名 | `arg` | 取参数值（variant） |

### variant `arg`（常见取值）
`simple(string)`、`msg(string)`、`bool(bool)`、`item(string)`、`resource-location(string)`、`block(string)`、`time(s32)`、`num(result<number, not-in-bounds>)`、`block-pos(block-pos)`、`pos3d`、`pos2d`、`rotation`、`gamemode`、`difficulty`、`players(list<player>)`、`particle`、`text-component`、`bossbar-color/style`、`sound-category`、`damage-type`、`effect`、`enchantment`、`advancement`、`entity-anchor`

---

## 6. 参数类型 `argument-type`

variant，可传以下之一：

- 基础：`bool`、`float(min,max)`、`double(min,max)`、`integer(min,max)`、`long(min,max)`、`string(string-type)`
- 实体/玩家：`entities`、`entity`、`players`、`game-profile`
- 坐标：`block-pos`、`column-pos`、`position3d`、`position2d`
- 方块：`block-state`、`block-predicate`
- 物品：`item`、`item-predicate`
- 文本/样式：`color`、`component`、`style`、`message`
- NBT：`nbt-compound-tag`、`nbt-tag`、`nbt-path`
- 记分板：`objective`、`objective-criteria`、`operation`、`scoreboard-slot`、`score-holder(bool)`、`team`
- 其它：`particle`、`angle`、`rotation`、`swizzle`、`item-slot`、`resource-location`、`mob-effect`、`function`、`entity-anchor`、`int-range`、`float-range`、`dimension`、`gamemode`、`difficulty`、`time(Option<s32>)`、`resource(string)`、`resource-or-tag(string)`、`resource-or-tag-key(string)`、`resource-key(string)`、`template-mirror`、`template-rotation`、`uuid`

### 枚举
- `string-type`：single-word / quotable / greedy
- `suggestion-type`：ask-server / all-recipes / available-sounds / available-biomes / summonable-entities
- `permission-level`：zero / one / two / three / four
- `bossbar-color`：pink / blue / red / green / yellow / purple / white
- `bossbar-style`：no-division / notches6 / notches10 / notches12 / notches20
- `sound-category`：master / music / records / weather / blocks / hostile / neutral / players / ambient / voice
- `entity-anchor`：eyes / feet

### 辅助记录
- `entity-argument { single: bool, players-only: bool }`
- `number`（variant）：float64 / float32 / int32 / int64
- `not-in-bounds`（variant）：lower-bound / upper-bound

---

## 7. 补全

### `suggestion-request`
`input: string`、`cursor: u32`、`start: u32`、`remaining: string`

### `command-suggestion`
`value: string`、`tooltip: Option<text-component>`

### `command-suggestions`
`start: u32`、`length: u32`、`values: command-suggestion[]`

> 补全处理器签名：`(sender, srv, request) => command-suggestions`，通过
> `Plugin.registerCommandSuggestionHandler(handler)` 注册，再在节点上 `suggest-with-handler-id(id)`。

---

## 8. 错误 `command-error`

variant：
- `invalid-consumption(Option<string>)` — 参数消费无效
- `invalid-requirement` — 条件不满足
- `permission-denied` — 无权限
- `command-failed(text-component)` — 执行失败，带原因

> 命令处理器返回数字结果码（成功计数）；返回错误时可抛出/转换为 `command-error`。
