//! 🤝️ Retains one private member root until its shared visibility owner decides.

use std::sync::Arc;
use semio_framework_os_store_engine_live::lifted_store::StoreParts;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication;
use semio_framework_os_store_read::{ArtifactStoreDisplacedOwnerReservation, ArtifactStoreDisplacedRetirements, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement};
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator, CursorRevisionRecord};
use semio_framework_os_store_one_item::{ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, LaneItemReceipt};
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase;
use semio_framework_os_spr::{Edit, Mutation, OpBinary, OpText};
use semio_framework_value::{ErasedSnapshotRetirement, FromValue, ToValue, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;

fn retain_group_owner(retirements: &mut ArtifactStoreDisplacedRetirements, reservation: &mut ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn ErasedSnapshotRetirement>) {
    if let Err(owner) = retirements.push_owner_reserved(reservation, owner) { retirements.push_reserved(owner); }
}

/// 🤝️ Publishes one store's member group under a common decision: stage while the decision is pending, adopt after it commits, abort after it refuses.
pub trait ApplyBatchGroup<P, Mu> {
    fn stage_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn adopt_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn abort_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
}

impl<P, Mu> ApplyBatchGroup<P, Mu> for StoreParts<'_, P, Mu>
where
    P: semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    Mu: semio_framework_os_spr::StoreMutationFor<P>,
{
    fn stage_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let group = publication.group_preparation.as_mut().ok_or("member group lacks its retained preparation")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.pending() { return Err("member group visibility authority does not match its pending decision".into()); }
        if self.cursor.generation != publication.expected_generation || self.cursor.content_revision != publication.expected_revision { return Err("member group became stale before its common decision".into()); }
        if group.metadata.phase == 12 { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),Default::default())); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let stage = publication.stage.as_ref().ok_or("member group lost its staged batch")?;
        if publication.phase != ArtifactStoreOneItemPublicationPhase::Publishing || stage.post.is_none() || self.wire.durable_group_root.is_some() { return Err("member group requires an exact private publication root".into()); }
        if publication.close_started || publication.cancel_requested { return Err("member group candidate was cancelled".into()); }
        let checkpoint=publication.progress();
        let metadata=&mut publication.group_preparation.as_mut().unwrap().metadata;
        let stage=publication.stage.as_mut().unwrap();
        match semio_framework_os_store_engine_bundle::advance_batch_metadata(self,metadata,stage.edit.as_ref().ok_or("member metadata lost its original edit")?,stage.digest,&mut stage.unit_flags,&mut stage.diagnostics,grant)?{
            semio_framework_os_store_engine_bundle::BatchCommitMetadataStep::Blocked=>Ok(ArtifactStoreOneItemPreparationStep::Blocked),
            semio_framework_os_store_engine_bundle::BatchCommitMetadataStep::Progress(ownership)=>Ok(ArtifactStoreOneItemPreparationStep::Progress(checkpoint,ownership)),
            semio_framework_os_store_engine_bundle::BatchCommitMetadataStep::Ready=>{
                let capacity=size_of::<semio_framework_os_store_durable_group_wire::lifted_durable_group::StagedRootRetirement<P,Mu>>();let release=size_of::<Edit<Mu>>();publication.group_preparation.as_mut().unwrap().metadata.demand=capacity;
                if grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<release{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)}
                let ownership=semio_framework_os_store_engine_stage::lifted_durable_group::stage_prebuilt_batch_root(self,publication,grant)?;let metadata=&mut publication.group_preparation.as_mut().unwrap().metadata;metadata.phase=12;metadata.completed=metadata.completed.saturating_add(1);
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(checkpoint,ownership))
            }
        }
    }

    fn adopt_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if publication.published { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),Default::default())); }
        let group = publication.group_preparation.as_ref().ok_or("member group lacks its retained adoption")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.committed() || group.metadata.phase != 12 { return Err("member adoption requires its exact committed common decision".into()); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let checkpoint=publication.progress();

        let(done,ownership)=semio_framework_os_store_engine_stage::lifted_durable_group::adopt_staged_store_member(self,visibility,grant.retained_grant()).map_err(|error|error.to_string())?;
        if !done{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        *self.wire.durable_group_root=None;publication.group_preparation=None;publication.retained_checkpoint=checkpoint;
        publication.receipt=Some(LaneItemReceipt{generation_before:publication.expected_generation,generation_after:self.cursor.generation});publication.expected_generation=self.cursor.generation;publication.expected_revision=self.cursor.content_revision;publication.published=true;publication.phase=ArtifactStoreOneItemPublicationPhase::AwaitingAck;
        Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),RetainedCloneProgress{copied_items:1,..ownership}))
    }

    fn abort_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        let error = |message: &'static str| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member group abort requires admitted depth")); }
        let Some(group) = publication.group_preparation.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if group.visibility.committed() || group.visibility.pending() { return Err(error("member abort requires its exact aborted common decision")); }
        if self.wire.durable_group_root.is_some() {
            semio_framework_os_store_engine_stage::lifted_durable_group::abort_staged_store_member(self, &group.visibility).map_err(|_| error("member abort lost its exact staged root"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(history) = group.metadata.history.take() {
            let ArtifactStoreHistoryCommitReservation { history, rejected_owner } = history;
            if let Err(history) = self.log.envelope.vcs.edits.cancel_reservation(history) { group.metadata.history = Some(ArtifactStoreHistoryCommitReservation { history, rejected_owner }); return Err(error("member abort lost its exact ledger reservation")); }
            self.owners.displaced_retirements.release_owner_slots(rejected_owner).map_err(|_| error("member abort lost its rejected owner reservation"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(reservation) = group.metadata.displaced.as_mut() {
            if group.metadata.applied.capacity() != 0 {
                let original = std::mem::replace(&mut group.metadata.applied, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.metadata.applied = original; return Err(error); } }
            }
            if group.metadata.cursor.capacity() != 0 {
                let original = std::mem::replace(&mut group.metadata.cursor, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.metadata.cursor = original; return Err(error); } }
            }
            if let Some(original) = group.metadata.revision.take() { match admit_artifact_retirement(original, grant, ArtifactStoreRevisionAccumulatorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.metadata.revision = Some(original); return Err(error); } } }
            if let Some(original) = group.metadata.checkpoint.take() { match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.metadata.checkpoint = Some(original); return Err(error); } } }
            self.owners.displaced_retirements.release_owner_slots(group.metadata.displaced.take().unwrap()).map_err(|_| error("member abort lost its displaced owner reservation"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let released_bytes = group.retirement_demands()?.release_bytes;
        if grant.maximum_release_bytes < released_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        publication.begin_close();
        publication.group_preparation = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }))
    }
}
