//! 🎛️ S Studio app — `ArtifactApp` impl, command dispatch, manifest (constitutional: ui + general,
//! merged at app level).
//!
//! 🕳️ Deviation from the usual "general"/"ui" split: the Studio app has no document type of its own —
//! its `ArtifactApp::Snapshot`/`Mutation` are `semio_framework_os::{WorkflowSnapshot, WorkflowMutation}`,
//! owned entirely by `framework/product/os/core/rs` (outside this plugin). There is therefore no
//! `🗿️artifacts/🪐️space` node anywhere in this crate — this file carries what would otherwise be the
//! artifact facade's constants (manifest/panel identifiers shared by `engine` config defaults and this
//! file's manifest/render dispatch), plus every DocumentHelpers used by 2+ command modules (a helper
//! that takes `&SpaceConfig` stays app-level no matter how many consumers it has, per the per-app
//! recipe's DocumentHelpers placement rule — artifacts must never depend on apps, and there is no
//! artifact here regardless).
//!
//! WIRING + DISPATCH ONLY beyond that: every command's real body lives in its own
//! `🎮️commands/<group>/🦀️.rs` payload module (see `app_commands!` below).

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
use semio_s_space_core::*;

use semio_hub_space_commands_authoring::node_graph_edit;
use semio_hub_space_commands_authoring::node_graph_viewport;
use semio_hub_space_commands_media::presence_heartbeat;
use semio_hub_space_commands_authoring::{add_parameter, bind_parameter_field, patch_parameter, remove_parameter, unbind_parameter_field};
use semio_hub_space_commands_authoring::{close_focused_instance, open_instance};
use semio_hub_space_commands_media::{compiled_dag_engagement_input, compiled_dag_engagement_submit, workflow_engagement_input, workflow_engagement_submit};
use semio_hub_space_commands_media::{connect_media_ports, disconnect_media_edge};
use semio_hub_space_commands_authoring::{copy_app_instance, delete_selection, duplicate_app_instance, move_media_node, paste_app_instance, patch_app_instances, remove_app_instance, rename_app_instance, reorganize_workflow, spawn_app};
use semio_hub_space_commands_media::patch_media_nodes;
use semio_hub_space_commands_media::{export_media, import_media, import_media_payload};
use semio_hub_space_commands_media::{export_studio_dsl, export_studio_pack, import_space_pack, import_space_pack_payload, open_space, set_active_example};
use semio_hub_space_commands_media::{go_home, navigate_virtual_file_system_node, set_active_panel_tab, set_app_registrations};
use semio_hub_space_foundation::config::{SpaceConfig, SpaceConfigMutation, ClipboardSetting};
use semio_hub_space_foundation::presence::{SpacePresence, SpacePresenceMutation};
use semio_hub_space_foundation::terminology::SStudioLabels;
use crate::parse_demo_space_document;
use semio_framework::{DslValue, ToolExecutionContract, ToolFactoryKey};
use semio_framework_os::workflow::ConnectPorts;
use semio_framework_os::{create_os_id, empty_workflow_snapshot, MediaContract, WorkflowEdge, WorkflowMutation, WorkflowSnapshot, S_WORKFLOW_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    app::InteractionView, app_commands, create_default_layout, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppOperationContext, ArtifactApp, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry,
    ArtifactView, CommandDefinition, ConfigView, DomainTopology, DraftView, Effect, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTarget,
    InteractionTopology, InteractiveJobClassification, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, WindowLayout, CLEAR_SELECTION_ACTION_ID,
    INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID,
};
use semio_framework_ui_locale::{Label, LocalizedLabel};
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use semio_hub_space_foundation::{S_PLAY_APP_ID, S_PLAY_CATALOGUE_BODY_KEY, S_PLAY_CONTROLLER_ID, S_PLAY_INSPECTOR_BODY_KEY, S_PLAY_INTERACTION_DOMAIN, S_PLAY_PARAMETERS_BODY_KEY, S_STUDIO_EXAMPLES};

#[path = "🧰️document-helpers/🦀️.rs"]
mod document_helpers;
pub use document_helpers::*;

#[path = "🕹️space-command/🦀️.rs"]
mod space_command;
pub use space_command::*;

#[path = "🏠️space-app/🦀️.rs"]
mod space_app;
pub use space_app::*;

#[path = "📜️space-manifest/🦀️.rs"]
mod space_manifest;
pub use space_manifest::*;

//#region 🧪️UnitTests
/// 🧪️ Shared test harness — every command-group test file needs this, added here first per the
/// per-app recipe so other nodes don't each re-derive it.
#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts

//#region 🔖️Windows
#[path = "."]
pub mod modes {
    #[path = "."]
    pub mod main {
        #[path = "🎭️modes/🌐️main/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod windows {
            #[path = "."]
            pub mod workflow {
                #[path = "🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "🎭️modes/🌐️main/🪟️windows/🔄️workflow/☑️options/🎯️active-instance/🦀️.rs"]
                    pub mod active_instance;
                }
            }

            #[path = "."]
            pub mod media_vfs {
                #[path = "🎭️modes/🌐️main/🪟️windows/🗂️media-vfs/🦀️.rs"]
                mod component;
                pub use component::*;
            }

            #[path = "."]
            pub mod compiled_dag {
                #[path = "🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}

#[path = "."]
pub mod panels {
    #[path = "📌️panels/🛍️catalogue/🦀️.rs"]
    pub mod catalogue;
    #[path = "📌️panels/🔍️inspection/🦀️.rs"]
    pub mod inspection;
    #[path = "📌️panels/🔢️parameters/🦀️.rs"]
    pub mod parameters;
}
//#endregion 🔖️Windows
