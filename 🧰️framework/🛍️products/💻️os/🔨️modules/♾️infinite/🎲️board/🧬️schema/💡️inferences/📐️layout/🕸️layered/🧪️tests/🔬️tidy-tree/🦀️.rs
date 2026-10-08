
use super::buchheim_positions;

#[test]
fn buchheim_tree_two_nodes() {
    let roots = vec!["a".into()];
    let directed = vec![("a".into(), "b".into())];
    let mut depth = std::collections::HashMap::new();
    depth.insert("a".into(), 0);
    depth.insert("b".into(), 1);
    let mut progress=|_|true;let mut control=super::LayoutControl::new(1_000_000,&mut progress);let pos = buchheim_positions(&roots, &directed, &depth,&mut control).unwrap();
    assert!(pos.contains_key("a"));
    assert!(pos.contains_key("b"));
}
