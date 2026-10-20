//! 🕹️ Interaction of the puzzle 2d editor: the action context every command arm reads and writes and the context menu. See [`super`].

use crate::{editor::puzzle2d::snapshot::puzzle2d_addresses_locked_entity, editor::puzzle2d::snapshot::puzzle2d_entity_hidden, editor::puzzle2d::snapshot::puzzle2d_handle_kind, editor::puzzle2d::snapshot::puzzle2d_kinds_compatible, editor::puzzle2d::snapshot::puzzle2d_occupied_handles};
use crate::editor::puzzle2d::engine::BoardHost;
pub use crate::{editor::puzzle2d::modes::edit::windows::overview::utilities::select::Puzzle2dSelectionRecord, editor::puzzle2d::modes::edit::windows::overview::utilities::select::SelectToolRequest};
use semio_framework_tool_machine::{ChartGesture, GesturePhase};
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select as select_utility;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_pack_json::Value;
use std::cell::RefCell;
use std::collections::HashSet;
use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot::Puzzle2dScene;
use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::recorder::Puzzle2dRecorder;

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
    pub fn publish_transaction(&mut self, transaction: protocol::TransactionRef, mutations: Vec<Puzzle2dMutation>) {
        if self.transaction.is_none() && !self.authoring_seed.is_empty() {
            *self.transaction = Some(transaction);
        }
        self.artifact_mutations.extend(mutations);
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
pub async fn puzzle2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, snapshot: &Value, selected: &[String], view_state: &semio_framework_plugin::ViewModel) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::selection_count_phrase;
    use semio_framework_plugin::ContextMenuItemSpec;
    use semio_framework_plugin::Menu;
    use semio_framework_plugin::SelectionKind;
    let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;
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
    let mut menu = Menu::of(registry, view_state);
    if let [only] = selected_handle_ids.as_slice() {
        menu = menu.item(item("suggestNodes", if is_de { "Knoten vorschlagen" } else { "Suggest nodes" }, "sparkles", "openHandleSuggestions", Some(semio_framework::dsl_value!({ "handleId": only })), false, false));
    }
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

/// 🖌️ Re-enters the board host's brush slot on the handle this window transient remembers. The guest
/// host is rebuilt from the document on EVERY dispatch (`puzzle2d_dispatch_emit`), so the slot
/// `openHandleSuggestions` opened is gone by the next verb — every candidate verb after it restores the
/// slot before it reads an index. The rebuild is deterministic (same snapshot, same kind catalogs, same
/// weights), so the candidate page is the one the client painted and the index the popup names is the
/// candidate that gets placed. An open popup claims the slot's hover; the armed brush only targets it.
/// `requested` is the handle the CALLER named — the popup's own rows carry it, so a client that still
/// has the menu on screen never depends on the transient having round-tripped back to the guest yet.
pub fn puzzle2d_restore_brush_slot(ctx: &mut Puzzle2dActionCtx<'_>, requested: Option<&str>) -> Option<String> {
    let popup = ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.handle_id.clone()).filter(|handle_id| !handle_id.is_empty());
    let named = requested.filter(|handle_id| !handle_id.is_empty()).map(str::to_string);
    let opened = named.clone().or_else(|| popup.clone());
    let handle_id = opened.clone().or_else(|| Some(ctx.scene.runtime.brush_candidate_source_handle_id.clone()).filter(|handle_id| !handle_id.is_empty()))?;
    let mut host = ctx.host.borrow_mut();
    if named.is_some() || popup.is_some() {
        host.brush_open_slot(&handle_id);
    } else {
        host.brush_target_slot(Some(&handle_id));
    }
    Some(handle_id)
}

//#region 📡️ProximityConnect
/// 📡️ The programmatic twin of the drop-time auto-connect: connects one node's (or the live
/// selection's) open handles to whatever compatible open handle is inside `radius`.
pub fn proximity_connect(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let explicit: Vec<String> = args
        .and_then(|value| value.get("nodeId").or_else(|| value.get("id")))
        .and_then(Value::as_str)
        .map(|id| vec![id.to_string()])
        .unwrap_or_else(|| args.and_then(|value| value.get("ids")).and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default());
    let node_ids = if explicit.is_empty() { ctx.selected_ids() } else { explicit };
    if node_ids.is_empty() {
        return;
    }
    let radius = args.and_then(|value| value.get("radius")).and_then(Value::as_f64).filter(|radius| radius.is_finite()).unwrap_or(ctx.scene.runtime.proximity_radius);
    crate::editor::puzzle2d::commands::proximity_connect::puzzle2d_proximity_connect(ctx.recorder, &node_ids, radius);
}
//#endregion 📡️ProximityConnect
