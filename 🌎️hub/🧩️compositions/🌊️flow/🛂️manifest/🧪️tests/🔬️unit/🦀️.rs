//! 🧪️ Native manifest identities owned by their concrete catalog.
use super::graph_manifest::*;

#[test]
fn flow_dag_manifest_resolves_from_the_permission_prefixed_source() {
    let m = manifest_by_id("flow-dag").expect("flow-dag");
    assert_eq!(m.id, "flow-dag");
    assert!(m.node_kind("computation").is_some());
    assert_eq!(flow_dag::FlowDagNodeKind::parse("appInstance"), Ok(flow_dag::FlowDagNodeKind::AppInstance));
}
