//! 🛑️ Fill contract: why a plan stops, the step context a planner transition runs under, the run events and the placed collision entry.

use super::*;

/// 🛑️ Why a fill plan stopped below its requested count — the only reasons a plan may stall with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillStall {
    /// 🕳️ The round enumerated no open vortex at all — every vortex is blocked or zero-weighted.
    NoOpenVortex,
    /// 🧩️ Open vortices existed, but no catalog kind is compatible with any of them.
    NoCompatibleKind,
    /// 🚧️ Candidates were built and every one of them was refused — the artifact has no room left.
    NoFreePlacement,
    /// 📄️ A fixed artifact page refused an owner: a visible stall, never a fault.
    ArtifactCapacity,
}

impl FillStall {
    pub const ALL: [Self; 4] = [Self::NoOpenVortex, Self::NoCompatibleKind, Self::NoFreePlacement, Self::ArtifactCapacity];

    /// 🏷️ The run reason the stall's `warning` step names.
    pub const fn reason(self) -> FillRunReason {
        match self {
            Self::NoOpenVortex => FillRunReason::NoOpenVortex,
            Self::NoCompatibleKind => FillRunReason::NoCompatibleKind,
            Self::NoFreePlacement => FillRunReason::NoFreePlacement,
            Self::ArtifactCapacity => FillRunReason::ArtifactCapacity,
        }
    }
}

/// 🏁️ How a fill plan ended: it reached its requested count, or it stalled with a declared reason. A plan cannot complete
/// without one, because [`FillJobStage::Complete`] carries it — there is no silent end to construct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillPlanEnd {
    Reached,
    Stalled(FillStall),
}

/// 🧊️ The planner's own spatial index answered for another owner: an invariant break the planner faults on.
pub(super) struct StaleSpatialIndex;

/// 🧭️ What one planner transition needs from its caller: the collision budget seam plus identity,
/// stage and fault payloads. The job context forwards everything; the fill run job
/// hands the planner a transition context whose fuel is the run's own candidate budget instead.
pub trait FillStepContext: CollisionStepContext {
    fn operation(&self) -> OperationId;
    fn generation(&self) -> Generation;
    fn set_stage(&mut self, label: &'static str);
}

impl FillStepContext for StepContext<'_> {
    fn operation(&self) -> OperationId {
        StepContext::operation(self)
    }

    fn generation(&self) -> Generation {
        StepContext::generation(self)
    }

    fn set_stage(&mut self, label: &'static str) {
        StepContext::set_stage(self, label);
    }
}

/// 🛰️ One observation the planner reports to a fill run job, in the order it happened.
#[derive(Clone, Debug, PartialEq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum FillRunEvent {
    Constructed { mesh_url: String, origin: [f64; 3], orientation: [f64; 4], scale: f32 },
    Refused(FillRunReason),
    /// 🎯️ A vortex whose every compatible candidate was refused: it is marked and never picked again.
    VortexMarked { position: [f64; 3] },
    Accepted,
    Abandoned,
    Discarded,
}

/// 📏️ Uniform trace scale of an object scale value: a number, the first component of a vector, else 1.
pub(super) fn fill_run_scale(scale: &Option<semio_framework_value::DslValue>) -> f32 {
    scale.as_ref().and_then(|value| value.as_f64().or_else(|| value.as_array().and_then(|values| values.first()).and_then(semio_framework_value::DslValue::as_f64))).map_or(1.0, |value| value as f32)
}

/// 🧱️ One already-placed object's collision footprint, kept alongside the plan so each new fill step
/// only has to test the candidate against bodies it can actually hit.
#[derive(Clone, semio_framework_value::RetireOwned)]
pub struct PlacedCollisionEntry {
    pub object_id: String,
    pub mesh_url: String,
    pub world: Pose3d,
}
