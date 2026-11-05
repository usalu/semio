use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use crate::editor::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_semio_drawing_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, SEMIO_DRAWING_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SemioDrawingEditor as ArtifactEditor>::DIALECT, SEMIO_DRAWING_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<SemioDrawingEditor, crate::viewer::SemioDrawingViewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio/SemioDrawingEditor", SemioDrawingEditor, || semio_plugin::App { definition: create_semio_drawing_editor(), examples: Vec::new() }, "../..");

fn render(node: &crate::schema::snapshot::DrawNode) -> String {
    match node {
        crate::schema::snapshot::DrawNode::Group { children, .. } => format!("[{}]", children.iter().map(render).collect::<Vec<_>>().join(",")),
        crate::schema::snapshot::DrawNode::Path { .. } => "p".into(),
        _ => "t".into(),
    }
}

#[semio_framework_async_macros::async_test]
async fn scene_graph_gestures_resolve_to_node_mutations() {
    use crate::schema::snapshot::{DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot, SemioTransform};
    use semio_framework_value::ToValue;
    use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
    use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
    let text = || DrawNode::Text { value: "t".into(), at: SemioPoint2 { x: 1.0, y: 2.0 }, style: None };
    let root = DrawNode::Group { transform: SemioTransform::identity(), children: vec![text(), DrawNode::Path { segments: vec![PathSegment::Close], style: None }, DrawNode::Group { transform: SemioTransform::identity(), children: vec![text()] }] };
    let snapshot = SemioDrawingSnapshot { layers: vec![DrawLayer { id: "l".into(), name: "L".into(), visible: true, root }], ..Default::default() };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🌳️node-edits/🔣️.json")).expect("neutral node-edit fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let path = case["path"].as_str().expect("path").to_string();
        let event = match case["op"].as_str().expect("op") {
            "insert" => SnapshotEditEvent::InsertValue { path: path.clone(), value: text().to_value() },
            "remove" => SnapshotEditEvent::RemoveValue { path: path.clone() },
            "set-at" => SnapshotEditEvent::SetValue { path: path.clone(), value: SemioPoint2 { x: 5.0, y: 6.0 }.to_value() },
            "set-segments" => SnapshotEditEvent::SetValue { path: path.clone(), value: vec![PathSegment::Close].to_value() },
            other => panic!("unknown op {other}"),
        };
        let resolved = crate::editor::node_edits::special(&event, &snapshot);
        match (case["expect"].as_str(), case["refused"].as_str()) {
            (Some("none"), _) => assert!(matches!(resolved, Ok(None)), "{path}"),
            (Some(kind), _) => {
                let mutations = resolved.unwrap_or_else(|fault| panic!("{path}: {fault:?}")).expect("resolved");
                assert_eq!(match &mutations[0] { crate::schema::mutations::SemioDrawingMutation::CreateNode(_) => "create-node", crate::schema::mutations::SemioDrawingMutation::DeleteNode(_) => "delete-node", crate::schema::mutations::SemioDrawingMutation::MoveNode(_) => "move-node", crate::schema::mutations::SemioDrawingMutation::ReplacePath(_) => "replace-path", _ => "other" }, kind, "{path}");
                let mut next = snapshot.clone();
                for mutation in &mutations {
                    let outcome = <crate::schema::mutations::SemioDrawingMutation as protocol::ColdMutation<SemioDrawingSnapshot>>::diff(mutation, &next);
                    next = protocol::apply_diff_cold(outcome.diff(), &next).expect("the node mutation applies");
                }
                assert_eq!(Some(render(&next.layers[0].root).as_str()), case["treeAfter"].as_str(), "{path}: the tree Python list operations compute");
            }
            (None, Some(code)) => assert_eq!(resolved.err().unwrap_or_else(|| panic!("{path} must be refused")).code.0, code, "{path}"),
            _ => panic!("fixture row needs expect or refused"),
        }
    }
}
