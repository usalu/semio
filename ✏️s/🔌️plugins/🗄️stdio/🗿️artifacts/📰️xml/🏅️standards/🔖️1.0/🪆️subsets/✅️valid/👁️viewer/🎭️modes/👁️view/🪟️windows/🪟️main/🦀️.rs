//! 📰 Xml viewer — `main` window: a real, READ-ONLY tree of the whole `XmlDocument`, built from
//! the framework `TreeWindowKit` (contract §2.6). Same node-id path encoding as the sibling
//! mutation-capable window (documentation only — this file never imports the editor module).

use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
/// 🆔️ The document root's node id. It must be a real, non-empty key that no child index can spell: a
/// tree row built with an empty id falls back to its positional `#0` key, which no
/// `TreeWindowRequest` can name and which collides with any sibling that does the same.
pub const XML_ROOT_NODE_ID: &str = "$";

/// 🧭️ The window PATH of the node reached by `path` (child indices from the root element) — the root
/// key, then one segment per index, joined by `TREE_WINDOW_PATH_SEPARATOR`. A node is keyed by its
/// SIBLING index alone, never by its ancestry: window identity is already this path, and a path is a
/// view-context identifier capped at 256 code points, so a self-describing key made it grow
/// quadratically and put a deep document out of the host's reach.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_node_path(path: &[usize]) -> String {
    let mut encoded = XML_ROOT_NODE_ID.to_string();
    for index in path {
        encoded.push_str(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR);
        encoded.push_str(&index.to_string());
    }
    encoded
}
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::xml_valid::create_xml_valid_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `XmlSnapshot -> BuiltNode` read: same shape as the editor's own render, no mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &XmlSnapshot, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let root = match &document.doc.root {
        Some(node) => node_view(XML_ROOT_NODE_ID.to_string(), node),
        None => TreeNodeView { id: XML_ROOT_NODE_ID.to_string(), label: "(empty document)".to_string(), children: Vec::new() },
    };
    TreeWindowKit::render_windowed(&TreeView { roots: vec![root] }, windows)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_view(key: String, node: &XmlNode) -> TreeNodeView {
    let id = key;
    match node {
        XmlNode::Element { name, attrs, children } => {
            let child_views = children.iter().enumerate().map(|(index, child)| node_view(index.to_string(), child)).collect();
            TreeNodeView { id, label: format!("<{name}> ({} attrs, {} children)", attrs.len(), children.len()), children: child_views }
        }
        XmlNode::Text { text } => TreeNodeView { id, label: format!("text: {text:?}"), children: Vec::new() },
        XmlNode::CData { text } => TreeNodeView { id, label: format!("cdata: {text:?}"), children: Vec::new() },
        XmlNode::Comment { text } => TreeNodeView { id, label: format!("comment: {text:?}"), children: Vec::new() },
        XmlNode::ProcessingInstruction { target, data } => TreeNodeView { id, label: format!("<?{target} {data}?>"), children: Vec::new() },
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
