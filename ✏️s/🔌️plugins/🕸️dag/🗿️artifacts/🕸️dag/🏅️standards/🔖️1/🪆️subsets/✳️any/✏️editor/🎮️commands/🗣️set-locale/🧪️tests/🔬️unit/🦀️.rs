
use super::*;
use crate::editor::dag::testkit;
use crate::editor::dag::{DAG_PLAY_BODY_DOCUMENT, DagCommand};
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn dag_play_labels_resolve_native_english_and_german() {
    let mut app = testkit::new_app().await;
    let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let json = serde_json::to_string(&node.root).unwrap();
    assert!(json.contains("Nodes"));
    assert!(json.contains("Edges"));

    app.dispatch_typed(DagCommand::SetLocale(SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("set locale");
    let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let json = serde_json::to_string(&node.root).unwrap();
    assert!(json.contains("Knoten"));
    assert!(json.contains("Kanten"));
}
