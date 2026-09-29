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
pub fn write_file(dir: &str, state: &DayState) -> std::io::Result<()> {
    std::fs::write(format!("{dir}/{FILE_NAME}"), encode(state))
}
pub fn read_file(dir: &str) -> Option<DayState> {
    std::fs::read_to_string(format!("{dir}/{FILE_NAME}"))
        .ok()
        .and_then(|text| decode(&text))
}
