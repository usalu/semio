//! ♻️ Original generic retirement, standard owners and typed factory runtime.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_paged_list::list;
pub use semio_framework_value_retirement_contract as retirement_contract;
pub use retirement_contract::*;
pub use retirement_contract::{OriginalRetirementFailure,OriginalRetirementOutcome,settle_original_retirement_work,run_original_retirement_work,run_original_retirement_loan};
#[path="../../🦀️.rs"]
pub mod retirement;
pub use retirement::{RetireOwned,admit_retirement_cursor,retirement_cursor_birth_demand,owned_retirement_birth_bytes,admit_owned_retirement};

pub use retirement::turn::{advance_retirement_turn,RetirementReceipt,RetirementTurnError};
pub use retirement::arc_custody::{shared_retirement_birth_bytes,shared_retirement_allocation_bytes,admit_shared_retirement_into};
#[path="../../../⚠️refusal/♻️retirement/🦀️.rs"]
mod refusal_retirement;
artifact_retire_leaf!(RetainedCloneGrant);
artifact_retire_leaf!(RetirementDemand);
artifact_retire_leaf!(OriginalWorkBindingIdentity);
artifact_retire_leaf!(RetainedCloneProgress);
artifact_retire_leaf!(RetainedCloneStep);
artifact_retire_leaf!(NativeAllocationRequest);
artifact_retire_leaf!(NativeAllocationProgress);
artifact_retire_leaf!(ValueRefusalKind);

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

pub use retirement_contract::tickets;


#[cfg(test)]
pub use semio_framework_value_factory::{FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,FactoryChildTickets,FactoryChildSlot,FactoryRetirementTicket,FactoryProducerReply,FactoryProducerReplyComplete,factory_arc_birth_bytes,factory_retirement_frame_bytes,factory_constructor_birth_bytes};
#[cfg(test)]
pub use semio_framework_value_factory::owned::FactoryOwnedRetirement;



#[path="../../🚧️gate/🦀️.rs"] mod readiness_gate;
pub use readiness_gate::NativeRetirementGate;
