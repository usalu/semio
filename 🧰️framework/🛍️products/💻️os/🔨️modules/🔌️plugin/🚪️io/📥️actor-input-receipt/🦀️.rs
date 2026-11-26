//! 🧾️ Shared original actor input failure projection and retained retirement.
use semio_framework_job::{OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_job_native_decode::OriginalNativeDecodeOutcome;
pub use semio_framework_pack_json_output_receiving::OriginalJsonOutputFailure;
use semio_framework_pack_json_output_receiving::{NativeReceivingFailure, OriginalJsonOutputOutcome};
use semio_framework_value::{retirement::controlled::ControlledRetirement, RetainedCloneStep, RetainedWorkOutcome, ValueError, ValueRefusalKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OriginalActorInputFailureIdentity {
    owner: usize,
    outcome: usize,
    settlement: usize,
    binding: OriginalSourceBinding,
}

impl OriginalActorInputFailureIdentity {
    /// 🪪️ Captures the borrowed caller and the exact retained failure branches.
    pub fn capture<O>(owner: &O, outcome: Option<&OriginalJsonOutputOutcome>, settlement: Option<&OriginalNativeDecodeOutcome<RetainedCloneStep>>, binding: OriginalSourceBinding) -> Self {
        Self { owner: std::ptr::from_ref(owner) as usize, outcome: outcome.map_or(0, |outcome| std::ptr::from_ref(outcome) as usize), settlement: settlement.map_or(0, |settlement| std::ptr::from_ref(settlement) as usize), binding }
    }
}

pub struct OriginalActorInputFailureView<'a> {
    pub identity: OriginalActorInputFailureIdentity,
    pub outcome: Option<&'a OriginalJsonOutputOutcome>,
    pub receiving: Option<&'a NativeReceivingFailure>,
    pub settlement: Option<&'a OriginalNativeDecodeOutcome<RetainedCloneStep>>,
    pub outcome_refusal: Option<&'a OriginalTurnFailure>,
    pub settlement_refusal: Option<&'a OriginalTurnFailure>,
}

fn projection_invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "input failure projection changed its retained branch")
}
impl OriginalActorInputFailureView<'_> {
    fn field(&self, index: usize) -> Result<&'static str, ValueError> {
        [
            self.outcome.and_then(|v| v.value.as_ref().err()).map(|_| "outcomeValue"),
            self.outcome_refusal.map(|_| "outcomeReceiving"),
            self.settlement.and_then(|v| v.value.as_ref().err()).map(|_| "settlementValue"),
            self.settlement_refusal.map(|_| "settlementReceiving"),
        ]
        .into_iter()
        .flatten()
        .nth(index)
        .ok_or_else(projection_invalid)
    }
}
impl semio_framework_job::OriginalDomainProjection for OriginalActorInputFailureView<'_> {
    fn projection_identity(&self) -> usize {
        self.identity.owner
    }
    fn projection_view(&self, path: &[usize]) -> Result<semio_framework_job::OriginalDomainView<'_>, ValueError> {
        let Some((&field, rest)) = path.split_first() else { return Ok(semio_framework_job::OriginalDomainView::IntrinsicObject((0..4).filter(|i| self.field(*i).is_ok()).count())) };
        match self.field(field)? {
            "outcomeValue" => match self.outcome.and_then(|v| v.value.as_ref().err()).ok_or_else(projection_invalid)? {
                OriginalJsonOutputFailure::Refused(error) => semio_framework_job::project_original_value_error(error, rest),
                OriginalJsonOutputFailure::Stored(_) => semio_framework_job::project_original_receiving_primary(self.receiving.ok_or_else(projection_invalid)?.primary(), rest),
            },
            "outcomeReceiving" => semio_framework_job::project_original_turn_failure(self.outcome_refusal.ok_or_else(projection_invalid)?, rest),
            "settlementValue" => semio_framework_job::project_original_value_error(self.settlement.and_then(|v| v.value.as_ref().err()).ok_or_else(projection_invalid)?, rest),
            _ => semio_framework_job::project_original_turn_failure(self.settlement_refusal.ok_or_else(projection_invalid)?, rest),
        }
    }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        let Some((&field, rest)) = path.split_first() else { return self.field(index) };
        match self.field(field)? {
            "outcomeValue" => match self.outcome.and_then(|v| v.value.as_ref().err()).ok_or_else(projection_invalid)? {
                OriginalJsonOutputFailure::Refused(error) => semio_framework_job::key_original_value_error(error, rest, index),
                OriginalJsonOutputFailure::Stored(_) => semio_framework_job::key_original_receiving_primary(self.receiving.ok_or_else(projection_invalid)?.primary(), rest, index),
            },
            "outcomeReceiving" => semio_framework_job::key_original_turn_failure(self.outcome_refusal.ok_or_else(projection_invalid)?, rest, index),
            "settlementValue" => semio_framework_job::key_original_value_error(self.settlement.and_then(|v| v.value.as_ref().err()).ok_or_else(projection_invalid)?, rest, index),
            _ => semio_framework_job::key_original_turn_failure(self.settlement_refusal.ok_or_else(projection_invalid)?, rest, index),
        }
    }
}

/// ♻️ Drives one paid retirement step of a retained owner through the caller's own original effect.
pub fn retire_step<U: semio_framework_value::retirement::RetireOwned>(owner: &mut ControlledRetirement<U>, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    let grant = cx.retained_grant();
    let body = grant.maximum_copy_bytes;
    let Some((poll, _)) = cx.with_original_effect(grant, |_, effect| owner.step(body, effect))? else { return Ok(()) };
    let turn = match poll {
        semio_framework_value::RetainedWorkPoll::Complete(turn) => turn,
        semio_framework_value::RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work },
    };
    semio_framework_job::receive_original_retained_work(cx, turn)?;
    Ok(())
}
