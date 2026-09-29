# UI API（文本 / GUI / 记分板 / 展示实体接口）

> WIT 模块：`text`、`gui`、`screens`、`forms`（基岩表单）、`java-dialogs`（Java 对话框）、`boss-bar`、`scoreboard`、`display`
> 命名说明：WIT kebab-case 名在 TS 绑定中为 camelCase。

---

## 1. 文本组件 `text`

### `resource text-component`

#### 静态构造
| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `text(plain)` | 纯文本 | text-component | 普通文本 |
| `translate(key, with)` | 翻译键、参数列表 | text-component | 翻译组件 |
| `translate-cross(java-key, bedrock-key, with)` | 双端键、参数 | text-component | 跨版本翻译 |
| `entity-names(selector, separator)` | 选择器、分隔符 | text-component | 实体名 |
| `keybind(keybind)` | 按键名（`key.jump`） | text-component | 按键提示 |
| `custom(namespace, key, locale, with)` | 命名空间、键、语言、参数 | text-component | 自定义翻译 |
| `from-legacy-string(input)` | § 格式文本 | text-component | 旧版格式解析 |
| `from-legacy-string-with-code(input, code-symbol)` | 文本、符号 | text-component | 自定义符号解析 |
| `from-json(json)` | JSON 字符串 | Result<text-component, string> | JSON 解析 |
| `to-json()` | — | string | 序列化为 JSON |

#### 内容
`add-child(child)`、`add-text(text)`、`get-text() -> string`、`encode() -> list<u8>`、`to-pretty-console() -> string`

#### 样式
`color-named(color)`、`color-rgb(color)`、`gradient-named(colors)`、`gradient(colors)`、`rainbow()`、`bold(bool)`、`italic(bool)`、`underlined(bool)`、`strikethrough(bool)`、`obfuscated(bool)`、`insertion(text)`、`font(font)`、`shadow-color(argb-color)`

#### 点击事件
`click-open-url(url)`、`click-open-file(path)`、`click-run-command(command)`、`click-suggest-command(command)`、`click-change-page(page)`、`click-copy-to-clipboard(text)`

#### 悬停事件
`hover-show-text(text)`、`hover-show-item(item: string)`（SNBT）、`hover-show-entity(entity-type, id, name)`

---

## 2. GUI `gui`

### 构造
`new gui(type: screen, title: text-component)`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get-inventory` | 无 | inventory | 底层容器 |
| `set-item(slot, item)` | 槽、物品 | 无 | 放物品 |
| `get-item(slot)` | 槽 | Option<item-stack> | 取物品 |
| `get-type` / `get-title` / `get-size` | 无 | screen / text-component / u32 | 类型/标题/槽数 |
| `clear-items` | 无 | 无 | 清空 |
| `set/get-allow-grab-items(allow)` | bool | 无 / bool | 是否允许拿走 |
| `set/get-allow-put-items(allow)` | bool | 无 / bool | 是否允许放入 |

### 枚举 `screen`
generic-9x1 … generic-9x6、generic-3x3、crafter-3x3、anvil、beacon、blast-furnace、brewing-stand、crafting、enchantment、furnace、grindstone、hopper、lectern、loom、merchant、shulker-box、smithing、smoker、cartography-table、stonecutter

打开方式：`player.open-gui(gui)` 或 `player.open-ender-chest()`。

---

## 3. 基岩表单 `forms`

- 枚举 `image-type`：url / path
- `form-image { type, data }`
- `simple-form-button { text, image: Option<form-image> }`
- `simple-form { title, content, buttons: simple-form-button[] }`
- `modal-form { title, content, button1, button2 }`
- variant `custom-form-element`：
  - `label(text)`
  - `toggle(tuple<text, bool>)`
  - `slider(tuple<text, min, max, step, default>)`
  - `step-slider(tuple<text, steps: string[], default: u32>)`
  - `dropdown(tuple<text, options: string[], default: u32>)`
  - `input(tuple<text, placeholder, default>)`
- `custom-form { title, elements: custom-form-element[] }`
- variant `form`：`simple` / `modal` / `custom`

使用：`bedrock-player.open-form(form) -> u32`。

---

## 4. Java 对话框 `java-dialogs`

- 枚举 `dialog-type`：notice / confirmation / multi-action / dialog-list / server-links
- variant `dialog-body`：`plain-message(text)` / `item(item-stack)`
- variant `dialog-input`：`bool` / `text` / `number-range` / `single-option`
  - `dialog-input-bool { label, default-value }`
  - `dialog-input-text { label, placeholder, default-value }`
  - `dialog-input-number-range { label, min-value, max-value, initial-value, step, label-format: Option<string> }`
  - `dialog-input-single-option { label, options: text-component[], initial-index: u32 }`
- 枚举 `after-action`：peek（返回上一界面）/ pop（关闭所有界面）
- `action-button { text, tooltip: Option<text>, width: Option<u32>, action }`
- variant `action`：`open-url(string)` / `custom-click(custom-click-action{ id, payload: Option<list<u8>> })`
- `link { label: link-label, url }`；variant `link-label`：`built-in(link-type)` / `custom(text)`
- 枚举 `link-type`：bug-report、community-guidelines、support、status、feedback、community、website、forums、news、announcements
- `dialog { title, type, body, inputs, buttons, links, after-action: Option, can-close-with-escape, external-title: Option }`

使用：`java-player.show-dialog(dialog)` / `clear-dialog()`。自定义点击事件通过事件 `dialog-click-action-event` 回传。

---

## 5. Boss 血条 `boss-bar`

- 枚举 `boss-bar-color`：pink / blue / red / green / yellow / purple / white
- 枚举 `boss-bar-division`：no-division / notches-6 / notches-10 / notches-12 / notches-20
- `boss-bar-metadata { darken-sky, dragon-bar, create-fog }`

### `resource boss-bar`
构造 `new boss-bar(title, color, division)`

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `get/set-title(title)` | 文本 | 文本 / 无 | 标题 |
| `get/set-health(health)` | f32 0.0–1.0 | f32 / 无 | 血量 |
| `get/set-color(color)` | boss-bar-color | color / 无 | 颜色 |
| `get/set-division(division)` | boss-bar-division | division / 无 | 分格 |
| `get/set-metadata(metadata)` | metadata | metadata / 无 | 元数据 |
| `add-player(player)` / `remove-player(player)` | 玩家 | 无 | 显示/隐藏给玩家 |
| `get-players()` | 无 | player[] | 可见玩家 |
| `remove-all()` | 无 | 无 | 全部移除并清理 |

---

## 6. 记分板 `scoreboard`

- 枚举 `render-type`：integer / hearts
- variant `number-format`：`blank` / `fixed(text-component)`
- 枚举 `display-slot`：player-list、sidebar、below-name、sidebar-team-*（16 种队伍颜色）
- 枚举 `nametag-visibility`：always / never / hide-for-other-teams / hide-for-own-team
- 枚举 `collision-rule`：always / never / push-other-teams / push-own-team
- `team-settings { display-name, friendly-fire, see-friendly-invisibles, nametag-visibility, collision-rule, color, prefix, suffix }`

### `resource scoreboard`（`world.get-scoreboard()`）

| 方法 | 接收 | 返回 | 说明 |
|---|---|---|---|
| `add-objective(name, display-name, render-type, number-format)` | 名称、显示名、渲染、格式 | 无 | 新建计分项 |
| `update-objective(...)` | 同上 | 无 | 更新 |
| `remove-objective(name)` | 名称 | 无 | 删除 |
| `set-display-slot(slot, objective-name)` | 槽位、计分项 | 无 | 显示位置 |
| `clear-display-slot(slot)` | 槽位 | 无 | 清除显示 |
| `update-score(entity-name, objective-name, value, number-format)` | 实体名、计分项、值、格式 | 无 | 设置分数 |
| `add-score(entity-name, objective-name, delta) -> s32` | 实体名、计分项、增量 | s32 | 加分并返回新值 |
| `remove-score(entity-name, objective-name)` | 实体名、计分项 | 无 | 删除分数 |
| `reset-entity-scores(entity-name)` | 实体名 | 无 | 重置该实体所有分数 |
| `create-team(name, settings)` / `remove-team(name)` / `update-team(name, settings)` | 队伍名、设置 | 无 | 队伍管理 |
| `add-player-to-team(team-name, player-name)` / `remove-player-from-team(...)` / `clear-team-players(team-name)` | 队伍、玩家 | 无 | 队伍成员 |
| `get-teams()` / `get-team(name)` / `get-team-players(team-name)` / `get-player-team(player-name)` | — | string[] / Option / string[] / Option<string> | 查询 |

### Bedrock 记分板 `bedrock-scoreboard`（`bedrock-player.get-scoreboard()`）
- 枚举 `bedrock-sort-order`：ascending / descending
- 枚举 `bedrock-display-slot`：player-list / sidebar / below-name
- 方法同 scoreboard 的 objective/score 部分（`add/update/remove-objective`、`set/clear-display-slot`、`update/add/remove-score`、`reset-entity-scores`）。

---

## 7. 展示实体 `display`

### 枚举与记录
- `billboard-mode`：fixed / vertical / horizontal / center
- `item-display-mode`：none / thirdperson-lefthand / thirdperson-righthand / firstperson-lefthand / firstperson-righthand / head / gui / ground / fixed
- `text-alignment`：center / left / right
- `vector3f { x, y, z }`、`quaternionf { x, y, z, w }`
- `display-transformation { translation, scale, left-rotation, right-rotation }`

### `resource display-entity`（所有展示实体基类）
`from-entity(entity) -> Option<display-entity>`（静态转换）、`get-entity()`
变换：`get/set-transformation`
插值：`get/set-interpolation-duration`、`get/set-interpolation-start`、`get/set-teleport-duration`
渲染：`get/set-billboard`、`get/set-view-range`、`get/set-shadow-radius`、`get/set-shadow-strength`、`get/set-display-width`、`get/set-display-height`、`get/set-glow-color-override`、`get/set-brightness`

### `resource block-display-entity`
`from-entity`、`get-display`、`get-entity`、`get/set-block-state-id(state-id)`

### `resource item-display-entity`
`from-entity`、`get-display`、`get-entity`、`get/set-item(Option<item-stack>)`、`get/set-item-display-mode`

### `resource text-display-entity`
`from-entity`、`get-display`、`get-entity`
`get/set-text`、`get/set-line-width`、`get/set-background`、`get/set-text-opacity`、`get/set-shadow`、`get/set-see-through`、`get/set-default-background`、`get/set-alignment`

### `resource interaction-entity`
`from-entity`、`get-entity`、`get/set-width`、`get/set-height`、`get/set-response`、`get-last-attacker() -> Option<uuid>`、`get-last-interaction() -> Option<uuid>`

> 展示实体本质是 `entity`，先用 `from-entity` 转换判断类型，再访问专属属性。
