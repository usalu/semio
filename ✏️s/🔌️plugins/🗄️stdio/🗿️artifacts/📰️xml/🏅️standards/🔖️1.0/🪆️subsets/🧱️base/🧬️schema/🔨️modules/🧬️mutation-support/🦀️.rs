//! 🧰 Shared node addressing for direct XML mutations.
use crate::schema::snapshot::{XmlDocument, XmlNode};
use crate::XmlSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(transparent)]
pub struct XmlNodePath(pub Vec<usize>);
impl XmlNodePath {
    pub fn root() -> Self {
        Self(Vec::new())
    }
    pub fn resolve<'a>(&self, root: Option<&'a XmlNode>) -> Option<&'a XmlNode> {
        let mut current = root?;
        for &index in &self.0 {
            let XmlNode::Element { children, .. } = current else { return None };
            current = children.get(index)?;
        }
        Some(current)
    }
}








