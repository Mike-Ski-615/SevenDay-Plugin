# Server API（服务器接口）

> WIT 模块：`server`（`wit/v0.1/server.wit`）
> 获取方式：`ctx.getServer()`，即 TS 中的 `Server` 实例。

服务器全局实例，以及 OP / 封禁 / 白名单三个管理器。

---

## 1. 资源 `server`

### 系统与状态

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-sys-info()` | 无 | `sys-info` | 系统信息（CPU/内存/OS）；字段仅在拥有对应权限时才有值 |
| `get-difficulty()` | 无 | `difficulty` | 当前难度 |
| `get-player-count()` | 无 | `u32` | 在线玩家总数 |
| `get-mspt()` | 无 | `f64` | 平均每 tick 毫秒数，<50 表示稳定 20 TPS |
| `get-tps()` | 无 | `f64` | 有效 TPS，理想 20.0 |
| `get-max-players()` | 无 | `u32` | 最大玩家数 |
| `is-hardcore()` | 无 | `bool` | 是否极限模式 |
| `is-online-mode()` | 无 | `bool` | 是否正版验证 |
| `get-motd()` | 无 | `string` | 服务器 MOTD |
| `has-whitelist()` | 无 | `bool` | 是否启用白名单 |
| `get-allow-nether()` | 无 | `bool` | 是否允许下界 |
| `get-allow-end()` | 无 | `bool` | 是否允许末地 |
| `get-view-distance()` | 无 | `u8` | 最大视距 |
| `get-simulation-distance()` | 无 | `u8` | 最大模拟距离 |
| `get-default-gamemode()` | 无 | `game-mode` | 默认游戏模式 |

### 玩家查询

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-all-players()` | 无 | `Player[]` | 所有世界在线玩家 |
| `get-player-by-name(name)` | 玩家名 | `Option<Player>` | 按名字精确查找 |
| `get-player-by-uuid(id)` | uuid | `Option<Player>` | 按 UUID 查找 |
| `get-players-in-world(world-ref)` | 世界 | `Player[]` | 指定世界在线玩家 |
| `get-player-count-in-world(world-ref)` | 世界 | `u32` | 指定世界在线玩家数 |

### 世界管理

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-all-worlds()` | 无 | `World[]` | 所有世界 |
| `get-world-by-name(name)` | 维度名或世界名 | `Option<World>` | 按名查世界 |
| `has-world(name)` | 名称 | `bool` | 世界是否存在 |
| `create-world(name, dimension)` | 名称、维度 | `World` | 创建/加载世界，已存在则返回它 |
| `unload-world(name)` | 名称 | `Result<_, string>` | 卸载并保存；主世界/有玩家时失败 |

### 广播与聊天

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `broadcast(message)` | 字符串 | 无 | 向所有玩家发送系统消息 |
| `broadcast-tab-list-header-footer(header, footer)` | 两个文本组件 | 无 | 设置所有人 tab 列表页眉/页脚 |
| `delete-message-by-signature(signature)` | 256 字节签名 | 无 | 按签名删除聊天消息 |
| `delete-message-by-id(signature-id: s32)` | 签名缓存 ID | 无 | 按 ID 删除聊天消息 |

### 命令与其它

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `execute-command(command, sender)` | 命令串、命令发送者 | 无 | 以指定身份执行命令 |
| `save-all()` | 无 | `Result<_, string>` | 保存所有玩家/进度/已加载世界 |
| `set-server-links(links)` | `server-link[]` | 无 | 广播自定义服务器链接（1.21+ 暂停菜单） |

### 子系统获取

| 方法 | 返回 | 说明 |
|---|---|---|
| `get-recipe-manager()` | `recipe-manager` | 注册自定义配方 |
| `get-op-manager()` | `op-manager` | OP 管理 |
| `get-ban-manager()` | `ban-manager` | 封禁管理 |
| `get-whitelist-manager()` | `whitelist-manager` | 白名单管理 |
| `get-enchantment-manager()` | `enchantment-manager` | 自定义附魔管理 |
| `get-datapack-manager()` | `datapack-manager` | 数据包管理 |

### 成就 / 附魔查询

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-advancement(id)` | 成就 ID | `Option<advancement-info>` | 查询成就信息 |
| `get-all-advancement-ids()` | 无 | `string[]` | 全部成就 ID |
| `get-enchantment(id)` | 附魔 ID | `Option<custom-enchantment>` | 查附魔（原版或自定义） |
| `get-all-enchantment-ids()` | 无 | `string[]` | 全部附魔 ID |

---

## 2. 操作员管理 `op-manager`

### 记录 `op-entry`
`uuid`、`name`、`level: permission-level`、`bypasses-player-limit: bool`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `is-op(id)` | uuid | `bool` | 是否为 OP |
| `get-op(id)` | uuid | `Option<op-entry>` | 取 OP 条目 |
| `get-permission-level(id)` | uuid | `permission-level` | 有效权限等级（非 OP 为 zero） |
| `op-player(name, id, level, bypasses-player-limit)` | 名称、uuid、等级、是否越玩家上限 | 无 | 设为 OP（在线/离线均可） |
| `deop-player(id)` | uuid | `bool` | 取消 OP，原为 OP 返回 true |
| `list-ops()` | 无 | `op-entry[]` | 全部 OP |

---

## 3. 封禁管理 `ban-manager`

### 记录
- `banned-player-entry { uuid, name, created, source, expires: Option<string>, reason }`
- `banned-ip-entry { ip, created, source, expires: Option<string>, reason }`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `is-player-banned(id)` | uuid | `bool` | 玩家是否被封 |
| `get-player-ban(id)` | uuid | `Option<banned-player-entry>` | 取玩家封禁条目 |
| `ban-player(name, id, options: ban-player-options)` | 名称、uuid、选项 | 无 | 封禁玩家 |
| `unban-player(id)` | uuid | `bool` | 解封，原本被封返回 true |
| `list-player-bans()` | 无 | `banned-player-entry[]` | 全部玩家封禁 |
| `is-ip-banned(ip)` | IP 串 | `bool` | IP 是否被封 |
| `get-ip-ban(ip)` | IP 串 | `Option<banned-ip-entry>` | 取 IP 封禁条目 |
| `ban-ip(ip, options: ban-ip-options)` | IP、选项 | 无 | 封禁 IP |
| `unban-ip(ip)` | IP 串 | `bool` | 解封 IP |
| `list-ip-bans()` | 无 | `banned-ip-entry[]` | 全部 IP 封禁 |

---

## 4. 白名单管理 `whitelist-manager`

### 记录 `whitelist-entry { uuid, name }`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `is-enabled()` | 无 | `bool` | 白名单是否启用 |
| `set-enabled(enabled: bool)` | 布尔 | 无 | 启用/禁用；启用且强制时踢出不在名单者 |
| `is-whitelisted(id)` | uuid | `bool` | 玩家是否在白名单 |
| `add-player(name, id)` | 名称、uuid | `bool` | 加入白名单，新加入返回 true |
| `remove-player(id)` | uuid | `bool` | 移除，原本在名单返回 true |
| `list-entries()` | 无 | `whitelist-entry[]` | 全部白名单条目 |

---

## 5. 相关数据类型

- `enum difficulty`：`peaceful`（和平）/ `easy` / `normal` / `hard`
- `enum dimension`：`overworld` / `nether` / `end`
- `record sys-info`：`cpu-count: Option<u32>`、`total-memory: Option<u64>`、`used-memory: Option<u64>`、`os-name: Option<string>`、`os-version: Option<string>`、`pumpkin-version: string`
- `variant command-sender`：`console`（控制台）或 `player(player)`（玩家）
