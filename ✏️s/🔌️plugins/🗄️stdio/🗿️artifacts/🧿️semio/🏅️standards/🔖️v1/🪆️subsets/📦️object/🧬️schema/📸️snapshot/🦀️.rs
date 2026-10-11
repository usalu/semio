//! 🧬️ SemioObjectSnapshot — one *spatial thing*: a placement/transform, its geometry, and its
//! property sets. FIRST COMPOSITE subset (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2c) — carries real
//! `store::ArtifactChild<S>` CHILD slots, unlike every leaf subset authored before it. ⚠️ The name
//! is reused from the old value-tree `object` (renamed to `🔢️value` earlier in this ticket); this
//! is a brand-new spatial subset, unrelated in shape.
//!
//! Composes `brep`/`mesh` (geometry, at most one representation of each kind) and `value`
//! (property sets) — all three as OWNED children: the child is its own document with its own
//! history, this snapshot holds only the two-string handle (`child_id`/`target`), never embedded
//! content (per `📌️important.md`'s composition section).

use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA: &str = "stdio.semio.object";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
/// 🧊️ One placed spatial thing. `transform` is the object's own placement (world-relative, no
/// parent-chain here — composition of OBJECTS into a scene graph is a later wave's concern, e.g.
/// `kit`'s designs); `brep`/`mesh` are alternative geometry REPRESENTATIONS (a real-world object
/// may carry a precise b-rep AND a tessellated preview mesh at once, hence both, each optional and
/// independently owned); `properties` is one owned `value` tree for arbitrary property-set data
/// (materials, IFC property sets, custom metadata).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub transform: SemioTransform,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub brep: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
}

impl Default for SemioObjectSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(), transform: SemioTransform::identity(), brep: None, mesh: None, properties: None }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Encodes composite child and link fields through their first-party value contracts.
impl semio_framework_value::ToValue for SemioObjectSnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)), ("transform".to_string(), semio_framework_value::ToValue::to_value(&self.transform))];
        if let Some(brep) = &self.brep {
            entries.push(("brep".to_string(), semio_framework_value::ToValue::to_value(brep)));
        }
        if let Some(mesh) = &self.mesh {
            entries.push(("mesh".to_string(), semio_framework_value::ToValue::to_value(mesh)));
        }
        if let Some(properties) = &self.properties {
            entries.push(("properties".to_string(), semio_framework_value::ToValue::to_value(properties)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for SemioObjectSnapshot {
    fn edit_value_at_path(&mut self, path: &[&str], edit: semio_framework_value::ValueEdit) -> Result<(), semio_framework_value::ValueError> {
        semio_framework_value::edit_through_value(self, path, edit)
    }

    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut schema = None;
        let mut transform = None;
        let mut brep = None;
        let mut mesh = None;
        let mut properties = None;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" => schema = Some(semio_framework_value::FromValue::from_value(value)?),
                "transform" => transform = Some(semio_framework_value::FromValue::from_value(value)?),
                "brep" => brep = Some(semio_framework_value::FromValue::from_value(value)?),
                "mesh" => mesh = Some(semio_framework_value::FromValue::from_value(value)?),
                "properties" => properties = Some(semio_framework_value::FromValue::from_value(value)?),
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Object field {key}"))),
            }
        }
        let snapshot = Self {
            schema: schema.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Object schema"))?,
            transform: transform.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Object transform"))?,
            brep, mesh, properties,
        };
        snapshot.validate().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))?;
        Ok(snapshot)
    }
}

impl SemioObjectSnapshot {
    /// 📦️ Validates the persisted parent boundary across JSON, text and Pack ingress.
    pub fn validate(&self) -> Result<(), String> {
        use crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema != STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA {
            return Err("Object schema required".into());
        }
        if let Some(child) = &self.brep { validate_semio_child_identity(&child.child_id, &child.target, "brep")?; }
        if let Some(child) = &self.mesh { validate_semio_child_identity(&child.child_id, &child.target, "mesh")?; }
        if let Some(child) = &self.properties { validate_semio_child_identity(&child.child_id, &child.target, "value")?; }
        Ok(())
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️ChildCodecPrimitives















//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives


//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
















//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.object` — a non-identity transform plus all three child handles
/// populated (real child_id/target pairs, never embedded content). Single source of truth for
/// `📚️examples/📦️crate/🖼️assets/…` and this facet's own conformance-law tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_object_snapshot() -> SemioObjectSnapshot {
    use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
    let dialect = |subset: &str| semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    SemioObjectSnapshot {
        schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(),
        transform: SemioTransform { translation: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 }, rotation: SemioQuaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
        brep: Some(store::ArtifactChild::new("crate-brep".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "crate-brep".into(), dialect: dialect("brep") })),
        mesh: Some(store::ArtifactChild::new("crate-mesh".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "crate-mesh".into(), dialect: dialect("mesh") })),
        properties: Some(store::ArtifactChild::new("crate-props".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "crate-props".into(), dialect: dialect("value") })),
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests







