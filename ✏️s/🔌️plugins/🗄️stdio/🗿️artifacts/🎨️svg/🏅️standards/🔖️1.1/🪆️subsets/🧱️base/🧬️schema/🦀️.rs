//! 🧬️ SvgArtifact schema — full artifact state.

use crate::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.svg` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg")]
pub struct SvgArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub doc: crate::schema::snapshot::SvgDocument,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SvgArtifact {
    fn default() -> Self {
        let snapshot = SvgSnapshot::default();
        Self { schema: snapshot.schema, doc: snapshot.doc }
    }
}

impl SvgArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SvgSnapshot {
        SvgSnapshot { schema: self.schema.clone(), doc: self.doc.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SvgSnapshot) -> Result<Self, String> {
        snapshot.doc.validate_boundaries()?;
        Ok(Self { schema: snapshot.schema, doc: snapshot.doc })
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SvgSnapshot) -> Result<(), String> {
        snapshot.doc.validate_boundaries()?;
        self.schema = snapshot.schema;
        self.doc = snapshot.doc;
        Ok(())
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.svg`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn svg_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.svg",
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

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `empty_svg_snapshot`/`demo_svg_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `SvgEngine` (zero construction sites) deleted outright;
// codecs/`io_registry` moved to `../🚪️io`; tests moved beside what they now test (see that
// file's own `mod tests`).
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_svg_snapshot() -> SvgSnapshot {
    SvgSnapshot::default()
}

/// 📄️ The demo `stdio.svg` document -- exercises every real-syntax construct the W0 census row
/// names (svg's snapshot IS an `SvgDocument`, so this mirrors `📰️xml`'s own `demo_xml_snapshot`
/// construct-for-construct): an XML declaration, a simple `<!DOCTYPE svg>`, a namespaced
/// (`:`-qualified) attribute name (`xmlns:xlink`), entity decode (`Tom &amp; Jerry`), a
/// self-closing element (carrying an attribute so its trailing `/` never fuses with the preceding
/// ident), `<![CDATA[...]]>`, `<!--...-->`, and a `<?target data?>` processing instruction. The
/// single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` in `../🚪️io`'s own tests).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_svg_snapshot() -> SvgSnapshot {
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDeclaration};
    use crate::schema::snapshot::{SvgAttr, SvgDocument, SvgNode, SvgAttributeValue, SvgLength, ViewBox};
    let root = SvgNode::Element {
        name: "svg".into(),
        attrs: vec![SvgAttr { name: "xmlns".into(), value: SvgAttributeValue::Text("http://www.w3.org/2000/svg".into()) }, SvgAttr { name: "xmlns:xlink".into(), value: SvgAttributeValue::Text("http://www.w3.org/1999/xlink".into()) }, SvgAttr { name: "viewBox".into(), value: SvgAttributeValue::ViewBox(ViewBox{min_x:0.0,min_y:0.0,width:100.0,height:100.0}) }],
        children: vec![
            SvgNode::Comment { text: " demo scene ".into() },
            SvgNode::ProcessingInstruction { target: "xml-stylesheet".into(), data: "text".into() },
            SvgNode::Element {
                name: "rect".into(),
                attrs: vec![
                    SvgAttr { name: "x".into(), value: SvgAttributeValue::Length(SvgLength{magnitude:0.0,unit:String::new()}) },
                    SvgAttr { name: "y".into(), value: SvgAttributeValue::Length(SvgLength{magnitude:0.0,unit:String::new()}) },
                    SvgAttr { name: "width".into(), value: SvgAttributeValue::Length(SvgLength{magnitude:10.0,unit:String::new()}) },
                    SvgAttr { name: "height".into(), value: SvgAttributeValue::Length(SvgLength{magnitude:10.0,unit:String::new()}) },
                    SvgAttr { name: "fill".into(), value: SvgAttributeValue::Text("red".into()) },
                ],
                children: vec![],
            },
            SvgNode::Element { name: "text".into(), attrs: vec![SvgAttr { name: "x".into(), value: SvgAttributeValue::Length(SvgLength{magnitude:5.0,unit:String::new()}) }], children: vec![SvgNode::Text { text: "Tom & Jerry".into() }] },
            SvgNode::Element { name: "circle".into(), attrs: vec![SvgAttr { name: "cx".into(), value: SvgAttributeValue::Length(SvgLength{magnitude:1.0,unit:String::new()}) }], children: vec![] },
            SvgNode::CData { text: "raw markup".into() },
        ],
    };
    let snapshot = SvgSnapshot {
        schema: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        doc: SvgDocument {
            declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true), ..Default::default() }),
            doctype: Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype{name:"svg".into(),..Default::default()}),
            prolog: Vec::new(),
            epilog: Vec::new(),
            root: Some(root),
        },
    };
    snapshot
}
//#endregion 🔖️DocumentHelpers
