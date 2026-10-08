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
#[path = "../../♻️retirement/🏭️factory/🦀️.rs"]
mod factory_retirement;
pub use factory_retirement::{FactoryPayloadRetirement, FactoryRetirement, FactoryChildTickets, factory_arc_birth_bytes, factory_retirement_frame_bytes, factory_constructor_birth_bytes, factory_ticket_byte_demand, close_factory_ticket};
#[path = "../../🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;

#[path = "../../🏷️type/🦀️.rs"]
mod types;
pub use types::{ValueKind, ValueType};
