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
        crate::standards::v_ecma_376::subsets::base::schema::inferences::presentation::project_presentation(&self.opc, &self.xml_parts).map_err(|error| error.to_string())
    }

}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs




#[cfg(test)]
#[path = "🧪️tests/🔬️shadow/🦀️.rs"]
mod shadow_tests;
//#endregion 🔖️HandcraftedArtifactCodecs





#[path = "🛡️subset/🦀️.rs"]
pub(crate) mod subset;
