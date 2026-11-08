//! 🪙️ Original Job receiving budget algebra and its physical construction recipient.
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
#[path="🪙️budget/🦀️.rs"]
mod budget;
pub use budget::{OriginalJobConstructionRecipient,OriginalJobConstructionDriver,retained_delta,retained_remaining,retained_min,retained_is_subset,native_call_context};

#[path="🧾️scope/🦀️.rs"]
mod receiving_scope;
pub use receiving_scope::OriginalReceivingScope;

#[path="🎟️state/🦀️.rs"]
mod decode_state;
pub use decode_state::OriginalDecodeState;

#[cfg(any(test,feature="testkit"))]
#[path="🧪️testing/👁️progress/🦀️.rs"]
pub mod testkit;

#[path="🎟️work/🦀️.rs"]
mod work;
pub use work::{OriginalJobWorkRecipient,OriginalJobWorkDriver,OriginalNativeReceivingOutcome,OriginalReceivingOperation,native_call_result};

#[path="🎟️source/🏠️native/🦀️.rs"]
mod native_source_engine;
