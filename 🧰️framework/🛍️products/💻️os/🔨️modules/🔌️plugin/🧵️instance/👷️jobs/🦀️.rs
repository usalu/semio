//! 🛠️ Instance components of the plugin SDK: pending reserved work, tool operations and tool machines.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;

#[path = "📥️staging/⏯️tool-run/🦀️.rs"]
pub mod jobs_tool_run;
#[path = "📥️staging/⏳️operation-progress/🦀️.rs"]
pub mod jobs_operation_progress;
#[path = "📥️staging/🎟️lease/🦀️.rs"]
pub mod jobs_lease;
#[path = "📥️staging/📨️dispatch/🦀️.rs"]
pub mod jobs_dispatch;
#[path = "📥️staging/🔁️document-replacement/🦀️.rs"]
pub mod jobs_document_replacement;
#[path = "📥️staging/🔒️close-ladder/🦀️.rs"]
pub mod jobs_close_ladder;
#[path = "📥️staging/🖼️render/🦀️.rs"]
pub mod jobs_render;
#[path = "📥️staging/🚪️ingress/🦀️.rs"]
pub mod jobs_ingress;
#[path = "📥️staging/🤝️group/🦀️.rs"]
pub mod jobs_group;
#[path = "📥️staging/🧵️instance-root/🦀️.rs"]
pub mod jobs_instance_root;
#[path = "📥️staging/🧷️command-routes/🦀️.rs"]
pub mod jobs_command_routes;
#[path = "📥️staging/🧾️typed-operations/🦀️.rs"]
pub mod jobs_typed_operations;
#[path = "📥️staging/🧱️components/🦀️.rs"]
pub mod jobs_components;
