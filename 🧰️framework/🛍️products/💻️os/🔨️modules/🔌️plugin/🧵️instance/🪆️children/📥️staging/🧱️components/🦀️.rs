//! 🛖️ The instance components of `instance-children`: ChildComposition (WP6, plan section 7.3).
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_child::{ChildContentRetirement, ChildMemberRegistry, ChildMemberRetirement};
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use store::os_store::composition_coordinator::CompositionCoordinator;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use std::mem::ManuallyDrop;
use store::{MemberFactory, Mutation, ReferenceSpaceMember, SpaceMember};
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
use semio_framework_plugin_publication_mounted::owner::MountedPrivateChildGroup;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

/// 🏕️ Child composition: the live owned children, their private groups, content roots and retirement registries.
pub struct ChildComposition<M: ReferenceSpaceMember + MemberFactory> {
    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): every owned child's LIVE store, keyed by
    /// `(slot, child_id)` — mirrors how `store`/`config_store`/`draft_store` above each hold one
    /// lane's live `ArtifactStore`, generalized to N children instead of one fixed lane. Each
    /// entry also carries the dialect captured at open/genesis time (`SpaceMember` itself has no
    /// object-safe dialect getter — see `dispatch_emit_group`'s own doc comment), so a
    /// `store::os_store::composition_coordinator::ChildDispatch`/`store::ArtifactRef` can be rebuilt for it without re-deriving one.
    pub children: ChildMemberRegistry<M>,
    pub private_child_groups: ArtifactFixedRegistry<Box<MountedPrivateChildGroup<M>>>,
    pub child_content_root: ManuallyDrop<ChildContentView>,
    pub child_content_generation: u64,
    pub child_content_retirements: ArtifactFixedRegistry<ChildContentRetirement>,
    pub child_member_retirements: ArtifactFixedRegistry<ChildMemberRetirement<M>>,
    pub child_member_retirement_generation: u64,
    pub child_member_retirement_cursor: usize,
    pub followed_parent_generation: u64,
    /// ⏳️ The last follow pass waited on child retirements — its own publications were not yet admissible, or the child
    /// the parent names was still retiring — so the instance stays runnable until a later pass followed the parent.
    pub child_follow_awaits_retirement: bool,
    /// 📌️ Checkout pins for children that were NOT open when a checkpoint cascade ran. Draining
    /// this on `open_child` is what keeps a lazily-adopted child from silently sitting at head
    /// while the rest of the composition sits at a pinned checkpoint — the alternative (dropping
    /// the pin) would make a checked-out composition quietly inconsistent, which is exactly the
    /// class of bug the cascade exists to prevent.
    pub pending_child_pins: Vec<vcs::CompositionPin>,
    /// 🧩️ STATEFUL by design (owns the incrementally-maintained `store::os_store::composition_coordinator::CompositionGraph`) — see
    /// `store::os_store::composition_coordinator::CompositionCoordinator`'s own doc comment. One coordinator per `VcsArtifactApp`
    /// instance (i.e. per document), exactly like `store`/`config_store`/`draft_store` are
    /// per-instance rather than global.
    pub composition: CompositionCoordinator,
}

impl<M: ReferenceSpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🛶️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new(composition: CompositionCoordinator) -> Self {
        Self {
            children: ChildMemberRegistry::new(),
            private_child_groups: ArtifactFixedRegistry::new(),
            child_content_root: ManuallyDrop::new(ChildContentView::EMPTY),
            child_content_generation: 0,
            child_content_retirements: ArtifactFixedRegistry::new(),
            child_member_retirements: ArtifactFixedRegistry::new(),
            child_member_retirement_generation: 0,
            child_member_retirement_cursor: 0,
            followed_parent_generation: 0,
            child_follow_awaits_retirement: false,
            pending_child_pins: Vec::new(),
            composition,
        }
    }

    /// 🔢️ Sorted by `(slot, child_id)`: the map's iteration order is not stable, and a
    /// persisted child list that reshuffles between reads would make every save look like a
    /// change to anything diffing it.
    pub async fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, semio_framework::Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut entries: Vec<protocol::ChildPackEntry> = Vec::with_capacity(self.children.len());
        for (key, entry) in self.children.keyed_entries() {
            let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
            let owner = self.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
            entries.push(protocol::ChildPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), envelope_pack, owner });
        }
        entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
        Ok(entries)
    }

    pub async fn child_head_packs(&self) -> Result<Vec<protocol::ChildHeadPackEntry>, semio_framework::Fault> {

        let mut entries: Vec<protocol::ChildHeadPackEntry> = Vec::with_capacity(self.children.len());
        for (key, entry) in self.children.keyed_entries() {
            let head_pack = entry.member.document_pack_bytes().await.map_err(|error| error.into_fault())?;
            let owner = self.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
            entries.push(protocol::ChildHeadPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), head_pack, owner });
        }
        entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
        Ok(entries)
    }
}
