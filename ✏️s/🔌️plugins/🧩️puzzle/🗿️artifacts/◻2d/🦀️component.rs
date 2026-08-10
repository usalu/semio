//! 🧩️ Puzzle 2d artifact — the `puzzle.2d.fixture` document schema: the `Puzzle2dSnapshot`
//! (schema/camera/nodes/edges/meta), its node/handle/edge/kind-compatibility records, and the
//! `artifact_kind()` spec the play app's manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`,
//! `🎒️pack`, `📡️spr`, `⚙️engine`.

use serde::{Deserialize, Serialize};

pub const PUZZLE_2D_SCHEMA: &str = "puzzle.2d.fixture";

//#region 🔖️Document
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

impl Default for Puzzle2dHandle {
    fn default() -> Self {
        Self {
            id: String::new(),
            handle_kind: None,
            angle: 0.0,
            radius: None,
            color: None,
            icon_kind: None,
            scale: None,
            visible: None,
            locked: None,
        }
    }
}

/// ⚓️ Whether a node keeps its stored pose (`Fixed`) or derives it from edges (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle2dNodeAnchor {
    #[default]
    Fixed,
    Derived,
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
    pub anchor: Puzzle2dNodeAnchor,
    #[serde(default)]
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
    #[serde(default)]
    pub gap: f64,
    #[serde(default)]
    pub shift: f64,
    #[serde(default)]
    pub rise: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub turn: f64,
    #[serde(default)]
    pub tilt: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl Default for Puzzle2dEdge {
    fn default() -> Self {
        Self {
            id: String::new(),
            source: String::new(),
            target: String::new(),
            edge_kind: None,
            gap: 0.0,
            shift: 0.0,
            rise: 0.0,
            rotation: 0.0,
            turn: 0.0,
            tilt: 0.0,
            x: 0.0,
            y: 0.0,
            source_tip: None,
            target_tip: None,
            visible: None,
            locked: None,
        }
    }
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
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub bidirectional: bool,
    #[serde(default)]
    pub important: bool,
    pub specificity: Puzzle2dCompatSpecificity,
}

/// 🏷️ Key/value attribute on a catalog kind (compose `Attribute` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dAttribute {
    pub id: String,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ Author credit on a catalog kind (compose `Author` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dAuthor {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ Tagged representation / LOD asset on a node kind (compose `Representation` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dRepresentation {
    pub id: String,
    pub name: String,
    pub url: String,
    pub mime: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    pub description: String,
}

/// 🌱️ Handle template on a node kind — 2d uses `angle` instead of point/direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dHandleTemplate {
    pub id: String,
    pub name: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind: Option<String>,
    #[serde(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

impl Default for Puzzle2dHandleTemplate {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            label: String::new(),
            description: String::new(),
            icon: String::new(),
            handle_kind: None,
            angle: 0.0,
            t: None,
            mandatory: None,
            radius: None,
        }
    }
}

/// 🧩 Type-like node-kind catalog row (compose `Type` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
    pub abstract_: bool,
    #[serde(default)]
    pub base_kinds: Vec<String>,
    #[serde(default)]
    pub representations: Vec<Puzzle2dRepresentation>,
    #[serde(default)]
    pub handles: Vec<Puzzle2dHandleTemplate>,
    #[serde(default)]
    pub attributes: Vec<Puzzle2dAttribute>,
    #[serde(default)]
    pub authors: Vec<Puzzle2dAuthor>,
}

/// 🔌️ Port-like handle-kind catalog row (compose `Port` parity).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dCatalogHandleKind {
    #[dsl(defines = "handle_kind")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default)]
    pub compatible_with: Vec<String>,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub default_wire_kind: String,
}

/// ➡️ Edge-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dKindCatalogs {
    #[serde(default)]
    #[dsl(table)]
    pub nodes: Vec<Puzzle2dCatalogNodeKind>,
    #[serde(default)]
    #[dsl(table)]
    pub handles: Vec<Puzzle2dCatalogHandleKind>,
    #[serde(default)]
    #[dsl(table)]
    pub edges: Vec<Puzzle2dCatalogEdgeKind>,
    #[serde(default)]
    #[dsl(table)]
    pub wires: Vec<Puzzle2dCatalogWireKind>,
}

/// 🗂️ Fixture-carried metadata: manifest id, link-compatibility table, and typed kind catalogs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle2dKindCompatibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle2dKindCatalogs>,
}
//#endregion 🔖️Document

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
        let edge = Puzzle2dEdge {
            id: "e1".into(),
            source: "a".into(),
            target: "b".into(),
            gap: 1.0,
            shift: 2.0,
            rise: 3.0,
            rotation: 10.0,
            turn: 20.0,
            tilt: 30.0,
            x: 4.0,
            y: 5.0,
            ..Default::default()
        };
        let json = serde_json::to_string(&edge).expect("serialize");
        let back: Puzzle2dEdge = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, edge);
        assert!(json.contains("\"gap\":1.0") || json.contains("\"gap\":1"));
        assert!(json.contains("\"rotation\":10"));
    }

    #[test]
    fn puzzle2d_kind_compatibility_includes_important() {
        let row = Puzzle2dKindCompatibility {
            source: "a".into(),
            target: "b".into(),
            bidirectional: true,
            important: true,
            specificity: Puzzle2dCompatSpecificity::Handle,
        };
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
                abstract_: false,
                base_kinds: vec!["base".into()],
                representations: vec![Puzzle2dRepresentation {
                    id: "r1".into(),
                    name: "mesh".into(),
                    url: "u".into(),
                    mime: "model/gltf-binary".into(),
                    tags: vec!["lod0".into()],
                    lod: Some("0".into()),
                    description: "rep".into(),
                }],
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
                attributes: vec![Puzzle2dAttribute {
                    id: "a1".into(),
                    key: "k".into(),
                    value: "v".into(),
                    definition: None,
                }],
                authors: vec![Puzzle2dAuthor {
                    id: "u1".into(),
                    name: "Ada".into(),
                    email: "a@b.c".into(),
                    role: Some("author".into()),
                    rank: Some(1),
                }],
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
            edges: vec![Puzzle2dCatalogEdgeKind {
                id: "link.e".into(),
                name: "Link".into(),
                label: "Link".into(),
                description: "".into(),
                icon: "".into(),
                color: "#000".into(),
            }],
            wires: vec![Puzzle2dCatalogWireKind {
                id: "link.w".into(),
                name: "W".into(),
                label: "W".into(),
                description: "".into(),
                icon: "".into(),
                color: "#111".into(),
                default_edge_kind: "link.e".into(),
            }],
        };
        let json = serde_json::to_value(&catalogs).expect("serialize");
        assert_eq!(json["nodes"][0]["abstract"], false);
        let back: Puzzle2dKindCatalogs = serde_json::from_value(json).expect("deserialize");
        assert_eq!(back, catalogs);
    }
}
//#endregion 🧪️Tests
