//! 🏄️ Staged `group-history` members of the `instance-dispatch` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🤝️ Group history actions over the composed children.
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::os_store::composition_coordinator::CompositionCoordinator;
use crate::dispatch_bundle::DispatchCtx;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework::kernel::{InvocationResult, UiDirtyScope};
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_ui_contract::*;
use semio_framework_plugin_instance_docs::docs_plugin_app_impl::history_changed_event;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// ↩️ Task 4: group-aware undo/redo. Routes through
    /// `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`/`redo_group` across `self.store` (the parent)
    /// plus every LIVE child in `self.children` — the coordinator itself filters to members whose
    /// tail actually carries `group_id` (a child never touched by this particular gesture, or one
    /// whose own `undo()`/`redo()` call fails, is SKIPPED, not aborted — see `GroupUndoReport`'s
    /// own doc comment for why abort-all would be actively harmful: one collaborator's foreign or
    /// failed child edit must never permanently freeze this parent's undo stack). This generalizes
    /// the plain single-store path's existing benign `NothingToUndo`/`NothingToRedo`/`ForeignEdit`
    /// collapse to the multi-member case. `self.store` (the parent) and `self.children` (this
    /// composition's CHILD-kind type) are passed as SEPARATE arguments now — the same `Mp`/`Mc`
    /// split `dispatch_group` uses (see its own doc comment) — rather than one homogeneous list,
    /// so no more caller-side ordering dance either: `undo_group`/`redo_group` order parent vs.
    /// children internally to match `dispatch_group`'s apply order, and (per their own doc
    /// comments) treat every member independently regardless of order anyway.
    ///
    /// 🎯️ RESOLVED (was BLOCKED — see `📓️terra-dedyn-fw-os-spacemember-report.md`
    /// §undo_group/redo_group, and `📓️terra-dispatch-group-split-report.md` for the fix):
    /// `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`/`redo_group<Mp: SpaceMember, Mc:
    /// SpaceMember>` now take `parent`/`children` as separate arguments, so `self.store`
    /// (`ArtifactStore<A::Snapshot, A::Mutation>`) unifies as `Mp` and `self.children`'s
    /// member type (`M`, this composition's CHILD-kind type) unifies as `Mc` independently.
    ///
    /// 🧾️ Nothing actually moved (every member was foreign/failed) — benign collapse,
    /// mirroring `NothingToUndo`/`NothingToRedo`/`ForeignEdit` just above: NOT logged
    /// (never touched any store), but the skip diagnostics still ride along so the caller
    /// can see WHY nothing happened instead of silently no-op'ing.
    ///
    /// 🧾️ Append-only, same as the plain path: undo/redo are pure cursor motion, never
    /// logged with an `edit_id` — `child_edit_ids` records every CHILD member this group call
    /// actually touched.
    pub async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.child_composition.admit_child_content_publication_span(self.child_composition.children.len())?;
        let parent_id = self.stores.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.stores.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-dialect"), "composition history requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let child_refs: Vec<ArtifactRef> = self.child_composition.children.entries_physical().map(|entry| entry.reference.clone()).collect();

        let mut children: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
        for (reference, entry) in child_refs.iter().zip(self.child_composition.children.entries_mut_physical()) {
            children.push((reference, &mut entry.member));
        }

        let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut *self.stores.store, &mut children, group_id, identity).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut *self.stores.store, &mut children, group_id, identity).await };
        drop(children);
        self.history.cache = None;
        for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
            let Some(key) = self.child_composition.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
            };
            let publication_generation = self.child_composition.admit_child_content_publication()?;
            self.child_composition.publish_member_content(publication_generation, key.borrowed()).await?;
        }

        let diagnostics: Vec<semio_framework_diagnostic::Diagnostic> = report
            .skipped
            .iter()
            .map(|(reference, error)| {
                semio_framework_diagnostic::Diagnostic::error("composition.group-history.skipped-member", semio_framework_diagnostic::TextSpan::default(), format!("{action} skipped member {} ({error})", reference.artifact_id))
            })
            .collect();

        if report.undone.is_empty() {
            let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
            result.diagnostics = diagnostics;
            return Ok(result);
        }

        self.tools.record_command(&mut *self.history, action, ActionKind::History, None, None, Vec::new(), None);
        let child_edit_ids: Vec<String> = report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id).map(|(_, edit_id)| edit_id.clone()).collect();
        if let Some(last) = self.history.command_log.last_mut() {
            last.child_edit_ids = child_edit_ids;
        }

        let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await;
        result.diagnostics = diagnostics;
        Ok(result)
    }
}
