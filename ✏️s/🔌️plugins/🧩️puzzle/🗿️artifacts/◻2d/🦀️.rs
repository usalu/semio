//! 🧩️ Puzzle 2d artifact — the `puzzle.2d.fixture` document schema: the `Puzzle2dSnapshot`
//! (schema/camera/nodes/edges/meta), its node/handle/edge/kind-compatibility records, and the
//! `artifact_kind()` spec the play app's manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`,
//! `🎒️pack`, `📡️spr`. No `⚙️engine` sibling anymore (ticket
//! 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e): an artifact is a `🧬️schema` plus a `🚪️io`
//! system, never an engine — the old `⚙️engine`'s pure/document-only pieces moved into `🧬️schema`
//! and `🚪️io` (this file's own `declaration()` and `io_registry` shim below), and its genuinely
//! stateful `BoardHost` facade moved to `🎛️apps/◻2d/⚙️engine` (since relocated again, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, to `✏️editor/⚙️engine` under this artifact's own
//! `✏️editor` surface).

use serde::{Deserialize, Serialize};

pub const PUZZLE_2D_SCHEMA: &str = "puzzle.2d.fixture";

//#region 🔖️Document
/// 🎥️ The canvas camera (pan/zoom) for a puzzle 2d fixture.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dHandle {
    #[dsl(defines = "handle")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind: Option<String>,
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl Default for Puzzle2dHandle {
    fn default() -> Self {
        Self { id: String::new(), handle_kind: None, angle: 0.0, radius: None, color: None, icon_kind: None, scale: None, visible: None, locked: None }
    }
}

/// ⚓️ Whether a node keeps its stored pose (`Fixed`) or derives it from edges (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum Puzzle2dNodeAnchor {
    #[default]
    Fixed,
    Derived,
}

/// 🔵️ One node — `shape: "circle"` (default, radius-sized) or `"rectangle"` (width/height-sized);
/// `handles` are its rim ports. Mirrors `infinite_board_port_directed::scene_json::FixtureJson`'s
/// per-node fields, the canonical parser this fixture format round-trips through.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dNode {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    pub x: f64,
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub anchor: Puzzle2dNodeAnchor,
    #[serde(default)]
    #[value(default)]
    pub handles: Vec<Puzzle2dHandle>,
}

impl Default for Puzzle2dNode {
    fn default() -> Self {
        Self {
            id: String::new(),
            node_kind: None,
            shape: None,
            x: 0.0,
            y: 0.0,
            radius: None,
            width: None,
            height: None,
            text: None,
            icon_kind: None,
            root: None,
            scale: None,
            visible: None,
            locked: None,
            anchor: Puzzle2dNodeAnchor::Fixed,
            handles: Vec::new(),
        }
    }
}

/// ➡️ One directed link between two handle ids, with compose-parity connection parameters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dEdge {
    pub id: String,
    #[dsl(refs = "handle")]
    pub source: String,
    #[dsl(refs = "handle")]
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edge_kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub gap: f64,
    #[serde(default)]
    #[value(default)]
    pub shift: f64,
    #[serde(default)]
    #[value(default)]
    pub rise: f64,
    #[serde(default)]
    #[value(default)]
    pub rotation: f64,
    #[serde(default)]
    #[value(default)]
    pub turn: f64,
    #[serde(default)]
    #[value(default)]
    pub tilt: f64,
    #[serde(default)]
    #[value(default)]
    pub x: f64,
    #[serde(default)]
    #[value(default)]
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl Default for Puzzle2dEdge {
    fn default() -> Self {
        Self { id: String::new(), source: String::new(), target: String::new(), edge_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0, source_tip: None, target_tip: None, visible: None, locked: None }
    }
}

/// 🔗️ How specifically two handle/wire kinds are allowed to link — `vortex` is a ported-graph alias
/// for `handle` (see `infinite_board_port_directed_normal::parse_compat_specificity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum Puzzle2dCompatSpecificity {
    General,
    Node,
    Edge,
    Handle,
    Wire,
    Vortex,
}

/// 🧩️ One allowed (or, unidirectional, one-way-allowed) link pair between two handle/wire kind ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dKindCompatibility {
    pub source: String,
    pub target: String,
    #[serde(default)]
    #[value(default)]
    pub bidirectional: bool,
    #[serde(default)]
    #[value(default)]
    pub important: bool,
    pub specificity: Puzzle2dCompatSpecificity,
}

/// 🏷️ Key/value attribute on a catalog kind (compose `Attribute` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dAttribute {
    pub id: String,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ Author credit on a catalog kind (compose `Author` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dAuthor {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ Tagged representation / LOD asset on a node kind (compose `Representation` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dRepresentation {
    pub id: String,
    pub name: String,
    pub url: String,
    pub mime: String,
    #[serde(default)]
    #[value(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    pub description: String,
}

/// 🌱️ Handle template on a node kind — 2d uses `angle` instead of point/direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dHandleTemplate {
    pub id: String,
    pub name: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

impl Default for Puzzle2dHandleTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), handle_kind: None, angle: 0.0, t: None, mandatory: None, radius: None }
    }
}

/// 🧩 Type-like node-kind catalog row (compose `Type` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogNodeKind {
    #[dsl(defines = "node_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    pub image: String,
    pub unit: String,
    #[serde(default, rename = "abstract")]
    #[value(default, rename = "abstract")]
    pub is_abstract: bool,
    #[serde(default)]
    #[value(default)]
    pub base_kinds: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub representations: Vec<Puzzle2dRepresentation>,
    #[serde(default)]
    #[value(default)]
    pub handles: Vec<Puzzle2dHandleTemplate>,
    #[serde(default)]
    #[value(default)]
    pub attributes: Vec<Puzzle2dAttribute>,
    #[serde(default)]
    #[value(default)]
    pub authors: Vec<Puzzle2dAuthor>,
}

/// 🔌️ Port-like handle-kind catalog row (compose `Port` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogHandleKind {
    #[dsl(defines = "handle_kind")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default)]
    #[value(default)]
    pub compatible_with: Vec<String>,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub default_wire_kind: String,
}

/// ➡️ Edge-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogEdgeKind {
    #[dsl(defines = "edge_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    pub color: String,
}

/// 🧵 Wire-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogWireKind {
    #[dsl(defines = "wire_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    #[dsl(refs = "edge_kind")]
    pub default_edge_kind: String,
}

/// 🗂️ Typed kind-catalog bundle carried on fixture meta.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dKindCatalogs {
    #[serde(default)]
    #[value(default)]
    #[dsl(table)]
    pub nodes: Vec<Puzzle2dCatalogNodeKind>,
    #[serde(default)]
    #[value(default)]
    #[dsl(table)]
    pub handles: Vec<Puzzle2dCatalogHandleKind>,
    #[serde(default)]
    #[value(default)]
    #[dsl(table)]
    pub edges: Vec<Puzzle2dCatalogEdgeKind>,
    #[serde(default)]
    #[value(default)]
    #[dsl(table)]
    pub wires: Vec<Puzzle2dCatalogWireKind>,
}

/// 🗂️ Fixture-carried metadata: manifest id, link-compatibility table, and typed kind catalogs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle2dKindCompatibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle2dKindCatalogs>,
}
//#endregion 🔖️Document

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

//#region 🔖️Dialect
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.puzzle.puzzle2d"` matches the artifact-kind id this subset's own
/// capability rows already key off (see `definition()`'s `"s.puzzle2d.schema.artifact"` row above),
/// standard `"1"` and subset `"*"` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
/// Lives at the artifact level (not under `editor`/`viewer`) so `policyViewerPurityBreaches` never sees
/// a viewer file importing through the sibling editor module just to read this constant.
pub const PUZZLE2D_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.puzzle.puzzle2d", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
//#endregion 🔖️Dialect

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
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ Puzzle2d's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated off
/// `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not engine
/// behaviour) — replaces the puzzle2d slice of the old umbrella `register()`, which also drove
/// puzzle3d's and puzzle5d's own registration (see their sibling `declaration()`s).
///
/// **W1d update.** `register_app_schemas()` is GONE — it was never a genuine coverage gap, just
/// category-1 app-scope schema under a different name; `Puzzle2dPlayApp::app_schema()` (see that
/// impl's own doc) now covers it, declared automatically the moment `Puzzle2dPlayApp` is bound to the
/// plugin root via `.editor::<crate::editor::puzzle2d::Puzzle2dPlayApp>(…)` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `.document_app()` is retired in favor of the
/// role-split `.editor()`/`.viewer()` builder methods).
/// `register_media_io()` (`register_2d_export_handlers`/`register_dwg_import_handler`, now on
/// `crate::editor::puzzle2d::register_media_io` — ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1e moved it off the deleted `⚙️engine` to the app that owns its callback bodies) is a genuinely
/// DIFFERENT case and is still kept on `🧩️puzzle/🦀️.rs`'s own `.setup()` — it is the OS
/// media-host registry, an entirely separate 14-function family
/// (`register_2d_export_handlers`/`register_mesh_exporter`/…) from the nine §6 registrars
/// `ArtifactDeclaration` covers, keyed by a legacy OS-kind string this declaration's own `kind` isn't
/// — see `🧩️puzzle/🦀️.rs`'s `plugin()` doc for the full judgement.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.puzzle2d.standard.v1", "standard", "1", &[], None),
        ("s.puzzle2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.puzzle2d.schema.artifact", "schema", "s.puzzle.puzzle2d", &[("schema", "s.puzzle.puzzle2d")], None),
        ("s.puzzle2d.inference.artifact", "inference", "s.puzzle.puzzle2d.inference", &[("schema", "s.puzzle.puzzle2d.inference")], None),
        ("s.puzzle2d.composer.native", "composer", "s.puzzle2d@1/*", &[("dialect", "s.puzzle2d@1/*")], None),
        ("s.puzzle2d.composer.format-1", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.puzzle2d.composer.format-2", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.puzzle2d.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.puzzle2d.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.puzzle2d.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.puzzle2d.composer.format-6", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.puzzle2d.grammar.1", "grammar", "puzzle.puzzle2d", &[("grammar", "puzzle.puzzle2d")], None),
        ("s.puzzle2d.grammar.2", "grammar", "puzzle.puzzle2d.op", &[("grammar", "puzzle.puzzle2d.op")], None),
        ("s.puzzle2d.grammar.3", "grammar", "puzzle.puzzle2d.diff", &[("grammar", "puzzle.puzzle2d.diff")], None),
        ("s.puzzle2d.grammar.4", "grammar", "2d.pack", &[("grammar", "2d.pack")], None),
        ("s.puzzle2d.grammar.5", "grammar", "2d.spr", &[("grammar", "2d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Puzzle2dPlayApp>>()` derives
        // its extension claim from `<Puzzle2dPlaySnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️mutations/🦀️.rs`, the editor's real `Snapshot` type), which is
        // `"puzzle2d-play"`, not the base `Puzzle2dSnapshot`'s `"puzzle2d"`.
        ("s.puzzle2d.codec.document-1", "codec", "puzzle.2d.fixture:puzzle2d-play", &[("codec", "puzzle.2d.fixture"), ("extension", "puzzle2d-play")], None),
        ("s.puzzle2d.localization.en", "localization", "2D Puzzle", &[], Some(("en", "2D Puzzle"))),
        ("s.puzzle2d.localization.de", "localization", "2D-Puzzle", &[], Some(("de", "2D-Puzzle"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.puzzle2d")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🌳️ This artifact's declaration tree root (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
/// RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe) —
/// replaces the old `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...)
/// .composers(...).languages(...).document_codec(...)` chain, deleted outright, no dual channel) as
/// the ONLY registration channel for schema/io/viewer/editor rows. `definition()` (old
/// `ArtifactDefinition`/capability rows, above) is kept per debt D1.
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::PuzzleApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.puzzle.puzzle2d").expect("canonical puzzle2d kind"), localization: &[], standards: vec![crate::artifacts::puzzle2d::standards::v1::standard()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "puzzle.puzzle2d",
                    extension: Some("puzzle2d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::puzzle2d::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::puzzle2d::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle2d"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle2d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::puzzle2d::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::puzzle2d::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle2d.op"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle2d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::puzzle2d::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::puzzle2d::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("puzzle.puzzle2d.diff"),
                },
                dsl::LanguageSpec {
                    id: "2d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("2d.pack"),
                },
                dsl::LanguageSpec {
                    id: "2d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("2d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

pub use crate::artifacts::puzzle2d::op::Puzzle2dPlaySnapshot;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle2d_edge_connection_params_default_to_zero() {
        let edge = Puzzle2dEdge::default();
        assert_eq!(edge.gap, 0.0);
        assert_eq!(edge.shift, 0.0);
        assert_eq!(edge.rise, 0.0);
        assert_eq!(edge.rotation, 0.0);
        assert_eq!(edge.turn, 0.0);
        assert_eq!(edge.tilt, 0.0);
        assert_eq!(edge.x, 0.0);
        assert_eq!(edge.y, 0.0);
    }

    #[test]
    fn puzzle2d_node_anchor_defaults_to_fixed() {
        let node = Puzzle2dNode::default();
        assert_eq!(node.anchor, Puzzle2dNodeAnchor::Fixed);
    }

    #[test]
    fn puzzle2d_edge_serde_roundtrips_connection_params() {
        let edge = Puzzle2dEdge { id: "e1".into(), source: "a".into(), target: "b".into(), gap: 1.0, shift: 2.0, rise: 3.0, rotation: 10.0, turn: 20.0, tilt: 30.0, x: 4.0, y: 5.0, ..Default::default() };
        let json = serde_json::to_string(&edge).expect("serialize");
        let back: Puzzle2dEdge = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, edge);
        assert!(json.contains("\"gap\":1.0") || json.contains("\"gap\":1"));
        assert!(json.contains("\"rotation\":10"));
    }

    #[test]
    fn puzzle2d_kind_compatibility_includes_important() {
        let row = Puzzle2dKindCompatibility { source: "a".into(), target: "b".into(), bidirectional: true, important: true, specificity: Puzzle2dCompatSpecificity::Handle };
        let json = serde_json::to_value(&row).expect("serialize");
        assert_eq!(json["important"], true);
        assert_eq!(json["bidirectional"], true);
        assert_eq!(json["specificity"], "handle");
    }

    #[test]
    fn puzzle2d_kind_catalogs_serde_roundtrip() {
        let catalogs = Puzzle2dKindCatalogs {
            nodes: vec![Puzzle2dCatalogNodeKind {
                id: "capsule".into(),
                name: "Capsule".into(),
                label: "Capsule".into(),
                description: "d".into(),
                icon: "i".into(),
                image: "img".into(),
                unit: "m".into(),
                is_abstract: false,
                base_kinds: vec!["base".into()],
                representations: vec![Puzzle2dRepresentation { id: "r1".into(), name: "mesh".into(), url: "u".into(), mime: "model/gltf-binary".into(), tags: vec!["lod0".into()], lod: Some("0".into()), description: "rep".into() }],
                handles: vec![Puzzle2dHandleTemplate {
                    id: "h0".into(),
                    name: "bottom".into(),
                    label: "Bottom".into(),
                    description: "".into(),
                    icon: "".into(),
                    handle_kind: Some("core.rect.bottom".into()),
                    angle: 0.0,
                    t: Some(0.5),
                    mandatory: Some(true),
                    radius: Some(3.0),
                }],
                attributes: vec![Puzzle2dAttribute { id: "a1".into(), key: "k".into(), value: "v".into(), definition: None }],
                authors: vec![Puzzle2dAuthor { id: "u1".into(), name: "Ada".into(), email: "a@b.c".into(), role: Some("author".into()), rank: Some(1) }],
            }],
            handles: vec![Puzzle2dCatalogHandleKind {
                id: "core.rect.bottom".into(),
                code: Some("B".into()),
                label: Some("Bottom".into()),
                order: Some(0),
                compatible_with: vec!["core.rect.top".into()],
                description: "".into(),
                icon: "".into(),
                color: "#112233".into(),
                default_wire_kind: "link.w".into(),
            }],
            edges: vec![Puzzle2dCatalogEdgeKind { id: "link.e".into(), name: "Link".into(), label: "Link".into(), description: "".into(), icon: "".into(), color: "#000".into() }],
            wires: vec![Puzzle2dCatalogWireKind { id: "link.w".into(), name: "W".into(), label: "W".into(), description: "".into(), icon: "".into(), color: "#111".into(), default_edge_kind: "link.e".into() }],
        };
        let json = serde_json::to_value(&catalogs).expect("serialize");
        assert_eq!(json["nodes"][0]["abstract"], false);
        let back: Puzzle2dKindCatalogs = serde_json::from_value(json).expect("deserialize");
        assert_eq!(back, catalogs);
    }
}
//#endregion 🧪️Tests
