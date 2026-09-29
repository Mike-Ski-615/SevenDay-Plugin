# vendor/

`pumpkin-plugin-api` 的**内置副本**，被本仓库内**所有插件共享**。

各插件在自己的 `Cargo.toml` 里引用：

```toml
pumpkin-plugin-api = { path = "../vendor/pumpkin-plugin-api" }
```

目的是把插件 API 版本**钉死在仓库里**，不再使用 `git = "...", rev = "..."` 这种构建时联网、
且随上游变动的写法。

- **来源**：`Pumpkin-MC/Pumpkin` @ commit `4426d1113a211e6018a2db416e33b6b8a7802614`（nightly）
- **内容**：
  - `pumpkin-plugin-api/` —— crate 源码（`crates/pumpkin-plugin-api/`）
  - `pumpkin-plugin-wit/v0.1/` —— WIT 定义（`crates/pumpkin-plugin-wit/v0.1/`）
- **改动**：`pumpkin-plugin-api/Cargo.toml` 里的 `workspace = true` 继承项已展开为具体值，
  并去掉了 `[lints] workspace = true`，使其能脱离原 workspace 独立编译。

## 为什么必须是这个 commit

宿主的插件 WIT 决定 ABI，**版本号会骗人**。唯一的判断依据是 WIT 文件的 git blob sha：

```
crates/pumpkin-plugin-wit/v0.1/event.wit
  blob sha = 5a220b016b4ce11ce1a7e4f4722f6372ed39f976   ← 与本服务器一致

（作为对照：tag 0.2.0+26.3-26.51 的该文件是 1d281d01…，装上去会报
  "Plugin is built against a different API version"，即使版本号一模一样。）
```

## 服务器升级后如何更新

1. 拿到新服务器的 `event.wit` 等文件，比对 blob sha，确定对应 commit
2. 重新拷贝 `crates/pumpkin-plugin-api/` 与 `crates/pumpkin-plugin-wit/v0.1/` 到本目录
3. 重新展开新 `Cargo.toml` 里的 workspace 继承项（见上）
4. `bash build.sh` 验证

> 校验 blob sha 的小技巧：`git hash-object <文件>` 输出的就是该 sha。
