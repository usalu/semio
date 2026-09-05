//! 🧬️ PptxSnapshot — typed OPC metadata, logical XML and binary parts, plus a presentation model
//! of the slide list and each slide's shape tree (`p:spTree`'s direct children, one `PptxShape`
//! per shape -- `📄docx`'s "shape -> text body -> paragraphs/runs" shape was too flat: a real
//! PresentationML slide's shapes carry a POSITION (`a:xfrm`) and a KIND (text box / picture /
//! placeholder) the old model discarded entirely, per this ticket's W0 finding). Shape kinds this
//! layer doesn't specially type (`p:graphicFrame` charts/tables/SmartArt, `p:grpSp` groups,
//! `p:cxnSp` connectors, anything unrecognized) fall back to `PptxShape::Other{node}` as a
//! logical XML node, so nothing real in the document is silently dropped; the typed
//! variants are the presentation authority. Unmodeled XML parts use `XmlDocument`; binary media
//! retain their genuine content bytes in `opc`.

use crate::artifacts::pptx::STDIO_PPTX_DOCUMENT_SCHEMA;
use crate::artifacts::xml::schema::snapshot::{XmlDocument, XmlNode};
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;

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
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxTransform {
    pub x: i64,
    pub y: i64,
    pub cx: i64,
    pub cy: i64,
}

/// 🖼️ One shape from a slide's `p:spTree` (direct children only -- shapes nested inside a
/// `p:grpSp` group fall back to `Other` on the group itself as a logical XML node; grouped-shape typing is
/// explicitly out of scope per the brief's "reasonably-scoped shape model" instruction).
// 🩹 The internal tag is `shapeKind` (NOT `kind`) -- `Placeholder`'s own field is itself named
// `kind` (the placeholder TYPE, per the brief's field naming), which would collide with an
// internal tag literally named `kind`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "shapeKind", rename_all = "camelCase")]
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn numbered_path(path: &str, prefix: &str) -> Option<u32> {
    path.strip_prefix(prefix)?.strip_suffix(".xml")?.parse().ok()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_type_override_key(path: &str) -> (u8, u32, &str) {
    if path == "/ppt/presentation.xml" {
        (0, 0, path)
    } else if let Some(number) = numbered_path(path, "/ppt/slideMasters/slideMaster") {
        (1, number, path)
    } else if let Some(number) = numbered_path(path, "/ppt/slides/slide") {
        (2, number, path)
    } else if let Some(number) = numbered_path(path, "/ppt/notesMasters/notesMaster") {
        (3, number, path)
    } else if path == "/ppt/presProps.xml" {
        (4, 0, path)
    } else if path == "/ppt/viewProps.xml" {
        (5, 0, path)
    } else if path == "/ppt/theme/theme1.xml" {
        (6, 1, path)
    } else if path == "/ppt/tableStyles.xml" {
        (7, 0, path)
    } else if let Some(number) = numbered_path(path, "/ppt/slideLayouts/slideLayout") {
        (8, number, path)
    } else if let Some(number) = numbered_path(path, "/ppt/theme/theme") {
        (9, number, path)
    } else if path == "/docProps/core.xml" {
        (10, 0, path)
    } else if path == "/docProps/app.xml" {
        (11, 0, path)
    } else {
        (12, 0, path)
    }
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
    #[state(artifact)]
    #[value(default)]
    pub presentation: PptxPresentation,
}

impl Default for PptxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc: OpcPackage::default(), xml_parts: Vec::new(), presentation: PptxPresentation::default() }
    }
}

impl PptxSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_parts(opc: OpcPackage, xml_parts: Vec<PptxXmlPart>, presentation: PptxPresentation) -> Self {
        let mut snapshot = Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc, xml_parts, presentation };
        snapshot.normalize_logical_keys();
        snapshot
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn normalize_logical_keys(&mut self) {
        self.opc.parts.sort_by(|left, right| left.path.cmp(&right.path));
        self.opc.content_types.defaults.sort_by(|left, right| left.0.cmp(&right.0));
        self.opc.content_types.overrides.sort_by(|left, right| content_type_override_key(&left.0).cmp(&content_type_override_key(&right.0)));
        self.xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct PptxBinaryPartRecord {
    path: String,
    content_type: String,
    #[dsl(base64)]
    bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct PptxRelationshipGroupRecord {
    owner: String,
    relationships: dsl::DslValue,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub(crate) struct PptxSnapshotRecord {
    schema: String,
    opc: dsl::DslValue,
    #[dsl(table)]
    binary_parts: Vec<PptxBinaryPartRecord>,
    #[dsl(table)]
    relationship_groups: Vec<PptxRelationshipGroupRecord>,
    xml_parts: dsl::DslValue,
    presentation: dsl::DslValue,
}

impl PptxSnapshotRecord {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn from_snapshot(snapshot: &PptxSnapshot) -> Result<Self, String> {
        let mut opc = snapshot.opc.clone();
        let binary_parts = std::mem::take(&mut opc.parts).into_iter().map(|part| PptxBinaryPartRecord { path: part.path, content_type: part.content_type, bytes: part.bytes }).collect();
        let relationships = std::mem::take(&mut opc.relationships);
        let mut relationship_groups = relationships.into_iter().map(|(owner, relationships)| Ok(PptxRelationshipGroupRecord { owner, relationships: dsl::to_dsl_value(&relationships)? })).collect::<Result<Vec<_>, String>>()?;
        relationship_groups.sort_by(|left, right| left.owner.cmp(&right.owner));
        Ok(Self { schema: snapshot.schema.clone(), opc: dsl::to_dsl_value(&opc)?, binary_parts, relationship_groups, xml_parts: dsl::ToValue::to_value(&snapshot.xml_parts), presentation: dsl::ToValue::to_value(&snapshot.presentation) })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn into_snapshot(self) -> Result<PptxSnapshot, String> {
        let schema = self.schema;
        let mut opc: OpcPackage = dsl::from_dsl_value(self.opc)?;
        if !opc.parts.is_empty() {
            return Err("PPTX DSL OPC metadata must not contain binary parts".into());
        }
        if !opc.relationships.is_empty() {
            return Err("PPTX DSL OPC metadata must not contain relationship groups".into());
        }
        opc.parts = self.binary_parts.into_iter().map(|part| crate::artifacts::zip::opc::OpcPart { path: part.path, content_type: part.content_type, bytes: part.bytes }).collect();
        for group in self.relationship_groups {
            if opc.relationships.contains_key(&group.owner) {
                return Err(format!("PPTX DSL repeats relationship owner {}", group.owner));
            }
            opc.relationships.insert(group.owner, dsl::from_dsl_value(group.relationships)?);
        }
        let mut snapshot = PptxSnapshot::from_parts(opc, dsl::FromValue::from_value(self.xml_parts).map_err(|error| error.to_string())?, dsl::FromValue::from_value(self.presentation).map_err(|error| error.to_string())?);
        snapshot.schema = schema;
        Ok(snapshot)
    }
}

impl store::ArtifactDsl for PptxSnapshot {
    const EXTENSION: &'static str = "pptx";
    fn envelope_id() -> &'static str {
        "stdio.pptx"
    }
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
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?);
        }
        crate::artifacts::pptx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pptx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PptxSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pptx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pptx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}

#[cfg(test)]
mod shadow_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn logical_snapshot_and_facets_have_no_shadow_state() {
        let json = format!("{:?}", PptxSnapshot::default());
        for forbidden in ["physical", "sourceBytes", "nativeArchive", "semanticBlake3"] {
            assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
        }
        for facet in [
            include_str!("🟦️.ts"),
            include_str!("🔗️.graphql"),
            include_str!("🔣️.json"),
            include_str!("🛰️.proto"),
            include_str!("../🟦️.ts"),
            include_str!("../🔗️.graphql"),
            include_str!("../🔣️.json"),
            include_str!("../🛰️.proto"),
            include_str!("../🔺️diff/🟦️.ts"),
            include_str!("../🔺️diff/🔗️.graphql"),
            include_str!("../🔺️diff/🔣️.json"),
            include_str!("../🔺️diff/🛰️.proto"),
            include_str!("../🧬️mutations/🟦️.ts"),
            include_str!("../🧬️mutations/🔗️.graphql"),
            include_str!("../🧬️mutations/🔣️.json"),
            include_str!("../🧬️mutations/🛰️.proto"),
        ] {
            for forbidden in ["PptxPhysical", "sourceBytes", "source_bytes", "nativeArchive", "native_archive", "semanticBlake3", "semantic_blake3", "archiveBytes", "archive_bytes", "rawXml", "raw_xml"] {
                assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
            }
        }
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
