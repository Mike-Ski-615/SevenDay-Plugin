# Event API（事件接口）

> WIT 模块：`event`（`wit/v0.1/event.wit`）
> 命名说明：WIT kebab-case 在 TS 绑定中为 camelCase。事件处理器签名：`(srv: Server, evt: <EventData>) => <EventData> | void`。

---

## 1. 事件系统

### 注册
```ts
this.registerEvent(ctx, "player-join-event", (srv, evt: PlayerJoinEventData) => {
  // ...
});
```

- `Plugin.registerEvent(ctx, eventType, handler, priority="normal", blocking=true)`
- 也可底层：`ctx.registerEvent(handlerId, eventType, priority, blocking)`

### 优先级 `event-priority`
`highest` / `high` / `normal` / `low` / `lowest`

### 取消
事件数据普遍含 `cancelled: bool` 字段。将 `cancelled` 置为 `true` 并返回该事件即可取消；因为传入的是对象引用，直接改字段也会生效。

### 通用包 variant
- `serverbound-packet`：`java(...)` / `bedrock(...)` / `unknown`
- `clientbound-packet`：`java(...)` / `bedrock(...)` / `unknown`

### 通用枚举
- `player-fish-state`：fishing / caught-fish / caught-entity / in-ground / failed-attempt / reel-in / bite
- `entity-interaction-action`：interact / attack / interact-at
- `interact-action`：left-click-block / left-click-air / right-click-air / right-click-block

---

## 2. 玩家事件

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `player-join-event` | 玩家加入 | player, join-message, cancelled |
| `player-leave-event` | 玩家离开 | player, leave-message, cancelled |
| `player-login-event` | 玩家登录 | player, kick-message, cancelled |
| `player-chat-event` | 玩家聊天 | player, message, recipients, signature, cancelled |
| `player-command-send-event` | 发送命令 | player, command, cancelled |
| `player-permission-check-event` | 权限检查 | player, permission, permission-result |
| `player-move-event` | 移动 | player, from-position, to-position, cancelled |
| `player-teleport-event` | 传送 | player, from-position, to-position, cancelled |
| `player-change-world-event` | 切换世界 | player, previous-world, new-world, position, yaw, pitch, cancelled |
| `player-respawn-event` | 重生 | player, previous-world, respawned-world, position, yaw, pitch, alive |
| `player-exp-change-event` | 经验变化 | player, amount |
| `player-item-held-event` | 切换手持槽 | player, previous-slot, new-slot, cancelled |
| `player-changed-main-hand-event` | 切换主手 | player, main-hand |
| `player-gamemode-change-event` | 游戏模式变化 | player, previous-gamemode, new-gamemode, cancelled |
| `player-custom-payload-event` | 自定义载荷（插件消息） | player, channel, data |
| `player-fish-event` | 钓鱼 | player, caught-uuid, caught-type, hook-uuid, state, hand, exp-to-drop, cancelled |
| `player-egg-throw-event` | 投掷鸡蛋 | player, egg-uuid, hatching, num-hatches, hatching-type, cancelled |
| `player-interact-unknown-entity-event` | 与未知实体交互 | player, entity-id, action, cancelled |
| `player-interact-entity-event` | 与实体交互 | player, entity-id, action, sneaking, cancelled |
| `player-interact-event` | 与世界交互 | player, action, clicked-pos, block, cancelled |
| `player-toggle-sneak-event` | 潜行切换 | player, is-sneaking, cancelled |
| `player-toggle-flight-event` | 飞行切换 | player, is-flying, cancelled |
| `player-toggle-sprint-event` | 疾跑切换 | player, is-sprinting, cancelled |
| `player-death-event` | 玩家死亡 | player, death-message, dropped-exp, keep-inventory, cancelled |
| `player-item-consume-event` | 消耗物品 | player, item-name, cancelled |
| `player-item-damage-event` | 物品耐久损耗 | player, item-name, damage, cancelled |
| `player-drop-item-event` | 丢弃物品 | player, item-name, count, cancelled |
| `player-bed-enter-event` | 上床 | player, bed-pos, cancelled |
| `player-bed-leave-event` | 离床 | player, bed-pos |
| `player-bucket-empty-event` | 倒空桶 | player, block-pos, bucket, cancelled |
| `player-bucket-fill-event` | 装桶 | player, block-pos, bucket, cancelled |
| `async-player-chat-event` | 异步聊天 | player, message, format, cancelled |
| `async-player-pre-login-event` | 异步预登录 | player-name, player-uuid, ip-address, kick-message, cancelled |
| `player-pre-login-event` | 预登录 | player-name, player-uuid, ip-address, kick-message, cancelled |
| `player-advancement-done-event` | 完成成就 | player, advancement-id, cancelled |
| `player-animation-event` | 播放动画 | player, animation-type, cancelled |
| `player-armor-stand-manipulate-event` | 操作盔甲架 | player, armor-stand-id, slot, cancelled |
| `player-bucket-entity-event` | 用桶捕获实体 | player, entity-id, bucket-item, cancelled |
| `player-changed-world-event` | 世界已切换 | player, from-world, to-world, cancelled |
| `player-channel-event` | 注册频道 | player, channel, cancelled |
| `player-command-preprocess-event` | 命令预处理 | player, command, cancelled |
| `player-edit-book-event` | 编辑书 | player, slot, pages, title, signing, cancelled |
| `player-elytra-boost-event` | 鞘翅加速 | player, firework-id, cancelled |
| `player-exp-cooldown-change-event` | 经验冷却变化 | player, new-cooldown, cancelled |
| `player-harvest-block-event` | 收获方块 | player, block-pos, harvested-items, cancelled |
| `player-hide-entity-event` | 隐藏实体 | player, entity-id, cancelled |
| `player-item-break-event` | 物品损坏 | player, item-name |
| `player-item-mend-event` | 经验修补 | player, item-name, repair-amount, exp-consumed, cancelled |
| `player-kick-event` | 踢出玩家 | player, reason, cancelled |
| `player-leash-entity-event` | 拴绳实体 | player, entity-id, holder-id, cancelled |
| `player-level-change-event` | 等级变化 | player, old-level, new-level |
| `player-locale-change-event` | 语言变化 | player, new-locale, cancelled |
| `player-name-entity-event` | 命名实体 | player, entity-id, name, cancelled |
| `player-open-sign-event` | 打开告示牌 | player, block-pos, is-front, cancelled |
| `player-portal-event` | 穿越传送门 | player, from-pos, to-pos, cancelled |
| `player-riptide-event` | 激流冲撞 | player, item-name, cancelled |
| `player-shear-entity-event` | 剪羊毛 | player, entity-id, hand, cancelled |
| `player-show-entity-event` | 显示实体 | player, entity-id, cancelled |
| `player-spawn-change-event` | 出生点变化 | player, new-spawn, forced, cancelled |
| `player-statistic-increment-event` | 统计增加 | player, statistic-id, amount, cancelled |
| `player-swap-hands-event` | 交换双手 | player, cancelled |
| `player-take-lectern-book-event` | 取讲台书 | player, block-pos, book, cancelled |
| `player-unleash-entity-event` | 解开拴绳 | player, entity-id, cancelled |
| `player-velocity-event` | 速度变化 | player, velocity, cancelled |
| `player-input-event` | 输入 | player, input, cancelled |
| `player-interact-at-entity-event` | 对实体精确交互 | player, entity-id, clicked-x/y/z, hand, cancelled |
| `player-links-send-event` | 发送服务器链接 | player, links, cancelled |
| `player-pickup-arrow-event` | 拾取箭 | player, arrow-id, cancelled |
| `player-recipe-book-click-event` | 点击配方书 | player, recipe-id, make-all, cancelled |
| `player-recipe-book-settings-change-event` | 配方书设置变化 | player, book-type, is-open, is-filtering, cancelled |
| `player-recipe-discover-event` | 发现配方 | player, recipe-id, cancelled |
| `player-register-channel-event` | 注册频道 | player, channel, cancelled |
| `player-resource-pack-status-event` | 资源包状态 | player, pack-id, status, cancelled |
| `player-spawn-location-event` | 出生位置 | player, spawn-pos, cancelled |
| `player-unregister-channel-event` | 注销频道 | player, channel, cancelled |

---

## 3. 方块事件

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `block-redstone-event` | 红石更新 | target-world, state-id, block-pos, old-current, new-current, cancelled |
| `block-break-event` | 方块破坏 | player, block, block-pos, exp, should-drop, cancelled |
| `block-burn-event` | 方块烧毁 | igniting-block, block, cancelled |
| `block-can-build-event` | 可否放置 | block-to-build, buildable, player, block, cancelled |
| `block-grow-event` | 方块生长 | target-world, old-block, old-state-id, new-block, new-state-id, block-pos, cancelled |
| `block-place-event` | 方块放置 | player, block-placed, block-placed-against, block-pos, can-build, cancelled |
| `block-damage-event` | 方块受损 | player, block-pos, insta-break, cancelled |
| `block-ignite-event` | 方块点燃 | block-pos, cancelled |
| `block-from-to-event` | 方块移动 | from-pos, to-pos, cancelled |
| `block-form-event` | 方块形成 | block-pos, cancelled |
| `block-fade-event` | 方块消退 | block-pos, cancelled |
| `block-dispense-event` | 发射器发射 | block-pos, item-name, cancelled |
| `block-explode-event` | 方块爆炸 | block-pos, yield-rate, cancelled |
| `block-physics-event` | 方块物理更新 | block-pos, changed-pos, cancelled |
| `block-piston-extend-event` | 活塞伸出 | block-pos, direction, cancelled |
| `block-piston-retract-event` | 活塞收回 | block-pos, direction, cancelled |
| `note-play-event` | 音符盒播放 | block-pos, instrument, note, cancelled |
| `sign-change-event` | 告示牌内容变化 | player, block-pos, lines, cancelled |
| `sponge-absorb-event` | 海绵吸水 | block-pos, cancelled |
| `tnt-prime-event` | TNT 点燃 | block-pos, prime-reason, cancelled |
| `bell-resonate-event` | 钟共振 | block-pos, target-world, cancelled |
| `bell-ring-event` | 敲钟 | block-pos, target-world, entity-id, direction, cancelled |
| `block-brush-event` | 刷子刷方块 | block-pos, target-world, player, item, cancelled |
| `block-cook-event` | 方块烹饪 | block-pos, target-world, source, cancelled |
| `block-damage-abort-event` | 方块损坏中止 | player, block-pos, target-world, item-in-hand |
| `block-dispense-armor-event` | 发射器给实体穿装备 | block-pos, target-world, target-entity-id, item, cancelled |
| `block-dispense-loot-event` | 发射器掉落物 | block-pos, target-world, items, cancelled |
| `block-drop-item-event` | 方块掉落物品 | block-pos, target-world, player, items, cancelled |
| `block-exp-event` | 方块经验 | block-pos, target-world, exp |
| `block-fertilize-event` | 骨粉催熟 | block-pos, target-world, player, changed-blocks: list<tuple<block-pos, u16>>, cancelled |
| `block-multi-place-event` | 批量放置 | player, target-world, placed-blocks: list<tuple<block-pos, u16>>, cancelled |
| `block-receive-game-event` | 方块接收游戏事件 | block-pos, target-world, game-event, source-entity-id, cancelled |
| `block-shear-entity-event` | 方块剪实体 | block-pos, target-world, target-entity-id, item, cancelled |
| `block-spread-event` | 方块扩散 | source-pos, target-pos, target-world, new-state-id, cancelled |
| `brewing-start-event` | 酿造开始 | block-pos, target-world, brewing-time, cancelled |
| `campfire-start-event` | 营火开始烹饪 | block-pos, target-world, item, slot, cooking-time, cancelled |
| `cauldron-level-change-event` | 炼药锅液面变化 | block-pos, target-world, old-level, new-level, reason, entity-id, cancelled |
| `crafter-craft-event` | 合成器合成 | block-pos, target-world, cancelled |
| `entity-block-form-event` | 实体使方块形成 | entity-id, block-pos, target-world, new-state-id, cancelled |
| `fluid-level-change-event` | 流体液面变化 | block-pos, target-world, new-state-id, cancelled |
| `inventory-block-start-event` | 方块容器交互开始 | block-pos, target-world |
| `leaves-decay-event` | 树叶 decay | block-pos, target-world, cancelled |
| `moisture-change-event` | 耕地湿度变化 | block-pos, target-world, new-moisture, cancelled |
| `sculk-bloom-event` | 幽匿蔓延 | block-pos, target-world, charge, cancelled |
| `vault-display-item-event` | 宝库展示物品 | block-pos, target-world, item, cancelled |

---

## 4. 实体事件

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `entity-damage-event` | 实体受伤 | entity-id, damage, damage-type, cancelled |
| `entity-death-event` | 实体死亡 | entity-id, dropped-exp |
| `entity-spawn-event` | 实体生成 | entity-id, entity-type, position, target-world, cancelled |
| `entity-combust-event` | 实体燃烧 | entity-id, duration-secs, cancelled |
| `entity-regain-health-event` | 回血 | entity-id, amount, cancelled |
| `entity-air-change-event` | 氧气变化 | entity-id, amount, cancelled |
| `entity-breed-event` | 繁殖 | father-id, mother-id, child-id, cancelled |
| `entity-dismount-event` | 下坐骑 | entity-id, dismounted-id, cancelled |
| `entity-dye-event` | 染色 | entity-id, color, player, cancelled |
| `entity-enter-love-mode-event` | 进入求偶 | entity-id, human-entity-id, ticks-in-love, cancelled |
| `entity-explode-event` | 实体爆炸 | entity-id, position, yield-rate, cancelled |
| `entity-mount-event` | 上坐骑 | entity-id, mounted-id, cancelled |
| `entity-pickup-item-event` | 拾取物品 | entity-id, item-name, count, cancelled |
| `entity-portal-event` | 穿传送门 | entity-id, portal-pos, cancelled |
| `entity-resurrect-event` | 复活 | entity-id, cancelled |
| `entity-shoot-bow-event` | 射箭 | entity-id, weapon-name, force, cancelled |
| `entity-tame-event` | 驯服 | entity-id, owner, cancelled |
| `entity-target-event` | 选择目标 | entity-id, target-id, cancelled |
| `entity-teleport-event` | 实体传送 | entity-id, from-position, to-position, cancelled |
| `entity-toggle-glide-event` | 滑翔切换 | entity-id, is-gliding, cancelled |
| `entity-transform-event` | 实体转化 | entity-id, new-entity-id, transform-reason, cancelled |
| `creature-spawn-event` | 生物生成 | entity-id, entity-type, position, target-world, spawn-reason, player, cancelled |
| `ender-dragon-change-phase-event` | 末影龙换阶段 | entity-id, current-phase, new-phase, cancelled |
| `entity-break-door-event` | 破坏门 | entity-id, block-pos, cancelled |
| `entity-change-block-event` | 改变方块 | entity-id, block-pos, new-block, cancelled |
| `entity-damage-by-block-event` | 被方块伤害 | entity-id, damager-pos, damage, cause, cancelled |
| `entity-damage-by-entity-event` | 被实体伤害 | entity-id, damager-id, damage, cause, cancelled |
| `entity-drop-item-event` | 掉落物品 | entity-id, item-name, count, cancelled |
| `entity-enter-block-event` | 进入方块 | entity-id, block-pos, cancelled |
| `entity-exhaustion-event` | 消耗 | entity-id, exhaustion, cancelled |
| `entity-interact-event` | 实体交互方块 | entity-id, block-pos, cancelled |
| `entity-knockback-event` | 击退 | entity-id, hit-by-id, knockback, cancelled |
| `entity-place-event` | 实体放置方块 | entity-id, block-pos, block-name, cancelled |
| `entity-pose-change-event` | 姿态变化 | entity-id, pose, cancelled |
| `entity-potion-effect-event` | 获得药水效果 | entity-id, effect-name, duration, amplifier, cancelled |
| `entity-spell-cast-event` | 施法 | entity-id, spell, cancelled |
| `entity-target-living-entity-event` | 锁定生物 | entity-id, target-id, reason, cancelled |
| `entity-toggle-swim-event` | 游泳切换 | entity-id, is-swimming, cancelled |
| `explosion-prime-event` | 爆炸物点燃 | entity-id, radius, fire, cancelled |
| `firework-explode-event` | 烟花爆炸 | entity-id, cancelled |
| `food-level-change-event` | 饥饿值变化 | entity-id, food-level, cancelled |
| `item-despawn-event` | 掉落物消失 | entity-id, cancelled |
| `item-merge-event` | 掉落物合并 | entity-id, target-id, cancelled |
| `item-spawn-event` | 掉落物生成 | entity-id, position, item-name, cancelled |
| `piglin-barter-event` | 猪灵以物易物 | entity-id, input-item, outcome, cancelled |
| `projectile-hit-event` | 投掷物命中 | entity-id, hit-position, hit-entity-id, cancelled |
| `projectile-launch-event` | 投掷物发射 | entity-id, shooter-id, cancelled |
| `sheep-dye-wool-event` | 羊染色 | entity-id, dye-color, player-id, cancelled |
| `sheep-regrow-wool-event` | 羊毛再生 | entity-id, cancelled |
| `slime-split-event` | 史莱姆分裂 | entity-id, count, cancelled |
| `strider-temperature-change-event` | 炽足兽温度变化 | entity-id, is-shivering, cancelled |
| `villager-acquire-trade-event` | 村民获得交易 | entity-id, recipe-index, cancelled |
| `villager-career-change-event` | 村民职业变化 | entity-id, profession, reason, cancelled |
| `villager-replenish-trade-event` | 村民补货 | entity-id, restock-quantity, cancelled |
| `warden-anger-change-event` | 监守者愤怒变化 | entity-id, target-id, old-anger, new-anger, cancelled |
| `area-effect-cloud-apply-event` | 药水云生效 | entity-id, affected-entities, cancelled |
| `arrow-body-count-change-event` | 箭数量变化 | entity-id, old-amount, new-amount, cancelled |
| `bat-toggle-sleep-event` | 蝙蝠睡眠切换 | entity-id, is-awake, cancelled |
| `creeper-power-event` | 苦力怕充能 | entity-id, lightning-id, cause, cancelled |
| `entity-combust-by-block-event` | 被方块点燃 | entity-id, combuster, duration, cancelled |
| `entity-combust-by-entity-event` | 被实体点燃 | entity-id, combuster-id, duration, cancelled |
| `entity-knockback-by-entity-event` | 被实体击退 | entity-id, hit-by-id, force, x, z, cancelled |
| `entity-portal-enter-event` | 进入传送门 | entity-id, location, cancelled |
| `entity-portal-exit-event` | 离开传送门 | entity-id, from-pos, to-pos, cancelled |
| `entity-remove-event` | 实体移除 | entity-id, cause, cancelled |
| `entity-target-block-event` | 锁定方块 | entity-id, block-pos, cancelled |
| `entity-unleash-event` | 解开拴绳 | entity-id, reason, cancelled |
| `exp-bottle-event` | 经验瓶 | entity-id, experience, location, show-effect, cancelled |
| `horse-jump-event` | 马跳跃 | entity-id, power, cancelled |
| `lingering-potion-splash-event` | 滞留药水溅射 | entity-id, location, potion-item, cancelled |
| `pig-zap-event` | 猪被雷劈 | entity-id, lightning-id, pig-zombie-id, cancelled |
| `pig-zombie-anger-event` | 僵尸猪灵愤怒 | entity-id, target-id, new-anger, cancelled |
| `potion-splash-event` | 喷溅药水 | entity-id, location, potion-item, affected-entities, cancelled |
| `spawner-spawn-event` | 刷怪笼生成 | entity-id, spawner-pos, cancelled |
| `trial-spawner-spawn-event` | 试炼刷怪笼生成 | entity-id, spawner-pos, cancelled |
| `villager-reputation-change-event` | 村民声望变化 | entity-id, target-id, reputation-change, cancelled |

---

## 5. 世界 / 区块 / 结构 / 天气

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `chunk-load-event` | 区块加载 | target-world, chunk-x, chunk-z, cancelled |
| `chunk-save-event` | 区块保存 | target-world, chunk-x, chunk-z, cancelled |
| `chunk-send-event` | 区块发送 | target-world, chunk-x, chunk-z, cancelled |
| `chunk-populate-event` | 区块填充 | chunk-x, chunk-z, cancelled |
| `chunk-unload-event` | 区块卸载 | chunk-x, chunk-z, cancelled |
| `entities-load-event` | 实体加载 | chunk-x, chunk-z, entity-count, cancelled |
| `entities-unload-event` | 实体卸载 | chunk-x, chunk-z, entity-count, cancelled |
| `world-load-event` | 世界加载 | target-world |
| `world-unload-event` | 世界卸载 | target-world, cancelled |
| `world-init-event` | 世界初始化 | target-world |
| `world-save-event` | 世界保存 | world-name, cancelled |
| `weather-change-event` | 天气变化 | target-world, to-weather-state, cancelled |
| `thunder-change-event` | 雷暴变化 | target-world, to-thunder-state, cancelled |
| `time-skip-event` | 时间跳跃 | skip-amount, cancelled |
| `async-structure-generate-event` | 结构生成 | world-name, structure-name, pos, cancelled |
| `async-structure-spawn-event` | 结构放置 | world-name, structure-name, pos, cancelled |
| `loot-generate-event` | 战利品生成 | loot-table, cancelled |
| `portal-create-event` | 传送门创建 | pos, portal-type, cancelled |
| `structure-grow-event` | 结构生长 | pos, species, bone-meal, cancelled |
| `generic-game-event` | 通用游戏事件 | event-id, pos, cancelled |
| `spawn-change-event` | 世界出生点变化 | target-world, previous-position/yaw/pitch, new-position/yaw/pitch |
| `lightning-strike-event` | 闪电 | position, is-effect, cancelled |

---

## 6. 背包 / 合成 / 熔炉 / 交易

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `inventory-click-event` | 点击槽位 | player, window-type, click-type, slot, raw-slot, clicked-item, cursor, hotbar-button, cancelled |
| `inventory-close-event` | 关闭容器 | player, window-type |
| `inventory-open-event` | 打开容器 | player, cancelled |
| `inventory-drag-event` | 拖拽物品 | player, cancelled |
| `inventory-interact-event` | 容器交互 | player, cancelled |
| `inventory-creative-event` | 创造模式取物 | player, slot, item-id, item-count, cancelled |
| `inventory-move-item-event` | 移动物品 | source-pos, target-pos, item-id, item-amount, cancelled |
| `inventory-pickup-item-event` | 拾取物品 | block-pos, item-entity-id, item-id, cancelled |
| `craft-item-event` | 合成物品 | player, recipe-id, cancelled |
| `furnace-smelt-event` | 熔炼 | block-pos, source-item, result-item, cancelled |
| `brew-event` | 酿造 | block-pos, fuel-level, cancelled |
| `brewing-stand-fuel-event` | 酿造台燃料 | block-pos, fuel-power, cancelled |
| `furnace-burn-event` | 熔炉燃烧 | block-pos, fuel-item, burn-time, cancelled |
| `furnace-extract-event` | 取出熔炼产物 | player, block-pos, item-id, item-amount, exp-gained |
| `furnace-start-smelt-event` | 开始熔炼 | block-pos, source-item, cooking-time, cancelled |
| `hopper-inventory-search-event` | 漏斗搜索容器 | block-pos, search-pos, cancelled |
| `prepare-anvil-event` | 铁砧结果预览 | player, rename-text, repair-cost |
| `prepare-grindstone-event` | 砂轮结果预览 | player, result-item |
| `prepare-inventory-result-event` | 容器结果预览 | player, result-item |
| `prepare-item-craft-event` | 合成预览 | player, recipe-id, cancelled |
| `prepare-smithing-event` | 锻造结果预览 | player, result-item |
| `smith-item-event` | 锻造物品 | player, recipe-id, cancelled |
| `prepare-item-enchant-event` | 附魔预览 | player, item, offers, bookshelf-count, cancelled |
| `enchant-item-event` | 附魔 | player, item, cost, enchantments-to-add, cancelled |
| `trade-select-event` | 选择交易 | player, slot-index, cancelled |
| `map-initialize-event` | 地图初始化 | map-id |

---

## 7. 服务器 / 数据包

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `server-command-event` | 服务器命令 | command, cancelled |
| `server-list-ping-event` | 服务器列表 ping | hostname, address, motd, max-players, num-players, favicon |
| `server-load-event` | 服务器加载 | load-type |
| `server-broadcast-event` | 广播 | message, sender, cancelled |
| `server-tick-start-event` | tick 开始 | tick |
| `server-tick-end-event` | tick 结束 | tick, duration-nanos |
| `packet-received-event` | 收到数据包 | player, packet, packet-id, raw-payload, cancelled |
| `packet-sent-event` | 发送数据包 | player, packet, packet-id, raw-payload, cancelled |

---

## 8. 载具事件

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `vehicle-block-collision-event` | 载具撞方块 | vehicle-id, block-pos, cancelled |
| `vehicle-collision-event` | 载具碰撞 | vehicle-id, cancelled |
| `vehicle-create-event` | 载具创建 | vehicle-id, cancelled |
| `vehicle-damage-event` | 载具受损 | vehicle-id, damage, attacker-id, cancelled |
| `vehicle-destroy-event` | 载具销毁 | vehicle-id, attacker-id, cancelled |
| `vehicle-enter-event` | 上载具 | vehicle-id, entered-id, cancelled |
| `vehicle-entity-collision-event` | 载具撞实体 | vehicle-id, collided-entity-id, cancelled |
| `vehicle-exit-event` | 下载具 | vehicle-id, exited-id, cancelled |
| `vehicle-move-event` | 载具移动 | vehicle-id, from-position, to-position, cancelled |
| `vehicle-update-event` | 载具更新 | vehicle-id, cancelled |

---

## 9. 悬挂实体事件

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `hanging-break-event` | 悬挂物被破坏 | entity-id, remover-entity-id, cancelled |
| `hanging-break-by-entity-event` | 被实体破坏 | entity-id, remover-entity-id, cancelled |
| `hanging-place-event` | 放置悬挂物 | entity-id, player, block-pos, block-face, cancelled |

---

## 10. 袭击事件

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `raid-finish-event` | 袭击结束 | victory, cancelled |
| `raid-spawn-wave-event` | 袭击刷波 | wave, pos, cancelled |
| `raid-stop-event` | 袭击停止 | reason, cancelled |
| `raid-trigger-event` | 触发袭击 | pos, cancelled |

---

## 11. UI / 对话框 / 表单

| 事件 | 说明 | 数据字段 |
|---|---|---|
| `dialog-click-action-event` | 对话框按钮点击 | player, id, payload, cancelled |
| `dialog-show-event` | 显示对话框 | player, dialog, cancelled |
| `dialog-clear-event` | 清除对话框 | player, cancelled |
| `bedrock-form-response-event` | 基岩表单响应 | player, form-id, response-data |
