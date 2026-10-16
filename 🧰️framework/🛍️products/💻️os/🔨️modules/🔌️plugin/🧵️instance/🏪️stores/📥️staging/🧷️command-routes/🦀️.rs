//! 🧼️ Staged slice of the `command-routes` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🧷️ Command routes of the mounted artifact instance: framework-reserved routes, action dispatch and command dispatch.
use super::*;

impl<A: ArtifactApp> InstanceStores<A> {
    /// ⏪️ One revert unit: undoes the newest document edit while `edit_id` is not yet the document store's tail.
    /// Answers whether an undo was applied (`false` once the target is the tail or the store refuses).
    pub(crate) async fn framework_revert_unit(&mut self, edit_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<bool, Fault> {
        let applied = self.store.applied_edit_ids();
        if !(applied.iter().any(|id| id.as_str() == edit_id) && applied.last().is_some_and(|tail| tail.as_str() != edit_id)) {
            return Ok(false);
        }
        let undone = self.store.dispatch(ArtifactCommand::Undo, identity).await.map(|_| ());
        match undone {
            Ok(()) => Ok(true),
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(false),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// 🪪️ The identity every id one admitted command mints is scoped by ([`AppOperationContext::authoring_seed`]):
    /// content-addressed with [`store::content_id`] over the admitting actor and this document store's hybrid
    /// logical clock ticked for the admission — per author and moment, never a per-instance counter a fresh
    /// session restarts.
    pub(crate) fn authoring_seed(&self, actor: &str) -> String {
        let mut clock = self.store.clock_now();
        clock.tick(semio_framework_job::default_now_ms().unwrap_or(clock.physical_ms));
        store::content_id("authoring-seed", format!("{actor}\u{1f}{}:{}:{}", clock.actor, clock.physical_ms, clock.logical).as_bytes())
    }
}
