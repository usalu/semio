fn exact_live_pump_binding(source: &str) -> bool {
    let source = &source.lines().filter(|line| !line.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n");
    let Some(helper_start) = source.find("fn pump_runtime_live_cooperative_turn<") else { return false };
    if source.matches("fn pump_runtime_live_cooperative_turn<").count() != 1 {
        return false;
    }
    let Some(start) = source.find("pub fn plugin_step_live_cleanup<") else { return false };
    if helper_start >= start {
        return false;
    }
    let helper = &source[helper_start..start];
    if helper.matches("pool.pump(now_ms);").count() != 1 || !helper.contains("let now_ms = semio_framework_job::default_now_ms();") || ["while ", "loop {", "for "].iter().any(|pattern| helper.contains(pattern)) {
        return false;
    }
    let source = &source[start..];
    let marker = "RuntimeMaintenanceStatus::Queued | RuntimeMaintenanceStatus::Running => {";
    let Some(start) = source.find(marker).map(|start| start + marker.len()) else { return false };
    let mut depth = 1;
    let Some(length) = source[start..].char_indices().find_map(|(index, character)| {
        if character == '{' {
            depth += 1;
        }
        if character == '}' {
            depth -= 1;
        }
        (depth == 0).then_some(index)
    }) else {
        return false;
    };
    let branch = &source[start..start + length];
    branch.matches("pump_runtime_live_cooperative_turn(cell)").count() == 1 && !["while ", "loop {", "for "].iter().any(|pattern| branch.contains(pattern))
}

#[test]
fn cooperative_maintenance_live_host_revisits_queued_owner() {
    let source = include_str!("../../🦀️.rs");
    assert!(exact_live_pump_binding(source), "queued maintenance must retain one actual host pump opportunity");
    for hostile in [
        source.replace("pump_runtime_live_cooperative_turn(cell)?;", "other_pump(cell)?;"),
        source.replace("pump_runtime_live_cooperative_turn(cell)?;", ""),
        source.replace("pump_runtime_live_cooperative_turn(cell)?;", "pump_runtime_live_cooperative_turn(cell)?; pump_runtime_live_cooperative_turn(cell)?;"),
        source.replace("pool.pump(now_ms);", "while pool.pump(now_ms) {}"),
        source.replace("let now_ms = semio_framework_job::default_now_ms();", "let now_ms = Some(0);"),
    ] {
        assert!(!exact_live_pump_binding(&hostile));
    }
}
