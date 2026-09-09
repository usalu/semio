//! 🖥️ Note editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum
//! and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `☑️options/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the
//! artifact's `⚙️engine`. This file is a routing table: `handle` → `NoteCommand::dispatch`, `render` →
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
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
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
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppDefinition, ArtifactEditor, ArtifactView, ConfigView, Dialect, DomainTopology, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UtilityCategory, UtilityDefinition, WindowEngagement,
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
pub use document_panel::NOTE_PLAY_BODY_DOCUMENT;
pub use inspection_panel::NOTE_PLAY_BODY_PROPERTIES;
pub use navigator::{NOTE_PLAY_BODY_NAVIGATOR, NOTE_PLAY_WINDOW_NAVIGATOR};
//#endregion 🔖️Constants

//#region 🔖️ResetDocument
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `setActiveExample`/`setFixtureJson` build a
/// `Effect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry.
pub fn reset_document_effect(document: &NoteSnapshot) -> semio_framework::kernel::Effect {
    let pack = <NoteSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<NoteSnapshot, NoteMutation>(NOTE_DOCUMENT_SCHEMA, "note", document.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("note document spr encode is infallible for a fresh, edit-free envelope");
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
//#endregion 🔖️Utilities

//#region 🔖️Interaction
/// 🕹️ "blocks" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own Group nesting, transitive
/// (selecting/hovering a group covers its descendants).
pub const NOTE_INTERACTION_BLOCKS: &str = "blocks";

/// 🕹️ Per-dispatch scratch: the "blocks" domain's current selection, resolved once by
/// `ArtifactEditor::handle` from `InteractionView` and threaded to every leaf command handler —
/// `app_commands!`'s generated `dispatch` has no way to thread `InteractionView` itself (mirrors
/// `📐️cad`'s `CadDispatchCtx`). Row-id-prefixed `InteractionTarget` ids (the SAME canonical ids the
/// document panel tree renders, see its own doc comment) are converted back to raw block ids here,
/// once, so every downstream handler keeps working with the raw ids it always did.
pub struct NoteDispatchCtx {
    pub selected_block_ids: Vec<String>,
    pub id_owner: crate::schema::NoteIdOwner,
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
    pub enum NoteCommand for NoteSnapshot, NoteMutation, NoteConfig, NoteConfigMutation, ctx = NoteDispatchCtx {
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

//#region 🔖️NotePlayApp
/// 🧪️ B1: unit struct — every former `NotePlayRuntime`/`ViewModel`-read field now lives in
/// `NoteConfig` (see `ArtifactEditor::Config`), written through `NoteConfigMutation`s.
#[derive(Default)]
pub struct NotePlayApp;

impl ArtifactEditor for NotePlayApp {
    type Snapshot = NoteSnapshot;
    type Mutation = NoteMutation;
    type Config = NoteConfig;
    type ConfigMutation = NoteConfigMutation;
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
    const DOCUMENT_SCHEMA: &'static str = NOTE_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<NotePlayApp>,
        owner_file: "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.note.note@1/*#editor",
        document_schema: "note.document",
        factory: "NoteCommandJobFactory",
        factory_type: crate::editor::note::retained::NoteCommandJobFactory,
        tools: {
            "setGridVisible" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setGridSpacing" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setCamera" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "setCameraZoom" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "engagementInput" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "navigatorEngagementInput" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
            "loadRequest" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
        }
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        crate::editor::note::retained::register(registry)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        crate::editor::note::retained::build(request)
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(crate::editor::note::retained::artifact_preparation_factory())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(crate::editor::note::retained::config_preparation_factory())
    }

    fn app_schema() -> Option<framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::note::config::schema::app_schema_descriptor())
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

    fn handle(
        command: &NoteCommand,
        doc: &ArtifactView<'_, NoteSnapshot>,
        cfg: &ConfigView<'_, NoteConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<NoteMutation, NoteConfigMutation, Self::DraftMutation>, Fault> {
        let selected_block_ids = interaction.selection(NOTE_INTERACTION_BLOCKS).ids.iter().filter_map(|id| crate::schema::block_id_from_tree_row_id(id)).collect();
        let mut ctx = NoteDispatchCtx { selected_block_ids, id_owner: crate::schema::NoteIdOwner::for_document_child(doc.snapshot, command.command_id()) };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🕹️ `blocks` domain: `HierarchyProvider::Topology` from the document's own Group nesting — see
    /// `note_blocks_topology`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(NOTE_INTERACTION_BLOCKS.to_string(), note_blocks_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = note_play_labels(view_state);
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or("selectDirect");
        match body_key {
            NOTE_PLAY_BODY_COMPOSITE => composite::render(document, config, active_utility),
            NOTE_PLAY_BODY_NAVIGATOR => navigator::render(document, config, active_utility),
            NOTE_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            NOTE_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            NOTE_PLAY_BODY_PROPERTIES => inspection_panel::render(document, active_utility, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "note diagnostic text admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    fn window_engagements(doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or("selectDirect");
        HashMap::from([(NOTE_PLAY_WINDOW_COMPOSITE.to_string(), composite::engagement(doc.snapshot, &config.camera, &config.engagement_input)), (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), navigator::engagement(active_utility))])
    }

    fn window_measures(doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = note_play_labels(view_state);
        HashMap::from([(NOTE_PLAY_WINDOW_COMPOSITE.to_string(), composite::window_measures(doc.snapshot, &config.camera, labels)), (NOTE_PLAY_WINDOW_NAVIGATOR.to_string(), navigator::window_measures(doc.snapshot, &config.camera, labels))])
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
                    ActionArgOption::new("semio", LocalizedLabel::native("Semio", "Semio")),
                ]).required().default_value(&"semio"),
            ])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()])
            .action_interactive_job("setGridVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSubdivisions", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setGridOpacity", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSnapEnabled", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSnapGridSpacing", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setPencilWidth", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setEraserRadius", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addBlock", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("moveBlock", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteBlock", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("duplicateBlock", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("duplicateSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchBlocks", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setFixtureJson", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("inkApplyEvents", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionUp", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionDown", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionLeft", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionRight", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionUpFast", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionDownFast", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionLeftFast", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nudgeSelectionRightFast", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setCameraZoom", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("navigatorEngagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("saveDownload", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
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
                granularities: vec![GranularityDefinition { id: "block".into(), label: LocalizedLabel::native("Block", "Block"), icon_id: "square".into() }],
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

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

/// 🏷️ Admits Note display text into a bounded semantic label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    value.as_ref().try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "note label admission failed"))
}
