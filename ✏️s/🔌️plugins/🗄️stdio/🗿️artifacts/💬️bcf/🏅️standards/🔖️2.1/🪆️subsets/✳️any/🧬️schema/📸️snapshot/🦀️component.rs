//! 🧬️ BcfSnapshot — bcfzip container (BCF-XML 2.1): a plain flat zip of per-topic folders, NOT
//! an OPC package (no content-types/relationships apparatus — see the F5 report's §1 for why this
//! artifact builds its own simple wrapper on top of zip's raw entries instead of reusing
//! `zip::opc::OpcPackage` the way docx/xlsx/pptx do). `version` is `bcf.version`'s `VersionId`;
//! `topics` is the guid-keyed typed view over every `<guid>/markup.bcf` + referenced `.bcfv`
//! viewpoint + snapshot image; `parts` retains every unmodeled/unknown file (e.g. `project.bcfp`,
//! non-topic root files, topic folders without a `markup.bcf`) verbatim.

use crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Geometry
/// 📐 A 3D point/vector (BCF-XML `visinfo.xsd` `Point`/`Direction` — both are `{X,Y,Z}` triples).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfPoint3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 📷 A viewpoint's camera — the `visinfo.xsd` `PerspectiveCamera`/`OrthogonalCamera` choice,
/// typed as a real Rust enum rather than two optional fields (the XSD makes them mutually
/// exclusive via `xs:choice`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum BcfCamera {
    Perspective {
        view_point: BcfPoint3,
        direction: BcfPoint3,
        up_vector: BcfPoint3,
        field_of_view: f64,
    },
    Orthogonal {
        view_point: BcfPoint3,
        direction: BcfPoint3,
        up_vector: BcfPoint3,
        view_to_world_scale: f64,
    },
}
//#endregion 🔖️Geometry

//#region 🔖️Components
/// 👁️ `visinfo.xsd` `ComponentVisibility`: `DefaultVisibility` attribute (spec default `true`)
/// plus the `Exceptions` component list (IFC guids of components whose visibility is the inverse
/// of the default).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfVisibility {
    pub default_visibility: bool,
    #[serde(default)]
    pub exceptions: Vec<String>,
}

/// 🎨 One `visinfo.xsd` `ComponentColoringColor`: a hex color (`ColorType`, e.g. `"FFFF0000"`)
/// applied to a set of component IFC guids.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfColoring {
    pub color: String,
    #[serde(default)]
    pub components: Vec<String>,
}

/// 🧩 `visinfo.xsd` `Components`: selection/visibility/coloring, each keyed by IFC guid strings
/// per this artifact's completeness target (a `Component` element's own optional
/// `OriginatingSystem`/`AuthoringToolId` children are out of scope — not modeled, matching the
/// target's `selection: Vec<String>` shape exactly).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfComponents {
    #[serde(default)]
    pub selection: Vec<String>,
    #[serde(default)]
    pub visibility: BcfVisibility,
    #[serde(default)]
    pub coloring: Vec<BcfColoring>,
}
//#endregion 🔖️Components

//#region 🔖️Comment
/// 💬 One `markup.bcf` `<Comment>` (BCF-XML 2.1 `markup.xsd`): `guid` is the comment's own
/// identity, `date`/`author`/`text` are the required `<Date>`/`<Author>`/`<Comment>` children
/// verbatim, `viewpoint_ref` is the optional `<Viewpoint Guid="...">` child's `Guid` attribute
/// (references a `BcfViewpoint` in the same topic by guid, not a filename).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfComment {
    pub guid: String,
    pub date: String,
    pub author: String,
    pub text: String,
    #[serde(default)]
    pub viewpoint_ref: Option<String>,
}
//#endregion 🔖️Comment

//#region 🔖️Viewpoint
/// 👁️‍🗨️ One BCF viewpoint: the typed union of a `markup.bcf` `<Viewpoints>` reference entry AND
/// the `.bcfv` file it points at (`VisualizationInfo`'s `Guid`, camera, components) AND the
/// referenced snapshot image bytes. `guid` is `VisualizationInfo/@Guid` (== the markup
/// `<Viewpoints>` entry's own `Guid`, per spec). On encode this artifact always regenerates
/// canonical `<guid>.bcfv`/`<guid>.png` filenames (documented normal form, per
/// `codec_retention_law` — arbitrary on-disk filenames from a real BCF tool are read on decode but
/// not preserved verbatim; the typed content is what round-trips).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfViewpoint {
    pub guid: String,
    #[serde(default)]
    pub camera: Option<BcfCamera>,
    #[serde(default)]
    pub components: Option<BcfComponents>,
    /// 🖼️ Referenced snapshot image bytes (PNG) — legitimate bytes-payload exception (the format
    /// IS bytes here), per the recipe's raw-payload allowlist.
    #[serde(default)]
    pub snapshot: Option<Vec<u8>>,
}
//#endregion 🔖️Viewpoint

//#region 🔖️Topic
/// 🗂️ One BCF topic (one `<guid>/markup.bcf` folder inside the container). `guid`/`status` mirror
/// `<Topic>`'s `Guid`/`TopicStatus` XML *attributes*; `title`/`description`/`priority`/`labels`/
/// `creation_date`/`creation_author` are the corresponding required/optional `<Topic>` *child
/// elements* per `markup.xsd`'s real element sequence (NOT attributes — a defect in this
/// artifact's pre-wave model, which wrongly treated `Priority` as an attribute). `comments` and
/// `viewpoints` are guid-keyed collections, each with its own per-field diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfTopic {
    pub guid: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub status: String,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub creation_date: String,
    #[serde(default)]
    pub creation_author: String,
    #[serde(default)]
    pub comments: Vec<BcfComment>,
    #[serde(default)]
    pub viewpoints: Vec<BcfViewpoint>,
}
//#endregion 🔖️Topic

//#region 🔖️RawPart
/// 🗄️ One unmodeled/unknown file retained verbatim: anything in the container that isn't
/// `bcf.version` or consumed by a topic's `markup.bcf`/`.bcfv`/snapshot triad -- e.g.
/// `project.bcfp`, custom extension files, or a topic folder that lacks a `markup.bcf` entirely.
/// This artifact's own simple package wrapper's raw-retention mechanism (see module doc).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfRawPart {
    pub name: String,
    #[serde(default)]
    pub data: Vec<u8>,
}
//#endregion 🔖️RawPart

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf")]
pub struct BcfSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 🏷️ `bcf.version`'s `VersionId` attribute (e.g. `"2.1"`).
    #[state(persistent)]
    #[serde(default)]
    pub version: String,
    /// 🗂️ Guid-keyed topics — the primary typed content.
    #[state(persistent)]
    #[serde(default)]
    pub topics: Vec<BcfTopic>,
    /// 🗄️ Name-keyed raw retention for everything not modeled above.
    #[state(persistent)]
    #[serde(default)]
    pub parts: Vec<BcfRawPart>,
}

impl Default for BcfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), version: String::new(), topics: Vec::new(), parts: Vec::new() }
    }
}

impl store::ArtifactDsl for BcfSnapshot {
    const EXTENSION: &'static str = "bcf";
    fn envelope_id() -> &'static str { "stdio.bcf" }
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
        crate::artifacts::bcf::engine::decode_bcf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::bcf::engine::encode_bcf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BcfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::bcf::engine::encode_bcf(self).map_err(|e| store::PackError::Schema(e))?;
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
        crate::artifacts::bcf::engine::decode_bcf(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️Snapshot
