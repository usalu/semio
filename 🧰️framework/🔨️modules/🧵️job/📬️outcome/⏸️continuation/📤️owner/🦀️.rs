//! 📤️ Original Job return and protocol own their live context-dependent state machine.
use semio_framework_job_context::{StepContext,OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalTurnFailure,InteractiveJobCloseStep,JobOutcomeDescriptor,JobOutcomeBorrow,JobOutcomeAdmission,JobOutcomeView,close_original_retained_step,close_original_effect_step,receive_original_retained_work};
use std::task::Waker;
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind};
pub use semio_framework_job_context::original_return_source as source;
#[path="../📦️frame/🦀️.rs"]
mod frame;
#[path="../📦️capsule/🦀️.rs"]
mod capsule;
#[path="../🎟️origin/🦀️.rs"]
mod origin;
#[path="../../../🚦️clock-stride-and-job-protocol/🦀️.rs"]
mod protocol;
pub use origin::{OriginalJobReturnOrigin,OriginalJobReturnReceiver,OriginalJobReturnClaim};
pub use capsule::{InteractiveJobStep,OriginalJobReturn,OriginalJobContinuation,OriginalJobContinuationView,OriginalJobOutcomeReturn,OriginalJobOutcome};
pub use protocol::{InteractiveJob,JobThreadTransfer};
