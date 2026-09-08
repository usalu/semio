//! 🧮️ Shared semio v1 geometry value types — used across every subset's snapshot. REAL,
//! complete, small types (NOT scaffolded placeholders): every W2 subset agent depends on these
//! existing correctly from day one. Named structs throughout (no bare tuples) — `dsl` has no
//! blanket `DslField` impl for tuples of any arity (f6-final-summary.md §4.3, las/jpg-confirmed
//! gap); `SemioQuaternion` is a named 4-field struct, never `[f64;4]`/a bare tuple.
//!
//! 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
//! STATE-MACHINES) — pure shared value types with no snapshot dependency of their own, so they
//! land in `✉️base`'s own schema (the artifact-wide shared vocabulary every subset already builds
//! on), never an engine. Reached at `standards::v1::subsets::any::schema::geometry` (no shorter
//! shim — every consumer, in-plugin and cross-plugin, now uses this full path).

#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Point
/// 🧪️ `Serialize`/`Deserialize` are test-only (`#[cfg_attr(test, ...)]`) — this module's own
/// `identity_transform_round_trips_through_json` test uses `serde_json` as a differential
/// round-trip oracle over `SemioTransform` (and transitively this struct); no production call
/// site needs it, `ToValue`/`FromValue` is the real production codec.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SemioPoint3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct SemioPoint2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct SemioUv {
    pub u: f64,
    pub v: f64,
}
//#endregion 🔖️Point

//#region 🔖️Color
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct SemioRgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
//#endregion 🔖️Color

//#region 🔖️Transform
/// 🧭️ Rotation as a NAMED quaternion struct — never a bare `[f64;4]`/tuple (see module doc
/// comment). Defaults to the identity rotation `(0,0,0,1)`.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SemioQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Default for SemioQuaternion {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SemioTransform {
    pub translation: SemioPoint3,
    pub rotation: SemioQuaternion,
    pub scale: SemioPoint3,
}

impl Default for SemioTransform {
    fn default() -> Self {
        Self::identity()
    }
}

impl SemioTransform {
    /// 🧭️ Identity transform: zero translation, identity rotation, unit scale.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn identity() -> Self {
        Self { translation: SemioPoint3::default(), rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
    }
}
//#endregion 🔖️Transform

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
