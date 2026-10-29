//! ⏪️ Lift file: 50 item(s), about 832 lines, of `⏪️time-travel/🦀️.rs` that belong to crate `command` (WP3): the history view vocabulary the instance command routes read (mutation pages, member histories and view stamps) together with the history wire row, its panel view model and the panel texts.
//!
//! Cut by WP5; WP3 wires this file with one `mod` line in the crate root at the flip. Until `command` exists, `⏪️time-travel/🦀️.rs` mounts it as a child module and re-exports it with `pub use`.

use semio_framework::{ActionArgDef, AppRole, ArgPresentation, ArgSchema, FaultFrom, TreeWindowRequest, mutation_input_defs, registered_input_schema_document};
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, Mutation, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use semio_framework_plugin_decl_views::history_view::{CommandView, MutationView};
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::{HISTORY_ROW_OPERATION_PREVIEW, history_intent_label_of, history_leaf_row_label};
use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryTimeTravel, HybridLogicalTimestamp};
#[cfg(test)]
use semio_framework_os_kernel::HistoryPageStack;
use semio_framework_ui_locale::{Locale, LocalizedLabel};
use protocol::{OpBinary, OpText};
use semio_framework_time_travel::{TimeTravelLabel, TimeTravelRefusal, TimeTravelReview, TimeTravelStage};
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;

//#region 🔖️Limits
/// ✏️ Mutation rows one history row carries: the first ones, then every one with messages or a supersession.
pub const HISTORY_ROW_MUTATION_ROWS: usize = 32;
//#endregion 🔖️Limits

//#region 🔖️PanelView
/// ✏️ The draft editor as the panel renders it: its rows ([`time_travel_input_rows`]) over the current draft, whether
/// the mutation's inputs can be edited at all (`editable`: an editor a row's Withdraw opened on a mutation without them
/// holds none), and the entity label of every id its reference rows name, as the app reads them in the previewed
/// document (`ArtifactApp::entity_label`; an id without one shows as itself).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelEditorPanel {
    pub target: String,
    pub label: LocalizedLabel,
    pub rows: Vec<TimeTravelInputRow>,
    pub editable: bool,
    pub inputs_refused: Option<String>,
    pub withdrawn: bool,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
    pub changed: bool,
    pub reference_labels: BTreeMap<String, LocalizedLabel>,
}

/// 🪧️ The live session as the history panel and the history wire read it: the band status, the composed member store it
/// edits (`None` for the document's own), which mutations are pending (downstream of the edited one in that store while
/// editing), edited (a draft, accepted or pending) or `accepted` (an accepted draft a row's Restore takes back), the
/// replay outcomes that override the durable ones, why finalizing is refused, the first blocking mutation, and the
/// draft editor.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelPanel {
    pub status: HistoryTimeTravel,
    pub store: Option<String>,
    pub stage: TimeTravelStage,
    pub pending_after: Option<(u32, u32)>,
    pub finalize_refusal: Option<TimeTravelRefusal>,
    pub review: Option<TimeTravelReview>,
    pub rerun_refusal: Option<TimeTravelRefusal>,
    pub begin_refusal: Option<TimeTravelRefusal>,
    pub next_problem: Option<String>,
    pub editor: Option<TimeTravelEditorPanel>,
    pub outcomes: BTreeMap<String, protocol::MutationReplayOutcome>,
    pub edited: BTreeSet<String>,
    pub accepted: BTreeSet<String>,
}

/// 🚥️ What the action controls of one mutation row answer in the open session and instance: why Edit (`begin`) and
/// Withdraw are refused (`None`: offered), whether the mutation holds an accepted draft — its row then offers Restore
/// in Withdraw's place — and why Restore is refused.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MutationRowRefusals {
    pub begin: Option<TimeTravelLabel>,
    pub withdraw: Option<TimeTravelLabel>,
    pub accepted: bool,
    pub restore: Option<TimeTravelLabel>,
}

/// 🧾️ One mutation row on the history wire: the durable row with the session overlay (replay outcome, pending,
/// edited, introduced) when a history edit is open. `introduced` marks a replay outcome carrying a message (level and
/// code) the durable pre-edit outcome of the same mutation does not ([`history_outcome_introduced`]); a mutation the
/// session's replay withdrew is no longer `withdrawable`.
pub fn history_mutation_entry(view: &MutationView, panel: Option<&TimeTravelPanel>) -> HistoryMutationEntry {
    let mut worst = view.worst;
    let mut messages = &view.messages;
    let (mut superseded, mut withdrawn) = (view.superseded, view.withdrawn);
    let (mut pending, mut edited, mut introduced) = (false, false, false);
    if let Some(panel) = panel {
        if let Some(outcome) = panel.outcomes.get(&view.mutation_id) {
            worst = outcome.worst;
            messages = &outcome.messages;
            superseded = outcome.superseded;
            withdrawn = outcome.withdrawn;
            introduced = history_outcome_introduced(&view.messages, &outcome.messages);
        }
        pending = view.store == panel.store && panel.pending_after.is_some_and(|after| (view.position, view.op_index) > after);
        edited = panel.edited.contains(&view.mutation_id);
    }
    HistoryMutationEntry {
        mutation_id: view.mutation_id.clone(),
        position: view.position,
        op_index: view.op_index,
        label: view.label.clone(),
        worst,
        messages: messages.iter().map(|message| HistoryMutationMessage { level: message.level, code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone(), op_index: message.op_index }).collect(),
        superseded,
        withdrawn,
        editable: view.editable,
        withdrawable: view.withdrawable && !withdrawn,
        pending,
        edited,
        introduced,
        store: view.store.clone(),
    }
}

/// 🆕️ Whether a replayed outcome carries a message whose level and code the durable pre-edit outcome of the same
/// mutation does not — the outcome an edit made new (design §16.5).
pub fn history_outcome_introduced(before: &[protocol::MutationMessage], replayed: &[protocol::MutationMessage]) -> bool {
    replayed.iter().any(|message| !before.iter().any(|earlier| earlier.level == message.level && earlier.code == message.code))
}
//#endregion 🔖️PanelView

//#region 🔖️Pointer
/// 🎛️ One row of the draft editor: the RFC 6901 pointer it writes, its label (with the path of labels and item numbers
/// that lead to it), its descriptor and its current value; a list input's own row carries its [`TimeTravelList`], and
/// the first row of a list item its [`TimeTravelListItem`].
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelInputRow {
    pub pointer: String,
    pub label: LocalizedLabel,
    pub input: ActionArgDef,
    pub value: DslValue,
    pub list: Option<TimeTravelList>,
    pub item: Option<TimeTravelListItem>,
}

/// 📋️ A list input's row: how many items it holds, whether one more is admitted, and its `maxItems`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeTravelList {
    pub len: usize,
    pub addable: bool,
    pub max: Option<u32>,
}

/// 🔢️ The first row of one list item: its index, whether removing it is admitted, and the list's `minItems`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeTravelListItem {
    pub index: usize,
    pub removable: bool,
    pub min: Option<u32>,
}
//#endregion 🔖️Pointer

//#region 🔖️Driver
#[doc(hidden)]
pub fn time_travel_arg_text<'a>(args: Option<&'a DslValue>, key: &str) -> Option<&'a str> {
    args?.get(key)?.as_str()
}

/// 🪨️ Whether the store's supersede law admits withdrawing `operation` (`store::os_store::replay::admit_replacement`, the one law every
/// authoring, ingest, load and fold site reads): what a mutation row offers its Withdraw by and what a row's Withdraw
/// opens by (design §22.1). A withdrawal names no schema, so the law is asked with none.
#[doc(hidden)]
pub fn time_travel_admits_withdrawal<P, Mu>(operation: &Mu) -> bool
where
    Mu: ::semio_framework_os_kernel::StoreMutationFor<P>,
{
    store::os_store::replay::admit_replacement::<P, Mu>(operation, &protocol::InputReplacement::Withdrawn, "").is_ok()
}

thread_local! {
    /// 🫙️ Whether a payload schema (by the address and length of its static text) describes an input an editor shows as a
    /// row ([`time_travel_schema_shows_inputs`]): read once per schema, since every mutation row asks.
    static TIME_TRAVEL_SCHEMA_SHOWS_INPUTS: std::cell::RefCell<HashMap<(usize, usize), bool>> = std::cell::RefCell::new(HashMap::new());
}

/// 👁️ Whether the payload schema `schema` describes at least one input a draft editor shows as a row (design §22.20: a
/// mutation whose inputs are all hidden is not editable, it keeps Withdraw): an input that is not hidden — an object one
/// through its fields ([`time_travel_input_rows`]). A schema that cannot be read answers `true` and is not remembered:
/// its editor opens naming why (the referenced documents may register later).
#[doc(hidden)]
pub fn time_travel_schema_shows_inputs(schema: &'static str) -> bool {
    fn shown(input: &ActionArgDef) -> bool {
        input.presentation != Some(ArgPresentation::Hidden) && !matches!(&input.schema, ArgSchema::Object { fields } if !fields.is_empty() && !fields.iter().any(shown))
    }
    let key = (schema.as_ptr() as usize, schema.len());
    if let Some(known) = TIME_TRAVEL_SCHEMA_SHOWS_INPUTS.with(|cache| cache.borrow().get(&key).copied()) {
        return known;
    }
    let Ok(inputs) = mutation_input_defs(schema, &registered_input_schema_document) else { return true };
    let shows = inputs.iter().any(shown);
    TIME_TRAVEL_SCHEMA_SHOWS_INPUTS.with(|cache| cache.borrow_mut().insert(key, shows));
    shows
}

/// 🏷️ The history label of one document operation in every shell locale: its leaf's `SemanticMutation::label` — never the
/// operation's text line (design §16.2).
#[doc(hidden)]
pub fn time_travel_mutation_label<A: ArtifactApp>(op: &A::Mutation) -> LocalizedLabel {
    protocol::SemanticMutation::<A::Snapshot>::label(op)
}

/// ✏️ The mutation rows of one edit's applied `ops` (in op order): the first [`HISTORY_ROW_MUTATION_ROWS`], then at most as
/// many later ones carrying messages or a supersession, the most severe first — so the row's worst severity is always
/// among them (audit W2A-11) — labelled from their effective input (a label of `labelled`, the
/// previous projection, is reused for an operation that was not superseded) with their durable outcome. `superseded`
/// and `withdrawn` read the store's effective supersession: an input restored to its original (the undo of a history
/// edit) is not superseded.
#[doc(hidden)]
pub fn history_mutation_views<A: ArtifactApp>(ops: &[&store::AppliedMutation<'_, A::Mutation>], outcomes: &HashMap<&str, &protocol::MutationReplayOutcome>, labelled: &HashMap<&str, &MutationView>) -> Vec<MutationView> {
    history_mutation_views_of::<A::Snapshot, A::Mutation>(ops, outcomes, labelled, time_travel_mutation_label::<A>, A::ROLE == AppRole::Viewer, None)
}

/// 🧩️ [`history_mutation_views`] over any store's typed operations: the document's own (labelled by the app) or one
/// composed member's (labelled by its leaves, each row naming that member `store`, design §12). A `viewer` edits nothing.
fn history_mutation_views_of<P, Mu>(
    ops: &[&store::AppliedMutation<'_, Mu>],
    outcomes: &HashMap<&str, &protocol::MutationReplayOutcome>,
    labelled: &HashMap<&str, &MutationView>,
    label_of: impl Fn(&Mu) -> LocalizedLabel,
    viewer: bool,
    store: Option<&str>,
) -> Vec<MutationView>
where
    Mu: ::semio_framework_os_kernel::StoreMutationFor<P>,
{
    let outcome_of = |op: &store::AppliedMutation<'_, Mu>| outcomes.get(op.mutation_id.0.as_str()).copied();
    let flagged = ops.iter().enumerate().filter_map(|(index, op)| (op.supersession.is_some() || outcome_of(op).is_some_and(|outcome| outcome.worst.is_some())).then(|| (index, outcome_of(op).and_then(|outcome| outcome.worst))));
    history_projected_operations(ops.len(), flagged).into_iter().map(|index| history_mutation_view_of::<P, Mu>(ops[index], outcome_of(ops[index]), labelled, &label_of, viewer, store)).collect()
}

/// 🔢️ Which operations of an edit of `len` operations one history row projects (audit W2A-11): the first
/// [`HISTORY_ROW_MUTATION_ROWS`], then at most as many later `flagged` ones (index and worst severity — `None` for a
/// supersession without an outcome), the most severe first and the earliest among equals, all back in op order.
#[doc(hidden)]
pub fn history_projected_operations(len: usize, flagged: impl IntoIterator<Item = (usize, Option<semio_framework_diagnostic::Severity>)>) -> Vec<usize> {
    let mut later: Vec<(usize, Option<semio_framework_diagnostic::Severity>)> = flagged.into_iter().filter(|(index, _)| *index >= HISTORY_ROW_MUTATION_ROWS && *index < len).collect();
    later.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    later.truncate(HISTORY_ROW_MUTATION_ROWS);
    let mut shown: Vec<usize> = (0..len.min(HISTORY_ROW_MUTATION_ROWS)).chain(later.into_iter().map(|(index, _)| index)).collect();
    shown.sort_unstable();
    shown
}

/// 🧾️ One operation's mutation row — [`history_mutation_views_of`] without the row cap: labelled from its effective input
/// (a label of `labelled`, the previous projection, reused for an operation that was not superseded) with its durable
/// `outcome`; `superseded`/`withdrawn` read the store's effective supersession; `editable` needs a payload schema that
/// shows an input ([`time_travel_schema_shows_inputs`], design §22.20) and no foreign steps; `withdrawable` asks the
/// store's supersede law for an operation that is not withdrawn yet ([`time_travel_admits_withdrawal`]); an operation
/// of a cross-document unit (`AppliedMutation::unit`) is neither — it takes no supersession in one document alone; a
/// `viewer` edits and withdraws nothing.
#[doc(hidden)]
pub fn history_mutation_view_of<P, Mu>(
    op: &store::AppliedMutation<'_, Mu>,
    outcome: Option<&protocol::MutationReplayOutcome>,
    labelled: &HashMap<&str, &MutationView>,
    label_of: &impl Fn(&Mu) -> LocalizedLabel,
    viewer: bool,
    store: Option<&str>,
) -> MutationView
where
    Mu: ::semio_framework_os_kernel::StoreMutationFor<P>,
{
    let id = op.mutation_id.0.as_str();
    let effective = match op.supersession.map(|supersession| &supersession.replacement) {
        Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
        _ => None,
    };
    let shown = effective.as_ref().unwrap_or(op.operation);
    let label = match labelled.get(id).filter(|_| op.supersession.is_none()) {
        Some(previous) => previous.label.clone(),
        None => label_of(shown),
    };
    let editable = !viewer && op.unit.is_none() && !shown.may_emit_foreign_steps() && shown.input_schema().is_some_and(time_travel_schema_shows_inputs);
    let superseded = match op.supersession.map(|supersession| &supersession.replacement) {
        None => false,
        Some(protocol::InputReplacement::Withdrawn) => true,
        Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::encode_op(op.operation).map_or(true, |original| original != *payload),
    };
    if let Some(effective) = effective {
        effective.retire_cold();
    }
    let withdrawn = op.supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn);
    MutationView {
        mutation_id: id.to_string(),
        position: u32::try_from(op.position).unwrap_or(u32::MAX),
        op_index: op.op_index,
        label,
        worst: outcome.and_then(|outcome| outcome.worst),
        messages: outcome.map(|outcome| outcome.messages.clone()).unwrap_or_default(),
        superseded,
        withdrawn,
        editable,
        withdrawable: !viewer && !withdrawn && op.unit.is_none() && time_travel_admits_withdrawal::<P, Mu>(op.operation),
        store: store.map(str::to_string),
    }
}

//#region 🔖️MutationPages
/// 🗂️ The node key of the history body's commands section, the window every history row is a row of.
pub const HISTORY_COMMANDS_SECTION_KEY: &str = "framework.history.commands";
/// 🗝️ The node key prefix of one history row (`framework.history.entry.<seq>`), React's and wgpu's `HISTORY_ROW_KEY_PREFIX`.
pub const HISTORY_ROW_KEY_PREFIX: &str = "framework.history.entry.";

/// 📖️ The mutation rows of one history row past its projected ones, as one host window shows them: `rows` are the edit's
/// remaining operations from the `from`-th on, in op order (gap N1).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryMutationPage {
    pub from: usize,
    pub rows: Vec<MutationView>,
}

/// 📚️ The [`HistoryMutationPage`] of every history row whose window shows rows past its projection, keyed by the row's `seq`.
pub type HistoryMutationPages = BTreeMap<u64, HistoryMutationPage>;

/// 📏️ How many mutation rows a history row projects and how many it holds in all (gap N1): the projected ones (the first
/// [`HISTORY_ROW_MUTATION_ROWS`] of each edit and its flagged ones, its own and its composed members'), then every other
/// operation of its own edit in op order — or, for a row of composed-member edits alone (design §12, audit W2A-11), every
/// other operation of its member edits in their order. A row without an applied operation of its own (or, member-only,
/// of its members) holds nothing further.
pub fn history_row_mutation_extent(entry: &CommandView) -> (usize, usize) {
    let projected = entry.mutations.len();
    let own = entry.mutations.iter().filter(|mutation| mutation.store.is_none()).count();
    let shown = if entry.edit_id.is_some() { own } else { projected - own };
    let rest = if shown == 0 { 0 } else { entry.op_count.saturating_sub(shown) };
    (projected, projected + rest)
}

/// 🪟️ The window path of history row `seq`'s mutation group inside the commands section — what a host files its
/// `TreeWindowRequest` under.
pub fn history_row_window_path(seq: u64) -> String {
    format!("{HISTORY_COMMANDS_SECTION_KEY}{TREE_WINDOW_PATH_SEPARATOR}{HISTORY_ROW_KEY_PREFIX}{seq}")
}

/// 🔓️ Whether a history row opens by default: while one of its mutation rows (session overlay included) carries an
/// outcome, so a warning or a problem stays visible in history without expanding anything.
pub fn history_row_opens_by_default(mutations: &[HistoryMutationEntry]) -> bool {
    mutations.iter().any(|mutation| mutation.worst.is_some())
}

/// 🔢️ The logical mutation rows `[start, end)` a history row's window materialises at most over `total` rows — the slice
/// `TreeWindows` cuts: a host `request` (its `open`, else `default_open`) at its clamped offset, or, unrequested and open
/// by default, the first rows the shared first-paint budget (`viewport_rows`) can hold. `None` while the row is closed.
pub fn history_row_window_rows(request: Option<&TreeWindowRequest>, default_open: bool, viewport_rows: usize, total: usize) -> Option<(usize, usize)> {
    if total == 0 || !request.and_then(|request| request.open).unwrap_or(default_open) {
        return None;
    }
    let Some(request) = request else { return Some((0, total.min(UI_BUILT_CHILDREN_MAX).min(viewport_rows))) };
    let rows = (request.rows as usize).min(UI_BUILT_CHILDREN_MAX);
    let start = (request.offset as usize).min(total.saturating_sub(rows.max(1)));
    Some((start, start + rows.min(total - start)))
}
//#endregion 🔖️MutationPages

//#region 🔖️MemberHistory
/// 🧩️ One edit of a composed member store as the composing parent's history reads it (design §12): the member that holds
/// it (`<slot>/<childId>`), its authoring transaction and moment, its operations' printed lines and its mutation rows,
/// each labelled from the child's own leaves and naming that member.
#[derive(Clone, Debug, PartialEq)]
pub struct MemberEditHistory {
    pub store: String,
    pub edit_id: String,
    pub started_at: String,
    pub timestamp: Option<HybridLogicalTimestamp>,
    pub transaction: Option<protocol::TransactionRef>,
    pub intent_label: Option<LocalizedLabel>,
    pub op_count: usize,
    pub op_lines: Vec<String>,
    pub mutations: Vec<MutationView>,
}

/// 🔭️ Reads the edits of one member store into [`MemberEditHistory`] rows — every edit, or only the `wanted` ones, each
/// read as one applied-edit slice (O(that edit), design §20.14); a mutation that was not superseded reuses its label from
/// `labelled`, the previous projection.
#[doc(hidden)]
pub struct MemberHistoryVisitor<'a, 'b> {
    pub store: String,
    pub labelled: &'a HashMap<&'b str, &'b MutationView>,
    pub viewer: bool,
    pub wanted: Option<&'a HashSet<&'b str>>,
    pub intent_kinds: fn(&str) -> &'static [&'static str],
}

impl store::MemberStoreVisitor for MemberHistoryVisitor<'_, '_> {
    type Output = Vec<MemberEditHistory>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        let edits = &store.envelope().vcs.edits;
        let history = |edit: &protocol::Edit<Mu>, position: Option<usize>| {
            let ops = position.and_then(|position| store.applied_edit_mutations(position).ok()).unwrap_or_default();
            let durable = position.and_then(|position| store.applied_edit_outcomes(position).ok()).unwrap_or_default();
            let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
            let ops: Vec<&store::AppliedMutation<'_, Mu>> = ops.iter().collect();
            let op_count = edit.forwards.len();
            let transaction = edit.mutation_meta.first().and_then(|meta| meta.transaction.clone());
            let mutations = history_mutation_views_of::<P, Mu>(&ops, &outcomes, self.labelled, |op| protocol::SemanticMutation::<P>::label(op), self.viewer, Some(&self.store));
            let intent_label = transaction.as_ref().and_then(|reference| history_intent_label_of::<P, Mu>((self.intent_kinds)(&reference.tool), &ops, &mutations));
            MemberEditHistory {
                store: self.store.clone(),
                edit_id: edit.id.clone(),
                started_at: edit.started_at.clone(),
                timestamp: edit.mutation_meta.first().map(|meta| meta.timestamp),
                transaction,
                intent_label,
                op_count,
                op_lines: edit.forwards[op_count.saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|op| op.print_op()).collect(),
                mutations,
            }
        };
        match self.wanted {
            Some(wanted) => {
                let mut found = Vec::new();
                for edit in edits.iter().rev().filter(|edit| wanted.contains(edit.id.as_str())) {
                    found.push(history(edit, store.applied_edit_position(&edit.id)));
                    if found.len() == wanted.len() {
                        break;
                    }
                }
                found
            }
            None => {
                let positions: HashMap<&str, usize> = store.applied_edit_ids().iter().enumerate().map(|(position, edit_id)| (edit_id.as_str(), position)).collect();
                edits.iter().map(|edit| history(edit, positions.get(edit.id.as_str()).copied())).collect()
            }
        }
    }
}
/// 📖️ Reads every mutation row (uncapped, unlabelled by any previous projection) of the `wanted` edits one member store holds,
/// each edit's rows in op order — what a member-only history row's window pages past its projection (audit W2A-11); O(those
/// edits).
#[doc(hidden)]
pub struct MemberMutationPageVisitor<'a> {
    pub store: String,
    pub wanted: &'a [String],
    pub viewer: bool,
}

impl store::MemberStoreVisitor for MemberMutationPageVisitor<'_> {
    type Output = Vec<(String, Vec<MutationView>)>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        let unlabelled = HashMap::new();
        self.wanted
            .iter()
            .filter_map(|edit_id| {
                let position = store.applied_edit_position(edit_id)?;
                let ops = store.applied_edit_mutations(position).ok()?;
                let durable = store.applied_edit_outcomes(position).unwrap_or_default();
                let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
                let rows = ops.iter().map(|op| history_mutation_view_of::<P, Mu>(op, outcomes.get(op.mutation_id.0.as_str()).copied(), &unlabelled, &|op: &Mu| protocol::SemanticMutation::<P>::label(op), self.viewer, Some(&self.store))).collect();
                Some((edit_id.clone(), rows))
            })
            .collect()
    }
}

/// 🧾️ One backfilled history row of composed-member edits no row carries yet: every unlogged member edit of one
/// transaction (or one edit without a transaction), ordered by its moment, labelled from its first leaf.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct MemberBackfillGroup {
    pub at: Option<HybridLogicalTimestamp>,
    pub transaction: Option<String>,
    pub edit_ids: Vec<String>,
    pub label: LocalizedLabel,
    pub started_at: String,
}

/// 🧾️ How a reload backfills composed-member edits (design §12): the edits of a transaction that also landed a parent
/// edit join that parent's row (`attached`, keyed by the parent edit id); every other unlogged edit forms a row of its
/// own transaction (`groups`, in moment order).
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MemberBackfill {
    pub attached: HashMap<String, Vec<String>>,
    pub groups: Vec<MemberBackfillGroup>,
}

/// 🧾️ The [`MemberBackfill`] of `histories` given the edit ids rows already carry (`logged`) and the parent edit each
/// unlogged parent transaction landed (`parent_transactions`, transaction id → parent edit id).
#[doc(hidden)]
pub fn member_backfill(histories: impl IntoIterator<Item = MemberEditHistory>, logged: &HashSet<&str>, parent_transactions: &HashMap<String, String>) -> MemberBackfill {
    let mut unlogged: Vec<MemberEditHistory> = histories.into_iter().filter(|history| !logged.contains(history.edit_id.as_str())).collect();
    unlogged.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.edit_id.cmp(&b.edit_id)));
    let mut backfill = MemberBackfill::default();
    for history in unlogged {
        let transaction = history.transaction.as_ref().map(|transaction| transaction.id.clone());
        if let Some(parent) = transaction.as_ref().and_then(|id| parent_transactions.get(id)) {
            backfill.attached.entry(parent.clone()).or_default().push(history.edit_id);
            continue;
        }
        if let Some(group) = backfill.groups.iter_mut().find(|group| transaction.is_some() && group.transaction == transaction) {
            group.edit_ids.push(history.edit_id);
            continue;
        }
        let label = history.intent_label.as_ref().or_else(|| history.mutations.first().map(|first| &first.label)).map_or_else(|| LocalizedLabel::data(history.op_lines.first().cloned().unwrap_or_else(|| history.edit_id.clone())), |leaf| history_leaf_row_label(leaf, history.op_count));
        backfill.groups.push(MemberBackfillGroup { at: history.timestamp, transaction, edit_ids: vec![history.edit_id], label, started_at: history.started_at });
    }
    backfill
}
//#endregion 🔖️MemberHistory

//#region 🔖️HistoryViewStamps
/// 🔢️ How many ids a [`HistoryStoreStamp`] keeps from the top of each of a store's applied and redo stacks: an undo or a
/// redo of up to this many edits between two refreshes still patches the cached history in place (design §20.14).
const HISTORY_STAMP_TAIL: usize = 8;

/// 🧭️ The O(1) facts of one store a cached history view was built against (design §20.14: per-mutation and per-render work
/// is O(change), never O(history)): its generation and content revision (a replaced store may restart at the same
/// generation), the sizes of its edit, checkpoint and alternative ledgers, its line and
/// checkpoint, the tops of its applied and redo stacks and the operation count of its applied top. The transition log is not
/// among them: an undo or redo is a `Revert`/`Reinstate` transition the stacks already show, and a history edit is a supersede
/// record the history view's own stamp counts.
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HistoryStoreStamp {
    pub generation: u64,
    pub revision: [u8; 32],
    pub edits: usize,
    pub checkpoints: usize,
    pub alternatives: usize,
    pub line: String,
    pub checkpoint: Option<String>,
    pub applied: usize,
    pub applied_tail: Vec<String>,
    pub redo: usize,
    pub redo_tail: Vec<String>,
    pub tail_ops: usize,
}

/// 🧭️ The [`HistoryStoreStamp`] of `store`: O(1) plus one lookup of the applied top from the ledger's tail.
#[doc(hidden)]
pub fn history_store_stamp<P, Mu>(store: &ArtifactStore<P, Mu>) -> HistoryStoreStamp
where
    P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ::semio_framework_os_kernel::StoreState,
    Mu: ::semio_framework_os_kernel::StoreMutationFor<P>,
{
    let envelope = store.envelope();
    let tail = |stack: &dsl::HistoryPageStack<String>| stack.iter_from(stack.len().saturating_sub(HISTORY_STAMP_TAIL)).cloned().collect::<Vec<_>>();
    let (applied, redo) = (store.applied_edit_ids(), store.redo_edit_ids());
    HistoryStoreStamp {
        generation: store.generation(),
        revision: store.content_revision_now(),
        edits: envelope.vcs.edits.len(),
        checkpoints: envelope.vcs.checkpoints.len(),
        alternatives: envelope.vcs.alternatives.len(),
        line: store.active_line_id(),
        checkpoint: store.current_checkpoint_id().map(str::to_string),
        applied: applied.len(),
        applied_tail: tail(applied),
        redo: redo.len(),
        redo_tail: tail(redo),
        tail_ops: applied.last().and_then(|top| envelope.vcs.edits.iter().rev().find(|edit| edit.id == *top)).map_or(0, |edit| edit.forwards.len()),
    }
}

/// 🧩️ Whether `moved` ids left the top of one stack (`from`, its recorded tail before) for the top of the other (`to`, its
/// recorded tail after) — an undo or a redo of `moved` edits: the moved ids are the same set on both sides, and what stayed
/// below them on each stack still agrees with its recorded tail over their overlap (both hold at most [`HISTORY_STAMP_TAIL`]
/// ids). `from_after` and `to_before` are the other two recorded tails.
fn history_stamp_moved(moved: usize, from: &[String], from_after: &[String], to: &[String], to_before: &[String]) -> bool {
    let overlaps = |kept: &[String], before: &[String]| {
        let overlap = kept.len().min(before.len());
        kept[kept.len() - overlap..] == before[before.len() - overlap..]
    };
    if moved > HISTORY_STAMP_TAIL || moved > from.len() || moved > to.len() {
        return false;
    }
    let (mut left, mut arrived): (Vec<&String>, Vec<&String>) = (from[from.len() - moved..].iter().collect(), to[to.len() - moved..].iter().collect());
    left.sort();
    arrived.sort();
    left == arrived && overlaps(from_after, &from[..from.len() - moved]) && overlaps(&to[..to.len() - moved], to_before)
}

/// 🔀️ The edits whose history rows a store change since `previous` touched, each with its applied position (`None` once it
/// is no longer applied): nothing when the store did not move; the appended, amended or redone applied tail with the previous
/// top (its row may have grown, and a composed member row reads its member's top); an undone tail of at most
/// [`HISTORY_STAMP_TAIL`] edits with the top it uncovered. `None` for anything else — a replay, a checkout, a history edit, a
/// checkpoint, a load — and the history view rebuilds (design §20.14).
#[doc(hidden)]
pub fn history_store_change(previous: &HistoryStoreStamp, current: &HistoryStoreStamp, applied: &dsl::HistoryPageStack<String>) -> Option<Vec<(String, Option<usize>)>> {
    if (previous.generation, previous.revision) == (current.generation, current.revision) {
        return Some(Vec::new());
    }
    if (previous.checkpoints, previous.alternatives, &previous.line, &previous.checkpoint) != (current.checkpoints, current.alternatives, &current.line, &current.checkpoint) || current.edits < previous.edits {
        return None;
    }
    let position = |at: usize| applied.get(at).map(|edit_id| (edit_id.clone(), Some(at)));
    let raised = || Some((previous.applied.saturating_sub(1)..current.applied).filter_map(position).collect());
    if current.applied < previous.applied {
        let undone = previous.applied - current.applied;
        let exact = current.redo == previous.redo + undone && current.edits == previous.edits && (current.applied == 0 || previous.applied_tail.len() > undone);
        return (exact && history_stamp_moved(undone, &previous.applied_tail, &current.applied_tail, &current.redo_tail, &previous.redo_tail))
            .then(|| previous.applied_tail[previous.applied_tail.len() - undone..].iter().map(|edit_id| (edit_id.clone(), None)).chain(current.applied.checked_sub(1).and_then(position)).collect());
    }
    if current.redo > 0 && current.redo < previous.redo {
        let redone = current.applied - previous.applied;
        let exact = previous.redo - current.redo == redone && current.edits == previous.edits;
        return if exact && history_stamp_moved(redone, &previous.redo_tail, &current.redo_tail, &current.applied_tail, &previous.applied_tail) { raised() } else { None };
    }
    let grown = (current.applied, current.tail_ops, current.edits) != (previous.applied, previous.tail_ops, previous.edits);
    let prefix_kept = previous.applied == 0 || applied.get(previous.applied - 1) == previous.applied_tail.last();
    let redo_kept = current.redo == 0 || (current.redo, &current.redo_tail) == (previous.redo, &previous.redo_tail);
    if grown && prefix_kept && redo_kept { raised() } else { None }
}

/// 🧾️ One member ledger as the command-log backfill reads it: its length and last edit id and, against the length and last
/// id a backfill mark recorded, the ids of every edit after them — `None` when that prefix no longer stands (or none was
/// recorded) and the backfill reads the whole history.
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemberLedgerTail {
    pub len: usize,
    pub last: Option<String>,
    pub fresh: Option<Vec<String>>,
}

/// 🧾️ Reads one member store's [`MemberLedgerTail`] past `known` (a recorded length and last id): O(new edits).
#[doc(hidden)]
pub struct MemberLedgerTailVisitor<'a> {
    pub known: Option<(usize, Option<&'a str>)>,
}

impl store::MemberStoreVisitor for MemberLedgerTailVisitor<'_> {
    type Output = MemberLedgerTail;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        let edits = &store.envelope().vcs.edits;
        let len = edits.len();
        let fresh = self.known.filter(|(known, last)| *known <= len && (*known == 0 || edits.iter().rev().nth(len - known).map(|edit| edit.id.as_str()) == *last)).map(|(known, _)| {
            let mut fresh: Vec<String> = edits.iter().rev().take(len - known).map(|edit| edit.id.clone()).collect();
            fresh.reverse();
            fresh
        });
        MemberLedgerTail { len, last: edits.last().map(|edit| edit.id.clone()), fresh }
    }
}

/// 🧭️ Reads one member store's [`HistoryStoreStamp`] and, against its `previous` stamp, the edits the change touched.
#[doc(hidden)]
pub struct MemberHistoryStampVisitor<'a> {
    pub previous: Option<&'a HistoryStoreStamp>,
}

impl store::MemberStoreVisitor for MemberHistoryStampVisitor<'_> {
    type Output = (HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>);

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        let stamp = history_store_stamp(store);
        let change = self.previous.and_then(|previous| history_store_change(previous, &stamp, store.applied_edit_ids()));
        (stamp, change)
    }
}
//#endregion 🔖️HistoryViewStamps
//#endregion 🔖️Driver

//#region 🔖️Panel
/// 💬️ The framework's history-panel copy, EN and DE, no default locale.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryPanelText {
    Actions,
    Commands,
    Undo,
    Redo,
    CommitCheckpoint,
    CheckIn,
    CreateAlternative,
    Filter,
    FilterAll,
    FilterWithoutMutations,
    FilterOnlyMutations,
    Backwards,
    TimeTravel,
    Edit,
    Accept,
    Discard,
    Withdraw,
    Restore,
    Finalize,
    Exit,
    CancelReplay,
    NextProblem,
    UseSelection,
    NothingSelected,
    Remove,
    Inputs,
    NoInputs,
    Pending,
    Edited,
    Superseded,
    Withdrawn,
    Draft,
    Hex,
    Opacity,
    ClearInput,
    AddItem,
    RemoveItem,
    ItemCount,
    MaxItems,
    MaxItem,
    MinItems,
    MinItem,
    Cleared,
    TooLong,
    Alternatives,
    Current,
    BranchedBy,
    EditedHistory,
    Switch,
    Trunk,
    MutationUnavailable,
}

impl HistoryPanelText {
    pub fn text(self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => match self {
                Self::Actions => "Actions",
                Self::Commands => "Commands",
                Self::Undo => "Undo",
                Self::Redo => "Redo",
                Self::CommitCheckpoint => "Commit Checkpoint",
                Self::CheckIn => "Check In",
                Self::CreateAlternative => "Create Alternative",
                Self::Filter => "Filter",
                Self::FilterAll => "All",
                Self::FilterWithoutMutations => "Without Mutations",
                Self::FilterOnlyMutations => "Only Mutations",
                Self::Backwards => "Backwards",
                Self::TimeTravel => "History editing",
                Self::Edit => "Edit",
                Self::Accept => "Accept",
                Self::Discard => "Discard",
                Self::Withdraw => "Withdraw",
                Self::Restore => "Restore",
                Self::Finalize => "Finalize",
                Self::Exit => "Exit",
                Self::CancelReplay => "Cancel replay",
                Self::NextProblem => "Next problem",
                Self::UseSelection => "Use selection",
                Self::NothingSelected => "Nothing referenced",
                Self::Remove => "Remove",
                Self::Inputs => "Inputs",
                Self::NoInputs => "No editable inputs",
                Self::Pending => "Not applied while editing",
                Self::Edited => "Edited",
                Self::Superseded => "Replaced",
                Self::Withdrawn => "Withdrawn",
                Self::Draft => "Draft",
                Self::Hex => "Hex code",
                Self::Opacity => "Opacity",
                Self::ClearInput => "Clear {label}",
                Self::AddItem => "Add item",
                Self::RemoveItem => "Remove item",
                Self::ItemCount => "Items: {count}",
                Self::MaxItems => "Maximum {count} items",
                Self::MaxItem => "Maximum 1 item",
                Self::MinItems => "Minimum {count} items",
                Self::MinItem => "Minimum 1 item",
                Self::Cleared => "Cleared (no value)",
                Self::TooLong => "Too long to edit here: shown shortened",
                Self::Alternatives => "Alternatives",
                Self::Current => "Current",
                Self::BranchedBy => "Branched by {author}, {time}",
                Self::EditedHistory => "Edited history",
                Self::Switch => "Switch",
                Self::Trunk => "Main line",
                Self::MutationUnavailable => "Mutation not available",
            },
            Locale::De => match self {
                Self::Actions => "Aktionen",
                Self::Commands => "Befehle",
                Self::Undo => "Rückgängig",
                Self::Redo => "Wiederholen",
                Self::CommitCheckpoint => "Checkpoint",
                Self::CheckIn => "Einchecken",
                Self::CreateAlternative => "Alternative erstellen",
                Self::Filter => "Filter",
                Self::FilterAll => "Alle",
                Self::FilterWithoutMutations => "Ohne Mutationen",
                Self::FilterOnlyMutations => "Nur Mutationen",
                Self::Backwards => "Zurück bis hier",
                Self::TimeTravel => "Verlaufsbearbeitung",
                Self::Edit => "Bearbeiten",
                Self::Accept => "Übernehmen",
                Self::Discard => "Verwerfen",
                Self::Withdraw => "Zurückziehen",
                Self::Restore => "Wiederherstellen",
                Self::Finalize => "Abschließen",
                Self::Exit => "Beenden",
                Self::CancelReplay => "Neuanwendung abbrechen",
                Self::NextProblem => "Nächstes Problem",
                Self::UseSelection => "Auswahl verwenden",
                Self::NothingSelected => "Nichts referenziert",
                Self::Remove => "Entfernen",
                Self::Inputs => "Eingaben",
                Self::NoInputs => "Keine bearbeitbaren Eingaben",
                Self::Pending => "Beim Bearbeiten nicht angewendet",
                Self::Edited => "Bearbeitet",
                Self::Superseded => "Ersetzt",
                Self::Withdrawn => "Zurückgezogen",
                Self::Draft => "Entwurf",
                Self::Hex => "Hex-Code",
                Self::Opacity => "Deckkraft",
                Self::ClearInput => "{label} leeren",
                Self::AddItem => "Element hinzufügen",
                Self::RemoveItem => "Element entfernen",
                Self::ItemCount => "Elemente: {count}",
                Self::MaxItems => "Höchstens {count} Einträge",
                Self::MaxItem => "Höchstens 1 Eintrag",
                Self::MinItems => "Mindestens {count} Einträge",
                Self::MinItem => "Mindestens 1 Eintrag",
                Self::Cleared => "Geleert (kein Wert)",
                Self::TooLong => "Zu lang, um hier bearbeitet zu werden: gekürzt angezeigt",
                Self::Alternatives => "Alternativen",
                Self::Current => "Aktuell",
                Self::BranchedBy => "Abgezweigt von {author}, {time}",
                Self::EditedHistory => "Bearbeiteter Verlauf",
                Self::Switch => "Wechseln",
                Self::Trunk => "Hauptlinie",
                Self::MutationUnavailable => "Mutation nicht verfügbar",
            },
        }
    }
}

/// 🚦️ A severity's word, EN and DE; a reader hears it, colour is never its only carrier.
#[doc(hidden)]
pub fn history_severity_text(level: semio_framework_diagnostic::Severity, locale: Locale) -> &'static str {
    match (locale, level) {
        (Locale::En, semio_framework_diagnostic::Severity::Info) => "Info",
        (Locale::En, semio_framework_diagnostic::Severity::Warning) => "Warning",
        (Locale::En, semio_framework_diagnostic::Severity::Error) => "Error",
        (Locale::En, semio_framework_diagnostic::Severity::Fatal) => "Fatal",
        (Locale::De, semio_framework_diagnostic::Severity::Info) => "Info",
        (Locale::De, semio_framework_diagnostic::Severity::Warning) => "Warnung",
        (Locale::De, semio_framework_diagnostic::Severity::Error) => "Fehler",
        (Locale::De, semio_framework_diagnostic::Severity::Fatal) => "Kritisch",
    }
}

/// 🏷️ An outcome code's words, EN and DE, never the code itself: a frozen `mutation.*` outcome code's short words, else the
/// framework's placeholder-free notice of the code (`kernel::history_notice`, `kernel::framework_fault_notice`), else the
/// generic "could not apply" words.
#[doc(hidden)]
pub fn history_code_text(code: &str, locale: Locale) -> &'static str {
    let known = match code {
        "mutation.target-missing" => Some(("Target missing", "Ziel fehlt")),
        "mutation.target-referenced" => Some(("Target still referenced", "Ziel wird noch referenziert")),
        "mutation.target-mismatch" => Some(("Inconsistent with the target", "Widerspricht dem Ziel")),
        "mutation.no-op" => Some(("No change", "Keine Änderung")),
        "mutation.partial" => Some(("Partially applied", "Teilweise angewendet")),
        "mutation.clamped" => Some(("Clamped", "Begrenzt")),
        "mutation.precondition-drifted" => Some(("Precondition drifted", "Vorbedingung nicht mehr erfüllt")),
        "mutation.duplicate-id" => Some(("Duplicate id", "ID bereits vergeben")),
        "mutation.invariant" => Some(("Invalid state", "Ungültiger Zustand")),
        "mutation.inverse-refused" => Some(("Cannot be reversed", "Nicht umkehrbar")),
        "mutation.cascade" => Some(("Cascaded", "Folgeänderung")),
        code if code.starts_with(protocol::APPLY_OUTCOME_CODE_PREFIX) => Some(("Could not apply", "Nicht anwendbar")),
        _ => None,
    };
    let framework = || semio_framework::kernel::history_notice(code).or_else(|| semio_framework::kernel::framework_fault_notice(code)).filter(|(en, de)| !en.contains('{') && !de.contains('{'));
    let (en, de) = known.or_else(framework).unwrap_or(("Could not apply", "Nicht anwendbar"));
    match locale {
        Locale::En => en,
        Locale::De => de,
    }
}

/// 🎨️ The tone and icon of a severity (none: applied cleanly).
#[doc(hidden)]
pub fn history_severity_tone(level: Option<semio_framework_diagnostic::Severity>) -> (Tone, &'static str) {
    match level {
        None => (Tone::Success, "check"),
        Some(semio_framework_diagnostic::Severity::Info) => (Tone::Info, "info"),
        Some(semio_framework_diagnostic::Severity::Warning) => (Tone::Warning, "triangle-alert"),
        Some(semio_framework_diagnostic::Severity::Error) => (Tone::Danger, "alert-circle"),
        Some(semio_framework_diagnostic::Severity::Fatal) => (Tone::Danger, "x"),
    }
}

/// 📝️ One line naming a mutation's state and its first message: severity word, then the localized code; editing
/// states lead.
#[doc(hidden)]
pub fn history_mutation_description(entry: &HistoryMutationEntry, locale: Locale) -> String {
    let mut parts: Vec<&str> = Vec::new();
    if entry.pending {
        parts.push(HistoryPanelText::Pending.text(locale));
    }
    if entry.edited {
        parts.push(HistoryPanelText::Edited.text(locale));
    }
    let introduced = TimeTravelLabel::OutcomeIntroduced.localized(|en, de| match locale {
        Locale::En => en,
        Locale::De => de,
    });
    if entry.introduced {
        parts.push(introduced);
    }
    if entry.withdrawn {
        parts.push(HistoryPanelText::Withdrawn.text(locale));
    } else if entry.superseded {
        parts.push(HistoryPanelText::Superseded.text(locale));
    }
    let mut line = parts.join(" \u{b7} ");
    if let Some(worst) = entry.worst {
        let first = entry.messages.iter().find(|message| message.level == worst);
        let severity = history_severity_text(worst, locale);
        let detail = first.map(|message| history_code_text(&message.code, locale)).unwrap_or_default();
        if !line.is_empty() {
            line.push_str(" \u{b7} ");
        }
        line.push_str(severity);
        if !detail.is_empty() {
            line.push_str(": ");
            line.push_str(detail);
        }
    }
    line
}
//#endregion 🔖️Panel
