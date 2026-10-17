//! 🧱️ Records of the puzzle2d schema: the camera, node, handle, edge, region, catalog and meta records, the schema identity and the payload invariants every leaf diff checks.

use {semio_framework_value::list::PagedList, semio_framework_value::paged::PagedUtf8};

pub const PUZZLE_2D_SCHEMA: &str = "board.ports.directed.v1";

/// 🎥️ The canvas camera (pan/zoom) for a puzzle 2d snapshot.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dHandle {
    #[dsl(defines = "handle")]
    pub id: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind: Option<PagedUtf8<{ usize::MAX }>>,
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl Default for Puzzle2dHandle {
    fn default() -> Self {
        Self { id: PagedUtf8::new(), handle_kind: None, angle: 0.0, radius: None, color: None, icon_kind: None, scale: None, visible: None, locked: None }
    }
}

/// ⚓️ Whether a node keeps its stored pose (`Fixed`) or derives it from edges (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Puzzle2dNodeAnchor {
    #[default]
    Fixed,
    Derived,
}

/// 🔵️ One node — `shape: "circle"` (default, radius-sized) or `"rectangle"` (width/height-sized);
/// `🐙️handles` are its rim ports. Mirrors `semio_framework_os_infinite::schema::BoardSnapshot`'s
/// per-node fields, the canonical parser this snapshot format round-trips through.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dNode {
    pub id: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<PagedUtf8<{ usize::MAX }>>,
    pub x: f64,
    pub y: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub anchor: Puzzle2dNodeAnchor,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub handles: PagedList<Puzzle2dHandle, { usize::MAX }>,
}

impl Default for Puzzle2dNode {
    fn default() -> Self {
        Self {
            id: PagedUtf8::new(),
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
            handles: PagedList::new(),
        }
    }
}

/// ➡️ One directed link between two handle ids, with compose-parity connection parameters.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dEdge {
    pub id: PagedUtf8<{ usize::MAX }>,
    #[dsl(refs = "handle")]
    pub source: PagedUtf8<{ usize::MAX }>,
    #[dsl(refs = "handle")]
    pub target: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edge_kind: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub gap: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub shift: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rise: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rotation: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub turn: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tilt: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub y: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source_tip: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_tip: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl Default for Puzzle2dEdge {
    fn default() -> Self {
        Self { id: PagedUtf8::new(), source: PagedUtf8::new(), target: PagedUtf8::new(), edge_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0, source_tip: None, target_tip: None, visible: None, locked: None }
    }
}

/// 🔗️ How specifically two handle/wire kinds are allowed to link — `vortex` is a ported-graph alias
/// for `handle` (see `semio_framework_os_infinite::parse_compat_specificity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "lowercase"))]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dKindCompatibility {
    pub source: PagedUtf8<{ usize::MAX }>,
    pub target: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub bidirectional: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub important: bool,
    pub specificity: Puzzle2dCompatSpecificity,
}

/// 🏷️ Key/value attribute on a catalog kind (compose `Attribute` parity).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dAttribute {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub key: PagedUtf8<{ usize::MAX }>,
    pub value: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<PagedUtf8<{ usize::MAX }>>,
}

/// ✍️ Author credit on a catalog kind (compose `Author` parity).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dAuthor {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub name: PagedUtf8<{ usize::MAX }>,
    pub email: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ Tagged representation / LOD asset on a node kind (compose `Representation` parity).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dRepresentation {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub name: PagedUtf8<{ usize::MAX }>,
    pub url: PagedUtf8<{ usize::MAX }>,
    pub mime: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tags: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<PagedUtf8<{ usize::MAX }>>,
    pub description: PagedUtf8<{ usize::MAX }>,
}

/// 🌱️ Handle template on a node kind — 2d uses `angle` instead of point/direction.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dHandleTemplate {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub name: PagedUtf8<{ usize::MAX }>,
    pub label: PagedUtf8<{ usize::MAX }>,
    pub description: PagedUtf8<{ usize::MAX }>,
    pub icon: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

impl Default for Puzzle2dHandleTemplate {
    fn default() -> Self {
        Self { id: PagedUtf8::new(), name: PagedUtf8::new(), label: PagedUtf8::new(), description: PagedUtf8::new(), icon: PagedUtf8::new(), handle_kind: None, angle: 0.0, t: None, mandatory: None, radius: None }
    }
}

/// 🧩 Type-like node-kind catalog row (compose `Type` parity).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogNodeKind {
    #[dsl(defines = "node_kind")]
    pub id: PagedUtf8<{ usize::MAX }>,
    pub name: PagedUtf8<{ usize::MAX }>,
    pub label: PagedUtf8<{ usize::MAX }>,
    pub description: PagedUtf8<{ usize::MAX }>,
    pub icon: PagedUtf8<{ usize::MAX }>,
    pub image: PagedUtf8<{ usize::MAX }>,
    pub unit: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, rename = "abstract"))]
    #[value(default, rename = "abstract")]
    pub is_abstract: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub base_kinds: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub representations: PagedList<Puzzle2dRepresentation, { usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub handles: PagedList<Puzzle2dHandleTemplate, { usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attributes: PagedList<Puzzle2dAttribute, { usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub authors: PagedList<Puzzle2dAuthor, { usize::MAX }>,
}

/// 🔌️ Port-like handle-kind catalog row (compose `Port` parity).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogHandleKind {
    #[dsl(defines = "handle_kind")]
    pub id: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub compatible_with: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>,
    pub description: PagedUtf8<{ usize::MAX }>,
    pub icon: PagedUtf8<{ usize::MAX }>,
    pub color: PagedUtf8<{ usize::MAX }>,
    pub default_wire_kind: PagedUtf8<{ usize::MAX }>,
}

/// ➡️ Edge-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogEdgeKind {
    #[dsl(defines = "edge_kind")]
    pub id: PagedUtf8<{ usize::MAX }>,
    pub name: PagedUtf8<{ usize::MAX }>,
    pub label: PagedUtf8<{ usize::MAX }>,
    pub description: PagedUtf8<{ usize::MAX }>,
    pub icon: PagedUtf8<{ usize::MAX }>,
    pub color: PagedUtf8<{ usize::MAX }>,
}

/// 🧵 Wire-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dCatalogWireKind {
    #[dsl(defines = "wire_kind")]
    pub id: PagedUtf8<{ usize::MAX }>,
    pub name: PagedUtf8<{ usize::MAX }>,
    pub label: PagedUtf8<{ usize::MAX }>,
    pub description: PagedUtf8<{ usize::MAX }>,
    pub icon: PagedUtf8<{ usize::MAX }>,
    pub color: PagedUtf8<{ usize::MAX }>,
    #[dsl(refs = "edge_kind")]
    pub default_edge_kind: PagedUtf8<{ usize::MAX }>,
}

/// 🗂️ Typed kind-catalog bundle carried on snapshot meta.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dKindCatalogs {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub nodes: PagedList<Puzzle2dCatalogNodeKind, { usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub handles: PagedList<Puzzle2dCatalogHandleKind, { usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub edges: PagedList<Puzzle2dCatalogEdgeKind, { usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub wires: PagedList<Puzzle2dCatalogWireKind, { usize::MAX }>,
}

/// 🎯️ A persisted axis-aligned board rectangle constraining fill placement (Area Brush painted or
/// gumball relocated) — the flat analogue of puzzle3d's `Puzzle3dTargetVolume`. `width`/`height` are
/// board units measured from `x`/`y` (the rectangle's minimum corner); a negative extent is
/// normalized by every reader through [`Puzzle2dTargetRegion::bounds`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dTargetRegion {
    pub id: PagedUtf8<{ usize::MAX }>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub y: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub width: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub height: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

impl Puzzle2dTargetRegion {
    /// 📐️ Normalized `[min_x, min_y, max_x, max_y]` — the one place a negative `width`/`height` is
    /// resolved, so containment never depends on which corner the brush started from.
    pub fn bounds(&self) -> [f64; 4] {
        let (min_x, max_x) = if self.width < 0.0 { (self.x + self.width, self.x) } else { (self.x, self.x + self.width) };
        let (min_y, max_y) = if self.height < 0.0 { (self.y + self.height, self.y) } else { (self.y, self.y + self.height) };
        [min_x, min_y, max_x, max_y]
    }

    /// 🎯️ Whether the axis-aligned box `[min_x, min_y, max_x, max_y]` lies fully inside this region.
    pub fn contains_bounds(&self, aabb: [f64; 4]) -> bool {
        let [min_x, min_y, max_x, max_y] = self.bounds();
        aabb[0] >= min_x && aabb[1] >= min_y && aabb[2] <= max_x && aabb[3] <= max_y
    }
}

/// 🎯️ Whether ANY visible region contains `aabb`. An empty visible set is unconstrained — exactly
/// puzzle3d's `world_volumes_contain_aabb` rule, flattened.
pub fn puzzle2d_regions_contain_bounds(regions: &PagedList<Puzzle2dTargetRegion, {usize::MAX}>, aabb: [f64; 4]) -> bool {
    let mut visible = regions.iter().filter(|region| !region.hidden).peekable();
    if visible.peek().is_none() {
        return true;
    }
    visible.any(|region| region.contains_bounds(aabb))
}

/// 🗂️ Fixture-carried metadata: manifest id, link-compatibility table, and typed kind catalogs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dMeta {
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub kind_compatibility: PagedList<Puzzle2dKindCompatibility, { usize::MAX }>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle2dKindCatalogs>,
}

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.puzzle.puzzle2d"` matches the artifact-kind id this subset's own
/// capability rows already key off (see `definition()`'s `"s.puzzle2d.schema.artifact"` row above),
/// standard `"1"` and subset `"*"` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
/// Lives at the artifact level (not under `editor`/`viewer`) so `policyViewerPurityBreaches` never sees
/// a viewer file importing through the sibling editor module just to read this constant.
pub const PUZZLE2D_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.puzzle.puzzle2d", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

/// 🗿️ The `2d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.puzzle".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D Puzzle", "2D-Puzzle"),
        source_format: "puzzle.2d".into(),
        component_kind: "puzzle2d".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Design },
        schema: PUZZLE_2D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.dxf".into(), "stdio.json".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.svg".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.dxf".into(), "stdio.json".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.svg".into()],
    }
}

// 🚨️ Schema-first payload invariants: every value a puzzle 2d leaf payload schema forbids through its hard
// bounds — a non-finite number, a non-positive extent, scale or factor, a shape outside `circle|rectangle`, a
// template rim parameter outside `0..=1`, a negative catalogue order or rank, an empty or repeated target set —
// is refused by the leaf's diff as a Fatal `mutation.invariant` with the default diff (the frozen verb-family
// rule "Fatal non-finite or non-positive"), before the base is consulted.

/// ♾️ Every named number is finite.
pub fn puzzle2d_finite(values: &[(&str, f64)]) -> Result<(), String> {
    values.iter().find(|(_, value)| !value.is_finite()).map_or(Ok(()), |(name, _)| Err(format!("{name} must be a finite number")))
}

/// ➕️ Every present named number is finite and greater than zero.
pub fn puzzle2d_positive(values: &[(&str, Option<f64>)]) -> Result<(), String> {
    values.iter().find(|(_, value)| value.is_some_and(|value| !(value.is_finite() && value > 0.0))).map_or(Ok(()), |(name, _)| Err(format!("{name} must be a finite number greater than 0")))
}

/// 🔷️ A present shape names one of the two figures a node can be.
pub fn puzzle2d_shape(name: &str, shape: Option<&PagedUtf8<{ usize::MAX }>>) -> Result<(), String> {
    match shape {
        Some(shape) if !(shape.eq_str("circle") || shape.eq_str("rectangle")) => Err(format!("{name} must be circle or rectangle, not {shape:?}")),
        _ => Ok(()),
    }
}

/// 🗃️ A selection target set names at least one id and no id twice.
pub fn puzzle2d_targets_invariant(targets: &PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>) -> Result<(), String> {
    if targets.is_empty() {
        return Err("targets must name at least one id".to_string());
    }
    match targets.iter().enumerate().find(|(at, id)| targets.iter().take(*at).any(|previous| previous == *id)) {
        Some((_, id)) => Err(format!("targets must not repeat {id:?}")),
        None => Ok(()),
    }
}

/// 🔘️ A handle record's bounds: a finite angle, a positive radius and scale when present.
pub fn puzzle2d_handle_invariant(handle: &crate::standards::v1::subsets::any::schema::records::Puzzle2dHandle) -> Result<(), String> {
    puzzle2d_finite(&[("handle angle", handle.angle)]).and_then(|()| puzzle2d_positive(&[("handle radius", handle.radius), ("handle scale", handle.scale)]))
}

/// 🔵️ A node record's bounds: a finite position, a known shape, positive extents and scale, valid handles.
pub fn puzzle2d_node_invariant(node: &crate::standards::v1::subsets::any::schema::records::Puzzle2dNode) -> Result<(), String> {
    puzzle2d_finite(&[("node x", node.x), ("node y", node.y)])
        .and_then(|()| puzzle2d_shape("node shape", node.shape.as_ref()))
        .and_then(|()| puzzle2d_positive(&[("node radius", node.radius), ("node width", node.width), ("node height", node.height), ("node scale", node.scale)]))
        .and_then(|()| node.handles.iter().try_for_each(puzzle2d_handle_invariant))
}

/// 📐️ A target region's bounds: a finite corner and extent. A negative or zero extent is a brush stroke
/// drawn from any corner, which the schema admits on purpose.
pub fn puzzle2d_region_invariant(region: &crate::standards::v1::subsets::any::schema::records::Puzzle2dTargetRegion) -> Result<(), String> {
    puzzle2d_finite(&[("region x", region.x), ("region y", region.y), ("region width", region.width), ("region height", region.height)])
}

/// 📚️ Kind catalogue bounds: a handle template's finite angle, positive radius and rim parameter `t` within
/// `0..=1`, a non-negative author rank and a non-negative handle-kind order.
pub fn puzzle2d_catalogs_invariant(catalogs: &crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs) -> Result<(), String> {
    for kind in &catalogs.nodes {
        for template in &kind.handles {
            puzzle2d_finite(&[("template angle", template.angle)])?;
            puzzle2d_positive(&[("template radius", template.radius)])?;
            if template.t.is_some_and(|t| !(0.0..=1.0).contains(&t)) {
                return Err(format!("template {:?} rim parameter t must lie within 0..=1", template.id));
            }
        }
        if let Some(author) = kind.authors.iter().find(|author| author.rank.is_some_and(|rank| rank < 0)) {
            return Err(format!("author {:?} rank must not be negative", author.id));
        }
    }
    match catalogs.handles.iter().find(|kind| kind.order.is_some_and(|order| order < 0)) {
        Some(kind) => Err(format!("handle kind {:?} order must not be negative", kind.id)),
        None => Ok(()),
    }
}
