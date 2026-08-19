//! 🔣️ Json editor — `main` window: a real, directly editable tree of the whole `JsonValue`, built
//! from the framework `TreeWindowKit` (contract §2.6). Every node's id encodes its `JsonPath` from
//! root (`k=<key>`/`i=<index>` segments joined by `/`, root itself is the empty string) so
//! `set-node` can address ANY node, not just leaves — the editor's own `handle` always applies
//! `JsonMutation::SetScalar` at that path, replacing whichever subtree previously lived there.

use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::json::JsonSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::json_any::create_json_editor`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️PathEncoding
/// 🧭️ `k=<key>` for an object member, `i=<index>` for an array element, joined by `/` — the
/// window's own node-id encoding of a `JsonPath`, independent of (and simpler than) the artifact's
/// own `JsonPathSegment` wire shape. Root is the empty string.
pub async fn encode_path_id(segments: &[String]) -> String {
    segments.join("/")
}

async fn scalar_label(value: &JsonValue) -> Option<String> {
    match value {
        JsonValue::Null => Some("null".to_string()),
        JsonValue::Bool { value } => Some(value.to_string()),
        JsonValue::Number { lexeme } => Some(lexeme.clone()),
        JsonValue::String { value } => Some(format!("{value:?}")),
        JsonValue::Array { .. } | JsonValue::Object { .. } => None,
    }
}
//#endregion 🔖️PathEncoding

//#region 🔖️Render
/// ✏️ Real `JsonSnapshot -> UiNode`: a labeled tree mirroring the document's own shape exactly —
/// object members keep source order, array elements keep position, scalars show their literal
/// value inline.
pub async fn render(document: &JsonSnapshot) -> UiNode {
    TreeWindowKit::render(&TreeView { roots: vec![node_view(Vec::new(), None, &document.value)] })
}

async fn node_view(path: Vec<String>, key_label: Option<&str>, value: &JsonValue) -> TreeNodeView {
    let id = encode_path_id(&path);
    let prefix = key_label.map(|key| format!("{key}: ")).unwrap_or_default();
    match value {
        JsonValue::Object { members } => {
            let children = members
                .iter()
                .map(|member: &JsonMember| {
                    let mut child_path = path.clone();
                    child_path.push(format!("k={}", member.key));
                    node_view(child_path, Some(&member.key), &member.value)
                })
                .collect();
            TreeNodeView { id, label: format!("{prefix}{{{}}}", members.len()), children }
        }
        JsonValue::Array { items } => {
            let children = items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let mut child_path = path.clone();
                    child_path.push(format!("i={index}"));
                    node_view(child_path, Some(&index.to_string()), item)
                })
                .collect();
            TreeNodeView { id, label: format!("{prefix}[{}]", items.len()), children }
        }
        scalar => TreeNodeView { id, label: format!("{prefix}{}", scalar_label(scalar).unwrap_or_default()), children: Vec::new() },
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    async fn render_walks_object_and_array_members() {
        let document = JsonSnapshot { schema: "stdio.json".into(), value: JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Array { items: vec![JsonValue::Bool { value: true }] } }] } };
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        assert_eq!(root.id, "");
        let a = &root.items.as_ref().unwrap()[0];
        assert_eq!(a.id, "k=a");
        let item0 = &a.items.as_ref().unwrap()[0];
        assert_eq!(item0.id, "k=a/i=0");
    }
}
//#endregion 🧪️Tests
