//! 🎮️ The command log and history views, and the history panel.
use crate::history_view::HistoryCommandFilter;
use crate::history_view::InverseAction;
use crate::history_view::CommandView;
use crate::history_view::MutationView;
use crate::history_view::HistoryView;
use crate::history_view::AlternativeView;
use semio_framework::{ActionKind, IconName, ViewModel, REVERT_TO_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY;
use crate::app_contract::ArtifactApp;
use crate::table_kit::row_action;
use crate::table_kit::row_target;
use crate::tool_jobs::HistoryMutationPages;
use crate::tool_jobs::TimeTravelPanel;
use crate::tool_jobs::history_mutation_entry;
use crate::tool_jobs::time_travel;
use crate::tree_kit::TreeWindows;
use crate::tree_kit::tree_window_indexed_item;
use crate::tree_kit::tree_window_section;
use crate::window_surface::UiAssemblyResult;
use crate::window_surface::ui_assembly_error;
use crate::window_surface::ui_label;
use crate::window_surface::ui_text;

/// 🗝️ What a backfilled command-log row references: a document edit (with the verb that authored it) or a
/// `Supersede` history transition.
pub(crate) enum CommandLogAppendKey {
    Edit { edit_id: String, verb: Option<String>, child_edit_ids: Vec<String> },
    Members { edit_ids: Vec<String> },
    Transition(String),
}

/// 🧭️ What a cached [`HistoryView`] was built against (design §20.14): the document's and every composed member's
/// [`time_travel::HistoryStoreStamp`], the newest command-log row and the row count it held, its shell rows and their
/// undone count, and the supersede records — what lets the next refresh rebuild only the rows a change touched.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct HistoryViewStamp {
    pub(crate) document: time_travel::HistoryStoreStamp,
    pub(crate) members: Vec<(String, time_travel::HistoryStoreStamp)>,
    pub(crate) newest_seq: u64,
    pub(crate) rows: usize,
    pub(crate) inverse_rows: usize,
    pub(crate) shell_undone: usize,
    pub(crate) supersede_records: usize,
}

/// 🗂️ The history view a refresh patches in place, with the stamp it was built against; the projection cache shares it.
pub(crate) struct HistoryViewCache {
    pub(crate) view: std::sync::Arc<HistoryView>,
    pub(crate) stamp: HistoryViewStamp,
}

/// 🩹️ The rows one refresh rebuilds in a cached [`HistoryView`]: the log rows from `fresh_from` on are new, `rebuild`
/// names the held rows a change touched, `positions` the applied position of every document edit it touched.
pub(crate) struct HistoryViewPatch {
    pub(crate) fresh_from: usize,
    pub(crate) rebuild: BTreeSet<u64>,
    pub(crate) positions: Vec<(String, Option<usize>)>,
}

/// 🧾️ How far the command log is proven to reference every edit (`backfill_command_log`): the document ledger's length
/// and its last edit id, each composed member ledger's, the supersede records, and the newest row at that moment — the
/// next backfill reads only what came after (design §20.14).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct HistoryBackfillMark {
    pub(crate) edits: usize,
    pub(crate) last_edit: Option<String>,
    pub(crate) members: Vec<(String, usize, Option<String>)>,
    pub(crate) supersede_records: usize,
    pub(crate) log_seq: u64,
}

#[cfg(test)]
thread_local! {
    /// 🧮️ History rows built on this test thread — what the O(change) laws of the history view count (design §20.14).
    static HISTORY_ROWS_BUILT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 📚️ What the history rows of one build or patch read beyond their command-log entries: the applied position of the
/// edits it knows (`full`: every applied edit, so an unknown one is not applied), the edits by id (`full` only; a patch
/// reads the ledger's tail), the previous projection of each row and its labels, the composed members' edits, the
/// members' applied tails, the history edits in effect and the app instance id.
pub(crate) struct HistoryRowReads<'a, Mu> {
    pub(crate) full: bool,
    pub(crate) positions: HashMap<&'a str, Option<usize>>,
    pub(crate) edits: HashMap<&'a str, &'a protocol::Edit<Mu>>,
    pub(crate) previous: HashMap<u64, &'a CommandView>,
    pub(crate) labelled: HashMap<&'a str, &'a MutationView>,
    pub(crate) members: HashMap<String, time_travel::MemberEditHistory>,
    pub(crate) child_applied_tails: HashSet<String>,
    pub(crate) applied_entries: HashSet<usize>,
    pub(crate) app_id: String,
}

/// 🏷️ A backfilled edit row's label: the registry label of the verb that authored it, else its first operation's label
/// `first` (a document leaf's localized `SemanticMutation::label`) — the rule a live row records; an edit without
/// operations is named by its id. An edit stores no description (design §20.6).
pub(crate) fn backfilled_edit_label<Mu>(edit: &protocol::Edit<Mu>, verb_label: Option<LocalizedLabel>, first: impl FnOnce(&Mu) -> LocalizedLabel) -> LocalizedLabel {
    verb_label.unwrap_or_else(|| edit.forwards.first().map_or_else(|| LocalizedLabel::data(edit.id.clone()), first))
}

/// 🧾️ Inputs for one appended session command before its sequence and timestamp are assigned.
pub(crate) struct CommandLogAppend<'a> {
    action_id: &'a str,
    label: LocalizedLabel,
    kind: ActionKind,
    edit_id: Option<String>,
    transition_id: Option<String>,
    timestamp: Option<String>,
    inverse: Option<InverseAction>,
}

/// 📜️ Operations a history row previews: its edit's newest ones (see `CommandView::op_lines`).
pub const HISTORY_ROW_OPERATION_PREVIEW: usize = 8;

/// 🏷️ A history row labelled by its edit's first leaf: the leaf's own localized label (`SemanticMutation::label`, resolved
/// at projection time so a reload keeps every locale), with `(+N)` for the edit's other operations.
// 🚫️async: a pure label composition.
pub fn history_leaf_row_label(leaf: &LocalizedLabel, op_count: usize) -> LocalizedLabel {
    let (leaf, more) = (leaf.clone(), op_count.saturating_sub(1));
    LocalizedLabel::from_fn(move |terminology, locale| {
        let first = leaf.resolve(terminology, locale);
        if more > 0 {
            format!("{first} (+{more})")
        } else {
            first.to_string()
        }
    })
}

/// ⏳️ The fault code every verb but a view or interaction verb answers while a whole-document load still runs (gap N17);
/// its en/de notice is the kernel's `history_notice("document.loading")`.
pub const DOCUMENT_LOADING_CODE: &str = "document.loading";

/// 🧱️ The code a refused adoption of this replica's own deferred history step reads by (its replay would leave Errors or
/// Fatals downstream); its en/de notice is the kernel's `history_notice("history.step-blocked")`.
pub const HISTORY_STEP_BLOCKED_CODE: &str = "history.step-blocked";

/// 🫱️ The code the document store refuses a supersession of one operation of a cross-document unit by
/// (`VcsError::UnitSpansDocuments`), which a mutation row's Withdraw answers for such an operation; its en/de notice is
/// the kernel's `history_notice("history.unit-spans-documents")`.
pub const HISTORY_UNIT_SPANS_DOCUMENTS_CODE: &str = "history.unit-spans-documents";

/// 🫥️ The fault code of a history verb, a history query or a history-carrying document lane on a head-only pure
/// evaluation (`📓️api-stepped-document-load.md` §4); its en/de notice is the kernel's
/// `history_notice("pure.history-unavailable")`.
pub const PURE_HISTORY_UNAVAILABLE_CODE: &str = "pure.history-unavailable";

/// 📚️ The framework's catalogue route: `setActiveExample` of an app that declares no such action loads the app's
/// catalogue document ([`ArtifactApp::catalogue_example_document`]).
pub const CATALOGUE_EXAMPLE_ACTION_ID: &str = "setActiveExample";

/// 🏷️ The framework's own label, in every shell locale, of a runtime verb no app declares (design §20.6): a history row
/// the runtime records for it never reads a hand-written description. `None` for every other verb.
// 🚫️async: a pure label lookup.
pub fn runtime_verb_label(verb: &str) -> Option<LocalizedLabel> {
    match verb {
        CATALOGUE_EXAMPLE_ACTION_ID => Some(LocalizedLabel::native("Load example", "Beispiel laden")),
        _ => None,
    }
}

/// 🎯️ The leaf label of a tool transaction's history row (design §19.1): the first of the edit's applied `ops` whose kind
/// the authoring `tool` declares as its intent ([`ArtifactApp::tool_intent_kinds`]), read from its mutation row in
/// `mutations` (its effective input) or labelled afresh when that row is windowed out; `None` when the tool declares no
/// kind any operation has, so the first leaf labels the row as before.
// 🚫️async: a pure lookup over the projected operations.
pub fn history_intent_label<A: ArtifactApp>(tool: &str, ops: &[&store::AppliedMutation<'_, A::Mutation>], mutations: &[MutationView]) -> Option<LocalizedLabel> {
    history_intent_label_of::<A::Snapshot, A::Mutation>(A::tool_intent_kinds(tool), ops, mutations)
}

/// 🎯️ Resolves declared intent through a store's concrete mutation authority and effective labels.
pub(crate) fn history_intent_label_of<P, Mu: protocol::SemanticMutation<P>>(intents: &[&str], ops: &[&store::AppliedMutation<'_, Mu>], mutations: &[MutationView]) -> Option<LocalizedLabel> {
    let intent = ops.iter().take(if intents.is_empty() { 0 } else { ops.len() }).find(|op| intents.contains(&protocol::SemanticMutation::<P>::semantics(op.operation).kind))?;
    Some(mutations.iter().find(|row| row.mutation_id == intent.mutation_id.0).map_or_else(|| protocol::SemanticMutation::<P>::label(intent.operation), |row| row.label.clone()))
}

/// 🙈️ Whether a dispatch of `kind` is a history row: never an interaction verb, and a `View` only when it edited a
/// store (`edited`) or carries an inverse — a pure view verb changes no history and undo cannot reach it.
// 🚫️async: a pure total predicate over the dispatch's own facts.
pub fn history_row_is_recorded(kind: ActionKind, edited: bool, inverse: bool) -> bool {
    match kind {
        ActionKind::Interaction => false,
        ActionKind::View => edited || inverse,
        ActionKind::Mutation | ActionKind::History | ActionKind::Clipboard | ActionKind::Shell => true,
    }
}

/// 🧾️ One history transition as the alternatives list reads it: a branch naming its alternative, author and wall
/// time, or a supersession with its scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlternativeFact<'a> {
    Branch { alternative_id: &'a str, actor: &'a str, physical_ms: u64 },
    Supersede { scope: Option<&'a str> },
}

/// 🌿️ The alternatives list of the history body over `(id, name)` pairs in declaration order (the trunk first, with the
/// empty name the history body localizes) and the log's `facts` in HLC order: current when it is the `active` line,
/// authored by the first branch naming it, edited when a supersession is scoped to it. Twin: `🧪️tests/🧪️history-alternatives/🟦️.ts` over `🧫️fixtures/🧫️history-alternatives/🔣️.json`.
// 🚫️async: a pure projection over already-decoded facts.
pub fn history_alternative_views<'a>(alternatives: impl IntoIterator<Item = (&'a str, &'a str)>, active: Option<&str>, facts: &[AlternativeFact<'_>]) -> Vec<AlternativeView> {
    alternatives
        .into_iter()
        .map(|(id, name)| {
            let branch = facts.iter().find_map(|fact| match fact {
                AlternativeFact::Branch { alternative_id, actor, physical_ms } if *alternative_id == id => Some((*actor, *physical_ms)),
                _ => None,
            });
            AlternativeView {
                id: id.to_string(),
                name: name.to_string(),
                current: active == Some(id),
                author: branch.map(|(actor, _)| actor.to_string()),
                branched_at: branch.map(|(_, physical_ms)| protocol::scalar::format_rfc3339_ms(i64::try_from(physical_ms).unwrap_or(i64::MAX))),
                edited: facts.iter().any(|fact| matches!(fact, AlternativeFact::Supersede { scope: Some(scope) } if *scope == id)),
            }
        })
        .collect()
}

/// 📌️ The framework-reserved controller the explicit check-in dialog funnel answers on — the
/// shell-owned twin of `framework.sync`: a renderer intercepts it before any app sees it
/// (`🐚️Shell/🎯️targets/🧊️/🦀️.rs`'s `dispatch_action`), because the check-in MESSAGE is shell state
/// no artifact app holds.
pub const FRAMEWORK_CHECKIN_CONTROLLER_ID: &str = "framework.checkin";

/// 📌️ Uncommitted-since-the-last-checkpoint count — the same fold the React shell's
/// `uncommittedEditCount` (`🏛️ShellHost/🟦️.tsx`) applies, walked OLDEST-first over a
/// newest-first command log: every applied mutation row counts, and a landed `commitCheckpoint`
/// resets it to zero. It is what React prints after the Check In label as ` (N)`.
// 🚫️async: a pure total fold over rows already in hand.
fn uncommitted_command_count(history: &HistoryView) -> u32 {
    let mut pending = 0u32;
    for entry in history.commands.iter().rev() {
        if entry.kind == ActionKind::History && entry.action_id == "commitCheckpoint" {
            pending = 0;
            continue;
        }
        if entry.kind == ActionKind::Mutation && entry.applied {
            pending += 1;
        }
    }
    pending
}

// 🚫️async: a pure total mapping, called inside the Commands window's synchronous row closure.
/// 🕶️ View is ephemeral cursor/selection/camera state; Shell is an outside-the-document
/// host effect — distinct icons so a folded ×count row reads at a glance.
fn history_panel_icon_id(kind: ActionKind) -> IconName {
    match kind {
        ActionKind::Mutation => IconName::Pencil,
        ActionKind::History => IconName::Undo,
        ActionKind::Clipboard => IconName::Clipboard,
        ActionKind::View => IconName::Eye,
        ActionKind::Shell => IconName::Monitor,
        ActionKind::Interaction => "mouse-pointer".into(),
    }
}

/// 🕰️ Builds the framework's history panel body from a `HistoryView` as a pure side-panel
/// `Tree` (same shape as Document/Catalogue): an Actions section (undo/redo/checkpoint/alternative +
/// filter control) and a Commands section of newest-first rows with optional "inverse" revert.
/// Shared by both renderers — `VcsArtifactApp::render` returns this verbatim for
/// `FRAMEWORK_HISTORY_BODY_KEY`. `read_only` is contract §2.3 clause 3 (`VcsArtifactApp::render`
/// passes `A::ROLE == AppRole::Viewer`): every mutating action row renders disabled and the
/// per-command "Backwards" revert action is omitted entirely, regardless of `can_undo`/`can_redo`
/// or `entry.revertible` — the filter control and Commands list themselves stay fully live, since
/// browsing history is not a mutation.
///
/// 🪟️ The Commands section is a windowed container ([`tree_window_section`]) keyed
/// `framework.history.commands` under body [`FRAMEWORK_HISTORY_BODY_KEY`]: it stamps the full live
/// filtered row count as its `TreeWindow::total` and materialises only the slice the host asked
/// for through `ViewModel::tree_windows` (one viewport on a cold paint). A session log of any
/// length therefore assembles, the scrollbar spans it all, and there is no `+N` row and no guest
/// page cursor.
///
/// 📌️ `#s-checkin` — React's EXPLICIT check-in row (`🏛️ShellHost/🟦️.tsx`'s
/// `framework.history.checkin`), a separate affordance from the no-message quick Checkpoint
/// above and absent outright for a viewer (never disabled — React's own `canCheckIn` gate,
/// whose wgpu twin is `can_check_in`, and which `read_only` carries here). The button keeps
/// React's bare `s-checkin` id: it is the id every battery and probe looks for, and it had no
/// trigger anywhere on the wgpu target at all (`📓️audit-w14-shell-residual.md` C2).
///
/// ⚠️ React's press OPENS a message dialog whose typed text rides the checkpoint; the message
/// is shell-local state this assembler cannot see, so the row dispatches `submit` directly and
/// the shell falls back to its own `"check-in"` message (React's own blank-draft fallback).
///
/// ⏪️ While a history edit is open (`time_travel`), the body leads with the session band
/// (`framework.history.timeTravel`) and, while a mutation is being edited, its draft editor
/// (`framework.history.editor` and its unwindowed inputs), then the alternatives, the actions and the commands;
/// otherwise the actions lead, then the alternatives and the commands. The history-lane actions and the per-row
/// revert are frozen during the edit, and every row's mutations carry the session overlay (replay outcome,
/// pending, edited).
///
/// ↩️ Undo is enabled exactly where its chord and palette entry are: for every editor outside a history edit. The
/// host routes an `undo` to a remote (inference) history it owns, which this guest cannot see, and an undo with
/// nothing to undo is a benign no-op; `can_undo` therefore never disables it here.
///
/// ✏️ A row with applied mutations is a tree window over ALL of them (gap N1): its projected ones first — the ones whose
/// outcome blocks finalizing lead (so the review's next problem is inside the first slice a host materialises, design
/// §22.2), then the other flagged ones — then every other operation of its edit in op order, read from `pages` for the
/// slice a host window opened past the projection ([`time_travel::history_row_mutation_extent`]). Each child carries its
/// label, a description naming its state and severity in words (colour is never the only carrier), tone and icon by
/// severity, and its row actions ([`history_panel_mutation_row`]): Edit, then Withdraw (design §22.1) — or Restore, on a
/// mutation holding an accepted draft — each disabled, naming why, wherever a viewer, the mutation, the store, the
/// session or the instance refuses it ([`time_travel::mutation_row_refusals`]: gap N15, audit W2A-9, design §22.20).
#[allow(clippy::too_many_arguments)]
pub async fn ui_history_panel(
    history: &HistoryView,
    time_travel: Option<&TimeTravelPanel>,
    reprojection: Option<&semio_framework::kernel::HistoryReprojection>,
    pages: &HistoryMutationPages,
    controller_id: &str,
    locale: Locale,
    read_only: bool,
    busy: bool,
    view: &ViewModel,
) -> UiAssemblyResult<BuiltNode> {
    let frozen = time_travel.is_some();
    let text_of = |text: time_travel::HistoryPanelText| text.text(locale);
    let action_item = |id: &str, icon_id: IconName, label: &str, action: &str, enabled: bool| -> UiAssemblyResult<BuiltNode> {
        let action = ActionId::try_v1(controller_id, action).ok_or_else(|| ui_assembly_error("history-panel.action-id"))?;
        let button_id = UiText::try_format(format_args!("{id}.run")).ok_or_else(|| ui_assembly_error("history-panel.button-id"))?;
        let icon = ui_text(icon_id.as_str(), "history-panel.icon")?;
        let builder = button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
        let builder = builder.try_id(&button_id).map_err(|_| ui_assembly_error("history-panel.button-id"))?;
        let button = builder.try_on(Trigger::Activate, action).map_err(|_| ui_assembly_error("history-panel.button-binding"))?.try_build().map_err(|_| ui_assembly_error("history-panel.button-build"))?;
        let builder = tree_item(ui_label(label, "history-panel.item-label")?).icon(icon).disabled(!enabled);
        let builder = builder.try_id(id).map_err(|_| ui_assembly_error("history-panel.item-id"))?;
        let builder = builder.try_child(button).map_err(|_| ui_assembly_error("history-panel.item-child"))?;
        builder.try_build().map_err(|_| ui_assembly_error("history-panel.item-build"))
    };

    let mut filter_builder = select(ui_text(history.command_filter.value(), "history-panel.filter-value")?);
    for filter in HistoryCommandFilter::ALL {
        let label = match filter {
            HistoryCommandFilter::All => time_travel::HistoryPanelText::FilterAll,
            HistoryCommandFilter::WithoutMutations => time_travel::HistoryPanelText::FilterWithoutMutations,
            HistoryCommandFilter::OnlyMutations => time_travel::HistoryPanelText::FilterOnlyMutations,
        };
        filter_builder = filter_builder.try_item(ui_text(filter.value(), "history-panel.filter-item-value")?, ui_label(text_of(label), "history-panel.filter-item-label")?).map_err(|_| ui_assembly_error("history-panel.filter-items"))?;
    }
    let filter_builder = filter_builder.try_id("framework.history.filter.control").map_err(|_| ui_assembly_error("history-panel.filter-id"))?;
    let filter_builder = filter_builder.try_label(text_of(time_travel::HistoryPanelText::Filter)).map_err(|_| ui_assembly_error("history-panel.filter-label"))?;
    let filter_action = ActionId::try_v1(controller_id, SET_HISTORY_COMMAND_FILTER_ACTION_ID).ok_or_else(|| ui_assembly_error("history-panel.filter-action"))?;
    let filter_select = filter_builder.try_on(Trigger::Change, filter_action).map_err(|_| ui_assembly_error("history-panel.filter-binding"))?.try_build().map_err(|_| ui_assembly_error("history-panel.filter-build"))?;
    let filter_builder = tree_item(ui_label(text_of(time_travel::HistoryPanelText::Filter), "history-panel.filter-label")?).icon(ui_text(IconName::Filter.as_str(), "history-panel.filter-icon")?);
    let filter_builder = filter_builder.try_id("framework.history.filter").map_err(|_| ui_assembly_error("history-panel.filter-item-id"))?;
    let filter_item = filter_builder.try_child(filter_select).map_err(|_| ui_assembly_error("history-panel.filter-child"))?.try_build().map_err(|_| ui_assembly_error("history-panel.filter-item-build"))?;

    let command_rows: Vec<&CommandView> = history
        .commands
        .iter()
        .filter(|entry| match history.command_filter {
            HistoryCommandFilter::All => true,
            HistoryCommandFilter::WithoutMutations => entry.edit_id.is_none(),
            HistoryCommandFilter::OnlyMutations => entry.edit_id.is_some(),
        })
        .collect();
    let windows = TreeWindows::for_body(view, FRAMEWORK_HISTORY_BODY_KEY);
    let commands_section = tree_window_section(&windows, time_travel::HISTORY_COMMANDS_SECTION_KEY, ui_label(text_of(time_travel::HistoryPanelText::Commands), "history-panel.commands-label")?, true, &command_rows, |entry| {
        let entry_label = entry.label.resolve(Terminology::Native, locale);
        let label = if entry.count > 1 { UiText::clipped(&format!("{entry_label} x{}", entry.count)) } else { UiText::clipped(entry_label) };
        let id = UiText::try_format(format_args!("{}{}", time_travel::HISTORY_ROW_KEY_PREFIX, entry.seq)).ok_or_else(|| ui_assembly_error("history-panel.command-id"))?;
        let mutations: Vec<semio_framework::kernel::HistoryMutationEntry> = entry.mutations.iter().map(|mutation| history_mutation_entry(mutation, time_travel)).collect();
        let worst = mutations.iter().filter_map(|mutation| mutation.worst).max();
        let mut builder = tree_item(Label(label)).icon(ui_text(history_panel_icon_id(entry.kind).as_str(), "history-panel.command-icon")?);
        builder = builder.try_id(&id).map_err(|_| ui_assembly_error("history-panel.command-id"))?;
        let mut description = if entry.mutations.is_empty() { entry.op_lines.join(" \u{b7} ") } else { String::new() };
        if let Some(worst) = worst {
            let severity = time_travel::history_severity_text(worst, locale);
            description = if description.is_empty() { severity.to_string() } else { format!("{severity} \u{b7} {description}") };
            builder = builder.tone(time_travel::history_severity_tone(Some(worst)).0);
        }
        if let Some(author) = entry.author.as_deref().filter(|_| entry.transition_id.is_some()) {
            description = if description.is_empty() { author.to_string() } else { format!("{author} \u{b7} {description}") };
        }
        if !description.is_empty() {
            builder = builder.description(UiText::clipped(&description));
        }
        if (entry.edit_id.is_some() || entry.transition_id.is_some()) && !entry.applied {
            builder = builder.dimmed(true);
        }
        if entry.revertible && !read_only && !frozen {
            let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.row-action-args"))?;
            args.push("entrySeq".to_owned(), UiValue::Number(entry.seq as f64)).map_err(|_| ui_assembly_error("history-panel.row-action-args"))?;
            let revert = row_action(IconName::RotateCcw.as_str(), text_of(time_travel::HistoryPanelText::Backwards), REVERT_TO_COMMAND_ACTION_ID, RowActionPlacement::Menu)?;
            builder = builder.try_row_action(revert).map_err(|_| ui_assembly_error("history-panel.row-actions"))?.target(row_target(controller_id, Some(UiValue::Map(args.finish())), None)?);
        }
        let blocking = |mutation: &semio_framework::kernel::HistoryMutationEntry| mutation.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst));
        let flagged = |mutation: &semio_framework::kernel::HistoryMutationEntry| mutation.worst.is_some() || mutation.edited || mutation.superseded;
        let mut shown: Vec<&semio_framework::kernel::HistoryMutationEntry> = mutations.iter().filter(|mutation| blocking(*mutation)).collect();
        shown.extend(mutations.iter().filter(|mutation| flagged(*mutation) && !blocking(*mutation)));
        shown.extend(mutations.iter().filter(|mutation| !flagged(*mutation)));
        let (projected, total) = time_travel::history_row_mutation_extent(entry);
        if total == 0 {
            return builder.try_build().map_err(|_| ui_assembly_error("history-panel.command-build"));
        }
        let page = pages.get(&entry.seq);
        tree_window_indexed_item(&windows, builder, id.as_str(), time_travel::history_row_opens_by_default(&mutations), total, |index| {
            let paged;
            let mutation = match shown.get(index) {
                Some(mutation) => *mutation,
                None => match page.and_then(|page| page.rows.get(index.checked_sub(projected + page.from)?)) {
                    Some(view) => {
                        paged = history_mutation_entry(view, time_travel);
                        &paged
                    }
                    None => return history_panel_unavailable_row(entry.seq, index, locale),
                },
            };
            history_panel_mutation_row(mutation, controller_id, locale, read_only, time_travel::mutation_row_refusals(time_travel, busy, &mutation.mutation_id, mutation.store.as_deref()))
        })
    })?;

    let checkin_item = if read_only {
        None
    } else {
        let pending = uncommitted_command_count(history);
        let text = text_of(time_travel::HistoryPanelText::CheckIn);
        let text = if pending > 0 { format!("{text} ({pending})") } else { text.to_string() };
        let icon = ui_text(IconName::GitCommit.as_str(), "history-panel.checkin-icon")?;
        let submit = ActionId::try_v1(FRAMEWORK_CHECKIN_CONTROLLER_ID, "submit").ok_or_else(|| ui_assembly_error("history-panel.checkin-action"))?;
        let builder = button(ui_label(text.as_str(), "history-panel.checkin-label")?).icon(icon.clone()).disabled(frozen);
        let builder = builder.try_id("s-checkin").map_err(|_| ui_assembly_error("history-panel.checkin-button-id"))?;
        let trigger = builder.try_on(Trigger::Activate, submit).map_err(|_| ui_assembly_error("history-panel.checkin-binding"))?.try_build().map_err(|_| ui_assembly_error("history-panel.checkin-button-build"))?;
        let builder = tree_item(ui_label(text.as_str(), "history-panel.checkin-item-label")?).icon(icon).disabled(frozen);
        let builder = builder.try_id("framework.history.checkin").map_err(|_| ui_assembly_error("history-panel.checkin-item-id"))?;
        Some(builder.try_child(trigger).map_err(|_| ui_assembly_error("history-panel.checkin-item-child"))?.try_build().map_err(|_| ui_assembly_error("history-panel.checkin-item-build"))?)
    };
    let mutable = !read_only && !frozen;
    let mut action_items = BuiltChildren::default();
    for item in [
        Some(action_item("framework.history.undo", IconName::Undo, text_of(time_travel::HistoryPanelText::Undo), "undo", mutable)?),
        Some(action_item("framework.history.redo", IconName::Redo, text_of(time_travel::HistoryPanelText::Redo), "redo", history.can_redo && mutable)?),
        Some(action_item("framework.history.commitCheckpoint", IconName::GitCommit, text_of(time_travel::HistoryPanelText::CommitCheckpoint), "commitCheckpoint", mutable)?),
        checkin_item,
        Some(action_item("framework.history.createAlternative", IconName::GitBranch, text_of(time_travel::HistoryPanelText::CreateAlternative), "createAlternative", mutable)?),
        Some(filter_item),
    ]
    .into_iter()
    .flatten()
    {
        action_items.try_push(item).map_err(|_| ui_assembly_error("history-panel.actions"))?;
    }
    let actions_builder = tree_section(ui_label(text_of(time_travel::HistoryPanelText::Actions), "history-panel.actions-label")?).default_open(true);
    let actions_builder = actions_builder.try_id("framework.history.actions").map_err(|_| ui_assembly_error("history-panel.actions-id"))?;
    let actions_section = actions_builder.try_children(action_items).map_err(|_| ui_assembly_error("history-panel.actions"))?.try_build().map_err(|_| ui_assembly_error("history-panel.actions-build"))?;
    let alternatives_section = if history.alternatives.is_empty() {
        None
    } else {
        Some(tree_window_section(&windows, "framework.history.alternatives", ui_label(text_of(time_travel::HistoryPanelText::Alternatives), "history-panel.alternatives-label")?, true, &history.alternatives, |alternative| {
            history_panel_alternative_row(alternative, controller_id, locale, mutable)
        })?)
    };
    let mut sections = BuiltChildren::default();
    let remote_section = match reprojection {
        Some(remote) => Some(time_travel::time_travel_reprojection_section(remote, time_travel.is_none() && !read_only, controller_id, locale)?),
        None => None,
    };
    let ordered: Vec<BuiltNode> = match time_travel {
        Some(panel) => {
            let mut session = vec![time_travel::time_travel_band_section(panel, controller_id, locale)?];
            if let Some(editor) = panel.editor.as_ref().filter(|_| !read_only) {
                session.extend(time_travel::time_travel_editor_sections(panel, editor, &windows, controller_id, locale)?);
            }
            session.into_iter().chain(remote_section).chain(alternatives_section).chain([actions_section, commands_section]).collect()
        }
        None => remote_section.into_iter().chain([actions_section]).chain(alternatives_section).chain([commands_section]).collect(),
    };
    for section in ordered {
        sections.try_push(section).map_err(|_| ui_assembly_error("history-panel.sections"))?;
    }
    let mut panel = tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_assembly_error("history-panel.build"))?;
    panel.layout = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, sizing: Sizing::Fill, ..Default::default() });
    Ok(panel)
}

/// 🌿️ One alternative row: its name (the trunk's empty one reads "Main line"), a check icon and "Current" when it is the
/// current one, who branched it and when,
/// and "Edited history" when history edits were kept as it; every other alternative switches on activation and by its
/// Switch row action (`switchAlternative{alternativeId}`) unless the panel is read-only or frozen by a history edit.
fn history_panel_alternative_row(alternative: &AlternativeView, controller_id: &str, locale: Locale, mutable: bool) -> UiAssemblyResult<BuiltNode> {
    let text = |text: time_travel::HistoryPanelText| text.text(locale);
    let id = UiText::try_format(format_args!("framework.history.alternative.{}", alternative.id)).ok_or_else(|| ui_assembly_error("history-panel.alternative-id"))?;
    let icon = if alternative.current { IconName::Check } else { IconName::GitBranch };
    let name = if alternative.name.is_empty() { text(time_travel::HistoryPanelText::Trunk) } else { alternative.name.as_str() };
    let mut builder = tree_item(Label(UiText::clipped(name))).icon(ui_text(icon.as_str(), "history-panel.alternative-icon")?);
    builder = builder.try_id(&id).map_err(|_| ui_assembly_error("history-panel.alternative-id"))?;
    let mut parts: Vec<String> = Vec::new();
    if alternative.current {
        parts.push(text(time_travel::HistoryPanelText::Current).to_string());
    }
    if let (Some(author), Some(at)) = (alternative.author.as_deref(), alternative.branched_at.as_deref()) {
        let when = at.get(..16).map_or_else(|| at.to_string(), |minute| format!("{} UTC", minute.replacen('T', " ", 1)));
        parts.push(text(time_travel::HistoryPanelText::BranchedBy).replace("{author}", author).replace("{time}", &when));
    }
    if alternative.edited {
        parts.push(text(time_travel::HistoryPanelText::EditedHistory).to_string());
    }
    if !parts.is_empty() {
        builder = builder.description(UiText::clipped(&parts.join(" \u{b7} ")));
    }
    if !alternative.current && mutable {
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.alternative-args"))?;
        args.push(semio_framework::SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID.to_owned(), UiValue::Text(UiText::clipped(&alternative.id))).map_err(|_| ui_assembly_error("history-panel.alternative-args"))?;
        let switch = row_action(IconName::GitBranch.as_str(), text(time_travel::HistoryPanelText::Switch), semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, RowActionPlacement::Row)?;
        builder = builder.try_row_action(switch).map_err(|_| ui_assembly_error("history-panel.alternative-actions"))?.target(row_target(controller_id, Some(UiValue::Map(args.finish())), Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID))?);
    }
    builder.try_build().map_err(|_| ui_assembly_error("history-panel.alternative-build"))
}

/// ⌛️ The placeholder child at `index` of history row `seq` whose page holds no row there (a window that moved past the
/// page this render read, audit W2A-10): a dimmed, actionless row naming that the mutation is not available, never a
/// failed history body.
fn history_panel_unavailable_row(seq: u64, index: usize, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let id = UiText::try_format(format_args!("{}{seq}.unavailable.{index}", time_travel::HISTORY_ROW_KEY_PREFIX)).ok_or_else(|| ui_assembly_error("history-panel.unavailable-id"))?;
    let builder = tree_item(Label(UiText::clipped(time_travel::HistoryPanelText::MutationUnavailable.text(locale)))).icon(ui_text("clock", "history-panel.unavailable-icon")?).dimmed(true);
    builder.try_id(&id).map_err(|_| ui_assembly_error("history-panel.unavailable-id"))?.try_build().map_err(|_| ui_assembly_error("history-panel.unavailable-build"))
}

/// ✏️ One mutation child of a history row: label, state-and-severity description in words, tone and icon by
/// severity, and its row actions on the one row target `{mutationId, store?}` — `store` names the composed member
/// store a child mutation lives in (design §12): Edit (`historyEditBegin`, also the row's activation), then Withdraw
/// (`historyEditWithdraw`, design §22.1) on every mutation that is not withdrawn — or, on a mutation holding an
/// accepted draft of the open session, Restore (`historyEditRestore`) in its place. An action a viewer, a mutation
/// without editable inputs (design §22.20), the store's supersede law, the session or the instance refuses is disabled
/// because of that refusal (`RowAction::disabled_because`, announced as its description; `refusals`), and a refused
/// Edit activates nothing (gap N15).
fn history_panel_mutation_row(mutation: &semio_framework::kernel::HistoryMutationEntry, controller_id: &str, locale: Locale, read_only: bool, refusals: time_travel::MutationRowRefusals) -> UiAssemblyResult<BuiltNode> {
    let (tone, icon) = time_travel::history_severity_tone(mutation.worst);
    let icon = if mutation.pending {
        "clock"
    } else if mutation.edited {
        "edit"
    } else {
        icon
    };
    let id = UiText::try_format(format_args!("framework.history.mutation.{}", mutation.mutation_id)).ok_or_else(|| ui_assembly_error("history-panel.mutation-id"))?;
    let mut builder = tree_item(Label(UiText::clipped(mutation.label.resolve(Terminology::Native, locale)))).icon(ui_text(icon, "history-panel.mutation-icon")?).tone(if mutation.worst.is_some() { tone } else { Tone::Neutral });
    builder = builder.try_id(&id).map_err(|_| ui_assembly_error("history-panel.mutation-id"))?;
    let description = time_travel::history_mutation_description(mutation, locale);
    if !description.is_empty() {
        builder = builder.description(UiText::clipped(&description));
    }
    if mutation.pending || mutation.withdrawn {
        builder = builder.dimmed(true);
    }
    let action = |icon: &str, text: time_travel::HistoryPanelText, verb: &str, refusal: Option<semio_framework_time_travel::TimeTravelLabel>| -> UiAssemblyResult<RowAction> {
        let action = row_action(icon, text.text(locale), verb, RowActionPlacement::Row)?;
        let Some(refusal) = refusal else { return Ok(action) };
        let reason = refusal.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
        Ok(action.disabled_because(ui_label(reason.as_str(), "history-panel.mutation-action-reason")?))
    };
    let viewer = read_only.then_some(semio_framework_time_travel::TimeTravelLabel::RefusalReadOnly);
    let begin = viewer.or((!mutation.editable).then_some(semio_framework_time_travel::TimeTravelLabel::RefusalNotEditable)).or(refusals.begin);
    let mut actions = vec![action(IconName::Edit.as_str(), time_travel::HistoryPanelText::Edit, semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, begin)?];
    if refusals.accepted {
        actions.push(action("undo-2", time_travel::HistoryPanelText::Restore, semio_framework::HISTORY_EDIT_RESTORE_ACTION_ID, refusals.restore)?);
    } else if !mutation.withdrawn {
        let withdraw = viewer.or((!mutation.withdrawable).then_some(semio_framework_time_travel::TimeTravelLabel::RefusalNotWithdrawable)).or(refusals.withdraw);
        actions.push(action("eye-off", time_travel::HistoryPanelText::Withdraw, semio_framework::HISTORY_EDIT_WITHDRAW_ACTION_ID, withdraw)?);
    }
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.mutation-args"))?;
    args.push(semio_framework::HISTORY_EDIT_ARG_MUTATION_ID.to_owned(), UiValue::Text(UiText::clipped(&mutation.mutation_id))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
    if let Some(store) = mutation.store.as_deref() {
        args.push(semio_framework::HISTORY_EDIT_ARG_STORE.to_owned(), UiValue::Text(UiText::clipped(store))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
    }
    for action in actions {
        builder = builder.try_row_action(action).map_err(|_| ui_assembly_error("history-panel.mutation-actions"))?;
    }
    let activation = begin.is_none().then_some(semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID);
    builder = builder.target(row_target(controller_id, Some(UiValue::Map(args.finish())), activation)?);
    builder.try_build().map_err(|_| ui_assembly_error("history-panel.mutation-build"))
}

/// ✅️ Whether ONE ledger row's edit is still live, as the host reads it: `dimmed: applied ===
/// false` in the History panel and `entry.kind === "mutation" && entry.applied !== false` in
/// `#s-checkin`'s uncommitted count.
///
/// A row publishes into the parent document store and its children; a config-lane edit is never part of a row (design
/// §20.13, L4). `undo`/`redo` dispatch against the PARENT document store alone (`interaction_store`'s own doc says so), so
/// the parent lane is authoritative for any row that published a parent edit, and a row with NO parent edit (🌊️flow's
/// `addWidget`, 🎬️sequence's `addStep` on the child lane) answers with its children.
///
/// Named rather than inlined so the law can drive every lane combination without an app (ticket 26/09/18 S11 §3.4b).
pub fn history_row_applied(has_parent_edit: bool, document_applied: bool, child_applied: bool) -> bool {
    if has_parent_edit {
        document_applied
    } else {
        child_applied
    }
}
