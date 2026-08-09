//! 🧩️ Puzzle 2d artifact — the `puzzle.2d.fixture` document schema: the `Puzzle2dSnapshot`
//! (schema/camera/nodes/edges/meta), its node/handle/edge/kind-compatibility records, and the
//! `artifact_kind()` spec the play app's manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`,
//! `🎒️pack`, `📡️spr`, `⚙️engine`.

use serde::{Deserialize, Serialize};

pub const PUZZLE_2D_SCHEMA: &str = "puzzle.2d.fixture";

// #region 🔖️Document
/// 🎥️ The canvas camera (pan/zoom) for a puzzle 2d fixture.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Puzzle2dCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🔘️ One port on a node's rim — `handle_kind` gates link compatibility, `angle`/`radius` place it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dHandle {
    #[dsl(defines = "handle")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind: Option<String>,
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🔵️ One node — `shape: "circle"` (default, radius-sized) or `"rectangle"` (width/height-sized);
/// `handles` are its rim ports. Mirrors `infinite_board_port_directed::scene_json::FixtureJson`'s
/// per-node fields, the canonical parser this fixture format round-trips through.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dNode {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    pub x: f64,
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(default)]
    pub handles: Vec<Puzzle2dHandle>,
}

/// ➡️ One directed link between two handle ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dEdge {
    pub id: String,
    #[dsl(refs = "handle")]
    pub source: String,
    #[dsl(refs = "handle")]
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🔗️ How specifically two handle/wire kinds are allowed to link — `vortex` is a ported-graph alias
/// for `handle` (see `infinite_board_port_directed_normal::parse_compat_specificity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "lowercase")]
pub enum Puzzle2dCompatSpecificity {
    General,
    Node,
    Edge,
    Handle,
    Wire,
    Vortex,
}

/// 🧩️ One allowed (or, unidirectional, one-way-allowed) link pair between two handle/wire kind ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dKindCompatibility {
    #[serde(default)]
    pub bidirectional: bool,
    pub specificity: Puzzle2dCompatSpecificity,
    pub source: String,
    pub target: String,
}

/// 🗂️ Fixture-carried metadata: the manifest this fixture's kinds resolve against, its explicit
/// link-compatibility table, and (rarely) a self-contained `kindCatalogs` payload for fixtures
/// exported standalone — that catalog shape is genuinely freeform (handle/wire/edge kind rows vary
/// per manifest), so it stays untyped rather than duplicating the manifest schema here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle2dKindCompatibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<dsl::DslValue>,
}

//#region 🔖️Snapshot
pub use crate::artifacts::puzzle2d::snapshot::schema::Puzzle2dSnapshot;
//#endregion 🔖️Snapshot

//#region 🔖️ArtifactKind
/// 🗿️ The `2d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.puzzle".into(),
        name: "2D Puzzle".into(),
        source_format: "puzzle.2d".into(),
        component_kind: "puzzle2d".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Design },
        schema: "puzzle.2d".into(),
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
    }
}
//#endregion 🔖️ArtifactKind

pub use crate::artifacts::puzzle2d::op::Puzzle2dPlaySnapshot;
