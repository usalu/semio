//! 🔬️ Test host of `declarations`: runs `🔬️app-declarations-fixture` in the facade, where the names it uses are in scope.
//! 🧪️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-C Task 4 — the executable
//! specification: a synthetic TWO-standard, THREE-subset artifact (`crate::declarations::
//! fixture`) declared entirely through this tree, proving open→mutate→save works end to end
//! and that `io_route` finds the `standard 1 / subset strict` conformance-profile hop back to
//! `standard 1 / subset any`. Plugin waves reading this ticket copy this shape verbatim.

use crate::surface_contract::remember_subset_examples;
use semio_framework_ui_contract::schema_metadata;
use crate::plugin_program::AppFactory;
use semio_framework::AppDefinition;
use semio_framework::kernel::ActorId;
use semio_framework::kernel::ArtifactKind;
use semio_framework::kernel::CapabilityRequirement;
use semio_framework::kernel::Rights;
use semio_framework::kernel::Scope;
use std::collections::BTreeMap;
use semio_framework_artifact_reference::ArtifactKindId;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_artifact_reference::Dialect;
use semio_framework_artifact_reference::StandardId;
use semio_framework::surface_app_id;
use semio_framework::AppRole;
use crate::plugin_app::NoPluginApp;
use crate::app_builder::App;
use crate::app_builder::ExampleSource;
use crate::window_kits::AppActionRegistry;
use crate::app_contract::ArtifactApp;
use crate::plugin_app::PluginApp;
use crate::app_contract::mount_natural_file_actions;
use crate::artifact_identity::ArtifactLocale;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::artifact_identity::preflight_artifact_identity;
use crate::artifact_inference::ArtifactInferenceService;
use crate::artifact_inference::preflight_artifact_inference_services;
use crate::artifact_inference::register_artifact_inference_services;
use crate::artifact_instance::VcsArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::OwnerMutationRoster;
use crate::surface_contract::ArtifactCodecTableV1;
use crate::surface_contract::ArtifactEditor;
use crate::surface_contract::ArtifactViewer;
use crate::surface_contract::EditorApp;
use crate::surface_contract::ViewerApp;
use crate::surface_contract::artifact_codec_table;
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
#[allow(unused_imports)]
use crate::declarations::*;
include!("../../../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs");
