//! 🖥️ Note play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `🎚️options/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the
//! artifact's `⚙️engine`. This file is a routing table: `handle` → `NoteCommand::dispatch`, `render` →
//! body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::note::commands::block::{add_block, delete_block, delete_selection, duplicate_block, duplicate_selection, move_block, patch_blocks};
use crate::apps::note::commands::camera::{set_camera, set_camera_zoom};
use crate::apps::note::commands::drawing::{set_eraser_radius, set_pencil_width};
use crate::apps::note::commands::engagement::{engagement_input, engagement_submit, navigator_engagement_input};
use crate::apps::note::commands::export::{load_request, save_download};
use crate::apps::note::commands::fixture::{set_active_example, set_fixture_json};
use crate::apps::note::commands::grid::{set_grid_opacity, set_grid_spacing, set_grid_subdivisions, set_grid_visible};
use crate::apps::note::commands::ink::ink_apply_events;
use crate::apps::note::commands::locale::set_locale;
use crate::apps::note::commands::nudge::{nudge_selection, nudge_selection_down, nudge_selection_down_fast, nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast};
use crate::apps::note::commands::selection::{clear_selection, select_all, set_hover, set_selection};
use crate::apps::note::commands::snap::{set_snap_enabled, set_snap_grid_spacing};
use crate::apps::note::commands::utility::set_active_utility;
use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::apps::note::presence::{NotePresence, NotePresenceMutation};
use crate::apps::note::modes::edit;
use crate::apps::note::modes::edit::windows::{composite, navigator};
use crate::apps::note::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::note::terminology::note_play_labels;
use crate::artifacts::note::engine::empty_note_snapshot;
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, UiNode, UtilityCategory, UtilityDefinition, WindowEngagement, WindowMeasure, SET_ACTIVE_UTILITY_ACTION_ID};
use store::EngineHandles;
use std::collections::HashMap;

//#region 🔖️Constants
pub const NOTE_PLAY_APP_ID: &str = "note-play";
pub const NOTE_PLAY_CONTROLLER_ID: &str = "note-play";
pub use composite::{NOTE_PLAY_BODY_COMPOSITE, NOTE_PLAY_WINDOW_COMPOSITE};
pub use navigator::{NOTE_PLAY_BODY_NAVIGATOR, NOTE_PLAY_WINDOW_NAVIGATOR};
pub use catalogue_panel::NOTE_PLAY_BODY_CATALOGUE;
pub use document_panel::NOTE_PLAY_BODY_DOCUMENT;
pub use inspection_panel::NOTE_PLAY_BODY_PROPERTIES;
//#endregion 🔖️Constants

//#region 🔖️ResetDocument
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `setActiveExample`/`setFixtureJson` build a
/// `HostEffect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry.
pub fn reset_document_effect(document: &NoteSnapshot) -> semio_framework::kernel::HostEffect {
    let pack = <NoteSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<NoteSnapshot, NoteMutation>(NOTE_DOCUMENT_SCHEMA, "note", document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("note document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::HostEffect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Utilities
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn note_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: NOTE_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/keybound vocabulary
/// dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn note_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()`/
/// utility bar).
fn note_utility(id: &str, label: LocalizedLabel, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}
//#endregion 🔖️Utilities

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `NotePlayApp::Command` — the SOLE dispatch surface for note's own behavior (B1 pure-trait
    /// migration, mirroring `shooting_protocol::ShootingCommand`). One variant per action id the pre-B1
    /// `NotePlayApp::handle_action` matched; combined `"x" | "y"` arms (e.g. the old
    /// `"setGridVisible" | "toggleGrid"` alias, never independently wired anywhere in the note ui crate
    /// or its hosts) collapse onto the one surviving action id's command instead of keeping a dead
    /// synonym. Row order is the binary variant ordinal: appending is safe, reordering is a wire-format
    /// break.
    pub enum NoteCommand for NoteSnapshot, NoteMutation, NoteConfig, NoteConfigMutation {
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
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        "selectAll" as "select-all" => select_all::SelectAll,
        "clearSelection" as "clear-selection" => clear_selection::ClearSelection,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setHover" as "set-hover" => set_hover::SetHover,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "navigatorEngagementInput" as "navigator-engagement-input" => navigator_engagement_input::NavigatorEngagementInput,
        "saveDownload" as "save-download" => save_download::SaveDownload,
        "loadRequest" as "load-request" => load_request::LoadRequest,
    }
}
//#endregion 🔖️Commands

//#region 🔖️NotePlayApp
/// 🧪️ B1: unit struct — every former `NotePlayRuntime`/`ViewModel`-read field now lives in
/// `NoteConfig` (see `ArtifactApp::Config`), written through `NoteConfigMutation`s.
#[derive(Default)]
pub struct NotePlayApp;

impl ArtifactApp for NotePlayApp {
    type Snapshot = NoteSnapshot;
    type Mutation = NoteMutation;
    type Config = NoteConfig;
    type ConfigMutation = NoteConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NotePresence;
    type PresenceMutation = NotePresenceMutation;

    type Command = NoteCommand;

    const APP_ID: &'static str = NOTE_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = NOTE_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> NoteSnapshot {
        empty_note_snapshot()
    }

    /// 🏷️ Maps each `NoteCommand` variant back to the action id it was declared under in
    /// `create_note_app` — used by `VcsArtifactApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(command: &NoteCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &NoteCommand, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<NoteMutation, NoteConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = note_play_labels(config);
        match body_key {
            NOTE_PLAY_BODY_COMPOSITE => composite::render(document, config),
            NOTE_PLAY_BODY_NAVIGATOR => navigator::render(document, config),
            NOTE_PLAY_BODY_DOCUMENT => document_panel::render(document, &config.selected_block_ids, labels),
            NOTE_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            NOTE_PLAY_BODY_PROPERTIES => inspection_panel::render(document, &config.selected_block_ids, &config.active_utility_id, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        HashMap::from([
            (NOTE_PLAY_WINDOW_COMPOSITE.to_string(), composite::engagement(doc.snapshot, &config.camera, &config.selected_block_ids, &config.engagement_input)),
            (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), navigator::engagement(&config.active_utility_id)),
        ])
    }

    fn window_measures(doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = note_play_labels(config);
        HashMap::from([(NOTE_PLAY_WINDOW_COMPOSITE.to_string(), composite::window_measures(doc.snapshot, &config.camera, labels)), (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), navigator::window_measures(doc.snapshot, &config.camera, labels))])
    }
}
//#endregion 🔖️NotePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_note_app() -> App {
    let document = empty_note_snapshot();
    let mut app = App::from_builder(
        App::builder(NOTE_PLAY_APP_ID, LocalizedLabel::native("Note", "Notiz"))
            .document(["semio", "note"])
            .artifact_kind(crate::artifacts::note::artifact_kind())
            .icon_id("note")
            .mode_def(edit::definition())
            .default_mode_id(edit::NOTE_PLAY_MODE_EDIT)
            .window_kind_def(composite::definition())
            .window_kind_def(navigator::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // 📇️ Palette-visible selection commands (P0) — ephemeral selection is View, block edits are Operations.
            .view_action("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .mutation("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"))
            // ➕️ Palette-visible block insertion (P1) with a staged argument form.
            .mutation("addBlock", LocalizedLabel::native("Add Block", "Block hinzufügen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
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
            // 👁️ Ephemeral view state — selection/hover/engagement/camera scratch, never a document operation.
            .action_with(note_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(note_internal_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"), ActionKind::View))
            .action_with(note_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(note_internal_action("navigatorEngagementInput", LocalizedLabel::native("Navigator Engagement Input", "Navigator-Eingabe"), ActionKind::View))
            .action_with(note_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(note_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            .action_with(note_internal_action(SET_ACTIVE_UTILITY_ACTION_ID, LocalizedLabel::native("Set Active Utility", "Aktives Werkzeug festlegen"), ActionKind::View))
            .action_with(note_internal_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), ActionKind::View))
            // 📝️ Staged argument forms for the palette-eligible actions.
            .action_args("addBlock", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Typ"), vec![
                    ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
                    ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
                    ActionArgOption::new("table", LocalizedLabel::native("Table", "Tabelle")),
                    ActionArgOption::new("math", LocalizedLabel::native("Math", "Mathe")),
                    ActionArgOption::new("stroke", LocalizedLabel::native("Ink", "Tinte")),
                    ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
                ]).required().default_value("text"),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).default_value(0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).default_value(0.0),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("semio", LocalizedLabel::native("Semio", "Semio")),
                ]).required().default_value("semio"),
            ])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()])
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
            .keybinding("mod+a", "selectAll")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("escape", "clearSelection")
            .keybinding("up", "nudgeSelectionUp")
            .keybinding("down", "nudgeSelectionDown")
            .keybinding("left", "nudgeSelectionLeft")
            .keybinding("right", "nudgeSelectionRight")
            .keybinding("shift+up", "nudgeSelectionUpFast")
            .keybinding("shift+down", "nudgeSelectionDownFast")
            .keybinding("shift+left", "nudgeSelectionLeftFast")
            .keybinding("shift+right", "nudgeSelectionRightFast")
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE) —
            // note has no user-visible sticky config defaults (unlike shooting's default shot/asset
            // format), so `config_spec()` stays the trait default (`ConfigSpec::empty()`); registering it
            // here still declares the config schema for the manifest.
            .config(NotePlayApp::config_spec()),
    );
    for window in app.definition.window_kinds.iter_mut() {
        if window.id == NOTE_PLAY_WINDOW_COMPOSITE {
            window.options.measures = composite::window_measures(&document, &crate::artifacts::note::NoteCamera::default(), &crate::apps::note::terminology::NotePlayLabels::NATIVE_EN);
        } else if window.id == NOTE_PLAY_WINDOW_NAVIGATOR {
            window.options.measures = navigator::window_measures(&document, &crate::artifacts::note::NoteCamera::default(), &crate::apps::note::terminology::NotePlayLabels::NATIVE_EN);
        }
    }
    app.example("semio", LocalizedLabel::native("Semio", "Semio"), crate::artifacts::note::engine::semio_example_json(), "sparkles").workflow("note", "Note", "document")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type NoteApp = VcsArtifactApp<NotePlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn note_app() -> NoteApp {
        new_app::<NotePlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn note_app_with_registry() -> NoteApp {
        new_app_with_registry::<NotePlayApp>(create_note_app)
    }

    pub fn dispatch(app: &mut NoteApp, command: NoteCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut NoteApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::note_app;
    use semio_framework_plugin::{testkit, ActionKind as Kind};

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[test]
    fn command_ids_are_unique_across_every_row() {
        let app = NotePlayApp;
        let ids: Vec<&str> = every_command().iter().map(|command| NotePlayApp::command_id(command)).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 40, "every NoteCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<NoteCommand> {
        vec![
            NoteCommand::SetGridVisible(set_grid_visible::SetGridVisible { value: Some(true) }),
            NoteCommand::SetGridSpacing(set_grid_spacing::SetGridSpacing { value: 16.0 }),
            NoteCommand::SetGridSubdivisions(set_grid_subdivisions::SetGridSubdivisions { value: 8.0 }),
            NoteCommand::SetGridOpacity(set_grid_opacity::SetGridOpacity { value: 0.6 }),
            NoteCommand::SetSnapEnabled(set_snap_enabled::SetSnapEnabled { value: Some(false) }),
            NoteCommand::SetSnapGridSpacing(set_snap_grid_spacing::SetSnapGridSpacing { value: 4.0 }),
            NoteCommand::SetPencilWidth(set_pencil_width::SetPencilWidth { value: 5.0 }),
            NoteCommand::SetEraserRadius(set_eraser_radius::SetEraserRadius { value: 20.0 }),
            NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 10.0, y: 20.0 }),
            NoteCommand::MoveBlock(move_block::MoveBlock { block_id: "b1".into(), target_row_id: "note-play-block:b2".into(), drop_position: "after".into() }),
            NoteCommand::DeleteBlock(delete_block::DeleteBlock { block_id: "b1".into() }),
            NoteCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            NoteCommand::DuplicateBlock(duplicate_block::DuplicateBlock { block_id: "b1".into() }),
            NoteCommand::DuplicateSelection(duplicate_selection::DuplicateSelection {}),
            NoteCommand::PatchBlocks(patch_blocks::PatchBlocks { block_ids: vec!["b1".into()], field: "name".into(), value: "Renamed".into() }),
            NoteCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "semio".into() }),
            NoteCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{\"schema\":\"note.document\"}".into() }),
            NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None }),
            NoteCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed".into()) }),
            NoteCommand::NudgeSelection(nudge_selection::NudgeSelection { dx: 1.0, dy: -1.0 }),
            NoteCommand::NudgeSelectionUp(nudge_selection_up::NudgeSelectionUp {}),
            NoteCommand::NudgeSelectionDown(nudge_selection_down::NudgeSelectionDown {}),
            NoteCommand::NudgeSelectionLeft(nudge_selection_left::NudgeSelectionLeft {}),
            NoteCommand::NudgeSelectionRight(nudge_selection_right::NudgeSelectionRight {}),
            NoteCommand::NudgeSelectionUpFast(nudge_selection_up_fast::NudgeSelectionUpFast {}),
            NoteCommand::NudgeSelectionDownFast(nudge_selection_down_fast::NudgeSelectionDownFast {}),
            NoteCommand::NudgeSelectionLeftFast(nudge_selection_left_fast::NudgeSelectionLeftFast {}),
            NoteCommand::NudgeSelectionRightFast(nudge_selection_right_fast::NudgeSelectionRightFast {}),
            NoteCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::note::NoteCamera { x: 9.0, y: 9.0, zoom: 2.0 } }),
            NoteCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 1.5 }),
            NoteCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "pencil".into() }),
            NoteCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            NoteCommand::SelectAll(select_all::SelectAll {}),
            NoteCommand::ClearSelection(clear_selection::ClearSelection {}),
            NoteCommand::SetSelection(set_selection::SetSelection { ids: vec!["b1".into()] }),
            NoteCommand::SetHover(set_hover::SetHover { block_id: Some("b1".into()) }),
            NoteCommand::EngagementInput(engagement_input::EngagementInput { value: "Renaming…".into() }),
            NoteCommand::NavigatorEngagementInput(navigator_engagement_input::NavigatorEngagementInput {}),
            NoteCommand::SaveDownload(save_download::SaveDownload {}),
            NoteCommand::LoadRequest(load_request::LoadRequest {}),
        ]
    }

    /// 🎞️ Pins the exact hex for rows whose `Option` fields make `None`/`Some` distinct wire cases —
    /// copied from the pre-migration `🧪️wire-baseline-before.txt` dump.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::SetGridVisible(set_grid_visible::SetGridVisible { value: None }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::SetSnapEnabled(set_snap_enabled::SetSnapEnabled { value: None }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::SetHover(set_hover::SetHover { block_id: None }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: "[]".into(), phase: "begin".into(), select_ids: None }));
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_note_app().definition).expect("app definition json");
        for id in [NOTE_PLAY_WINDOW_COMPOSITE, NOTE_PLAY_WINDOW_NAVIGATOR] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for body in [NOTE_PLAY_BODY_DOCUMENT, NOTE_PLAY_BODY_CATALOGUE, NOTE_PLAY_BODY_PROPERTIES] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("2d.note"), "artifact kind missing from the manifest");
    }

    #[test]
    fn utility_registry_declares_canvas_utilities_scoped_to_composite_window() {
        let definition = create_note_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectDirect", "selectMarquee", "text", "image", "table", "math", "pencil", "eraserStroke", "eraserPoint", "pan"]);
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectDirect", "selectMarquee"]);
        let composite_window = definition.window_kinds.iter().find(|window| window.id == NOTE_PLAY_WINDOW_COMPOSITE).expect("composite window");
        assert_eq!(composite_window.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite canvas");
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, Kind::View)));
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Locale
    #[test]
    fn note_labels_resolve_native_by_default() {
        let mut app = note_app();
        let document_json = crate::apps::note::testkit::render(&mut app, NOTE_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Add Text"));
        let catalogue_json = crate::apps::note::testkit::render(&mut app, NOTE_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Block kinds"));
    }
    //#endregion 🔖️Locale

    //#region 🔖️CrossCutting
    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = note_app();
        testkit::assert_undo_redo_round_trip(&mut app, NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("snapshot").blocks.len(), 0, 1);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same document,
    /// apply DISJOINT edits, and exchanging operations over a `MemoryBackbone` converges both sides to
    /// contain BOTH edits.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<NotePlayApp, (usize, Option<bool>)>(
            "mem://note-convergence",
            NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }),
            NoteCommand::SetGridVisible(set_grid_visible::SetGridVisible { value: Some(false) }),
            |app| {
                let projection = app.snapshot().expect("snapshot");
                (projection.blocks.len(), projection.grid_visible)
            },
        );
    }

    #[test]
    fn ingest_operations_is_idempotent_for_note() {
        testkit::assert_ingest_idempotent::<NotePlayApp, f64>(NoteCommand::SetGridSpacing(set_grid_spacing::SetGridSpacing { value: 48.0 }), |app| app.snapshot().expect("snapshot").grid_spacing.unwrap_or_default());
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
