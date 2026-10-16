//! 🏛️ Studio laws: the per-command laws over the real studio app.

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
use semio_s_space_core::*;

#[cfg(test)]
#[path = "../../🎮️commands/🔌️connect-media-ports/🧪️tests/🔬️unit/🦀️.rs"]
mod connect_media_ports_tests;

#[cfg(test)]
#[path = "../../🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod delete_selection_tests;

#[cfg(test)]
#[path = "../../🎮️commands/📤️export-media/🧪️tests/🔬️unit/🦀️.rs"]
mod export_media_tests;

#[cfg(test)]
#[path = "../../🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs"]
mod node_graph_edit_tests;

#[cfg(test)]
#[path = "../../🎮️commands/🔍️open-instance/🧪️tests/🔬️unit/🦀️.rs"]
mod open_instance_tests;

#[cfg(test)]
#[path = "../../🎮️commands/🩹️patch-parameter/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_parameter_tests;

#[cfg(test)]
#[path = "../../🎮️commands/👥️presence-heartbeat/🧪️tests/🔬️unit/🦀️.rs"]
mod presence_heartbeat_tests;

#[cfg(test)]
#[path = "../../🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod set_active_example_tests;

#[cfg(test)]
#[path = "../../🎮️commands/⚙️set-active-panel-tab/🧪️tests/🔬️unit/🦀️.rs"]
mod set_active_panel_tab_tests;

#[cfg(test)]
#[path = "../../🎮️commands/🚀️spawn-app/🧪️tests/🔬️unit/🦀️.rs"]
mod spawn_app_tests;

#[cfg(test)]
#[path = "../../🎮️commands/✅️workflow-engagement-submit/🧪️tests/🔬️unit/🦀️.rs"]
mod workflow_engagement_submit_tests;
