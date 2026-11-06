//! ⏸️ Claimed original return custody preserves a real continuation independently of semantic success.
#[path="🎟️source/🦀️.rs"]
pub mod source;
pub(crate) use source::{OriginalReturnSource,OriginalReturnSourceInput,OriginalReturnBudgetReference,OriginalReturnBudget};

pub(crate) use source::{OriginalReturnClaimCredit,OriginalReturnWitness,claim_original_return,claim_original_sized_return,receive_original_return_witness,OriginalReturnPhase};

#[path="🎟️origin/🦀️.rs"]
mod origin;
pub use origin::{OriginalJobReturnOrigin,OriginalJobReturnReceiver,OriginalJobReturnClaim};

#[path="📦️frame/🦀️.rs"]
mod frame;

#[path="📦️capsule/🦀️.rs"]
mod capsule;

pub use capsule::{InteractiveJobStep,OriginalJobReturn,OriginalJobContinuation,OriginalJobContinuationView,OriginalJobOutcomeReturn,OriginalJobOutcome};
