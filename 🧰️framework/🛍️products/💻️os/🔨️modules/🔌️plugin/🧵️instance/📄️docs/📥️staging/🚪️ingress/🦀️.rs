//! 🪙️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep, plugin_demand_grant};
use semio_framework_plugin_document_media::{ActiveArtifactEnvelopeDecode, ActiveArtifactEnvelopeDecodeState, ActiveArtifactEnvelopeIngress, ActiveArtifactStoreReplacement, ActiveArtifactStoreReplacementState, ActiveDocumentArchiveLoad, ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll, ArtifactStoreReplacementAdmissionTarget, ArtifactStoreReplacementRefusal, PendingDocumentArchiveMember};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_TOO_LARGE_CODE, DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, document_load_fault};
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use store::os_store::store_schema::ArtifactEnvelope;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_child::{ChildContentRetirement, OwnedDocumentMemberIngress};
use semio_framework_plugin_decl_emission::Emit;
use crate::docs_components::EnvelopeJobs;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::{InstanceStores, LocalInteraction};
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use semio_framework_plugin_instance_children::children_instance_root::genesis_member_schema;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// ♻️ One bounded unit of the fixed document/config/interaction/transient returned-read rotation: each
    /// pump either drains the disposer it owns or probes its Store for the next returned lease.
    pub fn snapshot_read_returns_demands(&self, stores: &InstanceStores<A>, interaction: &LocalInteraction<A>,body:usize)->Result<RetirementDemand,ValueError>{
        let stage=self.next_snapshot_read_return_stage(stores, interaction).unwrap_or(interaction.snapshot_read_return_stage);
        let parent_copy=size_of::<usize>();let child=match stage{0=>self.document_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||stores.store.returned_snapshot_read_retirement_demand())?,1=>interaction.config_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||stores.config_store.returned_snapshot_read_retirement_demand())?,2=>interaction.interaction_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||stores.interaction_store.returned_snapshot_read_retirement_demand())?,_=>match stores.transient_local_root_retirement_factory.as_deref(){Some(factory)=>stores.transient_store.maintenance_returned_reads_demands_root(factory,body.saturating_sub(parent_copy))?,None=>Default::default()}};ChildContentRetirement::parent_snapshot_demand(child,parent_copy)
    }

    pub fn advance_snapshot_read_returns_one(&mut self, stores: &mut InstanceStores<A>, interaction: &mut LocalInteraction<A>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();let demand=self.snapshot_read_returns_demands(stores, interaction, grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.into_message()))?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        let stage = self.next_snapshot_read_return_stage(stores, interaction).unwrap_or(interaction.snapshot_read_return_stage);
        let next_demand=match stage{0=>stores.store.returned_snapshot_read_retirement_demand(),1=>stores.config_store.returned_snapshot_read_retirement_demand(),2=>stores.interaction_store.returned_snapshot_read_retirement_demand(),_=>Ok(Default::default())}.map_err(|error|plugin_sdk_fault(error.into_message()))?;
        let parent_copy=size_of::<usize>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=match stage{0=>self.document_snapshot_read_returns.drive(next_demand,|original|stores.store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),1=>interaction.config_snapshot_read_returns.drive(next_demand,|original|stores.config_store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),2=>interaction.interaction_snapshot_read_returns.drive(next_demand,|original|stores.interaction_store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),_=>match stores.transient_local_root_retirement_factory.clone(){Some(factory)=>stores.transient_store.maintenance_returned_reads_step_root(factory.as_ref(),child).map_err(|error|plugin_sdk_fault(error.into_message())),None=>Ok(RetainedCloneStep::Progress(empty))}}?;let mut receipt=step.progress();interaction.snapshot_read_return_stage=(stage+1)%4;receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original snapshot rotation preflight");let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original snapshot return rotation").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
    }

    /// 👁️ Borrows whether the original returned-read frontier allows another publication item.
    /// Paid retirement remains on the mounted maintenance turn before another semantic callback.
    ///
    /// 🐛️ Every batch item is prepared against a registry lease on the previous item's post root, and the registry
    /// keeps that root alive after the lease returns until a pump takes it. The maintenance rotation reclaims one
    /// returned read per rotation at one item per step, while a publication folds an item every turn — so a
    /// 408-placement fill finalize (816 ops over a 409-object puzzle 3d document) retained one ~5 MB root per
    /// folded op and trapped the guest at its 512 MiB ceiling (`memory allocation of 1232 bytes failed`, ticket
    /// 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS). Draining here bounds a publication to the staged root plus the
    /// base its current item reads, whatever the batch size.
    ///
    /// A disposer that waits on outside ownership cannot be hurried here; the publication must not stall on it.
    pub fn reclaim_document_snapshot_read_returns(&self, stores: &InstanceStores<A>)->Result<bool,Fault>{
        Ok(self.document_snapshot_read_returns.terminal_is_empty() && stores.store.returned_snapshot_read_count() == 0)
    }

    /// 🪹️ No capture-store disposer or queued document read still awaits acknowledgement.
    pub fn snapshot_read_returns_terminal_is_empty(&self, stores: &InstanceStores<A>, interaction: &LocalInteraction<A>) -> bool {
        self.next_snapshot_read_return_stage(stores, interaction).is_none()
    }

    /// 🧭️ The first rotation stage at or after the cursor whose pump or store still holds a returned read.
    pub fn next_snapshot_read_return_stage(&self, stores: &InstanceStores<A>, interaction: &LocalInteraction<A>) -> Option<u8> {
        (0..4u8).map(|offset| (interaction.snapshot_read_return_stage + offset) % 4).find(|stage| match stage {
            0 => !self.document_snapshot_read_returns.terminal_is_empty() || stores.store.returned_snapshot_read_count() != 0,
            1 => !interaction.config_snapshot_read_returns.terminal_is_empty() || stores.config_store.returned_snapshot_read_count() != 0,
            2 => !interaction.interaction_snapshot_read_returns.terminal_is_empty() || stores.interaction_store.returned_snapshot_read_count() != 0,
            _ => stores.transient_local_root_retirement_factory.is_some() && stores.transient_store.returned_read_pending(),
        })
    }

    /// 🎟️ Pre-admits the exact page and byte extent before a producer constructs any
    /// envelope input owner. Occupied fixed slots and allocation failure leave no input owner.
    pub fn begin_artifact_envelope_ingress(&mut self, stores: &mut InstanceStores<A>, maximum_pages: usize, maximum_bytes: usize) -> Result<ArtifactEnvelopeDecodeOperationHandle, Fault> {
        let generation = stores.artifact_generation_now();
        let Some(slot) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.envelope_ingress.slot_is_vacant(*slot) && self.envelope_decode_jobs.slot_is_vacant(*slot) && self.store_replacement_jobs.slot_is_vacant(*slot)) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-saturated"), "every fixed artifact envelope ingress slot already owns a live operation"));
        };
        let operation = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64);
        let pages = store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages, maximum_bytes })
            .map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-credits"), format!("artifact envelope ingress credits were rejected: {fault:?}")))?;
        self.envelope_ingress.insert_admitted(operation.0, ActiveArtifactEnvelopeIngress::new(pages));
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    /// 📦️ Admits one already-decoded authoritative `.pack` + `.spr` envelope directly
    /// into the app-owned retained initializer. This path preserves the binary document format;
    /// the fresh JSON envelope decoder remains a separate ingress contract.
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact decoded envelope so its domain retirement catalog can close every nested owner.")]
    pub(crate) fn begin_persisted_document_store_replacement(&mut self, stores: &mut InstanceStores<A>, child_composition: &mut ChildComposition<M>, envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationHandle, (Fault, ArtifactEnvelope<A::Snapshot, A::Mutation>)> {
        let generation = stores.artifact_generation_now();
        let expected_dialect: ArtifactDialect = A::DIALECT.into();
        if envelope.schema != A::DOCUMENT_SCHEMA || envelope.dialect.as_ref() != Some(&expected_dialect) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-envelope-identity"), "persisted document envelope does not match the app's exact schema and dialect"), envelope));
        }
        let Some(slot) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.envelope_ingress.slot_is_vacant(*slot) && self.envelope_decode_jobs.slot_is_vacant(*slot) && self.store_replacement_jobs.slot_is_vacant(*slot)) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-ingress-saturated"), "every fixed persisted document replacement slot already owns a live operation"), envelope));
        };
        let operation = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64);
        let job = match A::build_document_store_initialization_job(envelope, operation, generation, stores.store.local_actor_id().clone()) {
            Ok(job) => job,
            Err(envelope) => {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-initializer-refused"), "app refused the persisted document's retained initialization authority"), envelope));
            }
        };
        self.store_replacement_jobs.insert_admitted(operation.0, ActiveArtifactStoreReplacement::new(operation,generation,child_composition.child_content_generation,job,(cx.operation(),cx.generation())));
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    /// 🔎️ Rejects stale, closed, full, or over-byte page ingress before the producer copies its raw owner.
    pub fn preflight_artifact_envelope_ingress_page(&self, stores: &InstanceStores<A>, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize) -> Result<(), Fault> {
        if stores.artifact_generation_now() != handle.generation {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before page preflight"));
        }
        let ingress = self.envelope_ingress.get(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"))?;
        if ingress.closing {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"));
        }
        ingress
            .pages
            .as_ref()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"))?
            .preflight_page_bytes(page_bytes)
            .map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-page"), format!("artifact envelope page preflight was rejected before owner construction: {fault:?}")))
    }

    /// 📨️ Runs the bounded page producer only after the same mutable ingress authority accepts its extent.
    pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(&mut self, stores: &mut InstanceStores<A>, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize, build: Build) -> Result<(), Fault>
    where
        Build: FnOnce() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage,
    {
        self.preflight_artifact_envelope_ingress_page(stores, handle, page_bytes)?;
        let ingress = self.envelope_ingress.get_mut(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle after preflight"))?;
        let pages = ingress.pages.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority after preflight"))?;
        build().admit_preflighted_into(pages);
        Ok(())
    }

    /// 📥️ Moves one exact fixed page into a pre-admitted operation or returns that page
    /// untouched with the fault. No page clone or aggregate command buffer is constructed.
    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub fn admit_artifact_envelope_ingress_page(&mut self, stores: &mut InstanceStores<A>, handle: ArtifactEnvelopeDecodeOperationHandle, page: store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage) -> Result<(), (Fault, store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage)> {
        if stores.artifact_generation_now() != handle.generation {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before page admission"), page));
        }
        let Some(ingress) = self.envelope_ingress.get_mut(handle.operation.0) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"), page));
        };
        if ingress.closing {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"), page));
        }
        let Some(pages) = ingress.pages.as_mut() else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"), page));
        };
        page.admit_into(pages).map_err(|(fault, page)| (Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-page"), format!("artifact envelope page was rejected untouched: {fault:?}")), page))
    }

    /// 🔒️ Seals and transfers the exact page authority into the retained decoder. A
    /// saturated decoder keeps the sealed ingress owner in its original fixed slot for retry.
    pub fn seal_artifact_envelope_ingress(&mut self, stores: &mut InstanceStores<A>, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, Fault> {
        if stores.artifact_generation_now() != handle.generation {
            self.cancel_artifact_envelope_load(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before sealing"));
        }
        if !self.envelope_decode_jobs.can_insert(handle.operation.0) {
            return Ok(false);
        }
        {
            let ingress = self.envelope_ingress.get_mut(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"))?;
            if ingress.closing {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"));
            }
            let pages = ingress.pages.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"))?;
            if !pages.is_sealed() {
                pages.seal().map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-seal"), format!("artifact envelope ingress could not seal: {fault:?}")))?;
            }
        }
        let mut ingress = self.envelope_ingress.remove(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress changed before exact transfer"))?;
        let pages = ingress.take_pages().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact transfer owner"))?;
        match self.submit_artifact_envelope_decode(handle.operation, handle.generation, pages, cx) {
            Ok(submitted) if submitted == handle => {
                drop(ingress);
                Ok(true)
            }
            Ok(_) => unreachable!("exact envelope transfer preserves its operation and generation"),
            Err(pages) => {
                *ingress.pages = Some(pages);
                self.envelope_ingress.insert_admitted(handle.operation.0, ingress);
                Ok(false)
            }
        }
    }

    /// 🔄️ Advances decode-to-initializer handoff without running either job inline. A decode that
    /// has not reached [`ArtifactEnvelopeDecodeOperationPoll::Ready`] reports its OWN poll — the
    /// store replacement it will hand to does not exist yet, and reading that absent job here
    /// turned every in-progress decode into a terminal `Fault` for all eight plugin loops
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn advance_artifact_envelope_load(&mut self, stores: &mut InstanceStores<A>, child_composition: &mut ChildComposition<M>, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationPoll, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if let Some(active) = self.store_replacement_jobs.get(handle.operation.0) {
            if active.operation != handle.operation || active.generation != handle.generation {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-handle"), "store replacement handle is stale").into());
            }
            return Ok(self.poll_artifact_store_replacement(handle));
        }
        if let Some(active) = self.envelope_decode_jobs.get(handle.operation.0) {
            if active.operation != handle.operation || active.generation != handle.generation {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "envelope decode handle is stale").into());
            }
            let decode = active.poll();
            if decode != ArtifactEnvelopeDecodeOperationPoll::Ready {
                return Ok(decode);
            }
            let _=self.advance_ready_envelope_decode_worker_owners(handle, cx.retained_grant(), cx)?;
            if !self.artifact_envelope_decode_worker_owners_are_terminal(handle) {
                return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
            }
            if !self.try_begin_artifact_store_replacement(stores, child_composition, handle, cx)? {
                return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
            }
            return Ok(self.poll_artifact_store_replacement(handle));
        }
        if self.envelope_ingress.get(handle.operation.0).is_some() {
            return Ok(ArtifactEnvelopeDecodeOperationPoll::Pending);
        }
        Ok(ArtifactEnvelopeDecodeOperationPoll::Fault)
    }

    pub fn cancel_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        if let Some(ingress) = self.envelope_ingress.get_mut(handle.operation.0) {
            ingress.closing = true;
            return Ok(());
        }
        if self.envelope_decode_jobs.get(handle.operation.0).is_some() {
            return self.cancel_artifact_envelope_decode(handle);
        }
        if self.store_replacement_jobs.get(handle.operation.0).is_some() {
            return self.cancel_artifact_store_replacement(handle);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.load-stale-handle"), "unknown artifact envelope load handle"))
    }

    /// 📥️ Admits one exact fixed-page envelope owner into app maintenance. A missing
    /// domain catalog or occupied operation slot returns every page untouched.
    pub fn submit_artifact_envelope_decode(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        if !self.envelope_decode_jobs.can_insert(operation.0) {
            return Err(pages);
        }
        let Some(bundle) = A::build_envelope_decode_owner_bundle() else { return Err(pages) };
        let completion = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion::new();
        let fields = bundle.begin_fresh_decoder(operation, generation, std::sync::Arc::clone(&self.envelope_completed_records), std::sync::Arc::clone(&completion));
        self.admit_artifact_envelope_decode_owner(operation, generation, pages, fields, completion, cx)
    }

    /// 📥️ Admits one exact envelope decode against a caller-supplied field owner. The domain
    /// route builds that owner from its declared catalog; the framework's own decode-ladder laws
    /// supply a bare one so the ladder is provable without a domain field catalog.
    pub fn admit_artifact_envelope_decode_owner(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages,
        fields: Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<A::Snapshot, A::Mutation>>,
        completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        if !self.envelope_decode_jobs.can_insert(operation.0) {
            return Err(pages);
        }
        let record = store::os_store::owned_schema_decode::artifact_envelope_decode_record(operation, generation, pages)?;
        let active = match store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority::try_new(record, &self.envelope_field_decoders, fields) {
            Ok(decode) => ActiveArtifactEnvelopeDecode::admitted(operation,generation,decode,completion,(cx.operation(),cx.generation())),
            Err((record, _fault, fields)) => ActiveArtifactEnvelopeDecode::rejected(operation, generation, Box::new(store::os_store::owned_schema_decode::ArtifactEnvelopeUnadmittedDecodeRejected::new(record, fields)), completion),
        };
        self.envelope_decode_jobs.insert_admitted(operation.0, active);
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    pub fn poll_artifact_envelope_decode(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.envelope_decode_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).map_or(ArtifactEnvelopeDecodeOperationPoll::Fault, ActiveArtifactEnvelopeDecode::poll)
    }

    /// 🧹️ Whether the decode worker session/outcome ladder is fully retired for one live handle.
    pub fn artifact_envelope_decode_worker_owners_are_terminal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> bool {
        self.envelope_decode_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).is_none_or(ActiveArtifactEnvelopeDecode::decode_worker_owners_are_terminal)
    }

    /// 🎟️ Advances the original ready worker ladder once under the caller's unchanged grant.
    pub fn advance_ready_envelope_decode_worker_owners(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, grant: RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let Some(active)=self.envelope_decode_jobs.get_mut(handle.operation.0)else{return Ok(PluginLifecycleStep::Complete(Default::default()));};
        if active.operation!=handle.operation||active.generation!=handle.generation{return Err(plugin_sdk_fault("ready envelope worker advancement lost its original handle").into());}
        if active.decode_worker_owners_are_terminal(){return Ok(PluginLifecycleStep::Complete(Default::default()));}
        active.drive(cx,active.generation,&self.envelope_completed_records,grant)
    }

    /// 📤️ Applies the sole non-suspending completed-record publication after exact live
    /// generation validation. Backpressure leaves both the record and active operation retained.
    pub fn try_publish_artifact_envelope_decode(&mut self, stores: &mut InstanceStores<A>, handle: ArtifactEnvelopeDecodeOperationHandle, target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<A::Snapshot, A::Mutation>) -> Result<bool, Fault> {
        if semio_framework_job::Generation(stores.store.generation_now()) != handle.generation {
            self.cancel_artifact_envelope_decode(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-publication"), "decoded envelope generation changed before exact publication"));
        }
        let active = self
            .envelope_decode_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "unknown or stale envelope decode handle"))?;
        if active.state != ActiveArtifactEnvelopeDecodeState::Ready || active.cancel.is_cancelled_now() {
            return Ok(false);
        }
        let ticket = active.completion.ticket().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.output-missing"), "ready envelope decode has no exact completed-record ticket"))?;
        match self.envelope_completed_records.try_publish_to(ticket, target) {
            Ok(false) | Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Contended) => Ok(false),
            Ok(true) => {
                active.state = ActiveArtifactEnvelopeDecodeState::Complete;
                if !active.terminal_is_empty(&self.envelope_completed_records) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-terminal"), "published envelope operation retained a nonterminal owner"));
                }
                let active =
                    self.envelope_decode_jobs.remove(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-owner"), "published envelope operation changed before exact removal"))?;
                drop(active);
                Ok(true)
            }
            Err(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-ticket"), "completed envelope ticket was stale or invalid")),
        }
    }

    pub fn cancel_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .envelope_decode_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "unknown or stale envelope decode handle"))?;
        active.cancel.cancel_now();
        Ok(())
    }

    /// 🏗️ Atomically transfers one decoded envelope into the app-owned retained store
    /// initializer. Capacity or factory rejection leaves the completed record in its exact slot.
    pub fn try_begin_artifact_store_replacement(&mut self, stores: &mut InstanceStores<A>, child_composition: &mut ChildComposition<M>, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if semio_framework_job::Generation(stores.store.generation_now()) != handle.generation {
            self.cancel_artifact_envelope_decode(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-admission"), "store replacement generation changed before initializer admission").into());
        }
        if !self.store_replacement_jobs.can_insert(handle.operation.0) {
            return Ok(false);
        }
        let ticket = self
            .envelope_decode_jobs
            .get(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation && active.state == ActiveArtifactEnvelopeDecodeState::Ready && !active.cancel.is_cancelled_now())
            .and_then(|active| active.completion.ticket())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-output-not-ready"), "decoded envelope is not ready for retained store initialization"))?;
        if !self.artifact_envelope_decode_worker_owners_are_terminal(handle){let _=self.advance_ready_envelope_decode_worker_owners(handle, cx.retained_grant(), cx)?;return Ok(false);}
        let child_content_generation = child_composition.child_content_generation;
        let mut target = ArtifactStoreReplacementAdmissionTarget::<A, M> { source_identity:(cx.operation(),cx.generation()),jobs: &mut self.store_replacement_jobs, operation: handle.operation, generation: handle.generation, child_content_generation, actor: stores.store.local_actor_id().clone(), marker: std::marker::PhantomData };
        match self.envelope_completed_records.try_publish_to(ticket, &mut target) {
            Ok(false) | Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Contended) => Ok(false),
            Ok(true) => {
                if !self.artifact_envelope_decode_worker_owners_are_terminal(handle){return Err(plugin_sdk_fault("initializer transfer changed its already admitted original decode worker witness").into());}
                let active = self
                    .envelope_decode_jobs
                    .get_mut(handle.operation.0)
                    .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-decode-owner"), "decoded envelope owner changed during exact initializer admission"))?;
                if !active.decode_worker_owners_are_terminal() {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-decode-terminal"), "decoded envelope retained an owner after initializer transfer").into());
                }
                active.state = ActiveArtifactEnvelopeDecodeState::Complete;

                Ok(true)
            }
            Err(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-completed-ticket"), "decoded envelope completed-record ticket was stale or invalid").into()),
        }
    }

    /// 🌳️ Pre-admits the exact recursive member count after the retained parent initializer
    /// is terminal and before any member input may enter the candidate transaction.
    pub fn try_begin_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, expected: usize, expires_at_us: u64) -> Result<bool, Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| plugin_sdk_fault("owned document member ingress has no matching replacement authority"))?;
        active.begin_members(expected, expires_at_us)
    }

    /// 📥️ Transfers one admitted member-open request into its exact retained ordinal.
    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact retained member request and identity owner unchanged.")]
    pub fn admit_owned_document_member(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        let Some(active) = self.store_replacement_jobs.get_mut(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation) else {
            return Err((plugin_sdk_fault("owned document member ingress has no matching replacement authority"), ingress));
        };
        active.admit_member(ingress)
    }

    /// 🔒️ Freezes the complete ordinal set so bounded typed opens can begin.
    pub fn seal_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| plugin_sdk_fault("owned document member seal has no matching replacement authority"))?;
        active.seal_members()
    }

    pub fn poll_artifact_store_replacement(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.store_replacement_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).map_or(ArtifactEnvelopeDecodeOperationPoll::Fault, |active| match active.state {
            ActiveArtifactStoreReplacementState::Initializing
            | ActiveArtifactStoreReplacementState::AwaitingMembers
            | ActiveArtifactStoreReplacementState::OpeningMembers
            | ActiveArtifactStoreReplacementState::ValidatingClosure
            | ActiveArtifactStoreReplacementState::PreparingCandidateViews
            | ActiveArtifactStoreReplacementState::CandidateReady => ArtifactEnvelopeDecodeOperationPoll::Pending,
            ActiveArtifactStoreReplacementState::ClosingRejectedMember
            | ActiveArtifactStoreReplacementState::RetiringRejectedMembers
            | ActiveArtifactStoreReplacementState::RetiringCommittedMembers
            | ActiveArtifactStoreReplacementState::RetiringCommittedStore
            | ActiveArtifactStoreReplacementState::RetiringRejectedCandidate => ArtifactEnvelopeDecodeOperationPoll::Progress,
            ActiveArtifactStoreReplacementState::Complete if active.committed && active.refusal.is_none() => ArtifactEnvelopeDecodeOperationPoll::Ready,
            ActiveArtifactStoreReplacementState::Complete if matches!(active.refusal, Some(ArtifactStoreReplacementRefusal::Cancelled)) => ArtifactEnvelopeDecodeOperationPoll::Cancelled,
            ActiveArtifactStoreReplacementState::Complete => ArtifactEnvelopeDecodeOperationPoll::Fault,
        })
    }

    /// 🧭️ The exact leg that refused this replacement, minted as a typed fault. Read BEFORE
    /// {@link acknowledge_artifact_store_replacement} retires the operation — after that the
    /// evidence is gone and only the generic collapse remains.
    pub fn artifact_store_replacement_refusal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Option<Fault> {
        self.store_replacement_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).and_then(|active| active.refusal).map(ArtifactStoreReplacementRefusal::into_fault)
    }

    pub fn acknowledge_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<bool, Fault> {
        let Some(active) = self.store_replacement_jobs.get(handle.operation.0) else { return Ok(false) };
        if active.operation != handle.operation || active.generation != handle.generation || !active.terminal_is_empty() {
            return Ok(false);
        }
        let active = self
            .store_replacement_jobs
            .remove(handle.operation.0)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-terminal-owner"), "terminal store replacement changed before exact acknowledgement"))?;
        drop(active);
        Ok(true)
    }

    pub fn cancel_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-handle"), "unknown or stale store replacement handle"))?;
        if !active.committed {
            active.request_cancel();
        }
        Ok(())
    }

    /// 🌱️ Completes an archive's member roster with every child the hydrated candidate parent declares,
    /// the archive omits, and the app derives (`ArtifactApp::genesis_child_pack`) — the content-addressed
    /// members a whole-document load (`Effect::LoadDocument`, which carries pack+spr only) can never
    /// ship. Each minted entry is owner-stamped to the candidate root and takes the next ordinal after the
    /// archived ones; the fixed member and byte authorities are re-checked over the completed roster. A
    /// slot the app cannot derive is left alone, so closure validation still rejects it as `Incomplete`.
    ///
    /// 🔢️ Archived entries are already reversed for ordinal-ordered popping; ordinals stay
    /// `0..n` for them and continue at `n` for every minted member, whatever the pop order.
    pub(crate) fn complete_document_archive_genesis(&mut self, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive member roster owner is absent"))?;
        let candidate = self
            .store_replacement_jobs
            .get(handle.operation.0)
            .filter(|replacement| replacement.operation == handle.operation && replacement.generation == handle.generation)
            .and_then(ActiveArtifactStoreReplacement::candidate_awaiting_members)
            .ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis has no hydrated candidate parent awaiting members"))?;
        let dialect = candidate.envelope().dialect.clone().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis requires the candidate parent's exact dialect"))?;
        let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
        let snapshot = candidate.snapshot_ref();
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive genesis child projection failed: {error}")))?;
        let mut ordinal = archive.members.len();
        let mut payload_bytes = archive.members.iter().fold(archive.parent_spr.len(), |total, entry| total + entry.envelope_pack.len());
        for index in 0..projection.len() {
            let Some((slot, fields)) = projection.get(index) else { break };
            if archive.members.iter().any(|entry| entry.owner.slot == slot && entry.owner.child_id == fields.child_id) {
                continue;
            }
            let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.to_string(), child_id: fields.child_id.to_string() };
            let expected = ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() };
            let envelope_pack = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack))
                .map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{} envelope: {error}", fields.child_id)))?;
            if ordinal >= protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive genesis exceeds its fixed 1024-member authority").with_param("count", (ordinal + 1).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));
            }
            payload_bytes = payload_bytes.checked_add(envelope_pack.len()).filter(|bytes| *bytes <= protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive genesis exceeds its fixed byte authority").with_param("bytes", payload_bytes.saturating_add(envelope_pack.len()).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()))?;
            archive.members.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: u32::try_from(ordinal).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis ordinal exceeds u32"))?,
                reference: protocol::DocumentArchiveArtifactRef { artifact_id: fields.artifact_id.to_string(), artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef { artifact_id: parent.artifact_id.clone(), artifact_kind: parent.dialect.artifact_kind.clone(), standard: parent.dialect.standard.clone(), subset: parent.dialect.subset.clone() },
                    slot: slot.to_string(),
                    child_id: fields.child_id.to_string(),
                },
                envelope_pack,
            });
            ordinal += 1;
            active.total = active.total.saturating_add(1);
        }
        Ok(())
    }

    /// 🌱️ Mints every derivable composed child onto a live envelope replacement's member roster —
    /// the JSON ingress path ships only the parent envelope, so `ArtifactApp::genesis_child_pack`
    /// must supply the same members `seed_genesis_children` opened at boot.
    pub fn complete_store_replacement_genesis(&mut self, stores: &mut InstanceStores<A>, operation_id: u64) -> Result<(), Fault> {
        let active = self.store_replacement_jobs.get(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis lost its exact replacement authority"))?;
        let handle = ArtifactEnvelopeDecodeOperationHandle { operation: active.operation, generation: active.generation };
        let candidate = active.retained_store.as_ref().ok_or_else(|| plugin_sdk_fault("live store replacement genesis has no hydrated candidate parent"))?;
        let dialect = candidate.envelope().dialect.clone().ok_or_else(|| plugin_sdk_fault("live store replacement genesis requires the candidate parent's exact dialect"))?;
        let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
        let snapshot = candidate.snapshot_ref();
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("live store replacement genesis child projection failed: {error}")))?;
        let mut entries = Vec::new();
        let mut ordinal = 0usize;
        for index in 0..projection.len() {
            let Some((slot, fields)) = projection.get(index) else { break };
            let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.to_string(), child_id: fields.child_id.to_string() };
            let expected = ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() };
            let envelope_pack = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack))
                .map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{} envelope: {error}", fields.child_id)))?;
            if ordinal >= protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(plugin_sdk_fault("live store replacement genesis exceeds its fixed 1024-member authority"));
            }
            entries.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: u32::try_from(ordinal).map_err(|_| plugin_sdk_fault("live store replacement genesis ordinal exceeds u32"))?,
                reference: protocol::DocumentArchiveArtifactRef { artifact_id: fields.artifact_id.to_string(), artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef { artifact_id: parent.artifact_id.clone(), artifact_kind: parent.dialect.artifact_kind.clone(), standard: parent.dialect.standard.clone(), subset: parent.dialect.subset.clone() },
                    slot: slot.to_string(),
                    child_id: fields.child_id.to_string(),
                },
                envelope_pack,
            });
            ordinal += 1;
        }
        let active = self.store_replacement_jobs.get_mut(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis changed before member admission"))?;
        active.begin_members(entries.len(), u64::MAX)?;
        for entry in entries {
            let mut pending = PendingDocumentArchiveMember::new(entry).map_err(|(fault, _)| fault)?;
            while !pending.fill_one_page(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES)? {}
            while !matches!(pending.retire_source_step(RetainedCloneGrant { maximum_copy_bytes: store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES, ..plugin_demand_grant(pending.source_retirement_demands()) }), RetainedCloneStep::Complete(_)) {}
            let ingress = pending.take_ingress(handle, stores.store.local_actor_id().clone()).map_err(|(fault, _)| fault)?;
            active.admit_member(ingress).map_err(|(fault, _)| fault)?;
        }
        active.seal_members()?;
        Ok(())
    }
}
