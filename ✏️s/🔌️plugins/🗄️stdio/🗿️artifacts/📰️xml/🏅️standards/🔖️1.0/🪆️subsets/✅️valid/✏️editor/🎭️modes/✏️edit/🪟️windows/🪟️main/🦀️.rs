//! 📰 Xml editor — `main` window: a real, directly editable tree of the whole `XmlDocument`, built
//! from the framework `TreeWindowKit` (contract §2.6). Node ids are `/`-joined child-index paths
//! from the root element (`XmlNodePath`'s own shape) — root itself is the empty string. Only
//! `Text` nodes are real `set-node` edit targets (`XmlMutation::SetText`'s own documented scope);
//! `Element`/`CData`/`Comment`/`ProcessingInstruction` nodes render read-only in this window.

use crate::schema::snapshot::{XmlNode};
use crate::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_to_text_checked};
use crate::XmlSnapshot;
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
pub fn render(document: &XmlSnapshot, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let root = match &document.doc.root {
        Some(node) => node_view(XML_ROOT_NODE_ID.to_string(), node),
        None => TreeNodeView { id: XML_ROOT_NODE_ID.to_string(), label: "(empty document)".to_string(), children: Vec::new() },
    };
    TreeWindowKit::render_windowed(&TreeView { roots: vec![root] }, windows)
}

/// 📝️ Adds the natural XML source draft to the structured tree for editor hosts.
pub fn render_editor(
    document: &XmlSnapshot,
    locale: Locale,
    windows: &TreeWindows<'_>,
    controller_id: &str,
    revision: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let root = match &document.doc.root {
        Some(node) => node_view(XML_ROOT_NODE_ID.to_string(), node),
        None => TreeNodeView { id: XML_ROOT_NODE_ID.to_string(), label: "(empty document)".to_string(), children: Vec::new() },
    };
    let tree = TreeWindowKit::render_editable_nodes_windowed(&TreeView { roots: vec![root] }, windows, controller_id, |path, _| {
        let XmlNode::Text { text } = node_at_path(document.doc.root.as_ref()?, path)? else { return None };
        UiText::try_from_str(text)?;
        let mut arguments = UiMapBuilder::try_new()?;
        arguments.try_insert("nodeId".into(), UiValue::Text(UiText::try_from_str(path)?)).ok()?;
        arguments.try_insert("revision".into(), UiValue::Text(UiText::try_from_str(revision)?)).ok()?;
        Some(EditableTreeNode::new(text, "set-node", UiValue::Map(arguments.finish())))
    })?;
    let source = xml_document_to_text_checked(&document.doc).map_err(|message| semio_framework_plugin::PluginAssemblyError::new("stdio.xml.invalid-document", message))?;
    semio_s_artifact_stdio_contract::editing::render_file_source_editor("stdio-xml-valid-source", source, "xml", "set-node", XML_ROOT_NODE_ID, revision, publication_revision, locale, tree)
}

fn node_at_path<'a>(root: &'a XmlNode, node_id: &str) -> Option<&'a XmlNode> {
    let mut node = root;
    let mut segments = node_id.split(semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR);
    (segments.next()? == XML_ROOT_NODE_ID).then_some(())?;
    for segment in segments {
        let index = segment.parse::<usize>().ok()?;
        if index.to_string() != segment {
            return None;
        }
        let XmlNode::Element { children, .. } = node else { return None };
        node = children.get(index)?;
    }
    Some(node)
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
