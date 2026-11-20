//! 🧷️ Live provenance checks for the retained analytic preview job.

use super::*;
pub use semio_framework_3d_brep_mesh_io::PrimitivePreviewJob;
use semio_framework_mesh_engine::MeshData;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

impl Brep {
    /// 🌉️ Admits only live immutable constructor provenance into direct analytic preview.
    pub fn admit_primitive_preview(&self, handle: &str, tolerance: f64) -> Result<Option<PrimitivePreviewJob>, ValueError> {
        if let Some((source,bore))=self.live_bore_preview(handle){
            return PrimitivePreviewJob::new_bore(source,bore,tolerance).map(Some);
        }
        let Some((source, provenance)) = self.native.live_primitive_preview(handle) else { return Ok(None) };
        let Some(first_label) = provenance.preview_first_label else { return Ok(None) };
        PrimitivePreviewJob::new(source, provenance.primitive, first_label, tolerance).map(Some)
    }

    /// 🧭️ Quotes the live source check and next preview buffer event.
    pub fn primitive_preview_demand(&self, handle: &str, job: &PrimitivePreviewJob) -> Result<RetirementDemand, ValueError> {
        let mut demand = job.demand();
        if !job.is_done() {
            demand.depth = demand.depth.max(self.native.live.next_contains_depth_demand(handle)?);
            demand.depth = demand.depth.max(self.native.body.solids.next_remove_depth_demand(job.source())?);
        }
        Ok(demand)
    }

    /// 🔍️ Rechecks native provenance before a geometrically bounded original preview sample.
    pub fn step_primitive_preview(&self, handle: &str, job: &mut PrimitivePreviewJob, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<MeshData>, Progress), ValueError> {
        if !retained_clone_grant_admits(effect.grant(), self.primitive_preview_demand(handle, job)?) {
            return Ok((None, Progress::default()));
        }
        let live=if job.is_bore(){self.live_bore_preview(handle).map(|(source,_)|source)}
            else{self.native.live_primitive_preview(handle).map(|(source,_)|source)};
        if live!=Some(job.source()) {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "primitive preview source no longer has live constructor provenance"));
        }
        job.step(effect)
    }
}
