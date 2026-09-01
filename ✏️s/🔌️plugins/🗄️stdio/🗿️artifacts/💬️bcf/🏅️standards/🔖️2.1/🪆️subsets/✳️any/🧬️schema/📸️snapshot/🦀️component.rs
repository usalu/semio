//! 🧬️ BcfSnapshot — bcfzip container (BCF-XML 2.1): a plain flat zip of per-topic folders, NOT
//! an OPC package (no content-types/relationships apparatus — see the F5 report's §1 for why this
//! artifact builds its own simple wrapper on top of zip's raw entries instead of reusing
//! `zip::opc::OpcPackage` the way docx/xlsx/pptx do). `version` is `bcf.version`'s `VersionId`;
//! `topics` is the guid-keyed typed view over every `<guid>/markup.bcf` + referenced `.bcfv`
//! viewpoint + snapshot image; `parts` retains every unmodeled/unknown file (e.g. `project.bcfp`,
//! non-topic root files, topic folders without a `markup.bcf`) verbatim.

use crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;

//#region 🔖️Geometry
/// 📐 A 3D point/vector (BCF-XML `visinfo.xsd` `Point`/`Direction` — both are `{X,Y,Z}` triples).
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfPoint3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 📷 A viewpoint's camera — the `visinfo.xsd` `PerspectiveCamera`/`OrthogonalCamera` choice,
/// typed as a real Rust enum rather than two optional fields (the XSD makes them mutually
/// exclusive via `xs:choice`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum BcfCamera {
    Perspective { view_point: BcfPoint3, direction: BcfPoint3, up_vector: BcfPoint3, field_of_view: f64 },
    Orthogonal { view_point: BcfPoint3, direction: BcfPoint3, up_vector: BcfPoint3, view_to_world_scale: f64 },
}
//#endregion 🔖️Geometry

//#region 🔖️Components
/// 👁️ `visinfo.xsd` `ComponentVisibility`: `DefaultVisibility` attribute (spec default `true`)
/// plus the `Exceptions` component list (IFC guids of components whose visibility is the inverse
/// of the default).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfVisibility {
    pub default_visibility: bool,
    #[value(default)]
    pub exceptions: Vec<String>,
}

/// 🎨 One `visinfo.xsd` `ComponentColoringColor`: a hex color (`ColorType`, e.g. `"FFFF0000"`)
/// applied to a set of component IFC guids.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfColoring {
    pub color: String,
    #[value(default)]
    pub components: Vec<String>,
}

/// 🧩 `visinfo.xsd` `Components`: selection/visibility/coloring, each keyed by IFC guid strings
/// per this artifact's completeness target (a `Component` element's own optional
/// `OriginatingSystem`/`AuthoringToolId` children are out of scope — not modeled, matching the
/// target's `selection: Vec<String>` shape exactly).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfComponents {
    #[value(default)]
    pub selection: Vec<String>,
    #[value(default)]
    pub visibility: BcfVisibility,
    #[value(default)]
    pub coloring: Vec<BcfColoring>,
}
//#endregion 🔖️Components

//#region 🔖️Comment
/// 💬 One `markup.bcf` `<Comment>` (BCF-XML 2.1 `markup.xsd`): `guid` is the comment's own
/// identity, `date`/`author`/`text` are the required `<Date>`/`<Author>`/`<Comment>` children
/// verbatim, `viewpoint_ref` is the optional `<Viewpoint Guid="...">` child's `Guid` attribute
/// (references a `BcfViewpoint` in the same topic by guid, not a filename).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfComment {
    pub guid: String,
    pub date: String,
    pub author: String,
    pub text: String,
    #[value(default)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfViewpoint {
    pub guid: String,
    #[value(default)]
    pub camera: Option<BcfCamera>,
    #[value(default)]
    pub components: Option<BcfComponents>,
    /// 🖼️ Referenced snapshot image bytes (PNG) — legitimate bytes-payload exception (the format
    /// IS bytes here), per the recipe's raw-payload allowlist.
    #[value(default)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfTopic {
    pub guid: String,
    pub title: String,
    #[value(default)]
    pub description: String,
    pub status: String,
    #[value(default)]
    pub priority: String,
    #[value(default)]
    pub labels: Vec<String>,
    #[value(default)]
    pub creation_date: String,
    #[value(default)]
    pub creation_author: String,
    #[value(default)]
    pub comments: Vec<BcfComment>,
    #[value(default)]
    pub viewpoints: Vec<BcfViewpoint>,
}
//#endregion 🔖️Topic

//#region 🔖️RawPart
/// 🗄️ One unmodeled/unknown file retained verbatim: anything in the container that isn't
/// `bcf.version` or consumed by a topic's `markup.bcf`/`.bcfv`/snapshot triad -- e.g.
/// `project.bcfp`, custom extension files, or a topic folder that lacks a `markup.bcf` entirely.
/// This artifact's own simple package wrapper's raw-retention mechanism (see module doc).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfRawPart {
    pub name: String,
    #[value(default)]
    pub data: Vec<u8>,
}
//#endregion 🔖️RawPart

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf")]
pub struct BcfSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🏷️ `bcf.version`'s `VersionId` attribute (e.g. `"2.1"`).
    #[state(artifact)]
    #[value(default)]
    pub version: String,
    /// 🗂️ Guid-keyed topics — the primary typed content.
    #[state(artifact)]
    #[value(default)]
    pub topics: Vec<BcfTopic>,
    /// 🗄️ Name-keyed raw retention for everything not modeled above.
    #[state(artifact)]
    #[value(default)]
    pub parts: Vec<BcfRawPart>,
}

impl Default for BcfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), version: String::new(), topics: Vec::new(), parts: Vec::new() }
    }
}

impl store::ArtifactDsl for BcfSnapshot {
    const EXTENSION: &'static str = "bcf";
    fn envelope_id() -> &'static str {
        "stdio.bcf"
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
        crate::artifacts::bcf::io::decode_bcf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::bcf::io::encode_bcf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BcfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::bcf::io::encode_bcf(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::bcf::io::decode_bcf(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SnapshotFixtures
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES) — pure snapshot constructors, no codec/IO concern.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_bcf_snapshot() -> BcfSnapshot {
    BcfSnapshot::default()
}

/// 🧪️ FG-wave: representative, non-empty `BcfSnapshot` -- one topic (title/description/status/
/// priority/labels/creation metadata all populated), one comment (with a `viewpoint_ref`), one
/// viewpoint (perspective camera, components w/ selection+visibility+coloring, a PNG snapshot),
/// and one raw-retained part -- the single source of truth reused by this file's own
/// `conformance_laws::grammar_conformance_law`/`protocol_walk_law`/`fixture_honesty_law` AND by
/// the shipped `📚️examples/🎬️demo` fixtures, same shape `📜️docx/…/⚙️engine/🦀️component.rs`'s own
/// `demo_docx_snapshot()` establishes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_bcf_snapshot() -> BcfSnapshot {
    BcfSnapshot {
        schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        version: "2.1".into(),
        topics: vec![BcfTopic {
            guid: "8f9e21f0-1c3e-4b6a-9b1d-9b6b6a6b6a6b".into(),
            title: "Clash on Level 2".into(),
            description: "MEP duct clashes with structural beam.".into(),
            status: "Open".into(),
            priority: "High".into(),
            labels: vec!["Clash".into(), "MEP".into()],
            creation_date: "2024-01-01T00:00:00+00:00".into(),
            creation_author: "ueli@example.com".into(),
            comments: vec![BcfComment {
                guid: "c1a3b8b0-1111-4b6a-9b1d-9b6b6a6b6a6b".into(),
                date: "2024-01-01T00:00:00+00:00".into(),
                author: "ueli@example.com".into(),
                text: "Please review this clash.".into(),
                viewpoint_ref: Some("v1a3b8b0-2222-4b6a-9b1d-9b6b6a6b6a6b".into()),
            }],
            viewpoints: vec![BcfViewpoint {
                guid: "v1a3b8b0-2222-4b6a-9b1d-9b6b6a6b6a6b".into(),
                camera: Some(BcfCamera::Perspective { view_point: BcfPoint3 { x: 1.0, y: 2.0, z: 3.0 }, direction: BcfPoint3 { x: 0.0, y: 0.0, z: -1.0 }, up_vector: BcfPoint3 { x: 0.0, y: 1.0, z: 0.0 }, field_of_view: 60.0 }),
                components: Some(BcfComponents {
                    selection: vec!["2O2Fr$t4X7Zf8NOew3FLOH".into()],
                    visibility: BcfVisibility { default_visibility: false, exceptions: vec!["1yQBoo7d5EEBLiyMxGgTLc".into()] },
                    coloring: vec![BcfColoring { color: "FFFF0000".into(), components: vec!["0BTBFw6f90Nfh9rP1dl_3n".into()] }],
                }),
                snapshot: Some(vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]),
            }],
        }],
        parts: vec![BcfRawPart { name: "project.bcfp".into(), data: b"<ProjectExtension/>".to_vec() }],
    }
}
//#endregion 🔖️SnapshotFixtures
