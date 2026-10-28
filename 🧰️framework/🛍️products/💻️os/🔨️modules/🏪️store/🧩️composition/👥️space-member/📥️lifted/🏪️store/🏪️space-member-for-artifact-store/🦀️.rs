//! 🏪️ Space member for artifact store: the space member implementation of the artifact store over its one item publication, snapshot reads, checkpoints, undo, redo, wire dispatch and pack operations. See [`super`].

use super::*;
use semio_framework_os_store_engine_member_group::ApplyBatchGroup as _;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️BlobStore
//#region 🔖️BlobStore
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️Backbone
//#region 🔖️BlobStore
//#region 🔖️BlobStore
impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
where
    P: ToValue + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn member_native_vcs_capture_demands(&self)->semio_framework_value::RetirementDemand{native_vcs::capture_demands::<P, Mutation>()}
    fn admit_member_native_vcs_command(&self,request:&mut Option<semio_framework_os_store_engine::lifted_store::ArtifactNativeVcsRequest>,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<Box<dyn MemberNativeVcsCommand>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{native_vcs::admit(self,request,cx,output)}
    fn advance_member_native_vcs_command(&mut self,owner:&mut dyn MemberNativeVcsCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_os_store_engine::lifted_store::ArtifactNativeVcsAdvance,semio_framework_job::OriginalTurnFailure>{native_vcs::advance(self,owner,cx)}
    fn take_member_native_vcs_result(&self,owner:&mut dyn MemberNativeVcsCommand,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<semio_framework_os_store_engine::lifted_store::ArtifactNativeVcsResult>)->Result<bool,semio_framework_job::OriginalTurnFailure>{native_vcs::take(self,owner,cx,output)}
    fn ack_member_native_vcs_command(&self,owner:&mut dyn MemberNativeVcsCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{native_vcs::acknowledge(self,owner,cx)}
    fn member_native_vcs_close_demands(&self,owner:&dyn MemberNativeVcsCommand,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{native_vcs::close_demands(self,owner,body)}
    fn close_member_native_vcs_command(&mut self,owner:&mut dyn MemberNativeVcsCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{native_vcs::close(self,owner,cx)}
    fn native_head_pack_source(&self)->Option<semio_framework_os_store_text_io::OriginalSnapshotPackView<'_>>{self.snapshot_ref().native_pack_source()}
    fn native_pack_generation(&self)->u64{self.generation()}
    fn native_checkpoint_id(&self)->Option<&str>{self.current_checkpoint_id()}
    fn native_checkpoint_probe_demands(&self,probe:&semio_framework_os_vcs::io::binary::checkpoint_identity::original::pending::OriginalPendingEditIds)->Result<semio_framework_value::RetirementDemand,ValueError>{probe.demands(&self.parts_ref().log.applied_edit_ids)}
    fn advance_native_checkpoint_probe(&self,probe:&mut semio_framework_os_vcs::io::binary::checkpoint_identity::original::pending::OriginalPendingEditIds,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{{let parts=self.parts_ref();probe.advance(self.generation(),&parts.log.applied_edit_ids,&parts.log.envelope.vcs.changes,grant,|capacity|if capacity==0{Ok(())}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"child checkpoint status cannot allocate pending edit identities"))})}}
    fn member_identity(&self)->Option<MemberIdentityRef<'_>>{let envelope=self.envelope();Some(MemberIdentityRef{artifact_id:&envelope.id,dialect:envelope.dialect.as_ref()?,owner:envelope.owner.as_ref()})}
    fn local_actor_id(&self) -> &ActorId {
        ArtifactStore::local_actor_id(self)
    }

    async fn document_id(&self) -> &str {
        self.envelope().id.as_str()
    }

    fn artifact_ref(&self) -> Option<semio_framework_artifact_reference::ArtifactRef> {
        self.envelope().dialect.clone().map(|dialect| semio_framework_artifact_reference::ArtifactRef { artifact_id: self.envelope().id.clone(), dialect })
    }

    fn owner_ref(&self) -> Option<OwnerRef> {
        self.envelope().owner.clone()
    }

    fn child_restore_projection(&self) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError> {
        ChildRestoreProjection::from_snapshot(self.snapshot_ref())
    }
    fn child_restore_projection_for_read<'read>(&self, read:&'read ErasedSnapshotRead)->Result<ChildRestoreProjection<'read>,ChildRestoreProjectionError> {
        if !read.belongs_to(&self.owners.snapshot_read_leases){return Err(ChildRestoreProjectionError::InvalidReference);}
        ChildRestoreProjection::from_snapshot(read.get::<P>().ok_or(ChildRestoreProjectionError::InvalidReference)?)
    }

    fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
        (self.generation(), self.content_revision())
    }

    fn one_item_wire_publication_supported(&self) -> bool {
        self.owners.one_item_wire_preparation_factory.is_some() && self.owners.snapshot_retirement_factory.is_some() && self.owners.mutation_retirement_factory.is_some() && self.wire.backbone.is_none() && !self.owners.owned_disposer_terminal
    }



    fn original_batch_receiving_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.log.operation_wire.as_ref().is_none_or(|wire|wire.original_receiving.is_none()) || self.owners.mutation_retirement_factory.is_none() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "member requires its installed original operation receiver and retirement issuer")); }
        Ok(semio_framework_os_store_one_item_batch::member_owned_batch::OriginalMemberBatchReceiving::<Mutation>::admission_demands())
    }
    fn admit_original_batch_receiving(&self, wires: &mut Option<Vec<Vec<u8>>>, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn semio_framework_os_store_one_item_batch::member_owned_batch::original_receiving::MemberOriginalBatchReceiving>, RetainedCloneProgress)>, ValueError> {
        self.original_batch_receiving_demands()?;
        semio_framework_os_store_one_item_batch::member_owned_batch::OriginalMemberBatchReceiving::<Mutation>::admit(wires, self.log.operation_wire.ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"member has no installed operation wire"))?, self.owners.mutation_retirement_factory.as_ref().unwrap(), control, grant)
    }
    fn owned_publication_birth_bytes(&self, request: &MemberStoreOwnedBatchRequest) -> Result<usize, String> {
        if request.mutations.mutations::<Mutation>().is_none() { return Err("member batch mutation type does not match its exact store".into()); }
        let factory = self.owners.one_item_preparation_factory.as_ref().filter(|_| self.owners.snapshot_retirement_factory.is_some() && self.owners.mutation_retirement_factory.is_some() && self.wire.backbone.is_none() && !self.owners.owned_disposer_terminal)
            .ok_or_else(|| "member requires its exact retained typed preparation and retirement authority".to_string())?;
        if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("member batch metadata exceeds its fixed identity capacity".into());
        }
        let authority = std::alloc::Layout::new::<[std::sync::atomic::AtomicUsize; 2]>().extend(std::alloc::Layout::new::<ArtifactStoreOneItemLiveAuthority>()).map_err(|_| "member authority layout is unrepresentable".to_string())?.0.pad_to_align().size();
        let source = size_of::<ArtifactStoreBatchSourceOf<P, Mutation, ArtifactTypedBatchItemAuthority<P, Mutation>>>();
        let publication = size_of::<MemberStoreOneItemPublication<P, Mutation>>();
        let metadata = [request.group_id.as_ref().map_or(0, String::len), self.log.envelope.active_alternative_id.as_ref().map_or(0, String::len), request.transaction.as_ref().map_or(0, |transaction| transaction.id.len()), request.transaction.as_ref().map_or(0, |transaction| transaction.tool.len()), factory.stamped_mutation_id().map_or(0, |identity| identity.0.len())];
        metadata.into_iter().try_fold(authority.checked_add(source).and_then(|bytes| bytes.checked_add(publication)).ok_or_else(|| "member publication frame size overflows".to_string())?, |total, bytes| total.checked_add(bytes).ok_or_else(|| "member publication metadata size overflows".to_string()))
    }

    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest, grant: ArtifactStoreOneItemGrant) -> Result<Option<(Box<dyn ErasedMemberStoreOneItemPublication>, semio_framework_value::RetainedCloneProgress)>, String> {
        if grant.maximum_items == 0 { return Ok(None); }
        let birth_bytes = self.owned_publication_birth_bytes(request)?;
        if grant.maximum_capacity_bytes < birth_bytes { return Ok(None); }
        let metadata_copy_bytes = size_of::<semio_framework_value::SharedUtf8>() + request.group_id.as_ref().map_or(0, String::len) + request.transaction.as_ref().map_or(0, |transaction| transaction.id.len() + transaction.tool.len()) + self.log.envelope.active_alternative_id.as_ref().map_or(0,String::len) + self.owners.one_item_preparation_factory.as_ref().and_then(|factory|factory.stamped_mutation_id()).map_or(0,|id|id.0.len());
        if grant.maximum_copy_bytes < metadata_copy_bytes { return Ok(None); }
        if request.mutations.mutations::<Mutation>().is_none() { return Err("member batch mutation type does not match its exact store".into()); }
        let factory = self.owners.one_item_preparation_factory.as_ref().filter(|_| self.owners.snapshot_retirement_factory.is_some() && self.owners.mutation_retirement_factory.is_some() && self.wire.backbone.is_none() && !self.owners.owned_disposer_terminal)
            .ok_or_else(|| "member requires its exact retained typed preparation and retirement authority".to_string())?;
        if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("member batch metadata exceeds its fixed identity capacity".into());
        }
        let issuer = request.mutations.source_retirement_issuer::<Mutation>().ok_or_else(|| "member batch lost its exact admitted retirement issuer".to_string())?;
        let inputs = request.mutations.take_mutations::<Mutation>().ok_or_else(|| "member batch source is already transferred or closing".to_string())?;
        let source = ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority { preparation: Arc::clone(factory), retirement: Arc::clone(self.owners.mutation_retirement_factory.as_ref().expect("admitted original mutation retirement authority")) }, inputs, Some(issuer), self.log.operation_wire);
        match self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor.clone(), HistoryLane::Document, request.group_id.clone(), source, false, request.transaction.clone()) {
            Ok(publication) => Ok(Some((Box::new(MemberStoreOneItemPublication { member: Some(SnapshotReadRegistryAliasRetirement::new(self.owners.snapshot_read_leases.clone())), publication, group_history: None, group_displaced: None }), semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:metadata_copy_bytes,retained_capacity_bytes:birth_bytes,..Default::default()}))),
            Err(rejected) => {
                let (reason, inputs) = rejected.into_owners();
                request.mutations.restore_mutations(inputs);
                Err(reason)
            }
        }
    }

    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemAdvance, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member publication concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) {
            return Err(semio_framework_os_vcs::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member publication belongs to a different exact store authority")));
        }
        if publication.group_history.is_some() || publication.group_displaced.is_some() {
            return Err(semio_framework_os_vcs::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"group-reserved member candidate requires an atomic group visibility authority")));
        }
        self.advance_apply_batch(&mut publication.publication, body, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant },receiving).map_err(|error| error.to_string())
    }

    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group preparation concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) {
            return Err(semio_framework_os_vcs::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member group preparation belongs to a different exact store authority")));
        }
        if self.cursor.generation != publication.publication.expected_generation || self.cursor.content_revision != publication.publication.expected_revision {
            publication.publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member group preparation became stale before visibility publication"));
            publication.publication.begin_close();
            return Err(semio_framework_os_vcs::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member group preparation became stale before visibility publication")));
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if publication.publication.phase() == ArtifactStoreOneItemPublicationPhase::Publishing {
            if publication.group_history.is_some() && publication.group_displaced.is_some() {
                return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.publication.progress(),Default::default()));
            }
            if publication.group_history.is_none(){
                let capacity=self.log.envelope.vcs.edits.next_reservation_allocation_bytes();
                if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                publication.group_history=Some(self.reserve_edit_history_slot()?);
                return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.publication.progress(),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}));
            }
            publication.group_displaced=Some(self.owners.displaced_retirements.reserve_owner_slots(12)?);
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.publication.progress(),RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        if matches!(publication.publication.phase(), ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Fault) {
            return Err(semio_framework_os_vcs::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member group preparation is no longer in a prepublication phase")));
        }
        match self.advance_apply_batch(&mut publication.publication, body, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant },receiving).map_err(|error| error.to_string())? {
            ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint,ownership) => Ok(ArtifactStoreOneItemPreparationStep::Progress(checkpoint,ownership)),
            ArtifactStoreOneItemAdvance::Progress(_) => Err(semio_framework_os_vcs::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member preparation returned an unmeasured publication turn"))),
            ArtifactStoreOneItemAdvance::Blocked => Ok(ArtifactStoreOneItemPreparationStep::Blocked),
            _ => Err(semio_framework_os_vcs::VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member group preparation crossed its reserved prepublication boundary"))),
        }
    }

    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group stage concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return Err("member group stage belongs to a different exact store authority".into()); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        if publication.publication.group_preparation.is_none() {
            let copy=size_of::<semio_framework_os_store_one_item_batch::lifted_member_group::MemberGroupPreparation>();
            if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
            publication.publication.group_preparation=Some(semio_framework_os_store_one_item_batch::lifted_member_group::MemberGroupPreparation::new(visibility,publication.group_history.take(),publication.group_displaced.take()));
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
        }
        self.parts().stage_apply_batch_group(&mut publication.publication, visibility, grant)
    }

    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group adoption concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return Err("member group adoption belongs to a different exact store authority".into()); }
        self.parts().adopt_apply_batch_group(&mut publication.publication, visibility, grant)
    }

    fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, original:&mut ::semio_framework_value::OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>()
            .ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort concrete owner type does not match"))?;
        if publication.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort belongs to a different exact store authority"));
        }
        if publication.publication.group_preparation.is_some() {
            let demand=publication.retirement_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}original.charge(demand.capacity_bytes)?;
            return self.parts().abort_apply_batch_group(&mut publication.publication, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        publication.publication.begin_close();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member abort requires admitted depth")); }
        if let Some(reservation) = publication.group_history.take() {
            let ArtifactStoreHistoryCommitReservation { history, rejected_owner } = reservation;
            if let Err(history) = self.log.envelope.vcs.edits.cancel_reservation(history) {
                publication.group_history = Some(ArtifactStoreHistoryCommitReservation { history, rejected_owner });
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort lost its exact history reservation"));
            }
            self.owners.displaced_retirements.release_owner_slots(rejected_owner).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(reservation) = publication.group_displaced.take() {
            self.owners.displaced_retirements.release_owner_slots(reservation).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        publication.close_original_step(original,RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant })
    }

    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError> {
        let demand = ErasedSnapshotRead::prepared_read_demands();
        if !grant.permits_one() || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(None); }
        let reject = |reason| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason);
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| reject("prepared member read concrete owner type does not match"))?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return Err(reject("prepared member read belongs to a different exact store authority")); }
        if !visibility.pending() || self.cursor.generation != publication.publication.expected_generation || self.cursor.content_revision != publication.publication.expected_revision { return Err(reject("prepared member read requires its pending unchanged store authority")); }
        let root = self.wire.durable_group_root.as_ref().ok_or_else(|| reject("prepared member read requires a fully staged common root"))?;
        if !Arc::ptr_eq(&root.visibility, visibility) { return Err(reject("prepared member read visibility authority does not match")); }
        let owner = Arc::clone(root.current.as_ref().ok_or_else(|| reject("prepared member read lost its immutable candidate"))?);
        let lease = match self.owners.snapshot_read_leases.try_issue(Arc::clone(&owner)) { Ok(lease) => lease, Err(alias) => { drop(alias); return Ok(None); } };
        Ok(Some((root.generation, root.content_revision, ErasedSnapshotRead::new(owner, lease), RetainedCloneProgress { copied_items: 1, ..Default::default() })))
    }

    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)> {
        let reject = |snapshot, reason: &'static str| Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason), snapshot));
        let Some(publication) = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>() else { return reject(snapshot, "prepared read return concrete publication type does not match"); };
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return reject(snapshot, "prepared read return belongs to a different exact member"); }
        let Some(root) = self.wire.durable_group_root.as_ref() else { return reject(snapshot, "prepared read return requires its retained staged root"); };
        if visibility.committed() || !Arc::ptr_eq(&root.visibility, visibility) { return reject(snapshot, "prepared read return requires its exact undecided or aborted visibility"); }
        if !root.current.as_ref().is_some_and(|candidate| snapshot.get::<P>().is_some_and(|borrowed| std::ptr::eq(candidate.as_ref(), borrowed))) { return reject(snapshot, "prepared read return does not retain this exact private candidate"); }
        if grant.maximum_items==0 { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"prepared read return requires its supplied item"),snapshot)); }
        let demand = ErasedSnapshotRead::prepared_return_demands();
        if grant.maximum_depth < demand.depth { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared read return requires its supplied depth"),snapshot)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"prepared read return requires its original full physical authority"),snapshot)); }
        match snapshot.into_typed::<P>(&self.owners.snapshot_read_leases) {
            Ok(owner) => { drop(owner); Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })) }
            Err(snapshot) => reject(snapshot, "prepared read return exact lease registry is busy or stale"),
        }
    }

    fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
        if !self.owners.snapshot_read_leases.publish_authority(self.generation(), self.content_revision()) {
            return Err("snapshot read commit authority is busy or exhausted".into());
        }
        let owner = self.snapshot_owner();
        let lease = self.owners.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| "snapshot read lease registry is busy, saturated, or exhausted".to_string())?;
        Ok(ErasedSnapshotRead::new(owner, lease))
    }

    async fn snapshot_read_erased(&self) -> Result<ErasedSnapshotRead, String> {
        self.snapshot_read_erased_now()
    }

    fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        let Some(read) = snapshot.as_mut() else { return Ok((None, Default::default())); };
        if grant.maximum_items == 0 { return Ok((None, Default::default())); }
        let factory = self.owners.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its installed original factory"))?;
        let admitted = read.admit_retirement::<P>(&self.owners.snapshot_read_leases, factory, grant)?;
        snapshot.take();
        Ok((Some(admitted.0), admitted.1))
    }

    fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        ArtifactStore::take_returned_snapshot_read_retirement(self, grant)
    }

    fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand {
        ArtifactStore::snapshot_read_retirement_demand(self)
    }

    fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        ArtifactStore::returned_snapshot_read_retirement_demand(self)
    }

    fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        ArtifactStore::snapshot_read_leases_terminal_is_empty(self)
    }

    fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.close_owned_store_step(grant)
    }

    fn close_owned_terminal_is_empty(&self) -> bool {
        self.close_owned_store_terminal_is_empty()
    }
    fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        ArtifactStore::close_owned_demands(self, maximum_body_bytes)
    }

    fn content_revision_now(&self) -> [u8; 32] {
        self.content_revision()
    }

    async fn content_revision(&self) -> [u8; 32] {
        self.content_revision()
    }

    async fn is_dirty(&self) -> bool {
        !uncommitted_edit_ids(&self.log.envelope, self.applied_edit_ids()).await.is_empty()
    }

    /// 🏁️ `self.current_checkpoint_id()` resolves to the inherent method (`Option<&str>`), not this
    /// trait method — Rust prefers inherent methods over trait methods of the same name.



    async fn current_checkpoint_id(&self) -> Option<String> {
        self.current_checkpoint_id().map(|id| id.to_string())
    }

    async fn current_alternative_id(&self) -> Option<String> {
        self.envelope().active_alternative_id.clone()
    }





    /// 🌀️ `edit_is_local`/`envelope` are async; `find_map`'s closure is sync (R10 shape 1), so both
    /// functions below use an explicit loop instead.
    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        let envelope = self.envelope();
        for edit_id in self.applied_edit_ids().iter().rev() {
            if !self.log.edit_is_local(edit_id) {
                continue;
            }
            if let Some(timestamp) = envelope.vcs.edits.find_near(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp) {
                return Some(timestamp);
            }
        }
        None
    }

    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        let envelope = self.envelope();
        for edit_id in self.redo_edit_ids().iter().rev() {
            if !self.log.edit_is_local(edit_id) {
                continue;
            }
            if let Some(timestamp) = envelope.vcs.edits.find_near(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp) {
                return Some(timestamp);
            }
        }
        None
    }





    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    /// 🌀️ `MutationMessage::at_op` is async (📡️replication); `Iterator::map`'s closure is
    /// sync (R10 shape 1), so it's hoisted into an explicit loop.





    /// 🌀️ The unawaited future holds `&mut self`, so `self.merge_policy` cannot be reassigned
    /// while it's still live (E0506) — awaited immediately instead of deferred, matching the
    /// original synchronous "restore after dispatch completes" intent.









    async fn tail_group_id(&self) -> Option<String> {
        let edit_id = self.applied_edit_ids().last()?;
        self.envelope().vcs.edits.find_near(|edit| edit.id == *edit_id)?.mutation_meta.last()?.group_id.clone()
    }

    async fn tail_edit_id(&self) -> Option<String> {
        self.applied_edit_ids().last().cloned()
    }

    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
        let edit_id = self.redo_edit_ids().last()?.clone();
        let group_id = self.envelope().vcs.edits.find_near(|edit| edit.id == edit_id).and_then(|edit| edit.mutation_meta.last()).and_then(|meta| meta.group_id.clone());
        Some((edit_id, group_id))
    }















    /// 🎯️ Overrides the trait default (`MergePolicy::default()`) now that lane 1-A's real
    /// `ArtifactStore::merge_policy` inherent method has landed (§C6) — `self.merge_policy()`
    /// resolves to the INHERENT method here (Rust prefers it over the trait one of the same name),
    /// exactly the `current_checkpoint_id` precedent this file already documents.
    async fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy {
        self.merge_policy()
    }
}
//#endregion 🔖️BlobStore
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️BlobStore
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
