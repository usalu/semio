
use super::*;

/// 🗂️ A configured `fileNodeKinds[kindId].icon` must win over the kind-name fallback table —
/// previously `vfs_glyph_icon` only checked `.is_some()` and always returned `"folder"` for any
/// kind with a custom icon configured, discarding the actual icon id.
#[test]
fn configured_kind_icon_is_used_verbatim_not_collapsed_to_folder() {
    let schema: VfsSchema = serde_json::from_str(r#"{"fileNodeKinds":{"asset":{"icon":"box"}}}"#).unwrap();
    let row = json!({ "fileNodeKindId": "asset" });
    assert_eq!(vfs_glyph_icon(&schema, &row), "box");
}

#[test]
fn folder_and_instance_kinds_fall_back_to_their_built_in_glyphs() {
    let schema: VfsSchema = serde_json::from_str("{}").unwrap();
    assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "folder" })), "folder");
    assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "instance" })), "box");
    assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "other" })), "file-text");
}

#[test]
fn missing_file_node_kind_id_defaults_to_the_file_kind() {
    let schema: VfsSchema = serde_json::from_str("{}").unwrap();
    assert_eq!(vfs_glyph_icon(&schema, &json!({})), "file-text");
}
