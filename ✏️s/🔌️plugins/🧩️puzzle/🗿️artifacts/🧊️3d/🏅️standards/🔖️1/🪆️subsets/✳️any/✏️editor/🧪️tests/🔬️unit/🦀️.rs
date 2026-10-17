//! 🕹️ Puzzle3d editor surface laws — action roster, local interaction reads, operations, command envelopes, inspector, manifest, suggestions and the paged scene carrier.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_editor::*;
use semio_s_artifact_puzzle_3d_retained_action::{PUZZLE3D_RETAINED_PAYLOAD_SCHEMA, Puzzle3dAcceptSuggestionWork, Puzzle3dAddBrushObjectWork, Puzzle3dFocusSelectionWork, Puzzle3dInstanceBinding, Puzzle3dPrecomputeCommandWork, Puzzle3dTransformWork, Puzzle3dWindowCommandWork, puzzle3d_retained_extent, puzzle3d_retained_reduce};
use {semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dCreateAttractionWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use {semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_view_session_key};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_DEFAULT_UTILITY, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_HOVER_CHANNEL, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_entity_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_fault_notices, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_argument, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_kind_ids, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_paste_operations, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_play_snapshot, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_selected_objects_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_upsert_catalog_rows, semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot};
use semio_s_artifact_puzzle_3d_commands_document::world_relocate;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use semio_s_artifact_puzzle_3d_editor_modes as edit;
use semio_s_artifact_puzzle_3d_editor_modes::tools::fill as fill_tool;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_panels::{catalogue, artifact, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsOwner;
use semio_s_artifact_puzzle_3d_editor_presence as presence;
use semio_s_artifact_puzzle_3d_editor_presence::{Puzzle3dPresence, Puzzle3dPresenceMutation};
use semio_s_artifact_puzzle_3d_editor_terminology::{puzzle3d_labels, puzzle3d_localized, puzzle3d_localized_phrase, Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, StepContext};
use semio_framework_plugin::panel_tab_element_id;
use semio_framework_plugin::panel_tab_first_draggable_element_id;
use semio_framework_plugin::window_element_id;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractionWrite;
use semio_framework_onboarding::IntroductionDefinition;
use semio_framework_onboarding::IntroductionInteraction;
use semio_framework_onboarding::IntroductionPlacement;
use semio_framework_onboarding::IntroductionStepDefinition;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_2d::compute::EngineHandles;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use std::collections::{HashMap, HashSet};
use std::collections::{BTreeMap};
use semio_s_artifact_puzzle_3d_commands_document::delete_selection;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
#[path = "../🤖️agent-lane/🦀️.rs"]
mod agent_lane_tests;
#[path = "../🧵️retained-wiring/🦀️.rs"]
mod retained_wiring_tests;
#[path = "../../🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
mod modes_layout;
use context::*;
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::PluginApp;

include!("🕹️roster-and-read-laws/🦀️.rs");
include!("🧵️cursorized-route-laws/🦀️.rs");

include!("🧮️extent-and-step-laws/🦀️.rs");

include!("🎛️dispatch-and-operation-laws/🦀️.rs");

include!("🧾️inspector-and-manifest-laws/🦀️.rs");

include!("💡️suggestion-and-scene-laws/🦀️.rs");
