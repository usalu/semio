//! 🎾️ Staged `producers` members of the `instance-dispatch` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🎟️ The reserved emit producers: lease admission, owner mounting and the producer admission fixtures.
use semio_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_plugin_decl_app::ArtifactApp;
use semio_framework_value::DslValue;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use std::mem::{ManuallyDrop,size_of};
use store::MemberFactory;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use store::ReferenceSpaceMember;
use semio_framework_value::ValueError;
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework::FaultFrom;
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_decl_app::tool_contract::{ArtifactReservedToolJob, ArtifactToolCompletion};
use semio_plugin_operation::{ArtifactDocumentAuthority, ToolCancellationLease, ToolOperationKey};
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::dispatch_bundle::DispatchCtx;
use semio_plugin_job::reserved_jobs::{FrameworkReservedCommitPermit, QualifiedToolView};
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use semio_framework::kernel::{InvocationResult, UiDirtyScope};
use semio_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
use store::Mutation;
use protocol::OpText;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_ui_contract::*;
use semio_plugin_instance_jobs::jobs_lease::OriginalToolLeaseAdmission;
use semio_plugin_decl_views::child_view::ChildContentRetainers;
use semio_os_kernel as store;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// 🎞️ Prepares the original media request, then mounts its producer through the stepped reserved mount until the receipt settles.
    pub async fn advance_original_reserved_media(&mut self,port:&str,media:&mut Option<semio_framework::Media>,meta:&ActionMeta,finished:&mut Option<semio_framework_job::Operation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if finished.is_some(){return Ok(true)}
        if self.original_reserved_prepares()&&!self.as_render().advance_reserved_media(port,media,meta,cx)?{return Ok(false)}
        self.advance_original_reserved_mount("import-media",meta,finished,cx).await
    }
    /// 🧾️ Prepares the original clipboard action request, then mounts its producer through the stepped reserved mount until the receipt settles.
    pub async fn advance_original_reserved_action(&mut self,action:&str,args:Option<&DslValue>,decoded_items:usize,meta:&ActionMeta,maximum:usize,finished:&mut Option<semio_framework_job::Operation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if finished.is_some(){return Ok(true)}
        if self.original_reserved_prepares()&&!self.as_render().advance_reserved_action(action,args,decoded_items,meta,maximum,cx)?{return Ok(false)}
        self.advance_original_reserved_mount(action,meta,finished,cx).await
    }
    /// 🪞️ The same started operation answers every settled reserved producer with its exact operation coordinates.
    pub async fn reserved_started_result(&self,verb:&str,meta:&ActionMeta,operation:semio_framework_job::Operation)->InvocationResult{
        let mut result=ToolOperations::<A>::empty_result(verb,meta,Vec::new(),Vec::new(),UiDirtyScope::None).await;
        result.output=DslValue::Object(vec![("operationId".into(),DslValue::String(operation.operation.0.to_string())),("generation".into(),DslValue::String(operation.generation.0.to_string()))]);
        result
    }
    fn original_reserved_prepares(&self)->bool{self.media.reserved_mount.is_none()&&self.media.reserved_receipt.is_none()}
    /// 🎞️ Retains the issued producer through mounted publication and paid source return.
    async fn advance_original_reserved_mount(&mut self,verb:&str,meta:&ActionMeta,finished:&mut Option<semio_framework_job::Operation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_plugin_retained_command::typed_operation::original_emission::{OriginalEmissionAdmission,OriginalEmissionPublicationFrontier};
        use semio_plugin_job::original_emission::OriginalEmissionReceipt;
        use semio_framework_job::{close_original_retained_turn,close_original_retained_step};
        use semio_framework_value::{RetirementDemand,RetainedCloneStep,RetainedWorkOutcome};
        if self.media.reserved_mount.is_none(){
            if let Some(receipt)=self.media.reserved_receipt{let operation=receipt.operation(cx)?;if self.pending.reserved_preparation.is_some()||self.pending.reserved_birth.is_some(){cx.with_original_retirement(|cx|semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::close_slot(&mut self.pending.reserved_preparation,&mut self.pending.reserved_birth,&*self.tools.tool_jobs,cx))?;return Ok(false)}let bytes=size_of::<Option<OriginalEmissionReceipt>>()+size_of::<bool>();let d=RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};cx.with_original_retirement(|cx|close_original_retained_turn(cx,d,|_|{self.media.reserved_receipt=None;*finished=Some(operation);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1}}))?;return Ok(finished.is_some())}
            let source=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
            if let Some((_,original))=self.media.pending_reserved_producer_permit.as_ref(){if *original!=source{return Ok(false)}}else{let Some(permit)=self.admit_reserved_emit_producer(verb,meta,cx)?else{return Ok(false)};*self.media.pending_reserved_producer_permit=Some((permit,source));return Ok(false)}            OriginalEmissionAdmission::<A>::admit(cx,&mut self.media.reserved_mount_construction,&mut self.media.reserved_mount)?;return Ok(false)
        }
        if !self.media.reserved_mount.as_ref().unwrap().matches(cx){return Ok(false)}
        let phase=self.media.reserved_mount.as_ref().unwrap().phase;
        match phase{
            0=>{self.media.reserved_mount.as_mut().unwrap().advance_phase(0,3,cx)?;},
            3=>{if self.media.reserved_mount.as_ref().unwrap().mounted.is_some(){self.media.reserved_mount.as_mut().unwrap().advance_phase(3,4,cx)?;return Ok(false)}let proof=self.tools.qualified_tool_proof(&*self.stores,verb)?;let frontier=OriginalEmissionPublicationFrontier{canonical_revision:self.stores.store.content_revision_now(),artifact_generation:self.stores.store.generation_now(),config_generation:self.stores.config_store.generation_now(),draft_generation:self.stores.draft_store.generation_now(),presence_generation:self.stores.presence_store.generation().await,transient_generation:self.stores.transient_store.generation().await,publication_lanes:proof.publication_lanes()};let operation=self.media.pending_reserved_producer_permit.as_ref().ok_or_else(||ValueError::invariant("reserved mount lost its producer permit"))?.0.operation;let prepared=self.pending.reserved_preparation.as_mut().ok_or_else(||ValueError::invariant("reserved mount lost its preparation"))?;let verb=&mut prepared.headers.as_mut().ok_or_else(||ValueError::invariant("reserved mount lost its original verb"))?[8];let owner=self.media.reserved_mount.as_mut().unwrap();if MountedTypedCommandFullOperation::<A>::admit_original_reserved(verb,meta,frontier,&mut self.media.pending_reserved_producer_permit,&mut prepared.job,&mut prepared.completion,&self.tools.tool_cancellations,cx,&mut owner.construction,&mut owner.mounted)?{owner.operation=Some(operation)}},            4=>{let owner=self.media.reserved_mount.as_mut().unwrap();if owner.window_config.as_ref().is_some_and(|capture|capture.handoff_complete()){owner.advance_phase(4,5,cx)?;}else{owner.capture_window_config(&mut self.stores.window_config_store,cx)?;}},
            5=>{let owner=self.media.reserved_mount.as_mut().unwrap();if owner.window_transient.as_ref().is_some_and(|capture|capture.handoff_complete()){owner.advance_phase(5,6,cx)?;}else{owner.capture_window_transient(&mut self.stores.window_transient_store,cx)?;}},
            6=>{let owner=self.media.reserved_mount.as_mut().unwrap();if owner.mounted.as_ref().unwrap().captured_child_content.is_some(){owner.advance_phase(6,7,cx)?;return Ok(false)}let mounted=owner.mounted.as_mut().unwrap();crate::original_emission::capture_child_content(&*self.child_composition.child_content_root,self.child_composition.child_content_generation,mounted,cx)?;},
            7=>{let owner=self.media.reserved_mount.as_ref().unwrap();if owner.mounted.is_none(){self.media.reserved_mount.as_mut().unwrap().advance_phase(7,8,cx)?;return Ok(false)}let mounted=owner.mounted.as_ref().unwrap();let operation=owner.operation.ok_or_else(||ValueError::invariant("reserved mount lost its operation"))?;let id=operation.operation.0;if mounted.canonical_revision!=self.stores.store.content_revision_now()||mounted.artifact_generation!=self.stores.store.generation_now()||mounted.config_generation!=self.stores.config_store.generation_now()||mounted.draft_generation!=self.stores.draft_store.generation_now()||mounted.captured_child_content_generation!=self.child_composition.child_content_generation||!self.tools.tool_operations.can_insert(id)||self.tools.typed_operation_reservations[id as usize%semio_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS]!=Some(id){return Err(ValueError::invariant("reserved mount changed its original destination or frontier").into())}let receipt=OriginalEmissionReceipt::capture(operation,cx)?;let mut output=None;if self.media.reserved_mount.as_mut().unwrap().take_mounted_into(&mut output,cx)?{self.tools.tool_operations.insert_admitted(id,output.unwrap());self.tools.release_reserved_emit_slot(id);self.media.reserved_receipt=Some(receipt);}},
            8=>{let owner=self.media.reserved_mount.as_mut().unwrap();if !owner.terminal_is_empty(){cx.with_original_retirement(|cx|owner.close_step(&self.tools.tool_cancellations,cx))?;return Ok(false)}let d=RetirementDemand{copy_bytes:size_of::<Option<Box<OriginalEmissionAdmission<A>>>>(),release_bytes:size_of::<OriginalEmissionAdmission<A>>(),depth:1,..Default::default()};cx.with_original_retirement(|cx|close_original_retained_turn(cx,d,|_|{self.media.reserved_mount.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}}))?;},            _=>return Err(ValueError::invariant("reserved mount has an invalid original phase").into())
        }Ok(false)
    }
    pub async fn admit_framework_reserved_spawn(&mut self, verb: &str, raw: &[u8], decoded_items: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<FrameworkReservedCommitPermit>, semio_framework_job::OriginalTurnFailure> {
        let proof = self.tools.qualified_tool_proof(&*self.stores, verb)?;
        if !matches!(proof, QualifiedToolView::FrameworkOwned(_) | QualifiedToolView::AppOwned(_)) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory")).into());
        }
        let key = proof.key();
        let admission = self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission")).into());
        }
        let max_output_bytes = proof.contract().max_output_bytes;
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,false,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,max_output_bytes).map(Some).map_err(Into::into)
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
            if pending.source.is_none(){cx.original_source_parent()?.admit_loan(&mut *pending.source,cx)?;return Ok(None)}
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
        if !matches!(proof,QualifiedToolView::FrameworkOwned(_)|QualifiedToolView::AppOwned(_)){return Err(plugin_sdk_fault("reserved producer lacks its exact registered publication proof").into())}
        let max_output_bytes=proof.contract().max_output_bytes;
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,true,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🔬️ Exercises real reserved slot admission without exporting private producer authority.
    #[cfg(any(test,feature="artifact-app-testing"))]
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
                self.tools.finish_reserved_emit_producer(&mut self.pending,permit);
                assert_eq!(self.tools.typed_operation_reservations, original);
                true
            }

            Ok(None)|Err(_) => { assert_eq!(self.tools.typed_operation_reservations, original); false }
        };
        self.tools.typed_operation_reservations = before;
        admitted
    }

    /// 🧪️ Keeps cancelled worker completion tied to the actual original permit through handoff.
    #[cfg(any(test,feature="artifact-app-testing"))]
    pub async fn test_reserved_emit_cancelled_worker_handoff(&mut self,meta:&ActionMeta,grant:semio_framework_value::RetainedCloneGrant)->bool{
        fn now()->Option<u64>{Some(1)}
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(899),semio_framework_job::Generation(3),grant,now,u64::MAX,1_000_000);
        let mut permit=None;
        for _ in 0..128{caller.turn(grant,|cx|{permit=self.admit_reserved_emit_producer("cut",meta,cx)?;Ok(())}).unwrap();if permit.is_some(){break}}
        let permit=permit.expect("original cut producer admission");let id=permit.operation.operation;permit.lease.cancel();
        let retained=permit.operation.operation==id&&self.tools.typed_operation_reservations[id.0 as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]==Some(id.0);
        self.tools.finish_reserved_emit_producer(&mut self.pending,permit);caller.retire(grant);retained
    }

    /// 🧷️ Drives the stepped clipboard `cut` mount until its producer settles into its admitted operation, or `None` while the turns stay unfunded.
    #[cfg(any(test,feature="artifact-app-testing"))]
    pub async fn test_mount_reserved_cut(&mut self,meta:&ActionMeta,turns:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::Operation>,semio_framework_job::OriginalTurnFailure>{
        let maximum=self.tools.qualified_tool_proof(&*self.stores,"cut")?.contract().max_raw_wire_bytes;
        let mut finished=None;
        for _ in 0..turns{if self.advance_original_reserved_action("cut",None,0,meta,maximum,&mut finished,cx).await?{break}}
        Ok(finished)    }
}
