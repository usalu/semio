//! 🧴️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_artifact_reference::ArtifactRef;
use crate::children_components::ChildComposition;
use semio_framework_plugin_decl_views::child_view::{ChildContentView, MemberKeyRef};
use semio_framework_plugin_instance_history::history_components::CommandHistory;
use semio_framework_plugin_decl_views::history_view::CommandLogEntry;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use std::collections::HashSet;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;

impl<M: ReferenceSpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🧩️ Every live child's TAIL applied edit id, plus whether any child has an edit on
    /// its redo stack — the composed-artifact halves of `can_undo`/`can_redo`/`revertible`. A
    /// `Child`-lane document verb (see `ArtifactToolPublicationLane::Child`) never touches the
    /// parent store at all, so without this the shell disables its own undo control over a
    /// perfectly undoable ledger row. Empty and free for every leaf app: `entries()` yields
    /// nothing when the registry holds no child.
    pub async fn child_history_tails(&self) -> (HashSet<String>, bool) {
        let mut applied_tails = HashSet::new();
        let mut has_redo_tail = false;
        for entry in self.children.entries() {
            if let Some(edit_id) = entry.member.tail_edit_id().await {
                let _ = applied_tails.insert(edit_id);
            }
            if !has_redo_tail {
                has_redo_tail = entry.member.redo_tail().await.is_some();
            }
        }
        (applied_tails, has_redo_tail)
    }

    /// 🧩️↩️ The composite group id an `undo`/`redo` must target when the PARENT store never
    /// moved. `commit_framework_history_route` reads `self.store.tail_group_id()` first, which is
    /// `None` for a gesture that published only on the `Child` lane; every such gesture still
    /// stamped its `invocation_id` onto each touched child (`CompositionCoordinator::dispatch_group`
    /// phase 2), so the group is recoverable from the children's own tails. Ordered by the
    /// append-only command log rather than by any member's clock: for `undo` the LAST row naming a
    /// live child tail is the most recent still-applied group, and for `redo` the FIRST row naming
    /// a child redo-tail is the one a single `redo` reapplies. A child whose tail carries no group
    /// id is not a composite gesture and is never a target.
    pub async fn child_group_history_target<A: ArtifactApp>(&self, history: &CommandHistory<A>, action: &str) -> Option<String> {
        if self.children.is_empty() {
            return None;
        }
        let mut candidates: Vec<(String, String)> = Vec::new();
        for entry in self.children.entries() {
            let candidate = if action == "undo" {
                match (entry.member.tail_edit_id().await, entry.member.tail_group_id().await) {
                    (Some(edit_id), Some(group_id)) => Some((edit_id, group_id)),
                    _ => None,
                }
            } else {
                match entry.member.redo_tail().await {
                    Some((edit_id, Some(group_id))) => Some((edit_id, group_id)),
                    _ => None,
                }
            };
            if let Some(candidate) = candidate {
                candidates.push(candidate);
            }
        }
        if candidates.is_empty() {
            return None;
        }
        let matched = |entry: &CommandLogEntry| candidates.iter().find(|(edit_id, _)| entry.child_edit_ids.iter().any(|logged| logged == edit_id)).map(|(_, group_id)| group_id.clone());
        let ordered = if action == "undo" { history.command_log.iter().rev().find_map(matched) } else { history.command_log.iter().find_map(matched) };
        ordered.or_else(|| candidates.first().map(|(_, group_id)| group_id.clone()))
    }


}
