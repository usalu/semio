//! 📤️ Original Job return and protocol own their live context-dependent state machine.
use semio_framework_job_context::{StepContext,OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalTurnFailure,InteractiveJobCloseStep,JobOutcomeDescriptor,JobOutcomeBorrow,JobOutcomeAdmission,JobOutcomeView,close_original_retained_step,close_original_effect_step,receive_original_retained_work};
use std::task::Waker;
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind};
pub use semio_framework_job_context::original_return_source as source;
use semio_framework_job_context::{OriginalCallerReturnPlace,RetainedCloneGrant,RetainedCloneProgress as P};
use semio_framework_job_context::original_return_source::{OriginalReturnSourceInput,post_publisher};
use semio_framework_value_retained_clone::{RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_job_return_escrow_publication::{OriginalEscrowPublisher,OriginalEscrowPublication};
use semio_framework_job_payload::{OriginalReturnEscrowMessages as E,OriginalReturnEscrowTable,OriginalReturnEscrowIdentity};
use semio_framework_job_return_budget::OriginalReturnDelivery;
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
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
#[path="../🎟️source/📤️header/🦀️.rs"]
mod header;
pub use header::{finish_original_header,begin_original_header,write_original_header,with_original_frozen_header,accept_original_header,close_original_header_failure,abandon_original_unexposed_header,cancel_original_unexposed_header_loan,OriginalHeaderWriteStatus};
#[path="../🎟️source/🛡️guard/🦀️.rs"]
mod header_guard;
pub use header_guard::{prepare_original_header_guard,qualify_original_header_guard,observe_original_header_guard,release_original_header_guard,check_original_header_guard,OriginalReturnGuardPermit};
#[path="../🎟️source/📮️exposure/🦀️.rs"]
mod header_exposure;
pub use header_exposure::{OriginalHeaderExposurePermit,prepare_original_header_exposure,observe_original_header_exposure,cancel_original_header_exposure,expose_original_header};
