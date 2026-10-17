//! 🖥️ Drawing editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum
//! and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window
//! render in `🎭️modes/✏️edit/🪟️windows/🖼️canvas`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`.
//! This file is a routing table: `handle` → `DrawingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_DEFAULT_UTILITY;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_INTERACTION_DOMAIN;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_INTERACTION_GRANULARITY;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_POINT_DOMAIN;
use semio_s_artifact_draw_drawing_editor_interaction::drawing_manifest_action;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_s_artifact_draw_drawing_editor_commands::nudge_selection::{nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast, nudge_selection_down, nudge_selection_down_fast};
use semio_s_artifact_draw_drawing_editor_tool::{DrawingGesturePreview, DrawingSession};
use semio_s_artifact_draw_drawing_editor_commands::{import_image, add_layer, combine_boolean, commit_document, delete_layer, delete_selection, drop_layer_kind, duplicate_layer, engagement_input, engagement_submit, export_document, edit_selection, edit_path, edit_fill, move_layer, patch_layer, patch_layers, set_active_example, set_camera, set_camera_zoom, load_document_json, set_selected_opacity, set_snapshot, toggle_layer_visible};
use semio_s_artifact_draw_drawing_editor_tool::{canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_move, canvas_pointer_up};
use semio_s_artifact_draw_drawing_editor_tool as canvas_pointer_down;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas as canvas_window;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::config::DrawingCanvasWindowConfig;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::transient::DrawingCanvasWindowTransient;
use semio_s_artifact_draw_drawing_editor_interaction::panels::{catalogue as catalogue_panel, layers as layers_panel, properties as properties_panel};
use semio_s_artifact_draw_drawing_editor_interaction::presence::{DrawingPresence, DrawingPresenceMutation};
use semio_s_artifact_draw_drawing_editor_interaction::terminology::DrawingPlayLabels;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_scene::identity::SceneIdentity;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA;
use semio_framework_job_registry::{FixedOperationOwner};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
use semio_framework_plugin::WindowEngagementStatus;
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;
use std::collections::HashMap;

use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::DRAWING_PLAY_BODY_COMPOSITE;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::DRAWING_PLAY_WINDOW_CANVAS;
use semio_s_artifact_draw_drawing_editor_interaction::panels::catalogue::DRAWING_PLAY_BODY_CATALOGUE;
use semio_s_artifact_draw_drawing_editor_interaction::panels::layers::DRAWING_PLAY_BODY_LAYERS;
use semio_s_artifact_draw_drawing_editor_interaction::panels::properties::DRAWING_PLAY_BODY_PROPERTIES;

//#region 🔖️Constants

/// 🧰️ The utility armed for THIS window. The React host keeps utilities per window
/// (`ViewModel::active_utility_by_window_id`, keyed by window instance id) and only mirrors the
/// ACTIVE window's utility into the flat `active_utility_id`; a gesture dispatched into a pane that
/// is not the shell's active window therefore carried `None` and every rectangle drag ran as a
/// marquee select (ticket 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17). Resolution order: the
/// addressed window's entry, the focused window's entry, the flat field, the default.
pub fn drawing_active_utility(view: &semio_framework_plugin::ViewModel) -> &str {
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or(DRAWING_DEFAULT_UTILITY)
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `DrawingPlayApp::Command` — the SOLE dispatch surface for drawing's own behavior, covering every
    /// action `create_drawing_app` declares. Field shapes mirror each action's real `args` object.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum DrawingCommand for DrawingSnapshot, DrawingMutation, NoConfig, NoConfigMutation, ctx = DrawingSession {
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "commitDocument" as "commit-document" => commit_document::CommitDocument,
        "loadDocumentJson" as "document-json" => load_document_json::LoadDocumentJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setSelectedOpacity" as "selected-opacity" => set_selected_opacity::SetSelectedOpacity,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "addLayer" as "add-layer" => add_layer::AddLayer,
        "dropLayerKind" as "drop-layer-kind" => drop_layer_kind::DropLayerKind,
        "moveLayer" as "move-layer" => move_layer::MoveLayer,
        "deleteLayer" as "delete-layer" => delete_layer::DeleteLayer,
        "duplicateLayer" as "duplicate-layer" => duplicate_layer::DuplicateLayer,
        "toggleLayerVisible" as "toggle-layer-visible" => toggle_layer_visible::ToggleLayerVisible,
        "combineBoolean" as "combine-boolean" => combine_boolean::CombineBoolean,
        "patchLayer" as "patch-layer" => patch_layer::PatchLayer,
        "patchLayers" as "patch-layers" => patch_layers::PatchLayers,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasDoubleClick" as "canvas-double-click" => canvas_double_click::CanvasDoubleClick,
        "canvasCommitDraft" as "canvas-commit-draft" => canvas_commit_draft::CanvasCommitDraft,
        "canvasEscape" as "canvas-escape" => canvas_escape::CanvasEscape,
        "exportDocument" as "export-document" => export_document::ExportDocument,
        "editSelection" as "edit-selection" => edit_selection::EditSelection,
        "editPath" as "edit-path" => edit_path::EditPath,
        "editFill" as "edit-fill" => edit_fill::EditFill,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "nudgeSelectionLeft" as "nudge-selection-left" => nudge_selection_left::NudgeSelectionLeft,
        "nudgeSelectionLeftFast" as "nudge-selection-left-fast" => nudge_selection_left_fast::NudgeSelectionLeftFast,
        "nudgeSelectionRight" as "nudge-selection-right" => nudge_selection_right::NudgeSelectionRight,
        "nudgeSelectionRightFast" as "nudge-selection-right-fast" => nudge_selection_right_fast::NudgeSelectionRightFast,
        "nudgeSelectionUp" as "nudge-selection-up" => nudge_selection_up::NudgeSelectionUp,
        "nudgeSelectionUpFast" as "nudge-selection-up-fast" => nudge_selection_up_fast::NudgeSelectionUpFast,
        "nudgeSelectionDown" as "nudge-selection-down" => nudge_selection_down::NudgeSelectionDown,
        "nudgeSelectionDownFast" as "nudge-selection-down-fast" => nudge_selection_down_fast::NudgeSelectionDownFast,
        "importImage" as "import-image" => import_image::ImportImage,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.

//#endregion 🔖️Commands

//#region 🌉️ActionBridge
/// 🌉️ Host action `{action, args}` → typed `DrawingCommand` (ticket 26/09/05/DRAW-PLUGIN-END-TO-END,
/// after forms' bridge). The React/wgpu shells send camelCase argument keys and JSON numbers; every
/// `🎮️commands/*` payload derives `FromValue` over its own snake_case field names, so this boundary
/// folds each key into both spellings, restores integral floats, prints structured `value`/`json`
/// arguments into the `String` wire fields that carry JSON text, and decodes. Without it the trait
/// default refuses every app action (`setActiveExample`, `addLayer`, …) as "not framework-reserved".
//#endregion 🌉️ActionBridge

//#region 🧵️GestureOperationJobs

#[path = "🖐️gesture-owners/🦀️.rs"]
mod gesture_owners;
pub use gesture_owners::*;

#[path = "⚙️gesture-jobs/🦀️.rs"]
mod gesture_jobs;
pub use gesture_jobs::*;

//#endregion 🧵️GestureOperationJobs

//#region 🧵️BoundedCommands

#[path = "🧾️bounded-commands/🦀️.rs"]
mod bounded_commands;
pub use bounded_commands::*;

#[path="🎮️commands/📤️export-document/🧵️export/🦀️.rs"]
pub mod export_job;
use export_job::DrawingExportCommandJobFactory;
#[path="🎮️commands/📥️import-image/🧵️admission/🦀️.rs"]
pub mod image_job;
use image_job::DrawingImageCommandJobFactory;
#[path="🎮️commands/✏️edit-path/🧵️simplify/🦀️.rs"]
pub mod path_job;
use path_job::DrawingPathCommandJobFactory;

//#endregion 🧵️BoundedCommands

//#region 📬️StorePreparation

#[path = "💾️store-preparation/🦀️.rs"]
mod store_preparation;
use store_preparation::*;

//#endregion 📬️StorePreparation

//#region 🧾️ProofCatalogs

#[path = "🔏️proof-catalogs/🦀️.rs"]
mod proof_catalogs;
use proof_catalogs::*;

//#endregion 🧾️ProofCatalogs

//#region 🔖️DrawingPlayApp

#[path = "🖥️play-app/🦀️.rs"]
mod play_app;
pub use play_app::*;

//#endregion 🔖️DrawingPlayApp

//#region 🔖️Io

/// 🔌️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 4: anything returning `AppIo` or
/// referencing an app type lives in `🎛️apps/<app>/`). This app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors the `2d.drawing` `ArtifactKindSpec` literal `create_drawing_app`
/// already declares via `.artifact_kind(...)` (schema/media type/export+import formats copied
/// verbatim), plus the app-specific `vector:out` port (see `drawing_vector_out_port` below).
pub fn drawing_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        artifact_schema: DRAWING_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![drawing_vector_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "2d.drawing".into(), name: "2D Drawing".into(), dimension: "2d".into(), component_kind: "drawing".into() },
    }
}

/// 🔌️ `vector:out` — the drawing document's current vector content, exported as SVG (workflow port
/// surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). Reuses the existing `2d.drawing`
/// kind (already declared by `create_drawing_app`'s `.artifact_kind(...)`) rather than minting a
/// duplicate — `kind_id` just pins this port to that same catalog entry. `Many`/optional: a
/// consumer (e.g. raster's Vector→Raster-converted `image:in`) may connect before the canvas has
/// any content, or fan out to several consumers at once.
pub fn drawing_vector_out_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "vector:out".into(),
        label: "Vector".into(),
        direction: semio_framework::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: Some("2d.drawing".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Exports the current drawing document as an SVG `Media` payload for the `vector:out` port —
/// reuses `crate::standards::v1::subsets::any::io::drawing_document_to_svg` (the same semio/drawing↔svg bridge the
/// export-svg shell path uses), so there is exactly one SVG renderer.
pub fn drawing_vector_media(doc: &DrawingSnapshot) -> Result<Media, MediaError> {
    let (svg, _width, _height) = semio_s_artifact_draw_drawing_write::drawing_document_to_svg(doc).map_err(|error| MediaError::Payload("vector:out".into(), error))?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: svg } })
}
//#endregion 🔖️Io


//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
/// 🚪️ Host document-archive door law for the example load (`setActiveExample` → `LoadDocument`).
/// 🌉️ The action bridge folds camelCase keys and JSON values into typed drawing commands.
//#endregion 🧪️UnitTests


#[path="📬️completion/🦀️.rs"]
pub mod completion;

/// 📣️ The en/de notices of every `drawing.gesture.*` refusal code (design §20.12).
pub fn drawing_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 9]> = std::sync::LazyLock::new(|| {
        [
            ("drawing.gesture.retained-route", LocalizedLabel::native("This drawing gesture can only run in an open drawing window.", "Diese Zeichengeste läuft nur in einem geöffneten Zeichenfenster.")),
            ("drawing.gesture.closing", LocalizedLabel::native("The drawing is closing; the gesture was not applied.", "Die Zeichnung wird geschlossen; die Geste wurde nicht angewendet.")),
            ("drawing.gesture.saturated", LocalizedLabel::native("Too many drawing gestures are running at once.", "Zu viele Zeichengesten laufen gleichzeitig.")),
            ("drawing.gesture.owner", LocalizedLabel::native("The drawing gesture ended before it could continue.", "Die Zeichengeste endete, bevor sie fortgesetzt werden konnte.")),
            ("drawing.gesture.point-capacity", LocalizedLabel::native("The stroke has too many points.", "Der Strich hat zu viele Punkte.")),
            ("drawing.gesture.query-owner", LocalizedLabel::native("Another drawing gesture owns this query.", "Eine andere Zeichengeste besitzt diese Abfrage.")),
            ("drawing.gesture.query-capacity", LocalizedLabel::native("Too many shapes match this point.", "Zu viele Formen treffen diesen Punkt.")),
            ("drawing.gesture.query-output-capacity", LocalizedLabel::native("The interaction produced too many results.", "Die Interaktion erzeugte zu viele Ergebnisse.")),
            ("drawing.gesture.command", LocalizedLabel::native("This action cannot run during a drawing gesture.", "Diese Aktion kann während einer Zeichengeste nicht ausgeführt werden.")),
        ]
    });
    &*NOTICES
}

#[cfg(any(test, feature = "testkit"))]
#[path="🧪️tests/🪪️identity/🦀️.rs"]
pub mod identity_test;



//#region 🪢️TaxonomyMounts
#[path = "📋️clipboard/🧵️job/🦀️.rs"]
pub mod clipboard_job;
//#endregion 🪢️TaxonomyMounts
