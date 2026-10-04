//! 🔣️ Json editor — `main` window: a real, directly editable tree of the whole `JsonValue`, built
//! from the framework `TreeWindowKit` (contract §2.6). Every node is keyed by its SIBLING ordinal
//! (`m=<member-index>` / `i=<item-index>`, the root by [`JSON_ROOT_NODE_ID`]); its window identity is the PATH
//! the SDK composes from its ancestors' keys, and `set-node` addresses a node by that same path, so
//! the editor's own `handle` applies `JsonMutation::SetScalar` there, replacing whichever subtree
//! previously lived at it.

use crate::schema::snapshot::{write_json_pretty, JsonMember, JsonValue};
use crate::JsonSnapshot;
use semio_framework_plugin::app::{EditableTreeNode, TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::WindowKindDefinition;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::json_i_json::create_json_i_json_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️PathEncoding
/// 🆔️ The document root's node id. It must be a real, non-empty key: a tree row built with an empty
/// id falls back to its positional `#0` key, which no `TreeWindowRequest` can name and which collides
/// with any sibling that does the same.
pub const JSON_ROOT_NODE_ID: &str = "$";

/// 🧭️ One node's key is its SIBLING segment — `m=<index>` for an object member, `i=<index>` for an
/// array element — NEVER the path from the document root. Window identity is already the container
/// PATH (its ancestors' keys, then its own, joined by `TREE_WINDOW_PATH_SEPARATOR`), and a path is a
/// view-context identifier capped at 256 code points: a key that repeated its own ancestry made that
/// path grow quadratically and put any document deeper than about seven levels out of the host's
/// reach entirely.
///
/// 🔢️ Object members use their source-order ordinal, so duplicate names, literal `#2` suffixes and
/// the tree separator remain exact user data in the visible label instead of leaking into UI ids.
pub fn member_segment(index: usize) -> String {
    format!("m={index}")
}

/// 🧭️ The `set-node` id of the node reached by `segments` (sibling segments, outermost first) — the
/// document root key, then each segment, joined by `TREE_WINDOW_PATH_SEPARATOR`.
///
/// ⚠️ NOT the `node_key` a `TreeWindowRequest` names: a window path is composed by the SDK from the
/// body's own section root outwards (`<TreeWindowKit::KIND_ID>-root`, then this id's segments), so it
/// carries one segment more than an action id does. Both share the 256-code-point view-context bound.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_path_id(segments: &[String]) -> String {
    let mut path = JSON_ROOT_NODE_ID.to_string();
    for segment in segments {
        path.push_str(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR);
        path.push_str(segment);
    }
    path
}

fn canonical_index(segment: &str, prefix: &str) -> Option<usize> {
    let value = segment.strip_prefix(prefix)?;
    let index = value.parse::<usize>().ok()?;
    (index.to_string() == value).then_some(index)
}

/// 🧭️ Resolves a row id through the exact source-order member/item ordinals it carries.
pub fn node_at_path_id<'a>(document: &'a JsonSnapshot, node_id: &str) -> Option<&'a JsonValue> {
    let mut value = &document.value;
    if node_id == JSON_ROOT_NODE_ID {
        return Some(value);
    }
    let mut segments = node_id.split(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR);
    (segments.next()? == JSON_ROOT_NODE_ID).then_some(())?;
    for segment in segments {
        value = match value {
            JsonValue::Object { members } => &members.get(canonical_index(segment, "m=")?)?.value,
            JsonValue::Array { items } => items.get(canonical_index(segment, "i=")?)?,
            _ => return None,
        };
    }
    Some(value)
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
//#endregion 🔖️PathEncoding

//#region 🔖️Render
/// ✏️ Real `JsonSnapshot -> BuiltNode`: a labeled tree mirroring the document's own shape exactly —
/// object members keep source order, array elements keep position, scalars show their literal
/// value inline.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &JsonSnapshot, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    TreeWindowKit::render_windowed(&TreeView { roots: vec![node_view(JSON_ROOT_NODE_ID.to_string(), None, &document.value)] }, windows)
}

/// 📝️ Adds the natural I-JSON source draft to the structured tree for editor hosts.
pub fn render_editor(
    document: &JsonSnapshot,
    locale: Locale,
    windows: &TreeWindows<'_>,
    controller_id: &str,
    revision: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let view = TreeView { roots: vec![node_view(JSON_ROOT_NODE_ID.to_string(), None, &document.value)] };
    let tree = TreeWindowKit::render_editable_nodes_windowed(&view, windows, controller_id, |path, _| {
        let source = write_json_pretty(node_at_path_id(document, path)?);
        UiText::try_from_str(&source)?;
        let mut arguments = UiMapBuilder::try_new()?;
        arguments.try_insert("nodeId".into(), UiValue::Text(UiText::try_from_str(path)?)).ok()?;
        arguments.try_insert("revision".into(), UiValue::Text(UiText::try_from_str(revision)?)).ok()?;
        Some(EditableTreeNode::new(source, "set-node", UiValue::Map(arguments.finish())))
    })?;
    semio_s_artifact_stdio_contract::editing::render_file_source_editor("stdio-i-json-source", write_json_pretty(&document.value), "json", "set-node", JSON_ROOT_NODE_ID, revision, publication_revision, locale, tree)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_view(key: String, key_label: Option<&str>, value: &JsonValue) -> TreeNodeView {
    let prefix = key_label.map(|label| format!("{label}: ")).unwrap_or_default();
    match value {
        JsonValue::Object { members } => {
            let children = members.iter().enumerate().map(|(index, member): (usize, &JsonMember)| node_view(member_segment(index), Some(&member.key), &member.value)).collect();
            TreeNodeView { id: key, label: format!("{prefix}{{{}}}", members.len()), children }
        }
        JsonValue::Array { items } => {
            let children = items.iter().enumerate().map(|(index, item)| node_view(format!("i={index}"), Some(&index.to_string()), item)).collect();
            TreeNodeView { id: key, label: format!("{prefix}[{}]", items.len()), children }
        }
        scalar => TreeNodeView { id: key, label: format!("{prefix}{}", scalar_label(scalar).unwrap_or_default()), children: Vec::new() },
    }
}

//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
