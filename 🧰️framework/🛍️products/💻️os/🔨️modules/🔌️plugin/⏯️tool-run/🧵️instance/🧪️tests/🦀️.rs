//! 🧪️ Tool runs reach every mounted instance through the registered instance extensions.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, MutationDiff, SpaceMember};
use protocol::{OpBinary, OpText};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_scene::SceneDoc;
use semio_framework_tool_run::TOOL_RUN_ACTION_IDS;
use semio_framework_plugin_tool_run::{ToolRunMemberEmissionOwner, ToolRunMemberOwners};
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;

#[test]
fn registered_extensions_route_every_tool_run_verb() {
    for action in TOOL_RUN_ACTION_IDS {
        assert!(ToolRunExtension::claims_action(action), "{action} is a tool run verb");
        assert!(<semio_framework_plugin_instance_app::InstanceExtensions as InstanceExtension>::claims_action(action), "{action} is routed by the registered list");
        assert!(!<semio_framework_plugin_instance_app::InstanceExtensions as InstanceExtension>::reads_history(action), "{action} never needs the document's history");
    }
}
