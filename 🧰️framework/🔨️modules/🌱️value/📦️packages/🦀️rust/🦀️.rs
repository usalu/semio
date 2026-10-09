//! 🌱️ Actual neutral value package; types are mounted once below replication and products.
extern crate self as semio_framework_value;
#[path = "../../🦀️.rs"]
pub mod value;
pub use value::*;
pub use serde;
pub use serde_json;
pub use semio_framework_io_base64::{base64_standard_encode,base64_standard_decode};
pub use semio_framework_value_derive::{FactoryPayloadRetirement, FromValue, RetainedClone, RetireOwned, ToValue};

#[path = "../../♻️retirement/🧬️contract/🦀️.rs"]
mod retirement_contract;
pub use retirement_contract::*;
#[path = "../../♻️retirement/🦀️.rs"]
pub mod retirement;
pub use retirement::aliases::OriginalAliasBatch;
pub use retirement::turn::{advance_retirement_turn,RetirementReceipt,RetirementTurnError};
pub use retirement::{owned_retirement_birth_bytes, admit_owned_retirement};
pub use retirement::shared::{shared_retirement_birth_bytes, admit_shared_retirement};
#[path = "../../♻️retirement/🏭️factory/🦀️.rs"]
mod factory_retirement;
pub use factory_retirement::{FactoryPayloadRetirement, FactoryRetirement, FactoryAuthority, FactoryChildTickets, factory_arc_birth_bytes, factory_retirement_frame_bytes, factory_constructor_birth_bytes, factory_ticket_demands, close_factory_ticket};
pub use factory_retirement::owned::FactoryOwnedRetirement;
pub use factory_retirement::boxed::{FactoryBoxedValue,FactoryBoxedPublication};
#[path = "../../🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;

#[path = "../../🔗️read/🦀️.rs"]
pub mod read;

#[path = "../../🏷️type/🦀️.rs"]
mod types;
pub use types::{ValueKind, ValueType};
