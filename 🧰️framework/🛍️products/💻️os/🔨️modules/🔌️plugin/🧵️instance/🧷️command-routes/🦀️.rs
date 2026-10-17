//! 🧷️ Command routes of the mounted artifact instance: framework-reserved routes, action dispatch and command dispatch.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[path = "📥️staging/🧵️instance-root/🦀️.rs"]
pub mod commands_instance_root;
#[path = "📥️staging/🧊️reserved-routes/🦀️.rs"]
pub mod command_reserved_routes;
#[path = "📥️staging/🚦️actions/🦀️.rs"]
pub mod command_actions;
#[path = "📥️staging/🗃️archive-merge/🦀️.rs"]
pub mod command_archive_merge;
#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod command_bundle;
