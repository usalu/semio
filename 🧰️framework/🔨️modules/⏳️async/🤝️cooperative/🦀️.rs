//#region 🧪️CooperativeMaintenance
use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture { cases: Vec<Case> }

#[derive(Deserialize)]
struct Case { lane: Lane, weight: u32, deficits: Vec<i64>, selected: Vec<bool> }

#[cfg(test)]
include!("🧪️tests/🔬️standalone/🦀️.rs");





fn exact_live_pump_binding(source: &str) -> bool {
    let Some(helper_start) = source.find("fn pump_runtime_live_cooperative_turn<") else { return false };
    if source.matches("fn pump_runtime_live_cooperative_turn<").count() != 1 { return false }
    let Some(start) = source.find("pub fn plugin_step_live_cleanup<") else { return false };
    if helper_start >= start { return false }
    let helper = &source[helper_start..start];
    if helper.matches("pool.pump(now_ms);").count() != 1 || !helper.contains("let now_ms = semio_framework_job::default_now_ms();") || ["while ", "loop {", "for "].iter().any(|pattern| helper.contains(pattern)) { return false }
    let source = &source[start..];
    let marker = "RuntimeMaintenanceStatus::Queued | RuntimeMaintenanceStatus::Running => {";
    let Some(start) = source.find(marker).map(|start| start + marker.len()) else { return false };
    let mut depth = 1;
    let Some(length) = source[start..].char_indices().find_map(|(index, character)| {
        if character == '{' { depth += 1; }
        if character == '}' { depth -= 1; }
        (depth == 0).then_some(index)
    }) else { return false };
    let branch = &source[start..start + length];
    branch.matches("pump_runtime_live_cooperative_turn(&cell)").count() == 1 && !["while ", "loop {", "for "].iter().any(|pattern| branch.contains(pattern))
}
//#endregion 🧪️CooperativeMaintenance
