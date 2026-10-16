//! 🪝️ `shell-window-actions` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region ⏯️ToolRunPanel
/// ⏯️ Whether the freshly rendered framework ToolRun panel `document` holds a run `seen` has not — the moment the shell
/// reveals the panel, exactly as the React `ShellHost` does through `toolRunPanelReveal`. The group-key law is
/// [`semio_framework_tool_run::tool_run_panel_new_runs`], pinned by the `⏯️tool-run` lifecycle fixture.
pub(crate) fn tool_run_panel_reveals(seen: &mut std::collections::BTreeSet<u64>, document: &UiDocumentLease) -> Result<bool, String> {
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

//#endregion ⏯️ToolRunPanel
//#endregion ShellLifecycle

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🎨️ A colour row's staged value: the hex its swatch or hex field committed (`ui_contract::parse_ui_color_hex`, keeping
/// the alpha when the text carries none) or the alpha its slider committed, over the colour already staged, else the
/// one the row painted; an unreadable hex keeps the colour.
pub(crate) fn staged_color_value(color: &Value, value: Option<&Value>, current: Option<&Value>) -> Value {
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
