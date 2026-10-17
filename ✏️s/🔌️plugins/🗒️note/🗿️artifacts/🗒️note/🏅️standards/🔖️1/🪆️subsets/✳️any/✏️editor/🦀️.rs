//! 🖥️ Note editor surface — the `ArtifactEditor` impl (dispatch-only) and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `☑️options/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, exact view state in `🪟️window`, shared compute in
//! the artifact's `⚙️engine`. This file is a routing table: `handle` → `NoteCommand::dispatch`, `render` →
//! body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::note::commands::ink_apply_events;
use crate::editor::note::command_enum::{args_bridge, NoteCommand};
use crate::editor::note::dispatch_ctx::NoteDispatchCtx;
use crate::editor::note::support::{note_active_utility, note_blocks_topology, note_internal_action, note_utility, NOTE_INTERACTION_BLOCKS, NOTE_INTERACTION_GRANULARITY};
use crate::editor::note::modes::edit;
use crate::editor::note::modes::edit::windows::{composite, navigator};
use crate::editor::note::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::note::presence::{NotePresence, NotePresenceMutation};
use crate::editor::note::terminology::note_play_labels;
use crate::op::NoteMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::empty_note_snapshot;
use crate::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants
pub use catalogue_panel::NOTE_PLAY_BODY_CATALOGUE;
pub use composite::{NOTE_PLAY_BODY_COMPOSITE, NOTE_PLAY_WINDOW_COMPOSITE};
pub use document_panel::NOTE_PLAY_BODY_ARTIFACT;
pub use inspection_panel::NOTE_PLAY_BODY_PROPERTIES;
pub use navigator::{NOTE_PLAY_BODY_NAVIGATOR, NOTE_PLAY_WINDOW_NAVIGATOR};
//#endregion 🔖️Constants

//#region 🧬️AppSchema
fn note_app_schema_descriptor() -> semio_framework_schema_registry::AppSchemaDescriptor {
    semio_framework_schema_registry::AppSchemaDescriptor {
        id: "s.note.note",
        config: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
        presence: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//#endregion 🧬️AppSchema

//#region 🔖️NotePlayApp
/// 🧪️ B1: unit struct — document preferences live in `NoteSnapshot`; composite camera and
/// engagement input live in their exact WindowConfig and WindowTransient owners.
#[derive(Default)]
pub struct NotePlayApp;

impl ArtifactEditor for NotePlayApp {
    /// 📢️ The localized notices of the ink tool and retained route refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        note_fault_notices()
    }

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
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
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
            "loadDocumentJson" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
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

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(crate::editor::note::retained::artifact_preparation_factory())
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
        Some(std::sync::Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<NotePresence>::default()))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<NotePresence>::default()))
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

    fn app_schema() -> Option<semio_framework_schema_registry::AppSchemaDescriptor> {
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
    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
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
    fn interaction_topology(doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(NOTE_INTERACTION_BLOCKS.to_string(), note_blocks_topology(doc.snapshot));
        InteractionTopology { domains }
    
})())
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
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key != NOTE_PLAY_BODY_COMPOSITE {
            return Self::render(body_key, doc, cfg, view_state);
        }
        let window = crate::editor::note::window::config_from_view(cfg);
        let preview = crate::editor::note::window::transient_from_view(transient).ink_tool.map(|state| ink_apply_events::note_ink_tool_preview(doc.snapshot, &state));
        composite::render_with_interaction(preview.as_ref().unwrap_or(doc.snapshot), &window.camera, note_active_utility(view_state), interaction).map(semio_framework_plugin::built_to_component_tree)

        })();
        rendered.map(Some).map_err(Into::into)
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
            // 🐚️ Export footer action → panel Shell action emitting a host effect (S); opening a file is the framework Import Document.
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
            .action_with(note_internal_action("loadDocumentJson", LocalizedLabel::native("Load Document JSON", "Dokument-JSON laden"), ActionKind::Mutation))
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
            .action_args("loadDocumentJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()])
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
            .action_describe("saveDownload", LocalizedLabel::native("Hands the note to the host as a downloadable file.", "Übergibt die Notiz dem Host als herunterladbare Datei."))
            .action_use_when("saveDownload", vec!["export the note".into(), "download this note".into()])
            .action_describe("moveBlock", LocalizedLabel::native("Moves one block to a new position on the page.", "Verschiebt einen Block an eine neue Position auf der Seite."))
            .action_describe("deleteBlock", LocalizedLabel::native("Removes one block from the note by id.", "Entfernt einen Block anhand seiner Id aus der Notiz."))
            .action_describe("duplicateBlock", LocalizedLabel::native("Copies one block and inserts the copy beside the original.", "Kopiert einen Block und fügt die Kopie neben dem Original ein."))
            .action_describe("patchBlocks", LocalizedLabel::native("Sets one named property on several blocks at once.", "Setzt eine benannte Eigenschaft auf mehreren Blöcken gleichzeitig."))
            .action_describe("nudgeSelection", LocalizedLabel::native("Shifts the selected blocks by a given offset.", "Verschiebt die ausgewählten Blöcke um einen angegebenen Versatz."))
            .action_use_when("nudgeSelection", vec!["move the selection a little".into(), "shift these blocks".into()])
            .action_describe("loadDocumentJson", LocalizedLabel::native("Loads a whole note document from JSON text.", "Lädt ein vollständiges Notizdokument aus JSON-Text."))
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
            .action_destructive("loadDocumentJson")
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
            .action_interactive_job("loadDocumentJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
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

//#endregion 🧪️UnitTests


//#region 🪢️TaxonomyMounts


//#endregion 🪢️TaxonomyMounts

/// 📣️ The en/de notices of every `note.ink-*` / `note.retained.transaction` refusal code (design §20.12).
pub fn note_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 5]> = std::sync::LazyLock::new(|| {
        [
            ("note.ink-tool.provisional", LocalizedLabel::native("The ink stroke could not be previewed.", "Der Tintenstrich konnte nicht in der Vorschau angezeigt werden.")),
            ("note.ink-gesture.invalid", LocalizedLabel::native("The ink gesture is not valid.", "Die Tintengeste ist ungültig.")),
            ("note.ink-events.invalid", LocalizedLabel::native("The ink input could not be read.", "Die Tinteneingabe konnte nicht gelesen werden.")),
            ("note.ink-phase.invalid", LocalizedLabel::native("The ink gesture step is not known.", "Der Schritt der Tintengeste ist unbekannt.")),
            ("note.retained.transaction", LocalizedLabel::native("This change must be made as one step.", "Diese Änderung muss in einem Schritt erfolgen.")),
        ]
    });
    &*NOTICES
}
