//! 🧮️ Shared semio v1 geometry value types — used across every subset's snapshot. REAL,
//! complete, small types (NOT scaffolded placeholders): every W2 subset agent depends on these
//! existing correctly from day one. Named structs throughout (no bare tuples) — `dsl` has no
//! blanket `DslField` impl for tuples of any arity (f6-final-summary.md §4.3, las/jpg-confirmed
//! gap); `SemioQuaternion` is a named 4-field struct, never `[f64;4]`/a bare tuple.
//!
//! 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
//! STATE-MACHINES) — pure shared value types with no snapshot dependency of their own, so they
//! land in `✳️any`'s own schema (the artifact-wide shared vocabulary every subset already builds
//! on), never an engine. Reached at `standards::v1::subsets::any::schema::geometry` (no shorter
//! shim — every consumer, in-plugin and cross-plugin, now uses this full path).


//#region 🔖️Point
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn identity_transform_round_trips_through_json() {
        let t = SemioTransform::identity();
        let json = serde_json::to_string(&t).expect("serialize");
        let back: SemioTransform = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(t, back);
        assert_eq!(back.rotation.w, 1.0);
        assert_eq!(back.scale, SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 });
    }

    #[semio_framework_async_macros::async_test]
    async fn rgba_and_uv_default_to_zero() {
        assert_eq!(SemioRgba::default(), SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
        assert_eq!(SemioUv::default(), SemioUv { u: 0.0, v: 0.0 });
    }

    #[semio_framework_async_macros::async_test]
    async fn point3_and_point2_are_plain_structs_not_tuples() {
        // 🧪️ Structural proof against the f6 §4.3 bare-tuple `DslField` gap: field ACCESS by
        // name, not `.0`/`.1` positional tuple indexing.
        let p3 = SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 };
        let p2 = SemioPoint2 { x: p3.x, y: p3.y };
        assert_eq!(p2, SemioPoint2 { x: 1.0, y: 2.0 });
    }
}
//#endregion 🔖️Tests
