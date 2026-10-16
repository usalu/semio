//! 🧴️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use super::*;
use crate::tool_jobs::time_travel;

impl<M: SpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🧩️ Every live child's TAIL applied edit id, plus whether any child has an edit on
    /// its redo stack — the composed-artifact halves of `can_undo`/`can_redo`/`revertible`. A
    /// `Child`-lane document verb (see `ArtifactToolPublicationLane::Child`) never touches the
    /// parent store at all, so without this the shell disables its own undo control over a
    /// perfectly undoable ledger row. Empty and free for every leaf app: `entries()` yields
    /// nothing when the registry holds no child.
    pub(crate) async fn child_history_tails(&self) -> (HashSet<String>, bool) {
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
    pub(crate) async fn child_group_history_target<A: ArtifactApp>(&self, history: &CommandHistory<A>, action: &str) -> Option<String> {
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

    /// 🌱️ Absorbs every `store::os_store::composition_coordinator::GroupReceipt::created_children` entry into the live
    /// `self.children` map — B2 flagged that a `ChildGenesis`-created member has no pre-existing
    /// caller-held reference the way an already-registered child does, so without this it would
    /// be silently dropped the moment `dispatch_group` returns, making `ChildGenesis` pointless
    /// (see `GroupReceipt::created_children`'s own doc comment). The slot name is recovered from
    /// `self.composition`'s own graph (`dispatch_group`'s phase 2 already called `insert_owns`
    /// for every genesis child before returning the receipt, so `slot_of` is always populated
    /// here). A free-standing method (not inlined into `dispatch_emit_group`) so it is directly
    /// unit-testable against a synthetic `created_children` list, independent of the `ChildEmit`
    /// wire path (which never emits a `ChildGenesis` itself today).
    pub(crate) async fn absorb_created_children<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, created_children: Vec<(ArtifactRef, M)>) -> Result<(), Fault> {
        for (target, mut member) in created_children {
            let publication_generation = self.admit_child_content_publication()?;
            let slot = self.composition.graph().await.slot_of(&target.artifact_id).await.ok_or_else(|| plugin_sdk_fault("created child has no exact target ownership slot"))?.to_owned();
            let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(stores.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("created child projection failed: {error}")))?;
            let child_id = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.artifact_id == target.artifact_id && fields.artifact_kind == target.dialect.artifact_kind && fields.standard == target.dialect.standard && fields.subset == target.dialect.subset).map(|(_, fields)| fields.child_id.to_owned()).ok_or_else(|| plugin_sdk_fault("created child is not declared by the published parent"))?;
            ChildContentView::hash(MemberKeyRef::root(&slot, &child_id))?;
            let admission = self.children.admit(&(slot.clone(), child_id.clone()))?;
            let parent = ArtifactRef { artifact_id: stores.store.envelope().id.clone(), dialect: A::DIALECT.into() };
            let owner = store::OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() };
            member.set_owner(Some(owner.clone())).await;
            self.children.insert_admitted(admission, target, owner, member);
            self.publish_child_content_member(publication_generation, &slot, &child_id).await?;
        }
        Ok(())
    }
}
