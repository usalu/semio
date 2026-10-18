//! 🔭️ Closure source and content owners of the child members: the document closure source view, the borrowed content owners, the retiring member lookups and the original content step owners. See [`super`].

use super::*;

#[doc(hidden)]
pub struct DocumentClosureSourceView<'a, P, M> {
    pub generation: u64,
    pub root_reference: &'a ArtifactRef,
    root_snapshot: &'a P,
    pub members: &'a ChildMemberRegistry<M>,
}

impl<'a, P, M> DocumentClosureSourceView<'a, P, M> {
    pub fn new(generation: u64, root_reference: &'a ArtifactRef, root_snapshot: &'a P, members: &'a ChildMemberRegistry<M>) -> Self {
        Self { generation, root_reference, root_snapshot, members }
    }
}

impl<P, M> store::OwnedDocumentClosureSource for DocumentClosureSourceView<'_, P, M>
where
    P: semio_framework_schema_composition::ArtifactCompositionFields,
    M: SpaceMember,
{
    fn generation(&self) -> u64 {
        self.generation
    }

    fn root_reference(&self) -> &ArtifactRef {
        self.root_reference
    }

    fn member_count(&self) -> usize {
        self.members.len()
    }

    fn member_reference(&self, index: usize) -> Option<&ArtifactRef> {
        self.members.entry_by_ordinal(index).map(|entry| &entry.reference)
    }

    fn member_owner(&self, index: usize) -> Option<&store::OwnerRef> {
        self.members.entry_by_ordinal(index).map(|entry| &entry.owner)
    }

    fn child_projection(&self, parent: Option<usize>) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_composition_admission::ChildRestoreProjectionError> {
        match parent {
            None => semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(self.root_snapshot),
            Some(index) => self.members.entry_by_ordinal(index).ok_or(semio_framework_composition_admission::ChildRestoreProjectionError::IncompleteSet)?.member.child_restore_projection(),
        }
    }
}

#[doc(hidden)]
pub struct ChildContentBorrowedOwners<'a> {
    pub views: [Option<&'a ChildContentView>; ARTIFACT_LIVE_OUTPUT_SLOTS + 2],
    pub len: usize,
}

impl<'a> ChildContentBorrowedOwners<'a> {
    pub fn none() -> Self { Self { views: [None; ARTIFACT_LIVE_OUTPUT_SLOTS + 2], len: 0 } }
    pub fn push(&mut self, view: &'a ChildContentView) {
        assert!(self.len < self.views.len(), "borrowed original child retainers exceed their fixed registry");
        self.views[self.len] = Some(view);
        self.len += 1;
    }
    pub fn copy_reservation() -> usize {
        size_of::<Self>() + size_of::<Option<&mut ChildContentRetirement>>() + size_of::<u64>()
    }
}

impl ChildContentRetainers for ChildContentBorrowedOwners<'_> {
    fn retains_root(&self, view: &ChildContentView) -> bool { self.views[..self.len].iter().flatten().any(|retainer| retainer.same_owner(view)) }
    fn retains_page(&self, page_index: usize, page: &std::sync::Arc<ChildContentPage>) -> bool { self.views[..self.len].iter().flatten().any(|view| ChildContentOwners::view_retains_page(view, page_index, page)) }
    fn retains_entry(&self, entry: &std::sync::Arc<ChildContentEntry>) -> bool { self.views[..self.len].iter().flatten().any(|view| view.contains_entry_owner(entry)) }
}

/// 🍂️ Whether the member with one exact artifact id is still retiring (it has not closed yet).
#[doc(hidden)]
pub fn member_is_retiring<M>(registry: &ArtifactFixedRegistry<ChildMemberRetirement<M>>, artifact_id: &str) -> bool {
    (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index| registry.entry(index).is_some_and(|(_, retirement)| retirement.retires(artifact_id)))
}

/// 🍂️ The retiring member with one exact artifact id, read-only for demand quotes.
fn retiring_member<'a, M>(registry: &'a ArtifactFixedRegistry<ChildMemberRetirement<M>>, artifact_id: &str) -> Option<&'a M> {
    (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| registry.entry(index).and_then(|(_, retirement)| retirement.entry.as_ref().filter(|entry| entry.reference.artifact_id == artifact_id)).map(|entry| &entry.member))
}

/// 🍂️ The retiring member with one exact artifact id — the disposer of the snapshot leases retired roots still hold.
fn retiring_member_mut<'a, M>(registry: &'a mut ArtifactFixedRegistry<ChildMemberRetirement<M>>, artifact_id: &str) -> Option<&'a mut M> {
    let id = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| registry.entry(index).and_then(|(id, retirement)| retirement.retires(artifact_id).then_some(*id)))?;
    registry.get_mut(id)?.entry.as_mut().map(|entry| &mut entry.member)
}

/// 🪪️ The member that disposes one child snapshot lease: the live member at its owner edge, else the retiring member
/// of that artifact id, which its coordinate left while a retired root still lent its snapshot.
pub(super) fn child_snapshot_owner<'a, M>(children: &'a mut ChildMemberRegistry<M>, retiring: Option<&'a mut ArtifactFixedRegistry<ChildMemberRetirement<M>>>, key: MemberKeyRef<'_>, artifact_id: &str) -> Option<&'a mut M> {
    if let Some(entry) = children.member_mut(key) {
        return Some(&mut entry.member);
    }
    retiring_member_mut(retiring?, artifact_id)
}

/// 🪪️ Read-only twin of [`child_snapshot_owner`] for demand quotes.
pub(super) fn child_snapshot_owner_ref<'a, M>(children: &'a ChildMemberRegistry<M>, retiring: Option<&'a ArtifactFixedRegistry<ChildMemberRetirement<M>>>, key: MemberKeyRef<'_>, artifact_id: &str) -> Option<&'a M> {
    if let Some(entry) = children.member(key) {
        return Some(&entry.member);
    }
    retiring_member(retiring?, artifact_id)
}

/// 🪢️ Borrows disjoint initialized registry slots and the exact live preview retainers.
#[doc(hidden)]
pub fn original_content_step_owners<'a>(registry: &'a mut ArtifactFixedRegistry<ChildContentRetirement>, stepping: u64, member: Option<&'a ChildContentView>, review: Option<&'a ChildContentView>) -> Option<(&'a mut ChildContentRetirement, ChildContentBorrowedOwners<'a>)> {
    let mut owners = ChildContentBorrowedOwners::none();
    let selected = registry.partition_mut(stepping, |retirement| owners.push(&retirement.view));
    if let Some(view) = member { owners.push(view); }
    if let Some(view) = review { owners.push(view); }
    Some((selected?, owners))
}
