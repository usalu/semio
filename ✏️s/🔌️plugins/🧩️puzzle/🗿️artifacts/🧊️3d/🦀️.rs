//! 🧩️ Puzzle 3d artifact — the `puzzle.3d` document schema: the `Puzzle3dSnapshot`
//! (schema/domain/meta/objects/attractions/targetVolumes/references), its object/vortex/attraction/
//! target-volume/reference/kind-catalog records, the `Puzzle3dScale` scalar-or-triple pose scale, the
//! shared `Puzzle3dError`, and the `artifact_kind()` spec the play app's manifest binds. Sibling
//! nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

#[cfg(feature = "component-app-assembly")]
pub use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;

#[cfg(feature = "component-app-assembly")]
#[path = "../../🎮️commands/🧵️retained/🦀️.rs"]
pub mod retained_command;

#[cfg(all(test, feature = "component-app-assembly"))]
fn retained_command_test_catalog() -> (&'static str, &'static str, &'static [&'static str], &'static str) {
    ("puzzle3d", "puzzle.3d.fixture", editor::puzzle3d::PUZZLE3D_RETAINED_TOOL_IDS, include_str!("🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json"))
}

//#region ⚠️ Errors
/// 🧯️ Puzzle 3d precompute session errors — JSON (de)serialization and brush/fill session state
/// failures. `Json` wraps `dsl::ValueError` (not `serde_json::Error`) since every production
/// (de)serialization call site now routes text through `dsl::os_pack::json::{from_json_str,
/// to_json_string}`, which are `ToValue`/`FromValue`-based, not `serde`-based.
#[derive(Debug)]
pub enum Puzzle3dError {
    Json(dsl::ValueError),
    BrushPlacementRejected,
    FillSessionUnavailable,
}

impl std::fmt::Display for Puzzle3dError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::BrushPlacementRejected => formatter.write_str("brush placement rejected"),
            Self::FillSessionUnavailable => formatter.write_str("fill session unavailable"),
        }
    }
}

impl std::error::Error for Puzzle3dError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            Self::BrushPlacementRejected | Self::FillSessionUnavailable => None,
        }
    }
}

impl From<dsl::ValueError> for Puzzle3dError {
    fn from(error: dsl::ValueError) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors

pub const PUZZLE_3D_SCHEMA: &str = "puzzle.3d";

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.puzzle.puzzle3d"` matches the artifact-kind id this subset's
/// own mutation/inference descriptors already key off (see `definition()`'s own
/// `"s.puzzle3d.schema.artifact"` row), standard `"1"` and subset `"*"` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Lives at the artifact level (not under the two
/// surfaces) so a viewer file can read it without ever importing through the sibling editor module.
pub const PUZZLE3D_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.puzzle.puzzle3d", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

//#region 📐️Scale
/// 📐️ A placed object's / target volume's freeform pose scale: either a single scalar broadcast
/// to all three axes, or an explicit per-axis `[x, y, z]` triple — the ONLY two shapes the engine
/// crate's `vec3_scale`/`volume_scale_vec` ever interpret (see that crate's implementation and its
/// `vec3_scale_variants` test), so this is a small closed union rather than genuinely heterogeneous
/// JSON. Replaces the former `serde_json::Value` passthrough with the actual shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle3dScale {
    Uniform(f64),
    Vec3([f64; 3]),
}

/// 🔗️ Wire shape stays identical to the former `serde_json::Value` passthrough (a bare number or
/// an `[x, y, z]` array) so every JSON-boundary consumer (the engine/ui wasm crates' own mirror
/// structs, which bind `scale` as `Option<serde_json::Value>` and are out of this derive's scope)
/// keeps parsing it exactly as before.
#[cfg(test)]
impl serde::Serialize for Puzzle3dScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Puzzle3dScale::Uniform(scale) => serializer.serialize_f64(*scale),
            Puzzle3dScale::Vec3(vec3) => serde::Serialize::serialize(vec3, serializer),
        }
    }
}

#[cfg(test)]
impl<'de> serde::Deserialize<'de> for Puzzle3dScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match <serde_json::Value as serde::Deserialize>::deserialize(deserializer)? {
            serde_json::Value::Number(n) => Ok(Puzzle3dScale::Uniform(n.as_f64().unwrap_or(1.0))),
            serde_json::Value::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle3dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => Err(serde::de::Error::custom(format!("expected scale to be a number or an [x, y, z] array, found {other}"))),
        }
    }
}

/// 🔁️ Hand-written (mirrors the `Serialize`/`Deserialize` pair directly above): same bare-number-or-
/// `[x, y, z]`-array wire shape.
impl dsl::ToValue for Puzzle3dScale {
    fn to_value(&self) -> dsl::DslValue {
        match self {
            Puzzle3dScale::Uniform(scale) => dsl::ToValue::to_value(scale),
            Puzzle3dScale::Vec3(vec3) => dsl::ToValue::to_value(vec3),
        }
    }
}
impl dsl::FromValue for Puzzle3dScale {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        match value {
            dsl::DslValue::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle3dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => f64::from_value(other).map(Puzzle3dScale::Uniform),
        }
    }
}

/// 🔗️ Hand `DslField` bridge for `Puzzle3dScale`: `objects`/`targetVolumes` are `#[dsl(table)]`
/// collections, so this field prints as a BARE positional table column — the unbounded
/// `Shape::Tuple(Float, None)` the sibling `puzzle_5d::Puzzle5dScale` uses (its `scale` is reached
/// through a nested keyed record field, not a bare column) is rejected there at parse time
/// (`table column 'scale' has a non-self-delimiting shape (TUPLE) and cannot be a table column`,
/// per the engine's own `validate_table_columns`), so this binds through the bracketed
/// `Shape::List(Float)` instead: `scale=[2]` (uniform) / `scale=[2 3 4]` (per-axis) — the brackets
/// make it self-delimiting regardless of item count.
impl dsl::DslField for Puzzle3dScale {
    fn shape() -> dsl::Shape {
        dsl::Shape::List(Box::new(dsl::Shape::Float))
    }
    fn to_value(&self) -> dsl::FieldValue {
        match self {
            Puzzle3dScale::Uniform(scale) => dsl::FieldValue::List(vec![dsl::FieldValue::Float(*scale)]),
            Puzzle3dScale::Vec3(vec3) => dsl::FieldValue::List(vec3.iter().map(|axis| dsl::FieldValue::Float(*axis)).collect()),
        }
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::List(items) if items.len() == 1 => match &items[0] {
                dsl::FieldValue::Float(scale) => Ok(Puzzle3dScale::Uniform(*scale)),
                other => Err(format!("expected Float, found {other:?}")),
            },
            dsl::FieldValue::List(items) if items.len() >= 3 => {
                let axis = |i: usize| match &items[i] {
                    dsl::FieldValue::Float(v) => Ok(*v),
                    other => Err(format!("expected Float, found {other:?}")),
                };
                Ok(Puzzle3dScale::Vec3([axis(0)?, axis(1)?, axis(2)?]))
            }
            other => Err(format!("expected a 1- or 3-item List, found {other:?}")),
        }
    }
}
//#endregion 📐️Scale

// #region 🔖️Document
/// ⚓️ Whether a root object keeps its stored plane (`Fixed`) or resets to default XY (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Puzzle3dObjectAnchor {
    #[default]
    Fixed,
    Derived,
}

/// 🔘️ One vortex on an object's rim — `vortex_kind` gates attraction compatibility, `position`/
/// `direction` place and orient it, `radius` sizes its brush-fill collision.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dVortex {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub direction: Option<[f64; 3]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🧱️ One placed object — `origin`/`orientation`/`scale` (a scalar-or-`[x,y,z]` `Puzzle3dScale`,
/// see that type and `vec3_scale`) pose it, `anchor` gates flatten-root plane retention, `vortices`
/// are its rim attraction ports.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dObject {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "object_kind")]
    pub object_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub anchor: Puzzle3dObjectAnchor,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🔗️ One attraction between two full vortex ids (`object_id:vortex_id`), with the eight compose
/// connection parameters (`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` plus diagram `x`/`y`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttraction {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
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
}

/// 🧊️ A persisted oriented box constraining fill placement (Volume Brush voxels or Transform-gumball
/// edited volumes). `scale` is a scalar-or-`[x,y,z]` `Puzzle3dScale` — see that type and
/// `volume_scale_vec`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🌐️ Where a reference image/media's bytes live and what kind of media it is.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_kind: Option<String>,
}

/// 🖼️ A reference plane pinned in world space at `origin`, `width_world` meters wide.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReference {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub source: Puzzle3dReferenceSource,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(unit = "m")]
    pub width_world: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
}

/// 🔗️ How specifically two vortex/cable kinds are allowed to attract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "lowercase"))]
#[value(rename_all = "lowercase")]
pub enum Puzzle3dCompatSpecificity {
    General,
    Object,
    Attraction,
    Cable,
    #[default]
    Vortex,
}

/// 🧩️ One allowed (or, unidirectional, one-way-allowed) link pair between two vortex/cable kind ids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibility {
    #[dsl(refs = "vortex_kind")]
    pub source: String,
    #[dsl(refs = "vortex_kind")]
    pub target: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub bidirectional: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub important: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub specificity: Puzzle3dCompatSpecificity,
}

/// 🏷️ One freeform attribute on a catalog object-kind (compose `Attribute` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttribute {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub key: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub value: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ One author credit on a catalog object-kind (compose `Author` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAuthor {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub email: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ One tagged representation/LOD URL on a catalog object-kind (compose `Representation` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dRepresentation {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub mime: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tags: Vec<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
}

/// 🌱️ One rim-vortex template on a `Puzzle3dCatalogObjectKind` — compose connector analogue with
/// `point`/`direction`/`t`/`mandatory`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexTemplate {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub point: [f64; 3],
    #[cfg_attr(test, serde(default = "puzzle3d_default_direction"))]
    #[value(default = "puzzle3d_default_direction")]
    #[dsl(dir)]
    pub direction: [f64; 3],
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

fn puzzle3d_default_direction() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

impl Default for Puzzle3dCatalogVortexTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), vortex_kind: None, point: [0.0, 0.0, 0.0], direction: puzzle3d_default_direction(), t: None, mandatory: None, radius: None }
    }
}

/// 🧱️ One object-kind catalog row — type-like (compose `Type` analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogObjectKind {
    #[dsl(defines = "object_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub image: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub unit: String,
    #[cfg_attr(test, serde(default, rename = "abstract"))]
    #[value(default, rename = "abstract")]
    pub is_abstract: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub base_kinds: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub representations: Vec<Puzzle3dRepresentation>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub vortices: Vec<Puzzle3dCatalogVortexTemplate>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attributes: Vec<Puzzle3dAttribute>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub authors: Vec<Puzzle3dAuthor>,
}

/// 🔘️ One vortex-kind catalog row — port-like (compose `Port` analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexKind {
    #[dsl(defines = "vortex_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub compatible_with: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub color: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "cable_kind")]
    pub default_cable_kind: String,
}

/// 🧵️ One cable-kind catalog row (mirrors `CableKindCatalog`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogCableKind {
    #[dsl(defines = "cable_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "attraction_kind")]
    pub default_attraction_kind: String,
}

/// 🔗️ One attraction-kind catalog row.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogAttractionKind {
    #[dsl(defines = "attraction_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained fixture export: object/vortex/cable/
/// attraction kind rows — see `puzzle/3d/manifest/*.manifest.json` for the same schema at the
/// manifest layer.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCatalogs {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub objects: Vec<Puzzle3dCatalogObjectKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub vortices: Vec<Puzzle3dCatalogVortexKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub cables: Vec<Puzzle3dCatalogCableKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub attractions: Vec<Puzzle3dCatalogAttractionKind>,
}

/// 🗂️ Fixture-carried metadata: the explicit link-compatibility table plus the object/vortex/cable/
/// attraction kind catalog bundle (typed — see `Puzzle3dKindCatalogs`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dMeta {
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle3dKindCatalogs>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle3dKindCompatibility>,
}

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.puzzle".into(),
        name: "3D Puzzle".into(),
        source_format: "puzzle.3d".into(),
        component_kind: "puzzle3d".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Design },
        schema: "puzzle.3d".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}

/// 🔌️ The `kit.catalog` artifact kind puzzle3d's `kit:in` port CONSUMES — kept as a spec puzzle can
/// read, but no longer registered as a declaration.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (registrar, 2026-08-18): the old doc here called the
/// duplicate declaration "harmless if a producer, e.g. block3d, declares an identical spec". It was
/// not harmless — it was the last thing blocking `🎪️demonstrator`, which bundles six plugins and
/// therefore registered `kit.catalog` twice, and the definition registry correctly rejected it.
/// An artifact kind is a SCHEMA: N declarations are N sources of truth for one contract, and the
/// first divergence between them would be undetectable. Ownership resolved to `🧱️block`, the only
/// plugin that actually PRODUCES the kind (`catalog:out`, `KIT_CATALOG_ARTIFACT_ID` across its 3d
/// and 5d artifact trees); `🧩️puzzle` and `🪵️sourcing` are consumers and reference it by `kind_id`
/// on their ports, which is all a consumer ever needed. `🗄️stdio`'s own docstrings describe
/// absorbing this kind into the shared vocabulary later — that move stays open and is not
/// pre-empted here.
pub fn kit_catalog_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "kit.catalog".into(),
        name: "Kit Catalog".into(),
        source_format: "kit.catalog".into(),
        component_kind: "kit".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        schema: "kit.catalog".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ Puzzle3d's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated off
/// the former `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, never
/// engine/app behaviour) — replaces the `ComposerEntry` half of the old `register_io()`. The
/// `"3d.puzzle"` OS-host mesh export/import bridge (`crate::editor::puzzle3d::register_mesh_io()`, moved
/// app-side by ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) has NO `ArtifactDeclaration`
/// field — it belongs to the same OS media-host 14-function family flagged on puzzle2d's
/// `declaration()` doc, a different mechanism from the nine §6 registrars this struct covers — so it
/// stays wired through `🧩️puzzle/🦀️.rs`'s own `.setup()`, not here.
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::puzzle3d::Puzzle3dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::puzzle3d::Puzzle3dViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::puzzle3d::Puzzle3dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::puzzle3d::Puzzle3dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.puzzle.puzzle3d.standard.v1", "standard", "1", &[], None),
        ("s.puzzle.puzzle3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.puzzle.puzzle3d.schema.artifact", "schema", "s.puzzle.puzzle3d", &[("schema", "s.puzzle.puzzle3d")], None),
        ("s.puzzle.puzzle3d.inference.artifact", "inference", "s.puzzle.puzzle3d.inference", &[("schema", "s.puzzle.puzzle3d.inference")], None),
        ("s.puzzle.puzzle3d.composer.native", "composer", "s.puzzle.puzzle3d@1/*", &[("dialect", "s.puzzle.puzzle3d@1/*")], None),
        ("s.puzzle.puzzle3d.composer.format-1", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
        ("s.puzzle.puzzle3d.composer.format-2", "composer", "s.stdio.ply@1.0/*", &[("dialect", "s.stdio.ply@1.0/*")], None),
        ("s.puzzle.puzzle3d.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.puzzle.puzzle3d.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.puzzle.puzzle3d.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.puzzle.puzzle3d.composer.format-6", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.puzzle.puzzle3d.composer.format-7", "composer", "s.stdio.gltf@2.0/*", &[("dialect", "s.stdio.gltf@2.0/*")], None),
        ("s.puzzle.puzzle3d.composer.format-8", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.puzzle.puzzle3d.grammar.1", "grammar", "puzzle.puzzle3d", &[("grammar", "puzzle.puzzle3d")], None),
        ("s.puzzle.puzzle3d.grammar.2", "grammar", "puzzle.puzzle3d.op", &[("grammar", "puzzle.puzzle3d.op")], None),
        ("s.puzzle.puzzle3d.grammar.3", "grammar", "puzzle.puzzle3d.diff", &[("grammar", "puzzle.puzzle3d.diff")], None),
        ("s.puzzle.puzzle3d.grammar.4", "grammar", "3d.pack", &[("grammar", "3d.pack")], None),
        ("s.puzzle.puzzle3d.grammar.5", "grammar", "3d.spr", &[("grammar", "3d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Puzzle3dPlayApp>>()` derives
        // its extension claim from `<Puzzle3dPlaySnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️mutations/🦀️.rs`, the editor's real `Snapshot` type), which is
        // `"puzzle3d-play"`, not the base `Puzzle3dSnapshot`'s `"puzzle3d"`.
        ("s.puzzle.puzzle3d.codec.document-1", "codec", "puzzle.3d.fixture:puzzle3d-play", &[("codec", "puzzle.3d.fixture"), ("codec-extension", "17:puzzle.3d.fixture:puzzle3d-play")], None),
        ("s.puzzle.puzzle3d.localization.en", "localization", "3D Puzzle", &[], Some(("en", "3D Puzzle"))),
        ("s.puzzle.puzzle3d.localization.de", "localization", "3D-Puzzle", &[], Some(("de", "3D-Puzzle"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.puzzle.puzzle3d")?);
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
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.puzzle.puzzle3d").expect("canonical puzzle3d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. This function
/// existed as a side-effecting `register_pilot_languages()` before M1 but was never called from
/// anywhere (dead code, confirmed by grep) — wiring it into `declaration()`'s `.languages(...)` is
/// this conversion's one real bug fix: puzzle3d's own grammars were never actually registered.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "puzzle.puzzle3d",
                    extension: Some("puzzle3d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle3d"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle3d.op"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("puzzle.puzzle3d.diff"),
                },
                dsl::LanguageSpec {
                    id: "3d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

pub use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle3dPlaySnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️design-parity-schema/🦀️.rs"]
mod design_parity_schema_tests;
//#endregion 🧪️Tests

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[cfg(feature = "component-app-assembly")]
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                #[cfg(feature = "component-app-assembly")]
                pub use component::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[cfg(feature = "component-app-assembly")]
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        #[cfg(feature = "component-app-assembly")]
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod flat_position {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📍️flat-position/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod flatten {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗜️flatten/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/🧪️tests/🌱️appends-object-c/🦀️.rs"]
                                    mod tests_appends_object_c;
                                }
                                #[path = "."]
                                pub mod delete_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-object/🧪️tests/🚫️removes-object-a-and-ce36fb/🦀️.rs"]
                                    mod tests_removes_object_a_and_severs_attraction;
                                }
                                #[path = "."]
                                pub mod move_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/🧪️tests/📍️moves-object-a/🦀️.rs"]
                                    mod tests_moves_object_a;
                                }
                                #[path = "."]
                                pub mod rotate_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/🧪️tests/📐️uniform-to-per-axis/🦀️.rs"]
                                    mod tests_uniform_to_per_axis;
                                }
                                #[path = "."]
                                pub mod change_object_mesh {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/🧪️tests/🕸️repoints-object-a-mesh/🦀️.rs"]
                                    mod tests_repoints_object_a_mesh;
                                }
                                #[path = "."]
                                pub mod edit_object_label {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/🧪️tests/🔤️relabels-object-a/🦀️.rs"]
                                    mod tests_relabels_object_a;
                                }
                                #[path = "."]
                                pub mod change_object_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-object-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-object-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-object-kind/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-object-kind/🧪️tests/🏷️reassigns-object-a-kind/🦀️.rs"]
                                    mod tests_reassigns_object_a_kind;
                                }
                                #[path = "."]
                                pub mod change_object_anchor {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/🧪️tests/⚓️fixed-to-derived/🦀️.rs"]
                                    mod tests_fixed_to_derived;
                                }
                                #[path = "."]
                                pub mod change_object_hidden {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-object-hidden/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-object-hidden/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-object-hidden/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-object-hidden/🧪️tests/🙈️hides-object-a/🦀️.rs"]
                                    mod tests_hides_object_a;
                                }
                                #[path = "."]
                                pub mod change_object_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/🧪️tests/🔒️locks-object-a/🦀️.rs"]
                                    mod tests_locks_object_a;
                                }
                                #[path = "."]
                                pub mod add_object_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/🧪️tests/🌀️appends-vortex-3-to-e60441/🦀️.rs"]
                                    mod tests_appends_vortex_3_to_object_b;
                                }
                                #[path = "."]
                                pub mod remove_object_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/🧪️tests/🚫️removes-vortex-2-8436d0/🦀️.rs"]
                                    mod tests_removes_vortex_2_and_severs_attraction;
                                }
                                #[path = "."]
                                pub mod replace_object_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/🧪️tests/⏸️rekind-vortex-1-is-noop/🦀️.rs"]
                                    mod tests_rekind_vortex_1_is_noop;
                                }
                                #[path = "."]
                                pub mod connect_vortices {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-vortices/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-vortices/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-vortices/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-vortices/🧪️tests/🧲️adds-second-attraction/🦀️.rs"]
                                    mod tests_adds_second_attraction;
                                }
                                #[path = "."]
                                pub mod disconnect_vortices {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/🧪️tests/🚫️removes-attraction-1/🦀️.rs"]
                                    mod tests_removes_attraction_1;
                                }
                                #[path = "."]
                                pub mod replace_attraction_geometry {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/🧪️tests/📍️repositions-43523e/🦀️.rs"]
                                    mod tests_repositions_attraction_1;
                                }
                                #[path = "."]
                                pub mod create_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🧪️tests/🧊️appends-volume-2/🦀️.rs"]
                                    mod tests_appends_volume_2;
                                }
                                #[path = "."]
                                pub mod delete_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🧪️tests/🚫️removes-volume-1/🦀️.rs"]
                                    mod tests_removes_volume_1;
                                }
                                #[path = "."]
                                pub mod move_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🧪️tests/⬆️lifts-volume-1/🦀️.rs"]
                                    mod tests_lifts_volume_1;
                                }
                                #[path = "."]
                                pub mod rotate_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🧪️tests/📏️per-axis-to-uniform/🦀️.rs"]
                                    mod tests_per_axis_to_uniform;
                                }
                                #[path = "."]
                                pub mod change_target_volume_hidden {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🧪️tests/🙈️hides-volume-1/🦀️.rs"]
                                    mod tests_hides_volume_1;
                                }
                                #[path = "."]
                                pub mod change_target_volume_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🧪️tests/🔒️locks-volume-1/🦀️.rs"]
                                    mod tests_locks_volume_1;
                                }
                                #[path = "."]
                                pub mod create_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-reference/🧪️tests/🖼️appends-reference-2/🦀️.rs"]
                                    mod tests_appends_reference_2;
                                }
                                #[path = "."]
                                pub mod delete_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/🧪️tests/🚫️removes-reference-1/🦀️.rs"]
                                    mod tests_removes_reference_1;
                                }
                                #[path = "."]
                                pub mod move_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/🧪️tests/↔️slides-reference-1/🦀️.rs"]
                                    mod tests_slides_reference_1;
                                }
                                #[path = "."]
                                pub mod resize_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/🧪️tests/↔️widens-reference-1/🦀️.rs"]
                                    mod tests_widens_reference_1;
                                }
                                #[path = "."]
                                pub mod replace_reference_source {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-source/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-source/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-source/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-source/🧪️tests/🖇️repoints-017eb5/🦀️.rs"]
                                    mod tests_repoints_reference_1_source;
                                }
                                #[path = "."]
                                pub mod change_reference_hidden {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/🧪️tests/🙈️hides-reference-1/🦀️.rs"]
                                    mod tests_hides_reference_1;
                                }
                                #[path = "."]
                                pub mod change_reference_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝️change-reference-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝️change-reference-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝️change-reference-locked/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝️change-reference-locked/🧪️tests/🔒️locks-reference-1/🦀️.rs"]
                                    mod tests_locks_reference_1;
                                }
                                #[path = "."]
                                pub mod change_domain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🧪️tests/⚙️architecture-to-engineering/🦀️.rs"]
                                    mod tests_architecture_to_engineering;
                                }
                                #[path = "."]
                                pub mod connect_kind_compatibility {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/🤝️adds-vortex-kind-664041/🦀️.rs"]
                                    mod tests_adds_vortex_kind_pair;
                                }
                                #[path = "."]
                                pub mod disconnect_kind_compatibility {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🧪️tests/🚫️removes-vortex-a24eec/🦀️.rs"]
                                    mod tests_removes_vortex_kind_pair;
                                }
                                #[path = "."]
                                pub mod replace_kind_catalogs {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🧪️tests/📇️installs-vortex-a9d291/🦀️.rs"]
                                    mod tests_installs_vortex_kind_catalog;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

                pub use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle3dSnapshot;
/// 🧩️ Cross-artifact solver inputs and outcomes used by Puzzle 2D and Puzzle 5D.
pub use crate::standards::v1::subsets::any::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
/// 📐️ Cross-artifact flattening primitives used by Puzzle 2D and Puzzle 5D.
pub use crate::standards::v1::subsets::any::schema::inferences::flatten::{flatten_objects, FlattenPlane, FlattenPose, DIAGRAM_HORIZONTAL_SCALE, DIAGRAM_RADIUS, DIAGRAM_VERTICAL_V_EXTRA};

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🦀️.rs"]
        pub mod concrete_forest;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🦀️.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs"]
        mod concrete_forest_tests;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🦀️.rs"]
        mod nakagin_capsule_tower_tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle3d {
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }

        #[path = "."]
        pub mod precompute {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️.rs"]
            pub mod brush;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs"]
            pub mod fill;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs"]
            pub mod geometry;
        }

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod window {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️accept-suggestion/🦀️.rs"]
            pub mod accept_suggestion;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️add-brush-object/🦀️.rs"]
            pub mod add_brush_object;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱️add-object-kind/🦀️.rs"]
            pub mod add_object_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-target-volume/🦀️.rs"]
            pub mod add_target_volume;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☀️apply-sun/🦀️.rs"]
            pub mod apply_sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔒️close-vortex-suggestions/🦀️.rs"]
            pub mod close_vortex_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💞️create-attraction/🦀️.rs"]
            pub mod create_attraction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-candidate/🦀️.rs"]
            pub mod cycle_candidate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💔️delete-attraction/🦀️.rs"]
            pub mod delete_attraction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪦️delete-target-volume/🦀️.rs"]
            pub mod delete_target_volume;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👯️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs"]
            pub mod engagement_abort;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️engagement-control-select/🦀️.rs"]
            pub mod engagement_control_select;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔂️engagement-repeat-last/🦀️.rs"]
            pub mod engagement_repeat_last;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📨️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs"]
            pub mod fill_build_tick;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs"]
            pub mod focus_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️hover-suggestion/🦀️.rs"]
            pub mod hover_suggestion;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs"]
            pub mod open_vortex_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-inspector/🦀️.rs"]
            pub mod patch_inspector;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs"]
            pub mod register_brush_mesh;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️relocate-target-volume/🦀️.rs"]
            pub mod relocate_target_volume;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️select-same-kind/🦀️.rs"]
            pub mod select_same_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active/🦀️.rs"]
            pub mod set_active;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤖️set-automatic/🦀️.rs"]
            pub mod set_automatic;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚧️set-brush-placement-overlap-budget/🦀️.rs"]
            pub mod set_brush_placement_overlap_budget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-chunk-size/🦀️.rs"]
            pub mod set_chunk_size;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📉️set-depth-variable/🦀️.rs"]
            pub mod set_depth_variable;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-fill-count/🦀️.rs"]
            pub mod set_fill_count;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚖️set-kind-weight/🦀️.rs"]
            pub mod set_kind_weight;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✋️set-manual/🦀️.rs"]
            pub mod set_manual;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📽️set-projection/🦀️.rs"]
            pub mod set_projection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📡️set-proximity-radius/🦀️.rs"]
            pub mod set_proximity_radius;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☑️set-selectable-kind/🦀️.rs"]
            pub mod set_selectable_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔖️set-selection-flag/🦀️.rs"]
            pub mod set_selection_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-snap-enabled/🦀️.rs"]
            pub mod set_snap_enabled;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️set-spacing/🦀️.rs"]
            pub mod set_spacing;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚩️set-target-volume-flag/🦀️.rs"]
            pub mod set_target_volume_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕹️set-transform-gumball-flag/🦀️.rs"]
            pub mod set_transform_gumball_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-visible/🦀️.rs"]
            pub mod set_visible;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️set-vortex-direction/🦀️.rs"]
            pub mod set_vortex_direction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌀️set-vortex-show/🦀️.rs"]
            pub mod set_vortex_show;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-voxel-dims/🦀️.rs"]
            pub mod set_voxel_dims;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️suggestions-tick/🦀️.rs"]
            pub mod suggestions_tick;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌍️world-relocate/🦀️.rs"]
            pub mod world_relocate;
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️settings/🦀️.rs"]
            pub mod settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🌐️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs"]
                    pub mod lod;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎥️projection/🦀️.rs"]
                    pub mod projection;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs"]
                    pub mod select;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/☀️sun/🦀️.rs"]
                    pub mod sun;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🌀️vortex/🦀️.rs"]
                    pub mod vortex;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs"]
                            pub mod brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️.rs"]
                            pub mod transform;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🧊️volume-brush/🦀️.rs"]
                            pub mod volume_brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🚚️world-relocate/🦀️.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle3d {
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
