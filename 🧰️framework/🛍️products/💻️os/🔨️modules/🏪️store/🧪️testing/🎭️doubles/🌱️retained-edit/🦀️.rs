//! 🌱️ A generic cold retained-clone edit of the demo snapshot: decides, inverts and applies any cold mutation family on an owned scratch projection, one funded turn per step.
//!
//! 🧭️ Shares the demo fixture behind native replay and one-item preparation for every mutation family of the demo document.

use semio_framework_os_store_testing_prelude::*;
use semio_framework_os_store_testing_fixtures::*;
use crate::*;
use semio_framework_os_spr::ColdMutation as _;
use semio_framework_os_store_snapshot_clone::{RetainedCloneEdit, RetainedCloneEditCursor, RetainedCloneEditStep};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_os_spr::apply_diff_cold;


#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ColdRetainedCloneEdit<M: 'static> {
    marker: std::marker::PhantomData<fn() -> M>,
}

pub struct ColdRetainedCloneEditCursor<M: 'static> {
    phase: u8,
    inverse: Option<Vec<M>>,
    active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    semantic_refusal: std::mem::ManuallyDrop<Option<semio_framework_os_spr::MutationApplyError>>,
    cancelled: bool,
    closing: bool,
}

impl<M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_os_spr::Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static> ColdRetainedCloneEditCursor<M> {
    fn close_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(retirement) = self.active_retirement.as_ref() {
            let mut demand = artifact_retirement_box_demands(retirement, body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "retained clone fixture close depth overflow"))?;
            return Ok(demand);
        }
        if self.semantic_refusal.is_some() { let mut demand = artifact_retirement_owned_birth_demands(&self.semantic_refusal)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "semantic refusal close depth overflow"))?; return Ok(demand); }
        if let Some(mutation) = self.inverse.as_ref().and_then(|inverse| inverse.last()) {
            return Ok(RetirementDemand { capacity_bytes: DemoMutationRetirementFactory.retirement_birth_bytes(mutation), depth: 2, ..Default::default() });
        }
        if let Some(inverse) = self.inverse.as_ref() {
            return Ok(RetirementDemand { release_bytes: inverse.capacity().checked_mul(std::mem::size_of::<M>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"inverse backing extent overflow"))?, depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
}

impl<M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_os_spr::Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static> RetainedCloneEdit<DemoSnapshot, M> for ColdRetainedCloneEdit<M> {
    type Cursor = ColdRetainedCloneEditCursor<M>;

    fn preflight(&self, _mutation: &M, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, &'static str> {
        Ok(ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 65_536 })
    }

    fn begin_demand(&self) -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 } }
    fn snapshot_cursor_birth_demand(&self) -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 } }
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError> {
        let progress=self.begin_demand().admit(grant)?;
        Ok((ColdRetainedCloneEditCursor { phase: 0, inverse: None, semantic_refusal: std::mem::ManuallyDrop::new(None), active_retirement: None, cancelled: false, closing: false },progress))
    }
}

impl<M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_os_spr::Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static> RetainedCloneEditCursor<DemoSnapshot, M> for ColdRetainedCloneEditCursor<M> {
    fn foreign_step_presence(&self)->bool{false}
    fn advance_demands(&self, _: RetainedCloneRef<'_, DemoSnapshot>, _: &DemoSnapshot, _: RetainedCloneRef<'_, M>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: if self.phase >= 2 { std::mem::size_of::<DemoSnapshot>() } else { 0 }, capacity_bytes: if self.phase == 1 { std::mem::size_of::<M>() } else { 0 }, depth: 1, ..Default::default() }) }
    fn advance(&mut self, base: RetainedCloneRef<'_, DemoSnapshot>, post: &mut DemoSnapshot, mutation: RetainedCloneRef<'_, M>, body:usize,grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
        if self.cancelled || self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress::default()));
        }
        if self.semantic_refusal.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original preparation retains its semantic refusal owner")); }
        if self.phase == 0 {
            self.phase = 1;
            return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: 0 }));
        }
        if self.phase == 1 {
            let bytes = std::mem::size_of::<M>();
            if grant.maximum_capacity_bytes < bytes { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
            let inverse = mutation.get().inverse(base.get())?;
            let valid = inverse.len() == 1;
            self.inverse = Some(inverse);
            if !valid { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "retained clone fixture expected one exact inverse row").with_retained_progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() })); }
            self.phase = 2;
            return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes, released_bytes: 0 }));
        }
        let copied_bytes = std::mem::size_of::<DemoSnapshot>();
        if grant.maximum_copy_bytes < copied_bytes { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
        let outcome = mutation.get().diff(base.get());
        if outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "retained clone fixture mutation rejected against its immutable base"));
        }
        let original = match protocol::apply_diff_cold(outcome.diff(), base.get()) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original retained mutation application refused")); } };
        *post = original;
        self.phase = 3;
        Ok(RetainedCloneEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
    }

    fn take_inverse(&mut self) -> Option<Vec<M>> {
        (self.phase == 3).then(|| self.inverse.take()).flatten()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "retained clone fixture close exceeds granted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active_retirement.is_some() {
            let step = artifact_retirement_box_close_step(&mut self.active_retirement, child)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.semantic_refusal.is_some() { return artifact_retirement_admit_owned(&mut self.semantic_refusal, &mut self.active_retirement, child); }
        if let Some(inverse) = self.inverse.as_mut() {
            if let Some(mutation) = inverse.pop() {
                return match DemoMutationRetirementFactory.retire_owned(mutation, child) {
                    Ok((owner, progress)) => {
                        self.active_retirement = Some(owner);
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                    Err((error, mutation)) => {
                        inverse.push(mutation);
                        Err(error)
                    }
                };
            }
            drop(self.inverse.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.semantic_refusal.is_none() && self.inverse.is_none() && self.active_retirement.is_none()
    }
}


/// 🏭️ Native replay preparation of one cold mutation family of the demo document.
pub fn cold_demo_replay_preparation_factory<M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_os_spr::Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot> + semio_framework_value::retirement::RetireOwned + ArtifactCanonicalJsonTree + Send + Sync + 'static>() -> std::sync::Arc<dyn semio_framework_os_store_replay_operation::ArtifactReplayPreparationFactory<DemoSnapshot, M>>
{
    std::sync::Arc::new(semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory::new(std::sync::Arc::new(ColdRetainedCloneEdit::<M> { marker: std::marker::PhantomData }), std::sync::Arc::new(DemoMutationRetirementFactory), std::sync::Arc::new(DemoSnapshotRetirementFactory), 64).expect("cold demo replay preparation factory"))
}
