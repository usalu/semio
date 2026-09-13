//! 🪟️ An action declared by a window body is addressed to THAT window instance.
//!
//! `ShellState::dispatch_action` builds `ActionAddress::window_instance_id` from the action's own
//! `windowId` argument first and falls back to the focused window. A retained body's row action
//! carried no `windowId`, so pressing `Add Generation` in the Generations window was addressed to
//! `procedural-main` and the guest refused it:
//! `handle_action promise failed: window kind procedural-main does not own action addGeneration`
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.3).
//!
//! These lanes drive the production `scope_action_to_window` over
//! `🧫️fixtures/🪟️action-window-scope/🔣️.json` and re-derive the shell's own address resolution, so
//! the fixture pins both the rewrite and what the rewrite BUYS.

use super::{scope_action_to_window, ActionDescriptor, DslValue};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🪟️action-window-scope/🔣️.json")).expect("action window scope fixture parses")
}

fn pairs(value: &Value) -> Vec<(String, String)> {
    value.as_array().map_or_else(Vec::new, |entries| {
        entries
            .iter()
            .map(|entry| {
                let pair = entry.as_array().expect("arg pair");
                (pair[0].as_str().expect("arg key").to_string(), pair[1].as_str().expect("arg value").to_string())
            })
            .collect()
    })
}

fn descriptor(case: &Value) -> ActionDescriptor {
    let args = case["args"].as_array().map(|_| DslValue::Object(pairs(&case["args"]).into_iter().map(|(key, value)| (key, DslValue::String(value))).collect()));
    ActionDescriptor { controller_id: "procedural".into(), action: "addGeneration".into(), args }
}

fn arg_pairs(action: &ActionDescriptor) -> Vec<(String, String)> {
    match action.args.as_ref() {
        Some(DslValue::Object(entries)) => entries.iter().map(|(key, value)| (key.clone(), value.as_str().unwrap_or_default().to_string())).collect(),
        _ => Vec::new(),
    }
}

/// 🪟️ The shell's own resolution order, re-derived: the action's `windowId` wins, else the focused
/// window. See `dispatch_action`'s `window_instance_id`.
fn resolve_window_instance(action: &ActionDescriptor, focused: &str) -> String {
    arg_pairs(action).into_iter().find(|(key, _)| key == "windowId").map(|(_, value)| value).unwrap_or_else(|| focused.to_string())
}

#[test]
fn scoping_binds_every_fixture_action_to_the_window_that_declared_it() {
    let fixture = fixture();
    let focused = fixture["focusedWindowId"].as_str().expect("focusedWindowId");
    let cases = fixture["cases"].as_array().expect("cases");
    assert!(!cases.is_empty(), "fixture declares no cases");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let window_id = case["windowId"].as_str().expect("windowId");
        let mut action = descriptor(case);
        let before = resolve_window_instance(&action, focused);
        scope_action_to_window(&mut action, window_id);
        let after = arg_pairs(&action);
        println!("[DEBUG] action-window-scope {name}: {before} -> {after:?}");
        assert_eq!(after, pairs(&case["expectedArgs"]), "{name}: scoped arguments");
        assert_eq!(after.iter().filter(|(key, _)| key == "windowId").count(), 1, "{name}: exactly one windowId survives");
        assert_eq!(resolve_window_instance(&action, focused), case["resolvesTo"].as_str().expect("resolvesTo"), "{name}: resolved window instance");
        assert_eq!(before, case["baselineResolvesTo"].as_str().expect("baselineResolvesTo"), "{name}: what the unscoped action resolved to");
    }
}

#[test]
fn scoping_is_idempotent_and_the_defect_case_actually_changes() {
    let fixture = fixture();
    let focused = fixture["focusedWindowId"].as_str().expect("focusedWindowId");
    let mut changed = 0usize;
    for case in fixture["cases"].as_array().expect("cases") {
        let window_id = case["windowId"].as_str().expect("windowId");
        let mut once = descriptor(case);
        scope_action_to_window(&mut once, window_id);
        let mut twice = once.clone();
        scope_action_to_window(&mut twice, window_id);
        assert_eq!(arg_pairs(&once), arg_pairs(&twice), "{}: scoping twice differs from scoping once", case["name"]);
        if case["resolvesTo"] != case["baselineResolvesTo"] {
            changed += 1;
            assert_ne!(resolve_window_instance(&once, focused), case["baselineResolvesTo"].as_str().expect("baselineResolvesTo"));
        }
    }
    assert!(changed >= 3, "the fixture must carry cases the unscoped path got wrong, got {changed}");
}
