//! 🎮️ The command log and history views, and the history panel.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
use semio_framework_plugin_decl_views::history_view::InverseAction;
use semio_framework_plugin_decl_views::history_view::CommandView;
use semio_framework_plugin_decl_views::history_view::MutationView;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_decl_views::history_view::AlternativeView;
use semio_framework::{ActionKind, IconName};
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
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_window_kits_rich::table_kit::row_action;
use semio_framework_plugin_window_kits_rich::table_kit::row_target;
use crate::lifted_command::HistoryMutationPages;
use crate::lifted_command::TimeTravelPanel;
use crate::lifted_command::history_mutation_entry;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_interaction::ui_assembly_error;
use semio_framework_plugin_window_transient::ui_label;
use semio_framework_plugin_window_transient::ui_text;

/// 🗝️ What a backfilled command-log row references: a document edit (with the verb that authored it) or a
/// `Supersede` history transition.
#[doc(hidden)]
pub enum CommandLogAppendKey {
    Edit { edit_id: String, verb: Option<String>, child_edit_ids: Vec<String> },
    Members { edit_ids: Vec<String> },
    Transition(String),
}

/// 🧭️ What a cached [`HistoryView`] was built against (design §20.14): the document's and every composed member's
/// [`time_travel::HistoryStoreStamp`], the newest command-log row and the row count it held, its shell rows and their
/// undone count, and the supersede records — what lets the next refresh rebuild only the rows a change touched.
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryViewStamp {
    pub document: crate::lifted_command::HistoryStoreStamp,
    pub members: Vec<(String, crate::lifted_command::HistoryStoreStamp)>,
    pub newest_seq: u64,
    pub rows: usize,
    pub inverse_rows: usize,
    pub shell_undone: usize,
    pub supersede_records: usize,
}

/// 🗂️ The history view a refresh patches in place, with the stamp it was built against; the projection cache shares it.
#[doc(hidden)]
pub struct HistoryViewCache {
    pub view: std::sync::Arc<HistoryView>,
    pub stamp: HistoryViewStamp,
}

/// 🩹️ The rows one refresh rebuilds in a cached [`HistoryView`]: the log rows from `fresh_from` on are new, `rebuild`
/// names the held rows a change touched, `positions` the applied position of every document edit it touched.
#[doc(hidden)]
pub struct HistoryViewPatch {
    pub fresh_from: usize,
    pub rebuild: BTreeSet<u64>,
    pub positions: Vec<(String, Option<usize>)>,
}

/// 🧾️ How far the command log is proven to reference every edit (`backfill_command_log`): the document ledger's length
/// and its last edit id, each composed member ledger's, the supersede records, and the newest row at that moment — the
/// next backfill reads only what came after (design §20.14).
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryBackfillMark {
    pub edits: usize,
    pub last_edit: Option<String>,
    pub members: Vec<(String, usize, Option<String>)>,
    pub supersede_records: usize,
    pub log_seq: u64,
}

#[cfg(any(test, feature = "artifact-app-testing"))]
thread_local! {
    /// 🧮️ History rows built on this test thread — what the O(change) laws of the history view count (design §20.14).
    pub static HISTORY_ROWS_BUILT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 📚️ What the history rows of one build or patch read beyond their command-log entries: the applied position of the
/// edits it knows (`full`: every applied edit, so an unknown one is not applied), the edits by id (`full` only; a patch
/// reads the ledger's tail), the previous projection of each row and its labels, the composed members' edits, the
/// members' applied tails, the history edits in effect and the app instance id.
#[doc(hidden)]
pub struct HistoryRowReads<'a, Mu> {
    pub full: bool,
    pub positions: HashMap<&'a str, Option<usize>>,
    pub edits: HashMap<&'a str, &'a protocol::Edit<Mu>>,
    pub previous: HashMap<u64, &'a CommandView>,
    pub labelled: HashMap<&'a str, &'a MutationView>,
    pub members: HashMap<String, crate::lifted_command::MemberEditHistory>,
    pub child_applied_tails: HashSet<String>,
    pub applied_entries: HashSet<usize>,
    pub app_id: String,
}

/// 🏷️ A backfilled edit row's label: the registry label of the verb that authored it, else its first operation's label
/// `first` (a document leaf's localized `SemanticMutation::label`) — the rule a live row records; an edit without
/// operations is named by its id. An edit stores no description (design §20.6).
#[doc(hidden)]
pub fn backfilled_edit_label<Mu>(edit: &protocol::Edit<Mu>, verb_label: Option<LocalizedLabel>, first: impl FnOnce(&Mu) -> LocalizedLabel) -> LocalizedLabel {
    verb_label.unwrap_or_else(|| edit.forwards.first().map_or_else(|| LocalizedLabel::data(edit.id.clone()), first))
}

/// 🧾️ Inputs for one appended session command before its sequence and timestamp are assigned.
#[doc(hidden)]
pub struct CommandLogAppend<'a> {
    pub action_id: &'a str,
    pub label: LocalizedLabel,
    pub kind: ActionKind,
    pub edit_id: Option<String>,
    pub transition_id: Option<String>,
    pub timestamp: Option<String>,
    pub inverse: Option<InverseAction>,
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
#[doc(hidden)]
pub fn history_intent_label_of<P, Mu: protocol::SemanticMutation<P>>(intents: &[&str], ops: &[&store::AppliedMutation<'_, Mu>], mutations: &[MutationView]) -> Option<LocalizedLabel> {
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
#[doc(hidden)]
pub fn uncommitted_command_count(history: &HistoryView) -> u32 {
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
#[doc(hidden)]
pub fn history_panel_icon_id(kind: ActionKind) -> IconName {
    match kind {
        ActionKind::Mutation => IconName::Pencil,
        ActionKind::History => IconName::Undo,
        ActionKind::Clipboard => IconName::Clipboard,
        ActionKind::View => IconName::Eye,
        ActionKind::Shell => IconName::Monitor,
        ActionKind::Interaction => "mouse-pointer".into(),
    }
}

/// 🌿️ One alternative row: its name (the trunk's empty one reads "Main line"), a check icon and "Current" when it is the
/// current one, who branched it and when,
/// and "Edited history" when history edits were kept as it; every other alternative switches on activation and by its
/// Switch row action (`switchAlternative{alternativeId}`) unless the panel is read-only or frozen by a history edit.
#[doc(hidden)]
pub fn history_panel_alternative_row(alternative: &AlternativeView, controller_id: &str, locale: Locale, mutable: bool) -> UiAssemblyResult<BuiltNode> {
    let text = |text: crate::lifted_command::HistoryPanelText| text.text(locale);
    let id = UiText::try_format(format_args!("framework.history.alternative.{}", alternative.id)).ok_or_else(|| ui_assembly_error("history-panel.alternative-id"))?;
    let icon = if alternative.current { IconName::Check } else { IconName::GitBranch };
    let name = if alternative.name.is_empty() { text(crate::lifted_command::HistoryPanelText::Trunk) } else { alternative.name.as_str() };
    let mut builder = tree_item(Label(UiText::clipped(name))).icon(ui_text(icon.as_str(), "history-panel.alternative-icon")?);
    builder = builder.try_id(&id).map_err(|_| ui_assembly_error("history-panel.alternative-id"))?;
    let mut parts: Vec<String> = Vec::new();
    if alternative.current {
        parts.push(text(crate::lifted_command::HistoryPanelText::Current).to_string());
    }
    if let (Some(author), Some(at)) = (alternative.author.as_deref(), alternative.branched_at.as_deref()) {
        let when = at.get(..16).map_or_else(|| at.to_string(), |minute| format!("{} UTC", minute.replacen('T', " ", 1)));
        parts.push(text(crate::lifted_command::HistoryPanelText::BranchedBy).replace("{author}", author).replace("{time}", &when));
    }
    if alternative.edited {
        parts.push(text(crate::lifted_command::HistoryPanelText::EditedHistory).to_string());
    }
    if !parts.is_empty() {
        builder = builder.description(UiText::clipped(&parts.join(" \u{b7} ")));
    }
    if !alternative.current && mutable {
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.alternative-args"))?;
        args.push(semio_framework::SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID.to_owned(), UiValue::Text(UiText::clipped(&alternative.id))).map_err(|_| ui_assembly_error("history-panel.alternative-args"))?;
        let switch = row_action(IconName::GitBranch.as_str(), text(crate::lifted_command::HistoryPanelText::Switch), semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, RowActionPlacement::Row)?;
        builder = builder.try_row_action(switch).map_err(|_| ui_assembly_error("history-panel.alternative-actions"))?.target(row_target(controller_id, Some(UiValue::Map(args.finish())), Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID))?);
    }
    builder.try_build().map_err(|_| ui_assembly_error("history-panel.alternative-build"))
}

/// ⌛️ The placeholder child at `index` of history row `seq` whose page holds no row there (a window that moved past the
/// page this render read, audit W2A-10): a dimmed, actionless row naming that the mutation is not available, never a
/// failed history body.
#[doc(hidden)]
pub fn history_panel_unavailable_row(seq: u64, index: usize, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let id = UiText::try_format(format_args!("{}{seq}.unavailable.{index}", crate::lifted_command::HISTORY_ROW_KEY_PREFIX)).ok_or_else(|| ui_assembly_error("history-panel.unavailable-id"))?;
    let builder = tree_item(Label(UiText::clipped(crate::lifted_command::HistoryPanelText::MutationUnavailable.text(locale)))).icon(ui_text("clock", "history-panel.unavailable-icon")?).dimmed(true);
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
#[doc(hidden)]
pub fn history_panel_mutation_row(mutation: &semio_framework::kernel::HistoryMutationEntry, controller_id: &str, locale: Locale, read_only: bool, refusals: crate::lifted_command::MutationRowRefusals) -> UiAssemblyResult<BuiltNode> {
    let (tone, icon) = crate::lifted_command::history_severity_tone(mutation.worst);
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
    let description = crate::lifted_command::history_mutation_description(mutation, locale);
    if !description.is_empty() {
        builder = builder.description(UiText::clipped(&description));
    }
    if mutation.pending || mutation.withdrawn {
        builder = builder.dimmed(true);
    }
    let action = |icon: &str, text: crate::lifted_command::HistoryPanelText, verb: &str, refusal: Option<semio_framework_time_travel::TimeTravelLabel>| -> UiAssemblyResult<RowAction> {
        let action = row_action(icon, text.text(locale), verb, RowActionPlacement::Row)?;
        let Some(refusal) = refusal else { return Ok(action) };
        let reason = refusal.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
        Ok(action.disabled_because(ui_label(reason.as_str(), "history-panel.mutation-action-reason")?))
    };
    let viewer = read_only.then_some(semio_framework_time_travel::TimeTravelLabel::RefusalReadOnly);
    let begin = viewer.or((!mutation.editable).then_some(semio_framework_time_travel::TimeTravelLabel::RefusalNotEditable)).or(refusals.begin);
    let mut actions = vec![action(IconName::Edit.as_str(), crate::lifted_command::HistoryPanelText::Edit, semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID, begin)?];
    if refusals.accepted {
        actions.push(action("undo-2", crate::lifted_command::HistoryPanelText::Restore, semio_framework::history_edit::HISTORY_EDIT_RESTORE_ACTION_ID, refusals.restore)?);
    } else if !mutation.withdrawn {
        let withdraw = viewer.or((!mutation.withdrawable).then_some(semio_framework_time_travel::TimeTravelLabel::RefusalNotWithdrawable)).or(refusals.withdraw);
        actions.push(action("eye-off", crate::lifted_command::HistoryPanelText::Withdraw, semio_framework::history_edit::HISTORY_EDIT_WITHDRAW_ACTION_ID, withdraw)?);
    }
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.mutation-args"))?;
    args.push(semio_framework::history_edit::HISTORY_EDIT_ARG_MUTATION_ID.to_owned(), UiValue::Text(UiText::clipped(&mutation.mutation_id))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
    if let Some(store) = mutation.store.as_deref() {
        args.push(semio_framework::history_edit::HISTORY_EDIT_ARG_STORE.to_owned(), UiValue::Text(UiText::clipped(store))).map_err(|_| ui_assembly_error("history-panel.mutation-args"))?;
    }
    for action in actions {
        builder = builder.try_row_action(action).map_err(|_| ui_assembly_error("history-panel.mutation-actions"))?;
    }
    let activation = begin.is_none().then_some(semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID);
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

#[path = "📥️admission/🦀️.rs"]
pub mod command_admission;
#[path = "🛬️lifted/⏪️time-travel/🦀️.rs"]
pub mod lifted_command;
