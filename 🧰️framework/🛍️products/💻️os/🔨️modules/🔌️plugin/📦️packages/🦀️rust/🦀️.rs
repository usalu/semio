// 🧩️ `ArtifactEditor::Members`/`ArtifactViewer::Members` default to `store::NoMembers` so a surface
// author states a roster only when the app composes children — every other impl stays untouched.
#![feature(associated_type_defaults)]
// 🚫️ R3/R7 (see 📓️terra-dyn-enum-macro-report.md): `#[dyn_enum]`-annotated traits with `async fn`
// methods (PluginApp) trip rustc's "auto trait bounds cannot be specified" lint on every method.
// Answered structurally — Send comes from the concrete per-plugin enum at each call site, never
// from a bound — so the lint is silenced here, never by adding `+ Send` or making a method sync.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use semio_framework_diagnostic::encode_fault_bytes;

/// 🌱️ `app_commands!`'s generated enum spells its derives as `$crate::ToValue`/`$crate::FromValue`
/// so the path resolves identically for every invoking plugin crate regardless of what it has
/// imported — re-exported here at this crate's own root for that macro hygiene to find them.
pub use semio_framework_value_derive::{FromValue, ToValue};

/// 🫧️ The crates `transient_root!`/`window_transient_owners!` expand against, re-exported so the expansion resolves the same in
/// every invoking plugin crate (audit K3).
#[doc(hidden)]
pub use semio_framework_diagnostic as __diagnostic;
#[doc(hidden)]
pub use semio_framework_os_kernel as __kernel;
#[doc(hidden)]
pub use semio_framework_pack_json as __pack_json;
#[doc(hidden)]
pub use semio_framework_value as __value;

#[path = "../../🦀️.rs"]
pub mod component;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub use component::component_persistent_local;
pub use component::*;
