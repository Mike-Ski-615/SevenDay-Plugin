//! 状态持久化。
//!
//! Rust 插件不走 world NBT，而是用插件私有数据目录下的普通文件
//! （需要 metadata 里申请 `fs.read.data` / `fs.write.data` 权限）。
//!
//! 格式刻意用极简的 `key=value` 文本，免掉 serde 依赖。

use crate::state::DayState;

const FILE_NAME: &str = "state.txt";

pub fn encode(state: &DayState) -> String {
    format!(
        "day={}\nround={}\nboundary={}\n",
        state.day, state.round, state.boundary_at
    )
}

pub fn decode(text: &str) -> Option<DayState> {
    let mut day = None;
    let mut round = None;
    let mut boundary = None;
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "day" => day = value.trim().parse().ok(),
            "round" => round = value.trim().parse().ok(),
            "boundary" => boundary = value.trim().parse().ok(),
            _ => {}
        }
    }
    Some(DayState {
        day: day?,
        round: round?,
        boundary_at: boundary?,
    })
}

/// 写入插件数据目录；目录为空或写失败时返回错误。
pub fn write_file(dir: &str, state: &DayState) -> std::io::Result<()> {
    std::fs::write(format!("{dir}/{FILE_NAME}"), encode(state))
}

/// 读取插件数据目录；文件不存在或格式不合法时返回 None。
pub fn read_file(dir: &str) -> Option<DayState> {
    std::fs::read_to_string(format!("{dir}/{FILE_NAME}"))
        .ok()
        .and_then(|text| decode(&text))
}
