//! 📥️ Latest-wins command admission and the typed command wire writer.
use semio_framework::Fault;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_async::PublicationClaimHandle;
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleStep, PluginLifecycleFailure};
use semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand;
use semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step;
use semio_framework_plugin_job::reserved_jobs::QualifiedToolProof;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_operation::ToolCancellationLease;
use semio_framework_plugin_job::TYPED_OPERATION_RESULT_PAGE_BYTES;



#[doc(hidden)]
#[path="../🗝️latest/🦀️.rs"]
pub mod original_latest_key;

use crate::command_admission::original_latest_key::ToolLatestWinsKeyCopy;

pub(crate) struct ToolLatestWinsScope {
    pub key: semio_framework_value_ordered::ordered::SharedOwner<String>,
    pub operation: u64,
    pub token: semio_framework_job::CancelToken,
    pub claim: PublicationClaimHandle,
}

impl ToolLatestWinsScope {
    pub fn cancel(&self) {
        self.claim.cancel();
        self.token.cancel_now();
    }
}

#[doc(hidden)]
pub struct ToolLatestWinsRegistry {
    pub map: Option<semio_framework_value_ordered::ordered::OrderedMap<ToolLatestWinsScope>>,
    pub update: Option<semio_framework_value_ordered::ordered::UpdateCursor<ToolLatestWinsScope>>,
    pub retirement: Option<semio_framework_value_ordered::ordered::Retirement<ToolLatestWinsScope>>,
    pub retired_key: Option<String>,
    retired_shared_key: Option<semio_framework_value_ordered::ordered::SharedOwner<String>>,
    retired_token: Option<semio_framework_job::CancelToken>,
    retired_claim: Option<PublicationClaimHandle>,
    token_retirement: Option<ControlledRetirement<semio_framework_job::CancelToken>>,
    claim_retirement: Option<ControlledRetirement<PublicationClaimHandle>>,
    removed_scope: Option<semio_framework_value_ordered::ordered::SharedOwner<ToolLatestWinsScope>>,
    retired_scope:ManuallyDrop<Option<ToolLatestWinsScope>>,
    pending_scope:ManuallyDrop<Option<ToolLatestWinsScope>>,
    pub active_operation: Option<u64>,
    pub outcome: Option<(u64, bool)>,
    pub scan: usize,
    admission_scan: Option<usize>,
    update_closing: bool,
    pub closing: bool,
}

impl ToolLatestWinsRegistry {
    pub fn new() -> Self {
        Self { map: Some(semio_framework_value_ordered::ordered::OrderedMap::new()), update: None, retirement: None, retired_key: None, retired_shared_key: None, retired_token: None, retired_claim: None, token_retirement: None, claim_retirement: None, removed_scope: None, retired_scope:ManuallyDrop::new(None),pending_scope:ManuallyDrop::new(None),active_operation: None, outcome: None, scan: 0, admission_scan: None, update_closing: false, closing: false }
    }

    pub fn can_begin(&self) -> bool {
        !self.closing && self.update.is_none() && self.retirement.is_none() && self.retired_key.is_none() && self.retired_shared_key.is_none() && self.removed_scope.is_none() && self.retired_scope.is_none()&&self.pending_scope.is_none()&&self.retired_token.is_none()&&self.retired_claim.is_none()&&self.token_retirement.is_none()&&self.claim_retirement.is_none()&&self.outcome.is_none()
    }

    pub fn begin(&mut self,operation:u64,key:&semio_framework_value_ordered::ordered::SharedOwner<String>,lease:&ToolCancellationLease,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,Fault>{
        let capacity=semio_framework_value_ordered::ordered::SharedOwner::<ToolLatestWinsScope>::allocation_bytes();
        if !self.can_begin()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<capacity{return Ok(None);}
        if self.map.as_ref().expect("live original key map").len()>=ARTIFACT_LIVE_OUTPUT_SLOTS{match self.admission_scan{None=>{self.admission_scan=Some(ARTIFACT_LIVE_OUTPUT_SLOTS);return Ok(Some(RetainedCloneProgress{copied_items:1,..Default::default()}));},Some(0)=>{},Some(_)=>return Ok(None)}}
        let scope=ToolLatestWinsScope{key:key.clone(),operation,token:lease.token.clone(),claim:lease.publication_claim.clone()};
        let(scope,progress)=match semio_framework_value_ordered::ordered::SharedOwner::admit(scope,grant){Ok(value)=>value,Err((error,original))=>{*self.pending_scope=Some(original);return Err(semio_framework_plugin_decl_emission::retirement_refusal_fault(&error));}};
        self.update=Some(self.map.as_ref().expect("original live key map").begin_set_shared(key.clone(),scope));self.active_operation=Some(operation);self.update_closing=false;Ok(Some(progress))
    }

    pub fn take_outcome(&mut self, operation: u64) -> Option<bool> {
        if self.outcome.is_some_and(|(owner, _)| owner == operation) {
            self.outcome.take().map(|(_, accepted)| accepted)
        } else {
            None
        }
    }

    pub fn cancel(&mut self, operation: u64) {
        if self.active_operation == Some(operation) && !self.update_closing {
            if let Some(update) = self.update.as_mut() {
                update.begin_close();
            }
            self.update_closing = true;
            self.outcome = Some((operation, false));
        }
    }

    fn retire_scope(&mut self,scope:ToolLatestWinsScope){assert!(self.retired_scope.is_none(),"one original latest scope frontier");*self.retired_scope=Some(scope);}
    pub fn retirement_step(&mut self,step:semio_framework_value_ordered::ordered::RetirementStep<ToolLatestWinsScope>)->Result<RetainedCloneStep,Fault>{
        use semio_framework_value_ordered::ordered::RetirementStep;
        Ok(match step{RetirementStep::Blocked=>RetainedCloneStep::Progress(Default::default()),RetirementStep::Progress{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}),RetirementStep::ProcessedBytes(copied_bytes)=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}),RetirementStep::OwnedValue(scope)=>{self.retire_scope(scope);RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})},RetirementStep::Failure(error)=>return Err(semio_framework_plugin_decl_emission::retirement_refusal_fault(&error)),RetirementStep::Complete=>RetainedCloneStep::Progress(Default::default())})
    }
    /// 📏️ Quotes the exact next [`Self::advance`] turn, one original owner at a time in the same order.
    pub fn advance_demands(&self)->Result<RetirementDemand,ValueError>{
        let item=RetirementDemand{depth:1,..Default::default()};
        let body=TYPED_OPERATION_RESULT_PAGE_BYTES;
        let scope_copy=RetirementDemand{copy_bytes:size_of::<ToolLatestWinsScope>(),..item};
        if self.pending_scope.is_some()&&self.retired_scope.is_none(){return Ok(scope_copy);}
        if self.retired_token.is_some()||self.token_retirement.is_some(){return plugin_typed_owner_demand(&self.retired_token,&self.token_retirement,body);}
        if self.retired_claim.is_some()||self.claim_retirement.is_some(){return plugin_typed_owner_demand(&self.retired_claim,&self.claim_retirement,body);}
        if let Some(scope)=self.retired_scope.as_ref(){
            if !scope.key.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:scope.key.next_release_byte_demand(),..item});}
            if self.retired_key.is_none()&&self.retired_shared_key.is_none()&&self.removed_scope.is_none(){return Ok(scope_copy);}
        }
        if let Some(scope)=self.removed_scope.as_ref(){return Ok(if scope.terminal_is_empty(){item}else{RetirementDemand{release_bytes:scope.next_release_byte_demand(),..item}});}
        if let Some(key)=self.retired_shared_key.as_ref(){return Ok(if key.terminal_is_empty(){item}else{RetirementDemand{release_bytes:key.next_release_byte_demand(),..item}});}
        if let Some(key)=self.retired_key.as_ref(){return Ok(RetirementDemand{release_bytes:key.capacity(),..item});}
        if let Some(retirement)=self.retirement.as_ref(){return Ok(if retirement.terminal_is_empty(){item}else{RetirementDemand{copy_bytes:retirement.next_copy_byte_demand(),release_bytes:retirement.next_close_byte_demand()?,depth:retirement.next_depth_demand().max(1),..Default::default()}});}
        if let Some(update)=self.update.as_ref(){
            if self.update_closing{return Ok(if update.terminal_is_empty(){item}else{RetirementDemand{copy_bytes:update.next_close_copy_byte_demand(),release_bytes:update.next_close_byte_demand()?,depth:update.next_close_depth_demand().max(1),..Default::default()}});}
            if !update.is_complete(){return Ok(RetirementDemand{copy_bytes:update.next_copy_byte_demand(),capacity_bytes:update.next_capacity_byte_demand()?,depth:update.next_depth_demand().max(1),..Default::default()});}
            return Ok(item);
        }
        Ok(if self.map.as_ref().is_some_and(|map|self.closing||!map.is_empty()){item}else{Default::default()})
    }
    pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let idle=RetainedCloneProgress::default();let item=RetainedCloneProgress{copied_items:1,..idle};
        let demand=self.advance_demands().map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;
        if !plugin_grant_funds(demand,grant){return Ok(RetainedCloneStep::Progress(idle));}
        if self.retired_scope.is_none(){if let Some(scope)=self.pending_scope.take(){self.retire_scope(scope);return Ok(RetainedCloneStep::Progress(item));}}
        if self.retired_token.is_some()||self.token_retirement.is_some(){let step=plugin_typed_owner_step(&mut self.retired_token,&mut self.token_retirement,grant).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.retired_claim.is_some()||self.claim_retirement.is_some(){let step=plugin_typed_owner_step(&mut self.retired_claim,&mut self.claim_retirement,grant).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(scope)=self.retired_scope.as_mut(){
            if !scope.key.terminal_is_empty(){let step=scope.key.release_step(grant).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;self.retired_key=step.value;return Ok(RetainedCloneStep::Progress(step.progress));}
            if self.retired_key.is_none()&&self.retired_shared_key.is_none()&&self.removed_scope.is_none(){
                let ToolLatestWinsScope{key,token,claim,..}=self.retired_scope.take().expect("original retired latest scope");
                drop(key);self.retired_token=Some(token);self.retired_claim=Some(claim);
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ToolLatestWinsScope>(),..idle}));
            }
        }
        if let Some(scope)=self.removed_scope.as_mut(){if scope.terminal_is_empty(){self.removed_scope.take();return Ok(RetainedCloneStep::Progress(item));}let step=scope.release_step(grant).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;if let Some(scope)=step.value{self.retire_scope(scope);}return Ok(RetainedCloneStep::Progress(step.progress));}
        if let Some(key)=self.retired_shared_key.as_mut(){if key.terminal_is_empty(){self.retired_shared_key.take();return Ok(RetainedCloneStep::Progress(item));}let step=key.release_step(grant).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;self.retired_key=step.value;return Ok(RetainedCloneStep::Progress(step.progress));}
        if let Some(key)=self.retired_key.as_ref(){let released_bytes=key.capacity();if released_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(idle));}drop(self.retired_key.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes,..item}));}
        if let Some(retirement)=self.retirement.as_mut(){if retirement.terminal_is_empty(){self.retirement.take();return Ok(RetainedCloneStep::Progress(item));}let step=retirement.advance(grant);return self.retirement_step(step);}
        if let Some(update)=self.update.as_mut(){
            if self.update_closing{if update.terminal_is_empty(){self.update.take();self.active_operation=None;self.update_closing=false;return Ok(RetainedCloneStep::Progress(item));}let step=update.close_step(grant);return self.retirement_step(step);}
            if !update.is_complete(){let step=update.advance(grant).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
            let candidate=update.take_result().expect("complete original key candidate");self.admission_scan=None;
            if candidate.len()>ARTIFACT_LIVE_OUTPUT_SLOTS{self.retirement=Some(candidate.retire());if let Some(operation)=self.active_operation{self.outcome=Some((operation,false));}}
            else{let previous=self.map.replace(candidate).expect("original live key map");self.retirement=Some(previous.retire());if let Some(scope)=update.take_removed(){if self.active_operation.is_some_and(|operation|operation!=scope.operation){scope.cancel();}self.removed_scope=Some(scope);}if let Some(operation)=self.active_operation{self.outcome=Some((operation,true));}}
            update.begin_close();self.update_closing=true;return Ok(RetainedCloneStep::Progress(item));
        }
        if self.closing{if let Some(map)=self.map.take(){self.retirement=Some(map.retire());return Ok(RetainedCloneStep::Progress(item));}return Ok(RetainedCloneStep::Complete(idle));}
        let map=self.map.as_ref().expect("original live key map");if map.is_empty(){return Ok(RetainedCloneStep::Progress(idle));}self.scan%=map.len();let(_,scope)=map.entry_at_rank(self.scan).expect("bounded original map rank");self.scan+=1;if let Some(remaining)=self.admission_scan.as_mut(){*remaining=remaining.saturating_sub(1);}if scope.claim.is_finished(){self.update=Some(map.begin_remove_shared(scope.key.clone()));self.update_closing=false;}Ok(RetainedCloneStep::Progress(item))
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(update) = self.update.as_mut() {
            update.begin_close();
            self.update_closing = true;
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.map.is_none() && self.update.is_none() && self.retirement.is_none() && self.retired_key.is_none() && self.retired_shared_key.is_none() && self.removed_scope.is_none()&&self.retired_scope.is_none()&&self.pending_scope.is_none()&&self.retired_token.is_none()&&self.retired_claim.is_none()&&self.token_retirement.is_none()&&self.claim_retirement.is_none()
    }
}

#[doc(hidden)]
pub struct AdmittedToolCommand {
    pub verb: String,
    pub proof: QualifiedToolProof,
    pub wire_admission: semio_framework::ToolWireAdmission,
    pub decoded_items: usize,
    pub raw_wire: semio_framework::action_bus::RetainedToolWireInput,
}

#[doc(hidden)]
pub struct PendingLatestWinsCommand<A: ArtifactApp> {
    pub command: ManuallyDrop<Option<Box<A::Command>>>,
    pub disposer: Box<dyn ArtifactOwnedDisposer<A::Command>>,
    pub admission: Option<AdmittedToolCommand>,
    pub meta: ActionMeta,
    pub operation: semio_framework_job::Operation,
    pub source_identity:semio_framework_job::OriginalSourceBinding,
    pub revision: [u8; 32],
    pub lease: Option<ToolCancellationLease>,
    pub key_copy: Option<ToolLatestWinsKeyCopy>,
    pub key: Option<semio_framework_value_ordered::ordered::SharedOwner<String>>,
    pub retired_key: Option<String>,
    pub lookup_started: bool,
    pub accepted: bool,
    pub restarting: bool,
    pub closing: bool,
    pub receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub fault:ManuallyDrop<Option<PluginLifecycleFailure>>,
    pub fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
}

impl<A: ArtifactApp> PendingLatestWinsCommand<A> {
    pub fn admit_original_lease(&mut self,issuer:&mut semio_framework_plugin_operation::ToolCancellationIssuer,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.closing||self.lease.is_some(){return Ok(self.lease.is_some())}
        if !self.source_identity.matches(cx){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted pending lease requires its original command caller").into())}
        let key=semio_framework_plugin_operation::ToolOperationKey{app_instance_id:self.meta.instance_id,document:semio_framework_plugin_operation::ArtifactDocumentAuthority(self.meta.instance_id),operation_id:self.operation.operation,base_revision:self.operation.base_revision,generation:self.operation.generation};
        self.lease=issuer.begin_keyed(&key,cx)?;
        Ok(self.lease.is_some())
    }

    pub fn record_original_receipt(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),Fault>{
        if progress!=RetainedCloneProgress::default(){if self.receipt.is_some(){return Err(plugin_sdk_fault("latest-wins original receipt awaits its caller handback").with_retained_progress(progress));}self.receipt=Some((grant,progress));}
        if !progress.fits(grant){return Err(plugin_sdk_fault("latest-wins original receipt exceeded its caller").with_retained_progress(progress));}
        Ok(())
    }
    fn key_demands(&self)->Result<RetirementDemand,ValueError>{
        if let Some(copy)=self.key_copy.as_ref(){return Ok(if copy.key.is_none(){RetirementDemand{depth:1,..Default::default()}}else{copy.retirement_demands()});}
        if let Some(key)=self.key.as_ref(){return Ok(RetirementDemand{release_bytes:key.next_release_byte_demand(),depth:1,..Default::default()});}
        Ok(RetirementDemand{release_bytes:self.retired_key.as_ref().map_or(0,String::capacity),depth:usize::from(self.retired_key.is_some()),..Default::default()})
    }
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.fault_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        if self.fault.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<PluginLifecycleFailure>(),depth:1,..Default::default()})}
        if let Some(command)=self.command.as_ref(){
            if self.disposer.terminal_is_empty(command){return Ok(RetirementDemand{release_bytes:size_of_val(command.as_ref()),depth:1,..Default::default()});}
            let mut demand=self.disposer.retirement_demands(command,body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"latest-wins command child depth overflow"))?;return Ok(demand);
        }
        if self.key_copy.is_some()||self.key.is_some()||self.retired_key.is_some(){return self.key_demands();}
        if let Some(admission)=self.admission.as_ref(){if !admission.raw_wire.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:admission.raw_wire.next_close_copy_byte_demand()?,capacity_bytes:admission.raw_wire.next_close_capacity_byte_demand(body)?,release_bytes:admission.raw_wire.next_close_release_byte_demand()?,depth:admission.raw_wire.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"latest-wins original wire depth overflow"))?});}}
        Ok(Default::default())
    }
    pub fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        let idle=RetainedCloneProgress::default();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if !Self::permits(grant,demand){return Ok(PluginLifecycleStep::Progress(idle))}
        if let Some(owner)=self.fault_retirement.as_mut(){let step=owner.close_step(grant)?;if owner.terminal_is_empty(){self.fault_retirement.take();}return Ok(PluginLifecycleStep::Progress(step.progress()))}
        if let Some(original)=self.fault.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{self.fault_retirement=Some(owner);Ok(PluginLifecycleStep::Progress(progress))},Err((error,original))=>{*self.fault=Some(original);Err(error.into())}}}
        if let Some(command)=self.command.as_mut(){
            if self.disposer.terminal_is_empty(command){drop(self.command.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..idle}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=self.disposer.close_step(command,child)?;let progress=step.progress().unwrap_or_default();
            semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original latest command disposer")?;
            if matches!(step,PluginLifecycleStep::Complete(_))&&!self.disposer.terminal_is_empty(command){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"latest-wins command disposer reported a false terminal").with_retained_progress(progress).into())}
            return Ok(match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step})
        }
        if self.key_copy.is_some()||self.key.is_some()||self.retired_key.is_some(){return self.close_key_step(grant)}
        if let Some(admission)=self.admission.as_mut(){if !admission.raw_wire.terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=admission.raw_wire.close_step(child)?.admit(child,admission.raw_wire.terminal_is_empty())?;return Ok(match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>PluginLifecycleStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason:"latest-wins original raw input awaits handback"}})}}
        Ok(PluginLifecycleStep::AwaitingInput{reason:"latest-wins original metadata and cancellation await their native retirement issuer"})
    }
    pub fn close_key_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        let idle=RetainedCloneProgress::default();if !Self::permits(grant,self.key_demands()?){return Ok(PluginLifecycleStep::Progress(idle))}
        if let Some(copy)=self.key_copy.as_mut(){if copy.key.is_none(){self.key_copy.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}))}return Ok(PluginLifecycleStep::Progress(copy.close_step(grant).progress()))}
        if let Some(key)=self.key.as_mut(){if key.terminal_is_empty(){self.key.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}))}let step=key.release_step(grant)?;self.retired_key=step.value;return Ok(PluginLifecycleStep::Progress(step.progress))}
        if let Some(key)=self.retired_key.as_ref(){let released_bytes=key.capacity();drop(self.retired_key.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..idle}))}
        Ok(PluginLifecycleStep::Complete(idle))
    }
    pub fn command_owners_are_empty(&self)->bool{self.command.is_none()&&self.admission.is_none()&&self.key_copy.is_none()&&self.key.is_none()&&self.retired_key.is_none()&&self.fault.is_none()&&self.fault_retirement.is_none()}
}

impl<A: ArtifactApp> Drop for PendingLatestWinsCommand<A> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(std::thread::panicking() || self.command_owners_are_empty(), "latest-wins admission must transfer or cursor-retire its exact command/key/raw owners");
        }
    }
}

#[doc(hidden)]
pub struct TypedCommandWireWriter {
    pub input: semio_framework::action_bus::RetainedToolWireInput,
    pub page: [u8; semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES],
    pub len: usize,
}

impl TypedCommandWireWriter {
    fn flush_page(&mut self) -> std::io::Result<()> {
        if self.len == 0 {
            return Ok(());
        }
        let page = semio_framework::action_bus::ToolWirePage::try_copy_from(&self.page[..self.len]).map_err(|error| std::io::Error::new(std::io::ErrorKind::WriteZero, error.detail))?;
        self.input.admit_page(page).map_err(|(error, _)| std::io::Error::new(std::io::ErrorKind::WriteZero, error.detail))?;
        self.len = 0;
        Ok(())
    }

    pub fn finish(mut self) -> Result<semio_framework::action_bus::RetainedToolWireInput, Fault> {
        self.flush_page().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        self.input.seal().map_err(|error| plugin_sdk_fault(error.detail))?;
        Ok(self.input)
    }

    pub fn finish_admitted_prefix(mut self) -> Result<semio_framework::action_bus::RetainedToolWireInput, Fault> {
        self.flush_page().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        self.input.seal_admitted_prefix().map_err(|error| plugin_sdk_fault(error.detail))?;
        Ok(self.input)
    }
}

impl std::io::Write for TypedCommandWireWriter {
    fn write(&mut self, mut bytes: &[u8]) -> std::io::Result<usize> {
        let source_len = bytes.len();
        while !bytes.is_empty() {
            if self.len == self.page.len() {
                self.flush_page()?;
            }
            let count = bytes.len().min(self.page.len() - self.len);
            self.page[self.len..self.len + count].copy_from_slice(&bytes[..count]);
            self.len += count;
            bytes = &bytes[count..];
        }
        Ok(source_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush_page()
    }
}
