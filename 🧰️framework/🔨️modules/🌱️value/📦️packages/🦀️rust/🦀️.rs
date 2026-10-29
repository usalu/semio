//! 🌱️ Actual neutral value package; types are mounted once below replication and products.
extern crate self as semio_framework_value;
#[path = "../../🦀️.rs"]
pub mod value;
pub use value::*;
pub use serde;
pub use serde_json;
pub use semio_framework_io_base64::{base64_standard_encode,base64_standard_decode};
pub use semio_framework_value_derive::{CanonicalJsonTree, FromValue, NativeReceive, RetainedClone, RetainedEquality, RetireOwned, SnapshotProjection, ToValue};

pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,RetirementDemand,factory_ticket_demands,close_factory_ticket,close_factory_ticket_original,owned_retirement_birth_bytes,admit_owned_retirement,shared_retirement_birth_bytes,shared_retirement_allocation_bytes,admit_shared_retirement,artifact_retire_leaf,artifact_retirement_sequence,artifact_retire_struct,OriginalRetirementControl,RetirementSelection,select_retirement_cursor,select_factory_ticket,close_factory_ticket_controlled};
#[path = "../../🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;
pub use retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep, RetainedOwnedProjection};

#[path = "../../🔗️read/🦀️.rs"]
pub mod read;

#[path = "../../🏷️type/🦀️.rs"]
mod types;
pub use types::{ValueKind, ValueType};

#[cfg(feature="testkit")]
#[path="../../🧪️testing/🧾️wire/🦀️.rs"]
pub mod testing;

pub use semio_framework_value_retirement::{advance_retirement_turn,RetirementReceipt,RetirementTurnError};

pub use semio_framework_value_factory as factory;
pub use semio_framework_value_factory::{SnapshotRetirementFactory,ArtifactOwnedValueRetirementFactory,OwnedValueRetirementFactory,SharedValueRetirementFactory,FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,FactoryChildTickets,FactoryChildSlot,FactoryRetirementTicket,FactoryRetirementAdmissionError,factory_arc_birth_bytes,factory_retirement_frame_bytes,factory_constructor_birth_bytes};
pub use semio_framework_value_shared_issuance as shared_issuance;
/// 🔒️ Producer paths emitted by the derive macros; never a consumer API.
#[doc(hidden)]
pub mod __private {
    pub use semio_framework_value_snapshot_projection::snapshot_projection;
}




pub use semio_framework_value_factory::owned::FactoryOwnedRetirement;
pub use semio_framework_value_factory::boxed::{FactoryBoxedValue,FactoryBoxedPublication};

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod internals {
    pub use super::types::testkit as types;
}
#[path="../../📝️text/🚶️utf8/🦀️.rs"]
pub mod retained_utf8;
pub use retained_utf8::{RetainedUtf8Cursor,RetainedUtf8CursorStep,RetainedUtf8Buffer,RetainedUtf8Append};

#[path="../../📶️sort/🦀️.rs"]
pub mod sort;

pub use semio_framework_value_control_contract::OriginalRetirementCloseLoan;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;
