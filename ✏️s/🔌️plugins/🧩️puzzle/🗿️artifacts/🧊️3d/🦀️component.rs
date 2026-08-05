//! 🧩️ Puzzle 3d artifact — the `puzzle.3d` document schema: the `Puzzle3dProjection`
//! (schema/domain/meta/objects/attractions/targetVolumes/references), its object/vortex/attraction/
//! target-volume/reference/kind-catalog records, the `Puzzle3dScale` scalar-or-triple pose scale, the
//! shared `Puzzle3dError`, and the `artifact_kind()` spec the play app's manifest binds. Sibling
//! nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`.

use serde::{Deserialize, Serialize};

//#region ⚠️ Errors
/// 🧯️ Puzzle 3d precompute session errors — JSON (de)serialization and brush/fill session state failures.
#[derive(Debug, thiserror::Error)]
pub enum Puzzle3dError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("brush placement rejected")]
    BrushPlacementRejected,
    #[error("fill session unavailable")]
    FillSessionUnavailable,
}
//#endregion ⚠️ Errors

pub const PUZZLE_3D_SCHEMA: &str = "puzzle.3d";

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
impl Serialize for Puzzle3dScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Puzzle3dScale::Uniform(scale) => serializer.serialize_f64(*scale),
            Puzzle3dScale::Vec3(vec3) => vec3.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Puzzle3dScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::Number(n) => Ok(Puzzle3dScale::Uniform(n.as_f64().unwrap_or(1.0))),
            serde_json::Value::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle3dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => Err(serde::de::Error::custom(format!("expected scale to be a number or an [x, y, z] array, found {other}"))),
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
/// 🔘️ One vortex on an object's rim — `vortex_kind` gates attraction compatibility, `position`/
/// `direction` place and orient it, `radius` sizes its brush-fill collision.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dVortex {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub direction: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 🧱️ One placed object — `origin`/`orientation`/`scale` (a scalar-or-`[x,y,z]` `Puzzle3dScale`,
/// see that type and `vec3_scale`) pose it, `vortices` are its rim attraction ports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dObject {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "object_kind")]
    pub object_kind: Option<String>,
    #[serde(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 🔗️ One attraction between two full vortex ids (`object_id:vortex_id`), with the gap/shift/rise/
/// rotation/turn/tilt offsets `compute_brush_placement_pose` resolves into a world pose.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dAttraction {
    #[serde(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
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
}

/// 🧊️ A persisted oriented box constraining fill placement (Volume Brush voxels or Transform-gumball
/// edited volumes). `scale` is a scalar-or-`[x,y,z]` `Puzzle3dScale` — see that type and
/// `volume_scale_vec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[serde(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 🌐️ Where a reference image/media's bytes live and what kind of media it is.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[serde(default)]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_kind: Option<String>,
}

/// 🖼️ A reference plane pinned in world space at `origin`, `width_world` meters wide.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReference {
    pub id: String,
    #[serde(default)]
    pub source: Puzzle3dReferenceSource,
    #[serde(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[serde(default)]
    #[dsl(unit = "m")]
    pub width_world: f64,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub hidden: bool,
}

/// 🔗️ How specifically two vortex/cable kinds are allowed to attract (mirrors `KindCompatEntry`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibility {
    #[dsl(refs = "vortex_kind")]
    pub source: String,
    #[dsl(refs = "vortex_kind")]
    pub target: String,
    #[serde(default)]
    pub bidirectional: bool,
    #[serde(default)]
    pub important: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<String>,
}

/// 🌱️ One rim-vortex template on a `Puzzle3dCatalogObjectKind` (no `label`/`hidden`/`locked` — those
/// are only per-instance `Puzzle3dVortex` fields, not catalog template fields).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexTemplate {
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: String,
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
    pub radius: f64,
}

/// 🧱️ One object-kind catalog row (mirrors this crate's internal `ObjectKind`, extended with the
/// fixture-observed `label`/`name` display fields).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogObjectKind {
    #[dsl(defines = "object_kind")]
    pub id: String,
    pub label: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub vortices: Vec<Puzzle3dCatalogVortexTemplate>,
}

/// 🔘️ One vortex-kind catalog row (mirrors `VortexKindCatalog`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexKind {
    #[dsl(defines = "vortex_kind")]
    pub id: String,
    pub label: String,
    pub name: String,
    pub color: String,
    #[dsl(refs = "cable_kind")]
    pub default_cable_kind: String,
}

/// 🧵️ One cable-kind catalog row (mirrors `CableKindCatalog`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogCableKind {
    #[dsl(defines = "cable_kind")]
    pub id: String,
    pub label: String,
    pub name: String,
    #[dsl(refs = "attraction_kind")]
    pub default_attraction_kind: String,
}

/// 🔗️ One attraction-kind catalog row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogAttractionKind {
    #[dsl(defines = "attraction_kind")]
    pub id: String,
    pub label: String,
    pub name: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained fixture export: object/vortex/cable/
/// attraction kind rows — see `puzzle/3d/manifest/*.manifest.json` for the same schema at the
/// manifest layer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dKindCatalogs {
    #[serde(default)]
    #[dsl(table)]
    pub objects: Vec<Puzzle3dCatalogObjectKind>,
    #[serde(default)]
    #[dsl(table)]
    pub vortices: Vec<Puzzle3dCatalogVortexKind>,
    #[serde(default)]
    #[dsl(table)]
    pub cables: Vec<Puzzle3dCatalogCableKind>,
    #[serde(default)]
    #[dsl(table)]
    pub attractions: Vec<Puzzle3dCatalogAttractionKind>,
}

/// 🗂️ Fixture-carried metadata: the explicit link-compatibility table (typed — a well-understood
/// small structured list, matching this crate's own `KindCompatEntry`) plus the object/vortex/cable/
/// attraction kind catalog bundle (typed — see `Puzzle3dKindCatalogs`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle3dKindCatalogs>,
    #[serde(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle3dKindCompatibility>,
}

/// 🧩️ The puzzle-3d projection: a typed fixture document (schema/domain/meta/objects/
/// attractions/targetVolumes/references) — see `puzzle/3d/example/*.3d.json` for real-world shapes.
/// Camera is intentionally absent: it is session-only per-window runtime state (never a document
/// field), owned by the app's `Puzzle3dWindowOptions` — see that crate's ticket-driven cutover.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "puzzle3d", layout = "lines")]
pub struct Puzzle3dProjection {
    pub schema: String,
    #[serde(default)]
    pub domain: String,
    #[dsl(block)]
    #[serde(default)]
    pub meta: Puzzle3dMeta,
    #[serde(default)]
    #[dsl(table)]
    pub objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    #[dsl(table)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[serde(default)]
    #[dsl(table)]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[serde(default)]
    #[dsl(table)]
    pub references: Vec<Puzzle3dReference>,
}

impl Default for Puzzle3dProjection {
    fn default() -> Self {
        Self { schema: PUZZLE_3D_SCHEMA.to_string(), domain: "architecture".to_string(), meta: Puzzle3dMeta::default(), objects: Vec::new(), attractions: Vec::new(), target_volumes: Vec::new(), references: Vec::new() }
    }
}

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
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Glb, semio_framework_plugin::OsMediaFormat::Obj, semio_framework_plugin::OsMediaFormat::Stl],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Glb, semio_framework_plugin::OsMediaFormat::Obj],
    }
}

/// 🔌️ The `kit.catalog` artifact kind puzzle3d's `kit:in` port consumes — declared here too (harmless
/// if a producer, e.g. block3d, declares an identical spec) so puzzle3d's own OS artifact catalog
/// knows this kind exists even before any producer is wired up.
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
    }
}
//#endregion 🔖️ArtifactKind
