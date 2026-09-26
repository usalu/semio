//! 🖥️ Note editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum
//! and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `☑️options/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, exact view state in `🪟️window`, shared compute in
//! the artifact's `⚙️engine`. This file is a routing table: `handle` → `NoteCommand::dispatch`, `render` →
//! body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::note::commands::ink_apply_events;
use crate::editor::note::commands::{add_block, delete_block, delete_selection, duplicate_block, duplicate_selection, move_block, patch_blocks};
use crate::editor::note::commands::{engagement_input, engagement_submit, navigator_engagement_input};
use crate::editor::note::commands::{load_request, save_download};
use crate::editor::note::commands::{nudge_selection, nudge_selection_down, nudge_selection_down_fast, nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast};
use crate::editor::note::commands::{set_active_example, set_fixture_json};
use crate::editor::note::commands::{set_camera, set_camera_zoom};
use crate::editor::note::commands::{set_eraser_radius, set_pencil_width};
use crate::editor::note::commands::{set_grid_opacity, set_grid_spacing, set_grid_subdivisions, set_grid_visible};
use crate::editor::note::commands::{set_snap_enabled, set_snap_grid_spacing};
use crate::editor::note::modes::edit;
use crate::editor::note::modes::edit::windows::{composite, navigator};
use crate::editor::note::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::note::presence::{NotePresence, NotePresenceMutation};
use crate::editor::note::terminology::note_play_labels;
use crate::op::NoteMutation;
use crate::schema::empty_note_snapshot;
use crate::{NoteBlockNode, NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppDefinition, ArtifactEditor, ArtifactView, ConfigView, Dialect, DomainTopology, DraftView, DslValue, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UtilityCategory, UtilityDefinition, WindowEngagement,
    WindowMeasure,
};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
/// 👁️✏️ C2 §2.1: the hand-written app id is retired — the canonical surface id
/// (`s.note.note@1/*#editor`) is now derived from `NotePlayApp::DIALECT`/`ROLE` via `surface_app_id`.
pub const NOTE_PLAY_CONTROLLER_ID: &str = "s.note.note@1/*#editor";
pub use catalogue_panel::NOTE_PLAY_BODY_CATALOGUE;
pub use composite::{NOTE_PLAY_BODY_COMPOSITE, NOTE_PLAY_WINDOW_COMPOSITE};
pub use document_panel::NOTE_PLAY_BODY_ARTIFACT;
pub use inspection_panel::NOTE_PLAY_BODY_PROPERTIES;
pub use navigator::{NOTE_PLAY_BODY_NAVIGATOR, NOTE_PLAY_WINDOW_NAVIGATOR};
//#endregion 🔖️Constants

//#region 🧬️AppSchema
fn note_app_schema_descriptor() -> framework_schema::AppSchemaDescriptor {
    framework_schema::AppSchemaDescriptor {
        id: "s.note.note",
        config: framework_schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
        presence: framework_schema::FacetLeaves {
            rust: include_str!("👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//#endregion 🧬️AppSchema

//#region 🔖️ResetDocument
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `setActiveExample`/`setFixtureJson` build a
/// `Effect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry. The spr is a
/// fresh, edit-free op-log (`store::empty_document_spr`) — never a live `ArtifactEnvelope` minted just to
/// print it: dropping such an envelope trapped the guest (`artifact envelope terminal shell reached Drop
/// before its app-owned bounded retirement authority detached every nested owner`) on the react shell's
/// boot `setActiveExample` (ticket 26/09/17/NOTE-PLUGIN-END-TO-END).
pub fn reset_document_effect(document: &NoteSnapshot) -> semio_framework::kernel::Effect {
    let pack = <NoteSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("note", NOTE_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Utilities
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn note_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: NOTE_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/keybound vocabulary
/// dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn note_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()`/
/// utility bar).
fn note_utility(id: &str, label: LocalizedLabel, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧰️ The utility armed for the window being rendered or dispatched: the React host arms utilities per
/// window instance (`active_utility_by_window_id`) and mirrors only the shell's ACTIVE window into the flat
/// `active_utility_id`, so a pane that is not the active window read `None` and ran every pencil drag as a
/// `selectDirect` pick (ticket 26/09/17/NOTE-PLUGIN-END-TO-END; draw's `drawing_active_utility` precedent).
pub fn note_active_utility(view: &semio_framework_plugin::ViewModel) -> &str {
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or("selectDirect")
}
//#endregion 🔖️Utilities

//#region 🔖️Interaction
/// 🕹️ "blocks" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own Group nesting, transitive
/// (selecting/hovering a group covers its descendants).
pub const NOTE_INTERACTION_BLOCKS: &str = "blocks";

/// 🕹️ The only granularity the "blocks" domain declares — stamped on every pick row of the document
/// tree so the host can synthesize `interactionSelect` without a per-row argument map.
pub const NOTE_INTERACTION_GRANULARITY: &str = "block";

/// 🕹️ Per-dispatch scratch: the "blocks" domain's current selection, resolved once by
/// `ArtifactEditor::handle` from `InteractionView` and threaded to every leaf command handler —
/// `app_commands!`'s generated `dispatch` has no way to thread `InteractionView` itself (mirrors
/// `📐️cad`'s `CadDispatchCtx`). Row-id-prefixed `InteractionTarget` ids (the SAME canonical ids the
/// document panel tree renders, see its own doc comment) are converted back to raw block ids here,
/// once, so every downstream handler keeps working with the raw ids it always did.
pub struct NoteDispatchCtx {
    pub selected_block_ids: Vec<String>,
    pub id_owner: crate::schema::NoteIdOwner,
    pub view_state: Option<semio_framework_plugin::ViewModel>,
    pub window_transient: crate::editor::note::window::NoteCompositeWindowTransient,
    pub window_transient_owner: Option<semio_framework_plugin::WindowTransientSnapshot>,
}

/// 🌳️ `blocks` domain topology from the document's own Group nesting — row-id-prefixed ids (matching
/// the document panel tree's own item ids), so `validate_state` prunes deleted blocks and
/// range/transitive selection walk the real tree structure.
fn note_blocks_topology(document: &NoteSnapshot) -> DomainTopology {
    fn visit(blocks: &[NoteBlockNode], parent: Option<&str>, out: &mut Vec<TopologyNode>) {
        for block in blocks {
            let id = crate::schema::block_tree_row_id(block);
            out.push(TopologyNode { id: id.clone(), granularity: "block".into(), parent: parent.map(str::to_string) });
            if let NoteBlockNode::Group { children, .. } = block {
                visit(children, Some(id.as_str()), out);
            }
        }
    }
    let mut ordered = Vec::new();
    visit(&document.blocks, None, &mut ordered);
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

//#region 🔖️Commands
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
        "setFixtureJson" as "set-fixture-json" => set_fixture_json::SetFixtureJson,
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
        "loadRequest" as "load-request" => load_request::LoadRequest,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ActionBridge
/// 🎯️ Folds the host's `{action, args}` vocabulary (camelCase keys, JSON floats, control `value`s) into
/// the snake_case `FromValue` payloads of `🎮️commands/*`, one arm per `NoteCommand` row.
mod args_bridge {
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
            DslValue::Number(dsl::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { DslValue::Number(dsl::Number::UInt(float as u64)) } else { DslValue::Number(dsl::Number::Int(float as i64)) }
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
            other => DslValue::String(dsl::json::to_json_string(&other)),
        }
    }

    /// 🔢️ Slider/number controls may deliver their value as text.
    fn number(value: DslValue) -> DslValue {
        match &value {
            DslValue::String(raw) => raw.trim().parse::<f64>().map(|parsed| DslValue::Number(dsl::Number::Float(parsed))).unwrap_or(value),
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

    fn decode<T: dsl::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("note action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<NoteCommand, Fault> {
        const BLOCK: &[(&str, &str)] = &[("id", "block_id"), ("value", "block_id")];
        let string = |value: &str| DslValue::String(value.into());
        let zero = || DslValue::Number(dsl::Number::Float(0.0));
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
            "setFixtureJson" => {
                let mut entries = fold(args, &[("value", "json"), ("text", "json")], &[]);
                map(&mut entries, "json", text);
                NoteCommand::SetFixtureJson(decode(action, only(entries, &["json"]))?)
            }
            "inkApplyEvents" => {
                let mut entries = fold(args, &[], &[("phase", string("atomic"))]);
                if !entries.iter().any(|(key, _)| key == "events_json") {
                    let events = entries.iter().find(|(key, _)| key == "events").map(|(_, value)| value.clone()).unwrap_or(DslValue::Array(Vec::new()));
                    entries.push(("events_json".into(), text(events)));
                }
                NoteCommand::InkApplyEvents(decode(action, only(entries, &["events_json", "phase", "select_ids"]))?)
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
            "loadRequest" => NoteCommand::LoadRequest(decode(action, empty())?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the note editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🔖️NotePlayApp
/// 🧪️ B1: unit struct — document preferences live in `NoteSnapshot`; composite camera and
/// engagement input live in their exact WindowConfig and WindowTransient owners.
#[derive(Default)]
pub struct NotePlayApp;

impl ArtifactEditor for NotePlayApp {
    type Snapshot = NoteSnapshot;
    type Mutation = NoteMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NotePresence;
    type PresenceMutation = NotePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = NoteCommand;

    /// 👁️✏️ C2 §2.1: `APP_ID` is removed from the authoring traits — the canonical surface id is
    /// derived from `DIALECT` + the trait's own fixed `ROLE` (`AppRole::Editor`, not overridable in
    /// practice — mirrors trinity/jack's `TrinityJackPlayApp`, the first real W2 packet to confirm
    /// `ArtifactEditor::ROLE` is NOT restated per-impl) via `surface_app_id`.
    const DIALECT: Dialect = crate::NOTE_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::note_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
        crate::note_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = NOTE_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<NotePlayApp>,
        owner_file: "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.note.note@1/*#editor",
        artifact_schema: "note.document",
        factory: "NoteCommandJobFactory",
        factory_type: crate::editor::note::retained::NoteCommandJobFactory,
        tools: {
            "setGridVisible" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setGridSpacing" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setGridSubdivisions" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setGridOpacity" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setSnapEnabled" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setSnapGridSpacing" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setPencilWidth" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setEraserRadius" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "addBlock" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "moveBlock" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "deleteBlock" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "deleteSelection" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "duplicateBlock" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "duplicateSelection" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "patchBlocks" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setActiveExample" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setFixtureJson" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "inkApplyEvents" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "engagementSubmit" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelection" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionUp" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionDown" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionLeft" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionRight" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionUpFast" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionDownFast" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionLeftFast" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "nudgeSelectionRightFast" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setCamera" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setCameraZoom" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "engagementInput" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "navigatorEngagementInput" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "saveDownload" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "loadRequest" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
        }
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        crate::editor::note::retained::register(registry)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        crate::editor::note::retained::build(request)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        crate::editor::note::window::register_config(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        crate::editor::note::window::register_transient(registry)
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(crate::editor::note::retained::artifact_preparation_factory())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    /// 🏗️ Admits the whole-document replacement every `Effect::LoadDocument` this editor emits
    /// (`reset_document_effect`: example switch, fixture import) — the trait default refuses the
    /// envelope, which faults every note document swap at the archive-load boundary.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, NOTE_DOCUMENT_SCHEMA, operation, generation))
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::note::presence::NotePresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::note::presence::NotePresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(crate::editor::note::presence::note_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn app_schema() -> Option<framework_schema::AppSchemaDescriptor> {
        Some(note_app_schema_descriptor())
    }

    fn initial_snapshot() -> NoteSnapshot {
        empty_note_snapshot()
    }

    /// 🏷️ Maps each `NoteCommand` variant back to the action id it was declared under in
    /// `create_note_app` — used by `VcsArtifactApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(command: &NoteCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Host-action bridge into the closed `NoteCommand` enum (ticket 26/09/17/NOTE-PLUGIN-END-TO-END):
    /// the trait default refuses every app action, which left every panel/canvas verb dead in the shell.
    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &NoteCommand,
        doc: &ArtifactView<'_, NoteSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<NoteMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let selected_block_ids = interaction.selection(NOTE_INTERACTION_BLOCKS).ids.iter().filter_map(|id| crate::schema::block_id_from_tree_row_id(id)).collect();
        let mut ctx = NoteDispatchCtx {
            selected_block_ids,
            id_owner: crate::schema::NoteIdOwner::for_document_child(doc.snapshot, command.command_id()),
            view_state: view_state.cloned(),
            window_transient: Default::default(),
            window_transient_owner: None,
        };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🕹️ `blocks` domain: `HierarchyProvider::Topology` from the document's own Group nesting — see
    /// `note_blocks_topology`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(NOTE_INTERACTION_BLOCKS.to_string(), note_blocks_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let window = crate::editor::note::window::config_from_view(cfg);
        let labels = note_play_labels(view_state);
        let active_utility = note_active_utility(view_state);
        match body_key {
            NOTE_PLAY_BODY_COMPOSITE => composite::render(document, &window.camera, active_utility),
            NOTE_PLAY_BODY_NAVIGATOR => navigator::render(document, &crate::NoteCamera::default(), active_utility),
            NOTE_PLAY_BODY_ARTIFACT => document_panel::render(document, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, NOTE_PLAY_BODY_ARTIFACT)),
            NOTE_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            NOTE_PLAY_BODY_PROPERTIES => inspection_panel::render(document, active_utility, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "note diagnostic text admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, NoteSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        if body_key != NOTE_PLAY_BODY_COMPOSITE {
            return Self::render(body_key, doc, cfg, view_state);
        }
        let window = crate::editor::note::window::config_from_view(cfg);
        composite::render_with_interaction(doc.snapshot, &window.camera, note_active_utility(view_state), interaction).map(semio_framework_plugin::built_to_component_tree)
    }

    fn window_engagements(doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let kind = view_state.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
        let config = crate::editor::note::window::config_from_view(cfg);
        let active_utility = note_active_utility(view_state);
        match kind {
            Some(NOTE_PLAY_WINDOW_COMPOSITE) => HashMap::from([(window_id.to_string(), composite::engagement(doc.snapshot, &config.camera, ""))]),
            Some(NOTE_PLAY_WINDOW_NAVIGATOR) => HashMap::from([(window_id.to_string(), navigator::engagement(active_utility))]),
            _ => HashMap::new(),
        }
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, NoteSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let kind = view_state.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
        let config = crate::editor::note::window::config_from_view(cfg);
        let transient = crate::editor::note::window::transient_from_view(transient);
        let active_utility = note_active_utility(view_state);
        match kind {
            Some(NOTE_PLAY_WINDOW_COMPOSITE) => HashMap::from([(window_id.to_string(), composite::engagement(doc.snapshot, &config.camera, &transient.engagement_input))]),
            Some(NOTE_PLAY_WINDOW_NAVIGATOR) => HashMap::from([(window_id.to_string(), navigator::engagement(active_utility))]),
            _ => HashMap::new(),
        }
    }

    fn window_measures(doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let kind = view_state.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
        let config = crate::editor::note::window::config_from_view(cfg);
        let labels = note_play_labels(view_state);
        match kind {
            Some(NOTE_PLAY_WINDOW_COMPOSITE) => HashMap::from([(window_id.to_string(), composite::window_measures(doc.snapshot, &config.camera, labels))]),
            Some(NOTE_PLAY_WINDOW_NAVIGATOR) => HashMap::from([(window_id.to_string(), navigator::window_measures(doc.snapshot, &crate::NoteCamera::default(), labels))]),
            _ => HashMap::new(),
        }
    }
}
//#endregion 🔖️NotePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_note_app() -> AppDefinition {
    let document = empty_note_snapshot();
    let mut app = Editor::builder(crate::NOTE_DIALECT)
            .document(["semio", "note"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("note")
            .mode_def(edit::definition())
            .default_mode_id(edit::NOTE_PLAY_MODE_EDIT)
            .window_kind_def(composite::definition())
            .window_kind_def(navigator::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // 📇️ Palette-visible selection-consuming commands (P0) — the framework auto-injects
            // selectAll/clearSelection (and interactionSelect/interactionHover/setSelectionMode/
            // setInteractionGranularity) for every app that declares an `.interaction(...)` domain
            // (see below); these two retained verbs still operate ON that framework-owned selection.
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .mutation("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"))
            // ➕️ Palette-visible block insertion (P1) with a staged argument form.
            .mutation("addBlock", LocalizedLabel::native("Add Block", "Block hinzufügen"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            // 🐚️ Import/export footer actions → panel Shell actions emitting host effects (S).
            .shell_action("loadRequest", LocalizedLabel::native("Import", "Importieren"))
            .shell_action("saveDownload", LocalizedLabel::native("Export", "Exportieren"))
            // 🔧️ Internal content operations — inspector/tree/drag/import-bound, not palette commands.
            // B1: the old `"setGridVisible" | "toggleGrid"`/`"setSnapEnabled" | "toggleSnap"`/
            // `"addBlock" | "dropBlockKind"` action-id aliases collapsed onto one `NoteCommand` variant
            // each (see `NoteCommand`'s doc comment) — `toggleGrid`/`toggleSnap`/`dropBlockKind` were
            // never independently wired to any UI element or host caller, so their dead alias
            // declarations are dropped here rather than kept as unreachable synonyms.
            .action_with(note_internal_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Rastersichtbarkeit festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("setGridSubdivisions", LocalizedLabel::native("Set Grid Subdivisions", "Rasterunterteilungen festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("setGridOpacity", LocalizedLabel::native("Set Grid Opacity", "Rasterdeckkraft festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("setSnapEnabled", LocalizedLabel::native("Set Snap Enabled", "Einrasten aktivieren"), ActionKind::Mutation))
            .action_with(note_internal_action("setSnapGridSpacing", LocalizedLabel::native("Set Snap Grid Spacing", "Rasterabstand für Einrasten festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("setPencilWidth", LocalizedLabel::native("Set Pencil Width", "Stiftbreite festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("setEraserRadius", LocalizedLabel::native("Set Eraser Radius", "Radiergummi-Radius festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("moveBlock", LocalizedLabel::native("Move Block", "Block verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("deleteBlock", LocalizedLabel::native("Delete Block", "Block löschen"), ActionKind::Mutation))
            .action_with(note_internal_action("duplicateBlock", LocalizedLabel::native("Duplicate Block", "Block duplizieren"), ActionKind::Mutation))
            .action_with(note_internal_action("patchBlocks", LocalizedLabel::native("Patch Blocks", "Blöcke aktualisieren"), ActionKind::Mutation))
            .action_with(note_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_with(note_internal_action("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation))
            .action_with(note_internal_action("inkApplyEvents", LocalizedLabel::native("Apply Note Events", "Notiz-Ereignisse anwenden"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelection", LocalizedLabel::native("Nudge Selection", "Auswahl verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionUp", LocalizedLabel::native("Nudge Selection Up", "Auswahl nach oben verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionDown", LocalizedLabel::native("Nudge Selection Down", "Auswahl nach unten verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionLeft", LocalizedLabel::native("Nudge Selection Left", "Auswahl nach links verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionRight", LocalizedLabel::native("Nudge Selection Right", "Auswahl nach rechts verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionUpFast", LocalizedLabel::native("Nudge Selection Up Fast", "Auswahl schnell nach oben verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionDownFast", LocalizedLabel::native("Nudge Selection Down Fast", "Auswahl schnell nach unten verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionLeftFast", LocalizedLabel::native("Nudge Selection Left Fast", "Auswahl schnell nach links verschieben"), ActionKind::Mutation))
            .action_with(note_internal_action("nudgeSelectionRightFast", LocalizedLabel::native("Nudge Selection Right Fast", "Auswahl schnell nach rechts verschieben"), ActionKind::Mutation))
            // 👁️ Ephemeral view state — engagement/camera scratch, never a document operation. Selection/
            // hover are no longer declared here: framework-owned, injected via `.interaction(...)` below.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_with(note_internal_action("navigatorEngagementInput", LocalizedLabel::native("Navigator Engagement Input", "Navigator-Eingabe"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(note_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            // 📝️ Staged argument forms for the palette-eligible actions.
            .action_args("addBlock", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Typ"), vec![
                    ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
                    ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
                    ActionArgOption::new("table", LocalizedLabel::native("Table", "Tabelle")),
                    ActionArgOption::new("math", LocalizedLabel::native("Math", "Mathe")),
                    ActionArgOption::new("stroke", LocalizedLabel::native("Ink", "Tinte")),
                    ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
                ]).required().default_value(&"text"),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).default_value(&0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).default_value(&0.0),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(crate::standards::v1::subsets::any::examples::demo::ID, crate::standards::v1::subsets::any::examples::demo::label()),
                ]).required().default_value(&crate::standards::v1::subsets::any::examples::demo::ID),
            ])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()])
            .action_args("setGridSpacing", vec![ActionArgDef::number("value", LocalizedLabel::native("Grid spacing", "Rasterabstand")).required()])
            .action_args("setGridSubdivisions", vec![ActionArgDef::slider("value", LocalizedLabel::native("Subdivisions", "Unterteilungen"), 1.0, 16.0).required()])
            .action_args("setGridOpacity", vec![ActionArgDef::slider("value", LocalizedLabel::native("Grid opacity", "Rasterdeckkraft"), 0.05, 1.0).required()])
            .action_args("setSnapGridSpacing", vec![ActionArgDef::number("value", LocalizedLabel::native("Snap spacing", "Einrastabstand")).required()])
            .action_args("setPencilWidth", vec![ActionArgDef::number("value", LocalizedLabel::native("Pencil width", "Stiftbreite")).required()])
            .action_args("setEraserRadius", vec![ActionArgDef::number("value", LocalizedLabel::native("Eraser radius", "Radiergummi-Radius")).required()])
            .action_args("setCameraZoom", vec![ActionArgDef::number("value", LocalizedLabel::native("Zoom", "Zoom")).required()])
            .action_args("moveBlock", vec![
                ActionArgDef::text("blockId", LocalizedLabel::native("Block", "Block")).required(),
                ActionArgDef::text("targetRowId", LocalizedLabel::native("Target row", "Zielzeile")).required(),
                ActionArgDef::select("dropPosition", LocalizedLabel::native("Position", "Position"), vec![
                    ActionArgOption::new("before", LocalizedLabel::native("Before", "Davor")),
                    ActionArgOption::new("inside", LocalizedLabel::native("Inside", "Hinein")),
                    ActionArgOption::new("after", LocalizedLabel::native("After", "Danach")),
                ]).default_value(&"inside"),
            ])
            .action_args("deleteBlock", vec![ActionArgDef::text("blockId", LocalizedLabel::native("Block", "Block")).required()])
            .action_args("duplicateBlock", vec![ActionArgDef::text("blockId", LocalizedLabel::native("Block", "Block")).required()])
            .action_args("patchBlocks", vec![
                ActionArgDef::text_list("blockIds", LocalizedLabel::native("Blocks", "Blöcke")).required(),
                ActionArgDef::select("field", LocalizedLabel::native("Field", "Feld"), vec![
                    ActionArgOption::new("name", LocalizedLabel::native("Name", "Name")),
                    ActionArgOption::new("visible", LocalizedLabel::native("Visible", "Sichtbar")),
                    ActionArgOption::new("locked", LocalizedLabel::native("Locked", "Gesperrt")),
                    ActionArgOption::new("x", LocalizedLabel::native("X", "X")),
                    ActionArgOption::new("y", LocalizedLabel::native("Y", "Y")),
                    ActionArgOption::new("width", LocalizedLabel::native("Width", "Breite")),
                    ActionArgOption::new("height", LocalizedLabel::native("Height", "Höhe")),
                    ActionArgOption::new("textContent", LocalizedLabel::native("Text", "Text")),
                    ActionArgOption::new("textSize", LocalizedLabel::native("Text size", "Textgröße")),
                    ActionArgOption::new("mathTex", LocalizedLabel::native("Math (TeX)", "Mathematik (TeX)")),
                    ActionArgOption::new("inkWidth", LocalizedLabel::native("Ink width", "Tintenbreite")),
                    ActionArgOption::new("tableAddRow", LocalizedLabel::native("Add table row", "Tabellenzeile hinzufügen")),
                    ActionArgOption::new("tableRemoveRow", LocalizedLabel::native("Remove table row", "Tabellenzeile entfernen")),
                    ActionArgOption::new("tableAddColumn", LocalizedLabel::native("Add table column", "Tabellenspalte hinzufügen")),
                    ActionArgOption::new("tableRemoveColumn", LocalizedLabel::native("Remove table column", "Tabellenspalte entfernen")),
                ]).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")),
            ])
            // 💬️ Agent-facing descriptions (ticket 26/09/18 slice M5a): what `capabilities_search`
            // matches on and `capabilities_describe` returns. EN first, DE second, no default language.
            .action_describe("addBlock", LocalizedLabel::native("Adds a new block to the note at the given position — text, image, table, math, ink stroke or group.", "Fügt der Notiz an der angegebenen Position einen neuen Block hinzu — Text, Bild, Tabelle, Mathematik, Tinte oder Gruppe."))
            .action_use_when("addBlock", vec!["add a text block".into(), "insert a table".into(), "add an image to the note".into(), "write a note".into()])
            .action_describe("deleteSelection", LocalizedLabel::native("Removes every currently selected block from the note.", "Entfernt alle aktuell ausgewählten Blöcke aus der Notiz."))
            .action_use_when("deleteSelection", vec!["delete the selected blocks".into(), "remove this".into()])
            .action_describe("duplicateSelection", LocalizedLabel::native("Copies every currently selected block and inserts the copies beside the originals.", "Kopiert alle ausgewählten Blöcke und fügt die Kopien neben den Originalen ein."))
            .action_use_when("duplicateSelection", vec!["duplicate the selection".into()])
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole note with one of the plugin's declared playground examples.", "Ersetzt die gesamte Notiz durch eines der deklarierten Beispiele des Plugins."))
            .action_describe("loadRequest", LocalizedLabel::native("Asks the host to open a file and import it into this note.", "Fordert den Host auf, eine Datei zu öffnen und in diese Notiz zu importieren."))
            .action_use_when("loadRequest", vec!["import a document".into(), "open a file into the note".into()])
            .action_describe("saveDownload", LocalizedLabel::native("Hands the note to the host as a downloadable file.", "Übergibt die Notiz dem Host als herunterladbare Datei."))
            .action_use_when("saveDownload", vec!["export the note".into(), "download this note".into()])
            .action_describe("moveBlock", LocalizedLabel::native("Moves one block to a new position on the page.", "Verschiebt einen Block an eine neue Position auf der Seite."))
            .action_describe("deleteBlock", LocalizedLabel::native("Removes one block from the note by id.", "Entfernt einen Block anhand seiner Id aus der Notiz."))
            .action_describe("duplicateBlock", LocalizedLabel::native("Copies one block and inserts the copy beside the original.", "Kopiert einen Block und fügt die Kopie neben dem Original ein."))
            .action_describe("patchBlocks", LocalizedLabel::native("Sets one named property on several blocks at once.", "Setzt eine benannte Eigenschaft auf mehreren Blöcken gleichzeitig."))
            .action_describe("nudgeSelection", LocalizedLabel::native("Shifts the selected blocks by a given offset.", "Verschiebt die ausgewählten Blöcke um einen angegebenen Versatz."))
            .action_use_when("nudgeSelection", vec!["move the selection a little".into(), "shift these blocks".into()])
            .action_describe("setFixtureJson", LocalizedLabel::native("Loads a whole note document from JSON text.", "Lädt ein vollständiges Notizdokument aus JSON-Text."))
            .action_describe("setGridVisible", LocalizedLabel::native("Shows or hides the page grid.", "Blendet das Seitenraster ein oder aus."))
            .action_describe("setGridSpacing", LocalizedLabel::native("Sets the distance between page grid lines.", "Legt den Abstand zwischen den Rasterlinien fest."))
            .action_describe("setGridSubdivisions", LocalizedLabel::native("Sets how many minor lines the page grid draws between major ones.", "Legt fest, wie viele Nebenlinien das Raster zwischen Hauptlinien zeichnet."))
            .action_describe("setGridOpacity", LocalizedLabel::native("Sets how strongly the page grid is drawn, from 0 to 1.", "Legt fest, wie stark das Seitenraster gezeichnet wird, von 0 bis 1."))
            .action_describe("setSnapEnabled", LocalizedLabel::native("Turns snapping of moved blocks to the grid on or off.", "Schaltet das Einrasten verschobener Blöcke am Raster ein oder aus."))
            .action_describe("setSnapGridSpacing", LocalizedLabel::native("Sets the spacing blocks snap to when snapping is on.", "Legt den Rasterabstand fest, an dem Blöcke bei aktiviertem Einrasten einrasten."))
            .action_describe("setPencilWidth", LocalizedLabel::native("Sets the stroke width the ink pencil draws with.", "Legt die Strichbreite des Tintenstifts fest."))
            .action_describe("setEraserRadius", LocalizedLabel::native("Sets the radius of the ink eraser.", "Legt den Radius des Tintenradierers fest."))
            // ⚠️ Discards content no later verb reconstructs — the gateway asks a human first.
            .action_destructive("deleteSelection")
            .action_destructive("deleteBlock")
            .action_destructive("setActiveExample")
            .action_destructive("setFixtureJson")
            .action_destructive("saveDownload")
            // 🖱️ Raw input plumbing — the surface and the keyboard feed these, agents never do.
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("navigatorEngagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("inkApplyEvents", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionUp", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionLeft", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionRight", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionUpFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionDownFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionLeftFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nudgeSelectionRightFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("setGridVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSubdivisions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridOpacity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSnapEnabled", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSnapGridSpacing", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setPencilWidth", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setEraserRadius", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addBlock", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("moveBlock", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteBlock", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateBlock", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("patchBlocks", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setFixtureJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("inkApplyEvents", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionUp", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionLeft", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionRight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionUpFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionDownFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionLeftFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nudgeSelectionRightFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setCameraZoom", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("navigatorEngagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("saveDownload", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("loadRequest", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🧰️ Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(note_utility("selectDirect", LocalizedLabel::native("Direct", "Direkt"), "text-cursor", "Select", UtilityCategory::Selection))
            .utility(note_utility("selectMarquee", LocalizedLabel::native("Marquee", "Rahmenauswahl"), "selection", "Select", UtilityCategory::Selection))
            .utility(note_utility("text", LocalizedLabel::native("Text", "Text"), "type", "Block", UtilityCategory::Utilities))
            .utility(note_utility("image", LocalizedLabel::native("Image", "Bild"), "image", "Block", UtilityCategory::Utilities))
            .utility(note_utility("table", LocalizedLabel::native("Table", "Tabelle"), "table-2", "Block", UtilityCategory::Utilities))
            .utility(note_utility("math", LocalizedLabel::native("Math", "Mathe"), "sigma", "Block", UtilityCategory::Utilities))
            .utility(note_utility("pencil", LocalizedLabel::native("Pencil", "Stift"), "pencil", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("eraserStroke", LocalizedLabel::native("Stroke Eraser", "Strich-Radiergummi"), "eraser", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("eraserPoint", LocalizedLabel::native("Point Eraser", "Punkt-Radiergummi"), "eraser", "Draw", UtilityCategory::Utilities))
            .utility(note_utility("pan", LocalizedLabel::native("Pan", "Schwenken"), "hand", "View", UtilityCategory::Utilities))
            .window_kind_utilities(NOTE_PLAY_WINDOW_COMPOSITE, vec![
                "selectDirect".into(), "selectMarquee".into(),
                "text".into(), "image".into(), "table".into(), "math".into(),
                "pencil".into(), "eraserStroke".into(), "eraserPoint".into(), "pan".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+y", "redo")
            // 🕹️ `mod+a`/`escape` are no longer declared here — the framework auto-injects `selectAll`/
            // `clearSelection` (with these SAME keys) for every app with at least one `.interaction(...)`
            // domain, see `interaction_action_definitions`.
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("up", "nudgeSelectionUp")
            .keybinding("down", "nudgeSelectionDown")
            .keybinding("left", "nudgeSelectionLeft")
            .keybinding("right", "nudgeSelectionRight")
            .keybinding("shift+up", "nudgeSelectionUpFast")
            .keybinding("shift+down", "nudgeSelectionDownFast")
            .keybinding("shift+left", "nudgeSelectionLeftFast")
            .keybinding("shift+right", "nudgeSelectionRightFast")
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "blocks" interaction
            // domain — one granularity ("block"), `HierarchyProvider::Topology` from the document's own
            // Group nesting (`note_blocks_topology`/`NotePlayApp::interaction_topology`), both hover and
            // selection transitive (selecting/hovering a group covers its descendants). Multi-select via
            // both Pick (tree rows) and Rectangle (the `selectMarquee` canvas utility), all five merges.
            .interaction(InteractionDefinition {
                id: NOTE_INTERACTION_BLOCKS.into(),
                label: LocalizedLabel::native("Blocks", "Blöcke"),
                granularities: vec![GranularityDefinition { id: NOTE_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Block", "Block"), icon_id: "square".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: true,
                    broadcast: true,
                },
            })
            .window_kind_interactions(NOTE_PLAY_WINDOW_COMPOSITE, vec![InteractionRef::new(NOTE_INTERACTION_BLOCKS)])
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE) —
            // note has no user-visible sticky config defaults (unlike shooting's default shot/asset
            // format), so `config_spec()` stays the trait default (`ConfigSpec::empty()`); registering it
            // here still declares the config schema for the manifest.
            .config(NotePlayApp::config_spec())
            .build_definition();
    for window in app.window_kinds.iter_mut() {
        if window.id == NOTE_PLAY_WINDOW_COMPOSITE {
            window.options.measures = composite::window_measures(&document, &crate::NoteCamera::default(), &crate::editor::note::terminology::NotePlayLabels::NATIVE_EN);
        } else if window.id == NOTE_PLAY_WINDOW_NAVIGATOR {
            window.options.measures = navigator::window_measures(&document, &crate::NoteCamera::default(), &crate::editor::note::terminology::NotePlayLabels::NATIVE_EN);
        }
    }
    // 👁️✏️ SDK gap (contract §2.4/§7.4): `.example(...)`/`.workflow(...)` do not exist on
    // `EditorBuilder` — `PluginBuilder::editor::<E>` only takes the bare `AppDefinition`, so the old
    // `"semio"` example registration and the `"note"` workflow tag are dropped here, not silently
    // ported. The subset's own `📚️examples/🎬️demo` facet is the likely intended replacement
    // mechanism (see this surface's own `📚️examples/🎬️demo-session`), per the pilot's report.
    app
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests


/// 🏷️ Admits Note display text into a bounded semantic label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    value.as_ref().try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "note label admission failed"))
}

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
