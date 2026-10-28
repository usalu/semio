/// 🌲️ Borrowed native traversal admits every model family, nested list and UTF8 owner without cloning.
pub(super) fn native_tree_extent(tree: &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, remaining: &mut usize, depth: usize) -> Option<()> {
    use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text};
    if depth > 64 { return None; }
    *remaining = remaining.checked_sub(1)?;
    let text_bytes = |text: Text<'_>| match text { Text::Contiguous(text) => text.len(), Text::Native(text) => text.text_bytes() };
    let children = match tree.canonical_tree_node().ok()? {
        Node::String(text) => { *remaining = remaining.checked_sub(text.len())?; return Some(()); }
        Node::Text(text) => { *remaining = remaining.checked_sub(text_bytes(text))?; return Some(()); }
        Node::Array(count) => count,
        Node::Object(count) => {
            if count > *remaining { return None; }
            for index in 0..count { *remaining = remaining.checked_sub(text_bytes(tree.canonical_tree_key(index).ok()?))?; }
            count
        }
        _ => return Some(()),
    };
    if children > *remaining { return None; }
    for index in 0..children { native_tree_extent(tree.canonical_tree_child(index).ok()?, remaining, depth + 1)?; }
    Some(())
}
