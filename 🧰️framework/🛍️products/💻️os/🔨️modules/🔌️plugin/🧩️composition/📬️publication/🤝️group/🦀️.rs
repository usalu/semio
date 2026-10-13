pub(crate) mod private_publication_group {
    use super::*;
    use semio_framework_value::{ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
    use std::{mem::ManuallyDrop, sync::Arc};

    mod child { include!("🪆️child/🦀️.rs"); }
    use child::PrivateChildPublicationInput;
    pub(crate) mod owner { include!("📦️owner/🦀️.rs"); }
    use owner::{PrivateChildGroupSource, PrivateChildPublicationGroup, PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN};

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PrivateOwnedPublicationPhase { Admission, Preparation, Staging, Staged, Adopted, Closing, Complete }

    struct PrivateOwnedPublicationLane {
        request: ManuallyDrop<Option<store::MemberStoreOwnedBatchRequest>>,
        publication: ManuallyDrop<Option<Box<dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>>>,
        metadata: Option<PrivatePublicationMetadata>,
        phase: PrivateOwnedPublicationPhase,
        grouped: bool,
    }

    impl PrivateOwnedPublicationLane {
        /// 🪪️ Retains one original typed source and its independently admitted metadata.
        fn new(request: store::MemberStoreOwnedBatchRequest) -> Self { Self { request: ManuallyDrop::new(Some(request)), publication: ManuallyDrop::new(None), metadata: None, phase: PrivateOwnedPublicationPhase::Admission, grouped: false } }
        fn request(&self) -> Option<&store::MemberStoreOwnedBatchRequest> { self.request.as_ref() }
        fn publication_mut(&mut self) -> Option<&mut (dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication + 'static)> { self.publication.as_deref_mut() }
        fn staged(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Staged }
        fn prepared_for_staging(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Staging }
        fn prepared_edit_id(&self) -> Option<&str> { self.prepared_for_staging().then(|| self.publication.as_ref()?.prepared_edit_id()).flatten() }
        fn prepared_publication(&self) -> Option<&dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication> { if self.prepared_for_staging() { self.publication.as_deref() } else { None } }
        fn adopted(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Adopted }
        fn progress(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.publication.as_ref().map_or(Default::default(), |publication| publication.progress()) }
        /// 🎟️ Borrows the complete constructor extent before any typed source handoff.
        fn next_capacity_byte_demand(&self, member: &impl SpaceMember) -> Result<usize, String> {
            if self.phase == PrivateOwnedPublicationPhase::Admission { return member.owned_publication_birth_bytes(self.request.as_ref().expect("private admission retains its original request")); }
            Ok(self.publication.as_ref().map_or(0, |publication| publication.next_group_byte_demand()))
        }
        /// 🪜️ Advances only one retained lane phase; staging never changes live authority.
        fn advance(&mut self, member: &mut impl SpaceMember, visibility: &Arc<vcs::ArtifactGroupVisibility>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
            if grant.maximum_items == 0 { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
            match self.phase {
                PrivateOwnedPublicationPhase::Admission => {
                    let birth_bytes = member.owned_publication_birth_bytes(self.request.as_ref().expect("private admission retains its original request"))?;
                    let Some(publication) = member.begin_one_item_owned_publication(self.request.as_mut().expect("private source retained until admission"), grant)? else { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); };
                    *self.publication = Some(publication);
                    self.phase = PrivateOwnedPublicationPhase::Preparation;
                    Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.progress(), RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: birth_bytes, ..Default::default() }))
                }
                PrivateOwnedPublicationPhase::Preparation => {
                    let step = member.prepare_one_item_publication(self.publication_mut().unwrap(), grant)?;
                    if let store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, progress) = step { self.phase = PrivateOwnedPublicationPhase::Staging; return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(checkpoint, progress)); }
                    Ok(step)
                }
                PrivateOwnedPublicationPhase::Staging => {
                    self.grouped = true;
                    let step = member.stage_one_item_publication(self.publication_mut().unwrap(), visibility, grant)?;
                    if matches!(step, store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { self.phase = PrivateOwnedPublicationPhase::Staged; }
                    Ok(step)
                }
                PrivateOwnedPublicationPhase::Staged => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.progress(), Default::default())),
                _ => Err("private publication preparation requires its original pending lane".into()),
            }
        }
        /// 🔓️ Adopts the already staged root only after the exact common visibility commits.
        fn adopt(&mut self, member: &mut impl SpaceMember, visibility: &Arc<vcs::ArtifactGroupVisibility>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
            if self.adopted() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.progress(), Default::default())); }
            if !self.staged() { return Err("private publication adoption requires its fully staged candidate".into()); }
            let step = member.adopt_one_item_publication(self.publication_mut().unwrap(), visibility, grant)?;
            if matches!(step, store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { self.phase = PrivateOwnedPublicationPhase::Adopted; self.grouped = false; }
            Ok(step)
        }
        /// 📐️ Exposes independent active source, staged root, metadata and physical frame currencies.
        fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
            use semio_framework_value::RetirementDemand;
            let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "private publication retirement depth overflow"))?; Ok(demand) };
            if let Some(publication) = self.publication.as_ref() { return if publication.terminal_is_empty() { Ok(RetirementDemand { release_bytes: size_of_val(publication.as_ref()), depth: 1, ..Default::default() }) } else { nested(publication.retirement_demands(body)?) }; }
            if let Some(request) = self.request.as_ref() { return if request.mutations.terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { nested(request.mutations.next_demands(body)?) }; }
            Ok(self.metadata.as_ref().map_or(Default::default(), |metadata| RetirementDemand { release_bytes: metadata.next_close_byte_demand(), depth: 2, ..Default::default() }))
        }
        /// ♻️ Returns staged authority before retiring original sources and whole physical frames.
        fn close_step(&mut self, member: &mut impl SpaceMember, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            use semio_framework_value::retained_clone::RetainedCloneProgress;
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
            if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "private publication exceeds admitted retirement depth")); }
            if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            self.phase = PrivateOwnedPublicationPhase::Closing;
            if let Some(publication) = self.publication.as_mut() {
                if !publication.terminal_is_empty() { let step = member.abort_one_item_publication(publication.as_mut(), child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, publication.terminal_is_empty(), "private member publication")?; return Ok(RetainedCloneStep::Progress(step.progress())); }
                self.publication.take();
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
            }
            if let Some(request) = self.request.as_mut() {
                if !request.mutations.terminal_is_empty() { let step = request.mutations.close_granted(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, request.mutations.terminal_is_empty(), "private original batch")?; return Ok(RetainedCloneStep::Progress(step.progress())); }
                let request = self.request.take().unwrap();
                self.metadata = Some(PrivatePublicationMetadata::from_parts(PrivatePublicationMetadataParts { actor: request.actor, transaction: request.transaction, group_id: request.group_id }));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            if let Some(metadata) = self.metadata.as_mut() { let step = metadata.close_granted(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, metadata.terminal_is_empty(), "private publication metadata")?; if metadata.terminal_is_empty() { self.metadata = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
            self.phase = PrivateOwnedPublicationPhase::Complete;
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }
        fn terminal_is_empty(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Complete && self.request.is_none() && self.publication.is_none() && self.metadata.is_none() }
    }

    impl Drop for PrivateOwnedPublicationLane { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private typed publication retains its original source and staged authority until bounded terminal close"); } }
}

use private_publication_group::{PrivateChildGroupSource, PrivateChildPublicationGroup};

#[cfg(test)]
use private_publication_group::{PrivateChildPublicationInput, PrivateOwnedPublicationLane};
