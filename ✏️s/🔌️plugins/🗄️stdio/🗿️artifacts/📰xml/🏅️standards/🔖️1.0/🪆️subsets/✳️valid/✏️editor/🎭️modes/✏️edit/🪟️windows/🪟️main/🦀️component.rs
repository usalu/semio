//! 📰 Xml editor — `main` window: a real, directly editable tree of the whole `XmlDocument`, built
//! from the framework `TreeWindowKit` (contract §2.6). Node ids are `/`-joined child-index paths
//! from the root element (`XmlNodePath`'s own shape) — root itself is the empty string. Only
//! `Text` nodes are real `set-node` edit targets (`XmlMutation::SetText`'s own documented scope);
//! `Element`/`CData`/`Comment`/`ProcessingInstruction` nodes render read-only in this window.

use crate::artifacts::xml::schema::snapshot::XmlNode;
use crate::artifacts::xml::XmlSnapshot;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::xml_valid::create_xml_valid_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Tree", "Baum"), icon_id: "list-tree".into(), ..TreeWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `XmlSnapshot -> BuiltNode`: an empty document renders a single placeholder leaf, otherwise
/// the real element tree, child-index paths as node ids.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &XmlSnapshot) -> BuiltNode {
    let root = match &document.doc.root {
        Some(node) => node_view(Vec::new(), node),
        None => TreeNodeView { id: String::new(), label: "(empty document)".to_string(), children: Vec::new() },
    };
    TreeWindowKit::render(&TreeView { roots: vec![root] })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_view(path: Vec<usize>, node: &XmlNode) -> TreeNodeView {
    let id = path.iter().map(|index| index.to_string()).collect::<Vec<_>>().join("/");
    match node {
        XmlNode::Element { name, attrs, children } => {
            let child_views = children
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    let mut child_path = path.clone();
                    child_path.push(index);
                    node_view(child_path, child)
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_walks_element_children() {
        let document = XmlSnapshot {
            schema: "stdio.xml".into(),
            doc: crate::artifacts::xml::schema::snapshot::XmlDocument {
                root: Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: vec![XmlNode::Text { text: "hi".into() }] }),
                doctype: None,
                declaration: None,
                prolog: Vec::new(),
            },
        };
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        assert_eq!(root.id, "");
        let child = &root.items.as_ref().unwrap()[0];
        assert_eq!(child.id, "0");
    }
}
//#endregion 🧪️Tests
