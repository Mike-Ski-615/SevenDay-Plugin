# Network API（网络数据包接口）

> WIT 模块：`java-packets`、`bedrock-packets`
> 命名说明：WIT kebab-case 在 TS 绑定中为 camelCase。

这两个模块**不是可调用的函数 API，而是原版协议数据包的 TypeScript 类型定义**（record/variant），用于：

- 在 `packet-received-event` / `packet-sent-event` 事件中检查和修改“收到/发出的包”。
- 通过 `java-player.send-packet(packet)` / `bedrock-player.send-packet(packet)` 主动发送客户端包。

> 各包的具体字段定义在 `node_modules/@pumpkinmc/pumpkin-api-ts/wit/v0.1/java-packets.wit` 和 `bedrock-packets.wit` 中；下方按类别列出包名，字段较多不逐一展开。

---

## 1. Java 数据包 `java-packets`

### 顶层 variant
- `serverbound-packet`（客户端 → 服务器，85 种，含 `unknown`）
- `clientbound-packet`（服务器 → 客户端，165 种，含 `unknown`）

### 服务端接收包（serverbound）按阶段
- **Config 阶段**（`config-s-*`）：accept-code-of-conduct、acknowledge-finish-config、client-information-config、config-cookie-response、custom-click-action、keep-alive、known-packs、plugin-message、config-pong、config-resource-pack
- **Login 阶段**（`login-s-*`）：login-cookie-response、login-acknowledged、login-start、login-plugin-response
- **Play 阶段**（`s-*`）：
  - 聊天/命令：chat-ack、chat-command、chat-command-signed、chat-message、client-command、command-suggestion、custom-payload、place-recipe
  - 移动：move-vehicle、paddle-boat、player-position、player-position-rotation、player-rotation、player-input、player-action、set-player-ground、player-loaded、teleport-to-entity、spectate-entity、player-session、player-abilities
  - 背包/交互：click-slot、close-container、container-button-click、container-slot-state-changed、set-creative-slot、set-held-item、set-beacon、select-trade、bundle-item-selected、rename-item、edit-book、update-sign、pick-item-from-block、pick-item-from-entity、use-item、use-item-on、interact、attack
  - 方块/结构：block-entity-tag-query、entity-tag-query、jigsaw-generate、set-command-block、set-command-minecart、set-jigsaw-block、set-structure-block、set-test-block、test-instance-block-action、set-game-rule
  - 其它：change-difficulty、change-game-mode、client-information-play、client-tick-end、cookie-response、debug-sample-subscription、debug-subscription-request、keep-alive、lock-difficulty、play-ping-request、play-pong、recipe-book-change-settings、recipe-book-seen-recipe、resource-pack、seen-advancement、configuration-acknowledged、custom-click-action
- **Status 阶段**：status-ping-request、status-request

### 客户端接收包（clientbound）按阶段
- **Config 阶段**（`config-c-*`）：add/remove-resource-pack、clear-dialog、code-of-conduct、disconnect、cookie-request、custom-report-details、feature-flags、finish-config、known-packs、ping、plugin-message、post-effects、registry-data、reset-chat、server-links、show-dialog、store-cookie、transfer、update-tags
- **Login 阶段**（`login-c-*`）：login-cookie-request、login-disconnect、login-plugin-request、set-compression
- **Play 阶段**（`c-*`）：
  - 玩家：abilities、chat-message、disguised-chat-message、combat-death/enter/end、player-info-update、remove-player-info、player-look-at、player-position、player-rotation、spawn-position、respawn、set-health、set-experience、set-player-inventory、set-player-team、set-selected-slot、set-held-item、set-camera、set-cursor-item、tab-list、action-bar、title-text、title-animation、subtitle、clear-title、delete-chat、award-stats、set-item-cooldown、set-cooldown、open-book、open-screen、open-mount-screen、open-sign-editor、set-passengers
  - 实体：spawn-entity、remove-entities、set-entity-metadata、entity-animation、entity-sound-effect、entity-status、entity-velocity、set-equipment、set-entity-link、update-entity-pos、update-entity-pos-rot、update-entity-rot、head-rot、animation、take-item-entity、teleport-entity、update-attributes、set-entity-motion
  - 世界/方块：block-update、multi-block-update、block-event、block-entity-data、light-update、chunk-data、chunks-biomes、unload-chunk、center-chunk、set-chunk-cache-radius、initialize-world-border、set-border-*、update-time、game-event、world-event、level-event、sound-effect、stop-sound、particle、explosion、set-simulation-distance、game-rule-values、acknowledge-block-change、set-block-destroy-stage
  - 容器/配方：container-content/property/slot、merchant-offers、update-recipes、recipe-book-add/remove/settings、place-ghost-recipe
  - 记分板/进度：display-objective、update-objectives、update-score、reset-score、update-advancements、select-advancements-tab
  - 数据包/状态：custom-payload、disconnect、keep-alive、ping-response、play-ping、server-data、play-server-links、show-dialog、clear-dialog、cookie-request、store-cookie、transfer、wavepoint、start-configuration
- **Status 阶段**：ping-response、status-response

---

## 2. Bedrock 数据包 `bedrock-packets`

### 顶层 variant
- `serverbound-packet`（客户端 → 服务器，29 种，含 `unknown`）
- `clientbound-packet`（服务器 → 客户端，60 种，含 `unknown`）

### `serverbound-packet`（`s-*`）
actor-event、animate、block-pick-request、client-cache-blob-status、client-cache-status、command-request、container-close、emote、emote-list、interact、inventory-transaction、item-stack-request、loading-screen、login、mob-equipment、modal-form-response、packet-violation-warning、player-action、player-auth-input、player-hotbar、request-ability、request-chunk-radius、request-network-settings、resource-pack-client-response、respawn、set-local-player-as-initialized、set-player-inventory-options、text

### `clientbound-packet`（`c-*`）
add-actor、add-item-actor、add-player、available-commands、biome-definition-list、block-actor-data、block-event、boss-event、change-dimension、chunk-radius-updated、client-cache-miss-response、container-open、correct-player-move-prediction、crafting-data、creative-content、disconnect、gamerules-changed、inventory-content、inventory-slot、item-registry、item-stack-response、jigsaw-structure-data、level-event、level-sound-event、mob-effect、mob-equipment、modal-form-request、move-actor-absolute、move-actor-delta、move-player、network-chunk-publisher-update、network-settings、play-status、player-hotbar、player-list、remove-actor、remove-objective、resource-pack-stack-packet、resource-packs-info、set-actor-data、set-actor-link、set-actor-motion、set-difficulty、set-display-objective、set-health、set-player-game-type、set-score、set-spawn-position、set-time、set-title、show-credits、start-game、take-item-actor、transfer、update-abilities、update-attributes、update-block、update-trade、voxel-shapes

---

## 3. 常用辅助类型

- `record command-origin-data`、`inventory-action`、`transaction-data`、`item-stack-request-action`（Bedrock）等用于构造具体包。
- Java 侧的 `argument-type`、`proto-node` 等用于 `c-commands` 包。
- `game-rule-entry`、`game-rule` 等用于 game-rule 相关包。

> 需要某个包的具体字段时，直接查对应 `.wit` 文件的 record 定义；TS 绑定与之一一对应（kebab-case → camelCase）。
