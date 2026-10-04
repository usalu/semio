//! 🧪️ Host node-graph catalogue projection of the canonical lower registry.
use super::*;
#[test]
fn contributed_registry_projects_node_graph_records() {
    let _serialized = lock_flow_extension_registry_for_test();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    install_flow_extension_manifest(fixture["pluginId"].as_str().unwrap(), &fixture["manifest"].to_string()).unwrap();
    assert!(!crate::catalogue::flow_operator_catalogue_records().is_empty());
    uninstall_flow_extension("owned").unwrap();
}
