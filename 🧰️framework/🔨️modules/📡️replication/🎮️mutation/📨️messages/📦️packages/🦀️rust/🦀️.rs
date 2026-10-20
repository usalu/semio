//! 📨️ Mutation message ledgers glue — wiring only. Domain lives at the owner `🦀️.rs` files.

#[path = "../../🧵️compose/🦀️.rs"]
mod message_composition;
pub use message_composition::{ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource};
#[path = "../../♻️retire/🦀️.rs"]
mod message_ledger_retirement;
pub use message_ledger_retirement::{EditMessageLedgerRetirement, MutationMessageLedgerRetirement};
#[path = "../../📦️accumulate/🦀️.rs"]
mod message_accumulation;
pub use message_accumulation::{MutationMessageRetirement, OperationMessageDrain, ReplayMessageAccumulator};
#[path = "../../✂️clamp/🦀️.rs"]
mod message_selection;
pub use message_selection::{FinalMessageSelection,FinalMessageSource};
#[path = "../../📋️copy/🦀️.rs"]
mod message_copy;
pub use message_copy::MessageCopyCursor;
#[path = "../../../♻️retirement/🦀️.rs"]
mod replay_retirement;
pub use replay_retirement::{ArtifactReplayRetirementFactory, ReplayRetirement, registered_replay_retirement_factory};
#[path = "../../../🧳️prepared/🦀️.rs"]
mod prepared;
pub use prepared::{ArtifactReplayPrepared,retirement as prepared_retirement};

pub use semio_framework_replication_mutation::{ColdMutation, Mutation, MutationOutcome, MutationLeafDescriptor, MutationMessage, MUTATION_MESSAGE_ENTRY_BYTES};
pub use semio_framework_mutation_diff::{ApplyCapability, MutationDiff, DiffAlgebra, MutationApplyResult};
pub use semio_framework_value as value;
#[path="../../🔗️concat/🦀️.rs"] mod message_concat;
pub use message_concat::MutationMessageConcat;
#[path="../../📤️publication/🦀️.rs"] mod message_publication;
pub use message_publication::{MutationMessageBorrowedPublication, MutationMessagePublication, MutationMessagePublicationBatch, MutationMessagePublicationJoin};

#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;
