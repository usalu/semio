//! 🧪️ Unit 1 of the facade test host of `testing-laws`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework::kernel::ActorId;
use semio_framework_plugin_app_builder::App;
use pack::snapshot::ArtifactPack;
use store::ArtifactStore;
use store::ArtifactStoreInitializationAuthority;
use std::collections::BTreeSet;
use semio_framework_ui_runtime::ComponentTree;
use store::ConfigStore;
use semio_framework_value::DslValue;
use semio_framework::kernel::Effect;
use semio_framework::FaultCode;
use semio_framework::kernel::InvocationResult;
use protocol::LOCAL_ACTOR_ID;
use store::Mutation;
use semio_framework_plugin_decl_views::NoTransientMutation;
use semio_framework_plugin_testing_caller::OriginalFixtureCaller;
use store::SpaceMember;
use semio_framework_ui_contract::*;
use std::future::Future;
use store::MemberFactory;
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::ArtifactView;
use semio_framework_plugin_decl_views::ConfigView;
use semio_framework_plugin_decl_views::DraftView;
use semio_framework_plugin_decl_views::InteractionView;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;

#[path = "../../../../../🧪️tests/🧬️mutation-fixtures/🦀️.rs"]
pub mod mutation_fixture;
