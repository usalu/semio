//! 🧪️ Native manifest identities owned by their concrete catalog.
use crate::graph_manifest::*;

#[test]
fn nakagin_manifest_loads() {
    let m = nakagin::nakagin_manifest();
    assert_eq!(m.id, "nakagin");
    assert!(m.node_kind("Piece").is_some());
    assert!(m.edge_kind("Connection").is_some());
}
#[test]
fn manifest_by_id_resolves() {
    let m = manifest_by_id("nakagin").expect("nakagin");
    assert!(m.node_kind("Balcony").is_some());
}
