//! 💡️ Namespace and root-attribute facts read from logical XML ownership.
use super::{XmlDocument, XmlNode, retained::{RetainedXmlDocument, RetainedXmlNodeKind}};
const MC_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";

#[derive(Default, Debug, PartialEq, Eq)]
pub struct XmlMarkupFacts {
    pub namespaces: Vec<String>,
    pub root_attributes: Vec<(String, String)>,
    pub alternate_content: bool,
}

impl XmlMarkupFacts {
    fn element(&mut self, name: &str, attributes: impl Iterator<Item = (String, String)>, parent: &[(String, String)], root: bool) -> Vec<(String, String)> {
        let mut scope = parent.to_vec();
        for (name, value) in attributes {
            if root { self.root_attributes.push((name.clone(), value.clone())); }
            if let Some(prefix) = if name == "xmlns" { Some("") } else { name.strip_prefix("xmlns:") } {
                if !self.namespaces.contains(&value) { self.namespaces.push(value.clone()); }
                if let Some(binding) = scope.iter_mut().find(|(key, _)| key == prefix) { binding.1 = value; } else { scope.push((prefix.into(), value)); }
            }
        }
        let (prefix, local) = name.split_once(':').unwrap_or(("", name));
        if local == "AlternateContent" && scope.iter().find(|(key, _)| key == prefix).is_some_and(|(_, uri)| uri == MC_NS) { self.alternate_content = true; }
        scope
    }

    /// 🌳️ Borrows AST nodes and resolves each element's namespace scope.
    pub fn from_document(document: &XmlDocument) -> Self {
        let mut facts = Self::default();
        let mut nodes = document.root.as_ref().map(|root| vec![(root, Vec::new(), true)]).unwrap_or_default();
        while let Some((node, parent, root)) = nodes.pop() {
            if let XmlNode::Element { name, attrs, children } = node {
                let scope = facts.element(name, attrs.iter().map(|attr| (attr.name.clone(), attr.value.clone())), &parent, root);
                nodes.extend(children.iter().rev().map(|child| (child, scope.clone(), false)));
            }
        }
        facts
    }

    /// 🧬️ Borrows flat retained nodes without materializing a document or encoding text.
    pub fn from_retained_document(document: &RetainedXmlDocument) -> Self {
        let mut facts = Self::default();
        let mut nodes = document.root.map(|root| vec![(root, Vec::new(), true)]).unwrap_or_default();
        let mut seen = std::collections::HashSet::new();
        while let Some((index, parent, root)) = nodes.pop() {
            if !seen.insert(index) { continue; }
            let Some(node) = document.nodes.get(index) else { continue };
            if let Some(sibling) = node.next_sibling { nodes.push((sibling, parent.clone(), false)); }
            if let RetainedXmlNodeKind::Element { name, first_attribute, attribute_count, first_child } = &node.value {
                let attributes = (*first_attribute..first_attribute.saturating_add(*attribute_count)).filter_map(|index| document.attributes.get(index)).map(|attr| (attr.name.to_string_owner(), attr.value.to_string_owner()));
                let scope = facts.element(&name.to_string_owner(), attributes, &parent, root);
                if let Some(child) = first_child { nodes.push((*child, scope, false)); }
            }
        }
        facts
    }

    /// 🏷️ Whether logical namespace declarations contain this exact URI.
    pub fn declares_namespace(&self, namespace: &str) -> bool { self.namespaces.iter().any(|uri| uri == namespace) }

    /// 🪪️ Whether the root carries this exact unqualified attribute and value.
    pub fn root_attribute_is(&self, name: &str, value: &str) -> bool { self.root_attributes.iter().any(|(key, entry)| key == name && entry == value) }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
