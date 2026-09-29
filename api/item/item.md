# Item API（物品 / 背包 / 合成 / 效果接口）

> WIT 模块：`item-stack`、`inventory`、`recipe`、`enchantments`、`potions`、`status-effect`、`attributes`、`data-components`、`damage-types`、`entity-types`、`entity-statuses`
> 命名说明：WIT kebab-case 名在 TS 绑定中为 camelCase。

---

## 1. 物品堆 `item-stack`

### 构造
`new item-stack(registry-key: string, count: u8)` — 如 `("minecraft:diamond", 1)`。

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-registry-key` | 无 | string | 物品注册键（`minecraft:diamond`） |
| `get-count` / `set-count(count)` | u8 | u8 / 无 | 数量 |
| `get-max-count` | 无 | u8 | 最大堆叠数 |
| `get-enchantments` | 无 | enchantment-value[] | 所有附魔 |
| `add-enchantment(enchantment, level)` | 原版附魔、等级 | 无 | 添加附魔 |
| `remove-enchantment(enchantment)` | 附魔 | 无 | 移除附魔 |
| `get-custom-enchantments` | 无 | custom-enchantment-value[] | 自定义附魔 |
| `add-custom-enchantment(id, level)` | ID、等级 | 无 | 添加自定义附魔 |
| `remove-custom-enchantment(id)` / `get-custom-enchantment-level(id)` / `has-custom-enchantment(id)` | ID | 无 / Option<u32> / bool | 自定义附魔操作 |
| `get-attribute-modifiers` | 无 | item-attribute-modifier[] | 属性修饰符 |
| `add-attribute-modifier(modifier)` | 修饰符 | 无 | 添加 |
| `remove-attribute-modifiers(attribute)` | 属性 | 无 | 移除该属性全部修饰符 |
| `clear-attribute-modifiers` | 无 | 无 | 清空 |
| `get-lore` / `set-lore(lore)` / `add-lore(line)` | 文本组件数组 | lore / 无 | 描述文本 |
| `get-custom-name` / `set-custom-name(name)` | Option<text-component> | Option / 无 | 自定义名称 |
| `set-custom-data(namespace, key, value)` | 命名空间、键、NBT | 无 | 自定义数据 |
| `get-custom-data(namespace, key)` | 命名空间、键 | Option<nbt-tree> | 读数据 |
| `remove-custom-data` / `has-custom-data` | 命名空间、键 | 无 / bool | 删/查数据 |
| `get-components` | 无 | data-component-value[] | 所有数据组件 |
| `set-component(component, value)` | 组件、序列化字节 | 无 | 设置组件 |
| `remove-component(component)` | 组件 | 无 | 移除组件 |

### 辅助记录
- `enchantment-value { enchantment, level: u32 }`
- `custom-enchantment-value { enchantment-id: string, level: u32 }`
- `item-attribute-modifier { attribute, modifier, slot }`
- `data-component-value { component, value: list<u8> }`

---

## 2. 背包 `inventory`

### 通用资源 `inventory`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-size` | 无 | u32 | 槽位数 |
| `is-empty` | 无 | bool | 是否全空 |
| `get-item(slot)` | 槽 | Option<item-stack> | 取物品 |
| `set-item(slot, item)` | 槽、物品 | 无 | 放物品 |
| `remove-item(slot)` | 槽 | Option<item-stack> | 移除并返回 |
| `clear` | 无 | 无 | 清空 |
| `get-all-items` / `set-all-items(items)` | Option[] | Option[] / 无 | 全部槽 |
| `count-item(item-id)` | 物品 ID | u32 | 统计数量 |
| `contains-item(item-id)` | 物品 ID | bool | 是否包含 |

### 玩家背包 `player-inventory`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `as-inventory` | 无 | inventory | 转 36 槽通用背包（快捷栏+主存储） |
| `get/set-item-in-hand(hand, item)` | hand、物品 | Option / 无 | 手上物品 |
| `get/set-selected-slot(slot)` | u8 | u8 / 无 | 快捷栏当前槽 0–8 |
| `get/set-helmet`、`chestplate`、`leggings`、`boots`、`off-hand` | 物品 | Option / 无 | 各装备槽 |
| `clear-armor` / `clear-main` / `clear-all` | 无 | 无 | 清空装备/主背包/全部 |

---

## 3. 合成配方 `recipe`

### 枚举 `recipe-category`：building / redstone / equipment / misc / food / blocks
### variant `ingredient`
- `item(string)` — 指定物品
- `tag(string)` — 标签组（`minecraft:logs`）
- `one-of(list<string>)` — 多种之一

### 记录
- `shaped-recipe { pattern: string[], key: tuple<string, ingredient>[], output: item-stack, group: Option<string>, category: Option<recipe-category>, show-notification: Option<bool> }`
- `shapeless-recipe { ingredients: ingredient[], output, group, category }`
- `cooking-recipe { ingredient, output, experience: f32, cooking-time: u32, group, category }`
- 枚举 `cooking-type`：smelting / blasting / smoking / campfire

### 资源 `recipe-manager`（`server.get-recipe-manager()`）

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `register-shaped(id, recipe)` | 配方 ID、配方 | 无 | 注册有序合成 |
| `register-shapeless(id, recipe)` | 配方 ID、配方 | 无 | 注册无序合成 |
| `register-cooking(id, station-type, recipe)` | ID、炉具类型、配方 | 无 | 注册烹饪配方 |

---

## 4. 附魔 `enchantments`

### 枚举 `attribute-modifier-slot`
`any`、`main-hand`、`off-hand`、`hand`、`feet`、`legs`、`chest`、`head`、`armor`、`body`、`saddle`

### 枚举 `enchantment`（原版 44 种）
aqua-affinity、bane-of-arthropods、binding-curse、blast-protection、breach、channeling、density、depth-strider、efficiency、feather-falling、fire-aspect、fire-protection、flame、fortune、frost-walker、impaling、infinity、knockback、looting、loyalty、luck-of-the-sea、lunge、lure、mending、multishot、piercing、power、projectile-protection、protection、punch、quick-charge、respiration、riptide、sharpness、silk-touch、smite、soul-speed、sweeping-edge、swift-sneak、thorns、unbreaking、vanishing-curse、wind-burst

### 记录 `custom-enchantment`
`id`、`description: text-component`、`max-level: u32`、`anvil-cost: u32`、`supported-items: string`、`weight: u32`、`slots: attribute-modifier-slot[]`、`exclusive-set: string[]`

### 资源 `enchantment-manager`（`server.get-enchantment-manager()`）

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `register-enchantment(enchantment)` | 自定义附魔 | Result<_, string> | 注册 |
| `get-enchantment(id)` | ID | Option<custom-enchantment> | 查询 |
| `has-enchantment(id)` | ID | bool | 是否存在 |
| `get-all-enchantment-ids()` | 无 | string[] | 全部自定义附魔 ID |

---

## 5. 药水 `potions`

- 枚举 `potion-type`：46 种（water、mundane、thick、awkward、night-vision、long-night-vision、invisibility、strength、healing、regeneration、poison、harming、swiftness、slowness、turtle-master、luck、wind-charged 等）。
  用于构造 `item-stack` 的药水数据组件。

---

## 6. 状态效果 `status-effect`

- 枚举 `status-effect-type`：40 种（speed、slowness、haste、mining-fatigue、strength、instant-health、instant-damage、jump-boost、nausea、regeneration、resistance、fire-resistance、water-breathing、invisibility、blindness、night-vision、hunger、weakness、poison、wither、health-boost、absorption、saturation、glowing、levitation、luck、unluck、slow-falling、conduit-power、dolphins-grace、bad-omen、hero-of-the-village、darkness、trial-omen、raid-omen、wind-charged、weaving、oozing、infested）
- 记录 `status-effect-instance { effect-type, duration: u32, amplifier: u8, ambient: bool, show-particles: bool, show-icon: bool }`
- 通过 `player.add/remove-effect`、`get-effect`、`get-active-effects` 操作。

---

## 7. 属性 `attributes`

- 枚举 `attribute`：40 种（max-health、attack-damage、movement-speed、armor、armor-toughness、knockback-resistance、luck、scale、step-height、water-movement-efficiency、waypoint-transmit-range 等）。
- 枚举 `modifier-operation`：`add` / `multiply-base` / `multiply-total`
- 记录 `attribute-modifier { id: string, amount: f64, operation }`
- 通过 `living-entity` 的属性方法读写，或用于 `item-stack` 的属性修饰符。

---

## 8. 数据组件 `data-components`

- 枚举 `data-component`：122 种（custom-data、max-stack-size、max-damage、damage、unbreakable、use-effects、custom-name、minimum-attack-charge、enchantments、food、potion-contents、tool、repair-cost 等原版数据组件）。
- 通过 `item-stack.get/set/remove-component` 操作，值以序列化字节 `list<u8>` 传输。

---

## 9. 伤害类型 `damage-types`

- 枚举 `damage-type`：51 种（arrow、bad-respawn-point、cactus、campfire、cramming、dragon-breath、drown、dry-out、fall、fireball、generic、in-fire、in-wall、lava、lightning-bolt、magic、mob-attack、player-attack、starve、thorns、void、wither 等）。
- 用于 `living-entity.damage(amount, damage-type)`、`player.damage(...)`。

---

## 10. 实体类型 `entity-types`

- 枚举 `entity-type`：161 种（acacia-boat、allay、armadillo、armor-stand、arrow、axolotl、bat、bee、blaze、camel、cat、chicken、cow、creeper、dragon-fireball、drowned、elder-guardian、ender-dragon、enderman、evoker、fox、ghast、giant、goat、guardian、hoglin、horse、husk、illusioner、iron-golem、llama、magma-cube、mooshroom、ocelot、panda、parrot、phantom、pig、piglin、pillager、polar-bear、pufferfish、rabbit、ravager、salmon、sheep、shulker、silverfish、skeleton、slime、snow-golem、spider、squid、stray、strider、trader-llama、tropical-fish、turtle、vex、villager、vindicator、wandering-trader、warden、witch、wither、wolf、zoglin、zombie、zombie-villager、zombified-piglin 等）。
- 用于 `world.spawn-entity`、`entity.get-type`。

---

## 11. 实体状态 `entity-statuses`

- 枚举 `entity-status`：64 种（armadillo-peek、armorstand-wobble、boat-launch、boat-sink、body-break、chest-break、death、eating、firework-explode、honey-slide、hurt、love-hearts、shake-wetness、sheep-eat、squid-ink、stop-sleeping、tnt-prime、witch-drink 等）。
- 通过 `java-player.send-entity-status(entity-id, status)` 发送。
