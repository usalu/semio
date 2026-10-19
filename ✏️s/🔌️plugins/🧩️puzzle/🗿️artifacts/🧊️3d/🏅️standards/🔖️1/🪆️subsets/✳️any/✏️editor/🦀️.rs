//! structural-twin scene_snapshot document model its command/panel/window nodes mutate and render, the
//! 🧊️ Puzzle 3d play app — the plugin's 3d editor surface: its `ArtifactEditor` impl (dispatch-only), the
//! attraction resolver that keeps every attracted object's pose derived from its attracting root,
//! and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/🧊️main`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle3dPlaySnapshot` newtype over a bare
//! `serde_json::Value` scene_snapshot (see `semio_s_artifact_puzzle_3d::standards::v1::subsets::any::io::text::mutations`'s `🔖️ValueBridge`), not the typed
//! `Puzzle3dSnapshot` — the `Puzzle3dSceneSnapshot` model below is this app's own structural twin of it,
//! and each action emits the concrete typed mutation kinds of the gesture it carries (`🎮️commands/<group>`),
//! never a delta of two scenes.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use crate::editor::puzzle3d::commands::add_target_volume;
use crate::editor::puzzle3d::commands::apply_sun;
use crate::editor::puzzle3d::commands::close_vortex_suggestions;
use crate::editor::puzzle3d::commands::cycle_candidate;
use crate::editor::puzzle3d::commands::delete_attraction;
use crate::editor::puzzle3d::commands::delete_selection;
use crate::editor::puzzle3d::commands::delete_target_volume;
use crate::editor::puzzle3d::commands::duplicate_selection;
use crate::editor::puzzle3d::commands::engagement_abort;
use crate::editor::puzzle3d::commands::export_snapshot;
use crate::editor::puzzle3d::commands::engagement_control_select;
use crate::editor::puzzle3d::commands::engagement_input;
use crate::editor::puzzle3d::commands::engagement_repeat_last;
use crate::editor::puzzle3d::commands::engagement_submit;
use crate::editor::puzzle3d::commands::focus_selection;
use crate::editor::puzzle3d::commands::hover_suggestion;
use crate::editor::puzzle3d::commands::import_snapshot;
use crate::editor::puzzle3d::commands::open_import_snapshot;
use crate::editor::puzzle3d::commands::open_vortex_suggestions;
use crate::editor::puzzle3d::commands::register_brush_mesh;
use crate::editor::puzzle3d::commands::relocate_target_volume;
use crate::editor::puzzle3d::commands::rotate_selection;
use crate::editor::puzzle3d::commands::scale_selection;
use crate::editor::puzzle3d::commands::select_same_kind;
use crate::editor::puzzle3d::commands::set_automatic;
use crate::editor::puzzle3d::commands::set_brush_placement_contact_tolerance;
use crate::editor::puzzle3d::commands::set_camera;
use crate::editor::puzzle3d::commands::set_chunk_size;
use crate::editor::puzzle3d::commands::set_depth_variable;
use crate::editor::puzzle3d::commands::set_fill_count;
use crate::editor::puzzle3d::commands::set_kind_weight;
use crate::editor::puzzle3d::commands::set_manual;
use crate::editor::puzzle3d::commands::set_projection;
use crate::editor::puzzle3d::commands::set_proximity_radius;
use crate::editor::puzzle3d::commands::set_selectable_kind;
use crate::editor::puzzle3d::commands::set_selection_flag;
use crate::editor::puzzle3d::commands::set_snap_enabled;
use crate::editor::puzzle3d::commands::set_spacing;
use crate::editor::puzzle3d::commands::set_target_volume_flag;
use crate::editor::puzzle3d::commands::set_transform_gumball_flag;
use crate::editor::puzzle3d::commands::set_visible;
use crate::editor::puzzle3d::commands::set_vortex_direction;
use crate::editor::puzzle3d::commands::set_vortex_show;
use crate::editor::puzzle3d::commands::set_voxel_dims;
use crate::editor::puzzle3d::commands::target_brush_suggestions;
use crate::editor::puzzle3d::commands::translate_selection;
use crate::editor::puzzle3d::commands::world_relocate;
use crate::editor::puzzle3d::config::Puzzle3dConfig;
use crate::editor::puzzle3d::config::Puzzle3dConfigMutation;
use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::config::Puzzle3dConfigSetFillCount;
use crate::editor::puzzle3d::config::Puzzle3dConfigSetObjectKindWeights;
use crate::editor::puzzle3d::config::Puzzle3dConfigSetVortexKindWeights;
use crate::editor::puzzle3d::config::Puzzle3dConfigSetActiveExampleId;
use crate::editor::puzzle3d::modes::edit;
use crate::editor::puzzle3d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle3d::modes::edit::windows::main;
use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use crate::editor::puzzle3d::panels::catalogue;
use crate::editor::puzzle3d::panels::artifact;
use crate::editor::puzzle3d::panels::inspection;
use crate::editor::puzzle3d::panels::settings as settings_panel;
use crate::editor::puzzle3d::precompute::brush::BrushSuggestionsLink;
use crate::editor::puzzle3d::precompute::brush::BrushSuggestionsOwner;
use crate::editor::puzzle3d::precompute::Puzzle3dCollisionSession;
use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::presence;
use crate::editor::puzzle3d::presence::Puzzle3dPresence;
use crate::editor::puzzle3d::presence::Puzzle3dPresenceMutation;
use crate::editor::puzzle3d::terminology::puzzle3d_labels;
use crate::editor::puzzle3d::terminology::puzzle3d_localized;
use crate::editor::puzzle3d::terminology::puzzle3d_localized_phrase;
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::window as window_ownership;
use crate::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::Puzzle3dEngineCommand;
use crate::Puzzle3dSnapshot;
use semio_framework_plugin::kernel::ClipboardError;
use semio_framework_plugin::kernel::ClipboardFragment;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::kernel::PastePlacement;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::StepContext;
use semio_framework_plugin::mesh_from_kind;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID;
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use semio_framework_pack_json::from_json_str;
use semio_framework_pack_json::json;
use semio_framework_pack_json::object;
use semio_framework_pack_json::parse;
use semio_framework_pack_json::to_json_string;
use semio_framework_pack_json::to_string;
use semio_framework_pack_json::Object;
use semio_framework_pack_json::Value;
use semio_framework_plugin::app::EphemeralEmit;
use semio_framework_plugin::app::InteractionView;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::OnceLock;
use semio_s_artifact_puzzle_3d_mutations::standards::v1::subsets::any::schema::mutations::quat_normalize;
use semio_s_artifact_puzzle_3d_retained_action::PUZZLE3D_RETAINED_PAYLOAD_SCHEMA;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dAcceptSuggestionWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dAddBrushObjectWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dFocusSelectionWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dInstanceBinding;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dPrecomputeCommandWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dTransformWork;
use semio_s_artifact_puzzle_3d_retained_action::Puzzle3dWindowCommandWork;
use semio_s_artifact_puzzle_3d_retained_action::puzzle3d_retained_extent;
use semio_s_artifact_puzzle_3d_retained_action::puzzle3d_retained_reduce;
use {semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dCreateAttractionWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use {semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_view_session_key};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_DEFAULT_UTILITY, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_HOVER_CHANNEL, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_entity_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_fault_notices, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_argument, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_kind_ids, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_paste_operations, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_play_snapshot, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_selected_objects_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_upsert_catalog_rows, semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot};
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use semio_framework::kernel::UiDirtyScope;
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
use semio_framework_2d_compute::EngineHandles;
// 🎭️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (contract §2.1): `ArtifactEditor`
// replaces `ArtifactApp` as the authoring trait; `EditorApp<E>` is the runtime `ArtifactApp`
// adapter, needed by this file's own test harness (`VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>`).
// 🕹️ `InteractionView`'s canonical home is `semio_framework_plugin::app`; it is now also re-exported
// at that crate's root alongside `InteractionWrite`, and this path keeps the module-qualified spelling
// the other reference implementations (`process3d`, `generation3d`) use.

use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
//#region 🔖️Document

//#region 🔖️Quaternions
pub use {semio_s_artifact_puzzle_3d_model::quat_from_axis_angle, semio_s_artifact_puzzle_3d_model::quat_mul, semio_s_artifact_puzzle_3d_model::quat_rotate_vector};

//#endregion 🔖️Quaternions

//#region 🔖️PlayApp

//#region 🧵️RetainedCommands

#[path = "🧰️retained-commands/🦀️.rs"]
mod retained_commands;
pub use self::retained_commands::*;
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use self::store_preparation::*;
//#endregion 📬️StorePreparation

//#region 📜️ToolProofs
//#endregion 📜️ToolProofs

#[path = "✂️clipboard-job/🦀️.rs"]
mod clipboard_job;
pub use self::clipboard_job::*;

#[path = "🧩️play-app/🦀️.rs"]
mod play_app;
pub use self::play_app::*;

//#endregion 🔖️PlayApp

//#region 🔖️Manifest
#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use self::manifest::*;

//#endregion 🔖️Manifest


//#region 🌳️Modules
#[path = "🌉️wasm/🦀️.rs"]
pub mod wasm;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo_session {
        #[path = "📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod component;
        pub use component::*;
    }
}
//#endregion 🌳️Modules
