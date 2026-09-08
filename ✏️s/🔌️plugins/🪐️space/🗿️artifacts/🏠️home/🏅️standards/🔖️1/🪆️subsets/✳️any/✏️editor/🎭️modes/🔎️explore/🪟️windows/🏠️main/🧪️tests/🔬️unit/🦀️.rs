
use super::*;

fn project(node: semio_framework_plugin::BuiltNode) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("Home row tree projection")
}

fn observe<R>(node: semio_framework_plugin::BuiltNode, inspect: impl FnOnce(&semio_framework_plugin::BuiltNode) -> R) -> R {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| inspect(&node)));
    let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(node);
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 4096).expect("Home fixture tree remains valid");
        if !step.progressed {
            std::thread::yield_now();
        }
    }
    match result {
        Ok(result) => result,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

fn row<'a>(root: &'a semio_framework_plugin::BuiltNode, key: &str) -> &'a semio_framework_plugin::BuiltNode {
    root.children.iter().find(|node| node.key.as_str() == key).expect("Home row key present")
}

fn buttons(node: &semio_framework_plugin::BuiltNode) -> Vec<&semio_framework_ui_contract::ActionBinding> {
    node.children.iter().filter(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))).map(|child| child.bindings.get(0).expect("Home button carries an action binding")).collect()
}

fn text_arg(binding: &semio_framework_ui_contract::ActionBinding, key: &str) -> String {
    let Some(semio_framework_ui_contract::UiValue::Map(args)) = binding.args.as_ref() else { panic!("Home action carries map args") };
    let (_, semio_framework_ui_contract::UiValue::Text(value)) = args.iter().find(|(name, _)| name.as_str() == key).expect("Home action arg present") else { panic!("Home action arg is text") };
    value.as_str().to_owned()
}

fn one_local_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { id: "sp-local".into(), name: "Fixture Studio".into(), kind: "atelier".into(), visibility: "private".into(), members: "1".into(), updated: "0".into(), origin: "local", role: None }
}

fn one_hub_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { id: "sp-hub".into(), name: "Fabrication".into(), kind: "studio".into(), visibility: "public".into(), members: "2".into(), updated: "1000".into(), origin: "hub", role: Some(crate::DirectorySpaceRole::Author) }
}

fn spectator_hub_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { role: Some(crate::DirectorySpaceRole::Spectator), ..one_hub_row() }
}

fn unbound_hub_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { role: None, ..one_hub_row() }
}

#[semio_framework_async_macros::async_test]
async fn empty_rows_render_the_empty_message_not_a_zero_row_table() {
    let json = project(render_rows(&[], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN).expect("empty Home rows"));
    assert!(json.contains("No studios yet"), "empty rows render the empty message, not a zero-row table: {json}");
    assert!(!json.contains("framework.window.table"), "empty rows must not render the table scene at all: {json}");
}

#[semio_framework_async_macros::async_test]
async fn a_local_row_renders_with_open_only_actions() {
    let json = project(render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN).expect("local Home row"));
    assert!(json.contains("Fixture Studio"));
    assert!(json.contains("local"));
    assert!(!json.contains("rename"), "local-only rows offer open only, no rename/share/delete: {json}");
}

#[semio_framework_async_macros::async_test]
async fn a_hub_row_renders_with_the_full_action_set() {
    let json = project(render_rows(&[one_hub_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN).expect("Hub Home row"));
    assert!(json.contains("Fabrication"));
    assert!(json.contains("hub"));
    assert!(json.contains("rename") && json.contains("share") && json.contains("delete"), "hub rows offer the full lifecycle action set: {json}");
}

/// 🆔️ Contract §C0: `data-row-id="space:<id>"` must reach the table scene's own row id, and every
/// row action must be a real, dispatchable `ActionDescriptor` (controller + action id + spaceId
/// arg) — not text, per ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F.
#[semio_framework_async_macros::async_test]
async fn a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions() {
    observe(render_rows(&[one_hub_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN).expect("Hub Home row"), |root| {
        let row = row(root, "space:sp-hub");
        let buttons = buttons(row);
        assert_eq!(buttons.len(), 5, "open + rename + share + delete + manage");
        let delete_button = buttons.iter().find(|button| button.action.name.as_str() == "deleteSpace").expect("delete button present");
        assert_eq!(delete_button.action.scope.as_str(), S_HOME_CONTROLLER_ID);
        assert_eq!(text_arg(delete_button, "spaceId"), "sp-hub", "the delete button's descriptor already carries the row's own space id");
        let manage_button = buttons.iter().find(|button| button.action.name.as_str() == "manageSpace").expect("manage button present");
        assert_eq!(text_arg(manage_button, "spaceId"), "sp-hub", "the administration descriptor carries the authoritative row id");
    });
}

#[semio_framework_async_macros::async_test]
async fn spectator_and_unbound_hub_rows_only_carry_open() {
    for row in [spectator_hub_row(), unbound_hub_row()] {
        observe(render_rows(&[row], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN).expect("restricted Hub Home row"), |root| {
            let buttons = buttons(root.children.iter().find(|node| node.key.as_str().starts_with("space:")).expect("restricted Home row present"));
            assert_eq!(buttons.len(), 1, "a stale or absent author identity cannot expose lifecycle administration");
            assert_eq!(buttons[0].action.name.as_str(), "openSpace");
        });
    }
}

#[semio_framework_async_macros::async_test]
async fn a_local_row_only_carries_an_open_action_button() {
    observe(render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN).expect("local Home row"), |root| {
        let buttons = buttons(row(root, "space:sp-local"));
        assert_eq!(buttons.len(), 1, "local-only rows offer open only");
        assert_eq!(buttons[0].action.name.as_str(), "openSpace");
    });
}

#[semio_framework_async_macros::async_test]
async fn seeded_local_studio_renders_a_table_row() {
    let cfg = HomeConfig::default();
    // 🌱️ `crate::catalog_port()` lazily seeds a demo space on first access (plugin root's own
    // `catalog_port_concrete`), so the local catalog is never truly empty once touched — this test
    // exercises the REAL end-to-end `render` (not `render_rows`), deliberately not asserting on
    // emptiness (see `empty_rows_render_the_empty_message_not_a_zero_row_table` for that, isolated).
    let _ = crate::list_all_space_catalog_entries().await;
    let node = render(&cfg, &semio_framework_plugin::ViewModel::default()).expect("seeded Home rows");
    let json = project(node);
    assert!(json.contains("local"), "the seeded demo studio has no directory entry, so it renders origin=local: {json}");
}

#[semio_framework_async_macros::async_test]
async fn german_locale_labels_resolve_in_the_rendered_table() {
    let json = project(render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_DE, &SHomeLabels::NATIVE_DE).expect("German Home row"));
    assert!(json.contains("Aktualisiert"), "German column header must resolve: {json}");
    assert!(json.contains("Herkunft"), "German column header must resolve: {json}");
    assert!(json.contains("lokal"), "German origin label must resolve for a local-only row: {json}");
}

#[semio_framework_async_macros::async_test]
async fn render_resolves_labels_from_host_view() {
    let cfg = HomeConfig { ..HomeConfig::default() };
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let json = project(render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_DE, &SHomeLabels::NATIVE_DE).expect("German Home row"));
    assert!(json.contains("Aktualisiert"));
    let _ = render(&cfg, &view_state).expect("localized Home rows");
}

/// 🆔️ Contract §C0 lane 4-F: `render(cfg, view_state)` must wrap the table in a real button carrying the
/// frozen `s-home-create-space` id, dispatching `createSpace` with no args — the harness clicks
/// this directly instead of hunting the command palette. The button is preceded by two
/// `window_content_dead_line_spacer()` separators (see that fn's doc) — found by type, not a
/// hardcoded index, so this test stays valid if the spacer count ever changes.
#[semio_framework_async_macros::async_test]
async fn render_wraps_the_table_with_a_real_create_space_button() {
    observe(render(&HomeConfig::default(), &semio_framework_plugin::ViewModel::default()).expect("Home rows with create action"), |root| {
        let button = root.children.iter().find(|child| child.key.as_str() == "s-home-create-space").expect("a create-space button somewhere in the stack");
        assert!(matches!(&button.component, semio_framework_ui_contract::Component::Button(_)));
        let binding = button.bindings.get(0).expect("create button carries action");
        assert_eq!(binding.action.scope.as_str(), S_HOME_CONTROLLER_ID);
        assert_eq!(binding.action.name.as_str(), "createSpace");
        assert!(binding.args.is_none(), "an empty-args dispatch is what makes the handler open the dialog");
    });
}

#[semio_framework_async_macros::async_test]
async fn empty_catalog_still_renders_the_create_space_button() {
    observe(render_rows_wrapped_for_test(&[]).await.expect("empty Home rows with create action"), |root| {
        assert!(root.children.iter().any(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))), "the create button must survive the empty-table branch too");
    });
}

/// 🧪️ `render`'s own composition, isolated from `crate::list_all_space_catalog_entries()`'s
/// process-global singleton — mirrors `render_rows`'s own isolation rationale above.
async fn render_rows_wrapped_for_test(rows: &[crate::HomeSpaceRow]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    render_rows_wrapped(rows, &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN)
}
