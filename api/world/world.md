# World API（世界 / 实体 / 方块接口）

> WIT 模块：`world`、`block-entity`、`biomes`、`game-rules`、`game-events`、`datapack`、`advancement`、`statistics`、`entity`
> 命名说明：下文用 WIT 的 kebab-case 名（如 `get-block-state`）。jco 生成的 TS 绑定会转成 camelCase（`getBlockState`），文档以 WIT 名为准。

---

## 1. 世界 `%world`

通过 `server.get-world-by-name()`、`server.get-all-worlds()`、玩家 `.getWorld()` 等获得。

### 方块操作

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-block-state-id(pos)` | block-pos | `u16` | 取方块状态 ID |
| `get-block-state(pos)` | block-pos | `block-state` | 取详细方块状态信息 |
| `set-block-state(pos, state: u16, update-flags)` | 位置、状态 ID、更新标志 | 无 | 设置方块状态 |
| `get-block(pos)` | block-pos | `block` | 取方块类型 |
| `get-block-id(pos)` | block-pos | `u16` | 取方块数字 ID |
| `set-block(pos, block, update-flags)` | 位置、block、标志 | 无 | 用默认状态设置方块 |
| `set-block-by-id(pos, block-id: u16, flags)` | 位置、ID、标志 | 无 | 按数字 ID 设置 |
| `set-block-by-name(pos, name, flags)` | 位置、命名空间名、标志 | `bool` | 按名字设置，识别成功返回 true |

### 世界属性

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-id()` | 无 | `string` | 世界唯一标识 |
| `get-name()` | 无 | `string` | 世界名（如 `world`、`arena_1`） |
| `get-dimension()` | 无 | `string` | 维度类型（如 `minecraft:overworld`） |
| `get-time-of-day()` / `set-time-of-day(time)` | u64 | u64 / 无 | 当前一天内时刻 |
| `get-world-age()` | 无 | `u64` | 世界总 tick 数 |
| `get-spawn-location()` | 无 | `world-spawn-location` | 共享出生点 |
| `get-top-block-y(x, z)` | 坐标 | `s32` | 最高非空气方块 Y |
| `get-motion-blocking-height(x, z)` | 坐标 | `s32` | 最高阻挡运动方块 Y |
| `is-raining()` / `set-raining(bool)` | bool | bool / 无 | 下雨/下雪状态 |
| `is-thundering()` / `set-thundering(bool)` | bool | bool / 无 | 雷暴状态 |
| `get-sea-level()` | 无 | `s32` | 海平面 |
| `get-min-y()` | 无 | `s32` | 世界最低 Y |
| `save()` | 无 | `Result<_, string>` | 保存全部区块/方块实体/实体 |

### 光照 / 生物群系

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-sky-light(pos)` / `set-sky-light(pos, level)` | 位置、等级 | `u8` / 无 | 天空光 |
| `get-block-light(pos)` / `set-block-light(pos, level)` | 位置、等级 | `u8` / 无 | 方块光 |
| `get-biome(pos)` | block-pos | `biome` | 取生物群系 |

### 实体 / 爆炸 / 音效 / 粒子

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `spawn-entity(entity-type, pos)` | 实体类型、坐标 | `entity` | 生成实体 |
| `get-entities()` | 无 | `entity[]` | 世界内所有实体 |
| `strike-lightning(pos, effect-only)` | 坐标、是否仅视觉 | 无 | 劈闪电 |
| `create-explosion(pos, power, create-fire, interaction)` | 坐标、威力、是否起火、交互方式 | 无 | 生成爆炸 |
| `play-sound(sound, category, pos, volume, pitch)` | 音效、分类、坐标、音量、音调 | 无 | 播放原版音效 |
| `play-custom-sound(name, category, pos, volume, pitch)` | 资源包音效名… | 无 | 播放自定义音效 |
| `spawn-particle(particle, pos, offset, max-speed, count)` | 粒子、坐标、偏移、速度、数量 | 无 | 生成粒子 |

### 射线检测（Raycast）

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `ray-trace-blocks(start, end)` | 起点、终点 | `Option<position>` | 命中方块的坐标 |
| `ray-trace-block(start, end, include-fluids)` | 起点、终点、含流体 | `Option<ray-trace-block-result>` | 方块命中详情 |
| `ray-trace-entity(start, end)` | 起点、终点 | `Option<ray-trace-entity-result>` | 最近实体命中 |
| `ray-trace-entities(start, end)` | 起点、终点 | `ray-trace-entity-result[]` | 所有命中实体 |

### 区块 / 方块实体 / 自定义数据 / 规则

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-chunk(x, z)` | 区块坐标 | `Option<chunk>` | 取区块 |
| `get-border()` / `get-world-border()` | 无 | `world-border` | 世界边界 |
| `get-block-entity(pos)` | block-pos | `Option<block-entity-type>` | 取方块实体 |
| `get-block-entity-nbt(pos)` | block-pos | `Option<list<u8>>` | 取方块实体原始 NBT |
| `set-block-entity-nbt(pos, nbt-data)` | 位置、NBT 字节 | `Result<_, string>` | 用 NBT 恢复方块实体（须含 `id`） |
| `set-chunk-generator(generator-id)` | 生成器 ID | 无 | 绑定自定义区块生成器 |
| `set-custom-data(namespace, key, value: nbt-tree)` | 命名空间、键、NBT | 无 | 写自定义数据 |
| `get-custom-data(namespace, key)` | 命名空间、键 | `Option<nbt-tree>` | 读自定义数据 |
| `remove-custom-data(namespace, key)` | 命名空间、键 | 无 | 删自定义数据 |
| `has-custom-data(namespace, key)` | 命名空间、键 | `bool` | 是否存在 |
| `get-game-rule(rule)` | game-rule | `game-rule-value` | 读游戏规则 |
| `set-game-rule(rule, value)` | 规则、值 | 无 | 写游戏规则 |

---

## 2. 区块 `chunk`

区块坐标的 X/Z 相对位置须在 `[0,15]`。方法与 world 类似：
- 坐标：`get-x` / `get-z`
- 方块：`get-block-state-id` / `get-block-state` / `set-block-state` / `get-block` / `set-block` / `set-block-by-id`
- 光照：`get-sky-light` / `get-block-light`
- 生物群系：`get-biome`
- 方块实体：`get-block-entity(pos) -> Option<block-entity-type>`
- `get-top-block-y(x, z) -> s32`
- 自定义数据：`set/get/remove/has-custom-data`（同 world）

---

## 3. 自定义区块生成

### 枚举 `generation-phase`：`biomes` / `noise` / `surface` / `features`

### 资源 `chunk-buffer`（生成阶段的 16×16 列缓冲）

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-x` / `get-z` | 无 | s32 | 区块坐标 |
| `get-min-y` | 无 | s32 | 最低 Y |
| `get-height` | 无 | u32 | 高度 |
| `get-block-state-id(x, y, z)` / `set-block-state-id(x, y, z, state-id)` | 局部坐标、状态 | u16 / 无 | 单方块 |
| `fill-layer(y, state-id)` | Y、状态 | 无 | 填满该 Y 的 16×16 层 |
| `fill-range(x, min-y, max-y, z, state-id)` | 局部坐标、范围、状态 | 无 | 填充竖直柱 |
| `fill-cuboid(min-x, min-y, min-z, max-x, max-y, max-z, state-id)` | 立方体范围、状态 | 无 | 填充 3D 立方体 |
| `set-biome(x, y, z, biome)` / `fill-biome(biome)` | 坐标/群系 | 无 | 设置生物群系 |

---

## 4. 世界边界 `world-border`

- 中心：`get-center-x/z`、`get-center`、`set-center(x, z)`
- 尺寸：`get-diameter` / `get-size`、`set-diameter(diameter, speed: Option<u64>)`、`set-size`、`set-size-transition(new-size, time-seconds)`
- 目标：`get-target-diameter`、`get-target-speed`
- 警告：`get/set-warning-distance`、`get/set-warning-delay`（别名 `warning-time`）
- 伤害：`get/set-damage-buffer`、`get/set-damage-amount`
- 判断：`contains(x, z)`、`contains-pos(pos)`
- `reset()` 恢复默认

---

## 5. 实体 `entity`

所有实体的基类句柄。可选转成 `living-entity` / `mob`。

### 基础信息
`get-id: u32`、`get-uuid`、`get-type: entity-type`、`get-position`、`get-world`、`get-yaw`、`get-pitch`、`get-head-yaw`、`get-width`、`get-height`、`get-eye-height`、`get-eye-position`、`get-bounding-box`

### 变换
`teleport(pos, world-ref)`、`set-velocity(velocity)`、`get-velocity`、`set-rotation(yaw, pitch)`、`get-nearby-entities(x, y, z) -> entity[]`

### 名称 / 状态
`get-name`、`set/get-custom-name`、`set/is-custom-name-visible`、`is/set-invulnerable`、`get/set-fire-ticks`、`get/set-fall-distance`、`get/set-ticks-lived`、`is/set-on-fire`、`has/set-visual-fire`、`get/set-portal-cooldown`、`get/set-remaining-air`、`get-max-air`
- 姿态开关：`is/set-sneaking`、`sprinting`、`swimming`、`invisible`、`glowing`、`fall-flying`、`silent`、`has-gravity`
- 查询：`is-on-ground`、`is-in-water`、`is-in-lava`、`is-living`、`is-mob`

### 载具 / 乘客
`get-vehicle` / `set-vehicle(Option<entity>)`、`get-passengers`、`add-passenger`、`remove-passenger`、`eject-passengers`

### 射线 / 目标
`raycast(max-distance, fluid-handling) -> Option<raycast-result>`、`ray-trace-block`、`ray-trace-entity`、`get-target-entity(max-distance)`

### 自定义数据 / 转换 / 移除
`set/get/remove/has-custom-data(namespace, key, nbt-tree)`、`as-living() -> Option<living-entity>`、`as-mob() -> Option<mob>`、`remove()`

---

## 6. 生物实体 `living-entity`

拥有生命/属性/装备的实体（怪物、玩家、盔甲架）。

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `as-entity` | 无 | entity | 转回基础实体 |
| `as-mob` / `is-mob` | 无 | Option<mob> / bool | 转/判断是否 AI 生物 |
| `get-health` / `set-health(health)` | f32 | f32 / 无 | 当前血量 |
| `get-max-health` / `set-max-health` | f32 | f32 / 无 | 最大血量 |
| `damage(amount, damage-type)` | 伤害值、伤害类型 | 无 | 造成伤害 |
| `is-dead` | 无 | bool | 是否死亡 |
| `get/set-absorption(amount)` | f32 | f32 / 无 | 伤害吸收 |
| `get-attribute-value(attr)` | attribute | f64 | 属性有效值 |
| `get/set-attribute-base(attr, value)` | 属性、值 | f64 / 无 | 属性基础值 |
| `add-attribute-modifier(attr, modifier)` | 属性、修饰符 | 无 | 添加属性修饰符 |
| `remove-attribute-modifier(attr, id)` | 属性、修饰符 ID | 无 | 移除修饰符 |
| `get-attribute-modifiers(attr)` | 属性 | attribute-modifier[] | 所有修饰符 |
| `reset-attribute(attr)` / `reset-all-attributes()` | 属性 | 无 | 重置 |
| `get-equipment(slot)` / `set-equipment(slot, stack)` | equipment-slot、物品 | Option<item-stack> / 无 | 装备槽 |
| `clear-equipment()` | 无 | 无 | 清空装备 |
| `get-age` / `set-age(age)` | s32 | s32 / 无 | 年龄 |
| `send-system-message(message)` | 文本组件 | 无 | 发系统消息 |

---

## 7. AI 生物 `mob`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `as-entity` / `as-living` | 无 | entity / living-entity | 转换 |
| `add-ai-goal(priority, goal: builtin-ai-goal)` | 优先级、内置目标 | 无 | 添加内置 AI 目标 |
| `add-custom-ai-goal(priority, goal-id)` | 优先级、注册 ID | 无 | 添加自定义目标 |
| `clear-ai-goals` / `set-ai-disabled` / `is-ai-disabled` | bool | 无 / bool | 控制 AI |
| `set-target` / `get-target` | Option<entity> | 无 / Option<entity> | 攻击目标 |
| `navigate-to-pos(pos, speed)` | 坐标、速度 | bool | 寻路到坐标 |
| `navigate-to-entity(target, speed)` | 实体、速度 | bool | 寻路到实体 |
| `stop-navigation` / `is-navigating` / `has-reached-destination` | 无 | 无 / bool | 导航状态 |
| `set-navigation-speed(speed)` | f64 | 无 | 移动速度 |
| `can-reach(pos, max-distance)` | 坐标、距离 | bool | 可否到达 |
| `set/get-pathfinding-malus(node-type, malus)` | 节点类型、代价 | 无 / f32 | 寻路代价 |
| `look-at(pos)` / `look-at-entity(target)` | 坐标/实体 | 无 | 朝向 |
| `get-mob-data()` | 无 | `mob-data` | 特殊生物数据 |
| `set-mob-data(data)` | mob-data | `bool` | 设置，类型不匹配返回 false |
| `set/get-freeze-ticks(ticks)` | s32 | 无 / s32 | 冰冻 tick（0–140） |

### 枚举 `builtin-ai-goal`
`swim`、`wander-around(f32)`、`melee-attack(f32)`、`look-at-player(f32)`、`look-around`、`escape-danger(f32)`、`avoid-entity(f32)`、`blaze-attack`、`creeper-ignite`、`eat-grass`、`zombie-attack(f32)`

### 枚举 `path-node-type`
blocked、open、walkable、walkable-door、trapdoor、powder-snow、danger-powder-snow、fence、lava、water、water-border、rail、unpassable-rail、danger-fire、damage-fire、danger-other、damage-other、door-open、door-wood-closed、door-iron-closed、breach、leaves、sticky-honey、cocoa、damage-cautious、danger-trapdoor

### 记录 `mob-data`（variant）
`sheep`、`wolf`、`cat`、`villager`、`creeper`、`slime`、`enderman`、`iron-golem`、`fox`、`ageable`、`zombie`、`shulker`、`generic`。各数据结构：
- `sheep-data { color, is-sheared }`
- `wolf-data { is-tamed, owner: Option<uuid>, is-sitting, collar-color, is-angry, is-begging }`
- `cat-data { is-tamed, owner, is-sitting, collar-color }`
- `villager-data { profession, level: u8, experience: u32 }`
- `creeper-data { is-powered, fuse: s32, is-ignited, explosion-radius: u8 }`
- `slime-data { size: s32 }`
- `enderman-data { carried-block-state: Option<u16>, is-screaming, is-staring }`
- `iron-golem-data { is-player-created }`
- `fox-data { is-sitting, is-sleeping, is-crouching }`
- `ageable-data { is-baby, age, in-love-ticks }`
- `zombie-data { is-baby, can-break-doors }`
- `shulker-data { attached-face, peek-amount, color: Option<dye-color> }`

---

## 8. 方块实体 `block-entity`

通过 `world.get-block-entity` / `chunk.get-block-entity` 得到 `block-entity-type`（variant，按类型分派）。

### 基础资源 `block-entity`
`resource-location() -> string`、`get-position`、`get-id`、`is-dirty` / `clear-dirty`、自定义数据 `set/get/remove/has-custom-data`。

### `container-block-entity`（箱子类通用容器）
`get-block-entity`、`get-inventory()`、`get-size`、`is-empty`、`get-stack(slot)`、`set-stack(slot, stack)`、`remove-stack(slot)`、`clear`

### 各特殊方块实体（均含 `get-block-entity`）
| 资源 | 额外方法 |
|---|---|
| `command-block-entity` | `last-output`、`track-output`、`success-count`、`command`、`auto`、`condition-met`、`powered` |
| `sign-block-entity` / `hanging-sign-block-entity` | `get/set-front-text`、`get/set-back-text`、`is/set-waxed` |
| `jukebox-block-entity` | `get-container`、`is-playing`、`stop-playing`、`start-playing(length-in-ticks)` |
| `chest-block-entity` / `trapped-chest-block-entity` / `barrel-block-entity` / `ender-chest-block-entity` / `shulker-box-block-entity` | `get-container`、`viewer-count` |
| `mob-spawner-block-entity` | `get-spawn-count`、`get-spawn-range`、`get-delay` |
| `map-block-entity` | `get/set-map-id`、`get/set-colors`、`set/get-pixel`、`update`、`stream-frame` |
| `banner-block-entity` | `get-custom-name` |
| `beacon-block-entity` | `get-container`、`get-primary-effect`、`get-secondary-effect`、`get-levels` |
| `beehive-block-entity` | `get-bee-count` |
| `bell-block-entity` | `is-ringing`、`get-ring-ticks` |
| `blasting-furnace-block-entity` / `furnace-block-entity` / `smoker-block-entity` | `get-container`、`get-cooking-time-spent`、`get-cooking-total-time`、`get-lit-time-remaining`、`get-lit-total-time`、`is-burning` |
| `brewing-stand-block-entity` | `get-container`、`get-brew-time`、`get-fuel` |
| `campfire-block-entity` / `dispenser-block-entity` / `dropper-block-entity` / `lectern-block-entity` / `shelf-block-entity` | `get-container` |
| `chiseled-bookshelf-block-entity` | `get-container`、`get-last-interacted-slot` |
| `comparator-block-entity` | `get-output-signal` |
| `crafter-block-entity` | `get-container`、`get-crafting-ticks-remaining`、`is-triggered` |
| `creaking-heart-block-entity` | `get-creaking-uuid` |
| `end-gateway-block-entity` | `get-age`、`is-exact-teleport` |
| `hopper-block-entity` | `get-container`、`get-cooldown` |
| `jigsaw-block-entity` | `get-name`、`get-target`、`get-pool`、`get-final-state`、`get-selection-priority`、`get-placement-priority` |
| `piston-block-entity` | `get-progress`、`is-extending`、`is-source` |
| `sculk-shrieker-block-entity` | `get-warning-level` |
| `skull-block-entity` | `get-note-block-sound` |
| `structure-block-block-entity` | `get-name`、`get-author`、`get-mode`、`get-integrity`、`get-seed` |
| 其余（bed、brushable、calibrated-sculk-sensor、conduit、copper-golem-statue、daylight-detector、decorated-pot、enchanting-table、end-portal、potent-sulfur、sculk-catalyst、sculk-sensor、test-block、test-instance-block、trial-spawner、vault） | 仅 `get-block-entity` |

> `block-entity-type` variant 覆盖上表所有类型，另有 `container-block-entity`。
> 辅助记录：`sign-text { messages: string[], color: dye-color, has-glowing-text: bool }`；枚举 `dye-color`（16 色）。

---

## 9. 方块与方块状态（`world` 顶层函数）

| 函数 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `resolve-block-state(name, properties)` | 名字、属性键值对 | `Option<u16>` | 名字+属性 → 状态 ID |
| `block-state-to-info(state-id)` | 状态 ID | `Option<block-state-info>` | 反向解析 |
| `get-block-by-id(id)` | u16 | `Option<block>` | 按数字 ID 取方块 |
| `get-block-by-name(name)` | 名字 | `Option<block>` | 按名字取方块 |
| `get-all-blocks()` / `get-all-block-names()` | 无 | block[] / string[] | 全部方块 |
| `get-block-count()` / `get-block-state-count()` | 无 | u32 | 数量 |
| `get-states-for-block(block)` / `get-states-for-block-id(block-id)` | 方块/ID | block-state[] | 全部状态 |
| `get-state-ids-for-block-id(block-id)` | ID | u16[] | 全部状态 ID |
| `get-block-properties(state-id)` | 状态 ID | tuple[] | 属性键值对 |
| `get-block-from-state-id(state-id)` / `get-block-from-state(state)` | 状态 | Option<block> / block | 取父方块 |
| `get-default-state-from-block(block)` / `get-default-state-from-block-id(block-id)` | 方块/ID | block-state / Option | 默认状态 |
| `get-block-state-by-id(state-id)` | 状态 ID | Option<block-state> | 详细状态 |

### 记录
- `block-state`：id、block-id、block-name、luminance、opacity、hardness、is-air/liquid/solid/full-cube、piston-behavior、has-random-ticks、burnable、tool-required、sided-transparency、replaceable、is-solid-block、block-entity-type、instrument、collision-shapes、outline-shapes、各面是否实心、map-color、properties
- `block`：id、name、hardness、blast-resistance、map-color、slipperiness、velocity-multiplier、jump-velocity-multiplier、item-id、default-state-id、state-ids、is-solid/air/flammable、flammable
- `block-state-id { id: u16 }`、`block-state-info { name, properties }`、`flammable { spread-chance, burn-chance }`、`bounding-box { min, max }`、`world-spawn-location { pos, yaw, pitch }`
- `flags block-flags`：notify-neighbors、notify-listeners、force-state、skip-drops、moved、skip-redstone-wire-state-replacement、skip-block-entity-replaced-callback、skip-block-added-callback

### 枚举
- `equipment-slot`：main-hand、off-hand、feet、legs、chest、head、body、saddle
- `piston-behavior`：normal、destroy、block、ignore、push-only
- `block-direction`：down、up、north、south、west、east
- `explosion-interaction`：none、block、mob、tnt、trigger
- `noteblock-instrument`：harp…custom-head（27 种）
- `dye-color`、`villager-profession`（见上）

### 射线结果记录
- `raycast-result { pos, face }`
- `ray-trace-block-result { pos, face, hit-pos }`
- `ray-trace-entity-result { entity, hit-pos, distance }`

---

## 10. 生物群系 `biomes`

- 枚举 `biome`：全部原版 + 新增群系，共 66 种（badlands、forest、plains、the-end、deep-dark、pale-garden、sulfur-caves 等）。

---

## 11. 游戏规则 `game-rules`

- 枚举 `game-rule`：60+ 条（advance-time、keep-inventory、pvp、spawn-monsters、random-tick-speed、max-entity-cramming 等）
- variant `game-rule-value`：`int(s32)` 或 `%bool(bool)`
- 通过 `world.get-game-rule` / `world.set-game-rule` 读写。

---

## 12. 游戏事件 `game-events`

- 枚举 `game-event`：block-activate、block-place、entity-die、explode、note-block-play、teleport、resonate-1…resonate-15 等，共 60+ 种。

---

## 13. 数据包 `datapack`

### 记录 `datapack-info`
`id`、`name`、`description`、`pack-format: u32`、`is-enabled`、`recipe-count`、`function-count`

### variant `enable-position`：`first` / `last` / `before(string)` / `after(string)`

### 资源 `datapack-manager`（`server.get-datapack-manager()`）

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `list-all-packs` | 无 | datapack-info[] | 全部数据包 |
| `list-enabled-packs` / `list-available-packs` | 无 | datapack-info[] | 已启用/可用 |
| `get-pack(name)` | 名/ID | Option<datapack-info> | 取详情 |
| `is-enabled(name)` | 名 | bool | 是否启用 |
| `enable-pack(name, position)` | 名、位置 | Result<_, string> | 启用 |
| `disable-pack(name)` | 名 | Result<_, string> | 禁用 |
| `reload()` | 无 | Result<_, string> | 重载全部数据包/配方 |
| `execute-function(name)` | `namespace:fn` 或 `#namespace:tag` | Result<u32, string> | 执行函数，返回执行命令数 |

---

## 14. 成就 `advancement`

### 数据
- 枚举 `frame-type`：task、challenge、goal
- `advancement-display { title, description, frame, show-toast, hidden, announce-to-chat, background: Option<string>, x, y }`
- `advancement-info { id, parent-id: Option<string>, criteria: string[], display: Option<advancement-display> }`
- `advancement-progress { advancement-id, done, awarded-criteria: string[], remaining-criteria: string[] }`

> 通过 `server.get-advancement` / `get-all-advancement-ids` 查询；玩家侧通过 `player` 的成就方法操作。

---

## 15. 统计 `statistics`

- 枚举 `statistic-category`：mined、crafted、used、broken、picked-up、dropped、killed、killed-by、custom
- 枚举 `custom-statistic`：80+ 项（play-time、walk-one-cm、deaths、mob-kills、damage-dealt、raid-win 等）
- 通过 `player.get/set/increment-statistic` 与 `player.*-custom-statistic` 读写。

---

## 16. `entity` 模块

`entity.wit` 本身只是一条转发声明，把 `%world` 中的实体相关类型（`entity`、`living-entity`、`mob`、`path-node-type`、`dye-color`、`villager-profession`、`mob-data` 及各 mob data、`raycast-result`、`block-direction` 等）重新导出给 `entity` 接口使用。功能定义见本文第 5–7 节。
