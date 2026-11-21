//! 🎟️ Retained nearest-point traversal bound to its original live topology.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand};

enum ClosestCursor {
    Face(analysis::PlanarClosest),
    Solid(analysis::SolidClosest),
}

/// 🎯️ Holds a projection cursor and its exact engine and topology revision.
pub struct PlanarClosestJob {
    owner: usize,
    revision: u64,
    cursor: ClosestCursor,
}

fn solid_closest_source(entity:&Entity)->Option<analysis::PlanarMeasureSource>{
    match entity {
        Entity::Solid(id,Some(SolidProvenance::PlanarBoundary(_) | SolidProvenance::Orthogonal(_)))=>Some(analysis::PlanarMeasureSource::Solid(*id)),
        Entity::Compound(roots,_)=>analysis::PlanarMeasureSource::compound(roots),
        _=>None,
    }
}

impl Brep {
    /// 🪪️ Admits the original boundary cursor when fixed analytic projection does not apply.
    pub fn plan_planar_closest(&self, handle: &str, target: EVec3) -> Result<Option<PlanarClosestJob>, ValueError> {
        if !target.iter().all(|value| value.is_finite()) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "closest point requires finite coordinates"));
        }
        let entity=self.native.original_query_entity(handle)?;
        if let Entity::Face(id)=entity {
            if self.conic_face_preview_source(*id).is_some(){return Ok(None);}
            return Ok(Some(self.closest_job(ClosestCursor::Face(analysis::PlanarClosest::new(*id,pnt(target))))));
        }
        Ok(solid_closest_source(entity).map(|source|self.closest_job(ClosestCursor::Solid(analysis::SolidClosest::new(source,pnt(target))))))
    }

    /// 🧭️ Opens a retained vertex or linear projection when one source is an authored face.
    pub fn plan_face_distance(&self, a: &str, b: &str) -> Result<Option<(PlanarClosestJob, bool)>, ValueError> {
        let (id, other, source, face_at_b) = match (self.native.original_query_entity(a)?, self.native.original_query_entity(b)?) {
            (Entity::Face(id), other) => (*id, other, b, false),
            (other, Entity::Face(id)) => (*id, other, a, true),
            _ => return Ok(None),
        };
        let cursor = match other {
            Entity::Vertex(vertex) if self.conic_face_preview_source(id).is_none() => {
                let point = self.native.body.vertices.get(*vertex).ok_or_else(|| ValueError::invariant("live distance vertex is missing"))?.position;
                analysis::PlanarClosest::from_vertex(id, point)
            }
            Entity::Curve(..) | Entity::Edge(..) => analysis::PlanarClosest::from_line(id, self.native.original_intersection_curve(source)?)?,
            _ => return Ok(None),
        };
        Ok(Some((self.closest_job(ClosestCursor::Face(cursor)), face_at_b)))
    }

    fn closest_job(&self, cursor: ClosestCursor) -> PlanarClosestJob {
        PlanarClosestJob { owner: std::ptr::from_ref(self) as usize, revision: self.native.shape_mutation_revision, cursor }
    }

    /// 📏️ Quotes the next bounded read from the same live topology and shape revision.
    pub fn planar_closest_demand(&self, handle: &str, job: &PlanarClosestJob) -> Result<RetirementDemand, ValueError> {
        if job.owner != std::ptr::from_ref(self) as usize || job.revision != self.native.shape_mutation_revision {
            return Err(ValueError::invariant("original closest point source changed"));
        }
        let entity=self.native.original_query_entity(handle)?;
        let mut demand=match &job.cursor {
            ClosestCursor::Face(cursor) if matches!(entity,Entity::Face(id) if *id==cursor.source())=>cursor.next_demand(&self.native.body)?,
            ClosestCursor::Solid(cursor) if solid_closest_source(entity)==Some(cursor.source())=>cursor.next_demand(&self.native.body)?,
            _=>return Err(ValueError::invariant("original closest point source changed")),
        };
        demand.depth = demand.depth.max(self.native.tessellation_source_depth_demand(handle)?);
        Ok(demand)
    }

    /// ⏱️ Advances one original boundary cursor step using the caller's grant.
    pub fn step_planar_closest(&self, handle: &str, job: &mut PlanarClosestJob, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<analysis::PlanarClosestOutput>, Progress), ValueError>
    {
        analysis::advance_measure(self.planar_closest_demand(handle, job)?, effect, || match &mut job.cursor {
            ClosestCursor::Face(cursor)=>cursor.advance(&self.native.body),
            ClosestCursor::Solid(cursor)=>cursor.step(&self.native.body).map(|point|point.map(analysis::PlanarClosestOutput::Point)),
        })
    }
}
