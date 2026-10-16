//! 🪞️ Staged slice of the `render` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 🖼️ Render side of the mounted artifact instance: composition graph, interaction state, render, media jobs and media export.
use super::*;

impl<M: SpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🕸️ Re-syncs `self.composition`'s ownership/link graph from the parent's own live
    /// `ArtifactRefs` projection (`child_refs()`/`links()`) — the mechanism
    /// `store::os_store::composition_coordinator::CompositionGraph::sync_member`'s own doc comment names as the required follow-up
    /// "after any dispatch that might have changed an artifact's own `ArtifactRefs`". Deliberately
    /// NOT called automatically by `dispatch_emit`/`dispatch_emit_group`: doing so would force
    /// `A::Snapshot: store::ArtifactRefs` onto EVERY `ArtifactApp` impl in the workspace (today
    /// zero of them implement it — `ArtifactRefs`-bearing snapshots are new, later-wave-scoped
    /// schema work), breaking every existing plugin's compile for a graph no leaf artifact needs
    /// synced. A composed artifact's own app calls this explicitly wherever its `handle` might
    /// have changed a `#[child(...)]`/`#[link_slot(...)]` field — see B2's own
    /// `📓️wave1-reports/b2-store-composition-report.md` sharedFileRequest #6.
    pub async fn sync_composition_graph<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>) -> Result<(), Fault>
    where
        A::Snapshot: store::ArtifactRefs,
    {
        let snapshot = stores.store.snapshot().map_err(|error| error.into_fault())?;
        let parent_id = stores.store.envelope().id.clone();
        self.composition.graph_mut().await.sync_member(&parent_id, &snapshot).await.map_err(|error| plugin_sdk_fault(error.to_string()))
    }
}
