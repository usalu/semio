//! 🎾️ Staged `producers` members of the `instance-dispatch` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🎟️ The reserved emit producers: lease admission, owner mounting and the producer admission fixtures.
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use std::mem::ManuallyDrop;
use store::MemberFactory;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use store::SpaceMember;
use semio_framework_value::ValueError;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework::FaultFrom;
use semio_framework_plugin_decl_emission::ActionMeta;
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactBoundedToolFault, ArtifactReservedToolJob, ArtifactToolCompletion, ArtifactToolCompletionValue};
use semio_framework_plugin_operation::{ArtifactDocumentAuthority, ToolCancellationLease, ToolOperationKey};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::dispatch_bundle::DispatchCtx;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_plugin_job::reserved_jobs::{FrameworkReservedCommitPermit, QualifiedToolProof};
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework::kernel::{InvocationResult, UiDirtyScope};
use semio_framework_plugin_retained_command::typed_operation::{MountedTypedCommandFullOperation, PendingPublicationOutcome};
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use store::Mutation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_plugin_interaction::interaction_topology::take_inline_interaction_verbs;
use semio_framework_ui_contract::*;
use semio_framework_plugin_instance_jobs::jobs_lease::OriginalToolLeaseAdmission;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_os_kernel as store;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    pub async fn admit_framework_reserved_spawn(&mut self, verb: &str, raw: &[u8], decoded_items: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<FrameworkReservedCommitPermit>, semio_framework_job::OriginalTurnFailure> {
        let proof = self.tools.qualified_tool_proof(&*self.stores, verb)?;
        if !matches!(proof, QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::AppOwned(_)) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory")).into());
        }
        let key = proof.key();
        let admission = self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission")).into());
        }
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,false,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    pub(super) async fn commit_framework_clipboard_completion(&mut self,action:&str,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(action,None,None,completion,meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        if let Some(fault)=permit.producer_fault{let mounted=self.tools.tool_operations.get_mut(id).expect("original completion owns its admitted slot");mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original completion lease").cancel();}
        self.tools.finish_mounted_reserved_completion(id);
        Ok(started)
    }

    /// 📥️ Keeps the original producer and completion in their fixed slot through every fallible worker boundary.
    pub async fn dispatch_reserved_original_producer(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:ArtifactReservedToolJob,completion:ArtifactToolCompletion<A>,meta:&ActionMeta,admission:Option<semio_framework::ToolWireAdmission>,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(verb,None,Some(job),Some(completion),meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        self.tools.tool_operations.get_mut(id).expect("original producer owns its admitted slot").producer_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));
        let validation=(||->Result<(),Fault>{
            let proof=self.tools.qualified_tool_proof(&*self.stores,verb)?;
            let key=proof.key();
            let admitted=match admission{Some(admitted)=>admitted,None=>self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(),key.tool_id.clone(),proof.schema_id(),raw).map_err(|error|plugin_sdk_fault(error.to_string()))?};
            if decoded_items>proof.contract().max_decoded_items||!proof.admits::<A>(&admitted){return Err(plugin_sdk_fault("original reserved producer failed its exact owner/factory/schema admission"));}
            Ok(())
        })();
        let mounted=self.tools.tool_operations.get_mut(id).expect("original producer owns its admitted slot");
        if let Err(fault)=validation{mounted.user_cancel_requested=true;mounted.terminal_seen=true;mounted.stage=MountedTypedCommandFullOperationStage::Publishing;mounted.cancellation_lease.as_ref().expect("original reserved source cancellation lease").cancel();return Err(fault)}
        mounted.terminal_seen=false;
        mounted.stage=MountedTypedCommandFullOperationStage::Worker;
        Ok(started)
    }

    pub(crate) fn admit_original_tool_lease(&mut self,verb:&str,meta:&ActionMeta,keyed:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(semio_framework_job::Operation,ToolCancellationLease)>,semio_framework_job::OriginalTurnFailure>{
        if self.tools.pending_tool_lease_admission.is_none(){
            let bytes=size_of::<OriginalToolLeaseAdmission>();let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
            *self.tools.pending_tool_lease_admission=Some(OriginalToolLeaseAdmission{source:ManuallyDrop::new(None),verb:ManuallyDrop::new(Some(String::new())),operation:None,lease:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),instance:meta.instance_id,identity:(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?),keyed,copied:false,returning:false});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        let pending=self.tools.pending_tool_lease_admission.as_mut().unwrap();
        if pending.identity!=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?)||pending.instance!=meta.instance_id||pending.keyed!=keyed{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool admission requires its retained receiving lane").into())}
        if !pending.returning{
            if pending.source.is_none(){let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(None)};*pending.source=Some(source);return Ok(None)}
            pending.source.as_ref().unwrap().validate_context(cx)?;
        }
        if pending.copied&&pending.verb.as_ref().is_some_and(|original|original!=verb){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool admission requires its retained verb").into())}
        if !pending.copied{
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<verb.len()||grant.maximum_capacity_bytes<verb.len()||grant.maximum_depth==0{return Ok(None)}
            let target=pending.verb.as_mut().unwrap();target.try_reserve_exact(verb.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original tool verb reservation refused"))?;
            target.push_str(verb);pending.copied=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:verb.len(),retained_capacity_bytes:target.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        if pending.operation.is_none(){
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<semio_framework_job::Operation>()||grant.maximum_depth==0{return Ok(None)}
            let Some(operation_id)=self.media.admit_typed_operation_slot(&*self.pending, &*self.tools)else{return Ok(None)};
            let revision=self.stores.store.content_revision_now();let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));let generation=semio_framework_job::Generation(self.stores.store.generation_now());
            let operation=semio_framework_job::Operation::new(operation_id,base_revision,generation,operation_id.0);
            self.tools.typed_operation_reservations[operation_id.0 as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]=Some(operation_id.0);self.tools.pending_tool_lease_admission.as_mut().unwrap().operation=Some(operation);
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<semio_framework_job::Operation>(),..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        let pending=self.tools.pending_tool_lease_admission.as_mut().unwrap();
        if pending.lease.is_none(){
            let operation=pending.operation.unwrap();let key=ToolOperationKey{app_instance_id:pending.instance,document:ArtifactDocumentAuthority(pending.instance),operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation};
            let lease=if keyed{self.tools.tool_cancellations.begin_keyed(&key,cx)?}else{self.tools.tool_cancellations.begin(&key,cx)?};
            *pending.lease=lease;return Ok(None)
        }
        if let Some(original)=pending.verb.as_ref(){
            let bytes=original.capacity();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
            drop(pending.verb.take());cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        if let Some(source)=pending.source.as_mut(){pending.returning=true;source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){pending.source.take();}return Ok(None)}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<OriginalToolLeaseAdmission>()||grant.maximum_depth==0{return Ok(None)}
        let mut pending=self.tools.pending_tool_lease_admission.take().unwrap();let result=(pending.operation.take().unwrap(),pending.lease.take().unwrap());cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<OriginalToolLeaseAdmission>(),..Default::default()})?;cx.consume_fuel(1);Ok(Some(result))
    }

    /// 🎟️ Seals one genuine publication slot and keyed lease before a reserved producer receives source owners.
    pub fn admit_reserved_emit_producer(&mut self,verb:&str,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<FrameworkReservedCommitPermit>,semio_framework_job::OriginalTurnFailure>{
        let proof=self.tools.qualified_tool_proof(&*self.stores, verb)?;
        if !matches!(proof,QualifiedToolProof::FrameworkOwned(_)|QualifiedToolProof::AppOwned(_)){return Err(plugin_sdk_fault("reserved producer lacks its exact registered publication proof").into())}
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,true,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🔬️ Exercises real reserved slot admission without exporting private producer authority.
    #[cfg(test)]
    pub fn test_reserved_emit_admission_case(&mut self, occupied: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> bool {
        let before = self.tools.typed_operation_reservations;
        for slot in 0..occupied { self.tools.typed_operation_reservations[slot] = Some((slot + 128) as u64); }
        let original = self.tools.typed_operation_reservations;
        let mut result = Ok(None);for _ in 0..128{result= self.admit_reserved_emit_producer("cut", meta,cx) ;if !matches!(result,Ok(None)){break}}
        let admitted = match result {
            Ok(Some(permit)) => {
                let id = permit.operation.operation.0;
                let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
                assert_eq!(slot, occupied);
                assert_eq!(self.tools.typed_operation_reservations[slot], Some(id));
                assert!(permit.lease.keyed);
                assert_eq!(permit.lease.key.operation_id, permit.operation.operation);
                assert_eq!(&self.tools.typed_operation_reservations[..occupied], &original[..occupied]);
                self.tools.release_reserved_emit_slot(id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64);
                assert_eq!(self.tools.typed_operation_reservations[slot], Some(id));
                self.tools.finish_reserved_emit_producer(permit);
                assert_eq!(self.tools.typed_operation_reservations, original);
                true
            }

            Ok(None)|Err(_) => { assert_eq!(self.tools.typed_operation_reservations, original); false }
        };
        self.tools.typed_operation_reservations = before;
        admitted
    }

    /// 🧪️ Keeps cancelled worker completion tied to the actual original permit through handoff.
    #[cfg(test)]
    pub async fn test_reserved_emit_cancelled_worker_handoff(&mut self,meta:&ActionMeta)->bool{
        fn now()->Option<u64>{Some(1)}
        let grant=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation;
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(899),semio_framework_job::Generation(3),grant,now,u64::MAX,1_000_000);
        let mut permit=None;
        for _ in 0..128{caller.turn(grant,|cx|{permit=self.admit_reserved_emit_producer("cut",meta,cx)?;Ok(())}).unwrap();if permit.is_some(){break}}
        let permit=permit.expect("original cut producer admission");let id=permit.operation.operation;permit.lease.cancel();
        let retained=permit.operation.operation==id&&self.tools.typed_operation_reservations[id.0 as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]==Some(id.0);
        self.tools.finish_reserved_emit_producer(permit);caller.retire(grant);retained
    }

    /// 🧷️ Preserves the mounted original reserved source across refused worker birth.
    #[cfg(test)]
    pub(crate) async fn test_reserved_original_before_birth(&mut self,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let mut permit=None;for _ in 0..128{permit=self.admit_reserved_emit_producer("cut",meta,cx)?;if permit.is_some(){break}}
        let permit=permit.expect("original fixture reserved permit");let operation=permit.operation;let id=operation.operation.0;
        let job=ArtifactReservedToolJob::new(FrameworkCutJob::new(Vec::new(),0));let original=job.test_source_identity();let completion=ArtifactToolCompletion::<A>::new();
        self.dispatch_reserved_original_producer("cut",&[],0,job,completion,meta,None,permit,cx).await?;
        let mounted=self.tools.tool_operations.get_mut(id).expect("actual original reserved producer mounted");assert!(mounted.reserved_producer.is_some()&&!mounted.terminal_seen);assert!(mounted.session.is_none()&&mounted.session_rejected.is_none());let before=cx.retained_progress();
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(Default::default(),|same|mounted.drive_worker_step(same)));result?;assert_eq!(cx.retained_progress(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(mounted.session_rejected.is_none());assert_eq!(mounted.reserved_producer.as_ref().unwrap().test_source_identity(),original);
        mounted.drive_worker_step(cx)?;let retained=mounted.session_rejected.is_some()&&mounted.session.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Worker&&mounted.reserved_producer.as_ref().unwrap().test_source_identity()==original&&mounted.reserved_producer.as_ref().unwrap().test_bound_operation()==Some(operation);
        mounted.user_cancel_requested=true;mounted.terminal_seen=true;mounted.cancellation_lease.as_ref().unwrap().cancel();Ok(retained)
    }

    /// 🧫️ Observes busy original completion retention without exposing producer authority.
    #[cfg(test)]
    pub async fn test_reserved_emit_busy_completion_owner(&mut self, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> bool {
        let  mut permit=None;for _ in 0..128{permit=self.admit_reserved_emit_producer("cut",meta,cx).expect("fixture cut admission");if permit.is_some(){break}}let permit=permit.expect("fixture original cut permit");
        let id=permit.operation.operation.0;
        let completion=ArtifactToolCompletion::<A>::new();
        let mut emit=Emit::default();emit.artifact_mutations=Vec::with_capacity(3);
        completion.complete(Ok(emit),EphemeralEmit::default()).expect("fixture original completion");
        let original=completion.inner.clone();
        let guard=original.lock().expect("fixture original lock");
        let result=self.commit_framework_clipboard_completion("cut",Some(completion),meta,permit, cx).await;
        let retained=result.is_ok()&&self.tools.tool_operations.get(id).is_some_and(|owner|owner.completion.as_ref().is_some_and(|cell|cell.inner.same_cell(&original))&&owner.terminal_fault.is_some()&&owner.cancellation_lease.as_ref().is_some_and(|lease|lease.token.is_cancelled_now()));
        drop(guard);
        let mut original=semio_framework_value::retirement::controlled::ControlledRetirement::new(ArtifactToolCompletion::<A>{inner:original}).unwrap_or_else(|(_,original)|{let _original=std::mem::ManuallyDrop::new(original);panic!("original completion observation alias retirement was refused");});
        for _ in 0..100000 {
            if original.terminal_is_empty(){break;}
            let copy=original.next_copy_byte_demand().expect("original completion alias copy quote").max(4096);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:original.next_capacity_byte_demand(copy).expect("original completion alias capacity quote"),maximum_release_bytes:original.next_release_byte_demand().expect("original completion alias release quote"),maximum_depth:original.next_depth_demand().expect("original completion alias depth quote")};
            assert!(original.step(grant).expect("funded original completion alias closure").progress().fits(grant));
        }
        assert!(original.terminal_is_empty(),"original completion observation alias remains in custody");
        if !retained {self.tools.release_reserved_emit_slot(id);}
        retained
    }

    pub(crate) async fn mount_original_emit_publication(&mut self,verb:&str,emit:Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>,meta:&ActionMeta,authority:Option<(semio_framework_job::Operation,ToolCancellationLease,semio_framework_value::native_encoding::NativeEncodeContinuation)>, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        self.mount_original_reserved_owners(verb,Some(ArtifactToolCompletionValue::Emit(Ok(emit),EphemeralEmit::default())),None,None,meta,authority, cx).await
    }

    /// 🎒️ Installs original reserved producer and completion owners before fallible publication capture.
    pub(crate) async fn mount_original_reserved_owners(&mut self,verb:&str,publication:Option<ArtifactToolCompletionValue<A>>,reserved_producer:Option<ArtifactReservedToolJob>,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,authority:Option<(semio_framework_job::Operation,ToolCancellationLease,semio_framework_value::native_encoding::NativeEncodeContinuation)>, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let revision=self.stores.store.content_revision_now();
        let generation=semio_framework_job::Generation(self.stores.store.generation_now());
        let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));
        let(operation,lease,identity)=match authority{
            Some((operation,lease,identity))=>(operation,Some(lease),Some(identity)),
            None=>{let operation_id=self.media.admit_typed_operation_slot(&*self.pending, &*self.tools).ok_or_else(||plugin_sdk_fault("every fixed publication slot already retains an original operation"))?;let operation=semio_framework_job::Operation::new(operation_id,base_revision,generation,operation_id.0);(operation,None,None)}
        };
        let operation_id=operation.operation.0;
        let reservation=self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
        if !self.tools.tool_operations.can_insert(operation_id)||reservation.is_some_and(|owner|owner!=operation_id){return Err(plugin_sdk_fault("original emission lost its exact pre-admitted publication slot"));}
        let mounted=MountedTypedCommandFullOperation::<A>{
            verb:verb.into(),meta:meta.clone(),operation,source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),canonical_revision:revision,artifact_generation:generation.0,
            config_generation:self.stores.config_store.generation_now(),draft_generation:self.stores.draft_store.generation_now(),presence_generation:0,transient_generation:0,
            window_config_authority:None,window_transient_authority:None,publication_lanes:&[],contract:None,
            session:None,session_rejected:None,producer_source:None,producer_request:None,producer_request_retirement:None,reserved_producer,completion,completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input:None,output_chunks:None,cancellation_lease:lease,identity,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},terminal_seen:true,worker_semantic_pending:false,worker_outcome_pending:false,worker_fault_capture:None,pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,original_retirement_receipt:None,
            publication,publication_ownership_progress:None,actor_capture:None,worker_resume_pending:false,pending_artifact_publication:None,pending_child_publication:None,
            owned_child_group:None,owned_child_committed:false,owned_child_result_pending:false,
            captured_child_content:Some(std::sync::Arc::new(ChildContentView::clone(&*self.child_composition.child_content_root))),captured_child_content_generation:self.child_composition.child_content_generation,
            result_page:None, diagnostics:None,result_page_presented:false,result_sequence:0,publication_progress:0,publication_checkpoint:None,publication_attempt:0,
            ui_pending:true,progress:None,progress_pending:false,user_cancel_requested:false,published_artifact:false,published_config:false,published_window_config:false,
            command_logged:false,interaction_revalidated:false,retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault:None,stage:MountedTypedCommandFullOperationStage::Publishing,
        };
        self.tools.tool_operations.insert_admitted(operation_id,mounted);
        self.tools.release_reserved_emit_slot(operation_id);
        let capture:Result<_,Fault>=async{
            if self.tools.live_runtime_instance_id!=Some(meta.instance_id){return Err(plugin_sdk_fault("original emission does not belong to its mounted live app instance"));}
            let proof=self.tools.qualified_tool_proof(&*self.stores, verb)?;
            let window_config=self.stores.window_config_store.capture(meta.view_state.as_ref()).await?;
            let window_transient=self.stores.window_transient_store.capture(meta.view_state.as_ref())?;
            Ok((proof.publication_lanes(),proof.contract().max_output_bytes,window_config,window_transient,self.stores.presence_store.generation().await,self.stores.transient_store.generation().await))
        }.await;
        let mounted=self.tools.tool_operations.get_mut(operation_id).expect("original completed output remains in its exact admitted slot");
        match capture{
            Ok((lanes,maximum_identity_bytes,window_config,window_transient,presence,transient))=>{
                if let Some(lease)=mounted.cancellation_lease.as_ref().filter(|_|mounted.identity.is_none()){let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum_identity_bytes,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);}
                mounted.publication_lanes=lanes;mounted.window_config_authority=window_config;mounted.window_transient_authority=window_transient;
                mounted.presence_generation=presence;mounted.transient_generation=transient;
                if let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut(){let inline=take_inline_interaction_verbs(&mut emit.effects);if !inline.is_empty(){self.tools.typed_inline_interaction_verbs.push((operation_id,inline));}}
            }
            Err(fault)=>{mounted.terminal_fault.get_or_insert_with(||ArtifactBoundedToolFault::from_fault(&fault));if let Some(lease)=mounted.cancellation_lease.as_ref(){lease.cancel();}}
        }
        let mut result=ToolOperations::<A>::empty_result(verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        result.output=DslValue::Object(vec![("operationId".into(),DslValue::String(operation_id.to_string())),("generation".into(),DslValue::String(operation.generation.0.to_string()))]);
        Ok(result)
    }
}
