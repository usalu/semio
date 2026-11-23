//! 🪝️ `shell-window-actions` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use semio_renderer_wgpu_shell_doc_open::{TaskManagerToolRun, TaskManagerToolRunControls};

//#region ShellLifecycle
//#region ⏯️ToolRunPanel
/// ⏯️ Whether the freshly rendered framework ToolRun panel `document` holds a run `seen` has not — the moment the shell
/// reveals the panel, exactly as the React `ShellHost` does through `toolRunPanelReveal`. The group-key law is
/// [`semio_framework_tool_run::tool_run_panel_new_runs`], pinned by the `⏯️tool-run` lifecycle fixture.
pub fn tool_run_panel_reveals(seen: &mut std::collections::BTreeSet<u64>, document: &UiDocumentLease) -> Result<bool, String> {
    let header = document.header().map_err(|error| format!("the ToolRun panel header is unreadable: {error:?}"))?;
    let mut keys = Vec::with_capacity(header.node_count);
    for index in 0..header.node_count {
        if let Some(page) = document.read_node_page(index).map_err(|error| format!("the ToolRun panel node {index} is unreadable: {error:?}"))? {
            keys.push(page.record().key.as_str().to_string());
        }
    }
    let (runs, added) = semio_framework_tool_run::tool_run_panel_new_runs(seen, keys.iter().map(String::as_str));
    *seen = runs;
    Ok(!added.is_empty())
}

/// 🏃️ The live runs a rendered ToolRun panel `document` holds, oldest first — React's `toolRunPanelTasksV1`, read from the SAME
/// panel the run's owner renders: the group's accessibility label, its `progress` node and whether it still offers Pause or Resume
/// (a group offering neither is terminal and no task). A run offering Pause is running; one offering only Resume is paused.
pub fn tool_run_panel_tasks(document: &UiDocumentLease) -> Result<Vec<TaskManagerToolRun>, String> {
    use semio_framework_tool_run::{TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, tool_run_panel_group_run};
    use std::collections::{BTreeMap, BTreeSet};
    struct Node {
        label: String,
        progress: Option<(f64, Option<f64>, String)>,
        activate: Option<ActionDescriptor>,
    }
    let header = document.header().map_err(|error| format!("the ToolRun panel header is unreadable: {error:?}"))?;
    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
    for index in 0..header.node_count {
        let Some(page) = document.read_node_page(index).map_err(|error| format!("the ToolRun panel node {index} is unreadable: {error:?}"))? else { continue };
        let record = page.record();
        let progress = match &record.component {
            ui_contract::Component::Progress(props) => Some((props.completed, props.total, props.value_text.to_string())),
            _ => None,
        };
        let label = record.accessibility.label.as_ref().map(|label| label.to_string()).unwrap_or_default();
        let activate = record.bindings.iter().find(|binding| binding.trigger == ui_contract::Trigger::Activate).map(|binding| ActionDescriptor { controller_id: binding.action.scope.as_str().to_string(), action: binding.action.name.as_str().to_string(), args: binding.args.as_ref().and_then(|args| serde_json::to_value(args).ok()).map(semio_framework::DslValue::from) });
        nodes.insert(record.key.as_str().to_string(), Node { label, progress, activate });
    }
    let groups: BTreeSet<(u64, String)> = nodes.keys().filter_map(|key| tool_run_panel_group_run(key).map(|run| (run, key.clone()))).collect();
    Ok(groups
        .into_iter()
        .filter_map(|(run, group)| {
            let control = |action: &str| nodes.get(&format!("{group}.{action}")).and_then(|node| node.activate.clone());
            let controls = TaskManagerToolRunControls { pause: control(TOOL_RUN_PAUSE_ACTION_ID), resume: control(TOOL_RUN_RESUME_ACTION_ID), abort: control(TOOL_RUN_ABORT_ACTION_ID) };
            let (pause, resume) = (controls.pause.is_some(), controls.resume.is_some());
            (pause || resume).then(|| {
                let (completed, total, value_text) = nodes.get(&format!("{group}.progress")).and_then(|node| node.progress.clone()).unwrap_or((0.0, None, String::new()));
                TaskManagerToolRun { run: run.to_string(), label: nodes.get(&group).map(|node| node.label.clone()).unwrap_or_default(), completed: completed.max(0.0) as u64, total: total.map(|total| total.max(0.0) as u64), value_text, paused: !pause, controls }
            })
        })
        .collect())
}

//#endregion ⏯️ToolRunPanel
//#endregion ShellLifecycle

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🎨️ A colour row's staged value: the hex its swatch or hex field committed (`ui_contract::parse_ui_color_hex`, keeping
/// the alpha when the text carries none) or the alpha its slider committed, over the colour already staged, else the
/// one the row painted; an unreadable hex keeps the colour.
pub fn staged_color_value(color: &Value, value: Option<&Value>, current: Option<&Value>) -> Value {
    let alpha = color.get("alpha").and_then(Value::as_bool).unwrap_or(false);
    let mut rgba = [0.0, 0.0, 0.0, 1.0];
    for (slot, component) in rgba.iter_mut().zip(current.or_else(|| color.get("rgba")).and_then(Value::as_array).into_iter().flatten()) {
        *slot = staged_number(component).unwrap_or(0.0).clamp(0.0, 1.0);
    }
    match value {
        Some(Value::String(text)) => {
            if let Some(parsed) = ui_contract::parse_ui_color_hex(text) {
                let typed_alpha = matches!(text.trim().trim_start_matches('#').len(), 4 | 8);
                rgba = [parsed[0], parsed[1], parsed[2], if typed_alpha { parsed[3] } else { rgba[3] }];
            }
        }
        Some(number) => {
            if let Some(number) = staged_number(number) {
                rgba[3] = number.clamp(0.0, 1.0);
            }
        }
        None => {}
    }
    Value::Array(rgba[..if alpha { 4 } else { 3 }].iter().map(|component| Value::from(*component)).collect())
}
//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle
