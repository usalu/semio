//! 🧪️ Host node-graph catalogue projection of the canonical lower registry.
use super::*;
#[test]
fn contributed_registry_projects_node_graph_records() {
    let _serialized = lock_flow_extension_registry_for_test();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    install_flow_extension_manifest(fixture["pluginId"].as_str().unwrap(), &fixture["manifest"].to_string()).unwrap();
    assert!(!crate::catalogue::flow_operator_catalogue_records().is_empty());
    uninstall_flow_extension("owned").unwrap();
}

/// ⚖️ LAW: a registry generation that moves AFTER the operator catalogue was first read — what a lazily activated extension does — retires the displaced catalogue without taking the actor down. The displaced catalogue of 600 contributed operators needs far more than the former fixed 4096 one-item steps (`superseded flow catalogue did not terminate`), so the law fails on that bound and passes only when the retirement is spread over the following catalogue reads and drains completely.
#[test]
fn a_catalogue_displaced_after_it_was_read_drains_without_panicking() {
    let _serialized = lock_flow_extension_registry_for_test();
    drain_flow_extension_registry_retirements();
    let operators: Vec<serde_json::Value> = (0..600).map(|index| serde_json::json!({ "id": format!("cataloguelaw.op{index}"), "extension": "cataloguelaw", "name": format!("Op {index}"), "abbreviation": "O", "icon": "circle", "summary": "A law operator.", "inputs": [], "outputs": [] })).collect();
    let manifest = serde_json::json!({ "schema": "flow.extension", "id": "cataloguelaw", "name": "Catalogue Law", "version": "1", "activationEvents": ["onStartup"], "contributes": { "schemas": [], "operators": operators, "widgets": [], "commands": [], "settings": [] } }).to_string();
    let read = || crate::catalogue::with_flow_neuron_kind_info_map(|infos| infos.len());
    let builtin = read();
    install_flow_extension_manifest("flow-extension-cataloguelaw", &manifest).expect("the extension registers");
    assert_eq!(read(), builtin + 600, "the moved generation is projected into a fresh catalogue");
    uninstall_flow_extension("cataloguelaw").expect("the extension withdraws");
    assert_eq!(read(), builtin, "the generation moved again and the 600-operator catalogue was displaced");
    for _ in 0..64 {
        read();
    }
    println!("[STATS] displaced catalogues pending after 64 reads: {}", crate::catalogue::superseded_flow_catalogues_pending());
    assert_eq!(crate::catalogue::superseded_flow_catalogues_pending(), 0, "the displaced catalogues drained across the following reads");
    drain_flow_extension_registry_retirements();
}
