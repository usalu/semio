//! 📡️ (dispatch-only), the structural-twin document model its command/panel/window nodes mutate and
//! 👯️ Puzzle 5d play app — the plugin's unified 2d+3d play app: its `ArtifactApp` impl
//! render, the shared scene/engine/brush helpers those nodes reach for, and the manifest that
//! stitches them together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/{◻2d,🧊️3d}`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle5dPlaySnapshot` newtype over a bare
//! `Value` document (see `crate::standards::v1::subsets::any::io::text::mutations`'s `🔖️ValueBridge`), not the
//! typed `Puzzle5dSnapshot` — the `Puzzle5dDocument` model below is this app's own structural twin
//! of it, and each action emits the concrete typed mutation kinds of the gesture it carries
//! (`🎮️commands/<group>`), never a delta of two documents.
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;

use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;

use crate::Puzzle5dSnapshot;
use crate::editor::puzzle5d::commands::apply_sun;
use crate::editor::puzzle5d::commands::cycle_brush_candidate;
use crate::editor::puzzle5d::commands::delete_selection;
use crate::editor::puzzle5d::commands::duplicate_selection;
use crate::editor::puzzle5d::commands::engagement_abort;
use crate::editor::puzzle5d::commands::engagement_control_select;
use crate::editor::puzzle5d::commands::engagement_input;
use crate::editor::puzzle5d::commands::engagement_repeat_last;
use crate::editor::puzzle5d::commands::engagement_submit;
use crate::editor::puzzle5d::commands::register_brush_mesh;
use crate::editor::puzzle5d::commands::rotate_selection;
use crate::editor::puzzle5d::commands::scale_selection;
use crate::editor::puzzle5d::commands::select_same_kind;
use crate::editor::puzzle5d::commands::set_brush_placement_contact_tolerance;
use crate::editor::puzzle5d::commands::set_camera;
use crate::editor::puzzle5d::commands::set_camera_2d;
use crate::editor::puzzle5d::commands::set_camera_3d;
use crate::editor::puzzle5d::commands::set_fill_count;
use crate::editor::puzzle5d::commands::set_grid_factor;
use crate::editor::puzzle5d::commands::set_grid_snap_enabled;
use crate::editor::puzzle5d::commands::set_kind_weight;
use crate::editor::puzzle5d::commands::set_lod_mode;
use crate::editor::puzzle5d::commands::set_selection_flag;
use crate::editor::puzzle5d::commands::set_suggestion_offset;
use crate::editor::puzzle5d::commands::target_brush_suggestions;
use crate::editor::puzzle5d::commands::translate_selection;
use crate::editor::puzzle5d::commands::world_relocate;
use crate::editor::puzzle5d::commands::set_grid_spacing;
use crate::editor::puzzle5d::commands::set_grid_visible;
use crate::editor::puzzle5d::commands::set_grip_direction;
use crate::editor::puzzle5d::commands::set_grip_show;
use crate::editor::puzzle5d::commands::set_lod_automatic;
use crate::editor::puzzle5d::commands::set_lod_depth_variable;
use crate::editor::puzzle5d::commands::set_lod_manual;
use crate::editor::puzzle5d::commands::set_projection;
use crate::editor::puzzle5d::commands::set_selectable_kind;
use crate::editor::puzzle5d::commands::set_transform_gumball_flag;
use crate::editor::puzzle5d::commands::add_target_volume;
use crate::editor::puzzle5d::commands::delete_target_volume;
use crate::editor::puzzle5d::commands::relocate_target_volume;
use crate::editor::puzzle5d::commands::set_target_volume_flag;
use crate::editor::puzzle5d::commands::set_voxel_dims;
use crate::editor::puzzle5d::commands::set_chunk_size;
use crate::editor::puzzle5d::commands::set_proximity_radius;
use crate::editor::puzzle5d::commands::accept_suggestion;
use crate::editor::puzzle5d::commands::close_vortex_suggestions;
use crate::editor::puzzle5d::commands::hover_suggestion;
use crate::editor::puzzle5d::commands::open_vortex_suggestions;
use crate::editor::puzzle5d::commands::export_snapshot;
use crate::editor::puzzle5d::commands::import_snapshot;
use crate::editor::puzzle5d::commands::open_add_part_dialog;
use crate::editor::puzzle5d::commands::open_import_snapshot;
use crate::editor::puzzle5d::config::Puzzle5dCamera2d;
use crate::editor::puzzle5d::config::Puzzle5dConfig;
pub use crate::editor::puzzle5d::config::Puzzle5dKindWeights;
use crate::editor::puzzle5d::config::Puzzle5dConfigMutation;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::config::Puzzle5dConfigSetObjectKindWeights;
use crate::editor::puzzle5d::config::Puzzle5dConfigSetVortexKindWeights;
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::panels::catalogue;
use crate::editor::puzzle5d::panels::artifact as artifact_panel;
use crate::editor::puzzle5d::panels::inspection;
use crate::editor::puzzle5d::panels::settings as settings_panel;
use semio_s_artifact_puzzle_3d_editor::Puzzle3dInstanceOperationOwner;
use crate::editor::puzzle5d::presence::Puzzle5dPresence;
use crate::editor::puzzle5d::presence::Puzzle5dPresenceMutation;
use crate::editor::puzzle5d::terminology::puzzle5d_labels;
use crate::editor::puzzle5d::terminology::puzzle5d_localized;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::ClipboardError;
use semio_framework_plugin::kernel::ClipboardFragment;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::kernel::PasteAnchor;
use semio_framework_plugin::kernel::PastePlacement;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactPresentation;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::InteractiveJobClassification;
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
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
// 🕹️ `InteractionView` — see 🧊️3d/🦀️.rs's identical import comment (missing top-level
// re-export from `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_job::InteractiveJob;
use semio_framework_job::Operation;
use semio_framework_job::StepContext;
use semio_framework_plugin::app::InteractionView;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_pack_json::parse;
use semio_framework_pack_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::LazyLock;
use semio_framework_2d_compute::EngineHandles;

pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
pub use entities::*;

pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
pub use view::*;

pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::catalog;
pub use catalog::*;

pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
pub use interaction::*;

#[path = "🖌️session/🦀️.rs"]
pub mod session;
use session::*;

//#endregion 🔖️Constants

//#region 🔖️Document

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[value(rename = "partKind")]
    pub part_kind: String,
    #[value(default)]
    pub anchor: Puzzle5dPartAnchor,
    #[value(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[value(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[value(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

pub use crate::standards::v1::subsets::any::schema::mutations::quat_from_axis_angle;

pub use crate::standards::v1::subsets::any::schema::mutations::quat_mul;

pub use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;
//#endregion 🔖️Document

//#region 🔖️Scene

//#endregion 🔖️Scene

//#region 🔖️Engine

//#endregion 🔖️Engine

//#region 🏷️Labels

//#endregion 🏷️Labels

//#region 🧬️InferredKinds

//#endregion 🧬️InferredKinds

//#region 🙈️SelectionFlags
//#endregion 🙈️SelectionFlags

//#region 🔖️Distribution

//#endregion 🔖️Distribution

//#region 🔖️CopyPaste

//#endregion 🔖️CopyPaste

#[path = "🍠️play-app/🦀️.rs"]
mod play_app;
pub use play_app::Puzzle5dPlayApp;

//#region 🔖️ContextMenu

//#endregion 🔖️ContextMenu

//#region 🔖️ActionContext

//#endregion 🔖️ActionContext

//#region 🔖️PlayApp

//#region 🧵️RetainedCommands

pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::retained;
use retained::*;

pub use semio_s_artifact_puzzle_5d_mut_any_6::editor::puzzle5d::gestures;
use gestures::*;

pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::kit;
use kit::*;

pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::dispatch;
use dispatch::puzzle5d_handle_action;

use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_inspector_nudge;

#[path = "🔧️play-app-retained-commands-and-tool-proofs/🦀️.rs"]
mod play_app_retained_commands_and_tool_proofs;
pub use play_app_retained_commands_and_tool_proofs::Puzzle5dRetainedCommandJobFactory;
use play_app_retained_commands_and_tool_proofs::{
    Puzzle5dHostConfigurationProofs, Puzzle5dRetainedCommandProofs,
};

#[path = "✍️artifact-editor/🦀️.rs"]
mod artifact_editor;

#[path = "🚀️app-assembly/🦀️.rs"]
mod app_assembly;
use app_assembly::{puzzle5d_fault_notices, puzzle5d_flag_value_argument};
pub use app_assembly::create_puzzle5d_app;

//#endregion 🧵️RetainedCommands

//#endregion 🔖️PlayApp

//#region 🔖️Manifest

// 🗂️ `Puzzle5dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-5d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle5dPlayApp>()` on
// `crate::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle5d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone. The 5d mesh export/import OS-host registration
// (`register_mesh_io()`/`puzzle5d_document_from_mesh`) was never rewired to a real `.setup()` caller
// after the artifacts-only-plugin-architecture migration (`🧩️puzzle/🦀️.rs`'s `plugin()`
// builder chain has no `.setup()` call at all) and was deleted as dead code (ticket
// 26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS) — same fate as puzzle3d's
// sibling mesh bridge. Mesh export/import should be re-derived from `io_dispatch`'s real
// `ComposerEntry` chain if/when this bridge is needed again.
//#endregion 🔖️Manifest

//#region 🧪️UnitTests

//#endregion 🧪️UnitTests

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{PUZZLE_5D_SCHEMA};

//#region 🔖️ArtifactKind
/// 🗿️ The `5d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "5d.puzzle".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("5D Puzzle", "5D-Puzzle"),
        source_format: "puzzle.5d".into(),
        component_kind: "puzzle5d".into(),
        dimension: "5d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Design },
        schema: PUZZLE_5D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.png".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4
