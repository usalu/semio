//! 📤️ `s.stdio.semio/v1/cad` → `step` (ap214) — mirrors the import leaf's real graph resolution
//! in reverse: only `CadEntity::Line`/`Circle` have a genuine AP214 geometric-primitive
//! equivalent in this bridge's scope (see the import leaf's module doc for the full rationale);
//! every other variant (Arc/Ellipse/Polyline/Text/Insert/Solid/Dimension) has no B-rep/solid
//! representation this bridge builds and is dropped on export, documented, not fabricated.
//! `z` is always written `0.0` (`cad` is 2D-only).

use crate::standards::v1::subsets::cad::schema::snapshot::{CadEntity, SemioCadSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_step::{
    schema::snapshot::{StepEntity, StepFileName, StepFileSchema, StepHeader, StepValue},
    StepSnapshot,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId::ANY };

//#region 🔖️GraphBuild
/// 🧮️ Sequential id allocator — every emitted entity gets its own fresh id, matching every real
/// Part-21 exchange file's convention (dense, monotonically-issued `#N` ids).
struct IdGen(u64);
impl IdGen {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_entity(id: u64, x: f64, y: f64) -> StepEntity {
    StepEntity { id, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String(String::new()), StepValue::Aggregate(vec![StepValue::Real(x), StepValue::Real(y), StepValue::Real(0.0)])], complex: vec![] }
}

/// 📐️ Real `LINE` → `CARTESIAN_POINT` + `DIRECTION` + `VECTOR` decomposition (the inverse of the
/// import leaf's resolution): direction is the normalized `b - a`, magnitude is `|b - a|`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn line_entities(ids: &mut IdGen, a: &crate::standards::v1::subsets::base::schema::geometry::SemioPoint2, b: &crate::standards::v1::subsets::base::schema::geometry::SemioPoint2) -> Vec<StepEntity> {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let magnitude = (dx * dx + dy * dy).sqrt();
    let (ndx, ndy) = if magnitude > 0.0 { (dx / magnitude, dy / magnitude) } else { (1.0, 0.0) };
    let p = point_entity(ids.next(), a.x, a.y);
    let dir = StepEntity { id: ids.next(), name: "DIRECTION".into(), args: vec![StepValue::String(String::new()), StepValue::Aggregate(vec![StepValue::Real(ndx), StepValue::Real(ndy), StepValue::Real(0.0)])], complex: vec![] };
    let vec_id = ids.next();
    let vector = StepEntity { id: vec_id, name: "VECTOR".into(), args: vec![StepValue::String(String::new()), StepValue::Reference(dir.id), StepValue::Real(magnitude)], complex: vec![] };
    let line = StepEntity { id: ids.next(), name: "LINE".into(), args: vec![StepValue::String(String::new()), StepValue::Reference(p.id), StepValue::Reference(vector.id)], complex: vec![] };
    vec![p, dir, vector, line]
}

/// ⭕️ Real `CIRCLE` → `AXIS2_PLACEMENT_3D` → `CARTESIAN_POINT` decomposition (axis/refdirection
/// left `Unset` — `$`, spec-legal for an unoriented 2D-only placement, matching real AP214 usage
/// when orientation is unspecified).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn circle_entities(ids: &mut IdGen, center: &crate::standards::v1::subsets::base::schema::geometry::SemioPoint2, radius: f64) -> Vec<StepEntity> {
    let p = point_entity(ids.next(), center.x, center.y);
    let placement = StepEntity { id: ids.next(), name: "AXIS2_PLACEMENT_3D".into(), args: vec![StepValue::String(String::new()), StepValue::Reference(p.id), StepValue::Unset, StepValue::Unset], complex: vec![] };
    let circle = StepEntity { id: ids.next(), name: "CIRCLE".into(), args: vec![StepValue::String(String::new()), StepValue::Reference(placement.id), StepValue::Real(radius)], complex: vec![] };
    vec![p, placement, circle]
}
//#endregion 🔖️GraphBuild

//#region 🔖️Serializer
pub struct SemioCadToStep;

impl ArtifactSerializer for SemioCadToStep {
    type From = SemioCadSnapshot;
    type Into = StepSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut ids = IdGen(0);
        let mut entities = Vec::new();
        for rec in &from.entities {
            match &rec.entity {
                CadEntity::Line { a, b } => entities.extend(line_entities(&mut ids, a, b)),
                CadEntity::Circle { center, radius } => entities.extend(circle_entities(&mut ids, center, *radius)),
                _ => {} // no B-rep/solid equivalent in this bridge's scope — documented, dropped.
            }
        }
        let header = StepHeader { file_description: Default::default(), file_name: StepFileName { name: "semio-cad-export".into(), ..Default::default() }, file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] } };
        Ok(StepSnapshot { schema: semio_s_artifact_stdio_step::STDIO_STEP_DOCUMENT_SCHEMA.into(), header, entities })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
