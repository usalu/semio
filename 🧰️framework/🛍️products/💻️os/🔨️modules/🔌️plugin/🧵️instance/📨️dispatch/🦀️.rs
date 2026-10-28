//! 🐤️ Instance use-cases of the plugin SDK: emit dispatch, reserved emit producers, host events and typed command admission over the dispatch bundle.
//!
//! 🧭️ WP7 root of `semio-framework-plugin-instance-dispatch`: the sources are staged under `📥️staging` and mounted by the instance parent until the crate is flipped.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[path = "📥️staging/🎟️producers/🦀️.rs"]
pub mod dispatch_producers;
#[path = "📥️staging/📤️emit/🦀️.rs"]
pub mod dispatch_emit;
#[path = "📥️staging/📣️host-events/🦀️.rs"]
pub mod dispatch_host_events;
#[path = "📥️staging/⌨️typed-command/🦀️.rs"]
pub mod dispatch_typed_command;
#[path = "📥️staging/🤝️group-history/🦀️.rs"]
pub mod dispatch_group_history;
#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod dispatch_bundle;

#[path="📥️staging/📤️emit/🎟️original/🦀️.rs"]
pub mod original_emission;
