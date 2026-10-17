//! 🧪️ Unit 4 of the facade test host of `testing-laws`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework::kernel::ActorId;
use store::os_store::store_schema::ArtifactCommand;
use pack::snapshot::ArtifactPack;
use store::ArtifactStoreInitializationAuthority;
use std::collections::BTreeSet;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_plugin_command::CATALOGUE_EXAMPLE_ACTION_ID;
use semio_framework_value::DslValue;
use semio_framework::kernel::Effect;
use semio_framework::FaultCode;
#[cfg(test)]
use semio_framework_plugin_command::HISTORY_ROWS_BUILT;
#[cfg(test)]
use semio_framework_plugin_command::lifted_command::HISTORY_ROW_MUTATION_ROWS;
#[cfg(test)]
use semio_framework_plugin_decl_views::history_view::HistoryCommandFilter;
use semio_framework_ui_locale::Locale;
use store::Mutation;
use semio_framework::NOTE_SHELL_COMMAND_ACTION_ID;
use semio_framework_plugin_testing_caller::OriginalFixtureCaller;
use semio_framework::SET_HISTORY_COMMAND_FILTER_ACTION_ID;
use store::SpaceMember;
use semio_framework_ui_locale::Terminology;
#[cfg(test)]
use semio_framework_plugin_command::lifted_command::TimeTravelEditorPanel;
#[cfg(test)]
use semio_framework_plugin_command::lifted_command::TimeTravelPanel;
#[cfg(test)]
use semio_framework_plugin_window_kits_rich::tree_kit::TreeWindows;
#[cfg(test)]
use semio_framework_plugin_command::history_panel_mutation_row;
#[cfg(test)]
use semio_framework_plugin_decl_surface::replay_envelopes_fault;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_plugin_time_travel as time_travel;
#[cfg(test)]
use semio_framework_plugin_decl_app::viewer_rejects_action;
use semio_framework_ui_contract::*;
use std::future::Future;
use store::MemberFactory;
#[cfg(test)]
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_window_config::WindowConfigOwner;
#[cfg(test)]
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;

#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️time-travel-part-2/🦀️.rs"]
mod time_travel_tests_2;
