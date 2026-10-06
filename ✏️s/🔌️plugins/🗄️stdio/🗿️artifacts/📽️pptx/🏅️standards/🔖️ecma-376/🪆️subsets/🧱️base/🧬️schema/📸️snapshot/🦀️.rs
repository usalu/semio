//! 🧬️ PptxSnapshot — typed OPC metadata plus authoritative logical XML and binary parts. A
//! presentation model of the slide list and each slide's shape tree (`p:spTree`'s direct children, one `PptxShape`
//! per shape -- `📄docx`'s "shape -> text body -> paragraphs/runs" shape was too flat: a real
//! PresentationML slide's shapes carry a POSITION (`a:xfrm`) and a KIND (text box / picture /
//! placeholder) the old model discarded entirely, per this ticket's W0 finding). Shape kinds this
//! layer doesn't specially type (`p:graphicFrame` charts/tables/SmartArt, `p:grpSp` groups,
//! `p:cxnSp` connectors, anything unrecognized) fall back to `PptxShape::Other{node}` as a
//! logical XML node, so nothing real in the document is silently dropped; the typed
//! variants are derived projections. Unmodeled XML parts use `XmlDocument`; binary media
//! retain their genuine content bytes in `opc`.

use crate::STDIO_PPTX_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::OpcPackage;

//#region 🔖️PptxModel
/// ✍️ One `a:r` run — same shape as `docx::DocxRun` (shared text-model convention), plus
/// `font_size` (`a:rPr@sz`, hundredths of a point in the XML, stored here already-converted to
/// whole points).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxRun {
    pub text: String,
    #[value(default)]
    pub bold: bool,
    #[value(default)]
    pub italic: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u32>,
}

/// 📄️ One `a:p` paragraph.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxParagraph {
    #[value(default)]
    pub runs: Vec<PptxRun>,
}

impl PptxParagraph {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn text(text: impl Into<String>) -> Self {
        Self { runs: vec![PptxRun { text: text.into(), bold: false, italic: false, font_size: None }] }
    }
}

/// 📐️ A shape's `a:xfrm` position/size, in EMUs (`a:off@x/y`, `a:ext@cx/cy`) -- a weak (value)
/// entity per the recipe: whole-value replaced in diffs, never sub-diffed.
#[path = "🧭️transform/🦀️.rs"]
mod transform;

#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxTransform {
    #[value(
        serialize_with = "transform::to_value",
        deserialize_with = "transform::from_value",
        serialize_controlled_with = "transform::to_value_controlled",
        deserialize_controlled_with = "transform::from_value_controlled",
        retire_with = "std::mem::drop"
    )]
    pub x: i64,
    #[value(
        serialize_with = "transform::to_value",
        deserialize_with = "transform::from_value",
        serialize_controlled_with = "transform::to_value_controlled",
        deserialize_controlled_with = "transform::from_value_controlled",
        retire_with = "std::mem::drop"
    )]
    pub y: i64,
    #[value(
        serialize_with = "transform::to_value",
        deserialize_with = "transform::from_value",
        serialize_controlled_with = "transform::to_value_controlled",
        deserialize_controlled_with = "transform::from_value_controlled",
        retire_with = "std::mem::drop"
    )]
    pub cx: i64,
    #[value(
        serialize_with = "transform::to_value",
        deserialize_with = "transform::from_value",
        serialize_controlled_with = "transform::to_value_controlled",
        deserialize_controlled_with = "transform::from_value_controlled",
        retire_with = "std::mem::drop"
    )]
    pub cy: i64,
}

/// 🖼️ One shape from a slide's `p:spTree` (direct children only -- shapes nested inside a
/// `p:grpSp` group fall back to `Other` on the group itself as a logical XML node; grouped-shape typing is
/// explicitly out of scope per the brief's "reasonably-scoped shape model" instruction).
// 🩹 The internal tag is `shapeKind` (NOT `kind`) -- `Placeholder`'s own field is itself named
// `kind` (the placeholder TYPE, per the brief's field naming), which would collide with an
// internal tag literally named `kind`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "shapeKind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PptxShape {
    /// 📝️ `p:sp` with no `p:nvSpPr/p:nvPr/p:ph` (a plain autoshape/text box).
    TextBox {
        #[value(default)]
        text_frame: Vec<PptxParagraph>,
        #[value(default)]
        position: PptxTransform,
    },
    /// 🖼️ `p:pic` -- `blip_rel_id` is the `p:blipFill/a:blip@r:embed` relationship id (resolves
    /// through the slide part's own `.rels` to the actual `ppt/media/*` part in `opc`).
    Picture {
        blip_rel_id: String,
        #[value(default)]
        position: PptxTransform,
    },
    /// 🏷️ `p:sp` WITH `p:nvSpPr/p:nvPr/p:ph` -- `kind` is the placeholder's `type` attribute
    /// (`title`/`body`/`subTitle`/`ctrTitle`/… ; ECMA-376's own default when the attribute is
    /// absent is `"body"`).
    Placeholder {
        kind: String,
        #[value(default)]
        text_frame: Vec<PptxParagraph>,
        #[value(default)]
        position: PptxTransform,
    },
    /// 🗄️ Logical XML retention for every shape kind this layer doesn't specially type.
    Other { node: XmlNode },
}

/// 🖼️ One slide: its shape tree, in document order (`p:spTree`'s direct children).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxSlide {
    #[value(default)]
    pub shapes: Vec<PptxShape>,
}

/// 🎞️ Typed semantic view of the slide list (`ppt/presentation.xml`'s `p:sldIdLst`, resolved
/// through `ppt/_rels/presentation.xml.rels` to each `ppt/slides/slideN.xml`) -- index-keyed:
/// presentations are ORDERED, slide order matters and is part of the model.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxPresentation {
    #[value(default)]
    pub slides: Vec<PptxSlide>,
}
//#endregion 🔖️PptxModel

//#region 🔖️XmlParts
/// 📄 One authoritative OPC XML part retained as a logical XML document.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxXmlPart {
    pub path: String,
    pub content_type: String,
    pub document: XmlDocument,
}

/// 📄 Classifies XML-bearing OPC parts without retaining imported syntax or container metadata.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pptx_part_is_xml(path: &str, content_type: &str) -> bool {
    let lower_path = path.to_ascii_lowercase();
    let lower_type = content_type.to_ascii_lowercase();
    lower_path.ends_with(".xml") || lower_path.ends_with(".vml") || lower_type.ends_with("+xml") || lower_type.ends_with("/xml") || lower_type.contains("vmldrawing")
}

//#endregion 🔖️XmlParts

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx")]
pub struct PptxSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub opc: OpcPackage,
    #[state(artifact)]
    #[value(default)]
    pub xml_parts: Vec<PptxXmlPart>,
}

impl Default for PptxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc: OpcPackage::default(), xml_parts: Vec::new() }
    }
}

impl PptxSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_parts(opc: OpcPackage, xml_parts: Vec<PptxXmlPart>) -> Self {
        Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc, xml_parts }
    }

    /// 🎞️ Projects the typed presentation view without creating a second persisted authority.
    pub fn presentation(&self) -> Result<PptxPresentation, String> {
        crate::standards::v_ecma_376::subsets::base::io::import::deserializers::project_presentation(&self.opc, &self.xml_parts).map_err(|error| error.to_string())
    }

    /// 🧾️ One part's content as the encoder writes it, wherever that part lives: a logical XML
    /// part is materialized through the OPC text writer, a retained binary part is read as utf-8.
    /// Every reader that used to call `opc.part_bytes` for an XML part must call THIS — since the
    /// logical split, `opc.parts` carries only the parts `pptx_part_is_xml` rejects.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn part_text(&self, path: &str) -> Option<String> {
        let key = path.trim_start_matches('/');
        if let Some(part) = self.xml_parts.iter().find(|part| part.path == key) {
            return Some(semio_s_artifact_stdio_zip::opc::xml_document_to_opc_text(&part.document));
        }
        self.opc.part_bytes(key).and_then(|bytes| String::from_utf8(bytes.to_vec()).ok())
    }

    /// 🧾️ Every part that has a textual form, in path order — the surface a whole-package
    /// conformance sweep (namespace/VML/AlternateContent checks) has to walk.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn part_texts(&self) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = self.xml_parts.iter().map(|part| (part.path.clone(), semio_s_artifact_stdio_zip::opc::xml_document_to_opc_text(&part.document))).collect();
        out.extend(self.opc.parts.iter().filter_map(|part| String::from_utf8(part.bytes.clone()).ok().map(|text| (part.path.clone(), text))));
        out.sort_by(|left, right| left.0.cmp(&right.0));
        out
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct PptxBinaryPartRecord {
    path: String,
    content_type: String,
    #[dsl(base64)]
    bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct PptxRelationshipGroupRecord {
    owner: String,
    relationships: semio_framework_value::DslValue,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct PptxSnapshotRecord {
    schema: String,
    opc: semio_framework_value::DslValue,
    #[dsl(table)]
    binary_parts: Vec<PptxBinaryPartRecord>,
    #[dsl(table)]
    relationship_groups: Vec<PptxRelationshipGroupRecord>,
    xml_parts: semio_framework_value::DslValue,
}

impl PptxSnapshotRecord {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn from_snapshot(snapshot: &PptxSnapshot) -> Result<Self, String> {
        let mut opc = snapshot.opc.clone();
        let binary_parts = std::mem::take(&mut opc.parts).into_iter().map(|part| PptxBinaryPartRecord { path: part.path, content_type: part.content_type, bytes: part.bytes }).collect();
        let relationships = std::mem::take(&mut opc.relationships);
        let mut relationship_groups = relationships.into_groups().map(|(owner, relationships)| Ok(PptxRelationshipGroupRecord { owner, relationships: semio_framework_value::ToValue::to_value(&relationships) })).collect::<Result<Vec<_>, String>>()?;
        relationship_groups.sort_by(|left, right| left.owner.cmp(&right.owner));
        Ok(Self { schema: snapshot.schema.clone(), opc: semio_framework_value::ToValue::to_value(&opc), binary_parts, relationship_groups, xml_parts: semio_framework_value::ToValue::to_value(&snapshot.xml_parts) })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn into_snapshot(self) -> Result<PptxSnapshot, String> {
        let schema = self.schema;
        let mut opc: OpcPackage = semio_framework_value::FromValue::from_value(self.opc).map_err(|error| error.to_string())?;
        if !opc.parts.is_empty() {
            return Err("PPTX DSL OPC metadata must not contain binary parts".into());
        }
        if opc.relationships.owner_count() != 0 {
            return Err("PPTX DSL OPC metadata must not contain relationship groups".into());
        }
        opc.parts = self.binary_parts.into_iter().map(|part| semio_s_artifact_stdio_zip::opc::OpcPart { path: part.path, content_type: part.content_type, bytes: part.bytes }).collect();
        for group in self.relationship_groups {
            if opc.relationships.relationships(&group.owner).is_some() {
                return Err(format!("PPTX DSL repeats relationship owner {}", group.owner));
            }
            opc.relationships.replace_owner(group.owner, semio_framework_value::FromValue::from_value(group.relationships).map_err(|error| error.to_string())?);
        }
        let mut snapshot = PptxSnapshot::from_parts(opc, semio_framework_value::FromValue::from_value(self.xml_parts).map_err(|error| error.to_string())?);
        snapshot.schema = schema;
        Ok(snapshot)
    }
}




#[cfg(test)]
#[path = "🧪️tests/🔬️shadow/🦀️.rs"]
mod shadow_tests;
//#endregion 🔖️HandcraftedArtifactCodecs



#[path = "🧩️native/🦀️.rs"]
mod native;

#[path = "🛡️subset/🦀️.rs"]
mod subset;
