//! 🏪️ Instance components of the plugin SDK: constructor state, stores, peer roster, action ingress and local interaction.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[path = "📥️staging/📮️action-ingress/🦀️.rs"]
pub mod action_ingress;
#[path = "📥️staging/⏪️time-travel/🦀️.rs"]
pub mod stores_time_travel;
#[path = "📥️staging/⏯️tool-run/🦀️.rs"]
pub mod stores_tool_run;
#[path = "📥️staging/📨️dispatch/🦀️.rs"]
pub mod stores_dispatch;
#[path = "📥️staging/🖼️render/🦀️.rs"]
pub mod stores_render;
#[path = "📥️staging/🚪️ingress/🦀️.rs"]
pub mod stores_ingress;
#[path = "📥️staging/🛠️tool-machine/🦀️.rs"]
pub mod stores_tool_machine;
#[path = "📥️staging/🧵️instance-root/🦀️.rs"]
pub mod stores_instance_root;
#[path = "📥️staging/🧷️command-routes/🦀️.rs"]
pub mod stores_command_routes;
#[path = "📥️staging/🧾️typed-operations/🦀️.rs"]
pub mod stores_typed_operations;
#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "📥️staging/🪆️child-test-view/🦀️.rs"]
pub mod stores_child_test_view;
#[path = "📥️staging/🪜️maintenance-ladder/🦀️.rs"]
pub mod stores_maintenance_ladder;
#[path = "📥️staging/🧱️components/🦀️.rs"]
pub mod stores_components;
