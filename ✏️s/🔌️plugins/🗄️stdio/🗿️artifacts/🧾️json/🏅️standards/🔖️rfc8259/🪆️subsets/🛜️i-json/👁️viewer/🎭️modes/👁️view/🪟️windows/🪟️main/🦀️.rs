//! 🔣️ Json viewer — `main` window: a real, READ-ONLY tree of the whole `JsonValue`, built from
//! the framework `TreeWindowKit` (contract §2.6). Same sibling-segment node keys as the sibling
//! mutation-capable window (documentation only, not a compile dependency — this file never
//! imports the editor module).

use crate::schema::snapshot::{JsonMember, JsonValue};
use crate::JsonSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::json_i_json::create_json_i_json_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️PathEncoding
/// 🆔️ The document root's node id. It must be a real, non-empty key: a tree row built with an empty
/// id falls back to its positional `#0` key, which no `TreeWindowRequest` can name and which collides
/// with any sibling that does the same.
pub const JSON_ROOT_NODE_ID: &str = "$";

/// 🧭️ One node's key is its SIBLING segment — `k=<member>` for an object member, `i=<index>` for an
/// array element — NEVER the path from the document root. Window identity is already the container
/// PATH (its ancestors' keys, then its own, joined by `TREE_WINDOW_PATH_SEPARATOR`), and a path is a
/// view-context identifier capped at 256 code points: a key that repeated its own ancestry made that
/// path grow quadratically and put any document deeper than about seven levels out of the host's
/// reach entirely.
///
/// 🔑️ The separator glyph is folded out of a member name, because a key carrying it would make the
/// composed path ambiguous and is refused at assembly with `ui.tree-window.separator-in-key`.
pub fn member_segment(key: &str) -> String {
    format!("k={}", key.replace(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR, "\u{fffd}"))
}

/// 🆔️ `segment`, made unique among the siblings already emitted. rfc8259's base grammar lets one
/// object repeat a member name, and two true siblings sharing a key are refused twice over — by the
/// UI document (`DuplicateSiblingKey`) and by the window ledger (`ui.tree-window.duplicate-key`).
fn unique_sibling_key(segment: &str, seen: &mut Vec<String>) -> String {
    let mut key = segment.to_string();
    let mut ordinal = 1usize;
    while seen.iter().any(|taken| taken == &key) {
        ordinal += 1;
        key = format!("{segment}#{ordinal}");
    }
    seen.push(key.clone());
    key
}

/// 🧭️ Drops the `#<n>` disambiguator [`unique_sibling_key`] adds to a repeated member name. A member
/// genuinely named `…#2` is indistinguishable here — repeated member names were never addressable by
/// `JsonPathSegment::Key` either, which resolves by name.
pub fn strip_sibling_ordinal(segment: &str) -> &str {
    match segment.rsplit_once('#') {
        Some((head, ordinal)) if !head.is_empty() && !ordinal.is_empty() && ordinal.chars().all(|character| character.is_ascii_digit()) => head,
        _ => segment,
    }
}

/// 🧭️ The window PATH of the node reached by `segments` (sibling segments, outermost first) — the
/// root key, then each segment, joined by `TREE_WINDOW_PATH_SEPARATOR`. This is the id `set-node`
/// addresses and the `node_key` a `TreeWindowRequest` names.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_path_id(segments: &[String]) -> String {
    let mut path = JSON_ROOT_NODE_ID.to_string();
    for segment in segments {
        path.push_str(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR);
        path.push_str(segment);
    }
    path
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
/// 👁️ Pure `JsonSnapshot -> BuiltNode` read: same shape as the editor's own render, no mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &JsonSnapshot, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    TreeWindowKit::render_windowed(&TreeView { roots: vec![node_view(JSON_ROOT_NODE_ID.to_string(), None, &document.value)] }, windows)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_view(key: String, key_label: Option<&str>, value: &JsonValue) -> TreeNodeView {
    let prefix = key_label.map(|label| format!("{label}: ")).unwrap_or_default();
    match value {
        JsonValue::Object { members } => {
            let mut seen: Vec<String> = Vec::new();
            let children = members.iter().map(|member: &JsonMember| node_view(unique_sibling_key(&member_segment(&member.key), &mut seen), Some(&member.key), &member.value)).collect();
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
