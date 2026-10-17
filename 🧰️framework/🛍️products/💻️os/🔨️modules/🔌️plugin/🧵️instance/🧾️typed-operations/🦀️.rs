//! 🧾️ Typed operations of the mounted artifact instance: typed-operation publication, typed-command dispatch, configuration and the history panel.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[path = "📥️staging/🔌️plugin-app-impl/🦀️.rs"]
pub mod typed_ops_plugin_app_impl;
#[path = "📥️staging/🧵️instance-root/🦀️.rs"]
pub mod typed_ops_instance_root;
#[path = "📥️staging/♻️retirement/🦀️.rs"]
pub mod typed_retirement;
#[path = "📥️staging/🪆️child-group/🦀️.rs"]
pub mod typed_child_group;
#[path = "📥️staging/📬️publication/🦀️.rs"]
pub mod typed_publication;
#[path = "📥️staging/⌨️typing/🦀️.rs"]
pub mod typed_typing;
#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod typed_bundle;
