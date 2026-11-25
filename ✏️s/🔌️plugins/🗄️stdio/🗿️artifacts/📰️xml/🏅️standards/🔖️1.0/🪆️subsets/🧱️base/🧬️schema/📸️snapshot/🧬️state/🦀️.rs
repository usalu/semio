//! 🧬️ XmlSnapshot logical state: the persisted `stdio.xml` root over the shared document model.

use crate::schema::snapshot::{XmlDocument, XmlNode};
use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 📸️ Persisted `stdio.xml` snapshot.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub doc: XmlDocument,
}

impl Default for XmlSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() }
    }
}

impl XmlSnapshot {
    /// 🪞️ Returns the lossless logical state used by diff and mutation laws.
    pub fn semantic_projection(&self) -> Self {
        self.clone()
    }

    /// 🌱️ A well-formed document holding exactly one empty `<root/>` element, the blank page a new XML editor starts from.
    pub fn with_empty_root() -> Self {
        Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument { root: Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: Vec::new() }), ..XmlDocument::default() } }
    }

}
