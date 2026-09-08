//! 🧩️ Puzzle 5d artifact — the `puzzle.5d` document schema: the `Puzzle5dSnapshot` (schema/domain/
//! label/meta/kindCatalogs/kindCompatibility/parts/fasteners), its unified 2d+3d part/grip/fastener
//! records, the `Puzzle5dScale` scalar-or-triple pose scale, the `Puzzle5dError` that delegates to
//! the 3d artifact's own precompute-session error, and the `artifact_kind()` spec the play app's
//! manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`. No `⚙️engine` node —
//! per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, an artifact is a `🧬️schema` +
//! `🚪️io` system only; behaviour lives in the sibling editor module, `crate::editor::puzzle5d`.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as artifact_schema;
extern crate semio_framework_value_derive as value_derive;

//#region ⚠️ Errors
/// 🧯️ Puzzle 5d precompute session errors — delegates entirely to `puzzle_3d`'s own precompute-session error.
#[derive(Debug)]
pub enum Puzzle5dError {
    Puzzle3d(semio_s_artifact_puzzle_3d::Puzzle3dError),
}

impl std::fmt::Display for Puzzle5dError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Puzzle3d(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for Puzzle5dError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Puzzle3d(error) => std::error::Error::source(error),
        }
    }
}

impl From<semio_s_artifact_puzzle_3d::Puzzle3dError> for Puzzle5dError {
    fn from(error: semio_s_artifact_puzzle_3d::Puzzle3dError) -> Self {
        Self::Puzzle3d(error)
    }
}
//#endregion ⚠️ Errors

pub const PUZZLE_5D_SCHEMA: &str = "puzzle.5d";

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1/§7.4 — lives at the
/// ARTIFACT level (not under `editor`/`viewer`) specifically so a viewer file can read it without
/// ever importing through the sibling editor module. `artifact_kind` matches this file's own
/// `"s.puzzle5d.schema.artifact"` capability row descriptor (`s.puzzle.puzzle5d`), `standard`/
/// `subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — canonical surface id
/// `s.puzzle.puzzle5d@1/*#editor` / `s.puzzle.puzzle5d@1/*#viewer`.
pub const PUZZLE5D_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.puzzle.puzzle5d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

// #region 🔖️Document
/// 📝️ Free-text scene description — the only field seen under the fixture's top-level `meta`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dMeta {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
}

/// 🔵️ A part's 2D-projection presentation (board node): `shape`/`radius` size the circle/rectangle,
/// `text`/`icon_kind` label it.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart2d {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub y: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
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
    pub text: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

//#region 📐️Scale
/// 📏️ A part's freeform 3D scale — a bare number scales all three axes uniformly, an `[x, y, z]`
/// array scales each axis independently. Same `Uniform`-or-`Vec3` shape as
/// `puzzle_3d::Puzzle3dObject.scale`/`Puzzle3dTargetVolume.scale` (see that type's own `Scale`
/// region), applied here to the identical `part_scale_json` decode `puzzle_5d_ui` already used to
/// read the old raw `serde_json::Value` two ways.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle5dScale {
    Uniform(f64),
    Vec3([f64; 3]),
}

/// 🔗️ Wire shape stays identical to the former `serde_json::Value` passthrough (a bare number or
/// an `[x, y, z]` array) so every JSON-boundary consumer (the `puzzle_5d_ui` wasm crate's own
/// mirror struct, which binds `scale` as `Option<serde_json::Value>` and is out of this derive's
/// scope) keeps parsing it exactly as before.
#[cfg(test)]
impl serde::Serialize for Puzzle5dScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Puzzle5dScale::Uniform(scale) => serializer.serialize_f64(*scale),
            Puzzle5dScale::Vec3(vec3) => serde::Serialize::serialize(vec3, serializer),
        }
    }
}

#[cfg(test)]
impl<'de> serde::Deserialize<'de> for Puzzle5dScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match <serde_json::Value as serde::Deserialize>::deserialize(deserializer)? {
            serde_json::Value::Number(n) => Ok(Puzzle5dScale::Uniform(n.as_f64().unwrap_or(1.0))),
            serde_json::Value::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle5dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => Err(serde::de::Error::custom(format!("expected scale to be a number or an [x, y, z] array, found {other}"))),
        }
    }
}

/// 🔁️ Hand-written (mirrors the `Serialize`/`Deserialize` pair directly above): same bare-number-or-
/// `[x, y, z]`-array wire shape.
impl dsl::ToValue for Puzzle5dScale {
    fn to_value(&self) -> dsl::DslValue {
        match self {
            Puzzle5dScale::Uniform(scale) => dsl::ToValue::to_value(scale),
            Puzzle5dScale::Vec3(vec3) => dsl::ToValue::to_value(vec3),
        }
    }
}
impl dsl::FromValue for Puzzle5dScale {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        match value {
            dsl::DslValue::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle5dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => f64::from_value(other).map(Puzzle5dScale::Uniform),
        }
    }
}

/// 🔗️ Hand `DslField` bridge for `Puzzle5dScale`: binds through the existing unbounded
/// `Shape::Tuple(Float, None)` primitive (the same one `#[dsl(tuple)] Vec<f64>` fields already use
/// elsewhere) rather than `Shape::Value` — `scale=2` (uniform) and `scale=2,3,4` (per-axis) print/
/// parse as plain packed literals, no bespoke Shape variant needed.
impl dsl::DslField for Puzzle5dScale {
    fn shape() -> dsl::Shape {
        dsl::Shape::Tuple(Box::new(dsl::Shape::Float), None)
    }
    fn to_value(&self) -> dsl::FieldValue {
        match self {
            Puzzle5dScale::Uniform(scale) => dsl::FieldValue::Tuple(vec![dsl::FieldValue::Float(*scale)]),
            Puzzle5dScale::Vec3(vec3) => dsl::FieldValue::Tuple(vec3.iter().map(|axis| dsl::FieldValue::Float(*axis)).collect()),
        }
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Tuple(items) if items.len() == 1 => match &items[0] {
                dsl::FieldValue::Float(scale) => Ok(Puzzle5dScale::Uniform(*scale)),
                other => Err(format!("expected Float, found {other:?}")),
            },
            dsl::FieldValue::Tuple(items) if items.len() >= 3 => {
                let axis = |i: usize| match &items[i] {
                    dsl::FieldValue::Float(v) => Ok(*v),
                    other => Err(format!("expected Float, found {other:?}")),
                };
                Ok(Puzzle5dScale::Vec3([axis(0)?, axis(1)?, axis(2)?]))
            }
            other => Err(format!("expected a 1- or 3-item Tuple, found {other:?}")),
        }
    }
}
//#endregion 📐️Scale

/// 🧱️ A part's 3D-projection presentation (world object): `origin`/`orientation` pose it, `mesh_url`
/// resolves its geometry, `scale` is `Puzzle5dScale` (bare number = uniform, `x,y,z` = per-axis).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart3d {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle5dScale>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🔘️ A grip's 2D-projection presentation (board handle) — `grip_kind` is duplicated here from
/// `Puzzle5dGrip::grip_kind` in real fixtures (a per-projection override slot), not simplified away.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip2d {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

/// 🔘️ A grip's 3D-projection presentation (world vortex).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip3d {
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
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🔘️ One rim grip on a part, unified across both projections.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[cfg_attr(test, serde(default, rename = "2d"))]
    #[value(default, rename = "2d")]
    pub grip_2d: Puzzle5dGrip2d,
    #[cfg_attr(test, serde(default, rename = "3d"))]
    #[value(default, rename = "3d")]
    pub grip_3d: Puzzle5dGrip3d,
}

/// ⚓️ Whether a part keeps its stored plane at a BFS root (`Fixed`) or resets the plane to default XY (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Puzzle5dPartAnchor {
    #[default]
    Fixed,
    Derived,
}

impl Puzzle5dPartAnchor {
    fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed)
    }
}

/// 🧱️ One placed part, unified across both projections — `grips` are its rim attraction/link ports.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "part_kind")]
    pub part_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Puzzle5dPartAnchor::is_fixed"))]
    #[value(default, skip_serializing_if = "Puzzle5dPartAnchor::is_fixed")]
    pub anchor: Puzzle5dPartAnchor,
    #[cfg_attr(test, serde(default, rename = "2d"))]
    #[value(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[cfg_attr(test, serde(default, rename = "3d"))]
    #[value(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

impl Default for Puzzle5dPart {
    fn default() -> Self {
        Self { id: String::new(), part_kind: None, anchor: Puzzle5dPartAnchor::Fixed, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() }
    }
}

/// 🔗️ One fastener (2D edge / 3D attraction) between two full grip ids (`part_id:grip_id`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dFastener {
    pub id: String,
    pub source: String,
    pub target: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "fastener_kind")]
    pub fastener_kind: Option<String>,
    /// 🔧️ Pose-solver offsets `compute_brush_placement_pose` / compose `geom::flatten` resolve into a world pose —
    /// mirrors `puzzle_3d::Puzzle3dAttraction` plus diagram offsets `x`/`y` (compose Connection `u`/`v`).
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

/// 🔗️ How specifically two grip/rope kinds are allowed to fasten.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "lowercase"))]
#[value(rename_all = "lowercase")]
pub enum Puzzle5dCompatSpecificity {
    #[default]
    General,
    Part,
    Fastener,
    Grip,
    Rope,
}

/// 🔗️ One allowed (or unidirectional) kind pair — unified with 2d/3d via `important` + `specificity`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCompatibility {
    #[dsl(refs = "grip_kind")]
    pub source: String,
    #[dsl(refs = "grip_kind")]
    pub target: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub bidirectional: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub important: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub specificity: Puzzle5dCompatSpecificity,
}

/// 🏷️ One freeform attribute on a part-kind (compose `Attribute` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dAttribute {
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

/// ✍️ One author credit on a part-kind (compose `Author` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dAuthor {
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

/// 🖼️ One tagged representation (mesh/image/…) on a part-kind.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dRepresentation {
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

fn default_grip_direction() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

/// 🌱️ One rim-grip template on a `Puzzle5dCatalogPartKind` (compose Connector analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGripTemplate {
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
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub point: [f64; 3],
    #[cfg_attr(test, serde(default = "default_grip_direction"))]
    #[value(default = "default_grip_direction")]
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

impl Default for Puzzle5dGripTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), grip_kind: None, point: [0.0, 0.0, 0.0], direction: default_grip_direction(), t: None, mandatory: None, radius: None }
    }
}

/// 🧱️ One part-kind catalog row (compose Type analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogPartKind {
    #[dsl(defines = "part_kind")]
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
    pub representations: Vec<Puzzle5dRepresentation>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub grips: Vec<Puzzle5dGripTemplate>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attributes: Vec<Puzzle5dAttribute>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub authors: Vec<Puzzle5dAuthor>,
}

/// 🔘️ One grip-kind catalog row (compose Port analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripKind {
    #[dsl(defines = "grip_kind")]
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
    #[dsl(refs = "rope_kind")]
    pub default_rope_kind: String,
}

/// 🔗️ One fastener-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogFastenerKind {
    #[dsl(defines = "fastener_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🧵️ One rope-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogRopeKind {
    #[dsl(defines = "rope_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "fastener_kind")]
    pub default_fastener_kind: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained fixture export: part/grip/fastener/rope
/// kind rows — see `puzzle/5d/manifest/*.manifest.json` for the same schema at the manifest layer.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCatalogs {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub parts: Vec<Puzzle5dCatalogPartKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub grips: Vec<Puzzle5dCatalogGripKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub fasteners: Vec<Puzzle5dCatalogFastenerKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub ropes: Vec<Puzzle5dCatalogRopeKind>,
}

/// 🏷️ Temporary name alias until Wave 3 app callers migrate to [`Puzzle5dCatalogPartKind`].
pub type Puzzle5dCatalogPart = Puzzle5dCatalogPartKind;
/// 🏷️ Temporary name alias until Wave 3 app callers migrate to [`Puzzle5dCatalogGripKind`].
pub type Puzzle5dCatalogGrip = Puzzle5dCatalogGripKind;

//#region 🔖️KindCatalogComposition
// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: `Puzzle5dSnapshot.kind_catalogs`
// duplicated stdio's `s.stdio.semio.kit` type-registry vocabulary (`SemioKitType { id, name,
// category }`) four times over (part/grip/fastener/rope kind catalogs), each row far richer than
// `SemioKitType` can represent. Same split-and-compose pattern `sourcing`'s wave-4 migration used for
// its own `stock: Vec<ObjectKind>` (see `../../../🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs`'s
// `🔖️CatalogComposition` region, this migration's primary precedent): every kind-catalog row splits
// into a shared `SemioKitType` half (id/name/category) plus a puzzle5d-owned `*Extra` half carrying
// everything `SemioKitType` cannot, id-joined back together by `kind_catalogs_of`.
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};

/// 🧩️ Puzzle5d-owned overflow for one part-kind catalog row — everything `SemioKitType` cannot
/// represent. Id-joined 1:1 to a `SemioKitType` (`category = "part"`) in the composed
/// `Puzzle5dSnapshot::kind_catalogs` child.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogPartKindExtra {
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
    pub representations: Vec<Puzzle5dRepresentation>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub grips: Vec<Puzzle5dGripTemplate>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attributes: Vec<Puzzle5dAttribute>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub authors: Vec<Puzzle5dAuthor>,
}

/// 🧩️ Puzzle5d-owned overflow for one grip-kind catalog row (`category = "grip"`). `SemioKitType` has
/// no `name` slot this row ever populated (`Puzzle5dCatalogGripKind` never carried one either) — the
/// composed `SemioKitType.name` is a display-only derivation (`label` else `code`), never round-
/// tripped back into `code`/`label` themselves, both of which live here unchanged.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripKindExtra {
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
    #[dsl(refs = "rope_kind")]
    pub default_rope_kind: String,
}

/// 🧩️ Puzzle5d-owned overflow for one fastener-kind catalog row (`category = "fastener"`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogFastenerKindExtra {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🧩️ Puzzle5d-owned overflow for one rope-kind catalog row (`category = "rope"`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogRopeKindExtra {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "fastener_kind")]
    pub default_fastener_kind: String,
}

/// 🗂️ The puzzle5d-owned overflow half of `Puzzle5dKindCatalogs`, sibling to the composed
/// `kind_catalogs` child — see the region doc for the split/join contract.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCatalogsExtra {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub parts: Vec<Puzzle5dCatalogPartKindExtra>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub grips: Vec<Puzzle5dCatalogGripKindExtra>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub fasteners: Vec<Puzzle5dCatalogFastenerKindExtra>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub ropes: Vec<Puzzle5dCatalogRopeKindExtra>,
}

//#region 🔖️RowConverters
pub fn kit_type_from_part_kind(k: &Puzzle5dCatalogPartKind) -> SemioKitType {
    SemioKitType { id: k.id.clone(), name: k.name.clone(), category: "part".into() }
}
pub fn part_kind_extra_from_part_kind(k: &Puzzle5dCatalogPartKind) -> Puzzle5dCatalogPartKindExtra {
    Puzzle5dCatalogPartKindExtra {
        id: k.id.clone(),
        name: k.name.clone(),
        label: k.label.clone(),
        description: k.description.clone(),
        icon: k.icon.clone(),
        image: k.image.clone(),
        unit: k.unit.clone(),
        is_abstract: k.is_abstract,
        base_kinds: k.base_kinds.clone(),
        representations: k.representations.clone(),
        grips: k.grips.clone(),
        attributes: k.attributes.clone(),
        authors: k.authors.clone(),
    }
}
pub fn part_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogPartKindExtra) -> Puzzle5dCatalogPartKind {
    Puzzle5dCatalogPartKind {
        id: kit_type.id.clone(),
        name: kit_type.name.clone(),
        label: extra.label.clone(),
        description: extra.description.clone(),
        icon: extra.icon.clone(),
        image: extra.image.clone(),
        unit: extra.unit.clone(),
        is_abstract: extra.is_abstract,
        base_kinds: extra.base_kinds.clone(),
        representations: extra.representations.clone(),
        grips: extra.grips.clone(),
        attributes: extra.attributes.clone(),
        authors: extra.authors.clone(),
    }
}

pub fn kit_type_from_grip_kind(k: &Puzzle5dCatalogGripKind) -> SemioKitType {
    let name = k.label.clone().or_else(|| k.code.clone()).unwrap_or_default();
    SemioKitType { id: k.id.clone(), name, category: "grip".into() }
}
pub fn grip_kind_extra_from_grip_kind(k: &Puzzle5dCatalogGripKind) -> Puzzle5dCatalogGripKindExtra {
    Puzzle5dCatalogGripKindExtra {
        id: k.id.clone(),
        code: k.code.clone(),
        label: k.label.clone(),
        order: k.order,
        compatible_with: k.compatible_with.clone(),
        description: k.description.clone(),
        icon: k.icon.clone(),
        color: k.color.clone(),
        default_rope_kind: k.default_rope_kind.clone(),
    }
}
pub fn grip_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogGripKindExtra) -> Puzzle5dCatalogGripKind {
    let _ = kit_type;
    Puzzle5dCatalogGripKind {
        id: extra.id.clone(),
        code: extra.code.clone(),
        label: extra.label.clone(),
        order: extra.order,
        compatible_with: extra.compatible_with.clone(),
        description: extra.description.clone(),
        icon: extra.icon.clone(),
        color: extra.color.clone(),
        default_rope_kind: extra.default_rope_kind.clone(),
    }
}

pub fn kit_type_from_fastener_kind(k: &Puzzle5dCatalogFastenerKind) -> SemioKitType {
    SemioKitType { id: k.id.clone(), name: k.name.clone(), category: "fastener".into() }
}
pub fn fastener_kind_extra_from_fastener_kind(k: &Puzzle5dCatalogFastenerKind) -> Puzzle5dCatalogFastenerKindExtra {
    Puzzle5dCatalogFastenerKindExtra { id: k.id.clone(), name: k.name.clone(), label: k.label.clone() }
}
pub fn fastener_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogFastenerKindExtra) -> Puzzle5dCatalogFastenerKind {
    Puzzle5dCatalogFastenerKind { id: kit_type.id.clone(), name: kit_type.name.clone(), label: extra.label.clone() }
}

pub fn kit_type_from_rope_kind(k: &Puzzle5dCatalogRopeKind) -> SemioKitType {
    SemioKitType { id: k.id.clone(), name: k.name.clone(), category: "rope".into() }
}
pub fn rope_kind_extra_from_rope_kind(k: &Puzzle5dCatalogRopeKind) -> Puzzle5dCatalogRopeKindExtra {
    Puzzle5dCatalogRopeKindExtra { id: k.id.clone(), name: k.name.clone(), label: k.label.clone(), default_fastener_kind: k.default_fastener_kind.clone() }
}
pub fn rope_kind_from_parts(kit_type: &SemioKitType, extra: &Puzzle5dCatalogRopeKindExtra) -> Puzzle5dCatalogRopeKind {
    Puzzle5dCatalogRopeKind { id: kit_type.id.clone(), name: kit_type.name.clone(), label: extra.label.clone(), default_fastener_kind: extra.default_fastener_kind.clone() }
}
//#endregion 🔖️RowConverters

//#region 🔖️WholeListConverters
/// 🔀️ `Puzzle5dKindCatalogs` → the shared `SemioKitType` half of the composed catalog child (all
/// four kind lists flattened into one `category`-tagged list, matching `SemioKitSnapshot.types`'s
/// own id-keyed, category-differentiated shape).
pub fn kind_catalogs_kit_types(catalogs: &Puzzle5dKindCatalogs) -> Vec<SemioKitType> {
    catalogs.parts.iter().map(kit_type_from_part_kind).chain(catalogs.grips.iter().map(kit_type_from_grip_kind)).chain(catalogs.fasteners.iter().map(kit_type_from_fastener_kind)).chain(catalogs.ropes.iter().map(kit_type_from_rope_kind)).collect()
}

/// 🔀️ `Puzzle5dKindCatalogs` → the puzzle5d-owned overflow half. Lossless together with
/// `kind_catalogs_kit_types`: every field of every row lands in exactly one of the two halves.
pub fn kind_catalogs_extra_from_kind_catalogs(catalogs: &Puzzle5dKindCatalogs) -> Puzzle5dKindCatalogsExtra {
    Puzzle5dKindCatalogsExtra {
        parts: catalogs.parts.iter().map(part_kind_extra_from_part_kind).collect(),
        grips: catalogs.grips.iter().map(grip_kind_extra_from_grip_kind).collect(),
        fasteners: catalogs.fasteners.iter().map(fastener_kind_extra_from_fastener_kind).collect(),
        ropes: catalogs.ropes.iter().map(rope_kind_extra_from_rope_kind).collect(),
    }
}

/// 🔀️ Inverse of the split above — reassembles a full `Puzzle5dKindCatalogs` from its composed-child
/// half (a flat, category-tagged `SemioKitType` list) and its puzzle5d-owned overflow half, id-joined
/// per category. A `SemioKitType` with no matching `*Extra` row (composed-child content the working-
/// scene cache hasn't seen yet — see `kind_catalogs_of`'s doc comment) is silently dropped rather than
/// fabricated with placeholder fields.
pub fn kind_catalogs_from_kit_types_and_extra(types: &[SemioKitType], extra: &Puzzle5dKindCatalogsExtra) -> Puzzle5dKindCatalogs {
    let by_category = |category: &str| -> std::collections::HashMap<&str, &SemioKitType> { types.iter().filter(|t| t.category == category).map(|t| (t.id.as_str(), t)).collect() };
    let part_types = by_category("part");
    let grip_types = by_category("grip");
    let fastener_types = by_category("fastener");
    let rope_types = by_category("rope");
    Puzzle5dKindCatalogs {
        parts: extra.parts.iter().filter_map(|e| part_types.get(e.id.as_str()).map(|t| part_kind_from_parts(t, e))).collect(),
        grips: extra.grips.iter().filter_map(|e| grip_types.get(e.id.as_str()).map(|t| grip_kind_from_parts(t, e))).collect(),
        fasteners: extra.fasteners.iter().filter_map(|e| fastener_types.get(e.id.as_str()).map(|t| fastener_kind_from_parts(t, e))).collect(),
        ropes: extra.ropes.iter().filter_map(|e| rope_types.get(e.id.as_str()).map(|t| rope_kind_from_parts(t, e))).collect(),
    }
}

/// 🔀️ The full kind-catalogs' shared half, as a fresh (design-less, link-less, object/model/
/// properties-less) `SemioKitSnapshot` — content-addressed by `kind_catalogs_child_handle` below,
/// never embedded inline in `Puzzle5dSnapshot`.
pub fn kind_catalogs_kit_snapshot(catalogs: &Puzzle5dKindCatalogs) -> SemioKitSnapshot {
    SemioKitSnapshot { types: kind_catalogs_kit_types(catalogs), ..SemioKitSnapshot::default() }
}

/// 🪪️ Content-addressed child handle for a kind-catalogs bundle — hashes the deterministic JSON of
/// the derived `SemioKitType` list so peers replaying the same catalogs converge on the same
/// `child_id` (never a random/incrementing id), mirroring `sourcing`'s `catalog_child_handle`.
pub fn kind_catalogs_child_handle(catalogs: &Puzzle5dKindCatalogs) -> store::ArtifactChild<SemioKitSnapshot> {
    use std::hash::{Hash, Hasher};
    let types = kind_catalogs_kit_types(catalogs);
    let canonical = dsl::json::to_json_string(&types);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    canonical.hash(&mut hasher);
    let child_id = format!("kind-catalogs-{:016x}", hasher.finish());
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️WholeListConverters

/// 🌱 Seeds the working-scene cache for `catalogs`' deterministic `kind_catalogs_child_handle`,
/// without building a whole `Puzzle5dSnapshot` — for fixture loaders that parse the persisted
/// snapshot from DSL text (which never embeds child content) but still need the SAME content-
/// addressed handle's catalogs resolvable immediately after loading.
pub fn validate_kind_catalogs_payload(catalogs: &Puzzle5dKindCatalogs) {
    let _ = kind_catalogs_child_handle(catalogs);
}

/// 🏗️ Mints a fresh content-addressed handle for `catalogs`, splits it into its composed-child half
/// and puzzle5d-owned overflow half, and seeds the working-scene cache so this SAME call's render/
/// export/inference/mutation paths can resolve the handle immediately. Returns the handle plus the
/// overflow half — the two fields a `Puzzle5dSnapshot`/`Puzzle5dArtifact`/`Puzzle5dDiff` now carry in
/// place of the old inline `Puzzle5dKindCatalogs` field.
pub fn split_and_seed_kind_catalogs(catalogs: Option<Puzzle5dKindCatalogs>) -> (Option<store::ArtifactChild<SemioKitSnapshot>>, Option<Puzzle5dKindCatalogsExtra>) {
    match catalogs {
        None => (None, None),
        Some(catalogs) => {
            let handle = kind_catalogs_child_handle(&catalogs);
            (Some(handle), Some(kind_catalogs_extra_from_kind_catalogs(&catalogs)))
        }
    }
}

/// 👁️ The one accessor every render/export/inference/mutation call site funnels through to read the
/// full reassembled kind-catalogs bundle back in its original `Puzzle5dKindCatalogs` shape.
pub fn kind_catalogs_of(handle: &Option<store::ArtifactChild<SemioKitSnapshot>>, extra: &Option<Puzzle5dKindCatalogsExtra>) -> Option<Puzzle5dKindCatalogs> {
    let _ = handle.as_ref()?;
    let extra = extra.clone().unwrap_or_default();
    Some(Puzzle5dKindCatalogs {
        parts: extra
            .parts
            .iter()
            .map(|row| Puzzle5dCatalogPartKind {
                id: row.id.clone(),
                name: row.name.clone(),
                label: row.label.clone(),
                description: row.description.clone(),
                icon: row.icon.clone(),
                image: row.image.clone(),
                unit: row.unit.clone(),
                is_abstract: row.is_abstract,
                base_kinds: row.base_kinds.clone(),
                representations: row.representations.clone(),
                grips: row.grips.clone(),
                attributes: row.attributes.clone(),
                authors: row.authors.clone(),
            })
            .collect(),
        grips: extra
            .grips
            .iter()
            .map(|row| Puzzle5dCatalogGripKind {
                id: row.id.clone(),
                code: row.code.clone(),
                label: row.label.clone(),
                order: row.order,
                compatible_with: row.compatible_with.clone(),
                description: row.description.clone(),
                icon: row.icon.clone(),
                color: row.color.clone(),
                default_rope_kind: row.default_rope_kind.clone(),
            })
            .collect(),
        fasteners: extra.fasteners.iter().map(|row| Puzzle5dCatalogFastenerKind { id: row.id.clone(), name: row.name.clone(), label: row.label.clone() }).collect(),
        ropes: extra.ropes.iter().map(|row| Puzzle5dCatalogRopeKind { id: row.id.clone(), name: row.name.clone(), label: row.label.clone(), default_fastener_kind: row.default_fastener_kind.clone() }).collect(),
    })
}
//#endregion 🔖️KindCatalogComposition

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

//#region 🔖️ArtifactKind
/// 🗿️ The `5d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "5d.puzzle".into(),
        name: "5D Puzzle".into(),
        source_format: "puzzle.5d".into(),
        component_kind: "puzzle5d".into(),
        dimension: "5d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Design },
        schema: "puzzle.5d".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.stl".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.stl".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ Puzzle5d's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated off
/// the former `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not
/// engine behaviour) — replaces the `ComposerEntry` half of the old `register_io()`. The `"5d.puzzle"`
/// OS-host mesh export/import bridge (`register_mesh_io()`, now `crate::editor::puzzle5d::register_mesh_io`
/// per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) has NO `ArtifactDeclaration`
/// field — same OS media-host 14-function family flagged on puzzle2d's `declaration()` doc — so it
/// stays wired through `🧩️puzzle/🦀️.rs`'s own `.setup()`, not here.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.puzzle.puzzle5d.standard.v1", "standard", "1", &[], None),
        ("s.puzzle.puzzle5d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.puzzle.puzzle5d.schema.artifact", "schema", "s.puzzle.puzzle5d", &[("schema", "s.puzzle.puzzle5d")], None),
        ("s.puzzle.puzzle5d.inference.artifact", "inference", "s.puzzle.puzzle5d.inference", &[("schema", "s.puzzle.puzzle5d.inference")], None),
        ("s.puzzle.puzzle5d.composer.native", "composer", "s.puzzle.puzzle5d@1/*", &[("dialect", "s.puzzle.puzzle5d@1/*")], None),
        ("s.puzzle.puzzle5d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.puzzle.puzzle5d.composer.format-2", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.puzzle.puzzle5d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.puzzle.puzzle5d.composer.format-4", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.puzzle.puzzle5d.composer.format-5", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.puzzle.puzzle5d.grammar.1", "grammar", "puzzle.puzzle5d", &[("grammar", "puzzle.puzzle5d")], None),
        ("s.puzzle.puzzle5d.grammar.2", "grammar", "puzzle.puzzle5d.op", &[("grammar", "puzzle.puzzle5d.op")], None),
        ("s.puzzle.puzzle5d.grammar.3", "grammar", "puzzle.puzzle5d.diff", &[("grammar", "puzzle.puzzle5d.diff")], None),
        ("s.puzzle.puzzle5d.grammar.4", "grammar", "5d.pack", &[("grammar", "5d.pack")], None),
        ("s.puzzle.puzzle5d.grammar.5", "grammar", "5d.spr", &[("grammar", "5d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Puzzle5dPlayApp>>()` derives
        // its extension claim from `<Puzzle5dPlaySnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️mutations/🦀️.rs`, the editor's real `Snapshot` type), which is
        // `"puzzle5d-play"`, not the base `Puzzle5dSnapshot`'s `"puzzle5d"`.
        ("s.puzzle.puzzle5d.codec.document-1", "codec", "puzzle.5d:puzzle5d-play", &[("codec", "puzzle.5d"), ("codec-extension", "9:puzzle.5d:puzzle5d-play")], None),
        ("s.puzzle.puzzle5d.localization.en", "localization", "5D Puzzle", &[], Some(("en", "5D Puzzle"))),
        ("s.puzzle.puzzle5d.localization.de", "localization", "5D-Puzzle", &[], Some(("de", "5D-Puzzle"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.puzzle.puzzle5d")?);
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
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::PuzzleApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.puzzle.puzzle5d").expect("canonical puzzle5d kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. This function
/// existed as a side-effecting `register_pilot_languages()` before M1 but was never called from
/// anywhere (dead code, confirmed by grep) — wiring it into `declaration()`'s `.languages(...)` is
/// this conversion's one real bug fix: puzzle5d's own grammars were never actually registered.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "puzzle.puzzle5d",
                    extension: Some("puzzle5d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle5d"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle5d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle5d.op"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle5d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("puzzle.puzzle5d.diff"),
                },
                dsl::LanguageSpec {
                    id: "5d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("5d.pack"),
                },
                dsl::LanguageSpec {
                    id: "5d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("5d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

pub use crate::op::Puzzle5dPlaySnapshot;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fastener_defaults_include_diagram_xy() {
        let fastener: Puzzle5dFastener = serde_json::from_value(serde_json::json!({
            "id": "f1",
            "source": "p1:g0",
            "target": "p2:g0"
        }))
        .unwrap();
        assert_eq!(fastener.gap, 0.0);
        assert_eq!(fastener.x, 0.0);
        assert_eq!(fastener.y, 0.0);
        assert_eq!(fastener.rotation, 0.0);
    }

    #[test]
    fn fastener_round_trips_eight_transform_params() {
        let fastener = Puzzle5dFastener { id: "f1".into(), source: "p1:g0".into(), target: "p2:g0".into(), fastener_kind: Some("fk".into()), gap: 1.0, shift: 2.0, rise: 3.0, rotation: 4.0, turn: 5.0, tilt: 6.0, x: 7.0, y: 8.0 };
        let value = serde_json::to_value(&fastener).unwrap();
        assert_eq!(value["x"], 7.0);
        assert_eq!(value["y"], 8.0);
        let back: Puzzle5dFastener = serde_json::from_value(value).unwrap();
        assert_eq!(back, fastener);
    }

    #[test]
    fn part_anchor_defaults_to_fixed() {
        let part: Puzzle5dPart = serde_json::from_value(serde_json::json!({ "id": "p1" })).unwrap();
        assert_eq!(part.anchor, Puzzle5dPartAnchor::Fixed);
        let derived: Puzzle5dPart = serde_json::from_value(serde_json::json!({ "id": "p2", "anchor": "derived" })).unwrap();
        assert_eq!(derived.anchor, Puzzle5dPartAnchor::Derived);
    }

    #[test]
    fn kind_compatibility_unifies_important_and_specificity() {
        let row: Puzzle5dKindCompatibility = serde_json::from_value(serde_json::json!({
            "source": "a",
            "target": "b",
            "bidirectional": true,
            "important": true,
            "specificity": "grip"
        }))
        .unwrap();
        assert!(row.important);
        assert_eq!(row.specificity, Puzzle5dCompatSpecificity::Grip);
        let sparse: Puzzle5dKindCompatibility = serde_json::from_value(serde_json::json!({
            "source": "a",
            "target": "b"
        }))
        .unwrap();
        assert!(!sparse.important);
        assert_eq!(sparse.specificity, Puzzle5dCompatSpecificity::General);
    }

    #[test]
    fn catalog_part_kind_carries_representations_and_grip_templates() {
        let kind = Puzzle5dCatalogPartKind {
            id: "hex".into(),
            name: "Hex".into(),
            label: "Hex".into(),
            description: "cut".into(),
            icon: "hexagon".into(),
            image: "".into(),
            unit: "m".into(),
            is_abstract: false,
            base_kinds: vec!["solid".into()],
            representations: vec![Puzzle5dRepresentation { id: "lod0".into(), name: "mesh".into(), url: "/mesh/hex.glb".into(), mime: "model/gltf-binary".into(), tags: vec!["mesh".into()], lod: Some("0".into()), description: "".into() }],
            grips: vec![Puzzle5dGripTemplate {
                id: "g0".into(),
                name: "north".into(),
                label: "N".into(),
                grip_kind: Some("b-l".into()),
                point: [1.0, 2.0, 3.0],
                direction: [0.0, 1.0, 0.0],
                t: Some(0.25),
                mandatory: Some(true),
                radius: Some(0.36),
                ..Default::default()
            }],
            attributes: vec![Puzzle5dAttribute { id: "a1".into(), key: "material".into(), value: "concrete".into(), definition: None }],
            authors: vec![Puzzle5dAuthor { id: "u1".into(), name: "Ada".into(), email: "ada@semio.tech".into(), role: Some("author".into()), rank: Some(1) }],
        };
        let value = serde_json::to_value(&kind).unwrap();
        assert_eq!(value["abstract"], false);
        assert_eq!(value["representations"][0]["url"], "/mesh/hex.glb");
        assert_eq!(value["grips"][0]["point"], serde_json::json!([1.0, 2.0, 3.0]));
        let back: Puzzle5dCatalogPartKind = serde_json::from_value(value).unwrap();
        assert_eq!(back.grips[0].direction, [0.0, 1.0, 0.0]);
        assert_eq!(back.authors[0].name, "Ada");
    }

    #[test]
    fn grip_template_direction_defaults_to_positive_z() {
        let template: Puzzle5dGripTemplate = serde_json::from_value(serde_json::json!({ "id": "g0" })).unwrap();
        assert_eq!(template.direction, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn catalog_grip_kind_is_port_like() {
        let kind = Puzzle5dCatalogGripKind {
            id: "b-l".into(),
            code: Some("BL".into()),
            label: Some("Long".into()),
            order: Some(1),
            compatible_with: vec!["b-l".into(), "b-s".into()],
            description: "long bond".into(),
            icon: "link".into(),
            color: "hsl(206 52% 48%)".into(),
            default_rope_kind: "cable.link".into(),
        };
        let value = serde_json::to_value(&kind).unwrap();
        assert_eq!(value["compatibleWith"], serde_json::json!(["b-l", "b-s"]));
        let back: Puzzle5dCatalogGripKind = serde_json::from_value(value).unwrap();
        assert_eq!(back.order, Some(1));
    }
}
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
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs"]
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
                                pub mod create_part {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🧪️tests/🌱️appends-part-c/🦀️.rs"]
                                    mod tests_appends_part_c;
                                }
                                #[path = "."]
                                pub mod delete_part {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-part/🧪️tests/🚫️removes-part-a-and-5f0581/🦀️.rs"]
                                    mod tests_removes_part_a_and_severs_fastener;
                                }
                                #[path = "."]
                                pub mod move_part_2d {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🧪️tests/📍️moves-part-a/🦀️.rs"]
                                    mod tests_moves_part_a;
                                }
                                #[path = "."]
                                pub mod replace_part_2d_geometry {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🧪️tests/🔳️circle-to-rectangle/🦀️.rs"]
                                    mod tests_circle_to_rectangle;
                                }
                                #[path = "."]
                                pub mod edit_part_2d_text {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🧪️tests/✏️retitles-part-a/🦀️.rs"]
                                    mod tests_retitles_part_a;
                                }
                                #[path = "."]
                                pub mod change_part_2d_icon {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🧪️tests/🎨️swaps-icon/🦀️.rs"]
                                    mod tests_swaps_icon;
                                }
                                #[path = "."]
                                pub mod change_part_2d_hidden {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🧪️tests/🙈️hides-part-a/🦀️.rs"]
                                    mod tests_hides_part_a;
                                }
                                #[path = "."]
                                pub mod change_part_2d_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🧪️tests/🔒️locks-part-a/🦀️.rs"]
                                    mod tests_locks_part_a;
                                }
                                #[path = "."]
                                pub mod move_part_3d {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🧪️tests/🗺️moves-part-a-in-world/🦀️.rs"]
                                    mod tests_moves_part_a_in_world;
                                }
                                #[path = "."]
                                pub mod rotate_part_3d {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_part_3d {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🧪️tests/📏️uniform-double/🦀️.rs"]
                                    mod tests_uniform_double;
                                }
                                #[path = "."]
                                pub mod change_part_3d_mesh {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🧪️tests/🕸️repoints-mesh/🦀️.rs"]
                                    mod tests_repoints_mesh;
                                }
                                #[path = "."]
                                pub mod edit_part_3d_label {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🧪️tests/🔤️relabels-part-a/🦀️.rs"]
                                    mod tests_relabels_part_a;
                                }
                                #[path = "."]
                                pub mod change_part_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-part-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-part-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-part-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-part-kind/🧪️tests/🏷️reassigns-kind/🦀️.rs"]
                                    mod tests_reassigns_kind;
                                }
                                #[path = "."]
                                pub mod change_part_anchor {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🧪️tests/⚓️fixed-to-derived/🦀️.rs"]
                                    mod tests_fixed_to_derived;
                                }
                                #[path = "."]
                                pub mod add_part_grip {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🧪️tests/✊️appends-grip-3/🦀️.rs"]
                                    mod tests_appends_grip_3;
                                }
                                #[path = "."]
                                pub mod remove_part_grip {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🧪️tests/🚫️removes-grip-1-and-5d2306/🦀️.rs"]
                                    mod tests_removes_grip_1_and_severs_fastener;
                                }
                                #[path = "."]
                                pub mod replace_part_grip {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🧪️tests/🏷️rekinds-grip-1/🦀️.rs"]
                                    mod tests_rekinds_grip_1;
                                }
                                #[path = "."]
                                pub mod connect_grips {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-grips/🧪️tests/🔩️adds-second-fastener/🦀️.rs"]
                                    mod tests_adds_second_fastener;
                                }
                                #[path = "."]
                                pub mod disconnect_grips {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🧪️tests/🚫️removes-fast-1/🦀️.rs"]
                                    mod tests_removes_fast_1;
                                }
                                #[path = "."]
                                pub mod replace_fastener_geometry {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🧪️tests/📍️repositions-fast-1/🦀️.rs"]
                                    mod tests_repositions_fast_1;
                                }
                                #[path = "."]
                                pub mod change_fastener_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🧪️tests/🏷️rekinds-fast-1/🦀️.rs"]
                                    mod tests_rekinds_fast_1;
                                }
                                #[path = "."]
                                pub mod rename_puzzle5d {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-puzzle5d/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-puzzle5d/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-puzzle5d/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-puzzle5d/🧪️tests/📃️relabels-document/🦀️.rs"]
                                    mod tests_relabels_document;
                                }
                                #[path = "."]
                                pub mod change_domain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🧪️tests/⚙️architecture-to-engineering/🦀️.rs"]
                                    mod tests_architecture_to_engineering;
                                }
                                #[path = "."]
                                pub mod change_description {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/🧪️tests/📝️rewrites-description/🦀️.rs"]
                                    mod tests_rewrites_description;
                                }
                                #[path = "."]
                                pub mod connect_kind_compatibility {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/🤝️adds-grip-pair/🦀️.rs"]
                                    mod tests_adds_grip_pair;
                                }
                                #[path = "."]
                                pub mod disconnect_kind_compatibility {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🧪️tests/🚫️removes-grip-1464bd/🦀️.rs"]
                                    mod tests_removes_grip_pair;
                                }
                                #[path = "."]
                                pub mod replace_kind_catalogs {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🧪️tests/📇️null-catalogs-is-noop/🦀️.rs"]
                                    mod tests_null_catalogs_is_noop;
                                }
                            }
                            #[path = "."]
                            pub mod transfer {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✂️transfer/🦀️.rs"]
                                mod component;
                                pub use component::*;
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
        pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
        pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle5d {
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
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
        pub mod precompute {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️add-brush-part/🦀️.rs"]
            pub mod add_brush_part;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️add-part-kind/🦀️.rs"]
            pub mod add_part_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎲️apply-board-events/🦀️.rs"]
            pub mod apply_board_events;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☀️apply-sun/🦀️.rs"]
            pub mod apply_sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️create-fastener/🦀️.rs"]
            pub mod create_fastener;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-brush-candidate/🦀️.rs"]
            pub mod cycle_brush_candidate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💔️delete-fastener/🦀️.rs"]
            pub mod delete_fastener;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👯️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️edit-fastener/🦀️.rs"]
            pub mod edit_fastener;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs"]
            pub mod engagement_abort;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️engagement-control-select/🦀️.rs"]
            pub mod engagement_control_select;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📨️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪛️patch-fastener/🦀️.rs"]
            pub mod patch_fastener;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✊️patch-grip/🦀️.rs"]
            pub mod patch_grip;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-part/🦀️.rs"]
            pub mod patch_part;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📡️proximity-connect/🦀️.rs"]
            pub mod proximity_connect;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs"]
            pub mod register_brush_mesh;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️retarget-fastener/🦀️.rs"]
            pub mod retarget_fastener;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️select-same-kind/🦀️.rs"]
            pub mod select_same_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active/🦀️.rs"]
            pub mod set_active;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚧️set-brush-placement-overlap-budget/🦀️.rs"]
            pub mod set_brush_placement_overlap_budget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-camera-2d/🦀️.rs"]
            pub mod set_camera_2d;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎦️set-camera-3d/🦀️.rs"]
            pub mod set_camera_3d;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-fill-count/🦀️.rs"]
            pub mod set_fill_count;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-fixture-json/🦀️.rs"]
            pub mod set_fixture_json;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-grid-factor/🦀️.rs"]
            pub mod set_grid_factor;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚖️set-kind-weight/🦀️.rs"]
            pub mod set_kind_weight;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-lod-mode/🦀️.rs"]
            pub mod set_lod_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚩️set-selection-flag/🦀️.rs"]
            pub mod set_selection_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️set-suggestion-offset/🦀️.rs"]
            pub mod set_suggestion_offset;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌍️world-relocate/🦀️.rs"]
            pub mod world_relocate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️zoom-to-selection/🦀️.rs"]
            pub mod zoom_to_selection;
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs"]
                    pub mod brush;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod actions {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🎬️actions/🧩️apply-board-events/🦀️.rs"]
                            pub mod apply_board_events;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🎬️actions/📷️set-camera/🦀️.rs"]
                            pub mod set_camera;
                        }

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs"]
                            pub mod lod;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🖌️brush/🦀️.rs"]
                            pub mod brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🪣️fill/🦀️.rs"]
                            pub mod fill;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🖱️select/🦀️.rs"]
                            pub mod select;
                        }
                    }

                    #[path = "."]
                    pub mod world3d {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod actions {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🔄️rotate-selection/🦀️.rs"]
                            pub mod rotate_selection;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/📐️scale-selection/🦀️.rs"]
                            pub mod scale_selection;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/📷️set-camera/🦀️.rs"]
                            pub mod set_camera;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/↔️translate-selection/🦀️.rs"]
                            pub mod translate_selection;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🚚️world-relocate/🦀️.rs"]
                            pub mod world_relocate;
                        }

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/☀️sun/🦀️.rs"]
                            pub mod sun;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs"]
                            pub mod transform;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🚚️world-relocate/🦀️.rs"]
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
    pub mod puzzle5d {
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
                    pub mod world3d {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️3d/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
