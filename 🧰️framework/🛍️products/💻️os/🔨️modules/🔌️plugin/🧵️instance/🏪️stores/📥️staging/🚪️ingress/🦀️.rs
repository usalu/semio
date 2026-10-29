//! 📂️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use crate::stores_components::{ConstructorState, InstanceStores, LocalInteraction, PeerRoster};
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitPermit;
use std::future::Future;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_peer::{PeerPresenceRootRetirement, PeerRosterCandidate, ValidatedPeerRosterCommit};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> ConstructorState<A> {
    pub fn has_constructor_receipt(&self)->bool { self.constructor_terminal_receipt.is_some() || (self.constructor_recipient_admitted && unsafe { self.constructor_receipts.assume_init_ref() }.iter().any(Option::is_some)) }

    pub fn constructor_receipt(&self)->Option<&(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if let Some(receipt)=self.constructor_terminal_receipt.as_ref(){return Some(receipt)} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_ref()}.iter().find_map(Option::as_ref) }

    pub fn take_constructor_receipt(&mut self)->Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if self.constructor_terminal_receipt.is_some(){return self.constructor_terminal_receipt.take()} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_mut()}.iter_mut().find_map(Option::take) }

    pub fn store_constructor_is_pending(&self)->bool { (!self.constructor_recipient_admitted&&!self.constructor_recipient_retired) || self.has_constructor_receipt() || self.constructor_lane<4 }

    pub fn constructor_terminal_is_empty(&self)->bool{self.constructor_recipient_retired&&!self.has_constructor_receipt()&&self.constructor_document.terminal_is_empty()&&self.constructor_config.terminal_is_empty()&&self.constructor_draft.terminal_is_empty()&&self.constructor_interaction.terminal_is_empty()}
}

impl<A: ArtifactApp> InstanceStores<A> {
    pub async fn validate_framework_reserved_commit(&self, refusal: Option<Fault>, verb: &str, permit: &FrameworkReservedCommitPermit) -> Result<(), Fault> {
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

    /// 📥️ The original instance recipient observes all four native initializer catalogs without recreating any factory.
    pub fn admit_original_constructor_recipient(&self,constructor:&mut ConstructorState<A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if constructor.has_constructor_receipt(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native constructor recipient cannot replace an outstanding original receipt").into())}
        if !constructor.constructor_recipient_admitted{let copy=std::mem::size_of::<[Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)>;24]>()+std::mem::size_of::<bool>();let mut done=None;semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{constructor.constructor_receipts.write([None;24]);constructor.constructor_recipient_admitted=true;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))})?;return Ok(false)}
        if constructor.constructor_lane>=4{return Ok(true)}let extra=std::mem::size_of::<usize>();let g=cx.retained_grant();if cx.should_yield()||g.maximum_copy_bytes<extra+std::mem::size_of::<bool>()||g.maximum_depth==0||g.maximum_items==0{return Ok(false)}let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,..g};let mut p=match constructor.constructor_lane{0=>constructor.constructor_document.acknowledge_installed(&self.store,child)?,1=>constructor.constructor_config.acknowledge_installed(&self.config_store,child)?,2=>constructor.constructor_draft.acknowledge_installed(&self.draft_store,child)?,_=>constructor.constructor_interaction.acknowledge_installed(&self.interaction_store,child)?};if p.copied_items!=0{constructor.constructor_lane+=1;p.copied_bytes+=extra;cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);}Ok(false)
    }

    pub fn drive_store_constructor_unit(&mut self, constructor: &mut ConstructorState<A>,grant:RetainedCloneGrant)->Result<bool,ValueError> {self.drive_store_constructor_unit_in_phase(constructor, grant, ArtifactStoreConstructorPhase::Preparation)}

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
            _=>{let receipt=constructor.constructor_interaction.advance(&mut self.interaction_store,child,|grant|Some(semio_framework_plugin_interaction::retirement::interaction_store_owners(grant)))?;(ArtifactStoreConstructorKind::Interaction,receipt,constructor.constructor_interaction.terminal_is_empty())},
        };
        if receipt==Default::default(){return Ok(true)}
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog parent exceeded its full grant"))}
        if done { constructor.constructor_lane+=1; }
        receipt.copied_bytes=receipt.copied_bytes.checked_add(copied_bytes).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original catalog outer header extent overflow"))?;
        unsafe{constructor.constructor_receipts.assume_init_mut()[0]=Some((kind,phase,grant,receipt));}Ok(true)
    }

    pub fn constructor_close_demands(&self, constructor: &ConstructorState<A>,body:usize)->Result<RetirementDemand,ValueError>{
        if constructor.has_constructor_receipt(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original constructor receipt awaits its recipient"))}
        if constructor.constructor_recipient_retired{return Ok(Default::default())}
        if !constructor.constructor_recipient_admitted{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        if constructor.constructor_lane>=4{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        let parent=0usize;
        let mut demand=match constructor.constructor_lane{0=>constructor.constructor_document.demands(body.saturating_sub(parent),self.store.owned_disposer_installed(),A::document_store_owners_source_demands)?,1=>constructor.constructor_config.demands(body.saturating_sub(parent),self.config_store.owned_disposer_installed(),A::config_store_owners_source_demands)?,2=>constructor.constructor_draft.demands(body.saturating_sub(parent),self.draft_store.owned_disposer_installed(),A::draft_store_owners_source_demands)?,_=>constructor.constructor_interaction.demands(body.saturating_sub(parent),self.interaction_store.owned_disposer_installed(),semio_framework_plugin_interaction::retirement::interaction_store_owners_source_demands)?};
        demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"constructor close parent quote overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"constructor close parent depth overflow"))?;Ok(demand)
    }

    pub fn close_store_constructor_unit(&mut self, constructor: &mut ConstructorState<A>,grant:RetainedCloneGrant)->Result<bool,ValueError>{
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

    pub fn peer_roster_slot(generation: u64) -> usize {
        generation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS
    }

    /// 🧭️ The `(slot, child_id)` the published parent declares for one exact child target, the inverse of [`Self::declared_child_reference`].
    pub fn declared_child_address(&self, target: &ArtifactRef) -> Result<(String, String), Fault> {
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("child member projection failed: {error}")))?;
        (0..projection.len())
            .filter_map(|index| projection.get(index))
            .find(|(_, fields)| fields.artifact_id == target.artifact_id && fields.artifact_kind == target.dialect.artifact_kind && fields.standard == target.dialect.standard && fields.subset == target.dialect.subset)
            .map(|(slot, fields)| (slot.to_owned(), fields.child_id.to_owned()))
            .ok_or_else(|| plugin_sdk_fault("created child is not declared by the published parent"))
    }

    pub fn declared_child_reference(&self, slot: &str, child_id: &str, dialect: &ArtifactDialect) -> Result<ArtifactRef, Fault> {
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("child member projection failed: {error}")))?;
        let fields = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.child_id == child_id).map(|(_, fields)| fields).ok_or_else(|| plugin_sdk_fault("child member identity is not declared by the current parent"))?;
        if fields.artifact_kind != dialect.artifact_kind || fields.standard != dialect.standard || fields.subset != dialect.subset {
            return Err(plugin_sdk_fault("child member dialect differs from the declared target"));
        }
        Ok(ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() })
    }

    /// 📐️ Quotes the next actual pin-source, publication or final frame retirement before mounted cancellation.
    pub fn checkpoint_composition_pins_close_demands(&self,pins:&Option<Vec<vcs::CompositionPin>>,publication:&Option<Box<semio_framework_os_store_engine_replay::repin::OriginalRepinPublication<A::Mutation>>>,retirement:&semio_framework_value::retained_clone::RetainedCloneClose,closing:bool,body:usize)->Result<RetirementDemand,ValueError>{
        if !closing{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()})}if !retirement.is_empty(){return retirement.demands(body)}if let Some(owner)=publication.as_ref(){return if owner.terminal_is_empty(){Ok(semio_framework_os_store_engine_replay::repin::OriginalRepinPublication::<A::Mutation>::boxed_release_demands())}else{owner.close_demands(&self.store,body)}}if pins.is_some(){return retirement.next_owner_demands::<Vec<vcs::CompositionPin>>(true,&None,body)}Ok(Default::default())
    }

    /// 📌️ The original pin vector and current canonical checkpoint advance through the actual native Job until full closure.
    pub fn checkpoint_composition_pins_unit(&mut self,pins:&mut Option<Vec<vcs::CompositionPin>>,publication:&mut Option<Box<semio_framework_os_store_engine_replay::repin::OriginalRepinPublication<A::Mutation>>>,retirement:&mut semio_framework_value::retained_clone::RetainedCloneClose,closing:&mut bool,publish:bool,body:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::close_original_retained_step;
        use semio_framework_value::RetainedCloneStep;
        if pins.is_none()&&publication.is_none()&&retirement.is_empty(){return Ok(true)}
        if !*closing&&(!publish||cx.is_cancelled()||publication.as_ref().is_some_and(|owner|owner.published())){let d=RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()};cx.with_original_retirement(|cx|close_original_retained_step(cx,d,|_|{*closing=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}))?;return Ok(false)}
        if !*closing{if let Some(owner)=publication.as_mut(){if owner.prepared(){owner.publish(&mut self.store,cx)?;}else{owner.advance(&mut self.store,cx)?;}}else{semio_framework_os_store_engine_replay::repin::OriginalRepinPublication::admit_current(&self.store,pins,cx,publication)?;}return Ok(false)}
        cx.with_original_retirement(|cx|{let d=self.checkpoint_composition_pins_close_demands(pins,publication,retirement,*closing,body)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}if !retirement.is_empty(){let selection=retirement.select(body)?;close_original_retained_step(cx,d,|g|selection.step(g))?;}else if let Some(owner)=publication.as_mut(){if !owner.terminal_is_empty(){owner.close_step(&mut self.store,body,cx)?;}else{close_original_retained_step(cx,d,|_|{*publication=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;}}else if pins.is_some(){close_original_retained_step(cx,d,|g|Ok(retirement.begin_granted(pins,g)?.unwrap_or(RetainedCloneStep::Progress(Default::default()))))?;}Ok(pins.is_none()&&publication.is_none()&&retirement.is_empty())})
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

    pub fn validate_peer_roster_publication(&self, stores: &InstanceStores<A>, seq: u64, generation: u64, cancel: &semio_framework_job::CancelToken) -> Result<ValidatedPeerRosterCommit, Fault> {
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
    pub fn publish_peer_roster_candidate_admitted(&mut self, stores: &mut InstanceStores<A>, admission: ValidatedPeerRosterCommit, candidate: PeerRosterCandidate<A::Presence>) -> Result<(), PeerRosterCandidate<A::Presence>> {
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
    pub fn advance_local_interaction_query_one(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, output_width: usize) -> Result<(), Fault> {
        if self.local_interaction_query_receipt.is_some(){return Ok(())}
        let Some(query)=self.local_interaction_query.as_mut()else{return Ok(())};
        let result=if query.is_closing(){
            let before=query.retired_bytes();let result=query.close_step(grant.retained_grant());self.local_interaction_retired_bytes+=(query.retired_bytes()-before)as u128;result.map(|step|step.progress())
        }else{
            query.advance(grant,output_width).map(|step|{if let semio_framework_plugin_interaction::live::LocalInteractionLiveStep::Advanced{emitted_bytes,retired_bytes,..}=step{self.local_interaction_emitted_bytes+=emitted_bytes as u128;self.local_interaction_retired_bytes+=retired_bytes as u128;}step.progress()})
        };
        let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
        if progress!=Default::default(){self.local_interaction_query_receipt=Some((grant.retained_grant(),progress));}
        result.map(|_|()).map_err(|error|plugin_sdk_fault(error.to_string()))
    }
}
