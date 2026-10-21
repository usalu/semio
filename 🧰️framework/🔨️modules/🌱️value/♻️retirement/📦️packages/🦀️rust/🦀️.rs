//! ♻️ Original generic retirement, standard owners and typed factory runtime.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_paged_list::list;
#[path="../../🧬️contract/🦀️.rs"]
mod retirement_contract;
pub use retirement_contract::*;
#[path="../../🦀️.rs"]
pub mod retirement;
pub use retirement::{RetireOwned,owned_retirement_birth_bytes,admit_owned_retirement};

pub use retirement::turn::{advance_retirement_turn,RetirementReceipt,RetirementTurnError};
pub use retirement::arc_custody::{shared_retirement_birth_bytes,shared_retirement_allocation_bytes,admit_shared_retirement};
#[path="../../../⚠️refusal/♻️retirement/🦀️.rs"]
mod refusal_retirement;
artifact_retire_leaf!(RetainedCloneGrant);
artifact_retire_leaf!(RetainedCloneProgress);
artifact_retire_leaf!(ValueRefusalKind);

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[path="../../🎟️tickets/🦀️.rs"]
mod tickets;
pub use tickets::{factory_ticket_demands,close_factory_ticket};

#[cfg(test)]
pub use semio_framework_value_factory::{FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,FactoryChildTickets,FactoryChildSlot,FactoryRetirementTicket,FactoryRetirementAdmissionError,factory_arc_birth_bytes,factory_retirement_frame_bytes,factory_constructor_birth_bytes};
#[cfg(test)]
pub use semio_framework_value_factory::owned::FactoryOwnedRetirement;
