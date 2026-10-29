//! 🗄️ Generic document version-graph algebra — Author/Change/Checkpoint/Alternative/ArtifactVcs,
//! `VcsError`, and collection-diff/operation helpers. Pure
//! data plus pure functions: nothing here touches a live document (that's `store::ArtifactStore`,
//! which depends on this crate — see `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`).

use protocol::ColdMutation;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use protocol::{Edit, Mutation, MutationApplyError, MutationDiff};

#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[path = "🌱️genesis/🦀️.rs"]
mod genesis;
pub use genesis::{ArtifactGenesis, ArtifactGenesisWitness};

#[path = "♻️retirement/🦀️.rs"]
mod retirement;

//#region 🔖️Schemas

#[path = "🪪️checkpoint-records/🦀️.rs"]
mod checkpoint_records;
pub use checkpoint_records::*;

#[path = "📚️history-page-stack/🦀️.rs"]
mod history_page_stack;
pub use history_page_stack::*;

#[path = "🪟️group-visibility/🦀️.rs"]
mod group_visibility;
pub use group_visibility::*;

#[path = "🗃️history-ledger/🦀️.rs"]
mod history_ledger;
pub use history_ledger::*;

#[path = "🌳️artifact-vcs/🦀️.rs"]
mod artifact_vcs;
pub use artifact_vcs::*;

//#endregion 🔖️Schemas

#[path = "🚨️vcs-errors/🦀️.rs"]
mod vcs_errors;
pub use vcs_errors::*;

#[path = "🧮️diff-and-mutation/🦀️.rs"]
mod diff_and_mutation;
pub use diff_and_mutation::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️fault-params/🦀️.rs"]
mod fault_params_tests;
//#endregion 🧪️Tests

/// 🚪️ Version-control representation boundaries.
#[path="🚪️io"]
pub mod io {
    /// 💾️ Version-control binary identity boundaries.
    #[path="💾️binary"]
    pub mod binary {
        #[path="🪪️checkpoint-identity/🦀️.rs"]
        pub mod checkpoint_identity;

        #[path="🪪️entity-identity/🦀️.rs"]
        pub mod entity_identity;

        #[path="🌱️genesis/🦀️.rs"]
        pub mod genesis;
    }

    /// 📝️ Version-control text boundaries.
    #[path="📝️text"]
    pub mod text {
        #[path="🌱️genesis/🦀️.rs"]
        pub mod genesis;

        #[path="🌿️vcs/🦀️.rs"]
        pub mod vcs;
    }
}
