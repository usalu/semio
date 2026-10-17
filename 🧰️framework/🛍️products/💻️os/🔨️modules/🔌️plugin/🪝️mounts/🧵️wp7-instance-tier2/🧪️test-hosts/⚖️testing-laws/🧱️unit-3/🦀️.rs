//! 🧪️ Unit 3 of the facade test host of `testing-laws`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework::kernel::ActorId;
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_artifact_reference::ArtifactDialect;
use pack::snapshot::ArtifactPack;
use store::ArtifactStore;
use store::ArtifactStoreInitializationAuthority;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use semio_framework_value::DslValue;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework::FaultCode;
use store::os_store::store_schema::HistoryLane;
#[cfg(test)]
use semio_framework_plugin_decl_emission::HostEvent;
use semio_framework::kernel::HybridLogicalTimestamp;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use semio_framework::kernel::InvocationResult;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_ui_locale::Locale;
use store::Mutation;
use semio_framework::kernel::MutationId;
use semio_framework_plugin_testing_caller::OriginalFixtureCaller;
#[cfg(test)]
use semio_framework_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
use semio_framework::SET_HISTORY_COMMAND_FILTER_ACTION_ID;
use store::SpaceMember;
#[cfg(test)]
use semio_framework_plugin_time_travel::TIME_TRAVEL_TURN_WALL_US;
use semio_framework_ui_locale::Terminology;
#[cfg(test)]
use semio_framework_plugin_decl_emission::TransactionPhase;
use semio_framework::kernel::UiDirtyScope;
use store::os_store::document_materialize::create_document_envelope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_plugin_time_travel as time_travel;
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
#[path = "../../../../../🧪️tests/🧪️time-travel-part-1/🦀️.rs"]
mod time_travel_tests_1;
