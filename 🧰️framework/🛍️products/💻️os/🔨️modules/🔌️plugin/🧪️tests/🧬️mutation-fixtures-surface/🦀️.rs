#[path = "../../🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::{SetSurfaceCount, SurfaceMutation};
crate::app_testing::mutation_fixture::wire::fixture_operation_text!(SurfaceMutation);

#[path = "../🎞️media-owner-context/🦀️.rs"]
mod media_owner_context;

// 🧪️ Proves the viewer helpers against a minimal editor/viewer pair sharing one dialect.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::app_testing::artifact_app_laws::new_registered_app;
use crate::app_testing::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates, close_registered_fixture_app, meta, new_app, new_viewer};
use crate::window_surface::{built_text_to_component_tree, UiAssemblyResult};
use crate::surface_contract::{ArtifactEditor, ArtifactViewer, EditorApp, ViewEmit};
use crate::tool_contract::ArtifactOwnedToolJobFactory;
use crate::tool_contract::ArtifactOwnedToolJobRequest;
use crate::tool_contract::ArtifactReservedToolInput;
use crate::tool_contract::ArtifactReservedToolJobRequest;
use crate::tool_contract::ArtifactToolCompletion;
use crate::tool_contract::ArtifactToolFactoryRegistry;
use crate::tool_contract::ArtifactToolPublicationContract;
use crate::tool_contract::ArtifactToolPublicationLane;
use crate::tool_contract::ArtifactReservedToolJob;
use crate::app_views::{ArtifactView, ConfigView, DraftView, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation};
use crate::child_view::ChildContentView;
use crate::app_emission::Emit;
use crate::history_view::HistoryView;
use semio_framework::{Media, MediaClass, MediaPayload, MediaType};
use crate::app_contract::NATURAL_FILE_PORT;
use crate::plugin_app::PluginApp;
use semio_framework::{MediaForm, ViewModel, REVERT_TO_COMMAND_ACTION_ID};
use protocol::MutationDiff;
use protocol::ColdMutationDiff;
use {semio_framework::action_bus,semio_framework::ActionKind,semio_framework_artifact_reference::Dialect,semio_framework::Fault,semio_framework::FaultOrigin,semio_framework::IconName,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework::ToolExecutionContract,semio_framework::ToolFactoryKey,semio_framework::ToolJobFactory,semio_framework::ToolOperationSpec};
use semio_framework_2d_compute::EngineHandles;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

include!("🗃️snapshot-diff-and-commands/🦀️.rs");
include!("🏭️registered-factory-and-natural-decode/🦀️.rs");
include!("🪟️editor-and-viewer-fixtures/🦀️.rs");
include!("👁️viewer-and-editor-lifecycle-laws/🦀️.rs");
include!("📄️natural-file-reserved-job-laws/🦀️.rs");
include!("🎫️controller-and-envelope-laws/🦀️.rs");
include!("🧵️retained-work-and-child-emission-laws/🦀️.rs");
