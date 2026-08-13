//! 🧬️ PptxSnapshot — an OPC package (every part verbatim, lossless) plus a typed semantic view
//! of the slide list and each slide's shape tree (`p:spTree`'s direct children, one `PptxShape`
//! per shape -- `📄docx`'s "shape -> text body -> paragraphs/runs" shape was too flat: a real
//! PresentationML slide's shapes carry a POSITION (`a:xfrm`) and a KIND (text box / picture /
//! placeholder) the old model discarded entirely, per this ticket's W0 finding). Shape kinds this
//! layer doesn't specially type (`p:graphicFrame` charts/tables/SmartArt, `p:grpSp` groups,
//! `p:cxnSp` connectors, anything unrecognized) fall back to `PptxShape::Other{xml}` -- raw,
//! verbatim, round-trip-preserving -- so nothing real on disk is silently dropped; the typed
//! variants are a convenience VIEW, `opc` is always the ground truth. Unmodeled PARTS (slide
//! layouts/masters, themes, media, …) stay verbatim inside `opc`.

use crate::artifacts::pptx::STDIO_PPTX_DOCUMENT_SCHEMA;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️PptxModel
/// ✍️ One `a:r` run — same shape as `docx::DocxRun` (shared text-model convention), plus
/// `font_size` (`a:rPr@sz`, hundredths of a point in the XML, stored here already-converted to
/// whole points).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxRun {
    pub text: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u32>,
}

/// 📄️ One `a:p` paragraph.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxParagraph {
    #[serde(default)]
    pub runs: Vec<PptxRun>,
}

impl PptxParagraph {
    pub fn text(text: impl Into<String>) -> Self {
        Self { runs: vec![PptxRun { text: text.into(), bold: false, italic: false, font_size: None }] }
    }
}

/// 📐️ A shape's `a:xfrm` position/size, in EMUs (`a:off@x/y`, `a:ext@cx/cy`) -- a weak (value)
/// entity per the recipe: whole-value replaced in diffs, never sub-diffed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxTransform {
    pub x: i64,
    pub y: i64,
    pub cx: i64,
    pub cy: i64,
}

/// 🖼️ One shape from a slide's `p:spTree` (direct children only -- shapes nested inside a
/// `p:grpSp` group fall back to `Other` on the GROUP itself, verbatim; grouped-shape typing is
/// explicitly out of scope per the brief's "reasonably-scoped shape model" instruction).
// 🩹 The internal tag is `shapeKind` (NOT `kind`) -- `Placeholder`'s own field is itself named
// `kind` (the placeholder TYPE, per the brief's field naming), which would collide with an
// internal tag literally named `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "shapeKind", rename_all = "camelCase")]
pub enum PptxShape {
    /// 📝️ `p:sp` with no `p:nvSpPr/p:nvPr/p:ph` (a plain autoshape/text box).
    TextBox {
        #[serde(default)]
        text_frame: Vec<PptxParagraph>,
        #[serde(default)]
        position: PptxTransform,
    },
    /// 🖼️ `p:pic` -- `blip_rel_id` is the `p:blipFill/a:blip@r:embed` relationship id (resolves
    /// through the slide part's own `.rels` to the actual `ppt/media/*` part in `opc`).
    Picture {
        blip_rel_id: String,
        #[serde(default)]
        position: PptxTransform,
    },
    /// 🏷️ `p:sp` WITH `p:nvSpPr/p:nvPr/p:ph` -- `kind` is the placeholder's `type` attribute
    /// (`title`/`body`/`subTitle`/`ctrTitle`/… ; ECMA-376's own default when the attribute is
    /// absent is `"body"`).
    Placeholder {
        kind: String,
        #[serde(default)]
        text_frame: Vec<PptxParagraph>,
        #[serde(default)]
        position: PptxTransform,
    },
    /// 🗄️ Raw retention for every shape kind this layer doesn't specially type (`p:graphicFrame`
    /// charts/tables/SmartArt, `p:grpSp` groups, `p:cxnSp` connectors, anything unrecognized) --
    /// the exact serialized XML of that `p:spTree` child, verbatim.
    Other {
        xml: String,
    },
}

/// 🖼️ One slide: its shape tree, in document order (`p:spTree`'s direct children).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxSlide {
    #[serde(default)]
    pub shapes: Vec<PptxShape>,
}

/// 🎞️ Typed semantic view of the slide list (`ppt/presentation.xml`'s `p:sldIdLst`, resolved
/// through `ppt/_rels/presentation.xml.rels` to each `ppt/slides/slideN.xml`) -- index-keyed:
/// presentations are ORDERED, slide order matters and is part of the model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxPresentation {
    #[serde(default)]
    pub slides: Vec<PptxSlide>,
}
//#endregion 🔖️PptxModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx")]
pub struct PptxSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(artifact)]
    #[serde(default)]
    pub presentation: PptxPresentation,
}

impl Default for PptxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc: OpcPackage::default(), presentation: PptxPresentation::default() }
    }
}

impl PptxSnapshot {
    pub fn from_parts(opc: OpcPackage, presentation: PptxPresentation) -> Self {
        Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc, presentation }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PptxSnapshot {
    const EXTENSION: &'static str = "pptx";
    fn envelope_id() -> &'static str { "stdio.pptx" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?);
        }
        crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_pptx(&bytes)
            .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_pptx(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PptxSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_pptx(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_pptx(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
