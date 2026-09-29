# Sound & Particle API（音效 / 粒子接口）

> WIT 模块：`sounds`、`particles`
> 命名说明：WIT kebab-case 在 TS 绑定中为 camelCase。

这两个模块主要是**枚举清单**（可用音效、粒子类型），配合 `player` / `world` 的播放方法使用。

---

## 1. 音效 `sounds`

### 枚举 `sound-category`（音效频道，11 种）
`master`、`music`、`records`、`weather`、`blocks`、`hostile`、`neutral`、`players`、`ambient`、`voice`、`ui`

### 枚举 `sound`（**2002 个**可用音效）
按命名前缀大致分组（实际以 `.wit` 为准）：

| 前缀组 | 示例 |
|---|---|
| `entity-*` | entity-allay-ambient、entity-creeper-death、entity-player-hurt、entity-zombie-ambient 等实体音效 |
| `ambient-*` | ambient-cave、ambient-underwater-loop、ambient-nether-wastes-additions 等环境音 |
| `block-*` | block-amethyst-block-break、block-chest-open、block-grindstone-use 等方块音 |
| `item-*` | item-totem-use、item-bucket-fill、item-trident-throw 等物品音 |
| `music-*` / `music-disc-*` | 背景音乐与唱片 |
| `ui-*` | 界面按钮等 |
| `weather-*` / `enchant-*` / `particle-*` 等 | 其它 |

### 播放方法
- 世界级：`world.play-sound(sound, category, pos, volume, pitch)`、`world.play-custom-sound(name, category, pos, volume, pitch)`
- 玩家级：`player.play-sound(sound, category, volume, pitch)`、`player.play-sound-at(pos, ...)`、`player.stop-sound(sound?, category?)`
- 玩家级自定义：`player.play-custom-sound(name, category, volume, pitch)`、`play-custom-sound-at`、`stop-custom-sound`
- 命令参数：`sound-category` 见 `command` 模块。

> 自定义音效可用任意资源包音效标识符字符串（`namespace:sound`），无需用 `sound` 枚举。

---

## 2. 粒子 `particles`

### 枚举 `particle`（**128 种**）
示例：`angry-villager`、`block`、`block-marker`、`bubble`、`sulfur-bubbles`、`noxious-gas`、`cloud`、`copper-fire-flame`、`crit`、`damage-indicator`、`dragon-breath`、`dripping-lava`、`falling-lava`、`landing-lava`、`dripping-water`、`falling-water`、`dust`、`dust-color-transition`、`effect`、`elder-guardian`、`enchant`、`end-rod`、`explosion-emitter`、`explosion`、`gust`、`small-gust`、`sonic-boom`、`firework`、`flame`、`infested`、`sculk-soul`、`sculk-charge`、`soul-fire-flame`、`heart`、`item`、`vibration`、`item-slime`、`large-smoke`、`lava`、`note`、`poof`、`portal`、`rain`、`smoke`、`snowflake`、`totem-of-undying`、`underwater`、`splash`、`witch`、`dolphin`、`campfire-cosy-smoke`、`ash`、`warped-spore`、`glow-squid-ink`、`electric-spark`、`shriek`、`egg-crack`、`dust-plume`、`trial-spawner-detection`、`vault-connection`、`ominous-spawning`、`raid-omen`、`firefly`、`sulfur-cube-goo` … 等。

### 生成方法
- 世界级：`world.spawn-particle(particle, pos, offset, max-speed, count)`
- 玩家级（仅该玩家可见）：`player.spawn-particles(particle, pos, count, offset, max-speed)`
- 命令参数：`particle` 见 `command` 模块。

---

## 3. 使用示例

```ts
// 世界播放音效
world.playSound("entity-player-levelup", "players", pos, 1.0, 1.0);

// 给单个玩家播放
player.playSound("block-note-block-harp", "records", 1.0, 1.0);

// 世界生成粒子
world.spawnParticle("flame", pos, { x: 0, y: 0.5, z: 0 }, 0.1, 5);

// 只给某玩家看
player.spawnParticles("heart", pos, 3, { x: 0.2, y: 0.5, z: 0.2 }, 0.1);
```
