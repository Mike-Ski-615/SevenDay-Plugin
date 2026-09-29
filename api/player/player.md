# Player API（玩家接口）

> WIT 模块：`player`（`wit/v0.1/player.wit`，含 Java / Bedrock 专属句柄）
> 命名说明：WIT kebab-case 名在 TS 绑定中为 camelCase。

---

## 1. 资源 `player`

通过 `server.get-player-by-name()` / `get-player-by-uuid()` / `get-all-players()` / 事件数据获得。

### 基础 / 状态
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `as-entity` | 无 | entity | 转为基础实体 |
| `get-id` | 无 | uuid | UUID |
| `get-name` | 无 | string | 用户名 |
| `get-position` | 无 | position | 坐标 |
| `get-yaw` / `get-pitch` | 无 | f32 | 水平/垂直朝向 |
| `get-world` | 无 | %world | 所在世界 |
| `get-gamemode` | 无 | game-mode | 游戏模式 |
| `set-gamemode(mode)` | game-mode | bool | 修改游戏模式，成功返回 true |
| `get-locale` | 无 | string | 语言（如 `en_us`） |
| `get-ping` | 无 | u32 | 延迟毫秒 |

### 权限 / 身份
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-permission-level` / `set-permission-level(level)` | permission-level | level / 无 | 权限等级 0–4 |
| `set-permission(node, value)` | 节点、布尔 | 无 | 显式授予/拒绝权限 |
| `unset-permission(node)` | 节点 | 无 | 移除显式设置 |
| `has-permission-set(node)` | 节点 | `Option<bool>` | true=授予 false=拒绝 none=未设置 |
| `has-permission(node)` | 节点 | bool | 是否有权限 |
| `get-display-name` / `set-display-name(name)` | text-component | text-component / 无 | 显示名 |
| `get-tab-list-name` / `set-tab-list-name(name)` | Option<text-component> | Option / 无 | tab 名称 |

### 消息 / 标题
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `send-system-message(text, overlay)` | 文本、是否动作栏 | 无 | 发系统消息 |
| `delete-message-by-signature(signature)` / `delete-message-by-id(id)` | 签名/ID | 无 | 删除聊天消息 |
| `set-tab-list-header-footer(header, footer)` | 两个文本 | 无 | tab 页眉页脚 |
| `set-tab-list-order(order)` / `set-tab-list-latency(latency)` / `set-tab-list-listed(listed)` | s32 / s32 / bool | 无 | tab 排序、延迟、是否可见 |
| `show-title(text)` / `show-subtitle(text)` / `show-actionbar(text)` | 文本 | 无 | 大标题/副标题/动作栏 |
| `send-title-animation(fade-in, stay, fade-out)` | 三个 s32 | 无 | 标题动画时长 |

### 摄像机 / 观战
`set-camera(Option<entity>)`、`set-camera-entity-id(entity-id: u32)`、`get-camera-entity-id() -> u32`、`reset-camera()`

### 音效 / 粒子
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `play-sound(sound, category, volume, pitch)` | 音效、分类、音量、音调 | 无 | 在当前位置播放 |
| `play-sound-at(pos, sound, category, volume, pitch)` | 坐标 + 同上 | 无 | 指定位置播放 |
| `stop-sound(Option<sound>, Option<sound-category>)` | 音效、分类 | 无 | 停止 |
| `play-custom-sound(name, category, volume, pitch)` / `play-custom-sound-at(pos, ...)` | 资源包音效名 | 无 | 自定义音效 |
| `stop-custom-sound(Option<string>, Option<sound-category>)` | 名称、分类 | 无 | 停止自定义音效 |
| `spawn-particles(particle, pos, count, offset, max-speed)` | 粒子、坐标、数量、偏移、速度 | 无 | 仅该玩家可见的粒子 |

### 视觉 / 客户端覆盖
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `send-block-change(pos, block-id)` | 位置、ID | 无 | 发送假方块（不改服务端） |
| `reset-block-change(pos)` | 位置 | 无 | 重置假方块 |
| `send-hurt-animation(yaw)` | 偏航 | 无 | 受伤画面抖动 |
| `open-book(hand)` | hand | 无 | 打开成书界面 |
| `open-sign-editor(pos, is-front-text)` | 位置、是否正面 | 无 | 强制打开告示牌编辑 |

### 物理 / 移动
`set-velocity(velocity)`、`apply-knockback(strength, x, z)`、`set-movement-locked(locked)` / `is-movement-locked()`、`set-freeze-ticks(ticks)` / `get-freeze-ticks()`、`set-server-links(links)`

### 传送 / 管理
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `teleport(position, Option<yaw>, Option<pitch>, %world)` | 坐标、朝向、世界 | 无 | 传送 |
| `teleport-world(world-ref, position, yaw, pitch)` | 世界 + 坐标 | 无 | 跨世界传送 |
| `respawn()` | 无 | 无 | 强制重生 |
| `open-gui(gui-ref)` | gui | 无 | 打开界面 |
| `ban(options: ban-player-options)` | 选项 | 无 | 封禁 |
| `ban-ip(options: ban-ip-options)` | 选项 | 无 | 封禁 IP |
| `transfer(host, port)` | 主机、端口 | 无 | 转移到其他服务器 |

### 状态效果
`add-effect(status-effect-instance)`、`remove-effect(status-effect-type)`、`clear-effects()`、`has-effect(type) -> bool`、`get-effect(type) -> Option<instance>`、`get-active-effects() -> instance[]`

### 生命 / 伤害
`heal(amount)`、`damage(amount, damage-type)`、`kill()`

### 统计
`get/set/increment-statistic(category, stat-id, value/amount)`、`get/set/increment-custom-statistic(stat, value/amount)`、`send-stats()`、`get-team() -> Option<string>`

### 冷却
`start-cooldown(group, duration-ticks)`、`get-cooldown(group) -> f32`、`is-on-cooldown(group) -> bool`

### 能力 / 飞行
`set-allow-flight(allowed)`、`set-fly-speed(speed)`、`set-walk-speed(speed)`、`set-invulnerable(invulnerable)`

### 背包
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-inventory()` | 无 | player-inventory | 玩家背包（含盔甲/装备） |
| `get-ender-chest()` | 无 | inventory | 末影箱 |
| `get-selected-slot()` | 无 | u8 | 当前快捷栏槽 0–8 |
| `get-item-in-hand(hand)` / `set-item-in-hand(hand, stack)` | hand、物品 | Option<item-stack> / 无 | 手上物品 |
| `get-inventory-item(slot)` / `set-inventory-item(slot, stack)` | 槽位、物品 | Option / 无 | 指定槽物品 |
| `get-ender-chest-item(slot)` / `set-ender-chest-item(slot, stack)` | 槽 0–26、物品 | Option / 无 | 末影箱物品 |
| `clear-ender-chest()` / `open-ender-chest()` | 无 | 无 | 清空/打开末影箱 |

### 身体状态
生命：`get/set-health`、`get/set-max-health`
饥饿：`get/set-food-level`（0–20）、`get/set-saturation`、`get/set-exhaustion`、`get/set-absorption`
经验：`get/set-experience-level`、`get/set-experience-progress`、`get/set-experience-points`、`add-experience-levels`、`add-experience-points`
飞行：`is-flying()` / `set-flying(bool)`
能力对象：`get-abilities() -> player-abilities` / `set-abilities(abilities)`

### 皮肤 / IP
`get-ip() -> string`、`get-skin() -> Option<player-skin>`、`set-skin(skin)`（仅更新字段，不重载）、`get-skin-parts() -> skin-parts` / `set-skin-parts(parts)`

### 逐玩家环境覆盖
`set-player-time(time, relative)` / `reset-player-time()` / `get-player-time() -> Option<u64>` / `is-player-time-relative()`
`set-player-weather(player-weather)` / `reset-player-weather()` / `get-player-weather() -> Option<player-weather>`
`set-compass-target(pos)` / `get-compass-target() -> position`
`set-respawn-location(pos)` / `get-respawn-location() -> Option<position>`

### 隐身 / 可见性
`hide-player(other)`、`show-player(other)`、`can-see(other) -> bool`、`can-see-player(other) -> bool`、`set-tab-list-ping(latency-ms)`

### 物品冷却
`set-item-cooldown(item-id, ticks)`、`get-item-cooldown(item-id) -> Option<s32>`、`has-item-cooldown(item-id) -> bool`

### 射线 / 目标 / 投掷
`ray-trace-block(max-distance, include-fluids)`、`ray-trace-entity(max-distance)`、`get-target-entity(max-distance)`、`get-target-block(max-distance: u32) -> Option<position>`、`get-target-block-exact(max-distance, include-fluids)`、`launch-projectile(projectile-type) -> Option<entity>`

### 成就
`get-advancement-progress(id) -> Option<advancement-progress>`、`award/revoke-advancement-criterion(id, criterion) -> bool`、`award/revoke-advancement(id) -> bool`、`has-advancement(id) -> bool`、`get-completed-advancements() -> string[]`、`get/set-selected-advancement-tab(Option<string>)`

### 平台分派
`as-java() -> Option<java-player>`、`as-bedrock() -> Option<bedrock-player>`

---

## 2. Java 专属 `java-player`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-version` | 无 | java-minecraft-version | 客户端 MC 版本 |
| `get-brand` | 无 | string | 客户端品牌（vanilla/fabric…） |
| `get-server-address` | 无 | string | 连接用的服务器地址 |
| `get-settings` | 无 | java-player-settings | 客户端设置 |
| `send-packet(packet)` | java clientbound 包 | 无 | 发送 Java 数据包 |
| `send-custom-payload(channel, data)` | 频道、字节 | 无 | 发插件消息 |
| `show-dialog(dialog)` / `clear-dialog()` | dialog | 无 | 显示/清除对话框 |
| `get-scoreboard()` / `reset-scoreboard()` | 无 | scoreboard / 无 | Java 记分板 |
| `send-resource-pack(pack)` / `remove-resource-pack(id)` / `clear-resource-packs()` | 资源包/ID | 无 | 资源包管理 |
| `kick(options: java-kick-options)` | 选项 | 无 | 踢出 |
| `send-game-event(event, value)` | client-game-event、f32 | 无 | 发送 GameEvent 包 |
| `send-entity-status(entity-id, status)` | 实体 ID、状态 | 无 | 发送实体动画事件 |

---

## 3. Bedrock 专属 `bedrock-player`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-version` | 无 | bedrock-minecraft-version | 客户端版本 |
| `get-settings` | 无 | bedrock-player-settings | 客户端设置 |
| `get-ability(ability)` / `set-ability(ability, value)` | bedrock-ability | bool / 无 | Bedrock 专属能力 |
| `get-status-flag(flag)` / `set-status-flag(flag, value)` | bedrock-status-flag | bool / 无 | 状态标记（着火/潜行等） |
| `send-packet(packet)` | bedrock clientbound 包 | 无 | 发送 Bedrock 数据包 |
| `open-form(form) -> u32` | 表单 | u32 | 打开自定义表单 |
| `get-scoreboard()` / `reset-scoreboard()` | 无 | bedrock-scoreboard / 无 | Bedrock 记分板 |
| `send-resource-packs-info(info)` | 资源包信息 | 无 | 发送资源包信息 |
| `kick(options: bedrock-kick-options)` | 选项 | 无 | 踢出 |

---

## 4. 顶层函数

- `get-world-players(world-ref) -> player[]` — 返回指定世界的所有玩家。

---

## 5. 记录

| 记录 | 字段 |
|---|---|
| `server-link` | `label: server-link-label`, `url: string` |
| `player-abilities` | `invulnerable, flying, allow-flying, creative, allow-modify-world, fly-speed: f32, walk-speed: f32` |
| `player-skin` | `value: string`（base64 纹理 JSON）, `signature: Option<string>` |
| `java-player-settings` | `locale, view-distance: u8, chat-mode, chat-colors, skin-parts, main-hand, text-filtering, server-listing` |
| `bedrock-player-settings` | `game-version, device-os, device-id, device-model, language-code, current/default-input-mode, ui-profile, gui-scale, is-editor-mode, max-view-distance, memory-tier, graphics-mode, playfab-id, client-random-id, platform-offline/id, skin-id, arm-size, is-persona/premium/trusted-skin` |
| `java-kick-options` | `reason: text-component, log-to-console: bool, teardown-policy` |
| `bedrock-kick-options` | `reason: bedrock-disconnect-reason, message, skip-message, filtered-message, log-to-console, teardown-policy` |
| `ban-player-options` | `reason: Option<text-component>, source: Option<string>, expires-at-utc: Option<string>, duration-seconds: Option<u64>, kick-if-online, log-to-console` |
| `ban-ip-options` | `reason, source, expires-at-utc, duration-seconds, kick-matching-players, log-to-console` |
| `java-resource-pack` | `id: uuid, url, hash, forced, prompt-message: Option<text-component>` |
| `bedrock-resource-pack-entry` | `id: uuid, version, size: u64, download-url, content-key, sub-pack-name, content-id, has-scripts, addon-pack, rtx-enabled` |
| `bedrock-resource-packs-info` | `required, has-addon-packs, has-scripts, is-vibrant-visuals-force-disabled, world-template-id, world-template-version, packs` |

---

## 6. 枚举

| 枚举 | 值 |
|---|---|
| `player-weather` | clear / downfall |
| `projectile-type` | arrow、snowball、egg、ender-pearl、splash-potion、fireball、small-fireball、trident、wind-charge |
| `client-game-event` | no-respawn-block-available … start-waiting-chunks（14 种） |
| `known-server-link` | bug-report、community-guidelines、support、status、feedback、community、website、forums、news、announcements |
| `server-link-label`（variant） | `known(known-server-link)` / `custom(text-component)` |
| `skin-parts`（flags） | cape、jacket、left-sleeve、right-sleeve、left-pants-leg、right-pants-leg、hat |
| `bedrock-ability` | build、mine、doors-and-switches、open-containers、attack-players 等 20 种 |
| `bedrock-status-flag` | on-fire、sneaking、riding、sprinting、invisible、in-love、baby 等 **124 种** |
| `bedrock-device-os` | android、ios、osx、windows-10、linux、unknown 等 16 种 |
| `bedrock-input-mode` | unknown / mouse / touch / game-pad / motion-controller |
| `bedrock-ui-profile` | classic / pocket / unknown |
| `bedrock-graphics-mode` | simple / fancy / ray-traced / unknown |
| `chat-mode` | enabled / commands-only / hidden |
| `socket-teardown-policy` | graceful / immediate-close / drop-connection |
| `bedrock-disconnect-reason` | unknown、kicked、timeout、server-full 等 **148 种** |
| `java-minecraft-version` | v-1-7-2 … v-1-26-3、unknown（53 种） |
| `bedrock-minecraft-version` | v-1-21、v-1-26-30、unknown |
