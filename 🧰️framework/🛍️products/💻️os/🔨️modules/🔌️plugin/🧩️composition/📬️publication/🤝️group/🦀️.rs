mod private_publication_group {
    use super::*;
    use semio_framework_value::{ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
    use std::{mem::ManuallyDrop, sync::Arc};

    mod child { include!("🪆️child/🦀️.rs"); }
    pub(crate) use child::{PrivateChildPublicationInput, PrivateChildPublicationInputParts};
    mod owner { include!("📦️owner/🦀️.rs"); }
    pub(crate) use owner::{PrivateChildGroupSource, PrivateChildPublicationGroup, PrivateChildPreparedReceipt, PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN};

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PrivateOwnedPublicationPhase { Admission, Preparation, Staging, Staged, Adopted, Closing, Complete }

    pub(crate) struct PrivateOwnedPublicationLane {
        request: ManuallyDrop<Option<store::MemberStoreOwnedBatchRequest>>,
        publication: ManuallyDrop<Option<Box<dyn store::ErasedMemberStoreOneItemPublication>>>,
        metadata: Option<PrivatePublicationMetadata>,
        phase: PrivateOwnedPublicationPhase,
        grouped: bool,
    }

    impl PrivateOwnedPublicationLane {
        /// 🪪️ Retains one original typed source and its independently admitted metadata.
        pub(crate) fn new(request: store::MemberStoreOwnedBatchRequest) -> Self { Self { request: ManuallyDrop::new(Some(request)), publication: ManuallyDrop::new(None), metadata: None, phase: PrivateOwnedPublicationPhase::Admission, grouped: false } }
        pub(crate) fn request(&self) -> Option<&store::MemberStoreOwnedBatchRequest> { self.request.as_ref() }
        pub(crate) fn publication_mut(&mut self) -> Option<&mut (dyn store::ErasedMemberStoreOneItemPublication + 'static)> { self.publication.as_deref_mut() }
        pub(crate) fn staged(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Staged }
        pub(crate) fn prepared_for_staging(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Staging }
        pub(crate) fn prepared_edit_id(&self) -> Option<&str> { self.prepared_for_staging().then(|| self.publication.as_ref()?.prepared_edit_id()).flatten() }
        pub(crate) fn prepared_publication(&self) -> Option<&dyn store::ErasedMemberStoreOneItemPublication> { if self.prepared_for_staging() { self.publication.as_deref() } else { None } }
        pub(crate) fn adopted(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Adopted }
        pub(crate) fn progress(&self) -> store::ArtifactStoreOneItemCheckpoint { self.publication.as_ref().map_or(Default::default(), |publication| publication.progress()) }
        /// 🎟️ Borrows the complete constructor extent before any typed source handoff.
        pub(crate) fn next_capacity_byte_demand(&self, member: &impl SpaceMember) -> Result<usize, String> {
            if self.phase == PrivateOwnedPublicationPhase::Admission { return member.owned_publication_birth_bytes(self.request.as_ref().expect("private admission retains its original request")); }
            Ok(self.publication.as_ref().map_or(0, |publication| publication.next_group_byte_demand()))
        }
        /// 🪜️ Advances only one retained lane phase; staging never changes live authority.
        pub(crate) fn advance(&mut self, member: &mut impl SpaceMember, visibility: &Arc<vcs::ArtifactGroupVisibility>, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
            if grant.maximum_items == 0 { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
            match self.phase {
                PrivateOwnedPublicationPhase::Admission => {
                    let Some(publication) = member.begin_one_item_owned_publication(self.request.as_mut().expect("private source retained until admission"), grant)? else { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); };
                    *self.publication = Some(publication);
                    self.phase = PrivateOwnedPublicationPhase::Preparation;
                    Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress()))
                }
                PrivateOwnedPublicationPhase::Preparation => {
                    let step = member.prepare_one_item_publication(self.publication_mut().unwrap(), grant)?;
                    if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Staging; return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress())); }
                    Ok(step)
                }
                PrivateOwnedPublicationPhase::Staging => {
                    self.grouped = true;
                    let step = member.stage_one_item_publication(self.publication_mut().unwrap(), visibility, grant)?;
                    if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Staged; }
                    Ok(if self.staged() { store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress()) } else { step })
                }
                PrivateOwnedPublicationPhase::Staged => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress())),
                _ => Err("private publication preparation requires its original pending lane".into()),
            }
        }
        /// 🔓️ Adopts the already staged root only after the exact common visibility commits.
        pub(crate) fn adopt(&mut self, member: &mut impl SpaceMember, visibility: &Arc<vcs::ArtifactGroupVisibility>, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
            if self.adopted() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress())); }
            if !self.staged() { return Err("private publication adoption requires its fully staged candidate".into()); }
            let step = member.adopt_one_item_publication(self.publication_mut().unwrap(), visibility, grant)?;
            if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Adopted; self.grouped = false; }
            Ok(step)
        }
        /// 📐️ Exposes one indivisible active source, staged root, metadata, or frame demand.
        pub(crate) fn next_close_byte_demand(&self) -> Result<usize, ValueError> {
            if let Some(publication) = self.publication.as_ref() {
                if publication.terminal_is_empty() { return Ok(std::mem::size_of_val(publication.as_ref())); }
                return Ok(if self.grouped && publication.phase() != store::ArtifactStoreOneItemPublicationPhase::Closing { publication.next_group_byte_demand().max(4096) } else { publication.next_close_byte_demand() });
            }
            if let Some(request) = self.request.as_ref() { let (capacity, release) = request.mutations.next_demands()?; return Ok(capacity.max(release).max(request.mutations.next_copy_byte_demand())); }
            Ok(self.metadata.as_ref().map_or(0, PrivatePublicationMetadata::next_close_byte_demand))
        }
        /// ♻️ Returns staged authority before retiring original sources and whole physical frames.
        pub(crate) fn close_step(&mut self, member: &mut impl SpaceMember, grant: store::ArtifactStoreOneItemGrant) -> Result<PluginCloseStep, ValueError> {
            if self.terminal_is_empty() { return Ok(PluginCloseStep::Complete); }
            if grant.maximum_items == 0 { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
            self.phase = PrivateOwnedPublicationPhase::Closing;
            if let Some(publication) = self.publication.as_mut() {
                if !publication.terminal_is_empty() {
                    return member.abort_one_item_publication(publication.as_mut(), grant).map(|step| match step { store::SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes }, store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "private publication retains its exact source or staged snapshot reader" }, store::SnapshotRetirementStep::Complete => PluginCloseStep::Pending { released_items: 1, released_bytes: 0 } });
                }
                let bytes = std::mem::size_of_val(publication.as_ref());
                if grant.maximum_bytes < bytes { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
                self.publication.take();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
            }
            let controlled = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_bytes.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: 64 };
            if let Some(request) = self.request.as_mut() {
                if !request.mutations.terminal_is_empty() { return request.mutations.close_granted(controlled).map(Self::close_progress); }
                let request = self.request.take().unwrap();
                self.metadata = Some(PrivatePublicationMetadata::from_parts(PrivatePublicationMetadataParts { actor: request.actor, transaction: request.transaction, group_id: request.group_id }));
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(metadata) = self.metadata.as_mut() {
                let step = metadata.close_granted(controlled)?;
                if metadata.terminal_is_empty() { self.metadata = None; }
                return Ok(Self::close_progress(step));
            }
            self.phase = PrivateOwnedPublicationPhase::Complete;
            Ok(PluginCloseStep::Complete)
        }
        fn close_progress(step: RetainedCloneStep) -> PluginCloseStep { let progress = step.progress(); PluginCloseStep::Pending { released_items: progress.copied_items, released_bytes: progress.released_bytes } }
        pub(crate) fn terminal_is_empty(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Complete && self.request.is_none() && self.publication.is_none() && self.metadata.is_none() }
    }

    impl Drop for PrivateOwnedPublicationLane { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private typed publication retains its original source and staged authority until bounded terminal close"); } }
}

pub(crate) use private_publication_group::{PrivateOwnedPublicationLane, PrivateChildPublicationInput, PrivateChildPublicationInputParts, PrivateChildGroupSource, PrivateChildPublicationGroup, PrivateChildPreparedReceipt, PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN};
