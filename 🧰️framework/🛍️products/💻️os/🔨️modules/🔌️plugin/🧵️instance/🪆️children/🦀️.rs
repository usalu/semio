//! 👶️ Instance component of the plugin SDK: child composition.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[path = "📥️staging/⏪️time-travel/🦀️.rs"]
pub mod children_time_travel;
#[path = "📥️staging/⏯️tool-run/🦀️.rs"]
pub mod children_tool_run;
#[path = "📥️staging/📨️dispatch/🦀️.rs"]
pub mod children_dispatch;
#[path = "📥️staging/🔌️plugin-app-impl/🦀️.rs"]
pub mod children_plugin_app_impl;
#[path = "📥️staging/🖼️render/🦀️.rs"]
pub mod children_render;
#[path = "📥️staging/🚪️ingress/🦀️.rs"]
pub mod children_ingress;
#[path = "📥️staging/🤝️group/🦀️.rs"]
pub mod children_group;
#[path = "📥️staging/🧵️instance-root/🦀️.rs"]
pub mod children_instance_root;
#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "📥️staging/🪆️child-test-view/🦀️.rs"]
pub mod children_child_test_view;
#[path = "📥️staging/🧱️components/🦀️.rs"]
pub mod children_components;
