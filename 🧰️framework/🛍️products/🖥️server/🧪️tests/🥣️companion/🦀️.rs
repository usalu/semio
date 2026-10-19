//! 🥣️ Companion tests of the server product: the actor turn laws, the policy engine, the reference storage conformance and the gateway laws that need no private item.
//!
//! 🧭️ The tests reach the product through its public modules and the `conformance` feature; the reference instance they run against lives in the product's `🧩️instance` test file. The gateway laws that drive sibling-private functions run inside the product crate.
#![cfg(test)]
#![allow(async_fn_in_trait)]

pub use server::{__semio_dispatch_AuthorityStore, __semio_dispatch_BlobStore, __semio_dispatch_Decider, __semio_dispatch_PrincipalResolver, __semio_dispatch_ProjectionStore, __semio_dispatch_Saga, __semio_dispatch_SessionStore};
pub use server::{authority, conformance, contract, gateway, policy, storage, throttle};

#[path = "../🧩️instance/🦀️.rs"]
pub mod test_instance;

#[path = "../../🔨️modules/🎭️authority/🧪️tests/🔬️unit/🦀️.rs"]
mod authority_unit;

#[path = "../../🔨️modules/🛡️policy/🧪️tests/🔬️unit/🦀️.rs"]
mod policy_unit;

#[path = "../../🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs"]
mod storage_unit;

#[path = "🛰️gateway/🦀️.rs"]
mod gateway_unit;
