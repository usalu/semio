//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.

// 🧬️ Crate declares async-fn-in-trait families (`Host`, `Inspector`, `Migration`, `Configuration`,
// `StatechartEvent`, `Machine`, …). The lint's Send-erasure concern is answered structurally, not by
// bound — R3/R7.
#![allow(async_fn_in_trait)]

#[cfg(target_arch = "wasm32")]
extern crate semio_framework_async as wasm_bindgen_futures;

// 🧯️ `extern crate self as` binds only at the crate root, so it lives here rather than in the domain
// file: `statechart!`-expanded code names this crate `fsm::…` both here and in consumer crates.
#[allow(unused_extern_crates)]
extern crate self as fsm;

#[path = "../../../🦀️.rs"]
mod component;
pub use component::*;
