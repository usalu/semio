//! 🪟️ Resumable interactive job protocol facade over the step context custody core.
//!
//! See `README.md`.
pub use semio_framework_job_context_step::{
    BACKGROUND_LANE_FUEL, BACKGROUND_LANE_WALL_US, CancelToken, CancelTokenRetirement, CancelTokenRetirementError, Checkpoint, ClockStride,
    CommitCandidate, CommitPolicy, CommitValidation, DBG_POST_ON, Generation, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US,
    INTERACTIVE_STEP_CEILING_US, InteractiveJobCloseStep, InteractiveStage, JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_OPERATION_PAGES,
    JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_PROCESS_BYTES, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JobFault, JobOutcomeAdmission, JobOutcomeBorrow, JobOutcomeDescriptor,
    JobOutcomeDescriptorSlot, JobOutcomeKind, JobOutcomeSlot, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadAuthority,
    JobPayloadOperationLedger, JobPayloadPageSource, JobPayloadSlot, JobPayloadStep, JobPayloadStream, JobSequenceExhausted, Lane,
    MAINTENANCE_LANE_FUEL, MAINTENANCE_LANE_WALL_US, NativeEncoderWork, Operation, OperationId, OriginalCallerCloseReturn, OriginalCallerReturnPlace,
    OriginalCallerSource, OriginalCallerSourceLoan, OriginalCallerSourceParent, OriginalCancelRootAdmission, OriginalCancelRootIdentity, OriginalClockAction,
    OriginalClockAuthority, OriginalClockDeadline, OriginalClockDecisionFrame, OriginalClockDecisionStatus, OriginalClockObservation,
    OriginalClockProducer, OriginalClockReply, OriginalClockReplyComplete, OriginalClosureFailure, OriginalCreditedOutput, OriginalDomainFailure,
    OriginalDomainPrimary, OriginalDomainProjection, OriginalDomainReservation, OriginalDomainView, OriginalEscrowHeaderLoanCell,
    OriginalFailureContext, OriginalJobDecodeOwner, OriginalJobPayloadAdmission, OriginalNativeBorrowed, OriginalNativeLease,
    OriginalNativeReceivingOutcome, OriginalNativeRecipientIdentity, OriginalNativeRetiredWitness, OriginalNativeRetirement,
    OriginalPayloadSourceIdentity, OriginalReceivingPrimary, OriginalRetainedLoanIdentity, OriginalRetirementReceivingPrimary,
    OriginalSizedClaimReserve, OriginalSourceBinding, OriginalSourceCloseFailure, OriginalSourceTurn, OriginalSourceTurnIdentity,
    OriginalSourceTurnWitness, OriginalSystemClock, OriginalSystemClockContext, OriginalTurnFailure, OriginalTurnPrimary, OriginalWorkFailure,
    ParkedState, ProcessKind, RUNTIME_DIAGNOSTICS_ENV, ReplacementIdentity, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
    RetainedJobPayload, RetainedPayloadBuilder, RetirementDemand, RevisionId, SUSTAINED_OVERRUN_QUARANTINE_STEPS, StepBudget, StepContext,
    StepOutcome, USER_VISIBLE_LANE_FUEL, USER_VISIBLE_LANE_WALL_US, WorkerPool, WorkerPoolConfig, admit_original_domain_failure,
    allocate_operation_id, allocate_operation_id_in_slot, child_cancel_token, close_original_effect_step, close_original_granted_step,
    close_original_nested_step, close_original_quoted_retained_step, close_original_retained_effect_step, close_original_retained_step,
    close_original_retained_turn, close_original_source_effect_step, close_step_outcome_slot, dbg_post_take, default_now_ms, default_now_us,
    install_microsecond_clock, interactive_step_contract_violated, key_original_fault,
    key_original_receiving_primary, key_original_retirement_failure, key_original_retirement_receiving, key_original_source_close_failure,
    key_original_turn_failure, key_original_value_error, logical_now_us, microseconds_from_milliseconds, original_domain_failure_demand,
    original_receiving_scope_headroom, original_receiving_service_scope_headroom, original_return_source, project_original_fault,
    project_original_receiving_primary, project_original_retirement_failure, project_original_retirement_receiving,
    project_original_source_close_failure, project_original_turn_failure, project_original_value_error, receive_original_receiving_work,
    receive_original_retained_work, retained_work, root_cancel_token, runtime_diagnostics_enabled, set_runtime_diagnostics,
    step_outcome_slot_retirement_demands, step_outcome_slot_retirement_work, unquoted_receiving_work, validate_commit, validate_commit_with
};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_framework_job_context_step::install_browser_monotonic_clock;
use semio_framework_job_context_step::*;
use semio_framework_job_context_step::sealed::*;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind,RetainedWorkOutcome,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};

#[path="../🎟️source/🏭️admission/🦀️.rs"]
mod source_admission;
pub use source_admission::OriginalCallerSourceAdmission;
#[path="../🎟️source/🏭️close/🦀️.rs"]
mod source_close_producer;
pub use source_close_producer::{OriginalSourceCloseContext,OriginalSourceCloseProducer,source_loan_transfer_demand};
#[path="../🎟️source/👶️child/🦀️.rs"]
mod child_source;
pub use child_source::{OriginalCallerChildSource,OriginalChildTurnReturn};

#[path="../📦️owner/🦀️.rs"]
mod step_context_owner;
pub use step_context_owner::StepContextOwner;
#[path="../🎟️source/🏭️return-origin/🦀️.rs"]
mod return_origin;
pub use return_origin::{OriginalReturnBirthState,StepContextReturnOrigin};
#[path="../🪙️retirement-pump/🦀️.rs"]
mod retirement_pump;
pub use retirement_pump::{OriginalRetirementPump,OriginalRetirementTurn};

#[cfg(any(test,feature="testkit"))]
pub mod testkit {
    pub use semio_framework_job_context_step::testkit::{
        admit_return_origin, close_ledger, issue_initiating_event, ledger_demands, operation_mut, original_return_source,
        original_total_retained_start, payload_ledger, refuse_current_retained_receipt, renew_payload_page_grant, renew_pinned_turn, renew_turn,
        request_return_publisher, retained_grant_mut, retained_progress_mut, return_budget_is_bound, return_budget_is_inherited,
        return_budget_snapshot, return_source_is_borrowed, return_source_pinned, return_table_present, set_cancel_receipt_refusal,
        set_loan_receipt_refusal, source_return_budget_is_bound, take_issued_event_stock, with_native_decode_progress, with_original_work_refusals,
        with_retained_receipt_policy
    };
    use super::*;
    pub fn child_source_status(child:&OriginalCallerChildSource)->(bool,bool,usize,OriginalSourceBinding){child.test_status()}
}
