//! 📦️ Engine ownership and retained grants for planar area and volume integration.

use super::{Brep, Curve3, Entity, PersistentLabel, Pnt3};
use semio_framework_3d_brep_analysis::{LengthAccumulator, LengthSource, PlanarMeasure, PlanarMeasureSource, advance_measure};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};

/// 🔬️ Keeps the planar integration bound to its original live engine and revision.
pub struct PlanarMeasureJob {
    owner: usize,
    revision: u64,
    integration: PlanarMeasure,
}

impl PlanarMeasureJob {
    pub fn center_of_mass(&self)->Option<[f64;3]>{self.integration.centroid().map(|point|point.to_array())}
    pub fn axis_bounds(&self)->Option<[Pnt3;2]>{self.integration.axis_bounds()}
}

impl Brep {
    fn planar_measure_source(&self, handle: &str) -> Option<PlanarMeasureSource> {
        match self.native.live.get(handle)? {
            Entity::Solid(id, _) => Some(PlanarMeasureSource::Solid(*id)),
            Entity::Face(id) => Some(PlanarMeasureSource::Face(*id)),
            Entity::Compound(roots,_)=>PlanarMeasureSource::compound(roots),
            _ => None,
        }
    }

    /// 🎟️ Opens planar integration on a live solid or area face.
    pub fn plan_planar_measure(&self, handle: &str, measure_area: bool) -> Result<PlanarMeasureJob, ValueError> {
        let source = self.planar_measure_source(handle).filter(|source| measure_area || !matches!(source, PlanarMeasureSource::Face(_)))
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original measurement requires a supported planar face or solid"))?;
        Ok(PlanarMeasureJob {
            owner: std::ptr::from_ref(self) as usize, revision: self.native.shape_mutation_revision,
            integration: PlanarMeasure::new(source, measure_area),
        })
    }

    /// ⚖️ Opens a signed planar center-of-mass traversal on the same original topology.
    pub fn plan_planar_center_of_mass(&self,handle:&str)->Result<PlanarMeasureJob,ValueError>{
        let job=self.plan_planar_measure(handle,false)?;
        Ok(PlanarMeasureJob{integration:job.integration.with_centroid(),..job})
    }

    /// 📦️ Opens the exact axis-range traversal over original straight-sided solids.
    pub fn plan_planar_bounds(&self,handle:&str)->Result<PlanarMeasureJob,ValueError>{
        let job=self.plan_planar_measure(handle,false)?;
        Ok(PlanarMeasureJob{integration:job.integration.with_axis_bounds(),..job})
    }

    /// 🪙️ Quotes the same bound source and next original topology projection.
    pub fn planar_measure_demand(&self, handle: &str, job: &PlanarMeasureJob) -> Result<RetirementDemand, ValueError> {
        if job.owner != std::ptr::from_ref(self) as usize || job.revision != self.native.shape_mutation_revision
            || self.planar_measure_source(handle) != Some(job.integration.source())
        {
            return Err(ValueError::invariant("original planar measurement source changed"));
        }
        let mut demand = job.integration.next_demand(&self.native.body)?;
        demand.depth = demand.depth.max(self.native.live.next_contains_depth_demand(handle)?);
        Ok(demand)
    }

    /// ⏱️ Integrates one original planar boundary step using the original caller effect.
    pub fn step_planar_measure(&self, handle: &str, job: &mut PlanarMeasureJob, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<f64>, Progress), ValueError> {
        advance_measure(self.planar_measure_demand(handle, job)?, effect, || job.integration.step(&self.native.body))
    }
}


/// 📏️ Binds incremental curve or distinct wire length to its original shape revision.
pub struct WireLengthJob {
    owner: usize,
    revision: u64,
    label: PersistentLabel,
    integration: LengthAccumulator,
}

impl Brep {
    /// 🧮️ Opens the retained length cursor on a bounded curved edge, spline, or wire.
    pub fn plan_wire_length(&self, handle: &str) -> Result<Option<WireLengthJob>, ValueError> {
        Ok(self.length_source(handle)?.map(|(label, _)| WireLengthJob {
            owner: std::ptr::from_ref(self) as usize, revision: self.native.shape_mutation_revision,
            label, integration: LengthAccumulator::default(),
        }))
    }

    fn length_source(&self, handle: &str) -> Result<Option<(PersistentLabel, LengthSource<'_>)>, ValueError> {
        Ok(match self.native.original_query_entity(handle)? {
            Entity::Wire(wire, label) => Some((*label, LengthSource::Wire(wire, label.0))),
            Entity::Curve(curve @ Curve3::Nurbs { .. }, label) => Some((*label, LengthSource::Curve(curve, None))),
            Entity::Edge(id) => {
                let edge = self.native.body.edges.get(*id).ok_or_else(|| ValueError::invariant("live length edge is missing"))?;
                let curve = self.native.body.curves3.get(edge.curve).ok_or_else(|| ValueError::invariant("live length curve is missing"))?;
                matches!(curve, Curve3::Nurbs { .. } | Curve3::Ellipse { .. }).then_some((edge.label, LengthSource::Curve(curve, Some(edge.range))))
            }
            _ => None,
        })
    }

    fn bound_length_source(&self, handle: &str, job: &WireLengthJob) -> Result<LengthSource<'_>, ValueError> {
        if job.owner != std::ptr::from_ref(self) as usize || job.revision != self.native.shape_mutation_revision {
            return Err(ValueError::invariant("original length source changed"));
        }
        self.length_source(handle)?.filter(|(label, _)| *label == job.label).map(|(_, source)| source)
            .ok_or_else(|| ValueError::invariant("original length source changed"))
    }

    /// 🥾️ Quotes one original wire row or bounded curved subdivision.
    pub fn wire_length_demand(&self, handle: &str, job: &WireLengthJob) -> Result<RetirementDemand, ValueError> {
        let mut demand = job.integration.demand(&self.native.body, self.bound_length_source(handle, job)?)?;
        demand.depth = demand.depth.max(self.native.original_intersection_curve_depth(handle)?);
        Ok(demand)
    }

    /// ⏱️ Advances the retained length by one grant and records its receipt.
    pub fn step_wire_length(&self, handle: &str, job: &mut WireLengthJob, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<f64>, Progress), ValueError> {
        let source = self.bound_length_source(handle, job)?;
        advance_measure(self.wire_length_demand(handle, job)?, effect, || job.integration.step(&self.native.body, source))
    }
}
