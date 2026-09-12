
use super::*;

fn project(node: semio_framework_plugin::BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("Space viewer tree projection")
}

fn observe<R>(node: semio_framework_plugin::BuiltNode, inspect: impl FnOnce(&semio_framework_plugin::BuiltNode) -> R) -> R {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| inspect(&node)));
    let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(node);
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 4096).expect("Space viewer fixture tree remains valid");
        if !step.progressed {
            std::thread::yield_now();
        }
    }
    match result {
        Ok(result) => result,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_node_for_the_default_document() {
    let _ = project(render(&SSpaceSnapshot::default()).expect("default Space viewer rows"));
}

/// 🆔️ Contract §C0: the read-only viewer's rows must still carry `data-row-id="artifact:<id>"` —
/// it just never attaches row action buttons to it.
#[semio_framework_async_macros::async_test]
async fn a_row_stamps_the_artifact_row_id_with_no_actions_cell() {
    use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
    let mut document = SSpaceSnapshot::default();
    document.artifacts.push(SpaceArtifactRow { id: "artifact-1".into(), name: "First".into(), dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() }, ..Default::default() });
    observe(render(&document).expect("Space viewer rows"), |root| {
        let row = root.children.iter().find(|node| node.key.as_str() == "artifact:artifact-1").expect("Space viewer row id");
        assert!(!row.children.iter().any(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))), "the viewer never carries a row action button");
    });
}
