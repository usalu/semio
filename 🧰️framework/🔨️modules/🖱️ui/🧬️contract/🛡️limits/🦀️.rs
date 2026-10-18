//! 🛡️ Quotas, `validate_snapshot`, and the one shared transactional `apply_patch`.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! `apply_patch` is the single most important function in the crate: the React DOM store, the GPU
//! frontend, and every future renderer all apply patches through this exact code, which is what makes
//! them agree. It is totally transactional — ops apply to a shadow draft, never to the caller's state,
//! and on ANY rejection the caller's state is byte-for-byte unchanged. Untrusted plugin documents flow
//! through here, so [`UiDocumentLimits`] are a security boundary, not a nicety: an oversized or
//! malformed document is rejected before it costs more than an O(1) or O(node count) check.

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_ui_contract::*;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[path = "📏️document-limits-and-text-estimates/🦀️.rs"]
mod document_limits_and_text_estimates;
pub use document_limits_and_text_estimates::{
    patch_byte_estimate, UiDocumentLimits, RETAINED_UI_INTAKE_SLICE_STEPS,
    RETAINED_UI_INTAKE_STEPS_PER_NODE, TREE_INLINE_TOOLBAR_BUTTONS,
};
use document_limits_and_text_estimates::{
    component_is_finite, component_number_range_is_valid, component_snaps_are_valid,
    component_text_bytes, is_section, op_text_bytes, row_target_is_valid, tree_detail_is_valid,
    tree_inline_toolbar_is_valid, tree_section_header_toolbar_is_valid,
};

#[path = "🚫️contract-violations-and-validation/🦀️.rs"]
mod contract_violations_and_validation;
pub use contract_violations_and_validation::{
    UiContractViolation, UiContractViolations, UI_DOCUMENT_VIOLATIONS,
};
#[cfg(test)]
pub use contract_violations_and_validation::validate_snapshot;
#[cfg(test)]
use contract_violations_and_validation::validate_state;

#[path = "🩹️patch-rejection-and-apply-arena/🦀️.rs"]
mod patch_rejection_and_apply_arena;
pub use patch_rejection_and_apply_arena::{PatchRejection, UiPatchApplyStep, UI_PATCH_APPLY_SLOTS};
use patch_rejection_and_apply_arena::{
    with_ui_patch_apply_arena, UiPatchApplyArena, UiPatchApplyHandle, UiPatchApplyPhase,
    UiPatchRetirement, UiPatchValidateFrame,
};

#[path = "🏭️patch-apply-producer/🦀️.rs"]
mod patch_apply_producer;
pub use patch_apply_producer::UiPatchApplyProducer;

#[path = "🎁️patch-apply-outcomes-and-retirement/🦀️.rs"]
mod patch_apply_outcomes_and_retirement;
pub use patch_apply_outcomes_and_retirement::{
    close_ui_patch_owner_one, QuotaKind, UiPatchApplyOutcome, UiPatchApplyRejected,
};
use patch_apply_outcomes_and_retirement::{
    retire_patch_one, retire_snapshot_one, retire_validation_seen_one,
};

#[path = "🔧️patch-ops-and-quotas/🦀️.rs"]
mod patch_ops_and_quotas;
#[cfg(test)]
pub use patch_ops_and_quotas::apply_patch;
use patch_ops_and_quotas::apply_op;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path="🧪️tests/🧩️components/🦀️.rs"]
mod component_integration;

#[cfg(test)]
#[path="🧪️tests/🗺️surface/🦀️.rs"]
mod surface_integration;

#[cfg(test)]
#[path="🧪️tests/🧬️conformance/🦀️.rs"]
mod conformance;
#[cfg(test)]
#[path="🧪️examples/🦀️.rs"]
mod conformance_examples;
