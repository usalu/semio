//! 🔺️ SemioObjectDiff — sparse per-field diff over `SemioObjectSnapshot`. Four independently
//! diffable fields (`transform`, `brep`, `mesh`, `properties`) — an `Option<…>` slot per field
//! (`None` = untouched by this diff), the same per-field shape `🖼️image`'s diff facet uses. The
//! two-level `Option<Option<store::ArtifactChild<S>>>` on the three child fields is real, not
//! decorative: outer `None` = "this diff doesn't touch the slot", `Some(None)` = "clear it",
//! `Some(Some(handle))` = "set it to this handle" — each triad's own `🔺️diff` leaf builds this
//! directly from `(payload, base)`, never apply-then-capture.

use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object.diff")]
pub struct SemioObjectDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<SemioTransform>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brep: Option<Option<store::ArtifactChild<SemioBrepSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Option<store::ArtifactChild<SemioMeshSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Option<store::ArtifactChild<SemioValueSnapshot>>>,
}

impl semio_framework_value::FromValue for SemioObjectDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut diff = Self::default();
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            match key.as_str() {
                "transform" => diff.transform = Some(semio_framework_value::FromValue::from_value(value)?),
                "brep" => diff.brep = Some(semio_framework_value::FromValue::from_value(value)?),
                "mesh" => diff.mesh = Some(semio_framework_value::FromValue::from_value(value)?),
                "properties" => diff.properties = Some(semio_framework_value::FromValue::from_value(value)?),
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Object diff field {key}"))),
            }
        }
        diff.validate().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))?;
        Ok(diff)
    }
}

impl SemioObjectDiff {
    /// 🧩️ Validates every supplied child replacement before publication.
    pub fn validate(&self) -> Result<(), String> {
        use crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if let Some(Some(child)) = &self.brep { validate_semio_child_identity(&child.child_id, &child.target, "brep")?; }
        if let Some(Some(child)) = &self.mesh { validate_semio_child_identity(&child.child_id, &child.target, "mesh")?; }
        if let Some(Some(child)) = &self.properties { validate_semio_child_identity(&child.child_id, &child.target, "value")?; }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.transform.is_none() && self.brep.is_none() && self.mesh.is_none() && self.properties.is_none()
    }
}

impl MutationDiff<SemioObjectSnapshot> for SemioObjectDiff {
    fn apply(&self, base: &SemioObjectSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioObjectSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
        let mut next = base.clone();
        if let Some(t) = &self.transform {
            next.transform = *t;
        }
        if let Some(b) = &self.brep {
            next.brep = b.clone();
        }
        if let Some(m) = &self.mesh {
            next.mesh = m.clone();
        }
        if let Some(p) = &self.properties {
            next.properties = p.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.transform.is_some() {
            self.transform = other.transform;
        }
        if other.brep.is_some() {
            self.brep = other.brep;
        }
        if other.mesh.is_some() {
            self.mesh = other.mesh;
        }
        if other.properties.is_some() {
            self.properties = other.properties;
        }
    }
}

/// 🧮️ `object`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch.
impl protocol::command::DiffAlgebra<SemioObjectSnapshot> for SemioObjectDiff {
    fn between(base: &SemioObjectSnapshot, other: &SemioObjectSnapshot) -> Self {
        SemioObjectDiff {
            transform: (base.transform != other.transform).then_some(other.transform),
            brep: (base.brep != other.brep).then(|| other.brep.clone()),
            mesh: (base.mesh != other.mesh).then(|| other.mesh.clone()),
            properties: (base.properties != other.properties).then(|| other.properties.clone()),
        }
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Self {
        SemioObjectDiff {
            transform: self.transform.as_ref().map(|_| base.transform),
            brep: self.brep.as_ref().map(|_| base.brep.clone()),
            mesh: self.mesh.as_ref().map(|_| base.mesh.clone()),
            properties: self.properties.as_ref().map(|_| base.properties.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec









//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioObjectDiff` cases — single source of truth for
/// `diff_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioObjectDiff> {
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    vec![
        SemioObjectDiff::default(),
        SemioObjectDiff { transform: Some(SemioTransform { translation: SemioPoint3 { x: 5.0, y: 0.0, z: 0.0 }, ..SemioTransform::identity() }), ..Default::default() },
        SemioObjectDiff { brep: Some(None), ..Default::default() },
        SemioObjectDiff {
            mesh: Some(Some(store::ArtifactChild::new(
                "m1".into(),
                semio_framework_artifact_reference::ArtifactRef { artifact_id: "m1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() } },
            ))),
            ..Default::default()
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
