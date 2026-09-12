
use super::*;

fn project(node: semio_framework_plugin::BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("Home viewer tree projection")
}

fn observe<R>(node: semio_framework_plugin::BuiltNode, inspect: impl FnOnce(&semio_framework_plugin::BuiltNode) -> R) -> R {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| inspect(&node)));
    let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(node);
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 4096).expect("Home viewer fixture tree remains valid");
        if !step.progressed {
            std::thread::yield_now();
        }
    }
    match result {
        Ok(result) => result,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

async fn one_hub_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { id: "sp-1".into(), name: "Fabrication".into(), kind: "studio".into(), visibility: "public".into(), members: "2".into(), updated: "1000".into(), origin: "hub", role: None }
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, S_HOME_VIEW_BODY);
}

#[semio_framework_async_macros::async_test]
async fn empty_rows_render_the_empty_message_not_a_zero_row_table() {
    let json = project(render_rows(&[], &HomeTableLabels::NATIVE_EN).expect("empty Home viewer rows"));
    assert!(json.contains("No studios yet."));
    assert!(!json.contains("framework.window.table"), "empty rows must not render the table scene at all: {json}");
}

#[semio_framework_async_macros::async_test]
async fn a_row_renders_without_the_actions_column() {
    let json = project(render_rows(&[one_hub_row().await], &HomeTableLabels::NATIVE_EN).expect("Home viewer row"));
    assert!(json.contains("Fabrication"));
    assert!(json.contains("hub"));
    assert!(json.contains("Origin"), "six columns render, the last being Origin: {json}");
    assert!(!json.contains("Actions"), "the viewer never renders an Actions column: {json}");
}

/// 🆔️ Contract §C0: even the read-only viewer's rows must carry `data-row-id="space:<id>"` — a
/// viewer just never attaches row-scoped action buttons to it.
#[semio_framework_async_macros::async_test]
async fn a_row_stamps_the_space_row_id() {
    observe(render_rows(&[one_hub_row().await], &HomeTableLabels::NATIVE_EN).expect("Home viewer row"), |root| {
        let row = root.children.iter().find(|node| node.key.as_str() == "space:sp-1").expect("Home viewer row id");
        assert!(!row.children.iter().any(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))), "the viewer never carries a row action button");
    });
}

#[semio_framework_async_macros::async_test]
async fn german_locale_labels_resolve() {
    let json = project(render_rows(&[one_hub_row().await], &HomeTableLabels::NATIVE_DE).expect("German Home viewer row"));
    assert!(json.contains("Aktualisiert"));
    assert!(json.contains("Herkunft"));
}

#[semio_framework_async_macros::async_test]
async fn render_with_a_folded_space_renders_a_table_row() {
    let event = store::os_directory::DirectoryEvent {
        seq: 1,
        id: "evt-1".into(),
        hlc: store::os_directory::Hlc { physical_ms: 0, logical: 0 },
        actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "u".into() },
        space_id: Some("sp-1".into()),
        user_id: None,
        body: store::os_directory::DirectoryEventBody::SpaceCreated {
            space_id: "sp-1".into(),
            name: "Fabrication".into(),
            space_kind: store::os_directory::DirectorySpaceKind::Studio,
            visibility: store::os_directory::DirectorySpaceVisibility::Public,
            owner_user_id: "u1".into(),
        },
        recorded_at_ms: 1000,
    };
    let directory = store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event);
    let json = project(render(&directory, &semio_framework_plugin::ViewModel::default(), "u1").expect("folded Home viewer row"));
    assert!(json.contains("Fabrication"), "the folded space renders: {json}");
    assert!(json.contains("hub"), "hub-folded spaces render origin=hub: {json}");
}
