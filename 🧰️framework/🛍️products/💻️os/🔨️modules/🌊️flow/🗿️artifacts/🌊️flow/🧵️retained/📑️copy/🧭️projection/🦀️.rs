//! 🧭️ A resident staged projection keeps one Flow snapshot admission's frame and action across turns, cancellation included.
use semio_framework_value::{RetainedAllocationEffect, RetainedWorkPoll, RetirementDemand, RetirementPollExit, ValueError, retained_clone::{OriginalEffectAction, OriginalProjectionFrame, OriginalProjectionFramePhase, OriginalSourceProjection, RetainedCloneProgress, RetainedCloneSourceAdmission, RetainedCloneStep}, retirement::RetireOwned};

/// 🧭️ The frame and unsettled action of one source admission's lifetime-bound projection.
pub(crate) struct StagedProjection<T: Sync + 'static, A: OriginalSourceProjection<T>> {
    frame: Option<OriginalProjectionFrame<T, A>>,
    action: Option<OriginalEffectAction<RetainedCloneStep, ValueError>>,
}

impl<T: Sync + 'static, A: OriginalSourceProjection<T>> Default for StagedProjection<T, A> {
    fn default() -> Self { Self { frame: None, action: None } }
}

impl<T: Sync + 'static, A: OriginalSourceProjection<T>> StagedProjection<T, A> {
    pub(crate) fn terminal_is_empty(&self) -> bool { self.frame.is_none() && self.action.is_none() }

    /// 🧭️ Selects the next projection stage of an admission waiting for or running its projection.
    pub(crate) fn stage<B: RetireOwned>(&self, admission: &RetainedCloneSourceAdmission<T, A, B>) -> Result<Option<u8>, ValueError> { admission.projection_stage(&self.frame) }

    /// 📏️ Quotes the physical demand and Work of one selected stage.
    pub(crate) fn quote<B: RetireOwned>(&self, admission: &RetainedCloneSourceAdmission<T, A, B>, stage: u8) -> Result<(RetirementDemand, u64), ValueError> { admission.projection_stage_quote(&self.frame, stage) }

    /// 🪡️ Runs one stage; an unsettled producer keeps its frame and action for the next turn.
    pub(crate) fn advance<B: RetireOwned>(&mut self, admission: &mut RetainedCloneSourceAdmission<T, A, B>, stage: u8, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        match admission.project_staged(&mut self.frame, &mut self.action, stage, effect) {
            RetainedWorkPoll::Pending { .. } => Ok(Default::default()),
            RetainedWorkPoll::Complete(turn) => turn.result.map(|step| step.progress()),
        }
    }

    /// 🚫️ Takes the recorded projection refusal out of the admission once it is funded.
    pub(crate) fn refusal<B: RetireOwned>(&mut self, admission: &mut RetainedCloneSourceAdmission<T, A, B>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<ValueError>, ValueError> {
        let mut refusal = None;
        admission.take_projection_failure(&mut refusal, effect).result?;
        Ok(refusal)
    }

    /// 📏️ Quotes the cancellation action that must precede the return of the admission's inputs; `None` once nothing is projected.
    pub(crate) fn close_quote<B: RetireOwned>(&self, admission: &RetainedCloneSourceAdmission<T, A, B>) -> Result<Option<(RetirementDemand, u64)>, ValueError> {
        if let Some(frame) = self.frame.as_ref() {
            return if frame.phase()? == OriginalProjectionFramePhase::ReturnOwner { Ok(Some((admission.return_projection_owner_demand(frame)?, 1))) } else {
                let stage = admission.projection_stage(&self.frame)?.ok_or_else(|| ValueError::invariant("open projection frame has no stage"))?;
                admission.projection_stage_quote(&self.frame, stage).map(Some)
            };
        }
        Ok(match admission.projection_result() {
            Some(true) => Some((admission.discard_projection_demand(), 1)),
            Some(false) => Some((admission.take_projection_failure_demand(), 1)),
            None => None,
        })
    }

    /// ♻️ Settles the open projection, then discards its answer; `None` once the admission may return its inputs.
    pub(crate) fn close<B: RetireOwned>(&mut self, admission: &mut RetainedCloneSourceAdmission<T, A, B>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<RetainedCloneProgress>, RetirementPollExit> {
        if let Some(frame) = self.frame.as_mut() {
            if frame.phase()? == OriginalProjectionFramePhase::ReturnOwner { return Ok(Some(admission.return_projection_owner(frame, effect).result?.progress())) }
            let stage = admission.projection_stage(&self.frame)?.ok_or_else(|| ValueError::invariant("open projection frame has no stage"))?;
            return match admission.project_staged(&mut self.frame, &mut self.action, stage, effect) {
                RetainedWorkPoll::Pending { .. } => Ok(Some(Default::default())),
                RetainedWorkPoll::Complete(turn) => Ok(Some(turn.result?.progress())),
            };
        }
        match admission.projection_result() {
            Some(true) => Ok(Some(admission.discard_projection(effect).result?.progress())),
            Some(false) => { let mut refusal = None; let progress = admission.take_projection_failure(&mut refusal, effect).result?.progress(); drop(refusal); Ok(Some(progress)) }
            None => Ok(None),
        }
    }
}
