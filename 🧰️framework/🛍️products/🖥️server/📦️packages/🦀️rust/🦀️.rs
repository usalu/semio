//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.

// 🔀️ R3/R7: `async fn` in a public trait warns because auto trait bounds (`Send`) cannot be named
// on it — answered STRUCTURALLY here, not by the compiler's suggested `+ Send`. Every port in this
// crate (`Decider`, `PrincipalResolver`, `QueryHandler`, `DocumentAuthority`, `Saga`,
// `ServerModule`, `AuthorityStore`, `ProjectionStore`, `BlobStore`, `SessionStore`) is reached
// through a concrete type an instance names as an associated type of `ServerInstance`, so the
// concrete type at each call site is always known and `Send` (where it matters) falls out on its
// own. Never resolve this lint by adding `+ Send` to a trait method or by making it sync.
#![allow(async_fn_in_trait)]

pub use semio_framework_server_contract as contract;
pub use semio_framework_server_storage as storage;
pub use semio_framework_server_policy as policy;
pub use semio_framework_server_authority as authority;
pub use semio_framework_server_throttle as throttle;

// 🔀️ `#[dyn_enum]` expands to a `#[macro_export]`ed `macro_rules! __semio_dispatch_<Trait>` at the root of the
// crate that declares the trait. The storage, resolver and authority ports live in producer crates, so this
// façade re-exports their macros next to the gateway's own: a downstream instance closes any port of the
// product through `use server::__semio_dispatch_<Trait>;`.
pub use semio_framework_server_storage::{__semio_dispatch_AuthorityStore, __semio_dispatch_BlobStore, __semio_dispatch_ProjectionStore, __semio_dispatch_SessionStore};
pub use semio_framework_server_policy::__semio_dispatch_PrincipalResolver;
pub use semio_framework_server_authority::{__semio_dispatch_Decider, __semio_dispatch_Saga};

#[macro_use]
#[path = "../../🔨️modules/📡️gateway/🦀️.rs"]
pub mod gateway;

// 🔬️ The storage contract as executable assertions, generic over the backend. Compiled for any
// instance crate that enables the `conformance` feature on its DEV-dependency, among them the
// companion crate that holds the reference backends — never in a default build, so a durable
// instance is held against the very same bodies without the product shipping a test harness.
#[cfg(feature = "conformance")]
#[path = "../../🧪️tests/🔬️conformance/🦀️.rs"]
pub mod conformance;

#[cfg(test)]
#[path = "../../🧪️tests/🧩️instance/🦀️.rs"]
pub mod test_instance;

#[path = "../../🦀️.rs"]
mod component;
