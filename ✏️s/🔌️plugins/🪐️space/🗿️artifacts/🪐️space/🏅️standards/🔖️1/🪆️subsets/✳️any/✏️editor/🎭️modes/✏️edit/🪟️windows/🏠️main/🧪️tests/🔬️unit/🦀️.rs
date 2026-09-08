
use super::*;

fn project(node: semio_framework_plugin::BuiltNode) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("Space row tree projection")
}

fn observe<R>(node: semio_framework_plugin::BuiltNode, inspect: impl FnOnce(&semio_framework_plugin::BuiltNode) -> R) -> R {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| inspect(&node)));
    let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(node);
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 4096).expect("Space row fixture tree remains valid");
        if !step.progressed {
            std::thread::yield_now();
        }
    }
    match result {
        Ok(result) => result,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

fn buttons(node: &semio_framework_plugin::BuiltNode) -> Vec<&semio_framework_ui_contract::ActionBinding> {
    node.children.iter().filter(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))).map(|child| child.bindings.get(0).expect("Space row button carries an action binding")).collect()
}

fn text_arg(binding: &semio_framework_ui_contract::ActionBinding, key: &str) -> String {
    let Some(semio_framework_ui_contract::UiValue::Map(args)) = binding.args.as_ref() else { panic!("Space row action carries map args") };
    let (_, semio_framework_ui_contract::UiValue::Text(value)) = args.iter().find(|(name, _)| name.as_str() == key).expect("Space row action arg present") else { panic!("Space row action arg is text") };
    value.as_str().to_owned()
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_node_for_the_default_document() {
    let _ = project(render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default()).expect("default Space rows"));
}

#[semio_framework_async_macros::async_test]
async fn render_reflects_live_presence_for_a_row() {
    use crate::editor::space_index::config::SpaceIndexArtifactPresence;
    use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
    let document = SSpaceSnapshot::default();
    let config = SpaceIndexConfig {
        indexed_artifacts: vec![SpaceArtifactRow { id: "artifact-1".into(), name: "First".into(), dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() }, ..Default::default() }],
        presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1,user:2".into() }],
        ..Default::default()
    };
    let json = project(render(&document, &config).expect("Space rows with presence"));
    assert!(json.contains("user:1, user:2"), "presence must reach the table cell: {json}");
}

/// 🆔️ Contract §C0: `data-row-id="artifact:<id>"` must reach the table scene's own row id, and the
/// row's open/delete buttons must be real, dispatchable `ActionDescriptor`s carrying the row's own
/// id — per ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F.
#[semio_framework_async_macros::async_test]
async fn a_directory_row_stamps_the_artifact_row_id_and_carries_only_the_safe_open_button() {
    use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
    let config = SpaceIndexConfig {
        indexed_artifacts: vec![SpaceArtifactRow { id: "artifact-1".into(), name: "First".into(), dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() }, ..Default::default() }],
        ..Default::default()
    };
    observe(render_table(&config).expect("Space artifact rows"), |root| {
        let row = root.children.iter().find(|node| node.key.as_str() == "artifact:artifact-1").expect("Space artifact row id");
        let buttons = buttons(row);
        assert_eq!(buttons.len(), 1, "Directory-owned rows expose only the authority-safe open action");
        let open_button = buttons.iter().find(|button| button.action.name.as_str() == "openArtifact").expect("open button present");
        assert_eq!(text_arg(open_button, "id"), "artifact-1");
    });
}

/// 🆔️ Contract §C0 lane 4-F: `render(...)` must wrap the table in a real button carrying the
/// frozen `s-space-create-artifact` id, dispatching `createArtifact` with no args — the harness
/// clicks this directly instead of hunting the command palette.
#[semio_framework_async_macros::async_test]
async fn render_wraps_the_table_with_a_real_create_artifact_button() {
    use crate::editor::space_index::SPACE_INDEX_CONTROLLER_ID;
    observe(render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default()).expect("Space rows with create action"), |root| {
        let button = root.children.iter().find(|child| child.key.as_str() == "s-space-create-artifact").expect("a create-artifact button somewhere in the stack");
        assert!(matches!(&button.component, semio_framework_ui_contract::Component::Button(_)));
        let binding = button.bindings.get(0).expect("create artifact button carries action");
        assert_eq!(binding.action.scope.as_str(), SPACE_INDEX_CONTROLLER_ID);
        assert_eq!(binding.action.name.as_str(), "createArtifact");
        assert!(binding.args.is_none(), "an empty-args dispatch is what makes the handler open the dialog");
    });
}
