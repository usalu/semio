//! 🔣️ Json viewer — `main` window: a real, READ-ONLY tree of the whole `JsonValue`, built from
//! the framework `TreeWindowKit` (contract §2.6). Same path-encoded node ids as the sibling
//! mutation-capable window (documentation only, not a compile dependency — this file never
//! imports the editor module).

use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::json::JsonSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::json_i_json::create_json_i_json_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `JsonSnapshot -> UiNode` read: same shape as the editor's own render, no mutation.
pub fn render(document: &JsonSnapshot) -> UiNode {
    TreeWindowKit::render(&TreeView { roots: vec![node_view(Vec::new(), None, &document.value)] })
}

fn scalar_label(value: &JsonValue) -> Option<String> {
    match value {
        JsonValue::Null => Some("null".to_string()),
        JsonValue::Bool { value } => Some(value.to_string()),
        JsonValue::Number { lexeme } => Some(lexeme.clone()),
        JsonValue::String { value } => Some(format!("{value:?}")),
        JsonValue::Array { .. } | JsonValue::Object { .. } => None,
    }
}

fn node_view(path: Vec<String>, key_label: Option<&str>, value: &JsonValue) -> TreeNodeView {
    let id = path.join("/");
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
    fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    fn render_walks_object_and_array_members() {
        let document = JsonSnapshot { schema: "stdio.json".into(), value: JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Bool { value: true } }] } };
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        assert_eq!(root.id, "");
    }
}
//#endregion 🧪️Tests
