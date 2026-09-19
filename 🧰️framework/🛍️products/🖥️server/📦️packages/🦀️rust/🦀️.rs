//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.

// 🔀️ R3/R7: `async fn` in a public trait warns because auto trait bounds (`Send`) cannot be named
// on it — answered STRUCTURALLY here, not by the compiler's suggested `+ Send`. Every port in this
// crate (`Decider`, `PrincipalResolver`, `QueryHandler`, `DocumentAuthority`, `Saga`,
// `ServerModule`, `AuthorityStore`, `ProjectionStore`, `BlobStore`, `SessionStore`) is reached
// through a concrete type an instance names as an associated type of `ServerInstance`, so the
// concrete type at each call site is always known and `Send` (where it matters) falls out on its
// own. Never resolve this lint by adding `+ Send` to a trait method or by making it sync.
#![allow(async_fn_in_trait)]

#[path = "../../🔨️modules/🧬️contract/🦀️.rs"]
pub mod contract;

// 🔀️ `#[macro_use]` on the four port modules: `#[dyn_enum]` expands to a `#[macro_export]`ed
// `macro_rules! __semio_dispatch_<Trait>` inside the module that declares the trait, and a
// `dyn_enum_close!` at a DIFFERENT module of the SAME crate can only reach it through textual
// scope — an absolute `crate::__semio_dispatch_<Trait>` path is the future-incompatible form
// rust-lang/rust#52234 rejects. Lifting the captures into the crate root's textual scope is what
// lets the test instance below close a set from outside the module that declared the trait, which
// is exactly the position every downstream instance crate is in.
#[macro_use]
#[path = "../../🔨️modules/🗄️storage/🦀️.rs"]
pub mod storage;

#[macro_use]
#[path = "../../🔨️modules/🛡️policy/🦀️.rs"]
pub mod policy;

#[macro_use]
#[path = "../../🔨️modules/🎭️authority/🦀️.rs"]
pub mod authority;

#[macro_use]
#[path = "../../🔨️modules/📡️gateway/🦀️.rs"]
pub mod gateway;

#[cfg(test)]
#[path = "../../🧪️tests/🧩️instance/🦀️.rs"]
pub mod test_instance;

// 🔬️ The storage contract as executable assertions, generic over the backend. Compiled for this
// crate's own tests and for any instance crate that enables the `conformance` feature on its
// DEV-dependency — never in a default build, so a durable instance is held against the very same
// bodies the reference backends are without the product shipping a test harness.
#[cfg(any(test, feature = "conformance"))]
#[path = "../../🧪️tests/🔬️conformance/🦀️.rs"]
pub mod conformance;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
