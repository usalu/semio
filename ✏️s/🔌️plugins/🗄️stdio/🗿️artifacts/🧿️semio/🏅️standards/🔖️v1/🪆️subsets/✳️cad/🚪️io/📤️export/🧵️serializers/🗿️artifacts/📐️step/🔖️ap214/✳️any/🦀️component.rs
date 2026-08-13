//! 📤️ `s.stdio.semio/v1/cad` → `step` (ap214) — mirrors the import leaf's real graph resolution
//! in reverse: only `CadEntity::Line`/`Circle` have a genuine AP214 geometric-primitive
//! equivalent in this bridge's scope (see the import leaf's module doc for the full rationale);
//! every other variant (Arc/Ellipse/Polyline/Text/Insert/Solid/Dimension) has no B-rep/solid
//! representation this bridge builds and is dropped on export, documented, not fabricated.
//! `z` is always written `0.0` (`cad` is 2D-only).

use crate::artifacts::step::{StepSnapshot, schema::snapshot::{StepEntity, StepFileName, StepFileSchema, StepHeader, StepValue}};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadEntity, SemioCadSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId::ANY };

//#region 🔖️GraphBuild
/// 🧮️ Sequential id allocator — every emitted entity gets its own fresh id, matching every real
/// Part-21 exchange file's convention (dense, monotonically-issued `#N` ids).
struct IdGen(u64);
impl IdGen {
    fn next(&mut self) -> u64 { self.0 += 1; self.0 }
}

fn point_entity(id: u64, x: f64, y: f64) -> StepEntity {
    StepEntity { id, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String(String::new()), StepValue::Aggregate(vec![StepValue::Real(x), StepValue::Real(y), StepValue::Real(0.0)])], complex: vec![] }
}

/// 📐️ Real `LINE` → `CARTESIAN_POINT` + `DIRECTION` + `VECTOR` decomposition (the inverse of the
/// import leaf's resolution): direction is the normalized `b - a`, magnitude is `|b - a|`.
fn line_entities(ids: &mut IdGen, a: &crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2, b: &crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2) -> Vec<StepEntity> {
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
fn circle_entities(ids: &mut IdGen, center: &crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2, radius: f64) -> Vec<StepEntity> {
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

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut ids = IdGen(0);
        let mut entities = Vec::new();
        for rec in &from.entities {
            match &rec.entity {
                CadEntity::Line { a, b } => entities.extend(line_entities(&mut ids, a, b)),
                CadEntity::Circle { center, radius } => entities.extend(circle_entities(&mut ids, center, *radius)),
                _ => {} // no B-rep/solid equivalent in this bridge's scope — documented, dropped.
            }
        }
        let header = StepHeader {
            file_description: Default::default(),
            file_name: StepFileName { name: "semio-cad-export".into(), ..Default::default() },
            file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
        };
        Ok(StepSnapshot { schema: crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA.into(), header, entities })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::CadEntityRecord;

    fn sample_cad() -> SemioCadSnapshot {
        SemioCadSnapshot {
            entities: vec![
                CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 5.0, y: 0.0 } } },
                CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.5 } },
                CadEntityRecord { handle: "h3".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2::default(), height: 1.0, rotation: 0.0, content: "dropped".into() } },
            ],
            ..SemioCadSnapshot::default()
        }
    }

    /// 🧪️ Real round trip through step's own real Part-21 text codec.
    #[test]
    fn real_text_round_trip_through_step_codec() {
        let cad = sample_cad();
        let step = SemioCadToStep::serialize(&cad).expect("serialize");
        assert_eq!(step.header.file_schema.schemas, vec!["AUTOMOTIVE_DESIGN".to_string()]);
        assert_eq!(step.entities.len(), 7, "4 for LINE + 3 for CIRCLE; Text is dropped");

        let text = store::ArtifactDsl::print_dsl(&step);
        let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse real step text");
        assert_eq!(reparsed, step, "step's own codec_retention_law must hold on our emitted graph");

        let line_count = reparsed.entities.iter().filter(|e| e.name == "LINE").count();
        let circle_count = reparsed.entities.iter().filter(|e| e.name == "CIRCLE").count();
        assert_eq!(line_count, 1);
        assert_eq!(circle_count, 1);
    }
}
//#endregion 🔖️Tests
