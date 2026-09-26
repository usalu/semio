
use super::*;

fn project(node: semio_framework_plugin::BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("Home viewer tree projection")
}

fn host_view() -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        session_identity: Some(semio_framework_plugin::ViewSessionIdentity { user_id: "u1".into(), display_name: "Ada".into() }),
        ..Default::default()
    }
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
    crate::HomeSpaceRow { id: "sp-1".into(), name: "Fabrication".into(), kind: semio_framework_artifact_space_space::SpaceKind::Studio, visibility: semio_framework_artifact_space_space::SpaceVisibility::Public, members: "2".into(), updated_ms: Some(1_790_370_316_130), origin: "hub", data_class: "persistedShared", role: None }
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, S_HOME_VIEW_BODY);
}

#[semio_framework_async_macros::async_test]
async fn empty_rows_render_the_empty_message_not_a_zero_row_table() {
    let json = project(render_rows(&[], &HomeTableLabels::NATIVE_EN, &TreeWindows::unhosted()).expect("empty Home viewer rows"));
    assert!(json.contains("No studios yet."));
    assert!(!json.contains("framework.window.table"), "empty rows must not render the table scene at all: {json}");
}

#[semio_framework_async_macros::async_test]
async fn a_row_renders_without_the_actions_column() {
    let json = project(render_rows(&[one_hub_row().await], &HomeTableLabels::NATIVE_EN, &TreeWindows::unhosted()).expect("Home viewer row"));
    assert!(json.contains("Fabrication"));
    assert!(json.contains("hub"));
    assert!(json.contains("Origin"), "six columns render, the last being Origin: {json}");
    assert!(!json.contains("Actions"), "the viewer never renders an Actions column: {json}");
}

/// 🆔️ Contract §C0: even the read-only viewer's rows must carry `data-row-id="space:<id>"` — a
/// viewer just never attaches row-scoped action buttons to it.
#[semio_framework_async_macros::async_test]
async fn a_row_stamps_the_space_row_id() {
    observe(render_rows(&[one_hub_row().await], &HomeTableLabels::NATIVE_EN, &TreeWindows::unhosted()).expect("Home viewer row"), |root| {
        let row = root.children.iter().find(|node| node.key.as_str() == "space:sp-1").expect("Home viewer row id");
        let semio_framework_ui_contract::Component::TableRow(props) = &row.component else { panic!("a viewer row is one TableRow record") };
        assert!(props.row_actions.is_empty() && row.bindings.is_empty(), "the viewer never carries a row action");
    });
}

#[semio_framework_async_macros::async_test]
async fn german_locale_labels_resolve() {
    let json = project(render_rows(&[one_hub_row().await], &HomeTableLabels::NATIVE_DE, &TreeWindows::unhosted()).expect("German Home viewer row"));
    assert!(json.contains("Aktualisiert"));
    assert!(json.contains("Herkunft"));
}

/// 🌐️ Every cell of a row speaks the viewer's language: kind, visibility and origin are words, never the
/// wire ids, and "Updated" is a UTC minute, never raw epoch milliseconds.
#[semio_framework_async_macros::async_test]
async fn row_cells_are_localized_words_and_a_utc_minute() {
    let row = one_hub_row().await;
    assert_eq!(row.cells(&HomeTableLabels::NATIVE_EN), ["Fabrication", "Studio", "public", "2", "2026-09-25 21:05 UTC", "hub"].map(String::from));
    assert_eq!(row.cells(&HomeTableLabels::NATIVE_DE), ["Fabrication", "Studio", "öffentlich", "2", "25.09.2026, 21:05 UTC", "Hub"].map(String::from));
    let draft = crate::HomeSpaceRow { kind: semio_framework_artifact_space_space::SpaceKind::Archive, visibility: semio_framework_artifact_space_space::SpaceVisibility::Private, updated_ms: None, origin: "local", ..row };
    assert_eq!(draft.cells(&HomeTableLabels::NATIVE_DE)[1..], ["Archiv", "privat", "2", "nie gespeichert", "lokal"].map(String::from));
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
    let json = project(render(&directory, &host_view()).expect("folded Home viewer row"));
    assert!(json.contains("Fabrication"), "the folded space renders: {json}");
    assert!(json.contains("hub"), "hub-folded spaces render origin=hub: {json}");
}
