use super::*;
use crate::editor_domain::editor_laws::context::{app_with_registry, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await.contains("node-graph"));
}

/// 🛍️ The scene names its operators by KIND ID (inside `fixtureJson`) and carries only the document's
/// own neuron kinds as operator records — never the registered catalogue (~100 KB, three times the
/// fixed 32 KiB per-surface admission; ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). Those records
/// are how the canvas instantiates the graph instead of `FlowHostSnapshot::default()`'s placeholder slider.
#[semio_framework_async_macros::async_test]
async fn main_graph_scene_exports_flow_backed_node_graph_fields() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<NodeGraphScene>(&json).expect("node-graph scene decodes off the rendered surface");
    assert!(scene.host_snapshot_json.as_deref().is_some_and(|host_snapshot| host_snapshot.contains("flow.host_snapshot")));
    let capabilities = scene.capabilities_json.clone().unwrap_or_default();
    assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
    assert!(!scene.nodes.is_empty(), "the open document's nodes must reach the scene");
    assert!(!scene.nodes.iter().any(|node| node.id == "slider"), "the engine default fixture must not replace the open document");
    assert!(
        scene.operators.len() < 32,
        "operators must be document-derived, not the registered catalogue, carries {}",
        scene.operators.len()
    );
    assert!(
        scene.operators.iter().any(|operator| operator.id.contains("brep.") || operator.id.contains("math.")),
        "the open document's neuron kinds must reach the scene as operator records: {:?}",
        scene.operators.iter().map(|operator| operator.id.as_str()).collect::<Vec<_>>()
    );
}

/// 🛍️ …and the catalogue the scene no longer carries is exactly what this app publishes on the
/// reserved `framework.section.catalogue` retained surface, once per app instance.
#[semio_framework_async_macros::async_test]
async fn the_app_catalogue_section_carries_the_registered_operators() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let catalogue: serde_json::Value = serde_json::from_str(&semio_framework_os_flow::flow_app_catalogue_json()).expect("app catalogue json");
    let operators = catalogue.get("operators").and_then(|value| value.as_array()).expect("operators array");
    assert!(operators.iter().any(|operator| operator.get("id").and_then(|value| value.as_str()).is_some_and(|id| id.contains("math.add") || id.contains("brep."))));
    assert!(catalogue.get("sections").and_then(|value| value.as_array()).is_some_and(|sections| !sections.is_empty()));
}

//#region 🔖️GraphOutline
/// 🕸️ The law both implementations of this window answer to: one committed flow fixture in, the exact
/// node/port/wire rows and the exact interaction bindings out. Language-agnostic on purpose — the row
/// vocabulary is the `🟦️.ts` twin's too (`Generation3dFlowOutline`).
const GRAPH_OUTLINE_LAW: &str = include_str!("../../../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧫️fixtures/🔬️unit/🔣️.json");

/// 🚦 A node row's description is the localized `NodeEvalStatus` the flow session reported for that
/// widget — English and German, no default language.
///
/// 🗑️ `stale` is deliberately absent: `build_flow_status_json` stopped emitting it when the census
/// became monotone, so the word had no producer and the vocabulary dropped it rather than keep a
/// status a user could never be shown (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).


/// 🧱️ The Flow window body is the node-graph canvas only — the artifact tree lives on the Artifact panel.
#[semio_framework_async_macros::async_test]
async fn main_body_is_canvas_only_without_an_embedded_artifact_tree() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let projection: serde_json::Value = serde_json::from_str(&render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await).expect("body projection json");
    fn find_tree(node: &serde_json::Value) -> Option<&serde_json::Value> {
        if node["component"]["type"].as_str() == Some("tree") {
            return Some(node);
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(find_tree))
    }
    assert!(find_tree(&projection).is_none(), "the flow window must not embed an artifact tree");
    assert!(projection.to_string().contains("node-graph"), "the flow window body must carry the node-graph surface");
}
//#endregion 🔖️GraphOutline

