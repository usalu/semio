//! 🔣️ Json viewer — `main` window: a real, READ-ONLY tree of the whole `JsonValue`, built from
//! the framework `TreeWindowKit` (contract §2.6). Same path-encoded node ids as the sibling
//! mutation-capable window (documentation only, not a compile dependency — this file never
//! imports the editor module).

use crate::schema::snapshot::{JsonMember, JsonValue};
use crate::JsonSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::json_any::create_json_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `JsonSnapshot -> BuiltNode` read: same shape as the editor's own render, no mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &JsonSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    TreeWindowKit::render(&TreeView { roots: vec![node_view(&[], None, &document.value)] })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn scalar_label(value: &JsonValue) -> Option<String> {
    match value {
        JsonValue::Null => Some("null".to_string()),
        JsonValue::Bool { value } => Some(value.to_string()),
        JsonValue::Number { lexeme } => Some(lexeme.clone()),
        JsonValue::String { value } => Some(format!("{value:?}")),
        JsonValue::Array { .. } | JsonValue::Object { .. } => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_view(path: &[String], key_label: Option<&str>, value: &JsonValue) -> TreeNodeView {
    let id = path.join("/");
    let prefix = key_label.map(|key| format!("{key}: ")).unwrap_or_default();
    match value {
        JsonValue::Object { members } => {
            let children = members
                .iter()
                .map(|member: &JsonMember| {
                    let mut child_path = path.to_vec();
                    child_path.push(format!("k={}", member.key));
                    node_view(&child_path, Some(&member.key), &member.value)
                })
                .collect();
            TreeNodeView { id, label: format!("{prefix}{{{}}}", members.len()), children }
        }
        JsonValue::Array { items } => {
            let children = items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let mut child_path = path.to_vec();
                    child_path.push(format!("i={index}"));
                    node_view(&child_path, Some(&index.to_string()), item)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
