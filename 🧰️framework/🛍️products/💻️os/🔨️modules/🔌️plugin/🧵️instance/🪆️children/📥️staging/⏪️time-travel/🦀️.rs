//! 💉️ Staged slice of the `time-travel` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! ⏪️ The mounted instance's time-travel turns (history editing sessions, replay and supersession) and the
//! [`TimeTravelExtension`] that registers them with the instance through [`InstanceExtension`].
use semio_framework::AppRole;
use semio_framework_plugin_decl_app::ArtifactApp;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework_plugin_command::lifted_command::HistoryMutationPage;
use semio_framework_plugin_command::lifted_command::HistoryMutationPages;
use semio_framework_plugin_command::lifted_command::HistoryStoreStamp;
use semio_framework_plugin_command::lifted_command::MemberEditHistory;
use semio_framework_plugin_command::lifted_command::MemberHistoryStampVisitor;
use semio_framework_plugin_command::lifted_command::MemberHistoryVisitor;
use semio_framework_plugin_command::lifted_command::MemberLedgerTail;
use semio_framework_plugin_command::lifted_command::MemberLedgerTailVisitor;
use semio_framework_plugin_command::lifted_command::MemberMutationPageVisitor;
use store::Mutation;
use semio_framework_plugin_decl_views::history_view::MutationView;
use semio_framework_plugin_command::lifted_command::TimeTravelPanel;
use semio_framework_plugin_command::lifted_command::history_mutation_entry;
use semio_framework_plugin_command::lifted_command::history_mutation_view_of;
use semio_framework_plugin_command::lifted_command::history_row_mutation_extent;
use semio_framework_plugin_command::lifted_command::history_row_window_path;
use semio_framework_plugin_command::lifted_command::history_row_window_rows;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
use semio_framework_plugin_command::lifted_command::time_travel_mutation_label;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use protocol::OpText;
use crate::children_components::ChildComposition;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework::Fault;
use std::future::Future;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel::*;
use semio_framework_plugin_window_kits_rich::tree_kit::TREE_WINDOW_DEFAULT_ROWS;
use semio_framework::ViewModel;
use semio_framework_os_kernel::{MemberFactory, ReferenceSpaceMember};

impl<M: ReferenceSpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🧩️ The composed members' edits as the history reads them, keyed by edit id (design §12): every edit (the rows a full
    /// history build lists for the owned-child edits it carries, and the edits a reload backfills), or only the `wanted` ones
    /// (the rows an incremental refresh rebuilds, O(those edits)).
    pub fn member_edit_histories<A: ArtifactApp>(&self, labelled: &HashMap<&str, &MutationView>, wanted: Option<&HashSet<&str>>) -> HashMap<String, MemberEditHistory> {
        let viewer = A::ROLE == AppRole::Viewer;
        let mut histories = HashMap::new();
        if wanted.is_some_and(HashSet::is_empty) {
            return histories;
        }
        for (store, entry) in self.children.addressed_entries() {
            for history in entry.member.visit_member(MemberHistoryVisitor { store, labelled, viewer, wanted, intent_kinds: A::tool_intent_kinds }) {
                histories.insert(history.edit_id.clone(), history);
            }
        }
        histories
    }

    /// 🧾️ Every composed member's [`MemberLedgerTail`] past the lengths and last ids a backfill mark recorded (`known`), keyed
    /// and ordered by member (`<slot>/<childId>`), and whether every member `known` names is still held.
    pub fn member_ledger_tails(&self, known: Option<&[(String, usize, Option<String>)]>) -> (Vec<(String, MemberLedgerTail)>, bool) {
        let mut tails: Vec<(String, MemberLedgerTail)> = self
            .children
            .addressed_entries()
            .map(|(store, entry)| {
                let recorded = known.and_then(|known| known.iter().find(|(member, _, _)| *member == store)).map(|(_, len, last)| (*len, last.as_deref()));
                let tail = entry.member.visit_member(MemberLedgerTailVisitor { known: recorded });
                (store, tail)
            })
            .collect();
        tails.sort_by(|a, b| a.0.cmp(&b.0));
        let held = known.is_none_or(|known| known.iter().all(|(member, _, _)| tails.iter().any(|(store, _)| store == member)));
        (tails, held)
    }

    /// 🧭️ Every composed member's [`HistoryStoreStamp`], keyed and ordered by member (`<slot>/<childId>`), with the edits a
    /// change since `previous` touched ([`history_store_change`]; `None` for a member that is new or changed otherwise).
    pub fn member_history_stamps(&self, previous: &[(String, HistoryStoreStamp)]) -> Vec<(String, HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)> {
        let mut stamps: Vec<(String, HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)> = self
            .children
            .addressed_entries()
            .map(|(store, entry)| {
                let before = previous.iter().find(|(member, _)| *member == store).map(|(_, stamp)| stamp);
                let (stamp, change) = entry.member.visit_member(MemberHistoryStampVisitor { previous: before });
                (store, stamp, change)
            })
            .collect();
        stamps.sort_by(|a, b| a.0.cmp(&b.0));
        stamps
    }

    /// 🧹️ Retires a history edit's children view through the child-content retirements, as one admitted generation.
    pub fn retire_time_travel_children(&mut self, view: ChildContentView) -> Result<(), Fault> {
        let generation = self.admit_child_content_publication()?;
        self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(view, false));
        self.child_content_generation = generation;
        Ok(())
    }

    /// 📖️ The mutation rows the history body's windows show past each row's projection (gap N1): for every row whose
    /// mutation window reaches past its projected rows ([`history_row_mutation_extent`], [`history_row_window_rows`] — a
    /// host request, or a row open by default under `panel`'s overlay), that slice of its edit's remaining operations in op
    /// order — or of its member edits' for a row of composed-member edits alone — so every mutation of a transaction is
    /// reachable however long it is, while the projection and the wire stay bounded. Each paged row reads only its own edits'
    /// slices (design §20.14), and only when some window reaches past its projection.
    pub fn history_mutation_pages<A: ArtifactApp>(&self, stores: &InstanceStores<A>, history: &HistoryView, panel: Option<&TimeTravelPanel>, view: &ViewModel) -> HistoryMutationPages {
        let viewport_rows = view.tree_viewport_rows.unwrap_or(TREE_WINDOW_DEFAULT_ROWS) as usize;
        let viewer = A::ROLE == AppRole::Viewer;
        let mut pages = HistoryMutationPages::new();
        for entry in &history.commands {
            let (projected, total) = history_row_mutation_extent(entry);
            if total <= projected {
                continue;
            }
            let path = history_row_window_path(entry.seq);
            let request = view.tree_windows.iter().find(|request| request.body_key == FRAMEWORK_HISTORY_BODY_KEY && request.node_key == path);
            let default_open = entry.mutations.iter().any(|mutation| history_mutation_entry(mutation, panel).worst.is_some());
            let Some((start, end)) = history_row_window_rows(request, default_open, viewport_rows, total).filter(|(_, end)| *end > projected) else { continue };
            let (from, to) = (start.max(projected) - projected, end - projected);
            let shown: HashSet<&str> = entry.mutations.iter().map(|mutation| mutation.mutation_id.as_str()).collect();
            let rows: Vec<MutationView> = match entry.edit_id.as_deref() {
                Some(edit_id) => {
                    let hint = entry.mutations.iter().find(|mutation| mutation.store.is_none()).map(|mutation| mutation.position as usize);
                    let position = hint.filter(|position| stores.store.applied_edit_ids().get(*position).is_some_and(|held| held == edit_id)).or_else(|| stores.store.applied_edit_position(edit_id));
                    let ops = position.and_then(|position| stores.store.applied_edit_mutations(position).ok()).unwrap_or_default();
                    let durable = position.and_then(|position| stores.store.applied_edit_outcomes(position).ok()).unwrap_or_default();
                    let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
                    let unlabelled = HashMap::new();
                    ops.iter()
                        .filter(|op| !shown.contains(op.mutation_id.0.as_str()))
                        .skip(from)
                        .take(to - from)
                        .map(|op| history_mutation_view_of::<A::Snapshot, A::Mutation>(op, outcomes.get(op.mutation_id.0.as_str()).copied(), &unlabelled, &time_travel_mutation_label::<A>, viewer, None))
                        .collect()
                }
                None => {
                    let mut by_edit: HashMap<String, Vec<MutationView>> = HashMap::new();
                    for (store, member) in self.children.addressed_entries() {
                        by_edit.extend(member.member.visit_member(MemberMutationPageVisitor { store, wanted: &entry.child_edit_ids, viewer }));
                    }
                    entry.child_edit_ids.iter().filter_map(|edit_id| by_edit.remove(edit_id)).flatten().filter(|row| !shown.contains(row.mutation_id.as_str())).skip(from).take(to - from).collect()
                }
            };
            pages.insert(entry.seq, HistoryMutationPage { from, rows });
        }
        pages
    }
}
