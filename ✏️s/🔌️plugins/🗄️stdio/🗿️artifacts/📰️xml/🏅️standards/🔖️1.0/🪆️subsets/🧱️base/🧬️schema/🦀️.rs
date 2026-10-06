//! 🧬️ XmlArtifact schema — full artifact state.

use crate::XmlSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.xml` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub doc: crate::schema::snapshot::XmlDocument,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for XmlArtifact {
    fn default() -> Self {
        let snapshot = XmlSnapshot::default();
        Self { schema: snapshot.schema, doc: snapshot.doc }
    }
}

impl XmlArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> XmlSnapshot {
        XmlSnapshot { schema: self.schema.clone(), doc: self.doc.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: XmlSnapshot) -> Result<Self, String> {
        crate::schema::snapshot::validate_xml_document_boundaries(&snapshot.doc)?;
        Ok(Self { schema: snapshot.schema, doc: snapshot.doc })
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: XmlSnapshot) -> Result<(), String> {
        crate::schema::snapshot::validate_xml_document_boundaries(&snapshot.doc)?;
        self.schema = snapshot.schema;
        self.doc = snapshot.doc;
        Ok(())
    }
}
//#endregion 🔖️Conversions

/// 🔗️ Exact XML document leaves retained by every artifact whose structural contract references them.
pub const XML_DOCUMENT_SCHEMA_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), typescript: include_str!("📸️snapshot/🟦️.ts"), graphql: include_str!("📸️snapshot/🔗️.graphql"), json_schema: include_str!("📸️snapshot/🔣️.json"), proto: include_str!("📸️snapshot/🛰️.proto") };

/// 📚️ Publishes XML's own exact document schema export.
pub const XML_SHARED_SCHEMA_DOCUMENTS: semio_framework_schema_registry::ScopeSchemaExports = semio_framework_schema_registry::ScopeSchemaExports {
    scope: "s.stdio.xml",
    exports: &[semio_framework_schema_registry::SchemaExport { id: "document", leaves: XML_DOCUMENT_SCHEMA_LEAVES }],
};

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.xml`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.xml",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_xml_snapshot() -> XmlSnapshot {
    XmlSnapshot::default()
}

/// 📄️ The demo `stdio.xml` document -- exercises every real-syntax construct the W0 census row
/// names: an XML declaration, a simple `<!DOCTYPE name>`, a namespaced (`:`-qualified) attribute
/// name, both quote-delimiter styles (`"`/`'`, via `attribute`'s shared `TEXT` terminal), entity
/// decode (`Tom &amp; Jerry`), a self-closing element (carrying an attribute so its trailing `/`
/// never fuses with the preceding ident -- see `../…/📸️snapshot/📝️text/📖️.grammar.semio`'s
/// own `name` doc comment), `<![CDATA[...]]>`, `<!--...-->`, and a `<?target data?>` processing
/// instruction. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_xml_snapshot() -> XmlSnapshot {
    use crate::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlNode};
    use crate::STDIO_XML_DOCUMENT_SCHEMA;
    let root = XmlNode::Element {
        name: "catalog".into(),
        attrs: vec![XmlAttr { name: "xmlns:c".into(), value: "urn:example:catalog".into() }, XmlAttr { name: "version".into(), value: "2".into() }],
        children: vec![
            XmlNode::Comment { text: " demo catalog ".into() },
            XmlNode::ProcessingInstruction { target: "xml-stylesheet".into(), data: "text".into() },
            XmlNode::Element { name: "item".into(), attrs: vec![XmlAttr { name: "id".into(), value: "1".into() }], children: vec![XmlNode::Text { text: "Tom & Jerry".into() }] },
            XmlNode::Element { name: "empty".into(), attrs: vec![XmlAttr { name: "flag".into(), value: "true".into() }], children: vec![] },
            XmlNode::CData { text: "raw markup".into() },
        ],
    };
    XmlSnapshot {
        schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
        doc: XmlDocument {
            declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true), ..Default::default() }),
            doctype: Some("<!DOCTYPE catalog>".into()),
            prolog: Vec::new(),
            epilog: Vec::new(),
            root: Some(root),
        },
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
