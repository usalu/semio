//! 🧪️ Unit 2 of the facade test host of `testing-laws`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework::kernel::ActorId;
#[cfg(test)]
use semio_framework_plugin_command::AlternativeFact;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_app_builder::App;
use store::os_store::store_schema::ArtifactCommand;
use pack::snapshot::ArtifactPack;
use store::ArtifactStore;
use store::ArtifactStoreInitializationAuthority;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactStoreInitializationJob;
use std::collections::BTreeMap;
use semio_framework_ui_runtime::ComponentTree;
use store::ConfigStore;
use semio_framework_value::DslValue;
use semio_framework::FaultCode;
use store::os_store::store_schema::HistoryLane;
use semio_framework::kernel::HybridLogicalTimestamp;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_ui_locale::Locale;
use store::Mutation;
use semio_framework::kernel::MutationId;
#[cfg(test)]
use semio_framework_plugin_decl_views::history_view::MutationView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoTransientMutation;
#[cfg(test)]
use semio_framework_plugin_window_kits::NodeGraphDeleteDispatch;
use semio_framework_plugin_testing_caller::OriginalFixtureCaller;
use store::{ReferenceSpaceMember, SpaceMember};
#[cfg(test)]
use semio_framework_plugin_time_travel::SupersedeLedger;
#[cfg(test)]
use semio_framework_plugin_time_travel::SupersedeRecord;
#[cfg(test)]
use semio_framework_plugin_time_travel::SupersedeRole;
use semio_framework_ui_locale::Terminology;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_initialization_job;
use store::os_store::document_materialize::create_document_envelope;
#[cfg(test)]
use semio_framework_plugin_window_kits::delete_selection;
#[cfg(test)]
use semio_framework_plugin_command::history_alternative_views;
#[cfg(test)]
use semio_framework_plugin_window_kits::node_graph_delete_selection_spec;
#[cfg(test)]
use semio_framework_plugin_window_kits::nothing_selected;
use semio_framework_plugin_time_travel as time_travel;
use semio_framework_ui_contract::*;
use std::future::Future;
use store::MemberFactory;
#[cfg(test)]
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
#[cfg(test)]
use semio_framework_plugin_composition_publication::PendingArtifactStorePublication;
#[cfg(test)]
use semio_framework_plugin_retained_command::typed_operation::PendingPublicationOutcome;
#[cfg(test)]
use semio_framework_plugin_window_config::WindowConfigMutation;
#[cfg(test)]
use semio_framework_plugin_window_transient::window_transient::WindowTransientMutation;
#[cfg(test)]
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerBundle;
#[cfg(test)]
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_decl_views::ArtifactView;
#[cfg(test)]
use semio_framework_plugin_decl_views::ConfigView;
#[cfg(test)]
use semio_framework_plugin_decl_views::DraftView;
#[cfg(test)]
use semio_framework_plugin_decl_views::InteractionView;
#[cfg(test)]
use semio_framework_plugin_interaction::UiAssemblyResult;
#[cfg(test)]
use semio_framework_plugin_window_config::WindowConfigOwner;
#[cfg(test)]
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
#[cfg(test)]
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
#[cfg(test)]
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;

#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️bounded-reload/🦀️.rs"]
mod bounded_reload_tests;
#[cfg(test)]
#[path = "../../../../../🧵️instance/👷️jobs/📥️staging/🖼️render/work-tests.rs"]
mod original_render_work_tests;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️composed-child-history/🦀️.rs"]
mod composed_child_history_tests;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🚫️empty-state/🦀️.rs"]
mod empty_state_contract;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️history-alternatives/🦀️.rs"]
mod history_alternatives_tests;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️history-label-reload/🦀️.rs"]
mod history_label_reload_tests;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️node-graph-delete-row/🦀️.rs"]
mod node_graph_delete_row_tests;
#[cfg(test)]
#[path = "../../../../../🧪️tests/♻️publication-retirement-authority/🦀️.rs"]
mod publication_retirement_authority;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️supersede-ledger/🦀️.rs"]
mod supersede_ledger_tests;
