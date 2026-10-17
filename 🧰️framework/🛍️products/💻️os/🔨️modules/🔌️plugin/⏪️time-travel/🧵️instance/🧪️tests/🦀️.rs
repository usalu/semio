//! 🧪️ History editing reaches every mounted instance through the registered instance extensions.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework::history_edit::{HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_COMMIT_ACTION_ID};
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, MutationDiff, SpaceMember};
use protocol::{OpBinary, OpText};
use semio_framework_plugin_time_travel::TimeTravelOwners;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;

#[test]
fn registered_extensions_route_every_history_edit_verb() {
    for action in [HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID] {
        assert!(TimeTravelExtension::claims_action(action), "{action} is a history-edit verb");
        assert!(<semio_framework_plugin_instance_app::InstanceExtensions as InstanceExtension>::claims_action(action), "{action} is routed by the registered list");
        assert!(<semio_framework_plugin_instance_app::InstanceExtensions as InstanceExtension>::reads_history(action), "{action} needs the document's history");
    }
}

#[test]
fn only_replay_cancellation_is_admitted_while_loading() {
    assert!(<semio_framework_plugin_instance_app::InstanceExtensions as InstanceExtension>::admitted_while_loading(HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID));
    assert!(!<semio_framework_plugin_instance_app::InstanceExtensions as InstanceExtension>::admitted_while_loading(HISTORY_EDIT_BEGIN_ACTION_ID));
}
