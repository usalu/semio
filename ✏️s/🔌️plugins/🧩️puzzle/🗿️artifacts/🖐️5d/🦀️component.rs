//! 🧩️ Puzzle 5d artifact — the `puzzle.5d` document schema: the `Puzzle5dSnapshot` (schema/domain/
//! label/meta/kindCatalogs/kindCompatibility/parts/fasteners), its unified 2d+3d part/grip/fastener
//! records, the `Puzzle5dScale` scalar-or-triple pose scale, the `Puzzle5dError` that delegates to
//! the 3d artifact's own precompute-session error, and the `artifact_kind()` spec the play app's
//! manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`.


pub use crate::artifacts::puzzle5d::schema::snapshot::Puzzle5dSnapshot;
pub use crate::artifacts::puzzle5d::schema::mutations::Puzzle5dMutation;
pub use crate::artifacts::puzzle5d::schema::diff::Puzzle5dDiff;

use serde::{Deserialize, Serialize};

//#region ⚠️ Errors
/// 🧯️ Puzzle 5d precompute session errors — delegates entirely to `puzzle_3d`'s own precompute-session error.
#[derive(Debug, thiserror::Error)]
pub enum Puzzle5dError {
    #[error(transparent)]
    Puzzle3d(#[from] crate::artifacts::puzzle3d::Puzzle3dError),
}
//#endregion ⚠️ Errors



pub const PUZZLE_5D_SCHEMA: &str = "puzzle.5d";

// #region 🔖️Document
/// 📝️ Free-text scene description — the only field seen under the fixture's top-level `meta`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dMeta {
    #[serde(default)]
    pub description: String,
}

/// 🔵️ A part's 2D-projection presentation (board node): `shape`/`radius` size the circle/rectangle,
/// `text`/`icon_kind` label it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart2d {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
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
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
impl Serialize for Puzzle5dScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Puzzle5dScale::Uniform(scale) => serializer.serialize_f64(*scale),
            Puzzle5dScale::Vec3(vec3) => vec3.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Puzzle5dScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::Number(n) => Ok(Puzzle5dScale::Uniform(n.as_f64().unwrap_or(1.0))),
            serde_json::Value::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle5dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => Err(serde::de::Error::custom(format!("expected scale to be a number or an [x, y, z] array, found {other}"))),
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart3d {
    #[serde(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle5dScale>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🔘️ A grip's 2D-projection presentation (board handle) — `grip_kind` is duplicated here from
/// `Puzzle5dGrip::grip_kind` in real fixtures (a per-projection override slot), not simplified away.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dGrip2d {
    #[serde(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

/// 🔘️ A grip's 3D-projection presentation (world vortex).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dGrip3d {
    #[serde(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub direction: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🔘️ One rim grip on a part, unified across both projections.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dGrip {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[serde(default, rename = "2d")]
    pub grip_2d: Puzzle5dGrip2d,
    #[serde(default, rename = "3d")]
    pub grip_3d: Puzzle5dGrip3d,
}

/// ⚓️ Whether a part keeps its stored plane at a BFS root (`Fixed`) or resets the plane to default XY (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "part_kind")]
    pub part_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Puzzle5dPartAnchor::is_fixed")]
    pub anchor: Puzzle5dPartAnchor,
    #[serde(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[serde(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[serde(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

impl Default for Puzzle5dPart {
    fn default() -> Self {
        Self {
            id: String::new(),
            part_kind: None,
            anchor: Puzzle5dPartAnchor::Fixed,
            part_2d: Puzzle5dPart2d::default(),
            part_3d: Puzzle5dPart3d::default(),
            grips: Vec::new(),
        }
    }
}

/// 🔗️ One fastener (2D edge / 3D attraction) between two full grip ids (`part_id:grip_id`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dFastener {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "fastener_kind")]
    pub fastener_kind: Option<String>,
    /// 🔧️ Pose-solver offsets `compute_brush_placement_pose` / compose `geom::flatten` resolve into a world pose —
    /// mirrors `puzzle_3d::Puzzle3dAttraction` plus diagram offsets `x`/`y` (compose Connection `u`/`v`).
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
}

/// 🔗️ How specifically two grip/rope kinds are allowed to fasten.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "lowercase")]
pub enum Puzzle5dCompatSpecificity {
    #[default]
    General,
    Part,
    Fastener,
    Grip,
    Rope,
}

/// 🔗️ One allowed (or unidirectional) kind pair — unified with 2d/3d via `important` + `specificity`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dKindCompatibility {
    #[dsl(refs = "grip_kind")]
    pub source: String,
    #[dsl(refs = "grip_kind")]
    pub target: String,
    #[serde(default)]
    pub bidirectional: bool,
    #[serde(default)]
    pub important: bool,
    #[serde(default)]
    pub specificity: Puzzle5dCompatSpecificity,
}

/// 🏷️ One freeform attribute on a part-kind (compose `Attribute` analogue).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dAttribute {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ One author credit on a part-kind (compose `Author` analogue).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dAuthor {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ One tagged representation (mesh/image/…) on a part-kind.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dRepresentation {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub mime: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    #[serde(default)]
    pub description: String,
}

fn default_grip_direction() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

/// 🌱️ One rim-grip template on a `Puzzle5dCatalogPartKind` (compose Connector analogue).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dGripTemplate {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[serde(default)]
    #[dsl(coord)]
    pub point: [f64; 3],
    #[serde(default = "default_grip_direction")]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

impl Default for Puzzle5dGripTemplate {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            label: String::new(),
            description: String::new(),
            icon: String::new(),
            grip_kind: None,
            point: [0.0, 0.0, 0.0],
            direction: default_grip_direction(),
            t: None,
            mandatory: None,
            radius: None,
        }
    }
}

/// 🧱️ One part-kind catalog row (compose Type analogue).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogPartKind {
    #[dsl(defines = "part_kind")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default, rename = "abstract")]
    pub is_abstract: bool,
    #[serde(default)]
    pub base_kinds: Vec<String>,
    #[serde(default)]
    pub representations: Vec<Puzzle5dRepresentation>,
    #[serde(default)]
    pub grips: Vec<Puzzle5dGripTemplate>,
    #[serde(default)]
    pub attributes: Vec<Puzzle5dAttribute>,
    #[serde(default)]
    pub authors: Vec<Puzzle5dAuthor>,
}

/// 🔘️ One grip-kind catalog row (compose Port analogue).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripKind {
    #[dsl(defines = "grip_kind")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default)]
    pub compatible_with: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    #[dsl(refs = "rope_kind")]
    pub default_rope_kind: String,
}

/// 🔗️ One fastener-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogFastenerKind {
    #[dsl(defines = "fastener_kind")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🧵️ One rope-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogRopeKind {
    #[dsl(defines = "rope_kind")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    #[dsl(refs = "fastener_kind")]
    pub default_fastener_kind: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained fixture export: part/grip/fastener/rope
/// kind rows — see `puzzle/5d/manifest/*.manifest.json` for the same schema at the manifest layer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dKindCatalogs {
    #[serde(default)]
    #[dsl(table)]
    pub parts: Vec<Puzzle5dCatalogPartKind>,
    #[serde(default)]
    #[dsl(table)]
    pub grips: Vec<Puzzle5dCatalogGripKind>,
    #[serde(default)]
    #[dsl(table)]
    pub fasteners: Vec<Puzzle5dCatalogFastenerKind>,
    #[serde(default)]
    #[dsl(table)]
    pub ropes: Vec<Puzzle5dCatalogRopeKind>,
}

/// 🏷️ Temporary name alias until Wave 3 app callers migrate to [`Puzzle5dCatalogPartKind`].
pub type Puzzle5dCatalogPart = Puzzle5dCatalogPartKind;
/// 🏷️ Temporary name alias until Wave 3 app callers migrate to [`Puzzle5dCatalogGripKind`].
pub type Puzzle5dCatalogGrip = Puzzle5dCatalogGripKind;

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
            export_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

pub use crate::artifacts::puzzle5d::op::Puzzle5dPlaySnapshot;

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
        let fastener = Puzzle5dFastener {
            id: "f1".into(),
            source: "p1:g0".into(),
            target: "p2:g0".into(),
            fastener_kind: Some("fk".into()),
            gap: 1.0,
            shift: 2.0,
            rise: 3.0,
            rotation: 4.0,
            turn: 5.0,
            tilt: 6.0,
            x: 7.0,
            y: 8.0,
        };
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
            representations: vec![Puzzle5dRepresentation {
                id: "lod0".into(),
                name: "mesh".into(),
                url: "/mesh/hex.glb".into(),
                mime: "model/gltf-binary".into(),
                tags: vec!["mesh".into()],
                lod: Some("0".into()),
                description: "".into(),
            }],
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
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::puzzle5d::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Puzzle5dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
