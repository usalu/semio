//! 🎼️ Note command enum — the closed dispatch surface of the editor and the host-action bridge into it.

use crate::editor::note::commands::ink_apply_events;
use crate::editor::note::commands::{add_block, delete_block, delete_selection, duplicate_block, duplicate_selection, move_block, patch_blocks};
use crate::editor::note::commands::{engagement_input, engagement_submit, navigator_engagement_input};
use crate::editor::note::commands::save_download;
use crate::editor::note::commands::{nudge_selection, nudge_selection_down, nudge_selection_down_fast, nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast};
use crate::editor::note::commands::{set_active_example, load_document_json};
use crate::editor::note::commands::{set_camera, set_camera_zoom};
use crate::editor::note::commands::{set_eraser_radius, set_pencil_width};
use crate::editor::note::commands::{set_grid_opacity, set_grid_spacing, set_grid_subdivisions, set_grid_visible};
use crate::editor::note::commands::{set_snap_enabled, set_snap_grid_spacing};
use crate::editor::note::dispatch_ctx::NoteDispatchCtx;
use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{NoConfig, NoConfigMutation};

semio_framework_plugin::app_commands! {
    /// 🎯️ `NotePlayApp::Command` — the SOLE dispatch surface for note's own behavior (B1 pure-trait
    /// migration, mirroring `shooting_protocol::ShootingCommand`). One variant per action id the pre-B1
    /// `NotePlayApp::handle_action` matched; combined `"x" | "y"` arms (e.g. the old
    /// `"setGridVisible" | "toggleGrid"` alias, never independently wired anywhere in the note ui crate
    /// or its hosts) collapse onto the one surviving action id's command instead of keeping a dead
    /// synonym. Row order is the binary variant ordinal: appending is safe, reordering is a wire-format
    /// break.
    pub enum NoteCommand for NoteSnapshot, NoteMutation, NoConfig, NoConfigMutation, ctx = NoteDispatchCtx {
        "setGridVisible" as "set-grid-visible" => set_grid_visible::SetGridVisible,
        "setGridSpacing" as "set-grid-spacing" => set_grid_spacing::SetGridSpacing,
        "setGridSubdivisions" as "set-grid-subdivisions" => set_grid_subdivisions::SetGridSubdivisions,
        "setGridOpacity" as "set-grid-opacity" => set_grid_opacity::SetGridOpacity,
        "setSnapEnabled" as "set-snap-enabled" => set_snap_enabled::SetSnapEnabled,
        "setSnapGridSpacing" as "set-snap-grid-spacing" => set_snap_grid_spacing::SetSnapGridSpacing,
        "setPencilWidth" as "set-pencil-width" => set_pencil_width::SetPencilWidth,
        "setEraserRadius" as "set-eraser-radius" => set_eraser_radius::SetEraserRadius,
        "addBlock" as "add-block" => add_block::AddBlock,
        "moveBlock" as "move-block" => move_block::MoveBlock,
        "deleteBlock" as "delete-block" => delete_block::DeleteBlock,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "duplicateBlock" as "duplicate-block" => duplicate_block::DuplicateBlock,
        "duplicateSelection" as "duplicate-selection" => duplicate_selection::DuplicateSelection,
        "patchBlocks" as "patch-blocks" => patch_blocks::PatchBlocks,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "loadDocumentJson" as "load-document-json" => load_document_json::LoadDocumentJson,
        "inkApplyEvents" as "ink-apply-events" => ink_apply_events::InkApplyEvents,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "nudgeSelection" as "nudge-selection" => nudge_selection::NudgeSelection,
        "nudgeSelectionUp" as "nudge-selection-up" => nudge_selection_up::NudgeSelectionUp,
        "nudgeSelectionDown" as "nudge-selection-down" => nudge_selection_down::NudgeSelectionDown,
        "nudgeSelectionLeft" as "nudge-selection-left" => nudge_selection_left::NudgeSelectionLeft,
        "nudgeSelectionRight" as "nudge-selection-right" => nudge_selection_right::NudgeSelectionRight,
        "nudgeSelectionUpFast" as "nudge-selection-up-fast" => nudge_selection_up_fast::NudgeSelectionUpFast,
        "nudgeSelectionDownFast" as "nudge-selection-down-fast" => nudge_selection_down_fast::NudgeSelectionDownFast,
        "nudgeSelectionLeftFast" as "nudge-selection-left-fast" => nudge_selection_left_fast::NudgeSelectionLeftFast,
        "nudgeSelectionRightFast" as "nudge-selection-right-fast" => nudge_selection_right_fast::NudgeSelectionRightFast,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "navigatorEngagementInput" as "navigator-engagement-input" => navigator_engagement_input::NavigatorEngagementInput,
        "saveDownload" as "save-download" => save_download::SaveDownload,
    }
}

/// 🎯️ Folds the host's `{action, args}` vocabulary (camelCase keys, JSON floats, control `value`s) into
/// the snake_case `FromValue` payloads of `🎮️commands/*`, one arm per `NoteCommand` row.
pub mod args_bridge {
    use super::NoteCommand;
    use semio_framework_plugin::{DslValue, Fault, FaultCode, FaultOrigin};

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

    fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    fn integral(value: DslValue) -> DslValue {
        match value {
            DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
            }
            DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    /// 🔁️ Snake-cases every key of `args`, applies `aliases` (snake_case source → destination, first
    /// present source wins and never overwrites a present destination) and seeds `defaults` for keys
    /// still absent.
    fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> Vec<(String, DslValue)> {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        if let Some(DslValue::Object(object)) = args {
            for (key, value) in object {
                put(&mut entries, &snake(key), integral(value.clone()));
            }
        }
        for (from, into) in aliases {
            if entries.iter().any(|(key, _)| key == into) {
                continue;
            }
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
                put(&mut entries, into, value);
            }
        }
        for (key, value) in defaults {
            if !entries.iter().any(|(existing, _)| existing == key) {
                entries.push(((*key).to_string(), value.clone()));
            }
        }
        entries
    }

    fn only(entries: Vec<(String, DslValue)>, keys: &[&str]) -> DslValue {
        DslValue::Object(entries.into_iter().filter(|(key, _)| keys.contains(&key.as_str())).collect())
    }

    fn map(entries: &mut [(String, DslValue)], key: &str, convert: impl Fn(DslValue) -> DslValue) {
        if let Some(slot) = entries.iter_mut().find(|(existing, _)| existing == key) {
            slot.1 = convert(std::mem::replace(&mut slot.1, DslValue::Null));
        }
    }

    /// 📝️ Prints a host control value into the `String` fields the text verbs carry.
    fn text(value: DslValue) -> DslValue {
        match value {
            DslValue::String(_) => value,
            DslValue::Null => DslValue::String(String::new()),
            other => DslValue::String(semio_framework_pack_json::to_json_string(&other)),
        }
    }

    /// 🔢️ Slider/number controls may deliver their value as text.
    fn number(value: DslValue) -> DslValue {
        match &value {
            DslValue::String(raw) => raw.trim().parse::<f64>().map(|parsed| DslValue::Number(semio_framework_value::Number::Float(parsed))).unwrap_or(value),
            _ => value,
        }
    }

    /// ☑️ Toggle controls may deliver their value as text.
    fn boolean(value: DslValue) -> DslValue {
        match &value {
            DslValue::String(raw) if raw == "true" || raw == "false" => DslValue::Bool(raw == "true"),
            _ => value,
        }
    }

    fn list(value: DslValue) -> DslValue {
        match value {
            DslValue::Array(_) | DslValue::Null => value,
            other => DslValue::Array(vec![other]),
        }
    }

    fn decode<T: semio_framework_value::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("note action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<NoteCommand, Fault> {
        const BLOCK: &[(&str, &str)] = &[("id", "block_id"), ("value", "block_id")];
        let string = |value: &str| DslValue::String(value.into());
        let zero = || DslValue::Number(semio_framework_value::Number::Float(0.0));
        let empty = || DslValue::Object(Vec::new());
        let value_number = || {
            let mut entries = fold(args, &[], &[]);
            map(&mut entries, "value", number);
            only(entries, &["value"])
        };
        let value_boolean = || {
            let mut entries = fold(args, &[("checked", "value")], &[]);
            map(&mut entries, "value", boolean);
            only(entries, &["value"])
        };
        Ok(match action {
            "setGridVisible" => NoteCommand::SetGridVisible(decode(action, value_boolean())?),
            "setGridSpacing" => NoteCommand::SetGridSpacing(decode(action, value_number())?),
            "setGridSubdivisions" => NoteCommand::SetGridSubdivisions(decode(action, value_number())?),
            "setGridOpacity" => NoteCommand::SetGridOpacity(decode(action, value_number())?),
            "setSnapEnabled" => NoteCommand::SetSnapEnabled(decode(action, value_boolean())?),
            "setSnapGridSpacing" => NoteCommand::SetSnapGridSpacing(decode(action, value_number())?),
            "setPencilWidth" => NoteCommand::SetPencilWidth(decode(action, value_number())?),
            "setEraserRadius" => NoteCommand::SetEraserRadius(decode(action, value_number())?),
            "addBlock" => {
                let mut entries = fold(args, &[("value", "kind")], &[("kind", string("text")), ("x", zero()), ("y", zero())]);
                map(&mut entries, "x", number);
                map(&mut entries, "y", number);
                NoteCommand::AddBlock(decode(action, only(entries, &["kind", "x", "y"]))?)
            }
            "moveBlock" => NoteCommand::MoveBlock(decode(action, only(fold(args, &[("id", "block_id"), ("target_id", "target_row_id"), ("position", "drop_position")], &[("drop_position", string("inside"))]), &["block_id", "target_row_id", "drop_position"]))?),
            "deleteBlock" => NoteCommand::DeleteBlock(decode(action, only(fold(args, BLOCK, &[]), &["block_id"]))?),
            "deleteSelection" => NoteCommand::DeleteSelection(decode(action, empty())?),
            "duplicateBlock" => NoteCommand::DuplicateBlock(decode(action, only(fold(args, BLOCK, &[]), &["block_id"]))?),
            "duplicateSelection" => NoteCommand::DuplicateSelection(decode(action, empty())?),
            "patchBlocks" => {
                let mut entries = fold(args, &[("ids", "block_ids"), ("block_id", "block_ids"), ("id", "block_ids")], &[("block_ids", DslValue::Array(Vec::new())), ("value", string(""))]);
                map(&mut entries, "block_ids", list);
                map(&mut entries, "value", text);
                NoteCommand::PatchBlocks(decode(action, only(entries, &["block_ids", "field", "value"]))?)
            }
            "setActiveExample" => NoteCommand::SetActiveExample(decode(action, only(fold(args, &[("value", "example_id"), ("id", "example_id")], &[("example_id", string(crate::standards::v1::subsets::any::examples::demo::ID))]), &["example_id"]))?),
            "loadDocumentJson" => {
                let mut entries = fold(args, &[("value", "json"), ("text", "json")], &[]);
                map(&mut entries, "json", text);
                NoteCommand::LoadDocumentJson(decode(action, only(entries, &["json"]))?)
            }
            "inkApplyEvents" => {
                let mut entries = fold(args, &[("gesture", "gesture_json")], &[]);
                if !entries.iter().any(|(key, _)| key == "events_json") {
                    let events = entries.iter().find(|(key, _)| key == "events").map(|(_, value)| value.clone()).unwrap_or(DslValue::Array(Vec::new()));
                    entries.push(("events_json".into(), text(events)));
                }
                map(&mut entries, "gesture_json", text);
                NoteCommand::InkApplyEvents(decode(action, only(entries, &["events_json", "phase", "reason", "gesture_json", "select_ids"]))?)
            }
            "engagementSubmit" => {
                let mut entries = fold(args, &[("text", "value"), ("input", "value")], &[]);
                entries.retain(|(key, value)| key != "value" || !matches!(value, DslValue::Null));
                map(&mut entries, "value", text);
                NoteCommand::EngagementSubmit(decode(action, only(entries, &["value"]))?)
            }
            "nudgeSelection" => {
                let mut entries = fold(args, &[("x", "dx"), ("y", "dy")], &[("dx", zero()), ("dy", zero())]);
                map(&mut entries, "dx", number);
                map(&mut entries, "dy", number);
                NoteCommand::NudgeSelection(decode(action, only(entries, &["dx", "dy"]))?)
            }
            "nudgeSelectionUp" => NoteCommand::NudgeSelectionUp(decode(action, empty())?),
            "nudgeSelectionDown" => NoteCommand::NudgeSelectionDown(decode(action, empty())?),
            "nudgeSelectionLeft" => NoteCommand::NudgeSelectionLeft(decode(action, empty())?),
            "nudgeSelectionRight" => NoteCommand::NudgeSelectionRight(decode(action, empty())?),
            "nudgeSelectionUpFast" => NoteCommand::NudgeSelectionUpFast(decode(action, empty())?),
            "nudgeSelectionDownFast" => NoteCommand::NudgeSelectionDownFast(decode(action, empty())?),
            "nudgeSelectionLeftFast" => NoteCommand::NudgeSelectionLeftFast(decode(action, empty())?),
            "nudgeSelectionRightFast" => NoteCommand::NudgeSelectionRightFast(decode(action, empty())?),
            "setCamera" => {
                let entries = fold(args, &[], &[]);
                let camera = match entries.iter().find(|(key, _)| key == "camera") {
                    Some((_, camera)) => camera.clone(),
                    None => only(entries, &["x", "y", "zoom"]),
                };
                NoteCommand::SetCamera(decode(action, DslValue::Object(vec![("camera".into(), camera)]))?)
            }
            "setCameraZoom" => NoteCommand::SetCameraZoom(decode(action, value_number())?),
            "engagementInput" => {
                let mut entries = fold(args, &[("text", "value"), ("input", "value")], &[("value", string(""))]);
                map(&mut entries, "value", text);
                NoteCommand::EngagementInput(decode(action, only(entries, &["value"]))?)
            }
            "navigatorEngagementInput" => NoteCommand::NavigatorEngagementInput(decode(action, empty())?),
            "saveDownload" => NoteCommand::SaveDownload(decode(action, empty())?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the note editor has no command for action '{action}'"))),
        })
    }
}
