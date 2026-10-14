//! 🧩️ Puzzle 2d play app — the plugin's 2d play app: its `ArtifactApp` impl (dispatch-only), the
//! transient `Puzzle2dScene` bundle its command/panel/window nodes mutate and render, the shared
//! snapshot helpers they build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.
//!
//! 📸️ The native play root owns a canonical typed snapshot and a derived first-party host view.
//! Commands publish typed deltas; tool runs append their typed mutations to the operation ledger.

#[path="📸️snapshot/🦀️.rs"]
pub mod snapshot;

#[path="📼️recorder/🦀️.rs"]
pub mod recorder;

use crate::editor::puzzle2d::snapshot::{PUZZLE2D_ADD_NODE_DIALOG_ID, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, PUZZLE2D_PROXIMITY_GESTURE_MAX, Puzzle2dInteractionSnapshot, Puzzle2dScene, apply_host_events, drain_board_events_json, puzzle2d_active_utility, puzzle2d_addresses_locked_entity, puzzle2d_clear_selection_write, puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_entity_hidden, puzzle2d_entity_label, puzzle2d_handle_kind, puzzle2d_interaction_definition, puzzle2d_kinds_compatible, puzzle2d_load_document_effect, puzzle2d_node_kind_arg, puzzle2d_occupied_handles, puzzle2d_paste_operations_on, puzzle2d_select_scope, puzzle2d_selected_node_ids, puzzle2d_selection_is_locked, puzzle2d_selection_write, puzzle2d_window_and_engagements_scope, puzzle2d_window_only_scope, sync_host_fixture_content, sync_host_runtime_state};
use crate::editor::puzzle2d::config::PUZZLE2D_LOD_MODE_AUTOMATIC;
use crate::editor::puzzle2d::commands::{
    accept_suggestion, add_node, add_target_region, apply_board_events, close_handle_suggestions, create_edge, cycle_candidate, delete_edge, delete_selection, delete_target_region, duplicate_selection, engagement_abort, engagement_control_select, engagement_input, engagement_repeat_last, engagement_submit,
    export_snapshot, focus_selection, force_layout, hover_suggestion, target_brush_suggestions, import_snapshot, lod_scale_json, open_handle_suggestions, open_import_snapshot, patch_inspector, proximity_connect, relocate_target_region, rotate_selection, scale_selection, select_same_kind, set_active_example,
    set_area_brush_size, set_brush_kind_weights, set_brush_node_size, set_brush_placement_contact_tolerance, set_brush_placement_overlap_budget, set_camera, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode_for_pane, set_proximity_radius,
    set_selectable_kind, set_selection_flag, set_suggestion_offset, set_target_region_flag, set_transform_gumball_flag, translate_selection,
};
use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dPlayRuntime};
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::engine::{handle_position_on_circle, handle_position_on_rectangle, BoardHost, Point, Puzzle2dExtension};
use crate::editor::puzzle2d::modes::edit;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::precompute::fill as fill_run;
pub use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord, SelectToolRequest};
use semio_framework_tool_machine::{ChartGesture, GesturePhase};
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::{area_brush as area_brush_utility, brush as brush_utility, select as select_utility};
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::panels::{artifact, catalogue, inspection, settings};
use crate::editor::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
pub use crate::editor::puzzle2d::terminology::{puzzle2d_localized, puzzle2d_localized_phrase};
use crate::editor::puzzle2d::window::{self, Puzzle2dWindowConfig, Puzzle2dWindowTransient};
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;

use crate::standards::v1::subsets::any::schema::mutations::{Puzzle2dMutation};
use crate::editor::puzzle2d::recorder::Puzzle2dRecorder;

use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactPresentation;
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
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
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
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
// 🕹️ `InteractionView` — see puzzle3d's identical import comment (missing top-level re-export from
// `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use semio_framework_pack_json::{json, Value};
#[cfg(test)] use semio_framework_pack_json::{Object};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants





























//#endregion 🔖️Constants

//#region 🔖️Scene

















































































//#endregion 🔖️Scene

//#region 🔖️BoardReads








//#region 🎯️TargetRegions










//#endregion 🎯️TargetRegions






















//#endregion 🔖️BoardReads

//#region 📋️Clipboard
















//#endregion 📋️Clipboard

//#region 🔖️BoardHostSync











//#endregion 🔖️BoardHostSync

//#region 🔖️UiScopes







//#endregion 🔖️UiScopes

//#region 🔖️Puzzle2dCommand





//#endregion 🔖️Puzzle2dCommand

//#region 🔖️ActionContext
/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (host sync, host-event replay, delta computation, config snapshotting) stays in
/// [`ArtifactApp::handle`]; an arm only mutates this bundle.
pub struct Puzzle2dActionCtx<'a> {
    /// 🎲️ The app's long-lived board engine — every arm reaching it goes through `borrow_mut()`.
    pub host: &'a RefCell<BoardHost>,
    pub scene: &'a mut Puzzle2dScene,
    /// 🪟️ The window instance this action was dispatched from, when the caller named one.
    pub window_id: Option<&'a str>,
    /// 🧭️ The registered kind of that exact window instance.
    pub window_kind: &'a str,
    /// 🧰️ The active utility resolved for `window_id` BEFORE this action ran.
    pub active_utility: String,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — retained selection-acting verbs read
    /// `.selected_ids()` here instead of the deleted `Puzzle2dConfig::selected_ids` field.
    pub selection: &'a protocol::DomainSelection,
    pub effects: &'a mut Vec<Effect>,
    /// 📼️ The kinds this action's board edits record, one entity at a time, over the document it reads.
    pub recorder: &'a mut Puzzle2dRecorder,
    pub artifact_mutations: &'a mut Vec<Puzzle2dMutation>,
    /// 🕹️ App-initiated selection writes riding alongside this action (`Emit::interaction_writes`).
    pub interaction_writes: &'a mut Vec<semio_framework_plugin::InteractionWrite>,
    pub ui_scope: &'a mut UiDirtyScope,
    /// 🗣️ The resolved locale×terminology label set an arm raises user-facing prose from — a refusal
    /// notice is a real sentence, never a fault code.
    pub labels: &'static crate::editor::puzzle2d::terminology::Puzzle2dLabels,
    /// 🪪️ Exact public command authority retained by framework continuations.
    pub operation: Option<semio_framework_plugin::AppOperationContext>,
    /// 🧬️ The typed document this action reads as its base — what the select tool yields against.
    pub base: &'a Puzzle2dPlaySnapshot,
    /// 🌱️ The admission's authoring seed every tool transaction this action commits is minted from; empty for
    /// a render/test view without command authority, which then publishes the yielded mutations plainly.
    pub authoring_seed: &'a str,
    /// 📐️ The document revision this action reads (hex) — a persisted select-tool gesture opened on another one
    /// is aborted, `baseMoved`.
    pub base_revision: &'a str,
    /// 🛠️ The tool transaction this action committed — stamped on every op it publishes.
    pub transaction: &'a mut Option<protocol::TransactionRef>,
    /// 🖐️ The dispatching window's gesture slot: every tool of this action drives the window's ONE gesture through it.
    pub gesture: &'a semio_framework_plugin::app::GestureSlot<Puzzle2dMutation>,
}

impl<'a> Puzzle2dActionCtx<'a> {
    pub fn selected_ids(&self) -> Vec<String> {
        self.selection.ids.clone()
    }

    /// 🛠️ Commits `records` through the select tool machine as ONE tool transaction of this action — the
    /// parametric selection leaves plus the connections their drops land, yielded as `verb`. A request whose
    /// every target is locked is refused at the tool with the one lock sentence and leaves zero trace; a request
    /// with a movable target is yielded whole, and its leaf reports the locked rest as `mutation.partial`.
    pub fn commit_selection(&mut self, verb: &str, records: Vec<Puzzle2dSelectionRecord>) {
        self.transform_selection(verb, GesturePhase::Once, records);
    }

    /// 🌊️ Drives this window's select tool through ONE dispatch of a selection transform on the window's gesture slot.
    /// `Once` commits the records as one transaction; `Stream` upserts them into the window's open transaction —
    /// opening it on the first tick — which the runtime keeps and overlays on the document every render reads; `Commit`
    /// folds the final records in and commits the whole gesture as ONE edit; `Abort` drops the open gesture with zero
    /// trace. Another verb, a one-shot, a moved base and every host fact (a blur, a lost capture, a utility switch, a
    /// frozen document) end an open gesture in the slot — this action maps none of them.
    pub fn transform_selection(&mut self, verb: &str, phase: GesturePhase, records: Vec<Puzzle2dSelectionRecord>) {
        if matches!(phase, GesturePhase::Abort(_)) && self.gesture.open().is_none() {
            *self.ui_scope = UiDirtyScope::None;
            return;
        }
        let base = self.base.typed();
        let refused = records.iter().any(|record| record.refused_as_locked(base));
        let request = (!matches!(phase, GesturePhase::Abort(_))).then(|| SelectToolRequest { base: self.base.shared_typed(), proximity_radius: self.scene.runtime.proximity_radius, records });
        let committed = self.gesture.drive::<ChartGesture<select_utility::select_tool::SelectTool>>(None, verb, phase, request, self.authoring_seed).ok().flatten();
        match committed {
            Some((transaction, mutations)) => self.publish_transaction(transaction, mutations),
            None if refused && self.gesture.open().is_none() => {
                self.notice(|labels| labels.selection_locked.as_str());
                *self.ui_scope = UiDirtyScope::None;
            }
            None => {}
        }
    }

    /// 🧾️ Publishes one committed tool transaction of this action: its leaves join the action's ONE edit, which the
    /// first transaction the action commits stamps.
    fn publish_transaction(&mut self, transaction: protocol::TransactionRef, mutations: Vec<Puzzle2dMutation>) {
        if self.transaction.is_none() && !self.authoring_seed.is_empty() {
            *self.transaction = Some(transaction);
        }
        self.artifact_mutations.extend(mutations);
    }

    /// 🧱️ Commits the board-tool rows of ONE flush (`rows`: kind and payload, in board order): every run of one kind is
    /// one release of that tool — ONE `ToolTransaction` of `<appId>#<kind>` on the window's gesture slot, folded on the
    /// document the runs before it left. Answers the document after every release, `None` when no row changed it.
    pub fn release_board(&mut self, rows: &[(&'static str, Value)]) -> Option<std::sync::Arc<crate::Puzzle2dSnapshot>> {
        let mut after: Option<std::sync::Arc<crate::Puzzle2dSnapshot>> = None;
        for run in rows.chunk_by(|left, right| left.0 == right.0) {
            let base = after.clone().unwrap_or_else(|| self.base.shared_typed());
            let release = apply_board_events::BoardToolRelease::new(base, run[0].0, run.iter().map(|row| row.1.clone()).collect());
            let committed = self.gesture.drive::<ChartGesture<apply_board_events::board_tool::BoardTool>>(None, run[0].0, GesturePhase::Once, Some(release.clone()), self.authoring_seed).ok().flatten();
            if let Some((transaction, mutations)) = committed {
                self.publish_transaction(transaction, mutations);
                after = Some(std::sync::Arc::clone(&release.outcome().after));
            }
        }
        after
    }

    /// 🎯️ The selected ids a selection transform may move: nodes and target regions of the base, in first-seen
    /// selection order without repeats — handles and edges are selection, never transform targets.
    pub fn selected_transform_targets(&self) -> Vec<String> {
        let typed = self.base.typed();
        select_utility::puzzle2d_unique_targets(self.selection.ids.iter().filter(|id| typed.nodes.iter().any(|node| &node.id == *id) || typed.target_regions.iter().any(|region| &region.id == *id)).cloned())
    }

    /// 🧯️ Raises exactly ONE localized sentence on the shell's transient-notice channel
    /// (`Effect::Notify` → `ShellHost`'s `showTransientNotice`). A second call inside the same action
    /// adds nothing, so the effect list of a refusal stays fixed-width — the twin of puzzle3d's
    /// `Puzzle3dActionCtx::notice`.
    pub fn notice(&mut self, message: impl Fn(&crate::editor::puzzle2d::terminology::Puzzle2dLabels) -> &'static str) {
        if self.effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })) {
            return;
        }
        let text = message(self.labels).to_string();
        self.effects.push(Effect::Notify { message: text });
    }

    /// 🔒️ The single lock gate every destructive or moving 2d verb asks first: a locked node, handle,
    /// edge or target region among `ids` refuses the whole gesture with one visible sentence, makes no
    /// document edit and raises no fault. Answers whether it refused, so an arm reads
    /// `if ctx.refuse_when_locked(&ids) { return }`. A silent no-op is what this replaces — the
    /// 2026-09-17 battery measured `deleteSelection` erasing 12 locked entities while the same node's
    /// drag was (silently) refused.
    pub fn refuse_when_locked(&mut self, ids: &[String]) -> bool {
        if !puzzle2d_addresses_locked_entity(&self.scene.board_snapshot, ids) {
            return false;
        }
        self.notice(|labels| labels.selection_locked.as_str());
        *self.ui_scope = UiDirtyScope::None;
        true
    }
}

/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d label admission failed"))
}
//#endregion 🔖️ActionContext

//#region 🔖️ContextMenu
/// 🖱️ On-demand puzzle 2d board context menu from selection snapshot. Grouped disclosure:
/// toggleHidden/toggleLocked/duplicate/focusSelection stay top-level (the four most frequent
/// verbs); selectSameKind folds into the "selection" taxonomy group; deleteSelection stays the
/// destructive tail. `organize_context_menu` (applied automatically at the
/// `VcsArtifactApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES` order and
/// inserts the pre-destructive separator itself, so no manual `.separator()` calls are needed here.
async fn puzzle2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, snapshot: &Value, selected: &[String], view_state: &semio_framework_plugin::ViewModel) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu, SelectionKind};
    let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;
    // 🧩️ Bespoke-row helper (dynamic label/icon/args/disabled per selection state — not a plain
    // declared-action lookup) — appended via `Menu::item(...)`, the documented escape hatch.
    let item = |id: &str, label: &str, icon: &str, action: &str, args: Option<semio_framework::DslValue>, destructive: bool, disabled: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args,
        destructive: destructive.then_some(true),
        disabled: disabled.then_some(true),
        ..Default::default()
    };
    if selected.is_empty() {
        // 🗨️ The empty-board branch is the second declared entry point of the Add Node dialog (the
        // shell palette is the first) — the same pair puzzle3d binds `openAddObjectDialog` to.
        return Menu::of(registry, view_state)
            .item(item("openAddNodeDialog", if is_de { "Knoten hinzufügen…" } else { "Add Node…" }, "plus", "openAddNodeDialog", None, false, false))
            .item(item("selectAll", if is_de { "Alles auswählen" } else { "Select All" }, "select-all", "selectAll", None, false, false))
            .item(item("paste", if is_de { "Einfügen" } else { "Paste" }, "clipboard", "paste", None, false, false))
            .build();
    }
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut entities: Vec<&Value> = Vec::new();
    let mut has_selected_node = false;
    let mut selected_handle_ids: Vec<&str> = Vec::new();
    if let Some(nodes) = snapshot.get("nodes").and_then(|v| v.as_array()) {
        for node in nodes {
            if node.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                entities.push(node);
                has_selected_node = true;
            }
            if let Some(handles) = node.get("handles").and_then(|v| v.as_array()) {
                for handle in handles {
                    if let Some(id) = handle.get("id").and_then(|v| v.as_str()).filter(|id| selected_set.contains(id)) {
                        entities.push(handle);
                        selected_handle_ids.push(id);
                    }
                }
            }
        }
    }
    if let Some(edges) = snapshot.get("edges").and_then(|v| v.as_array()) {
        for edge in edges {
            if edge.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                entities.push(edge);
            }
        }
    }
    let any_visible = entities.iter().any(|entity| !puzzle2d_entity_hidden(entity));
    let any_unlocked = entities.iter().any(|entity| entity.get("locked").and_then(|v| v.as_bool()) != Some(true));
    let phrase = selection_count_phrase(view_state.locale, &[(selected.len(), SelectionKind::Item)]).unwrap_or_default();
    let hide_label = match (any_visible, is_de) {
        (true, true) => "Ausblenden",
        (true, false) => "Hide",
        (false, true) => "Einblenden",
        (false, false) => "Show",
    };
    let lock_label = match (any_unlocked, is_de) {
        (true, true) => "Sperren",
        (true, false) => "Lock",
        (false, true) => "Entsperren",
        (false, false) => "Unlock",
    };
    // 💡️ One selected handle is the one-shot placement picker's entry point — the row the brush
    // utility never had: it opens the suggestions popup on that handle WITHOUT arming the brush.
    let mut menu = Menu::of(registry, view_state);
    if let [only] = selected_handle_ids.as_slice() {
        menu = menu.item(item("suggestNodes", if is_de { "Knoten vorschlagen" } else { "Suggest nodes" }, "sparkles", "openHandleSuggestions", Some(semio_framework::dsl_value!({ "handleId": only })), false, false));
    }
    // 🔗️ Two selected handles are the `createEdge` gesture: the row stays visible but refuses itself
    // when either end is already connected or no compatibility rule admits the pair, so the menu says
    // what the document allows instead of offering a verb that would only raise a notice.
    if let [source, target] = selected_handle_ids.as_slice() {
        let occupied = puzzle2d_occupied_handles(snapshot);
        let kinds = puzzle2d_handle_kind(snapshot, source).zip(puzzle2d_handle_kind(snapshot, target));
        let connectable = !occupied.contains(*source) && !occupied.contains(*target) && kinds.is_some_and(|(source_kind, target_kind)| puzzle2d_kinds_compatible(snapshot, &source_kind, &target_kind));
        menu = menu.item(item("connectHandles", if is_de { "Verbinden" } else { "Connect" }, "link", "createEdge", Some(semio_framework::dsl_value!({ "source": source, "target": target })), false, !connectable));
    }
    menu.item(item("toggleHidden", hide_label, if any_visible { "eye-off" } else { "eye" }, "setSelectionFlag", Some(semio_framework::dsl_value!({ "flag": "hidden", "value": any_visible })), false, false))
        .item(item("toggleLocked", lock_label, if any_unlocked { "lock" } else { "lock-open" }, "setSelectionFlag", Some(semio_framework::dsl_value!({ "flag": "locked", "value": any_unlocked })), false, false))
        .item(item("duplicate", if is_de { "Duplizieren" } else { "Duplicate" }, "copy", "duplicateSelection", None, false, !has_selected_node))
        .item(item("focusSelection", if is_de { "Auf Auswahl zoomen" } else { "Zoom to selection" }, "crosshair", "focusSelection", None, false, false))
        // 📋️ The framework declares copy/cut/paste (and their mod+c/x/v keys); these rows are the
        // pointer route to the same three reserved verbs [`Puzzle2dClipboardJob`] answers.
        //
        // 🗂️ ONE disclosure group, not three top-level leaves. `organize_context_menu` counts
        // INTERACTIVE rows against `CONTEXT_MENU_ROW_BUDGET = 9` and only then appends the
        // `separator-organized-*` row ahead of the destructive one — so nine interactive rows take the
        // within-budget path and still render TEN rows. Three clipboard verbs that every shell already
        // binds to mod+c/x/v are the right three to fold: the menu is shorter, the budget is respected
        // as the shell measures it, and no verb is lost.
        .group("clipboard", |m| {
            m.item(item("copy", if is_de { "Kopieren" } else { "Copy" }, "copy", "copy", None, false, !has_selected_node))
                .item(item("cut", if is_de { "Ausschneiden" } else { "Cut" }, "scissors", "cut", None, false, !has_selected_node || !any_unlocked))
                .item(item("paste", if is_de { "Einfügen" } else { "Paste" }, "clipboard", "paste", None, false, false))
        })
        .group("selection", |m| m.item(item("selectSameKind", if is_de { "Gleiche Art auswählen" } else { "Select same kind" }, "layers", "selectSameKind", None, false, false)))
        .item(item("deleteSelection", &format!("{} ({phrase})", if is_de { "Löschen" } else { "Delete" }), "trash", "deleteSelection", None, true, false))
        .build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️PlayApp
/// 🧩️ Puzzle-2d play app. Owns the `BoardHost` engine; the persisted document (the bare snapshot json)
/// lives in the wrapping `VcsArtifactApp`'s operation store and the view state in `Puzzle2dConfig`.
#[derive(Default, Clone, Copy)]
pub struct Puzzle2dPlayApp;

impl Puzzle2dPlayApp {
    fn scene_for(snapshot: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str) -> Puzzle2dScene {
        Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: active_utility.into(), interaction: Puzzle2dInteractionSnapshot::default() }
    }

    fn scene_with(snapshot: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str, interaction: Puzzle2dInteractionSnapshot) -> Puzzle2dScene {
        Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: active_utility.into(), interaction }
    }

    /// 🖼️ ONE render body for both `render` entry points: the window transient and the live selection
    /// are what the request-context path adds; the bare path passes their defaults.
    fn render_body(
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        window_transient: &Puzzle2dWindowTransient,
        interaction: Puzzle2dInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let runtime = window::runtime(cfg.snapshot, &window_config, window_transient, Some(window_kind));
        let active_utility = puzzle2d_active_utility(Some(view_state));
        let snapshot = doc.snapshot.value().clone();
        let document_json = snapshot.to_string();
        let envelope = Self::scene_with(snapshot, runtime, active_utility, interaction);
        let labels = puzzle2d_labels(view_state);
        // 🪟️ One `TreeWindows` per render, read off the host's `ViewModel::tree_windows` for exactly
        // the body being rendered — every panel container below shares its first-paint row budget.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            overview::BODY_KEY => overview::render(&document_json, &envelope)?,
            detail::BODY_KEY => detail::render(&document_json, &envelope)?,
            selection::BODY_KEY => selection::render(&document_json, &envelope)?,
            artifact::PUZZLE2D_PLAY_BODY_LAYERS => artifact::render(&envelope, labels, &windows)?,
            catalogue::PUZZLE2D_PLAY_BODY_CATALOGUE => catalogue::render(&envelope, labels, &windows)?,
            inspection::PUZZLE2D_PLAY_BODY_PROPERTIES => inspection::render(&envelope, labels, &windows)?,
            settings::PUZZLE2D_PLAY_BODY_SETTINGS => settings::render(&envelope, labels, settings::panel_window_id(view_state))?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}

//#region 🧵️RetainedCommands
/// 🧵️ The 2D editor actions that carry an exact app-owned tool proof. Only these may declare
/// [`InteractiveJobClassification::Migrated`] — UI dispatch resolves a controller/owner/factory/tool
/// /schema proof for every migrated verb, and one without a registered factory is refused outright.
pub(crate) const PUZZLE2D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "forceLayout",
    "addNode",
    "applyBoardEvents",
    "reorganize",
    "closeHandleSuggestions",
    "acceptSuggestion",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "openHandleSuggestions",
    "hoverSuggestion",
    "targetBrushSuggestions",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "lodScaleJson",
    "patchInspectorNodes",
    "redrawHandles",
    "selectSameKind",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setFillCount",
    "setGridFactor",
    "setGridSnapEnabled",
    "setGridVisible",
    "setSelectableKind",
    "setBrushPlacementContactTolerance",
    "setBrushPlacementOverlapBudget",
    "engagementRepeatLast",
    "openAddNodeDialog",
    "setLodModeForPane",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "exportSnapshot",
    "importSnapshot",
    "openImportSnapshot",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "setSuggestionOffset",
    "setTransformGumballFlag",
    "addTargetRegion",
    "deleteTargetRegion",
    "relocateTargetRegion",
    "setTargetRegionFlag",
    "setTargetRegionHidden",
    "setTargetRegionLocked",
    "setAreaBrushSize",
    "createEdge",
    "deleteEdge",
    "proximityConnect",
    "setProximityRadius",
];

/// 🧭️ The two framework-injected host-configuration verbs. They are `InteractiveJobClassification::Migrated`
/// in the manifest and [`Puzzle2dPlayApp::host_configuration_mutation`] resolves each to one Config
/// mutation, so they need a generated tool id — without which `validate_tool_job_rows` refuses
/// [`Puzzle2dHostConfigurationProofs`]' generic bounded proofs and `dispatch_action` fails closed.
/// They carry NO app-owned factory, so they stay out of [`PUZZLE2D_RETAINED_TOOL_IDS`], which is what
/// keys [`Puzzle2dRetainedCommandJobFactory`] and what the retained-jobs snapshot pins.
pub(crate) const PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS: &[&str] = &["setActiveTool", "setActiveUtility"];

/// 🗂️ Every generated tool id this app declares: the app-owned retained verbs plus the two
/// host-configuration verbs. `OpBinary::TOOL_JOB_IDS` — the list the framework joins against the
/// migrated manifest rows.
pub(crate) const PUZZLE2D_TOOL_JOB_IDS: [&str; PUZZLE2D_RETAINED_TOOL_IDS.len() + PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len()] = {
    let mut ids = [""; PUZZLE2D_RETAINED_TOOL_IDS.len() + PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len()];
    let mut index = 0;
    while index < PUZZLE2D_RETAINED_TOOL_IDS.len() {
        ids[index] = PUZZLE2D_RETAINED_TOOL_IDS[index];
        index += 1;
    }
    let mut host = 0;
    while host < PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len() {
        ids[index + host] = PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS[host];
        host += 1;
    }
    ids
};
const PUZZLE2D_RETAINED_PAYLOAD_SCHEMA: &str = "board.ports.directed.v1.tool-command.v1";

/// 🎬️ The retained verbs whose whole completion is [`puzzle2d_dispatch_emit`] — one `🎮️commands/*`
/// arm run over a rebuilt scene/board host, then the same document delta and config snapshot
/// `handle` derives. Everything outside this list carries a bespoke `Work` (
/// `forceLayout`/`reorganize`) or an isolated reducer (`addNode`, `setActiveExample`).
const PUZZLE2D_GENERIC_TOOL_IDS: &[&str] = &[
    "closeHandleSuggestions",
    "acceptSuggestion",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "openHandleSuggestions",
    "hoverSuggestion",
    "targetBrushSuggestions",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "patchInspectorNodes",
    "selectSameKind",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setFillCount",
    "setGridFactor",
    "setGridSnapEnabled",
    "setGridVisible",
    "setSelectableKind",
    "setBrushPlacementContactTolerance",
    "setBrushPlacementOverlapBudget",
    "engagementRepeatLast",
    "openAddNodeDialog",
    "setLodModeForPane",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "setSuggestionOffset",
    "setTransformGumballFlag",
    "addTargetRegion",
    "deleteTargetRegion",
    "relocateTargetRegion",
    "setTargetRegionFlag",
    "setTargetRegionHidden",
    "setTargetRegionLocked",
    "setAreaBrushSize",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "importSnapshot",
    "openImportSnapshot",
    "createEdge",
    "deleteEdge",
    "proximityConnect",
    "setProximityRadius",
];

/// 🫙️ The one verb whose `🎮️commands/*` arm is empty by construction — `lodScaleJson` only reads a
/// pure engine LOD table and declares [`UiDirtyScope::None`]. It publishes nothing, so it completes
/// through `NoopPuzzleCommandWork` under a solo `HostOnly` contract rather than the dispatch pipeline.
const PUZZLE2D_HOST_ONLY_TOOL_IDS: &[&str] = &["lodScaleJson"];

/// 📏️ Raw wire bytes ONE retained 2d command may carry — one whole `importSnapshot` file, escaped, plus its envelope
/// (`PUZZLE_IMPORT_RAW_BYTES`, the widest command this route admits; a whole-board `applyBoardEvents` select over
/// Nakagin's 180 ids is far below it), the same figure the 3d factory admits.
const PUZZLE2D_COMMAND_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;
const PUZZLE2D_COMMAND_DECODED_ITEMS: usize = 16_384;

struct Puzzle2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    contract: semio_framework::ToolExecutionContract,
}

impl Puzzle2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self {
            keys: PUZZLE2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(),
            contract: semio_framework::ToolExecutionContract::resumable(PUZZLE2D_COMMAND_RAW_BYTES, PUZZLE2D_COMMAND_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        }
    }
}

impl ToolJobFactory for Puzzle2dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Puzzle2dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        self.contract
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Puzzle 2d retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Puzzle2dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_BOARD_SNAPSHOT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "forceLayout", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "redrawHandles", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "closeHandleSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidate", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidateBack", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "targetBrushSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "openHandleSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "hoverSuggestion", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setBrushKindWeights", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setBrushNodeSize", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSelectableKind", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementContactTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementOverlapBudget", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "openAddNodeDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setLodModeForPane", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "exportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openImportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setSuggestionOffset", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addTargetRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteTargetRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "relocateTargetRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetRegionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetRegionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetRegionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setAreaBrushSize", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "applyBoardEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "acceptSuggestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "patchInspectorNodes", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "lodScaleJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKind", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "createEdge", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteEdge", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "proximityConnect", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setProximityRadius", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}

//#region 📬️StorePreparation
/// 📦️ Fixed envelope one retained `Puzzle2dConfig` commit may occupy. `brush_candidates` is the only
/// variable-length field (one entry per compatible brush candidate on the open slot), so 64 KiB
/// leaves two orders of magnitude of headroom over a real slot while still refusing an unbounded root.
const PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536;

struct Puzzle2dConfigStorePreparation {
    owners: store::OneItemOwners<Puzzle2dConfig, Puzzle2dConfigMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    phase: u8,
    cancelled: bool,
}

/// 🎚️ Config-lane preparation factory — the precondition every `Config` publication contract above
/// depends on (`build_config_store_one_item_preparation_factory`); mirrors
/// `Puzzle3dConfigStorePreparationFactory`.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle2dConfigStorePreparationFactory;

/// 🌉️ Measures the fully encoded config root against the fixed store envelope. `Puzzle2dConfig` is
/// encoded through the same in-house `semio_framework_pack_json` writer its `OpBinary` codec uses, so the measured
/// length is the length the store actually retains.
fn puzzle2d_config_store_bounded_bytes(value: &Puzzle2dConfig) -> Result<usize, String> {
    let encoded = semio_framework_pack_json::to_json_string(value);
    if encoded.len() > PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Puzzle2d Config Store root exceeds its fixed envelope".to_string());
    }
    Ok(encoded.len())
}

fn puzzle2d_config_store_mutation_bytes(mutation: &Puzzle2dConfigMutation) -> Option<usize> {
    let encoded = semio_framework_pack_json::to_json_string(mutation);
    (encoded.len() <= PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded.len())
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::{Mutation as _, MutationDiff as _};
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its mutation owner"))?;
                if puzzle2d_config_store_mutation_bytes(mutation).is_none() {
                    return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Puzzle2d Config preparation rejected its exact mutation envelope"));
                }
                let completed_bytes = puzzle2d_config_store_bounded_bytes(base.get()).map_err(|message| ValueError::new(ValueRefusalKind::OwnershipLimit, message))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Config mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = completed_bytes;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dConfig, Puzzle2dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dConfig, Puzzle2dConfigMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle2dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle2dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Puzzle2dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle2d Config preparation rejected its lane".into());
        }
        let retained_bytes = puzzle2d_config_store_mutation_bytes(mutation).ok_or_else(|| "Puzzle2d Config preparation rejected its exact mutation".to_string())?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
    }

    fn begin_demand(&self, _mutation: &Puzzle2dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle2dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dConfigMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle2d Config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle2dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

/// 🧩️ The Artifact-lane sibling of the Config preparation above. A `Puzzle2dMutation` is admitted
/// generically through `protocol::Mutation`/`protocol::MutationDiff` rather than an allowlist,
/// because one retained completion emits whatever the granular document delta produced (create /
/// delete node, connect / disconnect handles, manifest, compatibility, catalogs) one per store turn.

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle2dArtifactStorePreparationFactory;

struct Puzzle2dArtifactStorePreparation {
    owners: store::OneItemOwners<Puzzle2dPlaySnapshot, Puzzle2dMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    phase: u8,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle2dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): `delete-node`
    /// re-creates the node and re-connects every edge on its handles, a selection transform restores each target.
    fn preflight(&self, mutation: &Puzzle2dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle2d Artifact preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<Puzzle2dPlaySnapshot, _>(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle2dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle2dArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dPlaySnapshot, Puzzle2dMutation, Puzzle2dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle2dPlaySnapshot, Puzzle2dMutation, Puzzle2dMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle2dArtifactStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::{Mutation as _, MutationDiff as _};
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Artifact preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its mutation owner"))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle2d Artifact mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = 1;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle2d Artifact preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dPlaySnapshot, Puzzle2dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle2dPlaySnapshot, Puzzle2dMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️StorePreparation

/// 🖌️ Upper bound on the events one board flush may carry into a single retained step. The browser
/// flushes a handful of events per interaction (`PUZZLE2D_FLUSH_NOW_EVENT_NAMES` in `🖥️Board2dHost`), so
/// one bounded step covers a real interaction; an oversized batch is refused rather than silently
/// truncated.
const PUZZLE2D_BOARD_EVENT_BATCH_LIMIT: usize = 256;

/// 🚚️ Whether one board-event batch carries a gesture record — a drag drop is what arms the proximity
/// auto-connect, so a batch carrying one prices the gesture's whole [`PUZZLE2D_PROXIMITY_GESTURE_MAX`] edge budget.
fn puzzle2d_batch_drops_a_node(events: &[Value]) -> bool {
    events.iter().any(|event| event.get("name").and_then(Value::as_str) == Some("gesture"))
}

fn puzzle2d_board_events_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if command.action_id() != "applyBoardEvents" {
        return None;
    }
    let events = command.args().and_then(|args| args.get("eventsJson")).and_then(Value::as_str).unwrap_or("[]");
    let parsed: Value = semio_framework_pack_json::parse(events,semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    let rows = parsed.as_array().map_or(&[][..], Vec::as_slice);
    if rows.len() > PUZZLE2D_BOARD_EVENT_BATCH_LIMIT {
        return None;
    }
    let connects = if puzzle2d_batch_drops_a_node(rows) { PUZZLE2D_PROXIMITY_GESTURE_MAX } else { 0 };
    Some(rows.len().saturating_add(connects).max(1))
}

struct Puzzle2dWindowCommandWork {
    tool_id: &'static str,
    extent: crate::retained_command::PuzzleCommandExtent<EditorApp<Puzzle2dPlayApp>>,
    authoring_seed: String,
    base_revision: String,
    consumed: bool,
}

impl Puzzle2dWindowCommandWork {
    fn new(tool_id: &'static str, extent: crate::retained_command::PuzzleCommandExtent<EditorApp<Puzzle2dPlayApp>>, authoring_seed: String, base_revision: String) -> Self {
        Self { tool_id, extent, authoring_seed, base_revision, consumed: false }
    }
}

/// 📐️ A canonical document revision as lowercase hex — the identity a persisted select-tool gesture pins its base by.
fn puzzle2d_revision_hex(revision: &[u8; 32]) -> String {
    revision.iter().map(|byte| format!("{byte:02x}")).collect()
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dWindowCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        (self.extent)(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: _hover, context, .. } = *input;
        if self.consumed {
            return Err(Fault::from("puzzle2d-window-work-repeated"));
        }
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_config = window::config_from_snapshot_or_document(window_config_snapshot, snapshot.value());
        let window_transient = window::transient_from_snapshot(context.and_then(|context| context.window_transient.as_ref()));
        let window_kind = window_config_snapshot.map(semio_framework_plugin::WindowConfigSnapshot::window_kind_id).or_else(|| view_state.and_then(window::kind_for_view)).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let selection = interaction.selection.get(PUZZLE2D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
        let detached = semio_framework_plugin::app::GestureSlot::detached();
        let gesture = context.map_or(&detached, |context| context.gesture());
        let (emit, ephemeral) = puzzle2d_dispatch_emit(command, snapshot, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), &selection, &self.authoring_seed, &self.base_revision, gesture, None)?;
        self.consumed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
    }
}

/// 📤 `exportSnapshot` reads the document and publishes a download; it owns no mutation, so it resolves
/// from the snapshot and picks its lane by payload size: one inline effect under the guest's contiguous
/// request ceiling, the framework's segmented-download lane above it, a notice above what one segmented
/// download may carry.
#[derive(Default)]
struct Puzzle2dExportWork {
    consumed: bool,
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dExportWork {
    fn tool_id(&self) -> &'static str {
        "exportSnapshot"
    }

    fn extent(&self, _command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        Some(1)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        if self.consumed {
            return Err(Fault::from("puzzle2d-export-work-repeated"));
        }
        self.consumed = true;
        Ok(match export_snapshot::puzzle2d_export_publication(snapshot.value())? {
            export_snapshot::Puzzle2dExportPublication::Inline(effect) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { effects: vec![effect], ui_scope: UiDirtyScope::None, ..Default::default() }),
            export_snapshot::Puzzle2dExportPublication::Segmented(download) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { download: download, ephemeral: Default::default() },
            export_snapshot::Puzzle2dExportPublication::Refused(message) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { effects: vec![Effect::Notify { message }], ui_scope: UiDirtyScope::None, ..Default::default() }),
        })
    }
}

/// 🎬️ THE dispatch pipeline — the one implementation both [`ArtifactEditor::handle`]'s batch path and
/// every retained generic reduce run: rebuild the scene and a fresh board host from
/// `(command, document, config, selection)`, run the `🎮️commands/*` arm, replay the host's owned
/// events, then derive the granular document delta and the config snapshot. A selection transform never
/// touches the scene: its arm commits the select tool's ONE transaction, whose yielded parametric
/// mutations ride this emit stamped with the transaction ref. `operation` is the committed public
/// authority a mounted continuation carries and is simply `None` for a retained work, which never sees an
/// `ArtifactView`; `authoring_seed` is the admission's seed every tool transaction id is minted from.
fn puzzle2d_dispatch_emit(
    command: &Puzzle2dCommand,
    document: &Puzzle2dPlaySnapshot,
    config: &Puzzle2dConfig,
    window_config: &Puzzle2dWindowConfig,
    window_transient: &Puzzle2dWindowTransient,
    window_kind: &str,
    view_state: Option<&semio_framework_plugin::ViewModel>,
    active_utility: &str,
    selection: &protocol::DomainSelection,
    authoring_seed: &str,
    base_revision: &str,
    gesture: &semio_framework_plugin::app::GestureSlot<Puzzle2dMutation>,
    operation: Option<semio_framework_plugin::AppOperationContext>,
) -> Result<(Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, EphemeralEmit<EditorApp<Puzzle2dPlayApp>>), Fault> {
    let (action, args, window_id) = (command.action_id(), command.args(), command.window_id());
    let before = document.value();
    let active_utility = active_utility.to_string();
    let runtime = window::runtime(config, window_config, window_transient, Some(window_kind));
    let interaction = Puzzle2dInteractionSnapshot { granularity: selection.granularity.clone(), selected: selection.ids.clone(), ..Default::default() };
    let mut scene = Puzzle2dPlayApp::scene_with(before.clone(), runtime, &active_utility, interaction);
    // 🐚️ ArtifactApp::handle is pure (no &self) — rebuild a fresh BoardHost from the document
    // each call. The previous last_synced_fixture cache lived on &self and cannot return.
    let host = RefCell::new(BoardHost::default());
    {
        let mut host_mut = host.borrow_mut();
        if action != "applyBoardEvents" {
            sync_host_fixture_content(&mut host_mut, &scene);
            let _ = drain_board_events_json(&mut host_mut);
        }
        sync_host_runtime_state(&mut host_mut, &scene, &selection.ids);
    }
    let mut effects: Vec<Effect> = Vec::new();
    let mut recorder = Puzzle2dRecorder::new(document);
    let mut artifact_mutations = Vec::new();
    let mut interaction_writes = Vec::new();
    let mut transaction = None;
    // 🐢️ Default to Full (safe: every unrecognized/rare action re-renders everything); the
    // narrow-tier arms below override it to the smallest scope that actually covers what they touch.
    let mut ui_scope = UiDirtyScope::Full;
    {
        let labels = puzzle2d_labels(view_state.ok_or_else(|| Fault::from("puzzle2d action requires the caller's explicit presentation axes"))?);
        let ctx = &mut Puzzle2dActionCtx {
            host: &host,
            scene: &mut scene,
            window_id,
            window_kind,
            active_utility,
            selection,
            effects: &mut effects,
            recorder: &mut recorder,
            artifact_mutations: &mut artifact_mutations,
            interaction_writes: &mut interaction_writes,
            ui_scope: &mut ui_scope,
            labels,
            operation,
            base: document,
            authoring_seed,
            base_revision,
            transaction: &mut transaction,
            gesture,
        };
        match action {
            "selectSameKind" => select_same_kind::select_same_kind(ctx),
            "createEdge" => create_edge::create_edge(ctx, args),
            "deleteEdge" => delete_edge::delete_edge(ctx, args),
            "proximityConnect" => proximity_connect::proximity_connect(ctx, args),
            "setProximityRadius" => set_proximity_radius::set_proximity_radius(ctx, args),
            "deleteSelection" => delete_selection::delete_selection(ctx),
            "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
            "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
            "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),
            "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),
            "addNode" => add_node::add_node(ctx, args),
            "patchInspectorNodes" => patch_inspector::patch_inspector(ctx, args),
            "forceLayout" | "reorganize" => {let mut progress=|_|true;let mut control=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(100_000_000,&mut progress);force_layout::force_layout(ctx,&mut control)},
            "setCamera" => set_camera::set_camera(ctx, args),
            "focusSelection" => focus_selection::focus_selection(ctx),
            "engagementInput" => engagement_input::engagement_input(ctx, args),
            "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
            "engagementAbort" => engagement_abort::engagement_abort(ctx, args),
            "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
            "setLodModeForPane" => set_lod_mode_for_pane::set_lod_mode_for_pane(ctx, args),
            "translateSelection" => translate_selection::translate_selection(ctx, args),
            "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
            "scaleSelection" => scale_selection::scale_selection(ctx, args),
            "importSnapshot" => import_snapshot::import_snapshot(ctx, args),
            "openImportSnapshot" => open_import_snapshot::open_import_snapshot(ctx),
            "lodScaleJson" => lod_scale_json::lod_scale_json(ctx),
            "setGridSnapEnabled" => set_grid_snap_enabled::set_grid_snap_enabled(ctx, args),
            "setGridFactor" => set_grid_factor::set_grid_factor(ctx, args),
            "setGridVisible" => set_grid_visible::set_grid_visible(ctx, args),
            "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
            "setBrushPlacementContactTolerance" => set_brush_placement_contact_tolerance::set_brush_placement_contact_tolerance(ctx, args),
            "setBrushPlacementOverlapBudget" => set_brush_placement_overlap_budget::set_brush_placement_overlap_budget(ctx, args),
            "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
            "openAddNodeDialog" => {
                ctx.effects.push(Effect::OpenDialog { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), dialog_id: PUZZLE2D_ADD_NODE_DIALOG_ID.into(), args: None });
                *ctx.ui_scope = UiDirtyScope::None;
            }
            "setBrushKindWeights" => set_brush_kind_weights::set_brush_kind_weights(ctx, args),
            "setBrushNodeSize" => set_brush_node_size::set_brush_node_size(ctx, args),
            "setSuggestionOffset" => set_suggestion_offset::set_suggestion_offset(ctx, args),
            "setTransformGumballFlag" => set_transform_gumball_flag::set_transform_gumball_flag(ctx, args),
            "addTargetRegion" => add_target_region::add_target_region(ctx, args),
            "deleteTargetRegion" => delete_target_region::delete_target_region(ctx, args),
            "relocateTargetRegion" => relocate_target_region::relocate_target_region(ctx, args),
            "setTargetRegionFlag" => set_target_region_flag::set_target_region_flag(ctx, args),
            "setTargetRegionHidden" => set_target_region_flag::set_target_region_flag_value(ctx, args, "hidden"),
            "setTargetRegionLocked" => set_target_region_flag::set_target_region_flag_value(ctx, args, "locked"),
            "setAreaBrushSize" => set_area_brush_size::set_area_brush_size(ctx, args),
            "setFillCount" => set_fill_count::set_fill_count(ctx, args),
            "cycleBrushCandidate" => cycle_candidate::cycle_candidate(ctx, args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true)),
            "cycleBrushCandidateBack" => cycle_candidate::cycle_candidate(ctx, false),
            "hoverSuggestion" => hover_suggestion::hover_suggestion(ctx, args),
            "openHandleSuggestions" => open_handle_suggestions::open_handle_suggestions(ctx, args),
            "acceptSuggestion" => accept_suggestion::accept_suggestion(ctx, args),
            "closeHandleSuggestions" => close_handle_suggestions::close_handle_suggestions(ctx),
            "targetBrushSuggestions" => target_brush_suggestions::target_brush_suggestions(ctx, args),
            "applyBoardEvents" => apply_board_events::apply_board_events(ctx, args),
            _ => {}
        }
    }
    apply_host_events(&mut host.borrow_mut(), &mut recorder, &mut scene.runtime);
    let mut operations = recorder.into_rows();
    operations.append(&mut artifact_mutations);
    // 🐢️ Safety net: a `None` scope claims nothing needs re-rendering — never pair that with an
    // actual document mutation (would silently desync remote clients' UI from the committed operation).
    if !operations.is_empty() && matches!(ui_scope, UiDirtyScope::None) {
        ui_scope = UiDirtyScope::Full;
    }
    // 🧮️ B1: only a REAL config change becomes a `Puzzle2dConfigMutation` — `PartialEq` (derived)
    // makes this cheap, and keeps a pure read-only action from creating a no-op undo entry.
    let (next_config, next_window_config, next_window_transient) = window::split(&scene.runtime, window_kind);
    let config_mutations = config.mutations_to(&next_config);
    let window_config_mutations = if &next_window_config != window_config { vec![window::addressed_config(view_state.ok_or_else(|| Fault::from("puzzle2d-window-context-required"))?, next_window_config)?] } else { Vec::new() };
    let window_transient = if &next_window_transient != window_transient { vec![window::addressed_transient(view_state.ok_or_else(|| Fault::from("puzzle2d-window-context-required"))?, next_window_transient)?] } else { Vec::new() };
    // 🛠️ The first tool transaction an action commits stamps every op of its ONE edit — the yielded leaves and any
    // scene delta the same dispatch carried — so history lists the gesture as one row whose inputs stay editable.
    let transaction = transaction.filter(|_| !operations.is_empty());
    Ok((Emit { artifact_mutations: operations, config_mutations, window_config_mutations, transaction, effects, ui_scope, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() }))
}

/// 🗂️ Upper bound on the entities one selection-acting retained step may touch. Nakagin — this
/// artifact's largest example — carries 180 nodes, 179 edges and 358 handles, so a whole-board
/// selection is 717 entities: this ceiling admits that with headroom while still refusing an
/// unbounded selection well under the shared `PUZZLE_COMMAND_WORK_ITEMS` (4,096) budget.
const PUZZLE2D_SELECTION_BATCH_LIMIT: usize = 1_024;
/// 📥 Work items one whole-document import replacement claims: the largest example's entity count with
/// headroom, still well under the shared `PUZZLE_COMMAND_WORK_ITEMS` budget.
const PUZZLE2D_IMPORT_FIXTURE_WORK_ITEMS: usize = 1_024;

/// 🧮️ One work item per entity a selection-acting verb rewrites, one for every other generic verb
/// (each is a fixed-shape config/host setter whose cost is independent of document size). An
/// oversized selection returns `None` and is refused by the retained preflight rather than silently
/// truncated. `patchInspectorNodes` may address an explicit `ids` argument instead of the selection.
fn puzzle2d_generic_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    let action = command.action_id();
    if !PUZZLE2D_GENERIC_TOOL_IDS.contains(&action) {
        return None;
    }
    if action == "importSnapshot" {
        return Some(PUZZLE2D_IMPORT_FIXTURE_WORK_ITEMS);
    }
    if !matches!(action, "patchInspectorNodes" | "setSelectionFlag" | "setSelectionHidden" | "setSelectionLocked" | "setTargetRegionFlag" | "setTargetRegionHidden" | "setTargetRegionLocked" | "deleteSelection" | "duplicateSelection" | "translateSelection" | "rotateSelection" | "scaleSelection" | "proximityConnect") {
        return Some(1);
    }
    let selected = interaction.selection.get(PUZZLE2D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let addressed = if matches!(action, "setTargetRegionFlag" | "setTargetRegionHidden" | "setTargetRegionLocked") && command.args().and_then(|args| args.get("id")).is_some() {
        1
    } else {
        command.args().filter(|_| matches!(action, "patchInspectorNodes" | "setSelectionHidden" | "setSelectionLocked")).and_then(|args| args.get("ids")).and_then(Value::as_array).map_or(selected, Vec::len)
    };
    if addressed > PUZZLE2D_SELECTION_BATCH_LIMIT {
        return None;
    }
    // 🧲️ `translateSelection` and `proximityConnect` land open handles, so both carry the gesture's
    // fixed auto-connect budget on top of the entities they rewrite.
    let connects = if matches!(action, "translateSelection" | "proximityConnect") { PUZZLE2D_PROXIMITY_GESTURE_MAX } else { 0 };
    Some(addressed.saturating_add(connects).max(1))
}

/// 🛍️ The example switch is a whole-document LOAD: `Effect::LoadDocument` carrying the example's pack (an unknown id loads
/// the empty board) and no mutation row, so it leaves no history row either. One bounded step, shared by the retained
/// `setActiveExample` route and the batch `handle` path.
fn puzzle2d_active_example_emit(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _config: &Puzzle2dConfig) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    let id = set_active_example::canonical_example_id(command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or(""));
    Ok(Emit { effects: vec![puzzle2d_load_document_effect(set_active_example::target(id))], ui_scope: UiDirtyScope::Full, ..Default::default() })
}

/// ➕️ `addNode` places one node through the shared window-command dispatch, so it is one admitted unit.
fn puzzle2d_add_node_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    (command.action_id() == "addNode").then_some(1)
}

fn puzzle2d_active_example_reduce(
    command: &Puzzle2dCommand,
    snapshot: &Puzzle2dPlaySnapshot,
    config: &Puzzle2dConfig,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    puzzle2d_active_example_emit(command, snapshot, config)
}

fn puzzle2d_active_example_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    (command.action_id() == "setActiveExample").then_some(1)
}

/// 🧲️ Admission ceilings for one force-layout run. `MAX_NODES` admits the real
/// `🏗️nakagin-capsule-tower` snapshot (180 nodes / 179 edges / 358 handles) with room to spare — the
/// former 64-node ceiling made `forceLayout` structurally dead on the only non-trivial example this
/// artifact ships. `MAX_HANDLES` is checked by [`Puzzle2dForceLayoutWork::extent`] as well as by the
/// `Handles` stage, so an over-wide document is refused at preflight instead of faulting mid-run.
const PUZZLE2D_FORCE_MAX_NODES: usize = 512;
const PUZZLE2D_FORCE_MAX_EDGES: usize = 4_096;
const PUZZLE2D_FORCE_MAX_HANDLES: usize = 4_096;

/// 🔁️ Iteration schedule. A spring embedder's cost per iteration is `n·(n−1)/2` repulsion pairs plus
/// one pass per edge, so a fixed 420 iterations is affordable at 64 nodes and ~7 million pair
/// evaluations at 180. Iterations are therefore derived from the graph's own per-iteration cost
/// ([`puzzle2d_force_iterations`]) so total pair+spring work stays inside
/// [`PUZZLE2D_FORCE_WORK_BUDGET`]: small graphs keep the full 420-iteration quality, Nakagin gets
/// 122, and the 512-node ceiling gets the 24-iteration floor.
const PUZZLE2D_FORCE_ITERATIONS_MAX: u32 = 420;
const PUZZLE2D_FORCE_ITERATIONS_MIN: u32 = 24;
const PUZZLE2D_FORCE_WORK_BUDGET: u64 = 2_000_000;

/// 🍰️ Per-`step()` chunk sizes. One retained step is one checkpoint boundary, so a step must carry a
/// batch of force units rather than a single one — 8,192 pair evaluations are a few hundred
/// microseconds against the 7,500 µs step budget, while one pair per step would need ~7 million
/// checkpoints for Nakagin. `extent()` counts these chunks, which is what makes the declared budget
/// match the work actually performed.
const PUZZLE2D_FORCE_UNITS_PER_STEP: usize = 8_192;
const PUZZLE2D_FORCE_NODES_PER_STEP: usize = 512;
const PUZZLE2D_FORCE_SCAN_PER_STEP: usize = 256;
const PUZZLE2D_FORCE_ITERATION_STAGE_STEPS: usize = 4;
const PUZZLE2D_FORCE_PROLOGUE_STAGE_STEPS: usize = 8;

/// 🔢️ Unordered node pairs the repulsion pass visits per iteration.
fn puzzle2d_force_pairs(nodes: usize) -> u64 {
    let nodes = nodes as u64;
    nodes.saturating_mul(nodes.saturating_sub(1)) / 2
}

/// 🔁️ Iterations for a graph of `nodes` nodes and `edges` edges — `WORK_BUDGET / cost_per_iteration`,
/// clamped to `[MIN, MAX]`. Derived from the raw document counts (not the visible subset) so
/// [`Puzzle2dForceLayoutWork::extent`] and [`Puzzle2dForceLayoutWork::step`] always agree.
fn puzzle2d_force_iterations(nodes: usize, edges: usize) -> u32 {
    let cost = puzzle2d_force_pairs(nodes).saturating_add(edges as u64);
    if cost == 0 {
        return 1;
    }
    u32::try_from(PUZZLE2D_FORCE_WORK_BUDGET / cost).unwrap_or(PUZZLE2D_FORCE_ITERATIONS_MAX).clamp(PUZZLE2D_FORCE_ITERATIONS_MIN, PUZZLE2D_FORCE_ITERATIONS_MAX)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dForceStage {
    Nodes,
    Handles,
    Edges,
    Seed,
    Center,
    Reset,
    Repel,
    Springs,
    Integrate,
    Emit,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle2dForceLayoutWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dForceLayoutWorkOwners {
    node_ids: Vec<String>,
    raw_node_indices: Vec<usize>,
    original: Vec<Option<[f64; 2]>>,
    positions: Vec<[f64; 2]>,
    velocities: Vec<[f64; 2]>,
    forces: Vec<[f64; 2]>,
    radii: Vec<f64>,
    id_to_index: HashMap<String, usize>,
    handle_to_node: HashMap<String, String>,
    edges: Vec<(usize, usize)>,
    edge_set: HashSet<(usize, usize)>,
    mutations: Vec<Puzzle2dMutation>,
}

struct Puzzle2dForceLayoutWork {
    tool_id: &'static str,
    stage: Puzzle2dForceStage,
    node_cursor: usize,
    handle_cursor: usize,
    edge_cursor: usize,
    seed_cursor: usize,
    center_cursor: usize,
    force_cursor: usize,
    pair_i: usize,
    pair_j: usize,
    iteration: u32,
    rng: u64,
    center: [f64; 2],
    finite_count: usize,
    node_ids: Vec<String>,
    raw_node_indices: Vec<usize>,
    original: Vec<Option<[f64; 2]>>,
    positions: Vec<[f64; 2]>,
    velocities: Vec<[f64; 2]>,
    forces: Vec<[f64; 2]>,
    radii: Vec<f64>,
    id_to_index: HashMap<String, usize>,
    handle_to_node: HashMap<String, String>,
    edges: Vec<(usize, usize)>,
    edge_set: HashSet<(usize, usize)>,
    mutations: Vec<Puzzle2dMutation>,
    retained_bytes: usize,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle2dForceLayoutWorkOwners>,
}

impl Default for Puzzle2dForceLayoutWork {
    fn default() -> Self {
        Self::new("forceLayout")
    }
}

impl Puzzle2dForceLayoutWork {
    /// 🧲️ `forceLayout` and `reorganize` are the same simulation behind two verbs, so the admitted
    /// tool id travels with the work — the retained job's decode phase rejects a payload whose
    /// action id does not equal [`crate::retained_command::PuzzleCommandWork::tool_id`].
    fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            stage: Puzzle2dForceStage::Nodes,
            node_cursor: 0,
            handle_cursor: 0,
            edge_cursor: 0,
            seed_cursor: 0,
            center_cursor: 0,
            force_cursor: 0,
            pair_i: 0,
            pair_j: 1,
            iteration: 0,
            rng: 0x5eedfaced0,
            center: [0.0, 0.0],
            finite_count: 0,
            node_ids: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            raw_node_indices: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            original: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            positions: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            velocities: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            forces: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            radii: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            id_to_index: HashMap::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            handle_to_node: HashMap::with_capacity(PUZZLE2D_FORCE_MAX_HANDLES),
            edges: Vec::with_capacity(PUZZLE2D_FORCE_MAX_EDGES),
            edge_set: HashSet::with_capacity(PUZZLE2D_FORCE_MAX_EDGES),
            mutations: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            retained_bytes: 0,
        }
    }

    fn visible(object: &semio_framework_pack_json::Object) -> bool {
        object.get("hidden").and_then(Value::as_bool).map_or_else(|| object.get("visible").and_then(Value::as_bool).unwrap_or(true), |hidden| !hidden)
    }

    fn radius(node: &Value) -> f64 {
        let Some(object) = node.as_object() else { return 32.0 };
        if object.get("shape").and_then(Value::as_str) == Some("rectangle") {
            let width = object.get("width").and_then(Value::as_f64).unwrap_or(40.0);
            let height = object.get("height").and_then(Value::as_f64).unwrap_or(40.0);
            return ((width * width + height * height).sqrt() * 0.5).max(8.0);
        }
        object.get("radius").and_then(Value::as_f64).filter(|radius| radius.is_finite() && *radius > 0.0).unwrap_or(32.0)
    }

    fn split_mix64(mut value: u64) -> u64 {
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D049BB133111EB);
        value ^ (value >> 31)
    }

    fn random_unit(&mut self) -> f64 {
        self.rng = Self::split_mix64(self.rng);
        (self.rng as f64) / (u64::MAX as f64)
    }

    fn pop_one(&mut self) -> bool {
        macro_rules! pop {
            ($field:ident) => {
                if self.$field.pop().is_some() {
                    return true;
                }
            };
        }
        pop!(mutations);
        pop!(edges);
        pop!(radii);
        pop!(forces);
        pop!(velocities);
        pop!(positions);
        pop!(original);
        pop!(raw_node_indices);
        pop!(node_ids);
        if let Some(key) = self.handle_to_node.keys().next().cloned() {
            self.handle_to_node.remove(&key);
            return true;
        }
        if let Some(key) = self.id_to_index.keys().next().cloned() {
            self.id_to_index.remove(&key);
            return true;
        }
        if let Some(edge) = self.edge_set.iter().next().copied() {
            self.edge_set.remove(&edge);
            return true;
        }
        false
    }

    /// 🌡️ Simulated-annealing factor for the current iteration against the size-derived schedule.
    fn cool(&self, iterations: u32) -> f64 {
        (1.0 - f64::from(self.iteration) / f64::from(iterations.max(1))).max(0.08)
    }

    fn scan_node_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let Some(node) = nodes.get(self.node_cursor) else {
            if self.positions.is_empty() {
                self.stage = Puzzle2dForceStage::Emit;
                return Ok(());
            }
            if self.finite_count > 0 {
                self.center[0] /= self.finite_count as f64;
                self.center[1] /= self.finite_count as f64;
            }
            self.stage = Puzzle2dForceStage::Edges;
            return Ok(());
        };
        let object = node.as_object().ok_or_else(|| Fault::from("puzzle2d-force-node-not-object"))?;
        if !Self::visible(object) {
            self.node_cursor += 1;
            return Ok(());
        }
        let id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-force-node-id-missing"))?;
        if self.node_ids.len() >= PUZZLE2D_FORCE_MAX_NODES || self.retained_bytes.checked_add(id.len()).is_none_or(|bytes| bytes > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES) {
            return Err(Fault::from("puzzle2d-force-node-capacity"));
        }
        let x = object.get("x").and_then(Value::as_f64);
        let y = object.get("y").and_then(Value::as_f64);
        let original = match (x, y) {
            (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some([x, y]),
            _ => None,
        };
        let index = self.positions.len();
        self.retained_bytes += id.len();
        self.id_to_index.insert(id.to_string(), index);
        self.node_ids.push(id.to_string());
        self.raw_node_indices.push(self.node_cursor);
        self.original.push(original);
        self.positions.push(original.unwrap_or([0.0, 0.0]));
        if let Some(position) = original {
            self.center[0] += position[0];
            self.center[1] += position[1];
            self.finite_count += 1;
        }
        self.velocities.push([0.0, 0.0]);
        self.forces.push([0.0, 0.0]);
        self.radii.push(Self::radius(node));
        self.handle_cursor = 0;
        self.stage = Puzzle2dForceStage::Handles;
        Ok(())
    }

    fn scan_handle_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let object = nodes.get(self.node_cursor).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-force-node-owner-lost"))?;
        let handles = object.get("handles").and_then(Value::as_array);
        let Some(handle) = handles.and_then(|handles| handles.get(self.handle_cursor)) else {
            self.node_cursor += 1;
            self.stage = Puzzle2dForceStage::Nodes;
            return Ok(());
        };
        self.handle_cursor += 1;
        let Some(handle_object) = handle.as_object() else { return Ok(()) };
        if !Self::visible(handle_object) {
            return Ok(());
        }
        let Some(handle_id) = handle_object.get("id").and_then(Value::as_str) else { return Ok(()) };
        let node_id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-force-node-id-owner-lost"))?;
        let added_bytes = handle_id.len().saturating_add(node_id.len());
        if self.handle_to_node.len() >= PUZZLE2D_FORCE_MAX_HANDLES || self.retained_bytes.checked_add(added_bytes).is_none_or(|bytes| bytes > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES) {
            return Err(Fault::from("puzzle2d-force-handle-capacity"));
        }
        self.retained_bytes += added_bytes;
        self.handle_to_node.insert(handle_id.to_string(), node_id.to_string());
        Ok(())
    }

    fn scan_edge_one(&mut self, edges: Option<&Vec<Value>>) -> Result<(), Fault> {
        let Some(edge) = edges.and_then(|edges| edges.get(self.edge_cursor)) else {
            self.stage = Puzzle2dForceStage::Seed;
            return Ok(());
        };
        self.edge_cursor += 1;
        let Some(object) = edge.as_object() else { return Ok(()) };
        if !Self::visible(object) {
            return Ok(());
        }
        let (Some(source), Some(target)) = (object.get("source").and_then(Value::as_str), object.get("target").and_then(Value::as_str)) else {
            return Ok(());
        };
        let source_node = self.handle_to_node.get(source).map_or(source, String::as_str);
        let target_node = self.handle_to_node.get(target).map_or(target, String::as_str);
        let (Some(&a), Some(&b)) = (self.id_to_index.get(source_node), self.id_to_index.get(target_node)) else {
            return Ok(());
        };
        let pair = (a.min(b), a.max(b));
        if a != b && self.edge_set.insert(pair) {
            if self.edges.len() >= PUZZLE2D_FORCE_MAX_EDGES {
                return Err(Fault::from("puzzle2d-force-edge-capacity"));
            }
            self.edges.push(pair);
        }
        Ok(())
    }

    fn seed_one(&mut self) {
        if self.seed_cursor >= self.positions.len() {
            self.center = [0.0, 0.0];
            self.stage = Puzzle2dForceStage::Center;
            return;
        }
        if self.positions[self.seed_cursor][0].hypot(self.positions[self.seed_cursor][1]) < 1e-9 {
            let t = self.seed_cursor as f64;
            let angle = t * 2.399_963_229_728_653_5;
            let radius = 10.0 + t.sqrt() * 22.0;
            let jitter_x = (self.random_unit() - 0.5) * 6.0;
            let jitter_y = (self.random_unit() - 0.5) * 6.0;
            self.positions[self.seed_cursor] = [self.center[0] + radius * angle.cos() + jitter_x, self.center[1] + radius * angle.sin() + jitter_y];
        }
        self.seed_cursor += 1;
    }

    fn center_one(&mut self) {
        if let Some(position) = self.positions.get(self.center_cursor) {
            self.center[0] += position[0];
            self.center[1] += position[1];
            self.center_cursor += 1;
            return;
        }
        if !self.positions.is_empty() {
            self.center[0] /= self.positions.len() as f64;
            self.center[1] /= self.positions.len() as f64;
        }
        self.stage = Puzzle2dForceStage::Reset;
        self.force_cursor = 0;
    }

    fn reset_one(&mut self) {
        if self.force_cursor < self.forces.len() {
            self.forces[self.force_cursor] = [0.0, 0.0];
            self.force_cursor += 1;
            return;
        }
        self.pair_i = 0;
        self.pair_j = 1;
        self.stage = Puzzle2dForceStage::Repel;
    }

    fn repel_one(&mut self, iterations: u32) {
        let count = self.positions.len();
        if self.pair_i >= count || self.pair_j >= count {
            self.edge_cursor = 0;
            self.stage = Puzzle2dForceStage::Springs;
            return;
        }
        let i = self.pair_i;
        let j = self.pair_j;
        let dx = self.positions[j][0] - self.positions[i][0];
        let dy = self.positions[j][1] - self.positions[i][1];
        let distance = dx.hypot(dy).max(1e-4);
        let repulsion = 6500.0 * self.cool(iterations) * (self.radii[i] * self.radii[j]).max(1.0) / (distance * distance);
        let fx = dx / distance * -repulsion;
        let fy = dy / distance * -repulsion;
        self.forces[i][0] += fx;
        self.forces[i][1] += fy;
        self.forces[j][0] -= fx;
        self.forces[j][1] -= fy;
        self.pair_j += 1;
        if self.pair_j >= count {
            self.pair_i += 1;
            self.pair_j = self.pair_i.saturating_add(1);
        }
    }

    fn spring_one(&mut self, iterations: u32) {
        let Some(&(i, j)) = self.edges.get(self.edge_cursor) else {
            self.force_cursor = 0;
            self.stage = Puzzle2dForceStage::Integrate;
            return;
        };
        let dx = self.positions[j][0] - self.positions[i][0];
        let dy = self.positions[j][1] - self.positions[i][1];
        let distance = dx.hypot(dy).max(1e-4);
        let magnitude = 0.028 * self.cool(iterations) * (distance - 140.0);
        let fx = dx / distance * magnitude;
        let fy = dy / distance * magnitude;
        self.forces[i][0] += fx;
        self.forces[i][1] += fy;
        self.forces[j][0] -= fx;
        self.forces[j][1] -= fy;
        self.edge_cursor += 1;
    }

    fn integrate_one(&mut self, iterations: u32) {
        if self.force_cursor >= self.positions.len() {
            self.iteration += 1;
            self.force_cursor = 0;
            self.stage = if self.iteration >= iterations { Puzzle2dForceStage::Emit } else { Puzzle2dForceStage::Reset };
            return;
        }
        let index = self.force_cursor;
        let delta_time = 0.85 * self.cool(iterations).sqrt();
        let mut velocity = [(self.velocities[index][0] + self.forces[index][0] * delta_time) * 0.88, (self.velocities[index][1] + self.forces[index][1] * delta_time) * 0.88];
        let speed = velocity[0].hypot(velocity[1]);
        if speed > 48.0 {
            velocity[0] *= 48.0 / speed;
            velocity[1] *= 48.0 / speed;
        }
        self.velocities[index] = velocity;
        self.positions[index][0] += velocity[0] * delta_time;
        self.positions[index][1] += velocity[1] * delta_time;
        self.force_cursor += 1;
    }

    fn emit_one(&mut self) -> bool {
        let index = self.force_cursor;
        let Some(position) = self.positions.get(index).copied() else {
            self.stage = Puzzle2dForceStage::Complete;
            return true;
        };
        if self.original[index] != Some(position) {
            self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::move_node(self.node_ids[index].clone().into(), position[0], position[1]));
        }
        self.force_cursor += 1;
        false
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dForceLayoutWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 📐️ One unit of the declared extent is one `step()` call, not one force evaluation: the
    /// prologue scans `nodes + handles + edges` in [`PUZZLE2D_FORCE_SCAN_PER_STEP`] chunks, each of
    /// the [`puzzle2d_force_iterations`] iterations costs one reset pass, one integrate pass, and the
    /// repulsion/spring chunks, and the epilogue emits one mutation pass. Handles are counted here
    /// too, so a document that would trip `PUZZLE2D_FORCE_MAX_HANDLES` mid-run is refused at
    /// preflight instead.
    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        if command.action_id() != self.tool_id || snapshot.value().get("schema").and_then(Value::as_str) != Some(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA) {
            return None;
        }
        let nodes = snapshot.value().get("nodes")?.as_array()?;
        let edges = snapshot.value().get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        let handles = nodes.iter().filter_map(|node| node.get("handles")).filter_map(Value::as_array).map(Vec::len).sum::<usize>();
        if nodes.len() > PUZZLE2D_FORCE_MAX_NODES || edges > PUZZLE2D_FORCE_MAX_EDGES || handles > PUZZLE2D_FORCE_MAX_HANDLES {
            return None;
        }
        let pairs = usize::try_from(puzzle2d_force_pairs(nodes.len())).ok()?;
        let node_pass = nodes.len().div_ceil(PUZZLE2D_FORCE_NODES_PER_STEP).max(1);
        let iteration = pairs.div_ceil(PUZZLE2D_FORCE_UNITS_PER_STEP).checked_add(edges.div_ceil(PUZZLE2D_FORCE_UNITS_PER_STEP))?.checked_add(node_pass.checked_mul(2)?)?.checked_add(PUZZLE2D_FORCE_ITERATION_STAGE_STEPS)?;
        let items = nodes
            .len()
            .checked_add(handles)?
            .checked_add(edges)?
            .div_ceil(PUZZLE2D_FORCE_SCAN_PER_STEP)
            .checked_add(node_pass.checked_mul(3)?)?
            .checked_add(PUZZLE2D_FORCE_PROLOGUE_STAGE_STEPS)?
            .checked_add(iteration.checked_mul(usize::try_from(puzzle2d_force_iterations(nodes.len(), edges)).ok()?)?)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    /// 🍰️ Each call drains one chunk of the current stage and yields a progress boundary, so the
    /// number of `step()` calls matches [`Puzzle2dForceLayoutWork::extent`] instead of running one
    /// force evaluation per checkpoint.
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let nodes = snapshot.value().get("nodes").and_then(Value::as_array).ok_or_else(|| Fault::from("puzzle2d-force-nodes-missing"))?;
        let edges = snapshot.value().get("edges").and_then(Value::as_array);
        let iterations = puzzle2d_force_iterations(nodes.len(), edges.map_or(0, Vec::len));
        match self.stage {
            Puzzle2dForceStage::Nodes | Puzzle2dForceStage::Handles => {
                for _ in 0..PUZZLE2D_FORCE_SCAN_PER_STEP {
                    match self.stage {
                        Puzzle2dForceStage::Nodes => self.scan_node_one(nodes)?,
                        Puzzle2dForceStage::Handles => self.scan_handle_one(nodes)?,
                        _ => break,
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-node", "Reading layout node", "Layout-Knoten wird gelesen"))
            }
            Puzzle2dForceStage::Edges => {
                for _ in 0..PUZZLE2D_FORCE_SCAN_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Edges {
                        break;
                    }
                    self.scan_edge_one(edges)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert"))
            }
            Puzzle2dForceStage::Seed => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Seed {
                        break;
                    }
                    self.seed_one();
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-seed", "Seeding layout node", "Layout-Knoten wird initialisiert"))
            }
            Puzzle2dForceStage::Center => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Center {
                        break;
                    }
                    self.center_one();
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-center", "Centering layout", "Layout wird zentriert"))
            }
            Puzzle2dForceStage::Reset => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Reset {
                        break;
                    }
                    self.reset_one();
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-reset", "Resetting layout force", "Layout-Kraft wird zurückgesetzt"))
            }
            Puzzle2dForceStage::Repel => {
                for _ in 0..PUZZLE2D_FORCE_UNITS_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Repel {
                        break;
                    }
                    self.repel_one(iterations);
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-repel", "Applying layout repulsion", "Layout-Abstoßung wird angewendet"))
            }
            Puzzle2dForceStage::Springs => {
                for _ in 0..PUZZLE2D_FORCE_UNITS_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Springs {
                        break;
                    }
                    self.spring_one(iterations);
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-spring", "Applying layout spring", "Layout-Feder wird angewendet"))
            }
            Puzzle2dForceStage::Integrate => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Integrate {
                        break;
                    }
                    self.integrate_one(iterations);
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-integrate", "Integrating layout node", "Layout-Knoten wird integriert"))
            }
            Puzzle2dForceStage::Emit => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.emit_one() {
                        let mutations = std::mem::take(&mut self.mutations);
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }));
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-emit", "Preparing layout mutation", "Layout-Mutation wird vorbereitet"))
            }
            Puzzle2dForceStage::Complete => Err(Fault::from("puzzle2d-force-complete-repolled")),
            Puzzle2dForceStage::Closing => Err(Fault::from("puzzle2d-force-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dForceStage::Closing;
        self.close_owners.stage(Puzzle2dForceLayoutWorkOwners { node_ids: std::mem::take(&mut self.node_ids), raw_node_indices: std::mem::take(&mut self.raw_node_indices), original: std::mem::take(&mut self.original), positions: std::mem::take(&mut self.positions), velocities: std::mem::take(&mut self.velocities), forces: std::mem::take(&mut self.forces), radii: std::mem::take(&mut self.radii), id_to_index: std::mem::take(&mut self.id_to_index), handle_to_node: std::mem::take(&mut self.handle_to_node), edges: std::mem::take(&mut self.edges), edge_set: std::mem::take(&mut self.edge_set), mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle2dForceStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// 🕸️ Admission ceilings and chunk size for one `redrawHandles` run. The legacy handler serialized
/// the entire snapshot and called `apply_edge_handle_snap_to_host_snapshot_v1_json` once, with no size
/// guard and no yield point; these bound the same three passes so a large board is refused at
/// preflight rather than blocking a step.
const PUZZLE2D_REDRAW_MAX_NODES: usize = 4_096;
const PUZZLE2D_REDRAW_MAX_EDGES: usize = 4_096;
const PUZZLE2D_REDRAW_MAX_HANDLES: usize = 8_192;
const PUZZLE2D_REDRAW_UNITS_PER_STEP: usize = 256;
const PUZZLE2D_REDRAW_STAGE_STEPS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dRedrawStage {
    Nodes,
    Handles,
    Edges,
    Emit,
    Complete,
    Closing,
}

/// 🔵️ The snap geometry of one node, mirroring the engine's own `NodeShapeSnap`: a node without a
/// finite centre — or a circle without a radius, or a rectangle without extents — has no rim to
/// snap onto and is skipped, exactly as `apply_edge_handle_snap_to_host_snapshot_v1_value` skips it.
#[derive(Clone, Copy)]
struct Puzzle2dRedrawShape {
    center: [f64; 2],
    rectangle: Option<[f64; 2]>,
}

semio_framework_value::artifact_retire_leaf!(Puzzle2dRedrawShape);

/// 🕸️ `redrawHandles` as a bounded, resumable walk. `Nodes`/`Handles` index every visible handle to
/// its owning node, `Edges` resolves each visible edge into one angle per endpoint (last edge wins
/// on a shared handle, matching the engine), and `Emit` publishes one `replace-node-handle` per
/// handle whose angle actually changed, each from the handle's own snap geometry — never a whole-board difference.
/// ♻️ The owners one `Puzzle2dRedrawHandlesWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dRedrawHandlesWorkOwners {
    shapes: Vec<Option<Puzzle2dRedrawShape>>,
    handle_locations: HashMap<String, (usize, usize)>,
    angles: std::collections::BTreeMap<(usize, usize), f64>,
    mutations: Vec<Puzzle2dMutation>,
}

struct Puzzle2dRedrawHandlesWork {
    stage: Puzzle2dRedrawStage,
    node_cursor: usize,
    handle_cursor: usize,
    edge_cursor: usize,
    shapes: Vec<Option<Puzzle2dRedrawShape>>,
    handle_locations: HashMap<String, (usize, usize)>,
    angles: std::collections::BTreeMap<(usize, usize), f64>,
    mutations: Vec<Puzzle2dMutation>,
    retained_bytes: usize,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle2dRedrawHandlesWorkOwners>,
}

impl Default for Puzzle2dRedrawHandlesWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle2dRedrawStage::Nodes,
            node_cursor: 0,
            handle_cursor: 0,
            edge_cursor: 0,
            shapes: Vec::with_capacity(PUZZLE2D_REDRAW_MAX_NODES),
            handle_locations: HashMap::with_capacity(PUZZLE2D_REDRAW_MAX_HANDLES),
            angles: std::collections::BTreeMap::new(),
            mutations: Vec::with_capacity(PUZZLE2D_REDRAW_MAX_HANDLES),
            retained_bytes: 0,
        }
    }
}

impl Puzzle2dRedrawHandlesWork {
    fn shape(object: &semio_framework_pack_json::Object) -> Option<Puzzle2dRedrawShape> {
        let cx = object.get("x").and_then(Value::as_f64)?;
        let cy = object.get("y").and_then(Value::as_f64)?;
        if object.get("shape").and_then(Value::as_str) == Some("rectangle") {
            let width = object.get("width").and_then(Value::as_f64)?;
            let height = object.get("height").and_then(Value::as_f64)?;
            return Some(Puzzle2dRedrawShape { center: [cx, cy], rectangle: Some([width, height]) });
        }
        object.get("radius").and_then(Value::as_f64)?;
        Some(Puzzle2dRedrawShape { center: [cx, cy], rectangle: None })
    }

    /// 🧭️ Delegates to the board engine's own conventions — east-zero for circles, north-zero for
    /// rectangles — so a chunked redraw and the engine's whole-snapshot snap agree numerically.
    fn angle_toward(from: Puzzle2dRedrawShape, toward: Puzzle2dRedrawShape) -> Option<f64> {
        let center = Point::new(from.center[0], from.center[1]);
        let target = Point::new(toward.center[0], toward.center[1]);
        if crate::editor::puzzle2d::engine::distance_between(center, target) <= 1e-9 {
            return None;
        }
        Some(match from.rectangle {
            Some([width, height]) => crate::editor::puzzle2d::engine::rectangle_handle_angle_toward(center, width, height, target),
            None => crate::editor::puzzle2d::engine::circle_handle_angle_toward(center, target),
        })
    }

    fn admit_bytes(&mut self, bytes: usize) -> Result<(), Fault> {
        let next = self.retained_bytes.checked_add(bytes).ok_or_else(|| Fault::from("puzzle2d-redraw-byte-capacity"))?;
        if next > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES {
            return Err(Fault::from("puzzle2d-redraw-byte-capacity"));
        }
        self.retained_bytes = next;
        Ok(())
    }

    fn index_node_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let Some(node) = nodes.get(self.node_cursor) else {
            self.edge_cursor = 0;
            self.stage = Puzzle2dRedrawStage::Edges;
            return Ok(());
        };
        let object = node.as_object().ok_or_else(|| Fault::from("puzzle2d-redraw-node-not-object"))?;
        if self.shapes.len() >= PUZZLE2D_REDRAW_MAX_NODES {
            return Err(Fault::from("puzzle2d-redraw-node-capacity"));
        }
        let visible = crate::editor::puzzle2d::engine::board_visible_or_true(&crate::editor::puzzle2d::engine::BoardVisibility {hidden:object.get("hidden").and_then(|v|v.as_bool()),visible:object.get("visible").and_then(|v|v.as_bool()),locked:object.get("locked").and_then(|v|v.as_bool())});
        self.shapes.push(visible.then(|| Self::shape(object)).flatten());
        if !visible {
            self.node_cursor += 1;
            return Ok(());
        }
        self.handle_cursor = 0;
        self.stage = Puzzle2dRedrawStage::Handles;
        Ok(())
    }

    fn index_handle_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let object = nodes.get(self.node_cursor).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-redraw-node-owner-lost"))?;
        let Some(handle) = object.get("handles").and_then(Value::as_array).and_then(|handles| handles.get(self.handle_cursor)) else {
            self.node_cursor += 1;
            self.stage = Puzzle2dRedrawStage::Nodes;
            return Ok(());
        };
        let index = self.handle_cursor;
        self.handle_cursor += 1;
        let Some(handle_object) = handle.as_object() else { return Ok(()) };
        if !crate::editor::puzzle2d::engine::board_visible_or_true(&crate::editor::puzzle2d::engine::BoardVisibility {hidden:handle_object.get("hidden").and_then(|v|v.as_bool()),visible:handle_object.get("visible").and_then(|v|v.as_bool()),locked:handle_object.get("locked").and_then(|v|v.as_bool())}) {
            return Ok(());
        }
        let Some(handle_id) = handle_object.get("id").and_then(Value::as_str) else { return Ok(()) };
        if self.handle_locations.len() >= PUZZLE2D_REDRAW_MAX_HANDLES {
            return Err(Fault::from("puzzle2d-redraw-handle-capacity"));
        }
        self.admit_bytes(handle_id.len())?;
        self.handle_locations.insert(handle_id.to_string(), (self.node_cursor, index));
        Ok(())
    }

    fn resolve_edge_one(&mut self, edges: Option<&Vec<Value>>) -> Result<(), Fault> {
        let Some(edge) = edges.and_then(|edges| edges.get(self.edge_cursor)) else {
            self.stage = Puzzle2dRedrawStage::Emit;
            return Ok(());
        };
        self.edge_cursor += 1;
        let Some(object) = edge.as_object() else { return Ok(()) };
        if !crate::editor::puzzle2d::engine::board_visible_or_true(&crate::editor::puzzle2d::engine::BoardVisibility {hidden:object.get("hidden").and_then(|v|v.as_bool()),visible:object.get("visible").and_then(|v|v.as_bool()),locked:object.get("locked").and_then(|v|v.as_bool())}) {
            return Ok(());
        }
        let Some((source, target)) = crate::editor::puzzle2d::engine::board_edge_handle_ids(object.get("source").and_then(|v|v.as_str()),object.get("target").and_then(|v|v.as_str())) else { return Ok(()) };
        let (Some(&source_location), Some(&target_location)) = (self.handle_locations.get(source), self.handle_locations.get(target)) else {
            return Ok(());
        };
        let (Some(Some(source_shape)), Some(Some(target_shape))) = (self.shapes.get(source_location.0).copied(), self.shapes.get(target_location.0).copied()) else {
            return Ok(());
        };
        if let Some(angle) = Self::angle_toward(source_shape, target_shape) {
            self.angles.insert(source_location, angle);
        }
        if let Some(angle) = Self::angle_toward(target_shape, source_shape) {
            self.angles.insert(target_location, angle);
        }
        Ok(())
    }

    fn publish_handle_one(&mut self, nodes: &[Value]) -> Result<bool, Fault> {
        let Some(((node_index, handle_index), angle)) = self.angles.pop_first() else {
            self.stage = Puzzle2dRedrawStage::Complete;
            return Ok(true);
        };
        let object = nodes.get(node_index).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-redraw-node-owner-lost"))?;
        let node_id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-redraw-node-id-missing"))?;
        let handle = object.get("handles").and_then(Value::as_array).and_then(|handles| handles.get(handle_index)).ok_or_else(|| Fault::from("puzzle2d-redraw-handle-owner-lost"))?;
        let original = <crate::standards::v1::subsets::any::schema::Puzzle2dHandle as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(handle)).map_err(|_| Fault::from("puzzle2d-redraw-handle-malformed"))?;
        let mut next = original.clone();
        next.angle = angle;
        if next == original {
            return Ok(false);
        }
        self.admit_bytes(node_id.len().saturating_add(next.id.len()))?;
        self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_node_handle(node_id.into(), next.id.clone(), next));
        Ok(false)
    }

    fn pop_one(&mut self) -> bool {
        if self.mutations.pop().is_some() {
            return true;
        }
        if self.angles.pop_first().is_some() {
            return true;
        }
        if self.shapes.pop().is_some() {
            return true;
        }
        if let Some(key) = self.handle_locations.keys().next().cloned() {
            self.handle_locations.remove(&key);
            return true;
        }
        false
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dRedrawHandlesWork {
    fn tool_id(&self) -> &'static str {
        "redrawHandles"
    }

    /// 📐️ One index chunk per [`PUZZLE2D_REDRAW_UNITS_PER_STEP`] node-or-handle records, one chunk
    /// per that many edges, one chunk per that many published handles, plus one stage handover each.
    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        if command.action_id() != "redrawHandles" || snapshot.value().get("schema").and_then(Value::as_str) != Some(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA) {
            return None;
        }
        let nodes = snapshot.value().get("nodes")?.as_array()?;
        let edges = snapshot.value().get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        let handles = nodes.iter().filter_map(|node| node.get("handles")).filter_map(Value::as_array).map(Vec::len).sum::<usize>();
        if nodes.len() > PUZZLE2D_REDRAW_MAX_NODES || edges > PUZZLE2D_REDRAW_MAX_EDGES || handles > PUZZLE2D_REDRAW_MAX_HANDLES {
            return None;
        }
        let items = nodes
            .len()
            .checked_add(handles)?
            .div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP)
            .checked_add(edges.div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP))?
            .checked_add(handles.div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP))?
            .checked_add(PUZZLE2D_REDRAW_STAGE_STEPS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let nodes = snapshot.value().get("nodes").and_then(Value::as_array).ok_or_else(|| Fault::from("puzzle2d-redraw-nodes-missing"))?;
        match self.stage {
            Puzzle2dRedrawStage::Nodes | Puzzle2dRedrawStage::Handles => {
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    match self.stage {
                        Puzzle2dRedrawStage::Nodes => self.index_node_one(nodes)?,
                        Puzzle2dRedrawStage::Handles => self.index_handle_one(nodes)?,
                        _ => break,
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-redraw-node", "Indexing board node", "Board-Knoten wird indiziert"))
            }
            Puzzle2dRedrawStage::Edges => {
                let edges = snapshot.value().get("edges").and_then(Value::as_array);
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    if self.stage != Puzzle2dRedrawStage::Edges {
                        break;
                    }
                    self.resolve_edge_one(edges)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-redraw-edge", "Resolving handle angle", "Anschlusswinkel wird bestimmt"))
            }
            Puzzle2dRedrawStage::Emit => {
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    if self.publish_handle_one(nodes)? {
                        let mutations = std::mem::take(&mut self.mutations);
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }));
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-redraw-emit", "Publishing handle angle", "Anschlusswinkel wird veröffentlicht"))
            }
            Puzzle2dRedrawStage::Complete => Err(Fault::from("puzzle2d-redraw-complete-repolled")),
            Puzzle2dRedrawStage::Closing => Err(Fault::from("puzzle2d-redraw-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dRedrawStage::Closing;
        self.close_owners.stage(Puzzle2dRedrawHandlesWorkOwners { shapes: std::mem::take(&mut self.shapes), handle_locations: std::mem::take(&mut self.handle_locations), angles: std::mem::take(&mut self.angles), mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle2dRedrawStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧵️ReservedJobs
use semio_framework_job::{InteractiveJob, StepContext};
use semio_framework_plugin::app::ArtifactToolCompletionRejection;
use semio_framework_plugin::{ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, MediaPayload};

/// 🔢️ Fixed per-collection descriptor budget for one `kit:in` fragment — an oversized collection is
/// refused outright rather than silently truncated.
const PUZZLE2D_IMPORT_SEMANTIC_ITEMS: usize = 64;
const PUZZLE2D_IMPORT_DECODED_ITEMS: usize = 4_096;
/// 🧬️ At most one `connect-kind-compatibility` per admitted relation row, plus the single
/// `replace-kind-catalogs` that carries the merged bundle.
const PUZZLE2D_IMPORT_MUTATION_ITEMS: usize = PUZZLE2D_IMPORT_SEMANTIC_ITEMS + 1;
const PUZZLE2D_IMPORT_TOOL_ID: &str = "import-media";
const PUZZLE2D_IMPORT_PORT: &str = "kit:in";
/// 🗂️ The exact root keys a `kit.catalog` fragment may carry — the shape
/// `Block2dPlayApp::export_media("catalog:out")` produces via
/// `crate::standards::v1::subsets::any::schema::inferences::puzzle2d_manifest_fragment`, which is puzzle2d's
/// own manifest vocabulary (`s/plugin/puzzle/app/2d/manifest/🔣️.json`), not block3d's.
const PUZZLE2D_IMPORT_ROOT_KEYS: &[&str] = &["schema", "id", "name", "axes", "portKinds", "wireKinds", "edgeKinds", "nodeKinds", "kindCompatibility"];
const PUZZLE2D_IMPORT_COLLECTIONS: &[&str] = &["portKinds", "wireKinds", "edgeKinds", "nodeKinds", "kindCompatibility"];

fn puzzle2d_job_fault(detail: &str) -> crate::puzzle_job::JobTurn {
    let bytes = detail.as_bytes();
    crate::puzzle_job::JobTurn::Fault(bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec())
}

fn puzzle2d_import_checkpoint(stage: u8, cursor: usize, decoded_items: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    let mut state = [0u8; 25];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&(decoded_items as u64).to_le_bytes());
    state[17..25].copy_from_slice(&progress.to_le_bytes());
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: state.to_vec() }
}

fn puzzle2d_import_text(row: &Value, key: &str) -> semio_framework_value::paged::PagedUtf8<{ usize::MAX }> {
    row.get(key).and_then(Value::as_str).unwrap_or_default().into()
}

fn puzzle2d_import_identity(row: &Value) -> Option<&str> {
    row.get("id").and_then(Value::as_str).filter(|id| !id.trim().is_empty())
}

fn puzzle2d_import_presentation<'a>(row: &'a Value, key: &str) -> Option<&'a Value> {
    row.get("presentation").and_then(|presentation| presentation.get(key))
}

fn puzzle2d_import_presentation_text(row: &Value, key: &str) -> semio_framework_value::paged::PagedUtf8<{ usize::MAX }> {
    puzzle2d_import_presentation(row, key).and_then(Value::as_str).unwrap_or_default().into()
}

/// 🏷️ Manifest rows carry `name` and no `label`; the catalog row wants both, so an absent label
/// mirrors the name rather than inventing a third spelling.
fn puzzle2d_import_label(row: &Value, name: &str) -> semio_framework_value::paged::PagedUtf8<{ usize::MAX }> {
    row.get("label").and_then(Value::as_str).filter(|label| !label.trim().is_empty()).unwrap_or(name).into()
}

fn puzzle2d_import_handle_template(node_kind_id: &str, index: usize, template: &Value) -> Option<crate::standards::v1::subsets::any::schema::Puzzle2dHandleTemplate> {
    let handle_kind = template.get("handleKind").and_then(Value::as_str).filter(|kind| !kind.trim().is_empty())?;
    let name = template.get("name").and_then(Value::as_str).unwrap_or_default();
    Some(crate::standards::v1::subsets::any::schema::Puzzle2dHandleTemplate {
        id: puzzle2d_import_identity(template).map_or_else(|| format!("{node_kind_id}-h{index}"), str::to_string).into(),
        label: puzzle2d_import_label(template, name),
        name: name.into(),
        description: puzzle2d_import_text(template, "description"),
        icon: puzzle2d_import_text(template, "icon"),
        handle_kind: Some(handle_kind.into()),
        angle: template.get("angle").and_then(Value::as_f64).filter(|angle| angle.is_finite()).unwrap_or_default(),
        t: template.get("t").and_then(Value::as_f64).filter(|t| t.is_finite()),
        mandatory: template.get("mandatory").and_then(Value::as_bool),
        radius: template.get("radius").and_then(Value::as_f64).filter(|radius| radius.is_finite()),
    })
}

fn puzzle2d_import_node_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::Puzzle2dCatalogNodeKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id);
    let handles = puzzle2d_import_presentation(row, "handles").and_then(Value::as_array).map_or_else(semio_framework_value::list::PagedList::new, |templates| templates.iter().enumerate().filter_map(|(index, template)| puzzle2d_import_handle_template(id, index, template)).collect());
    Some(crate::standards::v1::subsets::any::schema::Puzzle2dCatalogNodeKind {
        id: id.into(),
        label: puzzle2d_import_label(row, name),
        name: name.into(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        image: puzzle2d_import_presentation_text(row, "image"),
        unit: puzzle2d_import_text(row, "unit"),
        is_abstract: row.get("abstract").and_then(Value::as_bool).unwrap_or_default(),
        base_kinds: Default::default(),
        representations: Default::default(),
        handles,
        attributes: Default::default(),
        authors: Default::default(),
    })
}

fn puzzle2d_import_handle_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::Puzzle2dCatalogHandleKind> {
    let id = puzzle2d_import_identity(row)?;
    Some(crate::standards::v1::subsets::any::schema::Puzzle2dCatalogHandleKind {
        id: id.into(),
        code: None,
        label: Some(puzzle2d_import_label(row, row.get("name").and_then(Value::as_str).unwrap_or(id))),
        order: None,
        compatible_with: Default::default(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
        default_wire_kind: puzzle2d_import_presentation_text(row, "defaultWireKind"),
    })
}

fn puzzle2d_import_edge_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::Puzzle2dCatalogEdgeKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id);
    Some(crate::standards::v1::subsets::any::schema::Puzzle2dCatalogEdgeKind {
        id: id.into(),
        label: puzzle2d_import_label(row, name),
        name: name.into(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
    })
}

fn puzzle2d_import_wire_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::Puzzle2dCatalogWireKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id);
    Some(crate::standards::v1::subsets::any::schema::Puzzle2dCatalogWireKind {
        id: id.into(),
        label: puzzle2d_import_label(row, name),
        name: name.into(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
        default_edge_kind: puzzle2d_import_presentation_text(row, "defaultEdgeKind"),
    })
}

/// 🗂️ Id-keyed upsert — deterministic and order-independent, so a `multiplicity: Many` port that fans
/// in several catalog producers converges on the same bundle whatever order they arrive in. Reports
/// whether the bundle actually changed, so an idempotent re-delivery emits no operation at all.
fn puzzle2d_upsert_catalog_row<T: PartialEq>(rows: &mut semio_framework_value::list::PagedList<T, { usize::MAX }>, incoming: T, matches: impl Fn(&T) -> bool) -> bool {
    match rows.iter().position(matches) {
        Some(index) if rows[index] == incoming => false,
        Some(index) => {
            rows[index] = incoming;
            true
        }
        None => {
            rows.push(incoming);
            true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dImportStage {
    Decode,
    HandleKinds,
    NodeKinds,
    EdgeKinds,
    WireKinds,
    Compatibility,
    CatalogMutation,
    Complete,
}

/// 🎞️ `kit:in`'s concrete resumable importer: it normalizes one `kit.catalog` manifest fragment
/// (`portKinds`/`wireKinds`/`edgeKinds`/`nodeKinds`/`kindCompatibility`) into puzzle2d's own typed
/// `meta.kindCatalogs` vocabulary (`handles`/`wires`/`edges`/`nodes`) and its `meta.kindCompatibility`
/// table, one row per bounded step, and publishes the result as real undoable `Puzzle2dMutation`s —
/// never a direct state write. The producer this pairs with is `Block2dPlayApp`'s `"catalog:out"`
/// port, whose payload IS this exact 2d manifest shape (see
/// `🧱️block/🗿️artifacts/◻️2d/…/💡️inferences/🦀️.rs`'s `puzzle2d_manifest_fragment`).
struct Puzzle2dImportJob {
    port: String,
    media_json: Option<String>,
    snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    fragment: Option<Value>,
    catalogs: crate::standards::v1::subsets::any::schema::Puzzle2dKindCatalogs,
    compatibility: Vec<crate::standards::v1::subsets::any::schema::Puzzle2dKindCompatibility>,
    mutations: Vec<Puzzle2dMutation>,
    stage: Puzzle2dImportStage,
    cursor: usize,
    decoded_items: usize,
    progress: u64,
    catalog_changed: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    owners: crate::puzzle_job::WorkClosing<Puzzle2dImportOwners>,
}

/// ♻️ The owners one `Puzzle2dImportJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dImportOwners {
    port: String,
    media_json: Option<String>,
    snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    fragment: Option<Value>,
    catalogs: crate::standards::v1::subsets::any::schema::Puzzle2dKindCatalogs,
    compatibility: Vec<crate::standards::v1::subsets::any::schema::Puzzle2dKindCompatibility>,
    mutations: Vec<Puzzle2dMutation>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
}

impl Puzzle2dImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            MediaPayload::Structured { json, .. } => Some(json),
            MediaPayload::Intrinsic { value, .. } => Some(semio_framework_pack_json::to_json_string(&value)),
            MediaPayload::Binary { .. } => None,
        };
        Self {
            port,
            media_json,
            snapshot: Some(request.snapshot),
            fragment: None,
            catalogs: crate::standards::v1::subsets::any::schema::Puzzle2dKindCatalogs::default(),
            compatibility: Vec::new(),
            mutations: Vec::new(),
            stage: Puzzle2dImportStage::Decode,
            cursor: 0,
            decoded_items: 0,
            progress: 0,
            catalog_changed: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle2d_import_checkpoint(self.stage as u8, self.cursor, self.decoded_items, self.progress)
    }

    fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|fragment| fragment.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn meta(&self, key: &str) -> Option<&Value> {
        self.snapshot.as_ref().and_then(|snapshot| snapshot.value().get("meta")).and_then(|meta| meta.get(key)).filter(|value| !value.is_null())
    }

    fn push_mutation(&mut self, mutation: Puzzle2dMutation) -> Result<(), &'static str> {
        if self.mutations.len() >= PUZZLE2D_IMPORT_MUTATION_ITEMS {
            return Err("puzzle2d kit:in mutation limit exceeded");
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn decode(&mut self) -> Option<crate::puzzle_job::JobTurn> {
        if self.port != PUZZLE2D_IMPORT_PORT {
            return Some(puzzle2d_job_fault("puzzle2d import only implements kit:in"));
        }
        let Some(media_json) = self.media_json.as_ref() else {
            return Some(puzzle2d_job_fault("puzzle2d kit:in requires a structured payload"));
        };
        let Ok(fragment) = semio_framework_pack_json::parse(media_json,semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return Some(puzzle2d_job_fault("puzzle2d kit:in payload is not valid json"));
        };
        if fragment.as_object().is_none() {
            return Some(puzzle2d_job_fault("puzzle2d kit:in root must be an object"));
        }
        if fragment.as_object().is_some_and(|root| root.iter().any(|(key, _)| !PUZZLE2D_IMPORT_ROOT_KEYS.contains(&key))) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in root contains an unknown field"));
        }
        if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in schema must be manifest when present"));
        }
        if PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).is_some_and(|value| value.as_array().is_none())) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in collection must be an array"));
        }
        if PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).and_then(Value::as_array).is_some_and(|rows| rows.len() > PUZZLE2D_IMPORT_SEMANTIC_ITEMS)) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in collection exceeds its fixed descriptor cap"));
        }
        let existing_catalogs = self.meta("kindCatalogs").cloned();
        let existing_compatibility = self.meta("kindCompatibility").and_then(|value| value.as_array().cloned()).unwrap_or_default();
        let fragment_items = PUZZLE2D_IMPORT_COLLECTIONS.iter().try_fold(0usize, |total, key| total.checked_add(fragment.get(*key).and_then(Value::as_array).map_or(0, Vec::len)));
        let catalog_items = existing_catalogs.as_ref().map(|catalogs| ["nodes", "handles", "edges", "wires"].into_iter().map(|slice| catalogs.get(slice).and_then(Value::as_array).map_or(0, Vec::len)).sum::<usize>()).unwrap_or_default();
        self.decoded_items = match fragment_items.and_then(|items| items.checked_add(catalog_items)).and_then(|items| items.checked_add(existing_compatibility.len())) {
            Some(items) if items <= PUZZLE2D_IMPORT_DECODED_ITEMS => items,
            _ => return Some(puzzle2d_job_fault("puzzle2d kit:in decoded item limit exceeded")),
        };
        self.catalogs = match existing_catalogs.as_ref() {
            Some(value) => match <crate::standards::v1::subsets::any::schema::Puzzle2dKindCatalogs as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(value)) {
                Ok(catalogs) => catalogs,
                Err(_) => return Some(puzzle2d_job_fault("puzzle2d kit:in cannot read the document's own kind catalogs")),
            },
            None => crate::standards::v1::subsets::any::schema::Puzzle2dKindCatalogs::default(),
        };
        for row in &existing_compatibility {
            match <crate::standards::v1::subsets::any::schema::Puzzle2dKindCompatibility as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(row)) {
                Ok(parsed) => self.compatibility.push(parsed),
                Err(_) => return Some(puzzle2d_job_fault("puzzle2d kit:in cannot read the document's own kind relations")),
            }
        }
        self.catalog_changed = existing_catalogs.is_none() && PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).and_then(Value::as_array).is_some_and(|rows| !rows.is_empty()));
        self.fragment = Some(fragment);
        None
    }
}

impl Puzzle2dImportJob {
    fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle2d_job_fault("puzzle2d import completion remains rejected");
        }
        match self.stage {
            Puzzle2dImportStage::Decode => {
                cx.set_stage("puzzle2d-import-decode");
                if let Some(outcome) = self.decode() {
                    return outcome;
                }
                self.stage = Puzzle2dImportStage::HandleKinds;
                self.cursor = 0;
            }
            Puzzle2dImportStage::HandleKinds => {
                cx.set_stage("puzzle2d-import-handle-kind");
                let admitted = self.rows("portKinds").get(self.cursor).map(puzzle2d_import_handle_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in port kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.handles, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::NodeKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::NodeKinds => {
                cx.set_stage("puzzle2d-import-node-kind");
                let admitted = self.rows("nodeKinds").get(self.cursor).map(puzzle2d_import_node_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in node kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.nodes, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::EdgeKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::EdgeKinds => {
                cx.set_stage("puzzle2d-import-edge-kind");
                let admitted = self.rows("edgeKinds").get(self.cursor).map(puzzle2d_import_edge_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in edge kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.edges, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::WireKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::WireKinds => {
                cx.set_stage("puzzle2d-import-wire-kind");
                let admitted = self.rows("wireKinds").get(self.cursor).map(puzzle2d_import_wire_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in wire kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.wires, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::Compatibility;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::Compatibility => {
                cx.set_stage("puzzle2d-import-kind-relation");
                let admitted = self.rows("kindCompatibility").get(self.cursor).cloned();
                let Some(row) = admitted else {
                    self.stage = Puzzle2dImportStage::CatalogMutation;
                    self.cursor = 0;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return self.checkpoint();
                };
                let Ok(parsed) = <crate::standards::v1::subsets::any::schema::Puzzle2dKindCompatibility as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&row)) else {
                    return puzzle2d_job_fault("puzzle2d kit:in kind relation is malformed");
                };
                let existing = self.compatibility.iter().position(|entry| entry.source == parsed.source && entry.target == parsed.target);
                if !existing.is_some_and(|index| self.compatibility[index] == parsed) {
                    match existing {
                        Some(index) => self.compatibility[index] = parsed.clone(),
                        None => self.compatibility.push(parsed.clone()),
                    }
                    if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source, parsed.target, parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                        return puzzle2d_job_fault(error);
                    }
                }
                self.cursor += 1;
            }
            Puzzle2dImportStage::CatalogMutation => {
                cx.set_stage("puzzle2d-import-kind-catalogs");
                if self.catalog_changed {
                    let mutation = crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(std::mem::take(&mut self.catalogs)));
                    if let Err(error) = self.push_mutation(mutation) {
                        return puzzle2d_job_fault(error);
                    }
                    self.catalog_changed = false;
                }
                self.stage = Puzzle2dImportStage::Complete;
            }
            Puzzle2dImportStage::Complete => {
                cx.set_stage("puzzle2d-import-publish");
                if !self.completed {
                    let mutations = std::mem::take(&mut self.mutations);
                    let Some(completion) = self.completion.as_ref() else {
                        return puzzle2d_job_fault("puzzle2d import lost its completion authority");
                    };
                    if !completion.has_mounted_consumer() {
                        return puzzle2d_job_fault("puzzle2d import completion consumer is absent");
                    }
                    if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle2d_job_fault(&message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}

impl InteractiveJob for Puzzle2dImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle2dImportOwners {
            port: std::mem::take(&mut self.port),
            media_json: self.media_json.take(),
            snapshot: self.snapshot.take(),
            fragment: self.fragment.take(),
            catalogs: std::mem::take(&mut self.catalogs),
            compatibility: std::mem::take(&mut self.compatibility),
            mutations: std::mem::take(&mut self.mutations),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle2dImportJob {}

/// 📋️ One-step reserved copy/cut/paste job — the framework route is an empty stub unless the app owns
/// this producer, so puzzle2d owning it is what makes `mod+c`/`mod+x`/`mod+v` and the context-menu
/// clipboard rows do anything at all. Every arm answers in one step: a cut is copy-plus-delete as ONE
/// artifact edit, and a paste is one insert whose clones the app re-selects.
struct Puzzle2dClipboardJob {
    tool_id: String,
    labels: Option<&'static crate::editor::puzzle2d::terminology::Puzzle2dLabels>,
    snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    raw_wire: Vec<u8>,
    args: Option<semio_framework_value::DslValue>,
    interaction: Option<semio_framework::InteractionState>,
    hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    owners: crate::puzzle_job::WorkClosing<Puzzle2dClipboardOwners>,
}

/// ♻️ The owners one `Puzzle2dClipboardJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dClipboardOwners {
    args: Option<semio_framework_value::DslValue>,
    interaction: Option<semio_framework::InteractionState>,
    hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    tool_id: String,
    snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    raw_wire: Vec<u8>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
}

impl Puzzle2dClipboardJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>) -> Self {
        let ArtifactReservedToolInput::Action { args, interaction, hover } = request.input else { unreachable!("clipboard routes carry action input") };
        let labels = request.ui_axes.map(|(locale, terminology)| puzzle2d_labels(&semio_framework_plugin::ViewModel::new(locale, terminology)));
        Self { tool_id: request.tool_id, labels, snapshot: Some(request.snapshot), raw_wire: request.raw_wire, args, interaction: Some(interaction), hover: Some(hover), completion: Some(request.completion), rejection: None, outbox: Default::default(), closing: false, owners: Default::default() }
    }

    fn emit(&mut self) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, NoDraftMutation>, Fault> {
        let (Some(interaction), Some(hover)) = (self.interaction.as_ref(), self.hover.as_ref()) else {
            return Ok(Emit::default());
        };
        let args = self.args.as_ref();
        let Some(owned) = self.snapshot.clone() else { return Ok(Emit::default()) };
        let snapshot = owned.value();
        let marks = Puzzle2dInteractionSnapshot::from_state(&interaction, &hover);
        let selected = puzzle2d_selected_node_ids(snapshot, marks.selected_ids());
        Ok(match self.tool_id.as_str() {
            // 🧾️ An unresolvable selection answers with ZERO effects — the same shape as "the hotkey
            // never reached the guest", exactly as puzzle3d's clipboard route answers it.
            "copy" => match puzzle2d_copy_fragment_from(snapshot, &selected) {
                Ok(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }], ui_scope: UiDirtyScope::None, ..Default::default() },
                Err(_) => Emit::default(),
            },
            "cut" => {
                let Ok(fragment) = puzzle2d_copy_fragment_from(snapshot, &selected) else { return Ok(Emit::default()) };
                if puzzle2d_selection_is_locked(snapshot, &selected) {
                    let labels = self.labels.ok_or_else(|| Fault::from("puzzle2d clipboard notification requires the caller's explicit presentation axes"))?;
                    return Ok(Emit { effects: vec![Effect::Notify { message: labels.cut_locked.as_str().to_string() }], ui_scope: UiDirtyScope::None, ..Default::default() });
                }
                let mutations = puzzle2d_cut_operations_from(&owned, &selected);
                let clear = puzzle2d_clear_selection_write(snapshot, &selected).into_iter().collect();
                Emit { artifact_mutations: mutations, effects: vec![Effect::ClipboardWrite { fragment }], interaction_writes: clear, ..Default::default() }
            }
            "paste" => {
                let Some(fragment) = args.as_ref().and_then(|value| value.get("fragment")).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()) else {
                    return Ok(Emit::default());
                };
                let placement = PastePlacement::default();
                match puzzle2d_paste_operations_on(&owned, &fragment, &placement) {
                    Ok((mutations, pasted)) => Emit { artifact_mutations: mutations, interaction_writes: vec![puzzle2d_selection_write(snapshot, &pasted)], ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            _ => Emit::default(),
        })
    }
}

impl InteractiveJob for Puzzle2dClipboardJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx);
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle2dClipboardOwners { args: self.args.take(), interaction: self.interaction.take(), hover: self.hover.take(), tool_id: std::mem::take(&mut self.tool_id), snapshot: self.snapshot.take(), raw_wire: std::mem::take(&mut self.raw_wire), completion: self.completion.take(), rejection: self.rejection.take() });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl Puzzle2dClipboardJob {
    fn turn(&mut self, cx: &StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if !self.raw_wire.is_empty() {
            return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw_wire));
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return crate::puzzle_job::JobTurn::Fault(b"puzzle2d-clipboard-completion-owner".to_vec()) };
        if let Err(rejected) = completion.complete(emit, EphemeralEmit::default()) {
            self.rejection = Some(rejected);
            return crate::puzzle_job::JobTurn::Fault(b"puzzle2d-clipboard-completion-rejected".to_vec());
        }
        crate::puzzle_job::JobTurn::Complete
    }
}

impl ArtifactReservedJob for Puzzle2dClipboardJob {}


/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// [`PUZZLE2D_RETAINED_TOOL_IDS`] entry, all joined to the single concrete
/// [`Puzzle2dRetainedCommandJobFactory`].
struct Puzzle2dRetainedCommandProofs;

impl Puzzle2dRetainedCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle2d@1/*#editor",
        artifact_schema: "board.ports.directed.v1",
        factory: "Puzzle2dRetainedCommandJobFactory",
        factory_type: Puzzle2dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(PUZZLE2D_COMMAND_RAW_BYTES, PUZZLE2D_COMMAND_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        tools: [
            "setActiveExample",
            "forceLayout",
            "addNode",
            "applyBoardEvents",
            "reorganize",
            "closeHandleSuggestions",
            "acceptSuggestion",
            "cycleBrushCandidate",
            "cycleBrushCandidateBack",
            "openHandleSuggestions",
            "hoverSuggestion",
            "targetBrushSuggestions",
            "deleteSelection",
            "duplicateSelection",
            "engagementAbort",
            "engagementControlSelect",
            "engagementInput",
            "engagementRepeatLast",
            "engagementSubmit",
            "focusSelection",
            "lodScaleJson",
            "openAddNodeDialog",
            "patchInspectorNodes",
            "redrawHandles",
            "selectSameKind",
            "setBrushKindWeights",
            "setBrushNodeSize",
            "setBrushPlacementContactTolerance",
            "setBrushPlacementOverlapBudget",
            "setCamera",
            "setFillCount",
            "setGridFactor",
            "setGridSnapEnabled",
            "setGridVisible",
            "setLodModeForPane",
            "setSelectableKind",
            "setSelectionFlag",
            "setSelectionHidden",
            "setSelectionLocked",
            "setSuggestionOffset",
            "setTransformGumballFlag",
            "addTargetRegion",
            "deleteTargetRegion",
            "relocateTargetRegion",
            "setTargetRegionFlag",
            "setTargetRegionHidden",
            "setTargetRegionLocked",
            "setAreaBrushSize",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "exportSnapshot",
            "importSnapshot",
            "openImportSnapshot",
            "createEdge",
            "deleteEdge",
            "proximityConnect",
            "setProximityRadius",
        ]
    }
}

/// 📜️ The two framework-injected host-configuration verbs. Deliberately GENERIC proofs (no
/// `factory_type`): they are not app-owned retained tools — [`Puzzle2dRetainedCommandJobFactory`]
/// never claims them — they only need the wire admission and output budget `dispatch_action`'s
/// host-configuration branch asks for before it applies `host_configuration_mutation`.
struct Puzzle2dHostConfigurationProofs;

impl Puzzle2dHostConfigurationProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle2d@1/*#editor",
        artifact_schema: "board.ports.directed.v1",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 8, 1, 8_192, 7_500, 1, 1),
        tools: ["setActiveTool", "setActiveUtility"]
    }
}
//#endregion 📜️ToolProofs

impl ArtifactEditor for Puzzle2dPlayApp {
    const DIALECT: Dialect = crate::standards::v1::subsets::any::schema::PUZZLE2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_BOARD_SNAPSHOT_SCHEMA;
    type Snapshot = Puzzle2dPlaySnapshot;
    type Mutation = Puzzle2dMutation;
    type Config = Puzzle2dConfig;
    type ConfigMutation = Puzzle2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle2dPresence;
    type PresenceMutation = Puzzle2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle2dCommand;

    /// 🐁️ The six framework interaction verbs answered out of this app's OWN narrow scopes instead of
    /// the framework's blanket [`UiDirtyScope::Full`]. A hover fires on every pointermove, so it
    /// repaints the three canvas panes alone — never the panels, the engagement bar, the measures or
    /// the labels; a pick additionally repaints the outliner/inspector rows that highlight it. A verb
    /// that touched a domain this app does not declare answers `None` and keeps the framework's widest scope.
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        if domains.is_empty() || domains.iter().any(|domain| *domain != PUZZLE2D_INTERACTION_DOMAIN) {
            return None;
        }
        Some(match verb {
            InteractionVerb::Hover => puzzle2d_window_only_scope(),
            InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll => puzzle2d_select_scope(),
            InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity => puzzle2d_window_and_engagements_scope(),
        })
    }

    /// 🗺️ The `vortex` domain's whole pickable universe, in board order: every node with its handles under it, every
    /// edge and every target region (a region answers to the node granularity, as `puzzle2d_selection_targets`
    /// classifies it) — what `selectAll` enumerates and what keeps a selection inside the document. A host fact never
    /// reaches this editor: the window's gesture slot ends an open gesture on a blur, a lost capture, a utility switch,
    /// a closing window, a frozen document and a moved base.
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
        let document = doc.snapshot.typed();
        let mut ordered = Vec::new();
        for node in &document.nodes {
            ordered.push(semio_framework_plugin::TopologyNode { id: node.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None });
            ordered.extend(node.handles.iter().map(|handle| semio_framework_plugin::TopologyNode { id: handle.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_HANDLE.into(), parent: Some(node.id.to_string_owner()) }));
        }
        ordered.extend(document.edges.iter().map(|edge| semio_framework_plugin::TopologyNode { id: edge.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_EDGE.into(), parent: None }));
        ordered.extend(document.target_regions.iter().map(|region| semio_framework_plugin::TopologyNode { id: region.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None }));
        Ok(semio_framework_plugin::InteractionTopology { domains: std::collections::BTreeMap::from([(PUZZLE2D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered })]) })
    }

    /// 🪧️ A history-edit reference chip names its board entity as the outliner does ([`puzzle2d_entity_label`]).
    fn entity_label(snapshot: &Puzzle2dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle2d_entity_label(snapshot.value(), kinds, id)
    }



    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle2dConfigStorePreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle2dArtifactStorePreparationFactory))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }


    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Puzzle 2d presence is three inline camera scalars, so the default root is its exact empty terminal.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default Puzzle2d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window::register_config(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window::register_transient(registry)
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle2d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle2dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::puzzle2d::config::schema::app_schema_descriptor())
    }

    /// 🚀️ The empty snapshot, nothing else. The example documents are `LazyLock`s the example load forces
    /// when it is actually switched to: warming them here parsed the Concrete Forest and Nakagin documents on
    /// every construction of this app — every mount, and every `codec` call of a hub creating or validating a
    /// document, where the interpreter paid for two documents the snapshot never reads.
    fn initial_snapshot() -> Puzzle2dPlaySnapshot {
        Puzzle2dPlaySnapshot::new(crate::standards::v1::subsets::any::schema::empty_puzzle2d_snapshot())
    }

    /// 🏷️ Maps each `Puzzle2dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle2dCommand) -> &'static str {
        command.action_id()
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle2d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle2d_fault_notices()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        if let Some(flag) = puzzle2d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle2d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        Puzzle2dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 2D action '{action}'")))
    }

    /// 🎬️ Routes the example load through its own `Work` before document materialization; every other
    /// command runs the one shared [`puzzle2d_dispatch_emit`] pipeline the retained generic reduce runs.
    fn handle(
        command: &Puzzle2dCommand,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, Fault> {
        let config = cfg.snapshot;
        let (action, _args) = (command.action_id(), command.args());
        if action == "setActiveExample" {
            return puzzle2d_active_example_emit(command, doc.snapshot, config);
        }
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_transient = Puzzle2dWindowTransient::default();
        let window_kind = view_state.and_then(window::kind_for_view).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        let base_revision = doc.operation_optional().map(|operation| puzzle2d_revision_hex(&operation.canonical_base_revision)).unwrap_or_default();
        puzzle2d_dispatch_emit(command, doc.snapshot, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), authoring_seed, &base_revision, &semio_framework_plugin::app::GestureSlot::detached(), doc.operation_optional().cloned())
            .map(|(emit, _)| emit)
    }

    /// 🧭️ The two framework-injected host-configuration verbs, resolved to no Config mutation of this
    /// app's own. The shell owns their session state and forwards the resolved value here so the app
    /// may clear its scratch; puzzle2d's active utility IS host view state
    /// (`puzzle2d_active_utility` reads `ViewModel::active_utility_id`) and its brush slot lives in
    /// the WINDOW TRANSIENT lane, which this Config-typed seam cannot address — the transient is
    /// rebuilt from the next dispatch anyway. Without this hook (and the proofs below) both verbs fell
    /// through to `admit_command_json` and failed closed with `interactive-job.missing-factory`, so
    /// the Fill tool tab could not even be selected.
    fn host_configuration_mutation(_action: &str, _args: Option<&semio_framework_value::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok(None)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle2dRetainedCommandProofs::bounded_first_step_tool_proofs();
        proofs.extend(Puzzle2dHostConfigurationProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PUZZLE2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "puzzle2d-command-tool-mismatch"));
        }
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Self>>> = match request.command.action_id() {
            "setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new("setActiveExample", puzzle2d_active_example_reduce, puzzle2d_active_example_extent)),
            "forceLayout" => Box::new(Puzzle2dForceLayoutWork::default()),
            "reorganize" => Box::new(Puzzle2dForceLayoutWork::new("reorganize")),
            "addNode" => Box::new(Puzzle2dWindowCommandWork::new("addNode", puzzle2d_add_node_extent, request.authoring_seed.clone(), puzzle2d_revision_hex(&request.canonical_base_revision))),
            "exportSnapshot" => Box::new(Puzzle2dExportWork::default()),
            "applyBoardEvents" => Box::new(Puzzle2dWindowCommandWork::new("applyBoardEvents", puzzle2d_board_events_extent, request.authoring_seed.clone(), puzzle2d_revision_hex(&request.canonical_base_revision))),
            generic if PUZZLE2D_GENERIC_TOOL_IDS.contains(&generic) => Box::new(Puzzle2dWindowCommandWork::new(generic, puzzle2d_generic_extent, request.authoring_seed.clone(), puzzle2d_revision_hex(&request.canonical_base_revision))),
            host_only if PUZZLE2D_HOST_ONLY_TOOL_IDS.contains(&host_only) => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(host_only)),
            "redrawHandles" => Box::new(Puzzle2dRedrawHandlesWork::default()),
            _ => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), "puzzle2d-command-tool-unmapped")),
        };
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            Puzzle2dCommand::action_id,
            PUZZLE2D_COMMAND_RAW_BYTES,
            crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// ⏯️ Builds the fill tool run's jobs (`ToolRunDefinition.runJob` / `.revalidateJob`): the run job over the run's
    /// base with the config's requested count, resuming from the ledger's checkpoint and provisional ops, and the
    /// revalidate job over the committed head.
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        if request.tool_id != fill::TOOL_ID {
            return Ok(None);
        }
        Ok(Some(match request.purpose {
            semio_framework_plugin::ToolRunJobPurpose::Run => {
                Box::new(fill_run::Puzzle2dFillRunJob::new(request.identity, request.snapshot, crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_SUGGESTION_OFFSET, request.config.fill_count, request.checkpoint, request.provisional).map_err(Fault::from)?)
            }
            // 🚧️ The placement-tuning settings reach the run through `ToolRunSettingsReads` — the
            // framework rebuilds this job whenever either pointer changes, so the net slack a
            // revalidation tests with is always the one the settings panel currently shows.
            semio_framework_plugin::ToolRunJobPurpose::Revalidate => Box::new(fill_run::Puzzle2dFillRevalidateJob::new(
                request.identity,
                request.snapshot,
                request.provisional,
                request.checkpoint,
                request.config.contact_tolerance - request.config.brush_placement_overlap_budget,
            )),
        }))
    }

    /// 🔌️ Declares puzzle2d's typed media I/O surface — the implicit document ports plus `kit:in` and
    /// `design:out`. `kit:in`'s producer is `Block2dPlayApp`'s `"catalog:out"` port, whose `kit.catalog`
    /// payload is literally a puzzle2d manifest fragment (`portKinds`/`wireKinds`/`edgeKinds`/
    /// `nodeKinds`/`kindCompatibility` — `crate::standards::v1::subsets::any::schema::inferences::puzzle2d_manifest_fragment`),
    /// so this is a 2d↔2d vocabulary, not block3d's object/vortex-kind one; `Puzzle2dImportJob` does the
    /// normalization one bounded row per step.
    fn io() -> Option<AppIo> {
        let io = ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.2d",
            MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
            ArtifactPresentation { id: "2d.puzzle".into(), name: "2D Puzzle".into(), dimension: "2d".into(), component_kind: "puzzle2d".into() },
        ));
        Some(::semio_framework_async::poll::resolve_ready(io.with_ports(vec![
            MediaPortSpec {
                id: PUZZLE2D_IMPORT_PORT.into(),
                label: "Kit Catalog".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                kind_id: Some("kit.catalog".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
            MediaPortSpec {
                id: "design:out".into(),
                label: "Puzzle Design".into(),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
                kind_id: Some("2d.puzzle".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ])))
    }

    /// 📋️ The `MediaType` a puzzle 2d copy writes and a paste accepts — without it the framework's
    /// injected `copy`/`cut`/`paste` actions silently no-op.
    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::TwoD, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let marks = Puzzle2dInteractionSnapshot::from_interaction(interaction);
        puzzle2d_copy_fragment_from(doc.snapshot.value(), &puzzle2d_selected_node_ids(doc.snapshot.value(), marks.selected_ids()))
    }

    fn cut_operations(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle2dMutation> {
        let marks = Puzzle2dInteractionSnapshot::from_interaction(interaction);
        let selected = puzzle2d_selected_node_ids(doc.snapshot.value(), marks.selected_ids());
        if puzzle2d_selection_is_locked(doc.snapshot.value(), &selected) {
            return Vec::new();
        }
        puzzle2d_cut_operations_from(doc.snapshot, &selected)
    }

    fn paste_operations(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle2dMutation>, ClipboardError> {
        puzzle2d_paste_operations_on(doc.snapshot, fragment, placement).map(|(mutations, _)| mutations)
    }

    /// 🎞️ The reserved routes puzzle2d owns: `import-media` (the framework registers no importer on an
    /// app's behalf, and every inbound media delivery goes through `dispatch_import_media` →
    /// `build_artifact_reserved_media_job`, never the unbounded one-shot `ArtifactApp::import_media`
    /// seam) and the three clipboard verbs, whose fragment vocabulary is this artifact's own
    /// `puzzle.2d.clipboard.v1` — see [`Puzzle2dClipboardJob`].
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") {
            return Ok(Some(ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request))));
        }
        if request.tool_id.as_str() != PUZZLE2D_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("puzzle2d import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("puzzle2d import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(Puzzle2dImportJob::new(request, port, media))))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &Puzzle2dWindowTransient::default(), Puzzle2dInteractionSnapshot::default())
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        Self::render_body(body_key, doc, cfg, view_state, &window::transient_from_view(transient), Puzzle2dInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let labels = puzzle2d_labels(view_state);
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        HashMap::from([(window_id.to_string(), edit::puzzle2d_engagement(&envelope, &puzzle_board_host(), window_kind, labels))])
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_transient = window::transient_from_view(transient);
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &window_transient, Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        HashMap::from([(window_id.to_string(), edit::puzzle2d_engagement(&envelope, &puzzle_board_host(), window_kind, puzzle2d_labels(view_state)))])
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = puzzle2d_labels(view_state);
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        let measures = match window_kind {
            crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID => detail::window_measures(&envelope, labels),
            crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID => selection::window_measures(&envelope, labels),
            _ => overview::window_measures(&envelope, labels),
        };
        HashMap::from([(window_id.to_string(), measures)])
    }

    fn tool_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        let labels = puzzle2d_labels(view_state);
        HashMap::from([(fill::TOOL_ID.to_string(), vec![fill::measures(&envelope, labels)])])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        _cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, view_state, &Puzzle2dInteractionSnapshot::default(), registry)
    }

    /// 🕹️ The context menu reads the AUTHORITATIVE framework-owned selection when the surface names
    /// none of its own — a right-click on an outliner row or an empty board still targets what the
    /// user selected; the surface's own hits/selection win when present, so a right-click on an
    /// unselected node still targets that node.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        _cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, view_state, &Puzzle2dInteractionSnapshot::from_interaction(interaction), registry)
    }
}

impl Puzzle2dPlayApp {
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle2dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let mut selected: Vec<String> = request.surface.as_ref().map(|surface| surface.selection.iter().flat_map(|g| g.ids.iter().cloned()).collect()).unwrap_or_default();
        if selected.is_empty() {
            selected = interaction.selected.clone();
        }
        ::semio_framework_async::poll::resolve_ready(puzzle2d_context_menu_items(registry, doc.snapshot.value(), &selected, view_state))
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/engagement-bound
/// vocabulary dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 📢️ The localized notice of every refusal code the puzzle2d guest names (design §20.12): a set-verb's missing flag value.
fn puzzle2d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 1]> = std::sync::LazyLock::new(|| {
        [
            ("puzzle2d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
        ]
    });
    &*NOTICES
}

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetRegionLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
fn puzzle2d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetRegionHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetRegionLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle2d_flag_value_action(id: &str, category: &str, identity: ActionArgDef, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    puzzle2d_internal_action(id, label, ActionKind::Mutation).with_category(category).with_args([identity, ActionArgDef::toggle(flag, value).required()])
}

/// 🎭️✏️ `.example_source(...)` (×2, concrete-forest + nakagin) and `.workflow("puzzle2d", …)` were
/// dropped, not ported — `EditorBuilder` has neither method (contract §2.4: `.editor::<E>(def:
/// AppDefinition)` only takes the definition, `App.examples` has no seam on this builder). Flagged to
/// the coordinator, not silently lost; see `📚️examples/🎬️demo-session` for this subset's own example
/// facet, the likely intended replacement mechanism.
pub fn create_puzzle2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(Puzzle2dPlayApp::DIALECT)
            .document(["semio", "puzzle", "2d"])
            .artifact_kind(crate::standards::v1::subsets::any::schema::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "2d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE2D_PLAY_MODE_EDIT)
            .window_kind_def(overview::definition())
            .window_kind_def(detail::definition())
            .window_kind_def(selection::definition())
            .interaction(puzzle2d_interaction_definition())
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            // 🗣️ Locale and terminology are real user-facing settings verbs (mirrors puzzle3d's own
            // `.view_action` pair); the labels stay inline `LocalizedLabel::native` like every other
            // action here, since `puzzle2d_localized` resolves a `Puzzle2dLabels` field, not a phrase.
            // 🗂️ Referenced by `puzzle2d_context_menu_items` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .keybinding("delete,backspace", "deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            // 🔄️ Transform verbs over the live selection — the 2d twins of puzzle3d's gumball commits.
            .action_with(ActionDefinition::bounded_catalog("translateSelection", puzzle2d_localized(|l| l.translate), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("rotateSelection", puzzle2d_localized(|l| l.rotate), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("scaleSelection", puzzle2d_localized(|l| l.scale), ActionKind::Mutation).with_category("transform"))
            .keybinding("mod+d", "duplicateSelection")
            // 📤📥 Fixture round trip: a JSON download of the document and a file-picker import.
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", puzzle2d_localized(|l| l.export), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", puzzle2d_localized(|l| l.import), ActionKind::Shell).with_category("file"))
            .action_with(puzzle2d_internal_action("importSnapshot", LocalizedLabel::native("Import Fixture", "Fixture importieren"), ActionKind::Mutation))
            // 🔗️ Edge CRUD over two handles — the host-dispatchable twin of the board engine's own
            // drag-connect gesture, and the drop-time auto-connect's programmatic entry point.
            .action_with(ActionDefinition::bounded_catalog("createEdge", puzzle2d_localized(|l| l.connect), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::bounded_catalog("deleteEdge", puzzle2d_localized(|l| l.disconnect), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteEdge")
            .action_with(ActionDefinition::bounded_catalog("proximityConnect", LocalizedLabel::native("Connect Nearby", "In der Nähe verbinden"), ActionKind::Mutation).with_category("create"))
            .action_with(puzzle2d_internal_action("setProximityRadius", puzzle2d_localized(|l| l.proximity_radius), ActionKind::View).with_category("settings"))
            // 👁️ Palette-visible ephemeral view/selection commands.
            .action_with(ActionDefinition::bounded_catalog("selectSameKind", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            // 🔧️ Internal content operations — inspector/panel/board/import-bound, not palette commands.
            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle2d_flag_value_action("setSelectionHidden", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle2d_flag_value_action("setSelectionLocked", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(puzzle2d_internal_action("patchInspectorNodes", LocalizedLabel::native("Patch Inspector Nodes", "Inspektorknoten aktualisieren"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("redrawHandles", LocalizedLabel::native("Redraw Handles", "Anschlüsse neu zeichnen"), ActionKind::Mutation))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw") })
            .action_with(puzzle2d_internal_action("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("acceptSuggestion", puzzle2d_localized(|l| l.accept_suggestion), ActionKind::Mutation))
            // 🖱️ Internal pointer/gesture/engagement view vocabulary — pure runtime/host state, emit no operations.
            // 🎥️ `setCamera` is session-only view state, so it belongs in this View-kind group.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            // ⌨️ A submitted engagement line ("move 50 25", "connect …") EDITS the document — its own
            // publication contract declares the Artifact lane — so it is a `Mutation`, exactly as
            // `📐️cad` and `🏭️process`'s identical verb declare it. Declared `View`, kind discipline
            // refused every submitted line at dispatch with "View-kind command 'engagementSubmit' must
            // not emit operations", i.e. the command line could not move, connect or place anything.
            .action_with(puzzle2d_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand") })
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand") })
            .action_with(puzzle2d_internal_action("setLodModeForPane", LocalizedLabel::native("Set LOD Mode For Pane", "LOD-Modus für Bereich festlegen"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3") })
            .action_with(puzzle2d_internal_action("setBrushKindWeights", LocalizedLabel::native("Set Brush Kind Weights", "Pinsel-Artgewichte festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushNodeSize", LocalizedLabel::native("Set Brush Node Size", "Pinsel-Knotengröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformationsgriff festlegen"), ActionKind::View))
            // 🎯️ Target regions — the flat analogue of puzzle3d's target volumes. `addTargetRegion` is
            // the Area Brush's Alt+click commit and the only palette-visible one; the three
            // entity-scoped verbs stay off the palette exactly as puzzle3d keeps its own off it.
            .mutation("addTargetRegion", puzzle2d_localized_phrase(|l| l.target_region, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("deleteTargetRegion", LocalizedLabel::native("Delete Target Region", "Zielbereich löschen"), ActionKind::Mutation).with_category("targets") })
            .action_destructive("deleteTargetRegion")
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setTargetRegionFlag", LocalizedLabel::native("Set Target Region Flag", "Zielbereichsmarkierung festlegen"), ActionKind::Mutation).with_category("targets") })
            .action_with(puzzle2d_flag_value_action("setTargetRegionHidden", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "hidden", LocalizedLabel::native("Set Target Region Hidden", "Zielbereich verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle2d_flag_value_action("setTargetRegionLocked", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "locked", LocalizedLabel::native("Set Target Region Locked", "Zielbereich gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(puzzle2d_internal_action("relocateTargetRegion", LocalizedLabel::native("Relocate Target Region", "Zielbereich verlagern"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setAreaBrushSize", LocalizedLabel::native("Set Area Brush Size", "Flächenpinselgröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("cycleBrushCandidate", puzzle2d_localized(|l| l.cycle_candidate), ActionKind::View))
            .action_with(puzzle2d_internal_action("cycleBrushCandidateBack", puzzle2d_localized(|l| l.cycle_candidate_back), ActionKind::View))
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .action_with(puzzle2d_internal_action("targetBrushSuggestions", puzzle2d_localized(|l| l.target_suggestions), ActionKind::View))
            .action_with(puzzle2d_internal_action("hoverSuggestion", puzzle2d_localized(|l| l.hover_suggestion), ActionKind::View))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(puzzle2d_internal_action("openHandleSuggestions", puzzle2d_localized(|l| l.suggest_nodes), ActionKind::View))
            .action_with(puzzle2d_internal_action("closeHandleSuggestions", puzzle2d_localized(|l| l.close_suggestions), ActionKind::View))
            .action_with(puzzle2d_internal_action("lodScaleJson", LocalizedLabel::native("LOD Scale Json", "LOD-Skalierung-Json"), ActionKind::View))
            // 🌐️🎯️ Window-option verbs: the grid's show/hide toggle and the per-granularity pick filter.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster einblenden"), ActionKind::View, "layout-grid") })
            .action_with(puzzle2d_internal_action("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"), ActionKind::View))
            // 🚧️🫂️ Placement tuning — the two settings-panel steppers a brush/fill collision test reads.
            .action_with(puzzle2d_internal_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Kontakttoleranz der Pinselplatzierung festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushPlacementOverlapBudget", LocalizedLabel::native("Set Brush Placement Overlap Budget", "Überlappungsbudget der Pinselplatzierung festlegen"), ActionKind::View))
            // 🔂️ The engagement bar's repeat-last: one more placement from the armed fill tool.
            .action_with(puzzle2d_internal_action("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"), ActionKind::View))
            // 🗨️ Shell-only effect (no document mutation): opens the declared "addNode" dialog. The
            // user-facing "Add Node…" row is this one, so the parametrized `addNode` verb below stays
            // the dialog's/catalogue row's/drop's target rather than a palette entry of its own.
            .action_with(ActionDefinition::bounded_catalog("openAddNodeDialog", puzzle2d_localized_phrase(|l| l.node, |w| format!("Add {w}…"), |w| format!("{w} hinzufügen…")), ActionKind::Shell).with_category("create"))
            // 📝️ Staged palette args for the two content commands that need a target.
            .action_args("addNode", vec![puzzle2d_node_kind_arg()])
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Δx", "Δx")).default_value(&0.0),
                ActionArgDef::number("dy", LocalizedLabel::native("Δy", "Δy")).default_value(&0.0),
            ])
            .action_args("rotateSelection", vec![ActionArgDef::number("angle", puzzle2d_localized(|l| l.angle)).required().default_value(&90.0)])
            .action_args("scaleSelection", vec![ActionArgDef::number("factor", LocalizedLabel::native("Factor", "Faktor")).required().default_value(&1.5)])
            .action_args("createEdge", vec![
                ActionArgDef::text("source", puzzle2d_localized(|l| l.source)).required(),
                ActionArgDef::text("target", puzzle2d_localized(|l| l.target)).required(),
            ])
            .action_args("deleteEdge", vec![ActionArgDef::text("id", puzzle2d_localized(|l| l.id)).required()])
            .action_args("setProximityRadius", vec![ActionArgDef::number("value", puzzle2d_localized(|l| l.proximity_radius)).required().default_value(&crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_PROXIMITY_RADIUS)])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, puzzle2d_localized(|l| l.example_concrete_forest)),
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin Capsule Tower")),
                ]).required().default_value(&PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID),
            ])
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("closeHandleSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("acceptSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("openHandleSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("forceLayout", InteractiveJobClassification::Migrated)
            .action_interactive_job("lodScaleJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchInspectorNodes", InteractiveJobClassification::Migrated)
            .action_interactive_job("redrawHandles", InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
            .action_interactive_job("createEdge", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteEdge", InteractiveJobClassification::Migrated)
            .action_interactive_job("proximityConnect", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushKindWeights", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushNodeSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodModeForPane", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAreaBrushSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementOverlapBudget", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddNodeDialog", InteractiveJobClassification::Migrated)
            // 🗨️ The dialog `openAddNodeDialog` opens, driving the existing `addNode` operation's LIVE
            // `kind` select.
            .dialog(
                DialogDefinition::new(PUZZLE2D_ADD_NODE_DIALOG_ID, puzzle2d_localized_phrase(|l| l.node, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")), ActionRef::new("addNode"))
                    .body(puzzle2d_localized_phrase(|l| l.node, |w| format!("Choose the kind of {w} to add to the board."), |_w| "Wählen Sie die Art zum Hinzufügen.".to_string()))
                    .args(vec![puzzle2d_node_kind_arg()])
                    .submit_label(LocalizedLabel::native("Add", "Hinzufügen")),
            )
            // 🧰️ Canvas utilities — one exclusive set, active utility host-owned (never a document
            // operation); bound to the interactive overview pane by that window's own definition.
            .utility(select_utility::definition(puzzle2d_localized(|l| l.select)))
            .utility(brush_utility::definition(puzzle2d_localized(|l| l.brush)))
            .utility(area_brush_utility::definition(puzzle2d_localized(|l| l.area_brush)))
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
            .tool(fill::definition(puzzle2d_localized(|l| l.fill)))
            .default_layout(edit::layout())
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 2D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 2D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected node from the 2D puzzle together with its connections.", "Löscht alle ausgewählten Knoten samt ihrer Verbindungen aus dem 2D-Puzzle."))
            .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected node next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Knoten eine Kopie hinzu; ohne Auswahl geschieht nichts."))
            .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected nodes; only the view changes.", "Richtet die Kamera auf die ausgewählten Knoten aus; nur die Ansicht ändert sich."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected nodes by dx and dy.", "Verschiebt die ausgewählten Knoten um dx und dy."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected nodes by the given angle.", "Dreht die ausgewählten Knoten um den angegebenen Winkel."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected nodes by the given factor.", "Skaliert die ausgewählten Knoten um den angegebenen Faktor."))
            .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 2D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 2D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
            .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 2D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 2D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
            .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 2D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 2D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
            .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected nodes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Knoten."))
            .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected nodes hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Knoten oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected nodes locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Knoten oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
            .action_describe("addNode", LocalizedLabel::native("Adds a node of the given kind (a 2D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y einen Knoten der angegebenen Art (einer 2D-Blockart) hinzu."))
            .action_describe("forceLayout", LocalizedLabel::native("Lays out every node of the 2D puzzle with a force-directed layout, overwriting their positions.", "Ordnet alle Knoten des 2D-Puzzles mit einem kraftbasierten Layout an und überschreibt ihre Positionen."))
            .action_describe("createEdge", LocalizedLabel::native("Connects two handles of two nodes (source and target) with an edge.", "Verbindet zwei Griffe zweier Knoten (Quelle und Ziel) mit einer Kante."))
            .action_describe("deleteEdge", LocalizedLabel::native("Removes one edge by id, disconnecting its two handles.", "Entfernt eine Kante anhand ihrer Id und trennt ihre zwei Griffe."))
            .action_describe("proximityConnect", LocalizedLabel::native("Connects every pair of compatible handles that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen."))
            .action_describe("selectSameKind", LocalizedLabel::native("Extends the selection to every node of the same kind as the selected one.", "Erweitert die Auswahl auf alle Knoten derselben Art wie der ausgewählte."))
            .action_describe("patchInspectorNodes", LocalizedLabel::native("Sets one field on the given nodes, or on handles addressed by id (such as a handle's angle or radius); an empty id list addresses every node.", "Setzt ein Feld auf den angegebenen Knoten oder per Id adressierten Griffen (etwa Winkel oder Radius eines Griffs); eine leere Id-Liste adressiert alle Knoten."))
            .action_describe("redrawHandles", LocalizedLabel::native("Re-aims every connected handle at the edge it carries, updating the handle angles.", "Richtet jeden verbundenen Griff neu auf seine Kante aus und aktualisiert die Griffwinkel."))
            .action_describe("reorganize", LocalizedLabel::native("Lays out every node of the 2D puzzle automatically, overwriting their manual positions.", "Ordnet alle Knoten des 2D-Puzzles automatisch an und überschreibt ihre manuellen Positionen."))
            .action_describe("addTargetRegion", LocalizedLabel::native("Adds a rectangular target region at the given origin and size (the area brush size by default).", "Fügt an Ursprung und Größe (standardmäßig der Flächenpinselgröße) einen rechteckigen Zielbereich hinzu."))
            .action_describe("deleteTargetRegion", LocalizedLabel::native("Deletes one target region by id.", "Löscht einen Zielbereich anhand seiner Id."))
            .action_describe("setTargetRegionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target regions.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielbereichen."))
            .action_describe("setTargetRegionHidden", LocalizedLabel::native("Sets the given or selected target regions hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Zielbereiche oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setTargetRegionLocked", LocalizedLabel::native("Sets the given or selected target regions locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Zielbereiche oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("relocateTargetRegion", LocalizedLabel::native("Moves or resizes one target region to the given rectangle.", "Verschiebt oder skaliert einen Zielbereich auf das angegebene Rechteck."))
            .action_describe("openAddNodeDialog", LocalizedLabel::native("Opens the Add Node dialog to pick a node kind to place.", "Öffnet den Dialog Knoten hinzufügen, um eine zu setzende Knotenart zu wählen."))
            .action_audience("applyBoardEvents", semio_framework_plugin::CapabilityAudience::Input)
            .action_destructive("importSnapshot")
            .action_destructive("forceLayout")
            .action_destructive("reorganize")
            .build_definition()
}

// 🗂️ `Puzzle2dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-2d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle2dPlayApp>()` on
// `crate::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle2d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone.
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ The one puzzle2d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

#[cfg(test)]
#[path = "🧪️tests/🤖️agent-lane/🦀️.rs"]
mod agent_lane_tests;

/// 📋️ The clipboard route's own laws — copy/cut/paste round trips, the locked refusal, and the
/// single-edit/undo discipline every one of them owes.
#[cfg(test)]
#[path = "🧪️tests/🔬️clipboard/🦀️.rs"]
mod clipboard_tests;

/// 🔒️ The lock's own laws — delete, the board drag rows, the rotate ring, the three transform verbs
/// and an inspector patch each refuse a locked entity with exactly ONE sentence and no edit.
#[cfg(test)]
#[path = "🧪️tests/🔬️locks/🦀️.rs"]
mod lock_tests;

/// 🛠️ The select tool's own laws — one drag, one rotation, one nudge, one HUD move, one inspector delta is ONE
/// tool transaction (one edit, one row, one ref, one parametric leaf), and a cancelled gesture leaves zero trace.
#[cfg(test)]
#[path = "🧪️tests/🧪️select-tool-transactions/🦀️.rs"]
mod select_tool_transaction_tests;

/// 🧱️ The board tools' laws — every board gesture that is not a selection transform is ONE tool transaction of its
/// tool, a transient flush commits nothing, and the `vortex` topology names every board entity.
#[cfg(test)]
#[path = "🧪️tests/🧪️board-tools/🦀️.rs"]
mod board_tool_tests;

/// 📼️ The recorded-kinds laws — every board-editing command emits the concrete kinds of its user action, undo restores the
/// board, and a whole-document import leaves through the load path instead of a mutation.
#[cfg(test)]
#[path = "🧪️tests/🧪️recorded-kinds/🦀️.rs"]
mod recorded_kind_tests;

/// ⏪️ The select tool's leaves under time travel — the `🧫️select-tool-history` corpus through the app and a standalone
/// store: every input editable, the preview and its highlight, blocking errors resolved by editing, warnings kept.
#[cfg(test)]
#[path = "🧪️tests/🧪️select-tool-history/🦀️.rs"]
mod select_tool_history_tests;

/// ⏪️ The generic history-edit runtime with puzzle 2d as its witness — the `🧫️history-edit-runtime` corpus: drafts from a
/// review, replay progress and cancel, warnings after finalize and reload, Fatal/Error resolved by withdraw and by "Use
/// selection", overwrite and new alternative undone and redone, and a long remote history change paused and resumed.
#[cfg(test)]
#[path = "🧪️tests/🧪️history-edit-runtime/🦀️.rs"]
mod history_edit_runtime_tests;
//#endregion 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🧵️retained-wiring/🦀️.rs"]
mod retained_wiring_tests;
