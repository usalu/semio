//! 📏️ AABB math and stage: the bounding box arithmetic and the boolean stage as a staged operation. See [`super`].

use super::*;

// #region 🔖️AabbMath
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_finite(bb: &AxisAlignedBox) -> bool {
    bb.min.x.is_finite() && bb.min.y.is_finite() && bb.min.z.is_finite() && bb.max.x.is_finite() && bb.max.y.is_finite() && bb.max.z.is_finite()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_dims(bb: &AxisAlignedBox) -> (f64, f64, f64) {
    (bb.max.x - bb.min.x, bb.max.y - bb.min.y, bb.max.z - bb.min.z)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_volume(bb: &AxisAlignedBox) -> f64 {
    let (w, d, h) = aabb_dims(bb);
    w * d * h
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_gap(a: &AxisAlignedBox, b: &AxisAlignedBox) -> f64 {
    let dx = gap_1d(a.min.x, a.max.x, b.min.x, b.max.x);
    let dy = gap_1d(a.min.y, a.max.y, b.min.y, b.max.y);
    let dz = gap_1d(a.min.z, a.max.z, b.min.z, b.max.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn gap_1d(a0: f64, a1: f64, b0: f64, b1: f64) -> f64 {
    if a1 < b0 {
        b0 - a1
    } else if b1 < a0 {
        a0 - b1
    } else {
        0.0
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_intersection(a: &AxisAlignedBox, b: &AxisAlignedBox) -> Option<AxisAlignedBox> {
    let min = Pnt3::new(a.min.x.max(b.min.x), a.min.y.max(b.min.y), a.min.z.max(b.min.z));
    let max = Pnt3::new(a.max.x.min(b.max.x), a.max.y.min(b.max.y), a.max.z.min(b.max.z));
    if min.x < max.x && min.y < max.y && min.z < max.z {
        Some(AxisAlignedBox { min, max })
    } else {
        None
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_union(a: &AxisAlignedBox, b: &AxisAlignedBox) -> AxisAlignedBox {
    AxisAlignedBox { min: Pnt3::new(a.min.x.min(b.min.x), a.min.y.min(b.min.y), a.min.z.min(b.min.z)), max: Pnt3::new(a.max.x.max(b.max.x), a.max.y.max(b.max.y), a.max.z.max(b.max.z)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_contains(outer: &AxisAlignedBox, inner: &AxisAlignedBox, tol: f64) -> bool {
    outer.min.x <= inner.min.x + tol && outer.min.y <= inner.min.y + tol && outer.min.z <= inner.min.z + tol && outer.max.x + tol >= inner.max.x && outer.max.y + tol >= inner.max.y && outer.max.z + tol >= inner.max.z
}

/// 🔀 Overlap test for the small `engine::Aabb` shape ([`face_aabb`]'s return type) rather than
/// [`AxisAlignedBox`] — inflated by `tol` on every side so a face pair whose supports are exactly
/// tangent still gets a chance to intersect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn aabb_overlap(a: &semio_framework_3d_brep_representation::contract::Aabb, b: &semio_framework_3d_brep_representation::contract::Aabb, tol: f64) -> bool {
    (0..3).all(|i| a.min[i] - tol <= b.max[i] + tol && b.min[i] - tol <= a.max[i] + tol)
}
// #endregion 🔖️AabbMath

// #region 🔖️Stage
/// 🔀 A boolean as a staged operation. The fast paths answer in one unit; the general engine runs
/// one [`BooleanJob`] unit per advance plus one finishing unit for its terminal transition.
pub struct BooleanStage {
    pub(super) job: Option<BooleanJob>,
    answered: Option<SolidId>,
    finished: bool,
}

impl BooleanStage {
    /// 🔀 Admits `a op b` against `body` (the fast paths run here).
    pub fn new(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Self, KernelError> {
        Ok(match boolean_job(body, a, b, op, tol, rec)? {
            BooleanAdmission::Answered(solid) => Self { job: None, answered: Some(solid), finished: false },
            BooleanAdmission::Job(job) => Self { job: Some(job), answered: None, finished: false },
        })
    }
}

impl StagedOperation for BooleanStage {
    fn progress(&self) -> StageProgress {
        match &self.job {
            None => StageProgress { done: usize::from(self.finished), total: 1, phase: "boolean" },
            Some(job) => {
                let progress = job.progress();
                StageProgress { done: progress.units_done + usize::from(self.finished), total: progress.units_total.max(progress.units_done) + 1, phase: progress.phase.tag() }
            }
        }
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        if let Some(solid) = self.answered {
            self.finished = true;
            return Ok(StageStep::Done(StageOutput::Solid(solid)));
        }
        let job = self.job.as_mut().ok_or_else(|| KernelError::Operation("boolean stage already finished".into()))?;
        match job.step_unbudgeted(body, rec, 1)? {
            BooleanStep::Working(_) => Ok(StageStep::Working),
            BooleanStep::Done(solid) => {
                self.finished = true;
                Ok(StageStep::Done(StageOutput::Solid(solid)))
            }
            BooleanStep::Cancelled(_) => Err(KernelError::Operation("boolean stage was cancelled".into())),
        }
    }
}
// #endregion 🔖️Stage
