//! 📂️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use super::*;
use crate::tool_jobs::time_travel;

impl<A: ArtifactApp> ConstructorState<A> {
    pub(crate) fn has_constructor_receipt(&self)->bool { self.constructor_terminal_receipt.is_some() || (self.constructor_recipient_admitted && unsafe { self.constructor_receipts.assume_init_ref() }.iter().any(Option::is_some)) }

    pub(crate) fn constructor_receipt(&self)->Option<&(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if let Some(receipt)=self.constructor_terminal_receipt.as_ref(){return Some(receipt)} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_ref()}.iter().find_map(Option::as_ref) }

    pub(crate) fn take_constructor_receipt(&mut self)->Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if self.constructor_terminal_receipt.is_some(){return self.constructor_terminal_receipt.take()} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_mut()}.iter_mut().find_map(Option::take) }

    pub(crate) fn store_constructor_is_pending(&self)->bool { (!self.constructor_recipient_admitted&&!self.constructor_recipient_retired) || self.has_constructor_receipt() || self.constructor_lane<4 }

    pub(crate) fn constructor_terminal_is_empty(&self)->bool{self.constructor_recipient_retired&&!self.has_constructor_receipt()&&self.constructor_document.terminal_is_empty()&&self.constructor_config.terminal_is_empty()&&self.constructor_draft.terminal_is_empty()&&self.constructor_interaction.terminal_is_empty()}
}

impl<A: ArtifactApp> InstanceStores<A> {
    pub(crate) async fn validate_framework_reserved_commit(&self, refusal: Option<Fault>, verb: &str, permit: &FrameworkReservedCommitPermit) -> Result<(), Fault> {
        if let Some(fault) = refusal {
            return Err(fault);
        }
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{verb}' was cancelled before commit")));
        }
        let live_revision_bytes = self.store.content_revision();
        let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
        let live_generation = semio_framework_job::Generation(self.store.generation());
        if !matches!(semio_framework_job::validate_commit(&permit.operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.stale"), format!("framework route '{verb}' reached commit with a stale revision or generation")));
        }
        Ok(())
    }

    pub(crate) fn drive_store_constructor_unit(&mut self, constructor: &mut ConstructorState<A>,grant:RetainedCloneGrant)->Result<bool,ValueError> {self.drive_store_constructor_unit_in_phase(constructor, grant, ArtifactStoreConstructorPhase::Preparation)}

    pub(crate) fn drive_store_constructor_unit_in_phase(&mut self, constructor: &mut ConstructorState<A>,grant:RetainedCloneGrant,phase:ArtifactStoreConstructorPhase)->Result<bool,ValueError> {
        if constructor.has_constructor_receipt(){return Ok(true)}
        if !constructor.constructor_recipient_admitted {
            let copied_bytes=0usize;
            if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(true)}
            let recipient=constructor.constructor_receipts.write([None;24]);constructor.constructor_recipient_admitted=true;
            recipient[0]=Some((ArtifactStoreConstructorKind::Recipient,phase,grant,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}));return Ok(true)
        }
        if constructor.constructor_lane>=4{return Ok(false)}
        let copied_bytes=0usize;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth<2{return Ok(true)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-copied_bytes,maximum_depth:grant.maximum_depth-1,..grant};
        let (kind,mut receipt,done)=match constructor.constructor_lane {
            0=>{let receipt=constructor.constructor_document.advance(&mut self.store,child,A::build_document_store_owners)?;(ArtifactStoreConstructorKind::Document,receipt,constructor.constructor_document.terminal_is_empty())},
            1=>{let receipt=constructor.constructor_config.advance(&mut self.config_store,child,A::build_config_store_owners)?;(ArtifactStoreConstructorKind::Config,receipt,constructor.constructor_config.terminal_is_empty())},
            2=>{let receipt=constructor.constructor_draft.advance(&mut self.draft_store,child,A::build_draft_store_owners)?;(ArtifactStoreConstructorKind::Draft,receipt,constructor.constructor_draft.terminal_is_empty())},
            _=>{let receipt=constructor.constructor_interaction.advance(&mut self.interaction_store,child,|grant|Some(crate::local_interaction::retirement::interaction_store_owners(grant)))?;(ArtifactStoreConstructorKind::Interaction,receipt,constructor.constructor_interaction.terminal_is_empty())},
        };
        if receipt==Default::default(){return Ok(true)}
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog parent exceeded its full grant"))}
        if done { constructor.constructor_lane+=1; }
        receipt.copied_bytes=receipt.copied_bytes.checked_add(copied_bytes).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original catalog outer header extent overflow"))?;
        unsafe{constructor.constructor_receipts.assume_init_mut()[0]=Some((kind,phase,grant,receipt));}Ok(true)
    }

    pub(crate) fn constructor_close_demands(&self, constructor: &ConstructorState<A>,body:usize)->Result<RetirementDemand,ValueError>{
        if constructor.has_constructor_receipt(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original constructor receipt awaits its recipient"))}
        if constructor.constructor_recipient_retired{return Ok(Default::default())}
        if !constructor.constructor_recipient_admitted{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        if constructor.constructor_lane>=4{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        let parent=0usize;
        let mut demand=match constructor.constructor_lane{0=>constructor.constructor_document.demands(body.saturating_sub(parent),self.store.owned_disposer_installed(),A::document_store_owners_source_demands)?,1=>constructor.constructor_config.demands(body.saturating_sub(parent),self.config_store.owned_disposer_installed(),A::config_store_owners_source_demands)?,2=>constructor.constructor_draft.demands(body.saturating_sub(parent),self.draft_store.owned_disposer_installed(),A::draft_store_owners_source_demands)?,_=>constructor.constructor_interaction.demands(body.saturating_sub(parent),self.interaction_store.owned_disposer_installed(),crate::local_interaction::retirement::interaction_store_owners_source_demands)?};
        demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"constructor close parent quote overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"constructor close parent depth overflow"))?;Ok(demand)
    }

    pub(crate) fn close_store_constructor_unit(&mut self, constructor: &mut ConstructorState<A>,grant:RetainedCloneGrant)->Result<bool,ValueError>{
        let phase=ArtifactStoreConstructorPhase::Close;
        if constructor.has_constructor_receipt(){return Ok(true)}
        if constructor.constructor_recipient_retired{return Ok(false)}
        let installed=match constructor.constructor_lane{0=>self.store.owned_disposer_installed(),1=>self.config_store.owned_disposer_installed(),2=>self.draft_store.owned_disposer_installed(),_=>self.interaction_store.owned_disposer_installed()};
        if constructor.constructor_lane<4&&!installed{return self.drive_store_constructor_unit_in_phase(constructor, grant, ArtifactStoreConstructorPhase::Close)}
        if !constructor.constructor_recipient_admitted{return self.drive_store_constructor_unit_in_phase(constructor, grant, ArtifactStoreConstructorPhase::Close)}
        if constructor.constructor_lane>=4{
            let copied_bytes=0usize;
            if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(true)}
            unsafe{constructor.constructor_receipts.assume_init_drop()}constructor.constructor_recipient_admitted=false;constructor.constructor_recipient_retired=true;constructor.constructor_terminal_receipt=Some((ArtifactStoreConstructorKind::Recipient,phase,grant,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}));return Ok(true)
        }
        let parent=0usize;
        if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth<2{return Ok(true)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
        let (kind,mut receipt,done)=match constructor.constructor_lane{0=>{let receipt=constructor.constructor_document.close(child)?;(ArtifactStoreConstructorKind::Document,receipt,constructor.constructor_document.terminal_is_empty())},1=>{let receipt=constructor.constructor_config.close(child)?;(ArtifactStoreConstructorKind::Config,receipt,constructor.constructor_config.terminal_is_empty())},2=>{let receipt=constructor.constructor_draft.close(child)?;(ArtifactStoreConstructorKind::Draft,receipt,constructor.constructor_draft.terminal_is_empty())},_=>{let receipt=constructor.constructor_interaction.close(child)?;(ArtifactStoreConstructorKind::Interaction,receipt,constructor.constructor_interaction.terminal_is_empty())}};
        if receipt==Default::default(){return Ok(true)}
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog close exceeded its full grant"))}
        if done{constructor.constructor_lane+=1}receipt.copied_bytes+=parent;unsafe{constructor.constructor_receipts.assume_init_mut()[0]=Some((kind,phase,grant,receipt));}Ok(true)
    }

    pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
        semio_framework_job::Generation(self.store.generation_now())
    }

    /// 🫧️ Reads the immutable transient projection of one exact live window.
    pub fn window_transient_snapshot(&mut self, view_state: &ViewModel) -> Result<Option<WindowTransientSnapshot>, Fault> {
        Ok(self.window_transient_store.capture(Some(view_state))?.map(|authority| authority.snapshot))
    }

    pub fn window_transient_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        Ok(self.window_transient_store.capture(Some(view_state))?.map(|authority| authority.generation))
    }

    pub async fn window_config_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        Ok(self.window_config_store.capture(Some(view_state)).await?.map(|authority| authority.generation))
    }

    pub(crate) fn peer_roster_slot(generation: u64) -> usize {
        generation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS
    }

    pub(crate) fn declared_child_reference(&self, slot: &str, child_id: &str, dialect: &ArtifactDialect) -> Result<ArtifactRef, Fault> {
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("child member projection failed: {error}")))?;
        let fields = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.child_id == child_id).map(|(_, fields)| fields).ok_or_else(|| plugin_sdk_fault("child member identity is not declared by the current parent"))?;
        if fields.artifact_kind != dialect.artifact_kind || fields.standard != dialect.standard || fields.subset != dialect.subset {
            return Err(plugin_sdk_fault("child member dialect differs from the declared target"));
        }
        Ok(ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() })
    }

    /// 📌️ Records `pins` on the checkpoint the parent's dispatch just created. Runs AFTER
    /// that dispatch because the checkpoint does not exist until then, and `composition_pins`
    /// participates in `content_addressed_checkpoint_id`, so the pins must be present on the
    /// envelope that gets persisted.
    pub(crate) async fn stamp_checkpoint_composition_pins(&mut self, pins: Vec<vcs::CompositionPin>) -> Result<(), Fault> {
        if pins.is_empty() {
            return Ok(());
        }
        if let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) {
            self.store.set_checkpoint_composition_pins(&checkpoint_id, pins).await.map_err(|error| error.into_fault())?;
        }
        Ok(())
    }
}

impl<A: ArtifactApp> PeerRoster<A> {
    /// 🧭️ The lowest mounted peer roster publication the ordered cursor has already passed. A
    /// publication whose generation is not ahead of `peer_roster_processed_generation` can never be the
    /// rotation's next exact generation again, so without this the maintenance stage answers
    /// `Pending { 0, 0 }` for it forever: it never reaches its outcome slot, `take_presence_outcome`
    /// never answers, and the app's close conjunction — which demands an empty publication registry —
    /// is unreachable. Walking it through the very same validate/outcome ladder lets
    /// `validate_peer_roster_publication` refuse its generation and hand the refusal to its caller.
    pub(crate) fn orphaned_peer_roster_generation(&self) -> Option<u64> {
        let processed = self.peer_roster_processed_generation;
        let mut orphaned: Option<u64> = None;
        self.peer_roster_publications.each_id(|generation| {
            if generation <= processed && orphaned.is_none_or(|lowest| generation < lowest) {
                orphaned = Some(generation);
            }
        });
        orphaned
    }

    pub(crate) fn validate_peer_roster_publication(&self, stores: &InstanceStores<A>, seq: u64, generation: u64, cancel: &semio_framework_job::CancelToken) -> Result<ValidatedPeerRosterCommit, Fault> {
        let slot = InstanceStores::<A>::peer_roster_slot(generation);
        if self.peer_roster_reservations[slot] != Some((generation, seq))
            || generation != self.peer_roster_processed_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-generation"), "peer roster publication generation exhausted"))?
            || !self.peer_presence_retirements.can_insert(generation)
            || !self.presence_peer_retirements.can_insert(generation)
            || !self.peer_roster_outcomes.can_insert(generation)
            || !stores.presence_store.peer_retirement_available()
            || stores.presence_store.retirement_started()
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-publication-authority"), "peer roster lost its exact generation, outcome, or retirement admission before publication"));
        }
        if ::semio_framework_async::poll::resolve_ready(cancel.is_cancelled()) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-cancelled"), "peer roster publication was cancelled before atomic commit"));
        }
        Ok(ValidatedPeerRosterCommit { generation })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    #[expect(clippy::needless_pass_by_value, reason = "This non-Copy token conveys one-use admission authority and must be consumed by the commit, even though its identity fields are scalar.")]
    pub(crate) fn publish_peer_roster_candidate_admitted(&mut self, stores: &mut InstanceStores<A>, admission: ValidatedPeerRosterCommit, candidate: PeerRosterCandidate<A::Presence>) -> Result<(), PeerRosterCandidate<A::Presence>> {
        let PeerRosterCandidate { seq, generation: candidate_generation, cancel, own_color, now_ms, root, typed } = candidate;
        let ValidatedPeerRosterCommit { generation } = admission;
        let _ = now_ms;
        let typed_retirement = match stores.presence_store.publish_peer_commit(typed) {
            Ok(retirement) => retirement,
            Err(typed) => return Err(PeerRosterCandidate { seq, generation: candidate_generation, cancel, own_color, now_ms, root, typed }),
        };
        let previous = std::mem::replace(&mut *self.peer_presence, root);
        if previous.is_empty() {
            drop(previous);
        } else {
            self.peer_presence_retirements.insert_admitted(generation, PeerPresenceRootRetirement::new(previous));
        }
        self.own_color = own_color;
        self.peer_presence_generation = generation;
        if let Some(retirement) = typed_retirement {
            self.presence_peer_retirements.insert_admitted(generation, retirement);
        }
        Ok(())
    }
}

impl<A: ArtifactApp> LocalInteraction<A> {
    /// 📃️ One bounded unit of the live local-interaction query. Its own captured roots are the
    /// only ownership it advances: the Store-side reclamation of the leases it hands back is
    /// [`Self::advance_snapshot_read_returns_one`]'s cooperative-maintenance business, because a
    /// registry's one-slot-per-step cleanup cursor is bounded by the registry's fixed capacity
    /// and would otherwise put a whole capacity's worth of probes inside a host continuation
    /// budget before a read could publish its terminal reply.
    pub(crate) fn advance_local_interaction_query_one(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, output_width: usize) -> Result<(), Fault> {
        if self.local_interaction_query_receipt.is_some(){return Ok(())}
        let Some(query)=self.local_interaction_query.as_mut()else{return Ok(())};
        let result=if query.is_closing(){
            let before=query.retired_bytes();let result=query.close_step(grant.retained_grant());self.local_interaction_retired_bytes+=(query.retired_bytes()-before)as u128;result.map(|step|step.progress())
        }else{
            query.advance(grant,output_width).map(|step|{if let crate::local_interaction::live::LocalInteractionLiveStep::Advanced{emitted_bytes,retired_bytes,..}=step{self.local_interaction_emitted_bytes+=emitted_bytes as u128;self.local_interaction_retired_bytes+=retired_bytes as u128;}step.progress()})
        };
        let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
        if progress!=Default::default(){self.local_interaction_query_receipt=Some((grant.retained_grant(),progress));}
        result.map(|_|()).map_err(|error|plugin_sdk_fault(error.to_string()))
    }
}
