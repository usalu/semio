//! 🧩️ Puzzle 3d artifact — the `puzzle.3d` document schema: the `Puzzle3dSnapshot`
//! (schema/domain/meta/objects/attractions/targetVolumes/references), its object/vortex/attraction/
//! target-volume/reference/kind-catalog records, the `Puzzle3dScale` scalar-or-triple pose scale, the
//! shared `Puzzle3dError`, and the `artifact_kind()` spec the play app's manifest binds. Sibling
//! nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`.


pub use crate::artifacts::puzzle3d::schema::snapshot::Puzzle3dSnapshot;
pub use crate::artifacts::puzzle3d::schema::mutations::Puzzle3dMutation;
pub use crate::artifacts::puzzle3d::schema::diff::Puzzle3dDiff;

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
/// ⚓️ Whether a root object keeps its stored plane (`Fixed`) or resets to default XY (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle3dObjectAnchor {
    #[default]
    Fixed,
    Derived,
}

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
/// see that type and `vec3_scale`) pose it, `anchor` gates flatten-root plane retention, `vortices`
/// are its rim attraction ports.
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
    pub anchor: Puzzle3dObjectAnchor,
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

/// 🔗️ One attraction between two full vortex ids (`object_id:vortex_id`), with the eight compose
/// connection parameters (`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` plus diagram `x`/`y`).
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
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
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

/// 🔗️ How specifically two vortex/cable kinds are allowed to attract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "lowercase")]
pub enum Puzzle3dCompatSpecificity {
    General,
    Object,
    Attraction,
    Cable,
    #[default]
    Vortex,
}

/// 🧩️ One allowed (or, unidirectional, one-way-allowed) link pair between two vortex/cable kind ids.
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
    #[serde(default)]
    pub specificity: Puzzle3dCompatSpecificity,
}

/// 🏷️ One freeform attribute on a catalog object-kind (compose `Attribute` analogue).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dAttribute {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ One author credit on a catalog object-kind (compose `Author` analogue).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dAuthor {
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

/// 🖼️ One tagged representation/LOD URL on a catalog object-kind (compose `Representation` analogue).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dRepresentation {
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

/// 🌱️ One rim-vortex template on a `Puzzle3dCatalogObjectKind` — compose connector analogue with
/// `point`/`direction`/`t`/`mandatory`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexTemplate {
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
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[serde(default)]
    #[dsl(coord)]
    pub point: [f64; 3],
    #[serde(default = "puzzle3d_default_direction")]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

fn puzzle3d_default_direction() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

impl Default for Puzzle3dCatalogVortexTemplate {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            label: String::new(),
            description: String::new(),
            icon: String::new(),
            vortex_kind: None,
            point: [0.0, 0.0, 0.0],
            direction: puzzle3d_default_direction(),
            t: None,
            mandatory: None,
            radius: None,
        }
    }
}

/// 🧱️ One object-kind catalog row — type-like (compose `Type` analogue).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogObjectKind {
    #[dsl(defines = "object_kind")]
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
    pub representations: Vec<Puzzle3dRepresentation>,
    #[serde(default)]
    pub vortices: Vec<Puzzle3dCatalogVortexTemplate>,
    #[serde(default)]
    pub attributes: Vec<Puzzle3dAttribute>,
    #[serde(default)]
    pub authors: Vec<Puzzle3dAuthor>,
}

/// 🔘️ One vortex-kind catalog row — port-like (compose `Port` analogue).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexKind {
    #[dsl(defines = "vortex_kind")]
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
    #[dsl(refs = "cable_kind")]
    pub default_cable_kind: String,
}

/// 🧵️ One cable-kind catalog row (mirrors `CableKindCatalog`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogCableKind {
    #[dsl(defines = "cable_kind")]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[dsl(refs = "attraction_kind")]
    pub default_attraction_kind: String,
}

/// 🔗️ One attraction-kind catalog row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogAttractionKind {
    #[dsl(defines = "attraction_kind")]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
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

/// 🗂️ Fixture-carried metadata: the explicit link-compatibility table plus the object/vortex/cable/
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
            export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
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
            export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ Puzzle3d's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated off
/// the former `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, never
/// engine/app behaviour) — replaces the `ComposerEntry` half of the old `register_io()`. The
/// `"3d.puzzle"` OS-host mesh export/import bridge (`crate::apps::puzzle3d::register_mesh_io()`, moved
/// app-side by ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) has NO `ArtifactDeclaration`
/// field — it belongs to the same OS media-host 14-function family flagged on puzzle2d's
/// `declaration()` doc, a different mechanism from the nine §6 registrars this struct covers — so it
/// stays wired through `🧩️puzzle/🦀️component.rs`'s own `.setup()`, not here.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.puzzle3d")
        .schema(crate::artifacts::puzzle3d::schema::puzzle3d_artifact_schema_descriptor())
        .inferences([crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::inferences::puzzle3d_artifact_inference_descriptor()])
        .composers(crate::artifacts::puzzle3d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::puzzle3d::Puzzle3dPlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. This function
/// existed as a side-effecting `register_pilot_languages()` before M1 but was never called from
/// anywhere (dead code, confirmed by grep) — wiring it into `declaration()`'s `.languages(...)` is
/// this conversion's one real bug fix: puzzle3d's own grammars were never actually registered.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "puzzle.puzzle3d",
                    extension: Some("puzzle3d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::puzzle3d::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::puzzle3d::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle3d"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::puzzle3d::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::puzzle3d::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("puzzle.puzzle3d.op"),
                },
                dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::puzzle3d::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::puzzle3d::diff::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

pub use crate::artifacts::puzzle3d::op::Puzzle3dPlaySnapshot;



//#region 🧪️Tests
#[cfg(test)]
mod design_parity_schema_tests {
    use super::*;

    #[test]
    fn attraction_exposes_eight_connection_parameters_with_zero_defaults() {
        let attraction = Puzzle3dAttraction {
            id: "a".into(),
            attracting: "o1:v0".into(),
            attracted: "o2:v0".into(),
            gap: 0.0,
            shift: 0.0,
            rise: 0.0,
            rotation: 0.0,
            turn: 0.0,
            tilt: 0.0,
            x: 0.0,
            y: 0.0,
        };
        let json = serde_json::to_value(&attraction).expect("serialize");
        for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"] {
            assert_eq!(json.get(key).and_then(|v| v.as_f64()), Some(0.0), "{key}");
        }
        let parsed: Puzzle3dAttraction = serde_json::from_value(serde_json::json!({
            "attracting": "o1:v0",
            "attracted": "o2:v0"
        })).expect("sparse attraction deserializes");
        assert_eq!(parsed.x, 0.0);
        assert_eq!(parsed.y, 0.0);
        assert_eq!(parsed.gap, 0.0);
    }

    #[test]
    fn object_anchor_defaults_to_fixed() {
        let object: Puzzle3dObject = serde_json::from_value(serde_json::json!({
            "id": "o1"
        })).expect("object");
        assert_eq!(object.anchor, Puzzle3dObjectAnchor::Fixed);
        assert_eq!(
            serde_json::to_value(Puzzle3dObjectAnchor::Derived).unwrap(),
            serde_json::json!("derived")
        );
    }

    #[test]
    fn object_kind_is_type_like_with_representations() {
        let kind = Puzzle3dCatalogObjectKind {
            id: "Capsule".into(),
            name: "Capsule".into(),
            label: "Capsule".into(),
            description: "demo".into(),
            icon: "".into(),
            image: "".into(),
            unit: "m".into(),
            is_abstract: false,
            base_kinds: vec!["Part".into()],
            representations: vec![Puzzle3dRepresentation {
                id: "mesh".into(),
                name: "mesh".into(),
                url: "/mesh/capsule.glb".into(),
                mime: "model/gltf-binary".into(),
                tags: vec!["default".into()],
                lod: Some("high".into()),
                description: "".into(),
            }],
            vortices: vec![Puzzle3dCatalogVortexTemplate {
                id: "v0".into(),
                name: "v0".into(),
                label: "v0".into(),
                description: "".into(),
                icon: "".into(),
                vortex_kind: Some("c-t".into()),
                point: [0.0, 0.0, 3.0],
                direction: [0.0, 0.0, 1.0],
                t: Some(0.25),
                mandatory: Some(true),
                radius: Some(0.36),
            }],
            attributes: vec![Puzzle3dAttribute {
                id: "a1".into(),
                key: "material".into(),
                value: "concrete".into(),
                definition: None,
            }],
            authors: vec![Puzzle3dAuthor {
                id: "u1".into(),
                name: "Ada".into(),
                email: "ada@example.com".into(),
                role: Some("author".into()),
                rank: Some(1),
            }],
        };
        let json = serde_json::to_value(&kind).expect("serialize");
        assert_eq!(json.get("abstract").and_then(|v| v.as_bool()), Some(false));
        assert!(json.get("meshUrl").is_none());
        assert_eq!(json["representations"][0]["url"], "/mesh/capsule.glb");
        assert_eq!(json["vortices"][0]["point"][2], 3.0);
        let round: Puzzle3dCatalogObjectKind = serde_json::from_value(json).expect("round");
        assert_eq!(round.vortices[0].point, [0.0, 0.0, 3.0]);
        assert_eq!(round.representations[0].mime, "model/gltf-binary");
    }

    #[test]
    fn vortex_kind_is_port_like() {
        let kind = Puzzle3dCatalogVortexKind {
            id: "c-t".into(),
            code: Some("CT".into()),
            label: Some("ceiling top".into()),
            order: Some(2),
            compatible_with: vec!["c-b".into()],
            description: "ceiling".into(),
            icon: "".into(),
            color: "hsl(169 52% 48%)".into(),
            default_cable_kind: "cable.link".into(),
        };
        let json = serde_json::to_value(&kind).expect("serialize");
        assert_eq!(json["compatibleWith"][0], "c-b");
        assert_eq!(json["defaultCableKind"], "cable.link");
    }

    #[test]
    fn kind_compatibility_uses_typed_specificity() {
        let rule = Puzzle3dKindCompatibility {
            source: "c-t".into(),
            target: "c-b".into(),
            bidirectional: true,
            important: false,
            specificity: Puzzle3dCompatSpecificity::Vortex,
        };
        let json = serde_json::to_value(&rule).expect("serialize");
        assert_eq!(json["specificity"], "vortex");
        let parsed: Puzzle3dKindCompatibility = serde_json::from_value(serde_json::json!({
            "source": "a",
            "target": "b"
        })).expect("defaults");
        assert_eq!(parsed.specificity, Puzzle3dCompatSpecificity::Vortex);
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::puzzle3d::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Puzzle3dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
