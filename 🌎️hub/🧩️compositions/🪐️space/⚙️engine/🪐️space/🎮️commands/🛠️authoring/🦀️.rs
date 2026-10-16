//! 🧱️ Studio authoring commands: node, instance, parameter and selection edits over the workflow document.

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
use semio_s_space_core::*;

#[path = "../➕️add-parameter/🦀️.rs"]
pub mod add_parameter;
#[path = "../🔗️bind-parameter-field/🦀️.rs"]
pub mod bind_parameter_field;
#[path = "../❎️close-focused-instance/🦀️.rs"]
pub mod close_focused_instance;
#[path = "../📋️copy-app-instance/🦀️.rs"]
pub mod copy_app_instance;
#[path = "../🗑️delete-selection/🦀️.rs"]
pub mod delete_selection;
#[path = "../👯️duplicate-app-instance/🦀️.rs"]
pub mod duplicate_app_instance;
#[path = "../🚚️move-media-node/🦀️.rs"]
pub mod move_media_node;
#[path = "../✏️node-graph-edit/🦀️.rs"]
pub mod node_graph_edit;
#[path = "../🖱️node-graph-viewport/🦀️.rs"]
pub mod node_graph_viewport;
#[path = "../🔍️open-instance/🦀️.rs"]
pub mod open_instance;
#[path = "../📌️paste-app-instance/🦀️.rs"]
pub mod paste_app_instance;
#[path = "../🩺️patch-app-instances/🦀️.rs"]
pub mod patch_app_instances;
#[path = "../🩹️patch-parameter/🦀️.rs"]
pub mod patch_parameter;
#[path = "../🚮️remove-app-instance/🦀️.rs"]
pub mod remove_app_instance;
#[path = "../➖️remove-parameter/🦀️.rs"]
pub mod remove_parameter;
#[path = "../🏷️rename-app-instance/🦀️.rs"]
pub mod rename_app_instance;
#[path = "../🗂️reorganize-workflow/🦀️.rs"]
pub mod reorganize_workflow;
#[path = "../🚀️spawn-app/🦀️.rs"]
pub mod spawn_app;
#[path = "../🔓️unbind-parameter-field/🦀️.rs"]
pub mod unbind_parameter_field;
