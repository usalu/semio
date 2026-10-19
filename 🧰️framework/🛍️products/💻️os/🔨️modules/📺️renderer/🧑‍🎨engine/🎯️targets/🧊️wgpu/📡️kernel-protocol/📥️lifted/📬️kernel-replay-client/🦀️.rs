//! 📬️ Kernel replay vocabulary: mounted product replay admission, recovery and refusal registries, typed operation result pages and the exchange and response slots.
use super::*;

#[path = "🔁️spawn-recovery/🦀️.rs"]
mod spawn_and_recovery;
pub use spawn_and_recovery::{
    mounted_product_replay_recovery_registry, MountedProductReplayRecoveryOwner,
    MountedProductReplayRecoveryToken, RawSpawnJobDisposition, PRODUCT_REPLAY_INPUT_BYTES,
    PRODUCT_REPLAY_KIND_BYTES, PRODUCT_REPLAY_PROFILE_COUNT,
};
use spawn_and_recovery::{RawSpawnJobOwner, RetainedMountedProductReplayRequest};

#[path = "🚫️request-refusal/🦀️.rs"]
mod request_and_refusal;
pub use request_and_refusal::{
    mounted_product_replay_refusal_registry, MountedProductReplayRefusalCause,
    MountedProductReplayRefusalToken, MountedProductReplayRequest,
};
use request_and_refusal::{MountedProductReplayRefusalPhase, RetainedRefusedProductReplay};

#[path = "🛂️admission-claim/🦀️.rs"]
mod admission_and_claim;
pub use admission_and_claim::{
    mounted_product_replay_profile, mounted_product_replay_worker_slot,
    MountedProductReplayAdmission, MountedProductReplayAdmissionPermit,
    MountedProductReplayAuthority, MountedProductReplayCheckpoint, MountedProductReplayClaim,
    MountedProductReplayExpected, RefusedProductReplay,
};

#[path = "📄️typed-result/🦀️.rs"]
mod typed_result_and_exchange;
pub use typed_result_and_exchange::{
    ExchangeOutcome, ExchangeSurfaceDocument, KernelOutcome, ResponseSlot, TypedOperationResultPage,
    TypedOperationResultToken, TYPED_OPERATION_RESULT_PAGE_BYTES,
};

//#region 🎠️KernelRuntime
//#region 🔖️Requests/Outcomes

//#region 📬️TypedOperationResultPage

#[cfg(test)]
#[path = "../../../../../🧪️tests/🗞️typed-result-page/🦀️.rs"]
mod typed_result_page_tests;

//#endregion 📬️TypedOperationResultPage

//#endregion

//#endregion 🎠️KernelRuntime
