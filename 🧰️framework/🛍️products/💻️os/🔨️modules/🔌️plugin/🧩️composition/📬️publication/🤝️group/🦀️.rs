use semio_framework::kernel::ActorId;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_child::ChildMemberAdmission;
use semio_framework_plugin_child::ChildMemberRegistry;
use semio_framework::Fault;
use std::mem::ManuallyDrop;
use store::MemberFactory;
use semio_framework_plugin_decl_views::child_view::MemberKey;
use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
use semio_framework_plugin_decl_emission::child_emit_preparation::OwnedChildEmit;
#[cfg(test)]
use semio_framework_plugin_decl_views::child_view::PreparedChildContentEntry;
use semio_framework_plugin_decl_views::child_view::PreparedChildContentIdentity;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::PrivateChildMemberMetadata;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::PrivateChildMemberMetadataIssuer;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::PrivateChildMemberMetadataSource;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::PrivatePublicationMetadata;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::PrivatePublicationMetadataIssuer;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::PrivatePublicationMetadataParts;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::PrivatePublicationMetadataSource;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use store::SpaceMember;
use semio_framework_value::ValueError;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
#[doc(hidden)]
#[path="⚠️failure/🦀️.rs"]pub mod member_publication_failure;
#[path="🪪️heads/🦀️.rs"]pub mod qualified_heads;
#[doc(hidden)]
pub mod private_publication_group {
    use super::*;
    use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    use std::mem::ManuallyDrop;
use std::sync::Arc;

    #[doc(hidden)]
pub mod child { include!("🪆️child/🦀️.rs"); }
    pub use crate::group::private_publication_group::child::PrivateChildPublicationInput;
    #[doc(hidden)]
pub mod owner { include!("📦️owner/🦀️.rs"); }
    pub use super::member_publication_failure as failure;
    pub use super::qualified_heads;
    pub use crate::group::private_publication_group::owner::{PrivateChildGroupSource, PrivateChildPublicationGroup, PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN};

    fn advance_original_lane(lane:&mut PrivateOwnedPublicationLane,member:&mut impl SpaceMember,visibility:&Arc<vcs::ArtifactGroupVisibility>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,failure:&mut Option<failure::OriginalMemberPublicationFailure>)->Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep,semio_framework_job::OriginalTurnFailure>{
        cx.with_retained_loan(grant,|same|{let original=same.retained_grant();let capture=2*size_of::<Option<failure::OriginalMemberPublicationFailure>>();if original.maximum_items==0||original.maximum_copy_bytes<capture{return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked)}let item=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(RetainedCloneGrant{maximum_copy_bytes:original.maximum_copy_bytes-capture,..original});let result=same.with_native_receiving(|receiving|lane.advance(member,visibility,item.maximum_copy_bytes,item,receiving));let mut step=match result{Ok(step)=>step,Err(source)=>{let p=match &source{store::VcsError::InverseRefused(e)|store::VcsError::NativeEncoding(e)=>e.retained_progress(),_=>Default::default()};assert!(failure.is_none(),"original member failure cannot replace an unreported original cause");*failure=Some(failure::OriginalMemberPublicationFailure{source,effects:p,receipt_refusal:None});store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(lane.progress(),p)}};let mut p=step.ownership_progress();if p!=RetainedCloneProgress::default()||failure.is_some(){p.copied_bytes=p.copied_bytes.checked_add(capture).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original member failure capture extent overflow"))?;p.copied_items=p.copied_items.max(1);step=match step{store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(checkpoint,_)=>store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(checkpoint,p),_=>store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(lane.progress(),p)}}if let Some(source)=failure.as_mut(){source.effects=p}if let Err(receipt)=same.consume_receiving_retained(original,p){if let Some(source)=failure.as_mut(){source.receipt_refusal=Some(receipt)}else{return Err(receipt.into())}}Ok(step)})
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PrivateOwnedPublicationPhase { Admission, Preparation, Staging, Staged, Adopted, Closing, Complete }

    #[doc(hidden)]
pub struct PrivateOwnedPublicationLane {
        pub request: ManuallyDrop<Option<store::MemberStoreOwnedBatchRequest>>,
        pub publication: ManuallyDrop<Option<Box<dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>>>,
        pub metadata: Option<PrivatePublicationMetadata>,
        pub phase: PrivateOwnedPublicationPhase,
        pub grouped: bool,
        pub origins_assigned:bool,
    }

    impl PrivateOwnedPublicationLane {
        /// 🪪️ Retains one original typed source and its independently admitted metadata.
        pub fn new(request: store::MemberStoreOwnedBatchRequest) -> Self { Self { request: ManuallyDrop::new(Some(request)), publication: ManuallyDrop::new(None), metadata: None, phase: PrivateOwnedPublicationPhase::Admission, grouped: false,origins_assigned:false } }
        pub fn request(&self) -> Option<&store::MemberStoreOwnedBatchRequest> { self.request.as_ref() }
        pub fn publication_mut(&mut self) -> Option<&mut (dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication + 'static)> { self.publication.as_deref_mut() }
        pub fn staged(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Staged }
        pub fn prepared_for_staging(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Staging }
        pub fn prepared_edit_id(&self) -> Option<&str> { self.prepared_for_staging().then(|| self.publication.as_ref()?.prepared_edit_id()).flatten() }
        pub fn prepared_publication(&self) -> Option<&dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication> { if self.prepared_for_staging() { self.publication.as_deref() } else { None } }
        fn adopted(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Adopted }
        pub fn progress(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.publication.as_ref().map_or(Default::default(), |publication| publication.progress()) }
        /// 🎟️ Borrows the complete constructor extent before any typed source handoff.
        pub fn next_capacity_byte_demand(&self, member: &impl SpaceMember) -> Result<usize, String> {
            if self.phase == PrivateOwnedPublicationPhase::Admission { return member.owned_publication_birth_bytes(self.request.as_ref().expect("private admission retains its original request")); }
            Ok(self.publication.as_ref().map_or(0, |publication| publication.next_group_byte_demand()))
        }
        /// 🧬️ Moves this member's actual provenance rows before the first semantic preparation turn.
        pub fn advance_original_origins(&mut self,origins:&mut Option<std::collections::VecDeque<protocol::MutationOrigin>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
            if self.origins_assigned||self.publication.is_none(){return Ok(None)}
            if self.phase!=PrivateOwnedPublicationPhase::Preparation{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original group provenance must precede semantic preparation"))}
            let overhead=size_of::<bool>();if grant.maximum_items==0||grant.maximum_copy_bytes<overhead||grant.maximum_depth==0{return Ok(Some(Default::default()))}
            let step=self.publication.as_mut().unwrap().admit_original_origins(origins,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant})?;let mut progress=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.origins_assigned=true;progress.copied_bytes+=overhead;}Ok(Some(progress))
        }
        /// 🪜️ Advances only one retained lane phase; staging never changes live authority.
        pub fn advance(&mut self, member: &mut impl SpaceMember, visibility: &Arc<vcs::ArtifactGroupVisibility>, body: usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, receiving: &mut store::MemberNativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
            if grant.maximum_items == 0 { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
            match self.phase {
                PrivateOwnedPublicationPhase::Admission => {
                    let Some((publication,progress)) = member.begin_one_item_owned_publication(self.request.as_mut().expect("private source retained until admission"), grant)? else { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); };
                    *self.publication = Some(publication);
                    self.phase = PrivateOwnedPublicationPhase::Preparation;
                    Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.progress(), progress))
                }
                PrivateOwnedPublicationPhase::Preparation => {
                    let step = member.prepare_one_item_publication(self.publication_mut().unwrap(), body, grant, receiving)?;
                    if let store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, progress) = step { self.phase = PrivateOwnedPublicationPhase::Staging; return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(checkpoint, progress)); }
                    Ok(step)
                }
                PrivateOwnedPublicationPhase::Staging => {
                    self.grouped = true;
                    let step = member.stage_one_item_publication(self.publication_mut().unwrap(), visibility, grant).map_err(store::VcsError::ValidationFailed)?;
                    if matches!(step, store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { self.phase = PrivateOwnedPublicationPhase::Staged; }
                    Ok(step)
                }
                PrivateOwnedPublicationPhase::Staged => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.progress(), Default::default())),
                _ => Err(store::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private publication preparation requires its original pending lane"))),
            }
        }
        /// 🔓️ Adopts the already staged root only after the exact common visibility commits.
        pub fn adopt(&mut self, member: &mut impl SpaceMember, visibility: &Arc<vcs::ArtifactGroupVisibility>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
            if self.adopted() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.progress(), Default::default())); }
            if !self.staged() { return Err("private publication adoption requires its fully staged candidate".into()); }
            let step = member.adopt_one_item_publication(self.publication_mut().unwrap(), visibility, grant)?;
            if matches!(step, store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { self.phase = PrivateOwnedPublicationPhase::Adopted; self.grouped = false; }
            Ok(step)
        }
        /// 📐️ Exposes independent active source, staged root, metadata and physical frame currencies.
        pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
            use semio_framework_value::RetirementDemand;
            let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "private publication retirement depth overflow"))?; Ok(demand) };
            if let Some(publication) = self.publication.as_ref() { return if publication.terminal_is_empty() { Ok(RetirementDemand { release_bytes: size_of_val(publication.as_ref()), depth: 1, ..Default::default() }) } else { nested(publication.retirement_demands(body)?) }; }
            if let Some(request) = self.request.as_ref() { return if request.mutations.terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { nested(request.mutations.next_demands(body)?) }; }
            Ok(self.metadata.as_ref().map_or(Default::default(), |metadata| RetirementDemand { release_bytes: metadata.next_close_byte_demand(), depth: 2, ..Default::default() }))
        }
        /// ♻️ Returns staged authority before retiring original sources and whole physical frames.
        pub fn close_step(&mut self, member: &mut impl SpaceMember,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            use semio_framework_value::retained_clone::RetainedCloneProgress;
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
            if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "private publication exceeds admitted retirement depth")); }
            if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            self.phase = PrivateOwnedPublicationPhase::Closing;
            if let Some(publication) = self.publication.as_mut() {
                if !publication.terminal_is_empty() { let step = cx.with_native_retirement(|retirement|retirement.with_original_close(|original|member.abort_one_item_publication(publication.as_mut(),original,child)))?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, publication.terminal_is_empty(), "private member publication")?; return Ok(RetainedCloneStep::Progress(step.progress())); }
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
        pub fn terminal_is_empty(&self) -> bool { self.phase == PrivateOwnedPublicationPhase::Complete && self.request.is_none() && self.publication.is_none() && self.metadata.is_none() }
    }

    impl Drop for PrivateOwnedPublicationLane { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private typed publication retains its original source and staged authority until bounded terminal close"); } }
}

pub use crate::group::private_publication_group::owner::{PrivateChildGroupSource, PrivateChildPublicationGroup};

#[cfg(test)]
pub use crate::group::private_publication_group::child::PrivateChildPublicationInput;
#[cfg(test)]
pub use crate::group::private_publication_group::PrivateOwnedPublicationLane;
