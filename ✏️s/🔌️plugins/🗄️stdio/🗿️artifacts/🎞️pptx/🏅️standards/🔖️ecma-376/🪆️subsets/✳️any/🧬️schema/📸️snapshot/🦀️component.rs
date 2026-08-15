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
/// `p:grpSp` group fall back to `Other` on the group itself as a logical XML node; grouped-shape typing is
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
    /// 🗄️ Logical XML retention for every shape kind this layer doesn't specially type.
    Other { node: XmlNode },
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

//#region 🔖️XmlParts
/// 📄 One authoritative OPC XML part retained as a logical XML document.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxXmlPart {
    pub path: String,
    pub content_type: String,
    pub document: XmlDocument,
}

/// 📄 Classifies XML-bearing OPC parts without retaining imported syntax or container metadata.
pub fn pptx_part_is_xml(path: &str, content_type: &str) -> bool {
    let lower_path = path.to_ascii_lowercase();
    let lower_type = content_type.to_ascii_lowercase();
    lower_path.ends_with(".xml") || lower_path.ends_with(".vml") || lower_type.ends_with("+xml") || lower_type.ends_with("/xml") || lower_type.contains("vmldrawing")
}

fn numbered_path(path: &str, prefix: &str) -> Option<u32> {
    path.strip_prefix(prefix)?.strip_suffix(".xml")?.parse().ok()
}

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
    pub xml_parts: Vec<PptxXmlPart>,
    #[state(artifact)]
    #[serde(default)]
    pub presentation: PptxPresentation,
}

impl Default for PptxSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc: OpcPackage::default(), xml_parts: Vec::new(), presentation: PptxPresentation::default() }
    }
}

impl PptxSnapshot {
    pub fn from_parts(opc: OpcPackage, xml_parts: Vec<PptxXmlPart>, presentation: PptxPresentation) -> Self {
        let mut snapshot = Self { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), opc, xml_parts, presentation };
        snapshot.normalize_logical_keys();
        snapshot
    }

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
    pub(crate) fn from_snapshot(snapshot: &PptxSnapshot) -> Result<Self, String> {
        let mut opc = snapshot.opc.clone();
        let binary_parts = std::mem::take(&mut opc.parts).into_iter().map(|part| PptxBinaryPartRecord { path: part.path, content_type: part.content_type, bytes: part.bytes }).collect();
        let relationships = std::mem::take(&mut opc.relationships);
        let mut relationship_groups = relationships.into_iter().map(|(owner, relationships)| Ok(PptxRelationshipGroupRecord { owner, relationships: dsl::to_dsl_value(&relationships)? })).collect::<Result<Vec<_>, String>>()?;
        relationship_groups.sort_by(|left, right| left.owner.cmp(&right.owner));
        Ok(Self { schema: snapshot.schema.clone(), opc: dsl::to_dsl_value(&opc)?, binary_parts, relationship_groups, xml_parts: dsl::to_dsl_value(&snapshot.xml_parts)?, presentation: dsl::to_dsl_value(&snapshot.presentation)? })
    }

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
        let mut snapshot = PptxSnapshot::from_parts(opc, dsl::from_dsl_value(self.xml_parts)?, dsl::from_dsl_value(self.presentation)?);
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
        let record = dsl::parse(body, &PptxSnapshotRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Document })?;
        PptxSnapshotRecord::__dsl_from_record(&record).and_then(|value| value.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1))))
    }
    fn print_dsl(&self) -> String {
        let model = PptxSnapshotRecord::from_snapshot(self).expect("serializable logical pptx model");
        let body = dsl::print(&model.__dsl_to_record(), &PptxSnapshotRecord::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PptxSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let model = PptxSnapshotRecord::from_snapshot(self).map_err(store::PackError::Schema)?;
        let raw = store::pack_rt::encode_document(&PptxSnapshotRecord::__dsl_spec(), &model.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &PptxSnapshotRecord::__dsl_spec(), options)?;
        PptxSnapshotRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(store::PackError::Schema)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(PptxSnapshotRecord::__dsl_spec())
    }
}

#[cfg(test)]
mod shadow_tests {
    use super::*;

    #[test]
    fn logical_snapshot_and_facets_have_no_shadow_state() {
        let json = format!("{:?}", PptxSnapshot::default());
        for forbidden in ["physical", "sourceBytes", "nativeArchive", "semanticBlake3"] {
            assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
        }
        for facet in [
            include_str!("🟦️component.ts"),
            include_str!("🔗️component.graphql"),
            include_str!("🔣️component.json"),
            include_str!("🛰️component.proto"),
            include_str!("../🟦️component.ts"),
            include_str!("../🔗️component.graphql"),
            include_str!("../🔣️component.json"),
            include_str!("../🛰️component.proto"),
            include_str!("../🔺️diff/🟦️component.ts"),
            include_str!("../🔺️diff/🔗️component.graphql"),
            include_str!("../🔺️diff/🔣️component.json"),
            include_str!("../🔺️diff/🛰️component.proto"),
            include_str!("../🧬️mutations/🟦️component.ts"),
            include_str!("../🧬️mutations/🔗️component.graphql"),
            include_str!("../🧬️mutations/🔣️component.json"),
            include_str!("../🧬️mutations/🛰️component.proto"),
        ] {
            for forbidden in ["PptxPhysical", "sourceBytes", "source_bytes", "nativeArchive", "native_archive", "semanticBlake3", "semantic_blake3", "archiveBytes", "archive_bytes", "rawXml", "raw_xml"] {
                assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
            }
        }
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
