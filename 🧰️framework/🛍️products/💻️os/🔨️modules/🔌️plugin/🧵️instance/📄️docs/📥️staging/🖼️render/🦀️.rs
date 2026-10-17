//! 🧺️ Staged slice of the `render` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🖼️ Render side of the mounted artifact instance: composition graph, interaction state, render, media jobs and media export.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep};
use semio_framework_plugin_document_media::media_export::{ActiveMediaExport, ActiveMediaExportCloseStage};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_operation::{ArtifactDocumentAuthority, ToolCancellationLease, ToolOperationKey};
use semio_framework_plugin_decl_views::media_contract::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportHandle, ArtifactMediaExportPoll, ArtifactOutputChunks, validate_media_export_structure};
use semio_framework_plugin_decl_app::media_contract::ArtifactMediaExportJobRequest;
use semio_framework_plugin_ownership::store_lifecycle::{ArtifactOwnedDisposer, ArtifactSnapshotCloseRetention};
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactReservedToolJob, ToolOwnerWitness};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom, MediaError};
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use std::mem::ManuallyDrop;
use crate::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_ui_contract::*;
use crate::docs_instance_root::MediaExportPreparation;

impl<A: ArtifactApp> MediaTransfers<A> {
    pub async fn receive_owned_media_poll(&mut self, stores: &mut InstanceStores<A>,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<ArtifactMediaExportPoll>,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(None)}
        match self.drive_owned_media_export(stores, operation_id, cx).await{
            Ok(poll)=>Ok(Some(poll)),
            Err(semio_framework_plugin_document_media::media_operation_refusal::MediaOperationFailure::Original(failure))=>Err(failure),
            Err(semio_framework_plugin_document_media::media_operation_refusal::MediaOperationFailure::Domain(source))=>self.retain_media_submission_refusal(source, cx)
        }
    }

    pub fn media_submission_refusal_terminal_is_empty(&self)->bool{self.media_download_preparation.is_none()&&self.media_export_preparation.is_none()&&self.media_submission_refusal.is_none()&&self.media_submission_refusal_retirement.is_none()&&self.media_submission_refusal_identity.is_none()}

    pub fn media_submission_refusal_demands(&self, tools: &ToolOperations<A>,body:usize)->Result<RetirementDemand,ValueError>{
        use semio_framework_value::retirement::RetireOwned;
        if let Some(original)=self.media_download_preparation.as_ref(){return original.retirement_demands(body)}
        if let Some(original)=self.media_export_preparation.as_ref(){
            if original.producer.is_some()||original.body.is_some()||original.request.is_some()||original.request_close.is_some()||original.parts.is_some()||original.parts_close.is_some()||original.handle.is_some()||original.handle_close.is_some()||original.metadata.is_some()||original.metadata_close.is_some()||original.snapshot.is_some()||original.snapshot_lease.is_some()||original.completion.is_some()||original.completion_close.is_some()||original.credit.is_some()||original.credit_close.is_some()||original.chunks.is_some()||original.chunks_close.is_some()||original.operation.is_some(){return original.retirement_demands(body)}
            if let Some(owner)=original.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}else{owner.retirement_demands(&tools.tool_cancellations,body)}}
            if original.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}
            if let Some(owner)=original.snapshot_retention.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.retirement_demands(body)}}
            if !original.snapshot_source.terminal_is_empty(){return original.snapshot_source.retirement_demands(body)}
            return Ok(RetirementDemand{copy_bytes:size_of::<MediaExportPreparation<A>>(),depth:1,..Default::default()})
        }
        if let Some(owner)=self.media_submission_refusal_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.media_submission_refusal_retirement),depth:1,..Default::default()})}else{owner.selected_demand(body)}}
        if self.media_submission_refusal.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        Ok(Default::default())
    }

    pub fn close_media_submission_refusal(&mut self, tools: &mut ToolOperations<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if let Some(original)=self.media_download_preparation.as_ref(){self.snapshot_retirements.get(original.operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending download lost its original snapshot source"))?.validate_retirement(cx)?}
        if let Some(original)=self.media_export_preparation.as_ref(){
            if let Some(retention)=original.snapshot_retention.as_ref(){if !retention.terminal_is_empty(){retention.validate_retirement(cx)?}}
            else if let Some(retention)=self.snapshot_retirements.get(original.key.operation_id.0){retention.validate_retirement(cx)?}
            else if !original.snapshot_source.terminal_is_empty(){original.snapshot_source.validate_retirement(cx)?}
            else if original.source_identity!=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"empty media preparation footer requires its unchanged original receiving source").into())}
        }
        let demand=self.media_submission_refusal_demands(tools, grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            if let Some(original)=self.media_download_preparation.as_mut(){if let Some(step)=original.close_step(grant)?{return Ok((PluginLifecycleStep::Progress(step.progress()),false))}self.media_download_preparation.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))}
            if let Some(original)=self.media_export_preparation.as_mut(){
                if let Some(progress)=original.close_material_step(cx,grant)?{return Ok((PluginLifecycleStep::Progress(progress),false))}
                let step=if let Some(owner)=original.retirement.as_mut(){if owner.terminal_is_empty(){RetainedCloneStep::Progress(semio_framework_plugin_operation::original_lease_cursor_release(&mut original.retirement,grant)?)}else{owner.close_step(&tools.tool_cancellations,grant)?}}else if original.lease.is_some(){let(owner,progress)=semio_framework_plugin_operation::original_lease_cursor_admit(&mut original.lease,grant)?;original.retirement=owner;RetainedCloneStep::Progress(progress)}
                else if let Some(owner)=original.snapshot_retention.as_mut(){if owner.terminal_is_empty(){original.snapshot_retention.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{return owner.close_step(cx,grant).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))}}
                else if !original.snapshot_source.terminal_is_empty(){return original.snapshot_source.close_step(cx).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))}
                else{self.media_export_preparation.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<MediaExportPreparation<A>>(),..Default::default()})};
                return Ok((PluginLifecycleStep::Progress(step.progress()),false))
            }
            if let Some(owner)=self.media_submission_refusal_retirement.as_mut(){
                if owner.terminal_is_empty(){drop(self.media_submission_refusal_retirement.take());self.media_submission_refusal_identity=None;return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),true))}
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok((PluginLifecycleStep::Progress(step.progress()),false));
            }
            if let Some(source)=self.media_submission_refusal.take(){
                match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>{*self.media_submission_refusal_retirement=Some(owner);Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))},Err((failure,source))=>{*self.media_submission_refusal=Some(source);Err(failure.into())}}
            }else{self.media_submission_refusal_identity=None;Ok((PluginLifecycleStep::Complete(Default::default()),true))}
        })
    }

    pub fn retain_media_submission_refusal<T>(&mut self,source:impl Into<semio_framework_plugin_document_media::media_operation_refusal::MediaOperationSource>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>{
        if self.media_submission_refusal.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending media source cannot be overwritten").into())}
        *self.media_submission_refusal=Some(semio_framework_plugin_document_media::media_operation_refusal::MediaOperationRefusal{source:source.into()});self.media_submission_refusal_identity=Some((cx.operation(),cx.generation()));let _=self.receive_media_submission_refusal(cx)?;Ok(None)
    }

    pub fn receive_media_submission_refusal(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media_submission_refusal.is_none(){return Ok(false)}
        if self.media_submission_refusal_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending media domain requires its unchanged initiating source identity").into())}
        if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.media_submission_refusal,cx)?{self.media_submission_refusal_identity=None;return Err(failure)}
        Ok(true)
    }

    pub async fn receive_owned_media_export(&mut self, stores: &mut InstanceStores<A>, tools: &mut ToolOperations<A>,port:&str,app_instance_id:u32,receiver:Option<&mut Option<ArtifactMediaExportHandle>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        match self.submit_owned_media_export(stores, tools, port, app_instance_id, receiver, cx).await{
            Ok(handle)=>Ok(handle),
            Err(semio_framework_plugin_document_media::media_operation_refusal::MediaOperationFailure::Domain(source))=>self.retain_media_submission_refusal::<()>(source, cx).map(|_|false),
            Err(semio_framework_plugin_document_media::media_operation_refusal::MediaOperationFailure::Original(failure))=>Err(failure)
        }
    }

    pub(crate) async fn submit_owned_media_export(&mut self, stores: &mut InstanceStores<A>, tools: &mut ToolOperations<A>,port:&str,app_instance_id:u32,mut receiver:Option<&mut Option<ArtifactMediaExportHandle>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_plugin_document_media::media_operation_refusal::MediaOperationFailure>{
        if self.media_export_preparation.is_none(){if let Some(handle)=receiver.as_deref().and_then(|slot|slot.as_ref()){
            let active=self.media_exports.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle lost its live operation"))?;
            if active.handle.original_loan()!=handle.original_loan()||active.port!=port{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle belongs to another operation").into())}
            self.snapshot_retirements.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle lost its original source"))?.validate(cx)?;return Ok(true)
        }}
        let source_identity=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);let port_identity=(port.as_ptr()as usize,port.len());let grant=cx.retained_grant();
        if let Some(original)=self.media_export_preparation.as_ref(){if original.source_identity!=source_identity||original.port_identity!=port_identity||original.key.app_instance_id!=app_instance_id{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media submission requires its unchanged original source and borrowed port").into())}}
        if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
        if self.media_export_preparation.is_none(){
            if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<MediaExportPreparation<A>>(){return Ok(false)}
            let Some(slot)=(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot|self.media_exports.slot_is_vacant(*slot)&&self.media_closures.slot_is_vacant(*slot)&&self.snapshot_retirements.slot_is_vacant(*slot))else{return Ok(false)};
            let revision=stores.store.content_revision_now();let operation_id=semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64,slot as u64);let key=ToolOperationKey{app_instance_id,document:ArtifactDocumentAuthority(app_instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())),generation:semio_framework_job::Generation(stores.store.generation_now())};
            *self.media_export_preparation=Some(MediaExportPreparation{canonical_revision:revision,metadata:ManuallyDrop::new(Some(std::array::from_fn(|_|Vec::new()))),metadata_stage:0,metadata_close:ManuallyDrop::new(None),request_close:ManuallyDrop::new(None),parts:ManuallyDrop::new(None),parts_close:ManuallyDrop::new(None),handle:ManuallyDrop::new(None),handle_close:ManuallyDrop::new(None),operation:None,body_ready:false,operation_bound:false,assembled:false,published:false,snapshot_source:semio_framework_plugin_ownership::store_lifecycle::ArtifactMediaSnapshotSource::empty(),snapshot:ManuallyDrop::new(None),snapshot_lease:ManuallyDrop::new(None),snapshot_retention:ManuallyDrop::new(None),request:ManuallyDrop::new(None),body:ManuallyDrop::new(None),producer:ManuallyDrop::new(None),source_identity,port_identity,key,lease:None,retirement:None,completion:None,completion_close:None,credit:None,credit_close:None,chunks:None,chunks_close:None});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<MediaExportPreparation<A>>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.media_export_preparation.as_mut().unwrap();if original.retirement.is_some(){return Ok(false)}
        if original.published{
            self.snapshot_retirements.get(original.key.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published media lost its original supplier"))?.validate(cx)?;
            let Some(receiver)=receiver.as_deref_mut()else{return Ok(true)};
            if original.handle.is_some(){let _=ArtifactMediaExportHandle::receive_original(&mut original.handle,receiver,cx)?;return Ok(false)}
            if receiver.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published media lost its retained receiving handle").into())}
            let bytes=size_of::<MediaExportPreparation<A>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            self.media_export_preparation.take();
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        if stores.store.content_revision_now()!=original.canonical_revision||stores.store.generation_now()!=original.key.generation.0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media preparation requires its exact captured canonical revision and generation").into())}
        let(id,registration)=tools.app_tool_registrations.iter().find(|(id,_)|id.strip_prefix("export-media:")==Some(port)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"media output lacks its published application factory"))?;
        if registration.owner!=semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness::of::<A>()||registration.key.tool_id!=*id{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media output requires its exact registered concrete application owner").into())}let contract=registration.contract;
        if original.snapshot.is_none()&&!original.assembled{
            if original.snapshot_source.ready(){let _=original.snapshot_source.handoff(&mut original.snapshot,&mut original.snapshot_lease,&mut original.snapshot_retention,cx)?;return Ok(false)}
            let factory=stores.store.owned_snapshot_retirement_factory()?;let store=&stores.store;let _=original.snapshot_source.admit_step(factory,||store.snapshot_owner(),cx)?;return Ok(false)
        }
        if let Some(retention)=original.snapshot_retention.as_ref(){retention.validate(cx)?}else{self.snapshot_retirements.get(original.key.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media producer lost its retained original source and installed supplier"))?.validate(cx)?}
        if original.lease.is_none(){let Some(lease)=tools.tool_cancellations.begin(&original.key,cx)?else{return Ok(false)};original.lease=Some(lease);return Ok(false)}
        if !original.assembled{
            if original.completion.is_none(){let _=ArtifactMediaExportCompletion::original_birth_admit(&mut original.completion,cx)?;return Ok(false)}
            if original.credit.is_none(){let _=ArtifactMediaExportCredit::original_birth_admit(contract.max_output_bytes,&mut original.credit,cx)?;return Ok(false)}
            if original.chunks.is_none(){let _=ArtifactOutputChunks::original_birth_admit(contract.max_output_bytes,&mut original.chunks,cx)?;return Ok(false)}
            if original.metadata_stage<6{
                let index=original.metadata_stage;let input=match index{0=>id.as_bytes(),1|3|5=>stores.store.envelope().id.as_bytes(),2|4=>port.as_bytes(),_=>unreachable!()};let output=&mut original.metadata.as_mut().unwrap()[index];
                if output.capacity()==0&&!input.is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<input.len(){return Ok(false)}output.try_reserve_exact(input.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"media original metadata allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
                if output.len()<input.len(){let count=(input.len()-output.len()).min(grant.maximum_copy_bytes);if grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}let start=output.len();output.extend_from_slice(&input[start..start+count]);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
                if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<usize>(){return Ok(false)}original.metadata_stage+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
            }
            let copy=size_of::<ArtifactMediaExportJobRequest<A>>()+size_of::<semio_framework_plugin_document_media::media_export::MediaExportOwnedParts>()+size_of::<ArtifactMediaExportHandle>()+size_of::<ArtifactMediaExportCredit>()+size_of::<ArtifactOutputChunks>()+size_of::<ArtifactMediaExportCompletion>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}
            let text=|bytes:Vec<u8>|String::from_utf8(bytes).expect("original source metadata retains its complete UTF-8 bytes");let fields=original.metadata.as_mut().unwrap();let tool_id=text(std::mem::take(&mut fields[0]));let parent_document_id=text(std::mem::take(&mut fields[1]));let request_port=text(std::mem::take(&mut fields[2]));let parts_document=text(std::mem::take(&mut fields[3]));let parts_port=text(std::mem::take(&mut fields[4]));let published_document=text(std::mem::take(&mut fields[5]));
            let operation=semio_framework_job::Operation::new(original.key.operation_id,original.key.base_revision,original.key.generation,original.key.operation_id.0^original.key.base_revision.0);
            let completion=original.completion.take().unwrap();let output_credit=original.credit.take().unwrap();let output_chunks=original.chunks.take().unwrap();
            *original.parts=Some(semio_framework_plugin_document_media::media_export::MediaExportOwnedParts{handle:ArtifactMediaExportHandle{app_instance_id,parent_document_id:parts_document,operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation},port:parts_port,retained_result:None,output_credit:output_credit.clone(),output_chunks:output_chunks.clone(),completion:completion.clone()});
            *original.handle=Some(ArtifactMediaExportHandle{app_instance_id,parent_document_id:published_document,operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation});
            *original.request=Some(ArtifactMediaExportJobRequest{source_identity:(cx.operation(),cx.generation()),retained:cx.retained_grant(),operation,tool_id,parent_document_id,canonical_base_revision:original.canonical_revision,snapshot:original.snapshot.take().unwrap(),snapshot_close:original.snapshot_lease.take().unwrap(),port:request_port,output_credit,output_chunks,completion});original.operation=Some(operation);original.assembled=true;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if !original.body_ready{original.body_ready=A::build_media_export_job(&mut original.request,&mut original.body,cx)?;if original.body_ready&&(original.request.is_some()||original.body.is_none()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media factory readiness must retain one actual body and consume its exact request").into())}return Ok(false)}
        if original.producer.is_none(){let _=ArtifactReservedToolJob::admit_boxed_original(&mut original.body,&mut original.producer,cx)?;return Ok(false)}
        if !original.operation_bound{original.operation_bound=original.producer.as_mut().unwrap().bind_original_operation(&mut original.operation,cx)?;return Ok(false)}
        let bytes=size_of::<semio_framework_plugin_document_media::media_export::ActiveMediaExport>()+size_of::<ArtifactSnapshotCloseRetention>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let operation=original.request.as_ref().map_or_else(||semio_framework_job::Operation::new(original.key.operation_id,original.key.base_revision,original.key.generation,original.key.operation_id.0^original.key.base_revision.0),|request|request.operation);let operation_id=operation.operation;
        let retention=original.snapshot_retention.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media publication lost its actual snapshot supplier"))?;
        self.snapshot_retirements.insert_admitted(operation_id.0,retention);
        let active=ActiveMediaExport{parts:ManuallyDrop::new(original.parts.take()),parts_retirement:ManuallyDrop::new(None),cancellation_retirement:None,operation,contract,session:None,session_rejected:Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(original.producer.take().unwrap(),(cx.operation(),cx.generation()))),outcome_pending:false,resume_pending:false,lease:ManuallyDrop::new(original.lease.take()),checkpoint_available:false,applied_progress:0,download_pending:false,close_stage:ActiveMediaExportCloseStage::Live};
        self.media_exports.insert_admitted(operation_id.0,active);self.current_media_export=Some(operation_id.0);original.published=true;
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
        Ok(false)
    }

    pub(crate) fn quarantine_media_snapshot(&self, operation_id: u64, active: &ActiveMediaExport) -> Result<(), MediaError> {
        self.snapshot_retirements.get(operation_id).map(|_| ()).ok_or_else(|| MediaError::Payload(active.port.clone(), "media export lost its pre-admitted app-owned snapshot retirement authority".into()))
    }

    pub(crate) fn finish_media_poll(&mut self, operation_id: u64, active: ActiveMediaExport, poll: ArtifactMediaExportPoll) -> Result<ArtifactMediaExportPoll,semio_framework_plugin_document_media::media_operation_refusal::MediaOperationFailure> {
        if let Err(error) = self.quarantine_media_snapshot(operation_id, &active) {
            debug_assert!(self.media_exports.can_insert(operation_id));
            self.media_exports.insert_admitted(operation_id, active);
            return Err(error.into());
        }
        debug_assert!(self.media_closures.can_insert(operation_id));
        self.media_closures.insert_admitted(operation_id, active);
        Ok(poll)
    }

    pub(crate) async fn drive_owned_media_export(&mut self, stores: &mut InstanceStores<A>, operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactMediaExportPoll,semio_framework_plugin_document_media::media_operation_refusal::MediaOperationFailure> {
        self.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media step lost its original retained source and snapshot supplier"))?.validate(cx)?;
        if !self.media_closures.can_insert(operation_id) {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media close authority is saturated or already owns this operation").into());
        }
        if self.media_exports.get(operation_id).is_none() {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media registry lost its admitted operation").into());
        }
        let mut active = self.media_exports.remove(operation_id).expect("checked exact media export owner");
        let grant=cx.retained_grant();
        if active.resume_pending || active.outcome_pending {
            if let Err(failure)=active.retire_nonterminal_outcome(cx){self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
            let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);
        }
        if let Some(rejected) = active.session_rejected.as_mut() {
            if rejected.terminal_is_empty() {
                if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of_val(rejected){let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);}
                let cancelled=cx.is_cancelled()||active.lease.as_ref().expect("live media lease").token.is_cancelled_now();drop(active.session_rejected.take());
                return self.finish_media_poll(operation_id, active, if cancelled{ArtifactMediaExportPoll::Cancelled}else{ArtifactMediaExportPoll::Failed("media export mounted session capacity is exhausted".into())});
            }
            if rejected.admission_pending()&&!cx.is_cancelled()&&!active.lease.as_ref().expect("live media lease").token.is_cancelled_now(){
                let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_media_export_submission",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let result=rejected.admit_pending(config,cx);
                match result{Ok(Some(session))=>{active.session=Some(session);drop(active.session_rejected.take());},Ok(None)=>{},Err(failure)=>{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}}
            }else{
                let demand=rejected.retirement_demands(grant.maximum_copy_bytes)?;
                let result=semio_framework_plugin_document_media::original_rejection_lifecycle(cx,demand,grant,"media rejection awaits its original worker",|cx|{let step=rejected.close_step(cx)?;Ok((step,rejected.terminal_is_empty()))});
                if let Err(failure)=result{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
            }
            let poll = active.running_poll();
            self.media_exports.insert_admitted(operation_id, active);
            return Ok(poll);
        }
        if grant.maximum_items==0||grant.maximum_depth==0{let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);}
        let pump = active.session.as_mut().map(|session| session.pump_one_for_interactive_turn(cx));
        match pump {
            Some(Ok(semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal)) => {}
            Some(Err(failure))=>{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());},
            Some(Ok(_)) => {
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                return Ok(poll);
            }
            None => return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export lost its mounted session".into())),
        }
        let receipt = active.session.as_mut().and_then(semio_framework_job_worker::MountedWorkerJobSession::take_checked_out_retained_step_receipt);
        if !receipt.is_some_and(|(issued, progress)| progress.fits(issued) && progress.fits(grant)) {
            return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export mounted worker receipt is absent or exceeded its issued grant".into()));
        }
        enum SeenMediaOutcome { Progress, Checkpoint { applied_progress: u64, state_len: usize }, Complete { output_len: usize }, Cancelled, Fault(String), Lost }
        let seen = match active.session.as_ref().map(semio_framework_job_worker::MountedWorkerJobSession::checked_out_outcome) {
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Yield { .. } | semio_framework_job::JobOutcomeView::PreviewReady { .. }))) => SeenMediaOutcome::Progress,
            Some(Ok(Some(semio_framework_job::JobOutcomeView::CheckpointReady { state, applied_progress, .. }))) => SeenMediaOutcome::Checkpoint { applied_progress, state_len: state.len() },
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Complete { output, .. }))) => SeenMediaOutcome::Complete { output_len: output.map_or(0, semio_framework_job::RetainedJobPayload::len) },
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Cancelled { .. }))) => SeenMediaOutcome::Cancelled,
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Fault { detail, .. }))) => SeenMediaOutcome::Fault(detail.single_page().and_then(|bytes| std::str::from_utf8(bytes).ok()).filter(|detail| !detail.is_empty()).unwrap_or("media export job faulted").to_string()),
            _ => SeenMediaOutcome::Lost,
        };
        match seen {
            SeenMediaOutcome::Progress => {
                active.outcome_pending = true;
                if let Err(failure) = active.retire_nonterminal_outcome(cx) {self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                Ok(poll)
            }
            SeenMediaOutcome::Checkpoint { applied_progress, state_len } => {
                if applied_progress < active.applied_progress || state_len > active.contract.max_output_bytes {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export checkpoint is non-monotone or oversized".into()));
                }
                active.applied_progress = applied_progress;
                active.checkpoint_available = true;
                active.outcome_pending = true;
                if let Err(failure) = active.retire_nonterminal_outcome(cx) {self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                Ok(poll)
            }
            SeenMediaOutcome::Complete { output_len } => {
                if output_len > active.contract.max_output_bytes || active.lease.as_ref().expect("live media lease").is_cancelled().await {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export was cancelled or its commit candidate exceeded the output cap".into()));
                }
                let live_revision_bytes = stores.store.content_revision();
                let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
                let live_generation = semio_framework_job::Generation(stores.store.generation());
                if !matches!(semio_framework_job::validate_commit(&active.operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export completed against a stale revision or generation".into()));
                }
                if active.retained_result.is_none(){match active.completion.take(){Ok(Some(result))=>active.retained_result=Some(result),Ok(None)=>return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export completed without producer output".into())),Err(error)=>return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(error.message))};}
                let result=match active.retained_result.as_ref(){Some(Ok(result))=>result,Some(Err(error))=>{let Some(Err(source))=active.retained_result.take()else{unreachable!("observed exact original media domain result")};self.media_closures.insert_admitted(operation_id,active);return Err(source.into());},None=>return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export lost its original completion result".into()))};
                let output_bytes = match active.output_credit.validate_terminal() {
                    Ok(output_bytes) => output_bytes,
                    Err(error) => return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(error.message)),
                };
                if let Err(error) = validate_media_export_structure(result, &active.output_chunks, output_bytes, active.contract.max_output_bytes) {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(error.message));
                }
                active.lease.as_mut().expect("live media lease").finish_in_place();
                let Some(Ok(result))=active.retained_result.take()else{return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export original completion changed during publication".into()));};
                if self.current_media_export == Some(operation_id) {
                    self.current_media_export = None;
                }
                self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Complete(result))
            }
            SeenMediaOutcome::Cancelled => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Cancelled),
            SeenMediaOutcome::Fault(detail) => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(detail)),
            SeenMediaOutcome::Lost => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export original outcome changed during publication".into())),
        }
    }

    pub async fn cancel_owned_media_export(&mut self, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        let operation_id = handle.operation_id.0;
        if let Some(output) = self.segmented_downloads.get(operation_id) {
            if !output.owns_media_export(handle) {
                return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export download handle authority mismatch".into()));
            }
            if !self.segmented_closures.can_insert(operation_id) {
                return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export download cleanup authority is saturated".into()));
            }
            let output = self.segmented_downloads.remove(operation_id).ok_or_else(|| MediaError::Payload(handle.parent_document_id.clone(), "media export download changed before cancellation".into()))?;
            self.segmented_closures.insert_admitted(operation_id,semio_framework_plugin_document_media::media_export::MediaDownloadRetirement::pending(output));
            return Ok(());
        }
        if !self.media_closures.can_insert(operation_id) {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "media close authority is saturated or already owns this operation".into()));
        }
        let Some(active) = self.media_exports.get(operation_id) else {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "unknown or completed media export handle".into()));
        };
        if active.handle != *handle {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export handle authority mismatch".into()));
        }
        self.quarantine_media_snapshot(operation_id, active)?;
        let active = self.media_exports.remove(operation_id).expect("checked exact media export owner");
        self.media_closures.insert_admitted(operation_id, active);
        if self.current_media_export == Some(operation_id) {
            self.current_media_export = None;
        }
        Ok(())
    }
}
