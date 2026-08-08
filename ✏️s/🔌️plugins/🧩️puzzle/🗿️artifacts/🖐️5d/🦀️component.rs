//! 🧩️ Puzzle 5d artifact — the `puzzle.5d` document schema: the `Puzzle5dProjection` (schema/domain/
//! label/meta/kindCatalogs/kindCompatibility/parts/fasteners), its unified 2d+3d part/grip/fastener
//! records, the `Puzzle5dScale` scalar-or-triple pose scale, the `Puzzle5dError` that delegates to
//! the 3d artifact's own precompute-session error, and the `artifact_kind()` spec the play app's
//! manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`.

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

/// 🧱️ One placed part, unified across both projections — `grips` are its rim attraction/link ports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "part_kind")]
    pub part_kind: Option<String>,
    #[serde(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[serde(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[serde(default)]
    pub grips: Vec<Puzzle5dGrip>,
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
    /// 🔧️ The six pose-solver offsets `compute_brush_placement_pose` resolves into a world pose —
    /// mirrors `puzzle_3d::Puzzle3dAttraction`'s gap/shift/rise/rotation/turn/tilt fields verbatim
    /// (the 5d fastener is the unification of a 2d edge and a 3d attraction).
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

/// 🔗️ How specifically two grip/rope kinds are allowed to fasten.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dKindCompatibility {
    #[dsl(refs = "grip_kind")]
    pub source: String,
    #[dsl(refs = "grip_kind")]
    pub target: String,
    #[serde(default)]
    pub bidirectional: bool,
}

/// 🌱️ One rim-grip template on a `Puzzle5dCatalogPart`, unified across both projections (either
/// projection may be absent — not every part-kind grip template models both).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripTemplate {
    #[dsl(refs = "grip_kind")]
    pub grip_kind: String,
    #[serde(default, rename = "2d", skip_serializing_if = "Option::is_none")]
    pub grip_2d: Option<Puzzle5dCatalogGripTemplate2d>,
    #[serde(default, rename = "3d", skip_serializing_if = "Option::is_none")]
    pub grip_3d: Option<Puzzle5dCatalogGripTemplate3d>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripTemplate2d {
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[dsl(refs = "grip_kind")]
    pub grip_kind: String,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripTemplate3d {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
    pub radius: f64,
}

/// 🧱️ One part-kind catalog row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogPart {
    #[dsl(defines = "part_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub grips: Vec<Puzzle5dCatalogGripTemplate>,
}

/// 🔘️ One grip-kind catalog row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGrip {
    #[dsl(defines = "grip_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    #[dsl(refs = "rope_kind")]
    pub default_rope_kind: String,
}

/// 🔗️ One fastener-kind catalog row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogFastener {
    #[dsl(defines = "fastener_kind")]
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🧵️ One rope-kind catalog row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCatalogRope {
    #[dsl(defines = "rope_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
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
    pub parts: Vec<Puzzle5dCatalogPart>,
    #[serde(default)]
    #[dsl(table)]
    pub grips: Vec<Puzzle5dCatalogGrip>,
    #[serde(default)]
    #[dsl(table)]
    pub fasteners: Vec<Puzzle5dCatalogFastener>,
    #[serde(default)]
    #[dsl(table)]
    pub ropes: Vec<Puzzle5dCatalogRope>,
}

/// 👯️ The puzzle-5d projection: a typed unified 2d+3d document (schema/domain/label/meta/
/// kindCatalogs/kindCompatibility/parts/fasteners) — see `puzzle/5d/example/*.5d.json` for
/// real-world shapes. Camera pose is session-only app runtime state, never part of this
/// VCS-tracked document — see `puzzle_5d_ui`'s `Puzzle5dRuntime`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "puzzle.puzzle5d", layout = "lines")]
pub struct Puzzle5dProjection {
    pub schema: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[dsl(block)]
    #[serde(default)]
    pub meta: Puzzle5dMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle5dKindCatalogs>,
    #[serde(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
    #[serde(default)]
    #[dsl(table)]
    pub parts: Vec<Puzzle5dPart>,
    #[serde(default)]
    #[dsl(table)]
    pub fasteners: Vec<Puzzle5dFastener>,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for Puzzle5dProjection {
    const EXTENSION: &'static str = "puzzle5d";
    fn envelope_id() -> &'static str { "puzzle.puzzle5d" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for Puzzle5dProjection {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs




impl Default for Puzzle5dProjection {
    fn default() -> Self {
        Self { schema: PUZZLE_5D_SCHEMA.to_string(), domain: "architecture".to_string(), label: None, meta: Puzzle5dMeta::default(), kind_catalogs: None, kind_compatibility: Vec::new(), parts: Vec::new(), fasteners: Vec::new() }
    }
}

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
    }
}
//#endregion 🔖️ArtifactKind

pub use crate::artifacts::puzzle5d::op::Puzzle5dPlayProjection;
