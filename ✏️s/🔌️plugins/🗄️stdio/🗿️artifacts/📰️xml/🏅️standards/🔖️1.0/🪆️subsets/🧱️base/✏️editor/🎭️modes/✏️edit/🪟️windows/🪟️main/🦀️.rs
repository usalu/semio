//! 📰 Xml editor — `main` window: a real, directly editable tree of the whole `XmlDocument`, built
//! from the framework `TreeWindowKit` (contract §2.6). Node ids are `/`-joined child-index paths
//! from the root element (`XmlNodePath`'s own shape) — root itself is the empty string. Only
//! `Text` nodes are real `set-node` edit targets (`XmlMutation::SetText`'s own documented scope);
//! `Element`/`CData`/`Comment`/`ProcessingInstruction` nodes render read-only in this window.

use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
/// 🆔️ The document root's node id. It must be a real, non-empty key that no child-index path can
/// spell: a tree row built with an empty id falls back to its positional `#0` key, which no
/// `TreeWindowRequest` can name and which collides with any sibling that does the same.
pub const XML_ROOT_NODE_ID: &str = "$";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::xml_any::create_xml_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `XmlSnapshot -> BuiltNode`: an empty document renders a single placeholder leaf, otherwise
/// the real element tree, child-index paths as node ids.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &XmlSnapshot, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let root = match &document.doc.root {
        Some(node) => node_view(&[], node),
        None => TreeNodeView { id: XML_ROOT_NODE_ID.to_string(), label: "(empty document)".to_string(), children: Vec::new() },
    };
    TreeWindowKit::render_windowed(&TreeView { roots: vec![root] }, windows)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_view(path: &[usize], node: &XmlNode) -> TreeNodeView {
    let id = if path.is_empty() { XML_ROOT_NODE_ID.to_string() } else { path.iter().map(|index| index.to_string()).collect::<Vec<_>>().join("/") };
    match node {
        XmlNode::Element { name, attrs, children } => {
            let child_views = children
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    let mut child_path = path.to_vec();
                    child_path.push(index);
                    node_view(&child_path, child)
                })
                .collect();
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
