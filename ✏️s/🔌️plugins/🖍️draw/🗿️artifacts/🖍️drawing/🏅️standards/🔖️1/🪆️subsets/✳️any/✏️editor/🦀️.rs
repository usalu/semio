//! 🖥️ Drawing editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum
//! and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window
//! render in `🎭️modes/✏️edit/🪟️windows/🖼️canvas`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`.
//! This file is a routing table: `handle` → `DrawingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::drawing::commands::nudge_selection::{nudge_selection_left,nudge_selection_left_fast,nudge_selection_right,nudge_selection_right_fast,nudge_selection_up,nudge_selection_up_fast,nudge_selection_down,nudge_selection_down_fast};
use crate::editor::drawing::commands::canvas_pointer_down::{DrawingGesturePreview, DrawingSession};
use crate::editor::drawing::commands::{
    import_image, add_layer, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, combine_boolean, commit_document, delete_layer, delete_selection, drop_layer_kind, duplicate_layer, engagement_input,
    engagement_submit, export_document, edit_selection, edit_path, edit_fill, move_layer, patch_layer, patch_layers, set_active_example, set_camera, set_camera_zoom, load_document_json, set_selected_opacity, set_snapshot, toggle_layer_visible,
};
use crate::editor::drawing::modes::edit;
use crate::editor::drawing::modes::edit::windows::canvas as canvas_window;
use crate::editor::drawing::modes::edit::windows::canvas::config::DrawingCanvasWindowConfig;
use crate::editor::drawing::modes::edit::windows::canvas::transient::DrawingCanvasWindowTransient;
use crate::editor::drawing::panels::{catalogue as catalogue_panel, layers as layers_panel, properties as properties_panel};
use crate::editor::drawing::presence::{DrawingPresence, DrawingPresenceMutation};
use crate::editor::drawing::terminology::DrawingPlayLabels;
use crate::op::DrawingMutation;
use crate::schema::scene_identity::SceneIdentity;
use crate::{DrawingSnapshot, DRAWING_DOCUMENT_SCHEMA};
use semio_framework_job::FixedOperationOwner;
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
use store::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;
use std::collections::HashMap;

pub use canvas_window::{DRAWING_PLAY_BODY_COMPOSITE, DRAWING_PLAY_WINDOW_CANVAS};
pub use catalogue_panel::DRAWING_PLAY_BODY_CATALOGUE;
pub use layers_panel::{DRAWING_LAYER_KIND_DRAG_MIME, DRAWING_PLAY_BODY_LAYERS};
pub use properties_panel::DRAWING_PLAY_BODY_PROPERTIES;

#[path = "🕹️interaction/🦀️.rs"]
pub(crate) mod interaction;
#[path = "🧮️status/🦀️.rs"]
mod status;
#[path="📋️clipboard/🦀️.rs"]
pub mod clipboard;

//#region 🔖️Constants
pub const DRAWING_PLAY_CONTROLLER_ID: &str = "drawing-play";
/// 🧰️ The utility the canvas returns to after committing a shape/draft/trace (first UtilityRef default).
pub const DRAWING_DEFAULT_UTILITY: &str = "selectDirect";

/// 🧰️ The utility armed for THIS window. The React host keeps utilities per window
/// (`ViewModel::active_utility_by_window_id`, keyed by window instance id) and only mirrors the
/// ACTIVE window's utility into the flat `active_utility_id`; a gesture dispatched into a pane that
/// is not the shell's active window therefore carried `None` and every rectangle drag ran as a
/// marquee select (ticket 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17). Resolution order: the
/// addressed window's entry, the focused window's entry, the flat field, the default.
fn drawing_active_utility(view: &semio_framework_plugin::ViewModel) -> &str {
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or(DRAWING_DEFAULT_UTILITY)
}
/// 🕹️ Framework-owned layer selection and snapshot-bound path point selection.
pub const DRAWING_INTERACTION_DOMAIN: &str = "strokes";
pub const DRAWING_INTERACTION_GRANULARITY: &str = "stroke";
pub(crate) const DRAWING_POINT_DOMAIN: &str = "points";
pub(crate) const DRAWING_POINT_GRANULARITY: &str = "point";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn drawing_play_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(DRAWING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎛️ Builds one manifest-side engagement action without crossing into the retained UI wire action type.
fn drawing_manifest_action(action: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: DRAWING_PLAY_CONTROLLER_ID.into(), action: action.into(), args: None }
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl std::fmt::Display) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_format(format_args!("{value}")).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector-bound vocabulary
/// that is dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn drawing_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🖱️ A raw canvas/engagement input event — the canvas dispatches it, the MCP capability catalog
/// never publishes it (`CapabilityAudience::Input`).
fn drawing_input_event(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind).input_event()
}

/// 🗂️ The `kind` argument shared by `addLayer`/`dropLayerKind` — the exact vocabulary
/// `schema::create_layer_by_kind` branches on, so the published JSON Schema enumerates what the
/// guest really accepts instead of leaving an agent to guess "rectangle".
fn drawing_layer_kind_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::select(
        "kind",
        LocalizedLabel::native("Layer Kind", "Ebenenart"),
        vec![
            semio_framework_plugin::ActionArgOption::new("shape:rect", LocalizedLabel::native("Rectangle", "Rechteck")),
            semio_framework_plugin::ActionArgOption::new("shape:ellipse", LocalizedLabel::native("Ellipse", "Ellipse")),
            semio_framework_plugin::ActionArgOption::new("shape:line", LocalizedLabel::native("Line", "Linie")),
            semio_framework_plugin::ActionArgOption::new("shape:polygon", LocalizedLabel::native("Polygon", "Polygon")),
            semio_framework_plugin::ActionArgOption::new("path", LocalizedLabel::native("Path", "Pfad")),
            semio_framework_plugin::ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
            semio_framework_plugin::ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
            semio_framework_plugin::ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
            semio_framework_plugin::ActionArgOption::new("boolean", LocalizedLabel::native("Boolean", "Boolean")),
            semio_framework_plugin::ActionArgOption::new("trace", LocalizedLabel::native("Trace", "Nachzeichnung")),
        ],
    )
    .default_value(&"path")
}

/// 🪪️ The `layerId` argument shared by every single-layer verb.
fn drawing_layer_id_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::text("layerId", LocalizedLabel::native("Layer", "Ebene")).required()
}

/// 🎯️ The layer-tree drop target shared by `moveLayer`/`dropLayerKind`.
fn drawing_target_row_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::text("targetRowId", LocalizedLabel::native("Target Row", "Zielzeile")).required()
}

/// ↕️ Where a moved/dropped layer lands relative to the target row.
fn drawing_drop_position_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::select(
        "dropPosition",
        LocalizedLabel::native("Drop Position", "Ablageposition"),
        vec![
            semio_framework_plugin::ActionArgOption::new("before", LocalizedLabel::native("Before", "Davor")),
            semio_framework_plugin::ActionArgOption::new("after", LocalizedLabel::native("After", "Danach")),
            semio_framework_plugin::ActionArgOption::new("inside", LocalizedLabel::native("Inside", "Hinein")),
        ],
    )
    .default_value(&"after")
}

/// 🩹️ The patchable layer property vocabulary — the exact `field` arm set of
/// `schema::mutations::drawing_op_for_layer_field`.
fn drawing_layer_field_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::select(
        "field",
        LocalizedLabel::native("Field", "Feld"),
        vec![
            semio_framework_plugin::ActionArgOption::new("name", LocalizedLabel::native("Name", "Name")),
            semio_framework_plugin::ActionArgOption::new("textContent", LocalizedLabel::native("Text Content", "Textinhalt")),
            semio_framework_plugin::ActionArgOption::new("textSize", LocalizedLabel::native("Text Size", "Schriftgröße")),
            semio_framework_plugin::ActionArgOption::new("imageKey", LocalizedLabel::native("Image Asset Key", "Bildressourcenschlüssel")),
            semio_framework_plugin::ActionArgOption::new("imageWidth", LocalizedLabel::native("Image Width", "Bildbreite")),
            semio_framework_plugin::ActionArgOption::new("imageHeight", LocalizedLabel::native("Image Height", "Bildhöhe")),
            semio_framework_plugin::ActionArgOption::new("rectX", LocalizedLabel::native("Rectangle X", "Rechteck X")),
            semio_framework_plugin::ActionArgOption::new("rectY", LocalizedLabel::native("Rectangle Y", "Rechteck Y")),
            semio_framework_plugin::ActionArgOption::new("rectWidth", LocalizedLabel::native("Rectangle Width", "Rechteckbreite")),
            semio_framework_plugin::ActionArgOption::new("rectHeight", LocalizedLabel::native("Rectangle Height", "Rechteckhöhe")),
            semio_framework_plugin::ActionArgOption::new("ellipseCx", LocalizedLabel::native("Ellipse Center X", "Ellipsenmittelpunkt X")),
            semio_framework_plugin::ActionArgOption::new("ellipseCy", LocalizedLabel::native("Ellipse Center Y", "Ellipsenmittelpunkt Y")),
            semio_framework_plugin::ActionArgOption::new("ellipseRx", LocalizedLabel::native("Ellipse Radius X", "Ellipsenradius X")),
            semio_framework_plugin::ActionArgOption::new("ellipseRy", LocalizedLabel::native("Ellipse Radius Y", "Ellipsenradius Y")),
            semio_framework_plugin::ActionArgOption::new("circleCx", LocalizedLabel::native("Circle Center X", "Kreismittelpunkt X")),
            semio_framework_plugin::ActionArgOption::new("circleCy", LocalizedLabel::native("Circle Center Y", "Kreismittelpunkt Y")),
            semio_framework_plugin::ActionArgOption::new("circleR", LocalizedLabel::native("Circle Radius", "Kreisradius")),
            semio_framework_plugin::ActionArgOption::new("lineX1", LocalizedLabel::native("Line Start X", "Linienanfang X")),
            semio_framework_plugin::ActionArgOption::new("lineY1", LocalizedLabel::native("Line Start Y", "Linienanfang Y")),
            semio_framework_plugin::ActionArgOption::new("lineX2", LocalizedLabel::native("Line End X", "Linienende X")),
            semio_framework_plugin::ActionArgOption::new("lineY2", LocalizedLabel::native("Line End Y", "Linienende Y")),
            semio_framework_plugin::ActionArgOption::new("polygonX", LocalizedLabel::native("Polygon Point X", "Polygonpunkt X")),
            semio_framework_plugin::ActionArgOption::new("polygonY", LocalizedLabel::native("Polygon Point Y", "Polygonpunkt Y")),
            semio_framework_plugin::ActionArgOption::new("opacity", LocalizedLabel::native("Opacity", "Deckkraft")),
            semio_framework_plugin::ActionArgOption::new("visible", LocalizedLabel::native("Visible", "Sichtbar")),
            semio_framework_plugin::ActionArgOption::new("locked", LocalizedLabel::native("Locked", "Gesperrt")),
            semio_framework_plugin::ActionArgOption::new("blendMode", LocalizedLabel::native("Blend Mode", "Mischmodus")),
            semio_framework_plugin::ActionArgOption::new("booleanOperation", LocalizedLabel::native("Boolean Operation", "Boolean-Operation")),
            semio_framework_plugin::ActionArgOption::new("isolation", LocalizedLabel::native("Isolate Group Blending", "Gruppenmischung isolieren")),
            semio_framework_plugin::ActionArgOption::new("fillRule", LocalizedLabel::native("Fill Rule", "Füllregel")),
            semio_framework_plugin::ActionArgOption::new("fillEnabled", LocalizedLabel::native("Fill Enabled", "Füllung aktiv")),
            semio_framework_plugin::ActionArgOption::new("strokeEnabled", LocalizedLabel::native("Stroke Enabled", "Kontur aktiv")),
            semio_framework_plugin::ActionArgOption::new("strokeColor", LocalizedLabel::native("Stroke Color", "Konturfarbe")),
            semio_framework_plugin::ActionArgOption::new("fillColor", LocalizedLabel::native("Fill Colour", "Füllfarbe")),
            semio_framework_plugin::ActionArgOption::new("strokeCap", LocalizedLabel::native("Line Caps", "Linienenden")),
            semio_framework_plugin::ActionArgOption::new("strokeJoin", LocalizedLabel::native("Line Joins", "Linienverbindungen")),
            semio_framework_plugin::ActionArgOption::new("strokeDash", LocalizedLabel::native("Dash Pattern", "Strichmuster")),
            semio_framework_plugin::ActionArgOption::new("strokeWidth", LocalizedLabel::native("Stroke Width", "Strichstärke")),
            semio_framework_plugin::ActionArgOption::new("transformX", LocalizedLabel::native("Transform X", "Transformation X")),
            semio_framework_plugin::ActionArgOption::new("transformY", LocalizedLabel::native("Transform Y", "Transformation Y")),
            semio_framework_plugin::ActionArgOption::new("transformScaleX", LocalizedLabel::native("Scale X", "Skalierung X")),
            semio_framework_plugin::ActionArgOption::new("transformScaleY", LocalizedLabel::native("Scale Y", "Skalierung Y")),
            semio_framework_plugin::ActionArgOption::new("transformRotation", LocalizedLabel::native("Rotation", "Drehung")),
            semio_framework_plugin::ActionArgOption::new("transformShear", LocalizedLabel::native("Shear", "Scherung")),
            semio_framework_plugin::ActionArgOption::new("rotationDegrees", LocalizedLabel::native("Rotation (°)", "Drehung (°)")),
            semio_framework_plugin::ActionArgOption::new("traceThreshold", LocalizedLabel::native("Trace Threshold", "Schwellenwert")),
            semio_framework_plugin::ActionArgOption::new("traceSimplify", LocalizedLabel::native("Trace Simplify", "Vereinfachung")),
        ],
    )
    .required()
}

/// 🔤️ The patch `value` — one `String` wire field carrying JSON text, so one verb covers bool,
/// number and string properties (see `patch_layer::patch_value_json`).
fn drawing_layer_value_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::json_text("value", LocalizedLabel::native("Value", "Wert")).describe(LocalizedLabel::native("JSON text: a quoted string for name/blendMode/fillColor, a number for opacity/strokeWidth/transforms, true/false for visible/locked.", "JSON-Text: eine Zeichenkette in Anführungszeichen für name/blendMode/fillColor, eine Zahl für opacity/strokeWidth/Transformationen, true/false für visible/locked.")).required()
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn drawing_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), allows_actions_while_active: true, ..UtilityDefinition::new(id, label, icon) }
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
mod args_bridge {
    use super::*;

    fn snake(key: &str) -> String {
        let mut out = String::with_capacity(key.len() + 4);
        for ch in key.chars() {
            if ch.is_ascii_uppercase() {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn camel(key: &str) -> String {
        let mut out = String::with_capacity(key.len());
        let mut upper = false;
        for ch in key.chars() {
            if ch == '_' {
                upper = true;
            } else if upper {
                out.push(ch.to_ascii_uppercase());
                upper = false;
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn put(entries: &mut Vec<(String, semio_framework_value::DslValue)>, key: &str, value: semio_framework_value::DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    /// 🔢️ The host's JSON round trip delivers every integer as `Number::Float`; the exact-integer
    /// codecs refuse that, so whole finite floats go back to their integer variant (`f64` fields
    /// accept any `Number`, so nothing else changes).
    fn integral(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
        match value {
            semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
            }
            semio_framework_value::DslValue::Array(items) => semio_framework_value::DslValue::Array(items.into_iter().map(integral).collect()),
            semio_framework_value::DslValue::Object(entries) => semio_framework_value::DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    /// 🔁️ Every key under both spellings (`FromValue` ignores keys it does not know), `aliases`
    /// applied on the snake_case key, and each `json` key printed to JSON text when the host sent a
    /// structured value for a `String` wire field (`patchLayer.value`, `loadDocumentJson.json`).
    fn fold(args: Option<&semio_framework_value::DslValue>, aliases: &[(&str, &str)], json: &[&str]) -> semio_framework_value::DslValue {
        let mut entries: Vec<(String, semio_framework_value::DslValue)> = Vec::new();
        if let Some(semio_framework_value::DslValue::Object(object)) = args {
            for (key, value) in object {
                let mut key = snake(key);
                if let Some((_, to)) = aliases.iter().find(|(from, _)| *from == key) {
                    key = (*to).to_string();
                }
                let mut value = integral(value.clone());
                if json.contains(&key.as_str()) && !matches!(value, semio_framework_value::DslValue::String(_)) {
                    value = semio_framework_value::DslValue::String(semio_framework_pack_json::to_json_string(&value));
                }
                put(&mut entries, &camel(&key), value.clone());
                put(&mut entries, &key, value);
            }
        }
        semio_framework_value::DslValue::Object(entries)
    }

    /// 🧩️ Supplies `key = value` when the host sent no such argument (a palette/Actions-pane row
    /// dispatches its verb arg-less; `create_layer_by_kind` treats any unknown kind as a path).
    fn default_key(mut folded: semio_framework_value::DslValue, key: &str, value: &str) -> semio_framework_value::DslValue {
        if let semio_framework_value::DslValue::Object(entries) = &mut folded {
            if !entries.iter().any(|(existing, _)| existing == key) {
                put(entries, &camel(key), semio_framework_value::DslValue::String(value.into()));
                put(entries, key, semio_framework_value::DslValue::String(value.into()));
            }
        }
        folded
    }

    fn decode<T: semio_framework_value::FromValue>(action: &str, value: semio_framework_value::DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("draw action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<DrawingCommand, Fault> {
        let plain = || fold(args, &[], &[]);
        Ok(match action {
            "setSnapshot" => DrawingCommand::SetSnapshot(decode(action, plain())?),
            "commitDocument" => DrawingCommand::CommitDocument(decode(action, plain())?),
            "loadDocumentJson" => DrawingCommand::LoadDocumentJson(decode(action, fold(args, &[], &["json"]))?),
            "setActiveExample" => DrawingCommand::SetActiveExample(decode(action, fold(args, &[("id", "example_id"), ("example", "example_id")], &[]))?),
            "setSelectedOpacity" => DrawingCommand::SetSelectedOpacity(decode(action, plain())?),
            "engagementSubmit" => DrawingCommand::EngagementSubmit(decode(action, plain())?),
            "importImage" => DrawingCommand::ImportImage(decode(action, plain())?),
            "addLayer" => DrawingCommand::AddLayer(decode(action, default_key(plain(), "kind", "path"))?),
            "exportDocument" => DrawingCommand::ExportDocument(decode(action, default_key(plain(), "format", export_document::DEFAULT_EXPORT_FORMAT))?),
            "dropLayerKind" => DrawingCommand::DropLayerKind(decode(action, plain())?),
            "moveLayer" => DrawingCommand::MoveLayer(decode(action, plain())?),
            "deleteLayer" => DrawingCommand::DeleteLayer(decode(action, fold(args, &[("id", "layer_id")], &[]))?),
            "duplicateLayer" => DrawingCommand::DuplicateLayer(decode(action, fold(args, &[("id", "layer_id")], &[]))?),
            "toggleLayerVisible" => DrawingCommand::ToggleLayerVisible(decode(action, fold(args, &[("id", "layer_id")], &[]))?),
            "editFill" => {
                let mut value = plain();
                if let semio_framework_value::DslValue::Object(entries) = &mut value {
                    let edit = entries.iter().find(|(key,_)| key == "edit").map(|(_,value)| value.clone()).ok_or_else(|| Fault::from("Missing fill edit"))?;
                    let mut edit: semio_framework_value::DslValue = if let semio_framework_value::DslValue::String(json) = edit { semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::from(error.to_string()))? } else { edit };
                    if let (Some((_,input)),semio_framework_value::DslValue::Object(fields)) = (entries.iter().find(|(key,_)| key == "value"),&mut edit) {
                        let kind = fields.iter().find(|(key,_)| key == "kind").and_then(|(_,value)| value.as_str()).unwrap_or("");
                        let input = if kind=="color" {
                            let alpha=fields.iter().find(|(key,_)|key=="value").and_then(|(_,value)|value.as_array()).and_then(|parts|parts.get(3)).and_then(|value|value.as_f64()).unwrap_or(1.0);
                            let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(4096,&mut accepted);
                            semio_framework_value::ToValue::to_value(&crate::standards::v1::subsets::any::io::text::color::decode_color_text(input.as_str().ok_or_else(||Fault::from("Choose a color"))?,alpha,&mut control).map_err(|error|Fault::from(error.to_string()))?)
                        } else if kind=="type" {
                            semio_framework_value::DslValue::String(input.as_str().ok_or_else(|| Fault::from("Choose a fill value"))?.into())
                        } else {
                            let number = match input { semio_framework_value::DslValue::String(text) => text.parse::<f64>().ok(), other => <f64 as semio_framework_value::FromValue>::from_value(other.clone()).ok() }.filter(|number| number.is_finite()).ok_or_else(|| Fault::from("Enter a finite number"))?;
                            semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(number))
                        };
                        put(fields,"value",input);
                    }
                    put(entries,"edit",integral(edit));
                }
                DrawingCommand::EditFill(decode(action,value)?)
            },
            "editPath" => {
                let mut value = plain();
                if let semio_framework_value::DslValue::Object(entries) = &mut value {
                    let edit = entries.iter().find(|(key, _)| key == "edit").map(|(_, value)| value.clone()).ok_or_else(|| Fault::from("Missing path edit"))?;
                    let mut edit: semio_framework_value::DslValue = if let semio_framework_value::DslValue::String(json) = edit { semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::from(error.to_string()))? } else { edit };
                    if let Some((_, input)) = entries.iter().find(|(key, _)| key == "value") {
                        let number = match input {
                            semio_framework_value::DslValue::String(text) => text.parse::<f64>().ok(),
                            other => <f64 as semio_framework_value::FromValue>::from_value(other.clone()).ok(),
                        }.filter(|number| number.is_finite()).ok_or_else(|| Fault::from("Enter a finite coordinate"))?;
                        if let semio_framework_value::DslValue::Object(fields) = &mut edit {
                            let key=if fields.iter().any(|(key,value)|key=="kind"&&matches!(value,semio_framework_value::DslValue::String(kind) if kind=="simplify")) {"tolerance"}else {"value"};
                            put(fields,key,semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(number)));
                        }
                    }
                    put(entries, "edit", integral(edit));
                }
                DrawingCommand::EditPath(decode(action, value)?)
            },
            "deleteSelection" => DrawingCommand::DeleteSelection(decode(action, plain())?),
            "nudgeSelectionLeft" => DrawingCommand::NudgeSelectionLeft(decode(action, plain())?),
            "nudgeSelectionLeftFast" => DrawingCommand::NudgeSelectionLeftFast(decode(action, plain())?),
            "nudgeSelectionRight" => DrawingCommand::NudgeSelectionRight(decode(action, plain())?),
            "nudgeSelectionRightFast" => DrawingCommand::NudgeSelectionRightFast(decode(action, plain())?),
            "nudgeSelectionUp" => DrawingCommand::NudgeSelectionUp(decode(action, plain())?),
            "nudgeSelectionUpFast" => DrawingCommand::NudgeSelectionUpFast(decode(action, plain())?),
            "nudgeSelectionDown" => DrawingCommand::NudgeSelectionDown(decode(action, plain())?),
            "nudgeSelectionDownFast" => DrawingCommand::NudgeSelectionDownFast(decode(action, plain())?),
            "editSelection" => {
                let mut value = plain();
                if let semio_framework_value::DslValue::Object(entries) = &mut value {
                    if !entries.iter().any(|(key, _)| key == "ids") { put(entries, "ids", semio_framework_value::DslValue::Array(Vec::new())); }
                }
                DrawingCommand::EditSelection(decode(action, value)?)
            },
            "combineBoolean" => DrawingCommand::CombineBoolean(decode(action, fold(args, &[("layer_ids", "ids")], &[]))?),
            "patchLayer" => DrawingCommand::PatchLayer(decode(action, fold(args, &[("id", "layer_id")], &["value"]))?),
            "patchLayers" => DrawingCommand::PatchLayers(decode(action, fold(args, &[("ids", "layer_ids")], &["value"]))?),
            "setCamera" => DrawingCommand::SetCamera(decode(action, plain())?),
            "setCameraZoom" => DrawingCommand::SetCameraZoom(decode(action, fold(args, &[("zoom", "value")], &[]))?),
            "engagementInput" => DrawingCommand::EngagementInput(decode(action, plain())?),
            "canvasPointerDown" => DrawingCommand::CanvasPointerDown(decode(action, plain())?),
            "canvasPointerMove" => DrawingCommand::CanvasPointerMove(decode(action, plain())?),
            "canvasPointerUp" => DrawingCommand::CanvasPointerUp(decode(action, plain())?),
            "canvasDoubleClick" => DrawingCommand::CanvasDoubleClick(decode(action, plain())?),
            "canvasCommitDraft" => DrawingCommand::CanvasCommitDraft(decode(action, plain())?),
            "canvasEscape" => DrawingCommand::CanvasEscape(decode(action, plain())?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the draw editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🌉️ActionBridge

//#region 🧵️GestureOperationJobs
const DRAWING_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"];
const DRAWING_GESTURE_RAW_BYTES: usize = 8_192;
const DRAWING_GESTURE_RETAINED_BYTES: usize = 131_072;

/// 🛣️ One publication lane row per gesture route, read off each route's real `Emit` construction, not
/// off its `ActionKind`: every gesture that reaches a commit does so through the canvas tool's one
/// `Emit::commit_transaction` (`drawing_tool_emit`, artifact lane), and the two routes that
/// additionally write the exact Canvas `WindowTransient` snapshot — `canvasPointerDown` through
/// `advance_trace_pointer`, `canvasEscape` through its own trace cancellation — carry the config lane
/// as well. Under-declaring a lane that is actually emitted faults at publication time.
const DRAWING_GESTURE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasDoubleClick", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasCommitDraft", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasEscape", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

struct DrawingGestureOperationOwner {
    session: Option<DrawingSession>,
    closing: bool,
    /// 🧾️ The declared `DRAWING_GESTURE_RETAINED_BYTES` budget still to be handed back to the
    /// registry. The session itself is dropped on the first granted close step; its budget is
    /// released one grant-sized page per step, because every framework close/maintenance pump grants
    /// at most `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` (4 KiB) — demanding the whole 32 KiB in one step
    /// meant no gesture owner ever closed outside a hand-rolled test (ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17).
    unreleased_bytes: usize,
}

impl DrawingGestureOperationOwner {
    fn new(active_utility_id: &str, authoring_seed: &str) -> Self {
        Self { session: Some(DrawingSession::new(active_utility_id, authoring_seed)), closing: false, unreleased_bytes: DRAWING_GESTURE_RETAINED_BYTES }
    }
}

impl FixedOperationOwner for DrawingGestureOperationOwner {
    fn retained_bytes(&self) -> usize {
        DRAWING_GESTURE_RETAINED_BYTES
    }

    fn cancel(&mut self) {
        self.closing = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        let released_items = usize::from(self.session.take().is_some());
        let released_bytes = self.unreleased_bytes.min(maximum_bytes);
        self.unreleased_bytes -= released_bytes;
        if released_items == 0 && released_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()};
        }
        semio_framework_job::InteractiveJobCloseStep::Pending { progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..Default::default()} }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none() && self.unreleased_bytes == 0
    }
}

const DRAWING_GESTURE_OPERATION_SLOTS: usize = 64;

struct DrawingInstanceOperationOwner {
    operations: semio_framework_job::FixedOperationRegistry<DrawingGestureOperationOwner, DRAWING_GESTURE_OPERATION_SLOTS>,
    active: Option<(semio_framework_job::FixedOperationKey, SceneIdentity)>,
    closing: bool,
    /// 🚪️ Every slot has been marked closing. `FixedOperationRegistry::begin_close_step` and
    /// `close_step` share one cursor and each advance it by one, so a close that calls both per step
    /// only ever begins the close of every OTHER slot — an owner on the wrong parity was never marked
    /// and its close spun `Pending { 0, 0 }` until the app's Drop witness fired (ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17). The first close step now sweeps all slots once.
    close_begun: bool,
}

impl DrawingInstanceOperationOwner {
    fn new() -> Self {
        Self { operations: semio_framework_job::FixedOperationRegistry::new(DRAWING_GESTURE_OPERATION_SLOTS * DRAWING_GESTURE_RETAINED_BYTES), active: None, closing: false, close_begun: false }
    }

    fn dispatch(&mut self, payload: &DrawingGestureOperationPayload) -> Result<Option<(Emit<DrawingMutation, NoConfigMutation, NoDraftMutation>, DrawingCanvasWindowTransient)>, Fault> {
        let key = semio_framework_job::FixedOperationKey::new(semio_framework_job::OperationId(payload.operation_context.operation_id), semio_framework_job::Generation(payload.operation_context.generation));
        let source_identity = payload.source_identity;
        let command = &payload.command;
        let snapshot = payload.snapshot.as_ref();
        let config = payload.config.as_ref();
        let active_utility_id = payload.active_utility_id.as_str();
        let history = payload.history.as_ref();
        let operation = payload.operation_context.clone();
        if self.closing {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.closing"), "the Drawing gesture operation owner is closing"));
        }
        if let Some((active, observed_revision)) = self.active {
            let same_utility = self.operations.get(active).and_then(|owner| owner.session.as_ref()).is_some_and(|session| session.active_utility_id == active_utility_id);
            if !observed_revision.matches(source_identity) || !same_utility {
                self.operations.cancel(active);
                self.active = None;
            }
        }
        let live_key = self.active.map_or(key, |(active, _)| active);
        if self.operations.get(live_key).is_none() {
            // 🧹️ The registry is direct-mapped by `operation_id % slots` and the framework mints every
            // tool operation id INTO the first vacant residue class of its own 64-slot table — so the
            // next gesture reuses residue 0 the moment the previous one settled, and lands on the slot
            // still held by that gesture's cancelled-but-not-yet-retired owner. Retire the retiring
            // owners now (bounded: one cursor sweep, each owner closes within its declared grant)
            // instead of answering `saturated` (every browser gesture failed so, 2026-09-17).
            if !self.operations.can_admit(live_key, DRAWING_GESTURE_RETAINED_BYTES) {
                for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS * 2 {
                    if self.operations.can_admit(live_key, DRAWING_GESTURE_RETAINED_BYTES) {
                        break;
                    }
                    let _ = self.operations.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
                }
            }
            self.operations.admit(live_key, DrawingGestureOperationOwner::new(active_utility_id, &operation.authoring_seed)).map_err(|mut rejected| {
                rejected.owner.cancel();
                rejected.owner.begin_close();
                let _ = rejected.owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
                Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.saturated"), "the fixed Drawing gesture operation authority is saturated")
            })?;
            self.active = Some((live_key, source_identity));
        }
        let retained = self.operations.get_mut(live_key).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the exact Drawing gesture owner changed before its bounded reducer step"))?;
        let session = retained.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the Drawing gesture session is already closing"))?;
        session.source_identity=Some(source_identity);
        session.window_config = payload.window_config.clone();
        session.window_transient = payload.window_transient.clone();
        session.base = Some(canvas_pointer_down::DrawingToolBase { document: payload.snapshot.clone(), operation: Some(operation.clone()) });
        if session.tool.context().points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.point-capacity"), "the fixed Drawing gesture point capacity was exceeded"));
        }
        if let Some(query) = session.point_query.as_mut() {
            if query.command_id != command.command_id() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query-owner"), "a retained Drawing point query rejects a different command owner"));
            }
            geometry_session::prepare_query(source_identity,snapshot);
            let admitted=geometry_session::with_query(source_identity,|status,borrowed|match status {
                crate::schema::scene_identity::admission::SceneAdmissionStatus::Pending=>Ok(None),
                crate::schema::scene_identity::admission::SceneAdmissionStatus::Ready=>{
                    let borrowed=borrowed.unwrap();
                    if query.traversal_complete{query.cursor.validate_cache(&borrowed)?;Ok(Some(true))}
                    else{Ok(Some(query.cursor.advance(snapshot,&borrowed)))}
                },
                _=>Err(Fault::from("Drawing pointer geometry is unavailable or changed")),
            });
            let admitted=match admitted{Ok(admitted)=>admitted,Err(fault)=>{self.operations.cancel(live_key);self.active=None;return Err(fault)}};
            let Some(complete)=admitted else{return Ok(None)};
            if !query.traversal_complete {
                if !complete {
                    return Ok(None);
                }
                if query.cursor.overflowed {
                    let error=query.cursor.failure.clone().unwrap_or_else(||"Drawing query exceeds result capacity".into());
                    session.point_query = None;
                    self.operations.cancel(live_key);
                    self.active = None;
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query"), error));
                }
                query.traversal_complete = true;
                return Ok(None);
            }
            let ids=payload.interaction_state.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]);
            let point_ids=payload.interaction_state.selection.get(DRAWING_POINT_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]);
            match session.prepare_grab(snapshot,ids,point_ids) {
                Ok(false)=>return Ok(None),
                Err(fault)=>{ self.operations.cancel(live_key); self.active=None; return Err(fault); },
                Ok(true)=>{},
            }
            if session.point_query.as_ref().is_some_and(|query|query.constrained) && session.tool.matches("pressing") && session.node_marquee.is_none() {
                session.escape()?;
            }
            let query=session.point_query.as_mut().expect("retained point query");
            let targets = match query.publication_step() {
                canvas_pointer_down::DrawingQueryPublication::Pending => return Ok(None),
                canvas_pointer_down::DrawingQueryPublication::Complete(targets) => targets,
                canvas_pointer_down::DrawingQueryPublication::Fault => {
                    session.point_query = None;
                    self.operations.cancel(live_key);
                    self.active = None;
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query-output-capacity"), "the fixed Drawing interaction output capacity was exceeded"));
                }
            };
            let query = session.point_query.take().expect("the exact published query remains retained");
            let mut emit = Emit::default();
            if query.node_selection.is_some() {emit.effects.push(canvas_pointer_down::point_selection_effect_from_targets(targets));}
            else if !query.preserve_selection {
                emit.effects.push(if query.hover {canvas_pointer_down::interaction_hover_effect_from_targets(targets)}else {canvas_pointer_down::interaction_select_effect_from_targets(targets,&query.merge)});
            }
            let window_transient = session.window_transient.clone();
            if session.tool.at_rest() && session.trace_pointer.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some((emit, window_transient)));
        }
        if let DrawingCommand::CanvasPointerDown(pointer) = command {
            if (active_utility_id=="editNodes" || active_utility_id=="selectDirect" && !pointer.ctrl && !pointer.meta) && pointer.generation.is_none() {
                let (x,y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport,pointer.x,pointer.y,pointer.width,pointer.height);
                let world = [x,y];
                session.press(canvas_pointer_down::DrawingPointer { utility:active_utility_id.into(),world,shift:pointer.shift,alt:pointer.alt,ctrl:pointer.ctrl,meta:pointer.meta })?;
                let tolerance = canvas_pointer_down::DRAWING_PICK_TOLERANCE_PX/session.window_config.viewport.zoom.max(1e-6);
                let mut query = canvas_pointer_down::DrawingPointQuery::new(command.command_id(),canvas_pointer_down::TracePointerJob::new_query(snapshot,world,tolerance,false),false,"replace".into(),false);
                query.cursor.node_editing=active_utility_id=="editNodes";
                query.point_pick_mode=if pointer.shift {interaction::points::PointPickMode::Toggle} else if pointer.ctrl || pointer.meta {interaction::points::PointPickMode::Add} else {interaction::points::PointPickMode::Replace};
                query.constrained=pointer.shift;
                query.centered=pointer.alt;
                query.cursor.retain_selection_bounds(payload.interaction_state.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]))?;
                query.drag_start = Some(world);
                session.point_query = Some(query);
                return Ok(None);
            }
        }
        if let DrawingCommand::CanvasPointerMove(payload) = command {
            if session.tool.at_rest() {
                // 🧵️ Idle hover hit-tests the LAST sample of a batch only (design L4 / §2 D).
                let [x, y] = payload.last_sample();
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport, x, y, payload.width, payload.height);
                let tolerance = canvas_pointer_down::DRAWING_PICK_TOLERANCE_PX / session.window_config.viewport.zoom.max(1e-6);
                session.point_query = Some(canvas_pointer_down::DrawingPointQuery::new(
                    command.command_id(),
                    canvas_pointer_down::TracePointerJob::new_query(snapshot, [world_x, world_y], tolerance, session.active_utility_id == "selectDirect"),
                    true,
                    "replace".into(),
                    false,
                ));
                return Ok(None);
            }
        }
        let document_owner = payload.snapshot.clone();
        let retained_emit = match command {
            DrawingCommand::CanvasPointerMove(pointer) if session.tool.matches("pressing") || session.tool.matches("dragging") => {
                let [x,y] = pointer.last_sample();
                let (x,y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport,x,y,pointer.width,pointer.height);
                Some(Some(session.sample([x,y],pointer.shift,pointer.alt)?))
            }
            DrawingCommand::CanvasPointerMove(payload) if session.tool.matches("marqueeing") && session.tool.context().method == "lasso" => Some(session.advance_lasso_move(payload)?),
            // 🚫️ A cancelled release aborts a live gesture with zero trace and selects/commits nothing.
            DrawingCommand::CanvasPointerUp(payload) if payload.cancelled => Some(Some(session.cancel())),
            DrawingCommand::CanvasPointerUp(payload) => {
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport, payload.x, payload.y, payload.width, payload.height);
                let pointer = canvas_pointer_down::DrawingPointer { utility: session.active_utility_id.clone(), world: [world_x, world_y], shift: payload.shift, alt: payload.alt, ctrl: payload.ctrl, meta: payload.meta };
                let base = canvas_pointer_down::DrawingToolBase { document: document_owner, operation: Some(operation.clone()) };
                Some(session.release(command.command_id(), pointer, base)?)
            }
            DrawingCommand::CanvasDoubleClick(_) | DrawingCommand::CanvasCommitDraft(_) => {
                let base = canvas_pointer_down::DrawingToolBase { document: document_owner, operation: Some(operation.clone()) };
                Some(Some(session.finish_draft(base)?))
            }
            _ => None,
        };
        if let Some(retained_emit) = retained_emit {
            let Some(emit) = retained_emit else { return Ok(None) };
            let window_transient = session.window_transient.clone();
            if session.tool.at_rest() && session.trace_pointer.is_none() && session.point_query.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some((emit, window_transient)));
        }
        let doc = ArtifactView::with_operation(snapshot, history, operation);
        let cfg = ConfigView { snapshot: config, window: None };
        let emit = match command {
            DrawingCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle(payload, &doc, &cfg, session),
            DrawingCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle(payload, &doc, &cfg, session),
            DrawingCommand::CanvasPointerUp(_) | DrawingCommand::CanvasDoubleClick(_) | DrawingCommand::CanvasCommitDraft(_) => unreachable!("retained Drawing gesture commands returned above"),
            DrawingCommand::CanvasEscape(payload) => canvas_escape::handle(payload, &doc, &cfg, session),
            _ => Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.command"), "the retained Drawing gesture owner rejects non-gesture commands")),
        }?;
        if session.tool.context().points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.point-capacity"), "the fixed Drawing gesture point capacity was exceeded"));
        }
        let window_transient = session.window_transient.clone();
        if session.tool.at_rest() && session.trace_pointer.is_none() {
            self.operations.cancel(live_key);
            self.active = None;
        }
        Ok(Some((emit, window_transient)))
    }

    fn preview_projection(&mut self, source_identity: SceneIdentity, active_utility: &str) -> Option<DrawingGesturePreview> {
        let (key, observed_revision) = self.active?;
        if !observed_revision.matches(source_identity) {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        let session = self.operations.get_mut(key).and_then(|owner| owner.session.as_mut())?;
        if session.active_utility_id != active_utility {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        if session.tool.at_rest() && session.trace_pointer.is_none() && session.point_query.is_none() {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        Some(session.preview())
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for DrawingInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        // 🔁️ The registry's close cursor visits one slot per call; an idle (live or vacant) slot answers
        // `Pending { 0, 0 }`. Skip past those within one bounded sweep so a retiring owner gets a page
        // every maintenance step rather than every 64th — 9 steps to retire a gesture, not 576.
        for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS {
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items,maximum_copy_bytes:maximum_bytes,maximum_capacity_bytes:maximum_bytes,maximum_release_bytes:maximum_bytes,maximum_depth:32};
            match self.operations.close_step(grant) {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } if progress==Default::default() && !self.operations.is_empty() => continue,
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "Drawing gesture close owner awaits a non-empty grant" }),
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } => return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items:progress.copied_items, released_bytes:progress.released_bytes }),
                semio_framework_job::InteractiveJobCloseStep::Complete {..} => return Ok(semio_framework_plugin::PluginCloseStep::Complete),
                semio_framework_job::InteractiveJobCloseStep::Refused(kind) => return Err(Fault::from(format!("Drawing gesture retirement refused: {kind:?}"))),
            }
        }
        Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if !self.close_begun {
            for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS {
                self.operations.begin_close_step();
            }
            self.close_begun = true;
        }
        self.maintenance_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.operations.is_empty()
    }
}

struct DrawingGestureOperationPayload {
    source_identity: SceneIdentity,
    command: DrawingCommand,
    snapshot: std::sync::Arc<DrawingSnapshot>,
    config: std::sync::Arc<NoConfig>,
    window_config: DrawingCanvasWindowConfig,
    window_transient: DrawingCanvasWindowTransient,
    view_state: semio_framework_plugin::ViewModel,
    history: std::sync::Arc<semio_framework_plugin::HistoryView>,
    interaction_state: std::sync::Arc<protocol::InteractionState>,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    operation_context: semio_framework_plugin::AppOperationContext,
    active_utility_id: String,
    completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<DrawingPlayApp>>,
}

/// 📦️ Validates the retained wire pages of one gesture against the typed command the host already
/// decoded: the pages are the command's OWN binary op encoding (`app_commands!` `OpBinary`, the same
/// bytes `ArtifactRetainedCommandJob` decodes for the bounded lane — since 09-15 the host crosses a
/// pack, not `["verb", {...}]` JSON text). Fail-closed exactly like the framework lane: the bytes
/// must decode to a command whose id is the registered verb, so truncation and a variant swap reject
/// before dispatch. No byte-equal re-encode: the host's pack encoder is not obliged to emit the
/// Rust encoder's canonical layout (float/int spelling, defaulted fields).
struct DrawingRetainedCommandDecoder {
    expected_verb: &'static str,
    raw: Vec<u8>,
    overflow: bool,
}

impl DrawingRetainedCommandDecoder {
    fn new(expected_verb: &'static str) -> Self {
        Self { expected_verb, raw: Vec::new(), overflow: false }
    }

    /// 📄️ Appends one wire page within `DRAWING_GESTURE_RAW_BYTES`; an oversized owner fails closed.
    fn feed_page(&mut self, page: &[u8]) -> bool {
        if self.overflow || self.raw.len().checked_add(page.len()).is_none_or(|end| end > DRAWING_GESTURE_RAW_BYTES) || self.raw.try_reserve_exact(page.len()).is_err() {
            self.overflow = true;
            return false;
        }
        self.raw.extend_from_slice(page);
        true
    }

    fn finish(&self) -> bool {
        if self.overflow {
            return false;
        }
        let Ok(decoded) = <DrawingCommand as ::protocol::OpBinary>::decode_op(&self.raw) else { return false };
        decoded.command_id() == self.expected_verb
    }
}

struct DrawingGestureOperationJob {
    payload: Option<DrawingGestureOperationPayload>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
    raw_input: Option<semio_framework::action_bus::RetainedToolWireInput>,
    raw_page_cursor: usize,
    raw_byte_cursor: usize,
    decoder: Option<DrawingRetainedCommandDecoder>,
    raw_validated: bool,
    completed: bool,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for DrawingGestureOperationJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if context.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if context.should_yield() || context.fuel_remaining() == 0 {
            return semio_framework_job::StepOutcome::Yield;
        }
        if !self.raw_validated {
            let Some(input) = self.raw_input.as_ref() else { return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }) };
            if let Some(page) = input.page(self.raw_page_cursor) {
                let Some(decoder) = self.decoder.as_mut() else {
                    return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
                };
                if !decoder.feed_page(page) {
                    return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
                }
                self.raw_page_cursor += 1;
                self.raw_byte_cursor = 0;
                context.consume_fuel(page.len().max(1) as u64);
                return semio_framework_job::StepOutcome::Yield;
            }
            let exact = self.decoder.as_ref().is_some_and(DrawingRetainedCommandDecoder::finish);
            if !exact {
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            return semio_framework_job::StepOutcome::Yield;
        }
        if self.pending_completion_rejection.is_some() {
            return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }) };
            let emit = payload.instance_owner.with_mut::<DrawingInstanceOperationOwner, _>(|owner| owner.dispatch(payload));
            let (emit, transient) = match emit {
                Ok(Some(output)) => output,
                Ok(None) => {
                    context.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                Err(error) => {
                    if let Err(rejected) = payload.completion.complete(Err(error), semio_framework_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                    }
                    return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
                }
            };
            let window_transient = (transient != payload.window_transient)
                .then(|| canvas_window::transient::addressed(&payload.view_state, transient))
                .transpose();
            let ephemeral = match window_transient {
                Ok(Some(mutation)) => semio_framework_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() },
                Ok(None) => semio_framework_plugin::EphemeralEmit::default(),
                Err(error) => {
                    if let Err(rejected) = payload.completion.complete(Err(error), semio_framework_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                    }
                    return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
                }
            };
            if let Err(rejected) = payload.completion.complete(Ok(emit), ephemeral) {
                self.pending_completion_rejection = Some(rejected);
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
            }
            self.completed = true;
            context.consume_fuel(1);
        }
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        let pending=|copied_items,released_bytes|semio_framework_job::InteractiveJobCloseStep::Pending {progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items,released_bytes,..Default::default()}};
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return match step {
                        semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => pending(released_items,released_bytes),
                        semio_framework_plugin::PluginCloseStep::Blocked { .. } | semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } => semio_framework_job::InteractiveJobCloseStep::Blocked,
                        semio_framework_plugin::PluginCloseStep::Complete => unreachable!("child close helper consumes completed children"),
                    };
                }
            }
            if maximum_items == 0 {
                return pending(0,0);
            }
            self.pending_completion_rejection = None;
            return pending(1,0);
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(input) = self.raw_input.as_mut() {
            let step = input.close_step(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,..grant});
            if input.terminal_is_empty() {
                self.raw_input = None;
            }
            return match step {
                semio_framework_job::InteractiveJobCloseStep::Complete {progress} => semio_framework_job::InteractiveJobCloseStep::Pending {progress},
                other => other,
            };
        }
        if self.payload.take().is_some() {
            return pending(1,0);
        }
        if let Some(decoder)=self.decoder.as_mut() {
            let bytes=decoder.raw.capacity();
            if bytes>maximum_bytes {return pending(0,0);}
            drop(std::mem::take(&mut decoder.raw));self.decoder=None;
            return pending(1,bytes);
        }
        semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()}
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(0)}
    fn next_close_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError> {Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {
        if let Some(input)=self.raw_input.as_ref(){return input.next_close_release_byte_demand();}
        Ok(self.decoder.as_ref().map_or(0,|decoder|decoder.raw.capacity()))
    }
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(usize::from(!self.terminal_is_empty()))}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.pending_completion_rejection.is_none() && self.payload.is_none() && self.raw_input.is_none() && self.decoder.is_none()
    }
}

struct DrawingGestureOperationJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl DrawingGestureOperationJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DRAWING_GESTURE_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller_id, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DrawingGestureOperationJobFactory {
    type Payload = DrawingGestureOperationPayload;
    type Job = DrawingGestureOperationJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAWING_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(DRAWING_GESTURE_RAW_BYTES, 32, 1, 16_384, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(DrawingGestureOperationJob { payload: Some(payload), pending_completion_rejection: None, raw_input: None, raw_page_cursor: 0, raw_byte_cursor: 0, decoder: None, raw_validated: true, completed: false, closing: false })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > DRAWING_GESTURE_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("Drawing gesture retained ingress rejects a checkpoint or oversized wire owner"), input, checkpoint));
        }
        let mut job = match self.create_job(operation, payload) {
            Ok(job) => job,
            Err(error) => return Err((error, input, None)),
        };
        let Some(expected_verb) = job.payload.as_ref().map(|payload| payload.command.command_id()) else {
            return Err((semio_framework::ToolJobFactoryError::new("Drawing gesture retained decoder has no exact typed command owner"), input, None));
        };
        job.raw_input = Some(input);
        job.decoder = Some(DrawingRetainedCommandDecoder::new(expected_verb));
        job.raw_validated = false;
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingGestureOperationJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_GESTURE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = DRAWING_GESTURE_PUBLICATION_CONTRACTS;
}

#[cfg(test)]
#[path = "🧪️tests/🔬️gesture-operation-owner/🦀️.rs"]
mod gesture_operation_owner_tests;
//#endregion 🧵️GestureOperationJobs

//#region 🧵️BoundedCommands
/// 🧵️ Every non-gesture `DrawingCommand` row, each reducing in one bounded first step through
/// [`DrawingBoundedCommandJobFactory`]. Together with [`DRAWING_GESTURE_TOOL_IDS`] this covers the
/// command enum exactly (pinned by `retained_route_dispositions_are_exact_and_exhaustive`): an id
/// missing from both lists is unreachable from the client, because `validate_tool_job_rows` demands
/// one proof row per `Migrated` generated id and `validate_ui_dispatch_classification` rejects
/// anything that is not `Migrated`.
const DRAWING_BOUNDED_TOOL_IDS: &[&str] = &[
    "deleteSelection",
    "nudgeSelectionLeft",
    "nudgeSelectionLeftFast",
    "nudgeSelectionRight",
    "nudgeSelectionRightFast",
    "nudgeSelectionUp",
    "nudgeSelectionUpFast",
    "nudgeSelectionDown",
    "nudgeSelectionDownFast",
    "setSnapshot",
    "commitDocument",
    "loadDocumentJson",
    "setActiveExample",
    "setSelectedOpacity",
    "engagementSubmit",
    "addLayer",
    "dropLayerKind",
    "moveLayer",
    "deleteLayer",
    "duplicateLayer",
    "toggleLayerVisible",
    "combineBoolean",
    "patchLayer",
    "patchLayers",
    "setCamera",
    "setCameraZoom",
    "engagementInput",
    "editSelection",
    "editFill",
];
const DRAWING_EXPORT_TOOL_IDS: &[&str] = &["exportDocument"];
const DRAWING_CLIPBOARD_RESERVED_IDS: &[&str] = &["copy","cut","paste"];
const DRAWING_EXPORT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
];
#[path="🎮️commands/📤️export-document/🧵️export/🦀️.rs"]
mod export_job;
use export_job::DrawingExportCommandJobFactory;
const DRAWING_IMAGE_RAW_BYTES:usize=89_481_584;
const DRAWING_IMAGE_TOOL_IDS: &[&str] = &["importImage"];
const DRAWING_IMAGE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importImage", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
];
#[path="🎮️commands/📥️import-image/🧵️admission/🦀️.rs"]
mod image_job;
use image_job::DrawingImageCommandJobFactory;
const DRAWING_PATH_TOOL_IDS: &[&str] = &["editPath"];
const DRAWING_PATH_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editPath", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
];
#[path="🎮️commands/✏️edit-path/🧵️simplify/🦀️.rs"]
mod path_job;
use path_job::DrawingPathCommandJobFactory;
const DRAWING_BOUNDED_PAYLOAD_SCHEMA: &str = "drawing.tool-command.v1";
const DRAWING_BOUNDED_RAW_BYTES: usize = 65_536;
const DRAWING_BOUNDED_WORK_ITEMS: usize = 4_096;

/// 🛣️ One publication lane row per bounded route, read off each handler's real `Emit` construction
/// (`🎮️commands/*/🦀️.rs`). The four whole-document routes emit nothing but an `Effect::LoadDocument`
/// — effects are not a store lane, so those are honestly `HostOnly`; the layer routes emit
/// `DrawingMutation`s; the view routes emit exact Canvas window mutations.
const DRAWING_BOUNDED_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeft", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeftFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionRight", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionRightFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionUp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionUpFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionDownFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "commitDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadDocumentJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSelectedOpacity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "dropLayerKind", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "duplicateLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleLayerVisible", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editFill", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "combineBoolean", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayers", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

fn drawing_bounded_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(DRAWING_BOUNDED_RAW_BYTES, DRAWING_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 📏️ One work item per bounded dispatch, admitted only while the live document still fits the
/// declared envelope — a gesture id or an oversized document answers `None`, which fails the route
/// closed instead of publishing an unbounded step.
fn drawing_bounded_extent(command: &DrawingCommand, snapshot: &DrawingSnapshot, _interaction: &::protocol::InteractionState) -> Option<usize> {
    if !DRAWING_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let items = [snapshot.layers.len(), snapshot.assets.len()].into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= DRAWING_BOUNDED_WORK_ITEMS).then_some(1)
}

struct DrawingWindowCommandWork {
    tool_id: &'static str,
    completed: bool,
}

impl DrawingWindowCommandWork {
    fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false } }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<DrawingPlayApp>> for DrawingWindowCommandWork {
    fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(
        &self,
        command: &DrawingCommand,
        snapshot: &DrawingSnapshot,
        interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
    ) -> Option<usize> {
        if self.completed || command.command_id() != self.tool_id || drawing_bounded_extent(command, snapshot, interaction) != Some(1) { return None; }
        match command {
            DrawingCommand::SetCamera(_) | DrawingCommand::SetCameraZoom(_) | DrawingCommand::EngagementInput(_) => context?.view_state.as_ref().map(|_| 1),
            _ => Some(1),
        }
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<DrawingPlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<DrawingPlayApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || input.command.command_id() != self.tool_id { return Err(Fault::from("drawing-window-work-terminal")); }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let active_utility = input.context.and_then(|context| context.view_state.as_ref()).map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility);
        let mut session = DrawingSession::new(active_utility, &input.operation.authoring_seed);
        session.interaction.ids = input.interaction.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
        session.interaction.points = input.interaction.selection.get(DRAWING_POINT_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
        session.window_config = canvas_window::config::from_snapshot(input.context.and_then(|context| context.window_config.as_ref()));
        session.window_transient = canvas_window::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let mut emit = Emit::default();
        let mut transient = None;
        match input.command {
            DrawingCommand::SetCamera(payload) => {
                payload.camera.validate().map_err(|error| Fault::from(error.to_string()))?;
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                let mut config = session.window_config;
                config.viewport = payload.camera;
                config.framed = true;
                emit.window_config_mutations.push(canvas_window::config::addressed(view, config)?);
            }
            DrawingCommand::SetCameraZoom(payload) => {
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                let mut config = session.window_config;
                config.viewport.zoom = payload.value;
                config.framed = true;
                config.viewport.validate().map_err(|error| Fault::from(error.to_string()))?;
                emit.window_config_mutations.push(canvas_window::config::addressed(view, config)?);
            }
            DrawingCommand::EngagementInput(payload) => {
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                session.window_transient.engagement_input = payload.value.clone();
                transient = Some(canvas_window::transient::addressed(view, session.window_transient)?);
            }
            _ => emit = input.command.dispatch(&doc, &cfg, &mut session)?,
        }
        self.completed = true;
        Ok(match transient {
            Some(mutation) => ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() },
            },
            None => ArtifactCommandWorkStep::Complete(emit),
        })
    }
}

struct DrawingBoundedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl DrawingBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DRAWING_BOUNDED_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller_id, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DrawingBoundedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<DrawingPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<DrawingPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAWING_BOUNDED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        drawing_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > DRAWING_BOUNDED_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("bounded Drawing command ingress rejects a checkpoint or oversized wire owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingBoundedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = DRAWING_BOUNDED_PUBLICATION_CONTRACTS;
}

/// 🏗️ The bounded half of `DrawingPlayApp::build_tool_job` — one `BoundedArtifactCommandWork` per
/// non-gesture route, refused outright when the id, the typed command and the live extent disagree.
fn drawing_bounded_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<DrawingPlayApp>>) -> Result<semio_framework::ToolOperationSpec, Fault> {
    if request.command.command_id() != request.tool_id || drawing_bounded_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "bounded Drawing command does not match its exact registered tool or exceeds its declared extent"));
    }
    let tool_id = request.command.command_id();
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<DrawingPlayApp>>> = Box::new(DrawingWindowCommandWork::new(tool_id));
    let operation_context = semio_framework_plugin::AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
        semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation: operation_context,
            completion: request.completion,
        },
        DrawingCommand::command_id,
        DRAWING_BOUNDED_RAW_BYTES,
        DRAWING_BOUNDED_WORK_ITEMS,
        work,
    );
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation))
}
//#endregion 🧵️BoundedCommands

//#region 📬️StorePreparation

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct DrawingArtifactStorePreparationFactory;

struct DrawingArtifactStorePreparation {
    base: Option<store::SnapshotRead<DrawingSnapshot>>,
    mutation: Option<DrawingMutation>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &DrawingMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("drawing-artifact-lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation>>, store::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation>> {
        let items = request.base.get().layers.len().saturating_add(request.base.get().assets.len());
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || items > DRAWING_BOUNDED_WORK_ITEMS
        {
            return Err(request);
        }
        Ok(Box::new(DrawingArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use ::protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "drawing-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "drawing-artifact-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get()).map_err(semio_framework_value::ValueError::into_message)?;
        let post = ::protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "drawing-artifact-authority-missing".to_string())?;
        let edit = authority.next_edit(mutation, inverse);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"drawing-artifact-base-retirement-rejected"));
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

//#endregion 📬️StorePreparation

//#region 🧾️ProofCatalogs
/// 🧾️ One `bounded_first_step_tool_proofs!` invocation per owned factory — the macro emits the whole
/// `bounded_first_step_tool_proofs()` body for a single factory, so an app with two factories declares
/// two catalogs and concatenates them in its `ArtifactEditor` item (process3d/flow precedent).
struct DrawingGestureProofs;
impl DrawingGestureProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingGestureOperationJobFactory",
        factory_type: DrawingGestureOperationJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
        tools: ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"]
    }
}

struct DrawingPathProofs;
impl DrawingPathProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingPathCommandJobFactory",
        factory_type: DrawingPathCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
        tools: ["editPath"]
    }
}

struct DrawingImageProofs;
impl DrawingImageProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingImageCommandJobFactory",
        factory_type: DrawingImageCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(89_481_584, 4_096, 1, 262_144, 7_500, 1, 1),
        tools: ["importImage"]
    }
}

struct DrawingExportProofs;
impl DrawingExportProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingExportCommandJobFactory",
        factory_type: DrawingExportCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(4_096, 4_096, 1, 67_108_864, 7_500, 1, 1),
        tools: ["exportDocument"]
    }
}
struct DrawingBoundedProofs;
impl DrawingBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingBoundedCommandJobFactory",
        factory_type: DrawingBoundedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "setSnapshot", "commitDocument", "loadDocumentJson", "setActiveExample", "setSelectedOpacity", "engagementSubmit",
            "addLayer", "dropLayerKind", "moveLayer", "deleteLayer", "duplicateLayer", "toggleLayerVisible", "combineBoolean",
            "patchLayer", "patchLayers", "setCamera", "setCameraZoom", "engagementInput", "editSelection", "editFill",
            "deleteSelection",
    "nudgeSelectionLeft", "nudgeSelectionLeftFast", "nudgeSelectionRight", "nudgeSelectionRightFast", "nudgeSelectionUp", "nudgeSelectionUpFast", "nudgeSelectionDown", "nudgeSelectionDownFast",
        ]
    }
}
//#endregion 🧾️ProofCatalogs

//#region 🔖️DrawingPlayApp
pub(crate) fn drawing_document_revision(doc: &ArtifactView<'_, DrawingSnapshot>) -> String {
    doc.operation_optional().map_or_else(|| "0".repeat(64), |operation| operation.canonical_base_revision_hex())
}

/// 🧪️ Drawing editor owner; durable content stays in the document while concrete Canvas windows own
/// persisted navigation and ephemeral engagement/trace progress.
pub struct DrawingPlayApp {
    arena_boot_fault: Option<&'static str>,
}

impl DrawingPlayApp {
    pub fn arena_boot_fault(&self) -> Option<&'static str> {
        self.arena_boot_fault.or_else(crate::spr::drawing_mutation_arena_pool_fault)
    }
}

fn render_drawing_body(
    body_key: &str,
    doc: &ArtifactView<'_ ,DrawingSnapshot>,
    config: &DrawingCanvasWindowConfig,
    preview: &DrawingGesturePreview,
    selection: &[String],
    point_selection: &[String],
    view_state: &semio_framework_plugin::ViewModel,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let document=doc.snapshot;
    let labels = semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state);
    let active_utility = drawing_active_utility(view_state);
    // 🪟️ One `TreeWindows` per panel body: the host's open/scroll state for exactly the containers
    // that body owns, plus the shared first-paint budget the panel spends in document order.
    let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
    let root = match body_key {
        DRAWING_PLAY_BODY_COMPOSITE => geometry_session::with_visual(doc.render_operation(),|plan,revision,fresh|{let idle=DrawingGesturePreview::default();canvas_window::render(plan,revision,document,config,if fresh{preview}else{&idle},active_utility,if fresh{selection}else{&[]},if fresh{point_selection}else{&[]})}),
        DRAWING_PLAY_BODY_LAYERS => layers_panel::render(document, labels, &windows),
        DRAWING_PLAY_BODY_CATALOGUE => catalogue_panel::render(document, labels, &windows),
        DRAWING_PLAY_BODY_PROPERTIES => properties_panel::render(document, &[], labels, &windows),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("drawing.body.label", "the fixed Drawing unknown-body label exceeds its UI bound")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(root))
}

impl Default for DrawingPlayApp {
    fn default() -> Self {
        let arena_boot_fault = match crate::spr::request_drawing_mutation_arena_pool() {
            crate::spr::DrawingMutationArenaPoolAvailability::Fault(error) => Some(error),
            crate::spr::DrawingMutationArenaPoolAvailability::Ready | crate::spr::DrawingMutationArenaPoolAvailability::NotReady | crate::spr::DrawingMutationArenaPoolAvailability::Contended => None,
        };
        Self { arena_boot_fault }
    }
}

impl ArtifactEditor for DrawingPlayApp {
    fn completion_retirement_birth_bytes(value:&semio_framework_plugin::ArtifactToolCompletionValue<semio_framework_plugin::EditorApp<Self>>)->Option<usize>{completion::birth(value)}
    fn admit_completion_retirement(value:&mut Option<semio_framework_plugin::ArtifactToolCompletionValue<semio_framework_plugin::EditorApp<Self>>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{completion::admit(value,grant)}
    /// 📢️ The localized notices of the retained gesture owner's refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
        drawing_fault_notices()
    }

    type Snapshot = DrawingSnapshot;
    type Mutation = DrawingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = DrawingPresence;
    type PresenceMutation = DrawingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = DrawingCommand;

    const DIALECT: semio_framework_artifact_reference::Dialect = crate::DRAWING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;

    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        protocol::InteractionTopology { domains: [(DRAWING_INTERACTION_DOMAIN.into(), interaction::drawing_interaction_topology(doc.snapshot)),(DRAWING_POINT_DOMAIN.into(),interaction::drawing_point_topology(doc.snapshot))].into() }

})())
}


    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields;
    /// without it every live envelope load faults `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("drawing.child-projection"), error.to_string()))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::drawing_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::drawing_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::spr::drawing_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    /// ♻️ A returned presence read is retired through the bounded transient root cursor, the same
    /// factory every other presence-carrying plugin installs. `SharedValueRetirementFactory` — what
    /// this used to be — answers `Blocked` for as long as the returned `Arc` has any other strong
    /// reference, and the app close ladder has no way to release that reference, so every fixture
    /// that had published presence once blocked forever on "presence returned local owner is held
    /// during app close" and then aborted the test binary out of `FixedOperationRegistry::drop`.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default Drawing presence is the exact empty terminal")))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(DrawingArtifactStorePreparationFactory))
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        canvas_window::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        canvas_window::transient::register(registry)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        DrawingGestureProofs::bounded_first_step_tool_proofs().into_iter().chain(DrawingBoundedProofs::bounded_first_step_tool_proofs()).chain(DrawingPathProofs::bounded_first_step_tool_proofs()).chain(DrawingImageProofs::bounded_first_step_tool_proofs()).chain(DrawingExportProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn mounted_job_prepare_snapshot_read(operation:semio_framework_plugin::AppRenderOperationContext,snapshot:&Self::Snapshot)->bool{geometry_session::prepare(operation,snapshot)}
    fn mounted_job_maintenance_step(instance:u32,items:usize,bytes:usize)->Result<semio_framework_plugin::PluginCloseStep,Fault>{Ok(geometry_session::maintenance(instance,items,bytes))}
    fn mounted_job_close_step(instance:u32,items:usize,bytes:usize)->Result<semio_framework_plugin::PluginCloseStep,Fault>{Ok(geometry_session::close(instance,items,bytes))}
    fn mounted_jobs_terminal_is_empty(instance:u32)->bool{geometry_session::terminal_is_empty(instance)}
    fn pending_effects(_owner:&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,_view:Option<&semio_framework_plugin::ViewModel>)->Vec<semio_framework::kernel::Effect>{geometry_session::reconcile(doc)}

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(DrawingInstanceOperationOwner::new())
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DrawingGestureOperationJobFactory::new(&controller))?;
        registry.register(DrawingBoundedCommandJobFactory::new(&controller))?;
        registry.register(DrawingPathCommandJobFactory::new(&controller))?;
        registry.register(DrawingImageCommandJobFactory::new(&controller))?;
        registry.register(DrawingExportCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if DRAWING_EXPORT_TOOL_IDS.contains(&request.tool_id.as_str()) {return export_job::build(request).map(Some);}
        if DRAWING_IMAGE_TOOL_IDS.contains(&request.tool_id.as_str()) {return image_job::build(request).map(Some);}
        if DRAWING_PATH_TOOL_IDS.contains(&request.tool_id.as_str()) {return path_job::build(request).map(Some);}
        if DRAWING_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return drawing_bounded_tool_job(request).map(Some);
        }
        if !DRAWING_GESTURE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "Drawing gesture command does not match its exact registered tool"));
        }
        let view_state = request.context.view_state.clone().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.canvas.window-required"), "Drawing gesture commands require one concrete Canvas window instance"))?;
        let window_config = canvas_window::config::from_snapshot(request.context.window_config.as_ref());
        let window_transient = canvas_window::transient::from_snapshot(request.context.window_transient.as_ref());
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = DrawingGestureOperationPayload {
            source_identity: SceneIdentity{instance:request.app_instance_id,base:request.operation.base_revision.0,generation:request.operation.generation.0,revision:request.canonical_base_revision},
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            window_config,
            window_transient,
            view_state,
            history: request.history,
            interaction_state: request.interaction_state,
            instance_owner: request.instance_operation_owner,
            operation_context,
            active_utility_id: request.context.view_state.as_ref().map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility).to_owned(),
            completion: request.completion,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::drawing::config::app_schema_descriptor())
    }

    fn initial_snapshot() -> DrawingSnapshot {
        crate::standards::v1::subsets::any::schema::default_drawing_document("empty", None)
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(drawing_io())
    }

    /// 🎞️ `vector:out` (see `drawing_vector_media`) plus the inherited `document:out` default (the pack
    /// of `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, DrawingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "vector:out" => drawing_vector_media(doc.snapshot),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    // 🖼️ No override: whole-document replacement has no `Mutation` vehicle any more (banned
    // vocabulary — see `🧬️mutations/🦀️.rs`'s module doc). The default `None` disables the
    // generic `import_media("artifact:in")` port for drawing; explicit whole-document load/replace
    // stays reachable through the `set_snapshot`/`commit_document`/`load_document_json`/
    // `set_active_example` commands, which now emit `Effect::LoadDocument` (the sanctioned
    // non-history reset path) instead.

    fn clipboard_media_type()->Option<semio_framework_plugin::MediaType>{Some(clipboard::media_type())}

    fn copy_fragment(doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,interaction:&InteractionView<'_>)->Result<semio_framework_plugin::ClipboardFragment,semio_framework_plugin::ClipboardError>{
        clipboard::copy(doc.snapshot,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids)
    }

    fn cut_operations(doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,interaction:&InteractionView<'_>)->Vec<DrawingMutation>{
        clipboard::cut(doc.snapshot,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids).unwrap_or_default()
    }

    fn paste_operations(doc:&ArtifactView<'_,DrawingSnapshot>,fragment:&semio_framework_plugin::ClipboardFragment,placement:&semio_framework_plugin::kernel::PastePlacement)->Result<Vec<DrawingMutation>,semio_framework_plugin::ClipboardError>{
        clipboard::paste(doc.snapshot,clipboard::decode(fragment)?,placement,None).map(|(mutations,_)|mutations)
    }

    fn build_reserved_tool_job(request:semio_framework_plugin::ArtifactReservedToolJobRequest<semio_framework_plugin::EditorApp<Self>>)->Result<Option<semio_framework_plugin::ArtifactReservedToolJob>,Fault>{
        if !DRAWING_CLIPBOARD_RESERVED_IDS.contains(&request.tool_id.as_str()){return Ok(None);}
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(clipboard::job::DrawingClipboardJob::new(request)?)))
    }

    /// 🏷️ `app_commands!`'s generated `command_id()`.
    fn command_id(command: &DrawingCommand) -> &'static str {
        command.command_id()
    }

    /// 🌉️ Shell `{action, args}` → `DrawingCommand` (see `args_bridge`); the trait default refuses
    /// every app action, which left the whole ribbon/palette dead in the React shell.
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &DrawingCommand,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DrawingMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if DRAWING_GESTURE_TOOL_IDS.contains(&command.command_id()) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.retained-route"), "Drawing gesture commands are reachable only through their exact retained factory owner"));
        }
        let mut session = DrawingSession::new(view_state.map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility), doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str()));
        session.interaction.ids = interaction.selection(DRAWING_INTERACTION_DOMAIN).ids.clone();
        session.interaction.points = interaction.selection(DRAWING_POINT_DOMAIN).ids.clone();
        session.window_config = canvas_window::config::current(cfg);
        command.dispatch(doc, cfg, &mut session)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_drawing_body(body_key, doc, &canvas_window::config::current(cfg), &DrawingSession::default().preview(), &[], &[], view_state)
    }

    fn render_with_instance_operation_owner(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview = match doc.render_operation() {
            Some(operation) => owner
                .with_mut::<DrawingInstanceOperationOwner, _>(|owner| Ok(owner.preview_projection(geometry_session::identity(operation), drawing_active_utility(view_state))))
                .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner", error.message))?
                .unwrap_or_default(),
            None => DrawingGesturePreview::default(),
        };
        render_drawing_body(body_key, doc, &canvas_window::config::current(cfg), &preview, &[], &[], view_state)
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        if body_key == DRAWING_PLAY_BODY_PROPERTIES {
            return properties_panel::render(doc.snapshot, &interaction.selection(DRAWING_INTERACTION_DOMAIN).ids, semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state), &semio_framework_plugin::TreeWindows::for_body(view_state, body_key)).map(semio_framework_plugin::built_to_component_tree);
        }
        let preview=match doc.render_operation() {
            Some(operation)=>owner.with_mut::<DrawingInstanceOperationOwner,_>(|owner|Ok(owner.preview_projection(geometry_session::identity(operation),drawing_active_utility(view_state))))
                .map_err(|error|semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner",error.message))?.unwrap_or_default(),
            None=>DrawingGesturePreview::default(),
        };
        render_drawing_body(body_key,doc,&canvas_window::config::current(cfg),&preview,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids,&interaction.selection(DRAWING_POINT_DOMAIN).ids,view_state)
    }

    fn window_engagements(doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        status::engagements(doc.snapshot.layers.len(),0,String::new(),view_state)
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, DrawingSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        status::engagements(doc.snapshot.layers.len(),interaction.selection(DRAWING_INTERACTION_DOMAIN).ids.len(),canvas_window::transient::current(transient).engagement_input,view_state)
    }
}
//#endregion 🔖️DrawingPlayApp

//#region 🔖️Io
/// 🌱️ Builds the single canonical non-history document-reset effect for Drawing.
/// 🔁️ The spr is a fresh, edit-free op-log (`store::empty_document_spr`, the `🏗️fem`/process3d
/// shape) — never a live `ArtifactEnvelope` minted just to print it: such an envelope owns a bounded
/// retirement authority, and dropping it at the end of this function trapped the guest (`artifact
/// envelope terminal shell reached Drop before its app-owned bounded retirement authority detached
/// every nested owner`) the moment the react shell dispatched its boot `setActiveExample`
/// (ticket 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-16).
pub(crate) fn drawing_reset_document_effect(scene: &DrawingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <DrawingSnapshot as ArtifactPack>::encode_pack(scene);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr(&scene.id.to_string_owner(), DRAWING_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

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
    let (svg, _width, _height) = crate::standards::v1::subsets::any::io::drawing_document_to_svg(doc).map_err(|error| MediaError::Payload("vector:out".into(), error))?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: svg } })
}
//#endregion 🔖️Io

//#region 🔖️Manifest
pub fn create_drawing_app() -> semio_framework_plugin::AppDefinition {
    let engagement = WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("drawing-canvas-engagement".into()),
            value: Some(String::new()),
            placeholder: Some("Layer name".into()),
            on_change: Some(drawing_manifest_action("engagementInput")),
            on_submit: Some(drawing_manifest_action("engagementSubmit")),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "drawing-layer-count".into(), text: "0 layers · 0 selected".into() }]),
        possible_engagements: None,
    };
    Editor::builder(crate::DRAWING_DIALECT).document(["semio", "drawing"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("drawing")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(DRAWING_PLAY_WINDOW_CANVAS, LocalizedLabel::native("Canvas", "Leinwand"), DRAWING_PLAY_BODY_COMPOSITE, semio_framework_ui_contract::SurfaceKind::Canvas2d, engagement, "pen-tool")
            .panel_tab_def(layers_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(properties_panel::definition())
            // ✏️ Palette-visible content operations.
            .action_with(
                semio_framework_plugin::ActionDefinition::bounded_catalog("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Appends a new layer of the given kind (a rectangle, ellipse, line, polygon, freehand path, text, image, group, boolean or trace) to the top of the drawing.",
                        "Fügt der Zeichnung oben eine neue Ebene der angegebenen Art hinzu (Rechteck, Ellipse, Linie, Polygon, Pfad, Text, Bild, Gruppe, Boolean oder Nachzeichnung).",
                    ))
                    .use_when(["draw a rectangle", "add a rectangle", "draw an ellipse or circle", "add a line", "add a polygon", "add a text layer", "add a new layer"])
                    .with_args([drawing_layer_kind_arg()]),
            )
            .action_interactive_job("addLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition { in_palette:false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("importImage",LocalizedLabel::native("Import PNG Image","PNG-Bild importieren"),ActionKind::Mutation) })
            .action_interactive_job("importImage", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::bounded_catalog("combineBoolean", LocalizedLabel::native("Combine Boolean", "Boolean kombinieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Combines two or more existing layers into one boolean layer (union, intersect, subtract or exclude). Defaults to the current selection when no ids are given.",
                        "Verbindet zwei oder mehr vorhandene Ebenen zu einer Boolean-Ebene (Vereinigung, Schnitt, Differenz oder Ausschluss). Ohne Ids wird die aktuelle Auswahl verwendet.",
                    ))
                    .use_when(["union these shapes", "subtract one shape from another", "intersect the selection", "combine shapes"])
                    .with_args([
                        semio_framework_plugin::ActionArgDef::select(
                            "operation",
                            LocalizedLabel::native("Operation", "Operation"),
                            vec![
                                semio_framework_plugin::ActionArgOption::new("union", LocalizedLabel::native("Union", "Vereinigung")),
                                semio_framework_plugin::ActionArgOption::new("intersection", LocalizedLabel::native("Intersect", "Schnitt")),
                                semio_framework_plugin::ActionArgOption::new("difference", LocalizedLabel::native("Subtract", "Differenz")),
                                semio_framework_plugin::ActionArgOption::new("xor", LocalizedLabel::native("Exclude", "Ausschluss")),
                            ],
                        )
                        .default_value(&"union")
                        .required(),
                        semio_framework_plugin::ActionArgDef::text_list("ids", LocalizedLabel::native("Layer Ids", "Ebenen-Ids")).describe(LocalizedLabel::native("Ids of the layers to combine; empty uses the current selection.", "Ids der zu kombinierenden Ebenen; leer verwendet die aktuelle Auswahl.")),
                    ]),
            )
            .action_interactive_job("combineBoolean", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("editSelection", LocalizedLabel::native("Arrange Selection", "Auswahl anordnen"), ActionKind::Mutation)
                .describe(LocalizedLabel::native(
                    "Arranges the given layers, or the current selection when none are given, in one undoable step: groups or ungroups them, duplicates or deletes them, moves them one step or all the way to the front or back, aligns their edges or centres (two or more layers), or distributes them evenly (three or more). Every layer must be unlocked; grouping, duplicating, deleting and reordering need layers of one parent group.",
                    "Ordnet die angegebenen Ebenen, oder die aktuelle Auswahl, wenn keine angegeben sind, in einem rückgängig machbaren Schritt an: gruppiert sie oder hebt ihre Gruppierung auf, dupliziert oder löscht sie, verschiebt sie eine Ebene oder ganz nach vorne oder hinten, richtet ihre Kanten oder Mitten aus (ab zwei Ebenen) oder verteilt sie gleichmäßig (ab drei Ebenen). Jede Ebene muss entsperrt sein; Gruppieren, Duplizieren, Löschen und Umordnen verlangen Ebenen derselben Gruppe.",
                ))
                .use_when(["group the selected shapes", "ungroup the selection", "duplicate the selection", "delete the selected layers", "bring the selection forward", "send the selection to the back", "align the selected layers", "distribute the selection evenly"])
                .with_args(vec![semio_framework_plugin::ActionArgDef::select("operation", LocalizedLabel::native("Operation", "Aktion"), vec![
                    semio_framework_plugin::ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppieren")),
                    semio_framework_plugin::ActionArgOption::new("duplicate", LocalizedLabel::native("Duplicate", "Duplizieren")),
                    semio_framework_plugin::ActionArgOption::new("delete", LocalizedLabel::native("Delete", "Löschen")),
                    semio_framework_plugin::ActionArgOption::new("ungroup", LocalizedLabel::native("Ungroup", "Gruppierung aufheben")),
                    semio_framework_plugin::ActionArgOption::new("bringForward", LocalizedLabel::native("Bring Forward", "Eine Ebene nach vorne")),
                    semio_framework_plugin::ActionArgOption::new("sendBackward", LocalizedLabel::native("Send Backward", "Eine Ebene nach hinten")),
                    semio_framework_plugin::ActionArgOption::new("bringToFront", LocalizedLabel::native("Bring to Front", "In den Vordergrund")),
                    semio_framework_plugin::ActionArgOption::new("sendToBack", LocalizedLabel::native("Send to Back", "In den Hintergrund")),
                    semio_framework_plugin::ActionArgOption::new("alignLeft", LocalizedLabel::native("Align Left", "Links ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("alignCenter", LocalizedLabel::native("Align Center", "Horizontal zentrieren")),
                    semio_framework_plugin::ActionArgOption::new("alignRight", LocalizedLabel::native("Align Right", "Rechts ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("alignTop", LocalizedLabel::native("Align Top", "Oben ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("alignMiddle", LocalizedLabel::native("Align Middle", "Vertikal zentrieren")),
                    semio_framework_plugin::ActionArgOption::new("alignBottom", LocalizedLabel::native("Align Bottom", "Unten ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("distributeHorizontal", LocalizedLabel::native("Distribute Horizontally", "Horizontal verteilen")),
                    semio_framework_plugin::ActionArgOption::new("distributeVertical", LocalizedLabel::native("Distribute Vertically", "Vertikal verteilen")),
                ]).required(), semio_framework_plugin::ActionArgDef::text_list("ids", LocalizedLabel::native("Layers", "Ebenen"))]))
            .action_interactive_job("editSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("editPath", LocalizedLabel::native("Edit Path", "Pfad bearbeiten"), ActionKind::Mutation)
                .describe(LocalizedLabel::native(
                    "Edits one unlocked path layer's geometry in one undoable step, given as a JSON node edit: moves or deletes chosen anchors and control handles, sets one point's position or coordinate, splits, deletes or converts a segment between line and cubic curve, opens, closes or joins the path at a node, or reverses its direction.",
                    "Bearbeitet die Geometrie einer entsperrten Pfadebene in einem rückgängig machbaren Schritt, angegeben als JSON-Knotenbearbeitung: verschiebt oder löscht gewählte Anker- und Steuerpunkte, setzt die Position oder eine Koordinate eines Punkts, teilt, löscht oder wandelt ein Segment zwischen Linie und kubischer Kurve um, öffnet, schließt oder verbindet den Pfad an einem Knoten oder kehrt seine Richtung um.",
                ))
                .use_when(["move a path node", "drag a bezier handle", "delete path points", "split a path segment", "convert a segment to a curve", "close the path", "reverse the path direction"])
                .with_args([semio_framework_plugin::ActionArgDef::text("layerId", LocalizedLabel::native("Path", "Pfad")).required(), semio_framework_plugin::ActionArgDef::json_text("edit", LocalizedLabel::native("Node Edit", "Knotenbearbeitung")).required()]))
            .action_interactive_job("editPath", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("editFill", LocalizedLabel::native("Edit Fill", "Füllung bearbeiten"), ActionKind::Mutation)
                .describe(LocalizedLabel::native(
                    "Edits one unlocked layer's fill in one undoable step, given as a JSON fill edit: sets its type (none, solid, linear or radial gradient), a colour or its opacity, a gradient coordinate or stop offset, or adds or removes a gradient stop.",
                    "Bearbeitet die Füllung einer entsperrten Ebene in einem rückgängig machbaren Schritt, angegeben als JSON-Füllungsbearbeitung: setzt ihren Typ (keine, einfarbig, linearer oder radialer Verlauf), eine Farbe oder deren Deckkraft, eine Verlaufskoordinate oder den Versatz eines Farbstopps, oder fügt einen Farbstopp hinzu oder entfernt ihn.",
                ))
                .use_when(["change the fill colour", "make the fill a gradient", "add a gradient stop", "set the fill opacity", "remove the fill"])
                .with_args([semio_framework_plugin::ActionArgDef::text("layerId", LocalizedLabel::native("Layer", "Ebene")).required(), semio_framework_plugin::ActionArgDef::json_text("edit", LocalizedLabel::native("Fill Edit", "Füllungsbearbeitung")).required()]))
            .action_interactive_job("editFill", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation))
            .action_audience("deleteSelection", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionLeft", LocalizedLabel::native("Nudge Left", "Auswahl nach links verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionLeft", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionLeft", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionLeftFast", LocalizedLabel::native("Nudge Left Fast", "Auswahl schnell nach links verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionLeftFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionLeftFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionRight", LocalizedLabel::native("Nudge Right", "Auswahl nach rechts verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionRight", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionRight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionRightFast", LocalizedLabel::native("Nudge Right Fast", "Auswahl schnell nach rechts verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionRightFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionRightFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionUp", LocalizedLabel::native("Nudge Up", "Auswahl nach oben verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionUp", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionUp", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionUpFast", LocalizedLabel::native("Nudge Up Fast", "Auswahl schnell nach oben verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionUpFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionUpFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionDown", LocalizedLabel::native("Nudge Down", "Auswahl nach unten verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionDownFast", LocalizedLabel::native("Nudge Down Fast", "Auswahl schnell nach unten verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionDownFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionDownFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::bounded_catalog("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Replaces the whole drawing with one of the plugin's declared playground examples, by example id.",
                        "Ersetzt die gesamte Zeichnung durch eines der deklarierten Beispiele des Plugins, anhand der Beispiel-Id.",
                    ))
                    .use_when(["load the demo drawing", "open an example"])
                    .with_args([semio_framework_plugin::ActionArgDef::text("exampleId", LocalizedLabel::native("Example", "Beispiel")).required()]),
            )
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 📤️ Export — a host download effect (`DownloadMediaExport`), never a document operation.
            .action_with(
                semio_framework_plugin::ActionDefinition { icon_id: "download".into(), ..semio_framework_plugin::ActionDefinition::bounded_catalog("exportDocument", LocalizedLabel::native("Export Drawing", "Zeichnung exportieren"), ActionKind::View) }
                    .describe(LocalizedLabel::native(
                        "Renders the drawing to a downloadable PNG image, vector-painted PDF page or SVG document.",
                        "Rendert die Zeichnung als PNG-Bild, vektorgezeichnete PDF-Seite oder SVG-Dokument in eine herunterladbare Datei.",
                    ))
                    .use_when(["export the document as png", "export the document as pdf", "save this drawing as an svg", "download the drawing"])
                    .with_args([semio_framework_plugin::ActionArgDef::select(
                        "format",
                        LocalizedLabel::native("Format", "Format"),
                        vec![
                            semio_framework_plugin::ActionArgOption::new("png", LocalizedLabel::native("PNG", "PNG")),
                            semio_framework_plugin::ActionArgOption::new("pdf", LocalizedLabel::native("PDF", "PDF")),
                            semio_framework_plugin::ActionArgOption::new("svg", LocalizedLabel::native("SVG", "SVG")),
                        ],
                    )
                    .default_value(&export_document::DEFAULT_EXPORT_FORMAT),
                    semio_framework_plugin::ActionArgDef {schema:semio_framework::ArgSchema::number(Some(1.0),Some(16384.0),Some(1.0),true),..semio_framework_plugin::ActionArgDef::index("width",LocalizedLabel::native("PNG Width in Pixels","PNG-Breite in Pixeln"))},
                    semio_framework_plugin::ActionArgDef {schema:semio_framework::ArgSchema::number(Some(1.0),Some(16384.0),Some(1.0),true),..semio_framework_plugin::ActionArgDef::index("height",LocalizedLabel::native("PNG Height in Pixels","PNG-Höhe in Pixeln"))},
                    semio_framework_plugin::ActionArgDef::toggle("transparent",LocalizedLabel::native("Transparent PNG Background","Transparenter PNG-Hintergrund")).default_value(&true)]),
            )
            .action_interactive_job("exportDocument", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🔧️ Internal content operations — inspector/layer-panel/import-bound, not palette commands,
            // but every one of them is an intent an agent can hold, so they stay agent-addressable.
            .action_with(
                drawing_internal_action("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Replaces the entire drawing document with a supplied snapshot.", "Ersetzt das gesamte Zeichnungsdokument durch einen übergebenen Snapshot."))
                    .use_when(["replace the whole drawing"]),
            )
            .action_interactive_job("setSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("commitDocument", LocalizedLabel::native("Commit Document", "Dokument übernehmen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Commits a supplied snapshot as the drawing's next revision.", "Schreibt einen übergebenen Snapshot als nächste Revision der Zeichnung fest."))
                    .use_when(["commit this document state"]),
            )
            .action_interactive_job("commitDocument", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("loadDocumentJson", LocalizedLabel::native("Load Document JSON", "Dokument-JSON laden"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Loads a whole drawing document from JSON text.", "Lädt ein vollständiges Zeichnungsdokument aus JSON-Text."))
                    .use_when(["load this drawing from json"])
                    .with_args([semio_framework_plugin::ActionArgDef::json_text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()]),
            )
            .action_interactive_job("loadDocumentJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("setSelectedOpacity", LocalizedLabel::native("Set Selected Opacity", "Deckkraft der Auswahl festlegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Sets the opacity of every currently selected layer, from 0 (invisible) to 1 (opaque).", "Setzt die Deckkraft aller ausgewählten Ebenen, von 0 (unsichtbar) bis 1 (deckend)."))
                    .use_when(["make the selection half transparent", "change the opacity"])
                    .with_args([semio_framework_plugin::ActionArgDef::slider("value", LocalizedLabel::native("Opacity", "Deckkraft"), 0.0, 1.0).required()]),
            )
            .action_interactive_job("setSelectedOpacity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Creates a layer of the given kind at a drop target in the layer tree.", "Erzeugt eine Ebene der angegebenen Art an einer Ablagestelle im Ebenenbaum."))
                    .with_args([drawing_layer_kind_arg(), drawing_target_row_arg(), drawing_drop_position_arg()]),
            )
            .action_interactive_job("dropLayerKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Reorders one layer within the layer tree, before or after a target row or inside a group.", "Ordnet eine Ebene im Ebenenbaum um — vor oder nach einer Zielzeile oder in eine Gruppe hinein."))
                    .use_when(["move a layer up", "reorder the layers", "put this layer in the group"])
                    .with_args([drawing_layer_id_arg(), drawing_target_row_arg(), drawing_drop_position_arg()]),
            )
            .action_interactive_job("moveLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Removes one layer from the drawing by id.", "Entfernt eine Ebene anhand ihrer Id aus der Zeichnung."))
                    .use_when(["delete a layer", "remove this shape"])
                    .with_args([drawing_layer_id_arg()]),
            )
            .action_interactive_job("deleteLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Copies one layer and inserts the copy next to the original.", "Kopiert eine Ebene und fügt die Kopie neben dem Original ein."))
                    .use_when(["duplicate this layer", "copy the shape"])
                    .with_args([drawing_layer_id_arg()]),
            )
            .action_interactive_job("duplicateLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Flips one layer between visible and hidden.", "Schaltet eine Ebene zwischen sichtbar und ausgeblendet um."))
                    .use_when(["hide this layer", "show the layer again"])
                    .with_args([drawing_layer_id_arg()]),
            )
            .action_interactive_job("toggleLayerVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Sets one named property of one layer — its name, opacity, visibility, lock, blend mode, boolean operation, fill colour, stroke width, transform or trace parameters.",
                        "Setzt eine benannte Eigenschaft einer Ebene — Name, Deckkraft, Sichtbarkeit, Sperre, Mischmodus, Boolean-Operation, Füllfarbe, Strichstärke, Transformation oder Nachzeichnungsparameter.",
                    ))
                    .use_when(["rename a layer", "change the fill colour", "set the stroke width", "rotate or move a layer", "lock a layer"])
                    .with_args([drawing_layer_id_arg(), drawing_layer_field_arg(), drawing_layer_value_arg()]),
            )
            .action_interactive_job("patchLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Sets the same named property on several layers at once — see Patch Layer for the field vocabulary.", "Setzt dieselbe benannte Eigenschaft auf mehreren Ebenen gleichzeitig — siehe Ebene aktualisieren für die Feldliste."))
                    .use_when(["set the colour of all selected layers", "hide several layers"])
                    .with_args([semio_framework_plugin::ActionArgDef::text_list("layerIds", LocalizedLabel::native("Layer Ids", "Ebenen-Ids")).required(), drawing_layer_field_arg(), drawing_layer_value_arg()]),
            )
            .action_interactive_job("patchLayers", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🖱️ Internal pointer/gesture vocabulary — commit-time handlers emit operations, the rest are
            // pure View. All of it is `CapabilityAudience::Input`: the canvas feeds these, agents never do.
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::Mutation, "mouse-pointer").input_event())
            .action_interactive_job("canvasPointerDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::Mutation, "mouse-pointer").input_event())
            .action_interactive_job("canvasPointerUp", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("canvasDoubleClick", LocalizedLabel::native("Canvas Double Click", "Leinwand-Doppelklick"), ActionKind::Mutation))
            .action_interactive_job("canvasDoubleClick", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("canvasCommitDraft", LocalizedLabel::native("Canvas Commit Draft", "Leinwand-Entwurf übernehmen"), ActionKind::Mutation))
            .action_interactive_job("canvasCommitDraft", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View, "mouse-pointer").input_event())
            .action_interactive_job("canvasPointerMove", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("canvasEscape", LocalizedLabel::native("Canvas Escape", "Leinwand abbrechen"), ActionKind::View))
            .action_interactive_job("canvasEscape", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 👁️ Ephemeral view state — selection/hover are framework-owned now (see `.interaction(...)`
            // below): interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity auto-inject, never declared here (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
            .action_with(semio_framework_plugin::ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand").input_event())
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 📷️ Camera — session-only runtime pose, never a document operation.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            .action_interactive_job("setCameraZoom", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🤖️ Named after a drag-and-drop gesture but fully specified by its arguments, so an
            // agent can reach it: declared `Agent` on purpose, which is also what stops the
            // capability audit asking about it again.
            .action_audience("dropLayerKind", semio_framework_plugin::CapabilityAudience::Agent)
            // ⚠️ Discards content no later verb reconstructs — the gateway asks a human first.
            .action_destructive("deleteLayer")
            .action_destructive("setSnapshot")
            .action_destructive("loadDocumentJson")
            .action_destructive("setActiveExample")
            .action_destructive("commitDocument")
            .action_destructive("exportDocument")
            // 🧰️ Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(drawing_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("editNodes", LocalizedLabel::native("Edit Nodes", "Knoten bearbeiten"), "spline", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("pen", LocalizedLabel::native("Pen", "Stift"), "pen-tool", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeRect", LocalizedLabel::native("Rectangle", "Rechteck"), "rectangle-tool", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeEllipse", LocalizedLabel::native("Ellipse", "Ellipse"), "circle", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeLine", LocalizedLabel::native("Line", "Linie"), "minus", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapePolygon", LocalizedLabel::native("Polygon", "Polygon"), "hexagon", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("booleanCombine", LocalizedLabel::native("Boolean", "Boolean"), "combine", "Combine", UtilityCategory::Utilities))
            .utility(drawing_utility("trace", LocalizedLabel::native("Trace", "Nachzeichnen"), "scan-line", "Combine", UtilityCategory::Utilities))
            .utility(drawing_utility("transformMove", LocalizedLabel::native("Pan", "Verschieben"), "move", "View", UtilityCategory::Utilities))
            .window_kind_utilities(DRAWING_PLAY_WINDOW_CANVAS, vec![
                "selectMarquee".into(), "selectLasso".into(), "selectDirect".into(), "editNodes".into(),
                "pen".into(), "shapeRect".into(), "shapeEllipse".into(), "shapeLine".into(), "shapePolygon".into(),
                "booleanCombine".into(), "trace".into(), "transformMove".into(),
            ])
            .window_kind_initial_utility(DRAWING_PLAY_WINDOW_CANVAS, "selectDirect")
            // 🕹️ The framework-owned "strokes" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — covers both the layers panel tree
            // (`.interaction_domain("strokes")?`) and the canvas's pick/marquee/lasso layer selection;
            // auto-injects interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity, replacing every deleted bespoke setSelection/setHover/
            // clearSelection/selectAll action.
            .interaction(InteractionDefinition {
                id: DRAWING_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Strokes", "Striche"),
                granularities: vec![GranularityDefinition { id: DRAWING_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Stroke", "Strich"), icon_id: "pen-tool".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .interaction(InteractionDefinition {
                id:DRAWING_POINT_DOMAIN.into(),
                label:LocalizedLabel::native("Path Points","Pfadpunkte"),
                granularities:vec![GranularityDefinition {id:DRAWING_POINT_GRANULARITY.into(),label:LocalizedLabel::native("Point","Punkt"),icon_id:"spline".into()}],
                hierarchy:HierarchyProvider::Topology,
                hover:HoverSpec::default(),
                selection:SelectionSpec {modes:vec![SelectionMode::Multiple],methods:vec![SelectionMethod::Pick],merges:vec![MergeMode::Replace,MergeMode::Additive,MergeMode::Subtractive,MergeMode::Invertive],transitive:false,broadcast:true},
            })
            .window_kind_interactions(DRAWING_PLAY_WINDOW_CANVAS, vec![InteractionRef::new(DRAWING_INTERACTION_DOMAIN),InteractionRef::new(DRAWING_POINT_DOMAIN)])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("escape", "canvasEscape")
            .keybinding("enter", "canvasCommitDraft")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("arrowleft", "nudgeSelectionLeft")
            .keybinding("shift+arrowleft", "nudgeSelectionLeftFast")
            .keybinding("arrowright", "nudgeSelectionRight")
            .keybinding("shift+arrowright", "nudgeSelectionRightFast")
            .keybinding("arrowup", "nudgeSelectionUp")
            .keybinding("shift+arrowup", "nudgeSelectionUpFast")
            .keybinding("arrowdown", "nudgeSelectionDown")
            .keybinding("shift+arrowdown", "nudgeSelectionDownFast")
            .default_layout(edit::layout())
            // 🎯️ Typed channel surface — the SAME `drawing_io()` the trait's `io()` override returns,
            // declared on the manifest so the committed descriptor carries it too. Without this the
            // app shipped a DEFAULT `AppIo` (`artifactSchema: ""`, `ports: []`) while
            // `DrawingPlayApp::export_media` answered `vector:out` for real, so every host that reads
            // the descriptor to decide what a drawing can be exported through — the MCP gateway's
            // `installed_artifact_kinds` among them — saw an app with no export surface at all
            // (`📓️wr2-headless-command-response-wire.md` §7.2, measured as `declaredExportFormats: []`).
            // Mirrors `writer_io()`/`lowpoly_io()`'s identical wiring.
            .io(drawing_io())
            // 📚️ Examples are declared once, on the subset (`🪆️subsets/✳️any/🦀️.rs`'s `examples()`,
            // reached by the shell through `SubsetDeclaration.examples`), never a second time on this
            // builder — `setActiveExample` resolves its `example_id` against that same slice, so the
            // switcher's ids and the command's ids cannot drift apart.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
/// 🚪️ Host document-archive door law for the example load (`setActiveExample` → `LoadDocument`).
#[cfg(test)]
#[path = "🧪️tests/🔬️archive-load/🦀️.rs"]
mod archive_load_tests;
/// 🌉️ The action bridge folds camelCase keys and JSON values into typed drawing commands.
#[cfg(test)]
#[path = "🧪️tests/🌉️args-bridge/🦀️.rs"]
mod args_bridge_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts

#[path="🧵️geometry/🦀️.rs"]
pub mod geometry_session;
#[path="📬️completion/🦀️.rs"]
mod completion;

/// 📣️ The en/de notices of every `drawing.gesture.*` refusal code (design §20.12).
pub fn drawing_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, semio_framework_ui_locale::LocalizedLabel); 9]> = std::sync::LazyLock::new(|| {
        [
            ("drawing.gesture.retained-route", semio_framework_ui_locale::LocalizedLabel::native("This drawing gesture can only run in an open drawing window.", "Diese Zeichengeste läuft nur in einem geöffneten Zeichenfenster.")),
            ("drawing.gesture.closing", semio_framework_ui_locale::LocalizedLabel::native("The drawing is closing; the gesture was not applied.", "Die Zeichnung wird geschlossen; die Geste wurde nicht angewendet.")),
            ("drawing.gesture.saturated", semio_framework_ui_locale::LocalizedLabel::native("Too many drawing gestures are running at once.", "Zu viele Zeichengesten laufen gleichzeitig.")),
            ("drawing.gesture.owner", semio_framework_ui_locale::LocalizedLabel::native("The drawing gesture ended before it could continue.", "Die Zeichengeste endete, bevor sie fortgesetzt werden konnte.")),
            ("drawing.gesture.point-capacity", semio_framework_ui_locale::LocalizedLabel::native("The stroke has too many points.", "Der Strich hat zu viele Punkte.")),
            ("drawing.gesture.query-owner", semio_framework_ui_locale::LocalizedLabel::native("Another drawing gesture owns this query.", "Eine andere Zeichengeste besitzt diese Abfrage.")),
            ("drawing.gesture.query-capacity", semio_framework_ui_locale::LocalizedLabel::native("Too many shapes match this point.", "Zu viele Formen treffen diesen Punkt.")),
            ("drawing.gesture.query-output-capacity", semio_framework_ui_locale::LocalizedLabel::native("The interaction produced too many results.", "Die Interaktion erzeugte zu viele Ergebnisse.")),
            ("drawing.gesture.command", semio_framework_ui_locale::LocalizedLabel::native("This action cannot run during a drawing gesture.", "Diese Aktion kann während einer Zeichengeste nicht ausgeführt werden.")),
        ]
    });
    &*NOTICES
}
