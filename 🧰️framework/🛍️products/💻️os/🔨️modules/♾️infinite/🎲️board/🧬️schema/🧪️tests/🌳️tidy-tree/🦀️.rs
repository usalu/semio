//! 🧪️ schema tests (🌳️tidy-tree): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::{buchheim_positions, LayoutControl};
use std::collections::HashMap;

#[test]
fn buchheim_tree_two_nodes() {
    let roots = vec!["a".into()];
    let directed = vec![("a".into(), "b".into())];
    let mut depth = std::collections::HashMap::new();
    depth.insert("a".into(), 0);
    depth.insert("b".into(), 1);
    let mut progress=|_|true;let mut control=LayoutControl::new(1_000_000,&mut progress);let pos = buchheim_positions(&roots, &directed, &depth,&mut control).unwrap();
    assert!(pos.contains_key("a"));
    assert!(pos.contains_key("b"));
}
