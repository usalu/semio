/// ✏️ Focus row control: the one `focusRowControl` verb moves the keyboard into the first editable cell of a tree or grid row (`F2` on the row raises it), announces a row without such a cell politely in the interface language, and leaves the typing law (Escape reverts to the committed value, Enter commits) to the shared text keyboard law.

fn focus_row_control_law() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎯️focus-row-control/🔣️.json")).unwrap()
}

#[test]
fn the_row_control_verb_shares_its_names_and_picks_the_cell_like_the_language_agnostic_rows() {
    let law = focus_row_control_law();
    assert_eq!(law["action"], semio_framework::FOCUS_ROW_CONTROL_ACTION_ID);
    assert_eq!(law["argument"], semio_framework::FOCUS_ROW_CONTROL_ARG_ROW);
    assert_eq!(law["key"], semio_framework::FOCUS_ROW_CONTROL_KEY);
    let kinds: Vec<&str> = law["editableKinds"].as_array().unwrap().iter().map(|kind| kind.as_str().unwrap()).collect();
    assert_eq!(kinds, semio_framework::FOCUS_ROW_CONTROL_EDITABLE_KINDS);
    for row in law["plans"].as_array().unwrap() {
        let controls: Vec<semio_framework::RowControl<'_>> = row["controls"].as_array().unwrap().iter().map(|control| semio_framework::RowControl { id: control["id"].as_str().unwrap(), kind: control["kind"].as_str().unwrap(), disabled: control["disabled"].as_bool().unwrap_or(false) }).collect();
        assert_eq!(semio_framework::plan_row_control_focus(&controls), row["target"].as_str(), "{}", row["id"]);
    }
}

fn focus_row_control_action(row: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: "framework".into(), action: semio_framework::FOCUS_ROW_CONTROL_ACTION_ID.into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "rowId": row }) }
}

#[test]
fn a_row_nobody_presents_is_refused_and_announced_in_both_languages_like_the_language_agnostic_rows() {
    let law = focus_row_control_law();
    for row in law["announcements"].as_array().unwrap() {
        let mut shell = focus_window_shell();
        shell.prefs.locale_id = row["locale"].as_str().unwrap().into();
        let dispatched = semio_framework_async::block_on(semio_renderer_wgpu_shell_actions::ActionsCtx::of(&mut shell).dispatch_action(focus_row_control_action("table.marker.m0")));
        assert!(dispatched.is_err(), "{}: a row without an editable cell is refused", row["locale"]);
        let notice = shell.chrome.transient_notice().expect("the refusal is announced");
        assert_eq!((notice.message.as_str(), notice.code.as_deref()), (row["message"].as_str().unwrap(), row["code"].as_str()), "{}", row["locale"]);
        assert!(matches!(notice.severity, semio_framework::Severity::Warning), "{}", row["locale"]);
        assert!(shell.input.pending_row_control_focus.is_none(), "a refusal queues no focus");
    }
    let mut shell = focus_window_shell();
    let missing = semio_framework_async::block_on(semio_renderer_wgpu_shell_actions::ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: semio_framework::FOCUS_ROW_CONTROL_ACTION_ID.into(), args: None }));
    assert!(missing.is_err_and(|reason| reason.contains(semio_framework::FOCUS_ROW_CONTROL_ARG_ROW)), "the row is a required argument");
    for german in [false, true] {
        assert!(!shell_chrome_string("row.control.refused", german).is_empty());
    }
}
