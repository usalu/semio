//! 🧺️ Borrowed XML document views and iterative owned-node retirement.
use super::{XmlDocument,XmlNode,XmlDoctype,XmlDeclaration};
/// 🪟️ Borrowed XML component roots for enclosing typed owners.
#[derive(Clone,Copy)]
pub struct XmlDocumentView<'a>{pub root:Option<&'a XmlNode>,pub doctype:Option<&'a XmlDoctype>,pub declaration:Option<&'a XmlDeclaration>,pub prolog:&'a[XmlNode],pub epilog:&'a[XmlNode]}
impl<'a> From<&'a XmlDocument> for XmlDocumentView<'a>{fn from(doc:&'a XmlDocument)->Self{Self{root:doc.root.as_ref(),doctype:doc.doctype.as_ref(),declaration:doc.declaration.as_ref(),prolog:&doc.prolog,epilog:&doc.epilog}}}
impl<'a> XmlDocumentView<'a>{pub fn node(node:&'a XmlNode)->Self{Self{root:Some(node),doctype:None,declaration:None,prolog:&[],epilog:&[]}}}

fn retire_nodes_with_frontier(nodes: impl IntoIterator<Item = XmlNode>, mut pending: Vec<XmlNode>) {
    pending.extend(nodes);
    while let Some(node) = pending.pop() { if let XmlNode::Element { children, .. } = node { pending.extend(children); } }
}
pub(crate) fn retire_nodes(nodes: impl IntoIterator<Item = XmlNode>) { retire_nodes_with_frontier(nodes, Vec::new()); }
pub(crate) struct XmlNodeList(pub(crate) Vec<XmlNode>);
impl Drop for XmlNodeList { fn drop(&mut self) { retire_nodes(std::mem::take(&mut self.0)); } }
pub(crate) struct XmlDocumentOwner(pub(crate) Option<XmlDocument>);
impl Drop for XmlDocumentOwner { fn drop(&mut self) { if let Some(mut doc) = self.0.take() { let mut nodes = std::mem::take(&mut doc.prolog); nodes.extend(std::mem::take(&mut doc.epilog)); nodes.extend(doc.root.take()); retire_nodes(nodes); } } }
/// ♻️ Retires an enclosing owner's complete typed XML document iteratively.
pub fn retire_xml_document(doc:XmlDocument){drop(XmlDocumentOwner(Some(doc)));}
/// 🧺️ Retires a validated XML document through caller-admitted fixed-capacity backing.
pub fn retire_xml_document_with_frontier(mut doc:XmlDocument,mut frontier:Vec<XmlNode>){frontier.extend(std::mem::take(&mut doc.prolog));frontier.extend(std::mem::take(&mut doc.epilog));frontier.extend(doc.root.take());retire_nodes_with_frontier(std::iter::empty(),frontier);}
