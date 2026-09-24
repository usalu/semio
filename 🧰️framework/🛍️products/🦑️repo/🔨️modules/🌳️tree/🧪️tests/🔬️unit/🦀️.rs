use super::*;

fn leaf(kind: TreeNodeKind, id: &str, label: &str) -> TreeNode {
    tree_node(kind, id, label, "")
}

#[test]
fn sorts_folders_before_files_then_by_label() {
    let mut root = tree_node(TreeNodeKind::Category, "", "root", "");
    root.children = Some(vec![
        leaf(TreeNodeKind::File, "", "z.ts"),
        leaf(TreeNodeKind::File, "", "a.ts"),
        leaf(TreeNodeKind::Folder, "", "src"),
    ]);
    sort_tree_children(&mut root);
    let labels: Vec<String> = children_of(&root).iter().map(|node| node.label.clone()).collect();
    assert_eq!(labels, vec!["src", "a.ts", "z.ts"]);
}

#[test]
fn digest_is_stable_and_content_addressed() {
    let mut left = tree_node(TreeNodeKind::Category, "", "root", "");
    left.children = Some(vec![leaf(TreeNodeKind::File, "a", "a.ts")]);
    let mut right = left.clone();
    assert_eq!(tree_content_digest(&left), tree_content_digest(&right));
    right.children = Some(vec![leaf(TreeNodeKind::File, "b", "b.ts")]);
    assert_ne!(tree_content_digest(&left), tree_content_digest(&right));
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

#[test]
fn mermaid_treemap_indents_by_four_spaces_per_level() {
    let treemap = MermaidTreemap {
        title: "Lines of Code".to_string(),
        nodes: vec![MermaidNode {
            label: "🛠️repo".to_string(),
            value: None,
            children: vec![MermaidNode { label: "\"cli\"".to_string(), value: Some(12), children: Vec::new() }],
        }],
    };
    assert_eq!(render_mermaid_treemap(&treemap), "treemap-beta\n\"Lines of Code\"\n    \"🛠️repo\"\n        \"'cli'\": 12\n");
}

#[test]
fn cache_envelope_round_trips_through_the_shared_gzip_codec() {
    let mut tree = leaf(TreeNodeKind::Category, "root", "Root");
    tree.children = Some(vec![leaf(TreeNodeKind::Folder, "f", "folder"), leaf(TreeNodeKind::File, "a", "a.ts")]);
    let payload = encode_tree_cache(&tree);
    assert_eq!(&payload[..3], &[0x1f, 0x8b, 0x08]);
    assert_eq!(decode_tree_cache(&payload).unwrap(), tree);
    assert!(decode_tree_cache(b"not gzip at all!!!!").is_err());
}
