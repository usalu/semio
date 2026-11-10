//! 🧳️ Retained mutation glue — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_mutation_diff::{DiffAlgebra, MutationApplyResult, MutationDiff};
use semio_framework_replication_mutation::{ColdMutation, Mutation, MutationLeafDescriptor, MutationMessage, MutationOutcome, MUTATION_MESSAGE_ENTRY_BYTES};
use semio_framework_replication_mutation_messages::MutationMessageLedgerRetirement;

pub use semio_framework_mutation_diff::ApplyCapability;
pub use semio_framework_value as value;

/// 🔏️ Required granular semantic preparation and central grant admission.
#[path = "../../🦀️.rs"]
pub mod retained;
pub use retained::{RetainedMutation, RetainedMutationPreparation, RetainedMutationLeaf, RetainedMutationLeafPreparation, RetainedMutationOwner, RetainedMutationCloseContext, RetainedFieldPreparation, RetainedFieldChange, RetainedRecordChange, RetainedRecordChangeField, RetainedRecordFieldProjection, RetainedRecordFieldChange, RetainedRecordChangeOwner, boxed_record_field, RetainedByteSpan, RetainedKeyLookup, RetainedKeyedRow, RetainedKeyMatch, RetainedRowPatch, RetainedRowPatchFields, RetainedPatchSource, RetainedRowPatchApply, RetainedDiagnosticSpec, RetainedDiagnosticTarget, RetainedDiagnosticPreparation, RetainedMutationDecisionStep, RetainedMutationInverseStep, RetainedMutationDecision, RetainedDiff, RetainedDiffFragment, RetainedDiffSequence, RetainedDiffApply, RetainedDiffOwner, RetainedDiffAdmission, RetainedDiffPreflightAdvance, RetainedDiffPreflightStep, retained_diff_birth_demand, begin_retained_diff, preflight_retained_diff_into, recover_preflight_retained_diff_into, commit_retained_diff, retained_mutation_birth_demand, begin_retained_mutation, decide_retained_mutation_into, recover_decide_retained_mutation_into, inverse_retained_mutation_into, recover_inverse_retained_mutation_into, retained_mutation_demand_fits};

#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
