#![cfg_attr(any(feature = "component-guest", feature = "component-extension-guest"), feature(linkage))]
// 🚫️ R3/R7 (see 📓️terra-dyn-enum-macro-report.md): `#[dyn_enum]`-annotated traits with `async fn`
// methods (PluginApp) trip rustc's "auto trait bounds cannot be specified" lint on every method.
// Answered structurally — Send comes from the concrete per-plugin enum at each call site, never
// from a bound — so the lint is silenced here, never by adding `+ Send` or making a method sync.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🦀️component.rs"]
pub mod component;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub use component::component_persistent_local;
pub use component::*;
