//! 📥️ Latest-wins command admission and the typed command wire writer.
use semio_framework::Fault;
use semio_framework_job::{OriginalConstructionAdmission,OriginalTurnFailure,StepContext};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError};
use semio_framework_value::retained_clone::{NativeAllocationIdentity,RetainedCloneClose,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_async::PublicationClaimHandle;
use std::mem::{ManuallyDrop,size_of};
use semio_plugin_ownership::bounded_ownership::plugin_grant_funds;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_plugin_ownership::bounded_ownership::{PluginLifecycleStep, PluginLifecycleFailure,plugin_pay_unit,plugin_settle_poll,plugin_settle_value,plugin_unit_step};
use semio_plugin_job::reserved_jobs::QualifiedToolProof;
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_operation::ToolCancellationLease;
use semio_plugin_job::TYPED_OPERATION_RESULT_PAGE_BYTES;
pub(crate) use semio_plugin_job::original_steps::{adopted,begun,built,built_with,construction,fixed,funded,receipt,retired,retired_poll,unbuilt};
/// 🪙️ A pure retirement state change pays one original work unit on the turn's own effect.
pub(crate) fn paid(effect:&mut RetainedAllocationEffect<'_>,step:impl FnOnce()->RetainedCloneStep)->Result<RetainedCloneStep,ValueError>{plugin_pay_unit(effect)?;Ok(step())}
/// 🧭️ One effect-shaped forward step settles its performed work as a complete poll and posts its receipt; `false` means the turn denied it.
pub(crate) fn forwarded(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<bool,OriginalTurnFailure>{
    funded(cx,demand,|effect|{let result=run(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()})}).map(|step|step.is_some())
}
type Shared<T>=semio_framework_value_ordered::ordered::SharedOwner<T>;
/// 🧱️ One bounded state change settles through the owner's resident admission, taken out for the call so the change may borrow the whole owner.
pub(crate) fn stepped<S>(cx:&mut StepContext<'_>,owner:&mut S,resident:fn(&mut S)->&mut OriginalConstructionAdmission,demand:RetirementDemand,apply:impl FnOnce(&mut S)->Result<RetainedCloneProgress,ValueError>)->Result<bool,OriginalTurnFailure>{
    let mut admission=std::mem::take(resident(owner));let mut done=None;
    let settled=built(cx,&mut admission,&mut done,construction(demand),||apply(owner).map(|progress|((),progress)));
    *resident(owner)=admission;settled
}

#[doc(hidden)]
#[path="../🗝️latest/🦀️.rs"]
pub mod original_latest_key;

#[path="../🎟️typed/🦀️.rs"]
pub mod original_typed_command;

use crate::command_admission::original_latest_key::ToolLatestWinsKeyCopy;

#[derive(semio_framework_value::RetireOwned)]
pub struct ToolLatestWinsScope {
    pub key: Shared<String>,
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

fn refused(error:&ValueError)->Fault{semio_plugin_decl_emission::retirement_refusal_fault(error)}
fn worked(effect:&mut RetainedAllocationEffect<'_>,work:u128)->Result<(),Fault>{RetirementPollExit::pay(effect,work).map_err(|_|plugin_sdk_fault("latest-wins work exceeds its original grant"))}
fn unit(effect:&mut RetainedAllocationEffect<'_>)->Result<(),Fault>{plugin_pay_unit(effect).map_err(|error|refused(&error))}
fn progressed(turn:RetainedWorkOutcome<Option<RetainedCloneStep>>)->Result<RetainedCloneStep,Fault>{turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(|error|refused(&error))}
fn polled(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,Fault>{plugin_settle_value(poll).map_err(|error|refused(&error))}

#[doc(hidden)]
pub struct ToolLatestWinsRegistry {
    pub map: Option<semio_framework_value_ordered::ordered::OrderedMap<ToolLatestWinsScope>>,
    pub update: Option<semio_framework_value_ordered::ordered::UpdateCursor<ToolLatestWinsScope>>,
    pub retirement: Option<semio_framework_value_ordered::ordered::Retirement<ToolLatestWinsScope>>,
    removed_scope: Option<Shared<ToolLatestWinsScope>>,
    retired_scope:ManuallyDrop<Option<ToolLatestWinsScope>>,
    scope_close:RetainedCloneClose,
    pub active_operation: Option<u64>,
    pub outcome: Option<(u64, bool)>,
    pub scan: usize,
    admission_scan: Option<usize>,
    update_closing: bool,
    pub closing: bool,
}

impl ToolLatestWinsRegistry {
    pub fn new() -> Self {
        Self { map: Some(semio_framework_value_ordered::ordered::OrderedMap::new()), update: None, retirement: None, removed_scope: None, retired_scope:ManuallyDrop::new(None),scope_close:Default::default(),active_operation: None, outcome: None, scan: 0, admission_scan: None, update_closing: false, closing: false }
    }

    /// 🚦️ A new operation may start only while the whole frontier of the previous update has returned its owners.
    pub fn can_begin(&self) -> bool {
        !self.closing && self.update.is_none() && self.retirement.is_none() && self.removed_scope.is_none() && self.retired_scope.is_none()&&self.scope_close.is_empty()&&self.outcome.is_none()
    }

    pub fn begin(&mut self,operation:u64,key:&Shared<String>,lease:&ToolCancellationLease,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,Fault>{
        let grant=effect.grant();let admission=Shared::<ToolLatestWinsScope>::source_admission_demand().map_err(|error|refused(&error))?;
        if !self.can_begin()||!retained_clone_grant_admits(grant,admission)||effect.remaining_work_units().map_err(|error|refused(&error))?<Shared::<ToolLatestWinsScope>::source_admission_work_demand(){return Ok(None);}
        if self.map.as_ref().expect("live original key map").len()>=ARTIFACT_LIVE_OUTPUT_SLOTS{match self.admission_scan{None=>{self.admission_scan=Some(ARTIFACT_LIVE_OUTPUT_SLOTS);unit(effect)?;return Ok(Some(RetainedCloneProgress{copied_items:1,..Default::default()}));},Some(0)=>{},Some(_)=>return Ok(None)}}
        let mut source=Some(ToolLatestWinsScope{key:key.clone(),operation,token:lease.cancel_token().clone(),claim:lease.publication_claim().clone()});let mut admitted=None;
        let progress=match Shared::admit_from(&mut source,&mut admitted,effect).result{Ok(step)=>step.progress(),Err(error)=>{*self.retired_scope=source;return Err(refused(&error));}};
        let Some(scope)=admitted else{*self.retired_scope=source;return Ok(Some(progress));};
        self.update=Some(self.map.as_ref().expect("original live key map").begin_set_shared(key.clone(),scope));self.active_operation=Some(operation);self.update_closing=false;Ok(Some(progress))
    }

    pub fn take_outcome(&mut self, operation: u64) -> Option<bool> {
        if self.outcome.is_some_and(|(owner, _)| owner == operation) {
            self.outcome.take().map(|(_, accepted)| accepted)
        } else {
            None
        }
    }

    /// 🛑️ A canceled operation loses its pending insertion: its update and every unadmitted owner retire through paid turns.
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
        Ok(match step{RetirementStep::Blocked=>RetainedCloneStep::Progress(Default::default()),RetirementStep::Progress{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}),RetirementStep::ProcessedBytes(copied_bytes)=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}),RetirementStep::OwnedValue(scope)=>{self.retire_scope(scope);RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})},RetirementStep::Failure(error)=>return Err(refused(&error)),RetirementStep::Complete=>RetainedCloneStep::Progress(Default::default())})
    }
    /// 📏️ Quotes the exact next [`Self::advance`] turn, one original owner at a time in the same order.
    pub fn advance_demands(&self)->Result<RetirementDemand,ValueError>{
        let item=RetirementDemand{depth:1,..Default::default()};
        let body=TYPED_OPERATION_RESULT_PAGE_BYTES;
        if !self.scope_close.is_empty(){return self.scope_close.demands(body);}
        if self.retired_scope.is_some(){return self.scope_close.next_owner_demands::<ToolLatestWinsScope>(true,&None,body);}
        if self.removed_scope.is_some(){return self.scope_close.next_owner_demands::<Shared<ToolLatestWinsScope>>(true,&None,body);}
        if let Some(retirement)=self.retirement.as_ref(){return Ok(if retirement.terminal_is_empty(){item}else{RetirementDemand{copy_bytes:retirement.next_copy_byte_demand(),release_bytes:retirement.next_close_byte_demand()?,depth:retirement.next_depth_demand().max(1),..Default::default()}});}
        if let Some(update)=self.update.as_ref(){
            if self.update_closing{return Ok(if update.terminal_is_empty(){item}else{RetirementDemand{copy_bytes:update.next_close_copy_byte_demand(),release_bytes:update.next_close_byte_demand()?,depth:update.next_close_depth_demand().max(1),..Default::default()}});}
            if !update.is_complete(){return Ok(RetirementDemand{copy_bytes:update.next_copy_byte_demand(),capacity_bytes:update.next_capacity_byte_demand()?,depth:update.next_depth_demand().max(1),..Default::default()});}
            return Ok(item);
        }
        Ok(if self.map.as_ref().is_some_and(|map|self.closing||!map.is_empty()){item}else{Default::default()})
    }
    /// ⏩️ Performs exactly one funded unit of the registry frontier; an unfunded unit leaves every owner in place.
    pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,Fault>{
        let grant=effect.grant();let idle=RetainedCloneProgress::default();let item=RetainedCloneProgress{copied_items:1,..idle};let body=TYPED_OPERATION_RESULT_PAGE_BYTES;
        let demand=self.advance_demands().map_err(|error|refused(&error))?;
        if !plugin_grant_funds(demand,grant){return Ok(RetainedCloneStep::Progress(idle));}
        if !self.scope_close.is_empty(){return Ok(RetainedCloneStep::Progress(polled(self.scope_close.step_granted(body,effect))?.progress()));}
        if self.retired_scope.is_some(){return Ok(RetainedCloneStep::Progress(progressed(self.scope_close.begin_granted(&mut self.retired_scope,effect))?.progress()));}
        if self.removed_scope.is_some(){return Ok(RetainedCloneStep::Progress(progressed(self.scope_close.begin_granted(&mut self.removed_scope,effect))?.progress()));}
        if let Some(retirement)=self.retirement.as_mut(){if retirement.terminal_is_empty(){self.retirement.take();unit(effect)?;return Ok(RetainedCloneStep::Progress(item));}let step=retirement.advance(grant);worked(effect,u128::from(retirement.last_turn_work()))?;return self.retirement_step(step);}
        if let Some(update)=self.update.as_mut(){
            let maximum=effect.remaining_work_units().map_err(|error|refused(&error))?;
            if self.update_closing{if update.terminal_is_empty(){self.update.take();self.active_operation=None;self.update_closing=false;unit(effect)?;return Ok(RetainedCloneStep::Progress(item));}let step=update.close_step(grant,maximum);worked(effect,u128::from(update.last_close_work()))?;return self.retirement_step(step);}
            if !update.is_complete(){let turn=update.advance(grant,maximum);worked(effect,turn.work)?;let step=turn.result.map_err(|error|refused(&error))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
            let candidate=update.take_result().expect("complete original key candidate");self.admission_scan=None;
            if candidate.len()>ARTIFACT_LIVE_OUTPUT_SLOTS{self.retirement=Some(candidate.retire());if let Some(operation)=self.active_operation{self.outcome=Some((operation,false));}}
            else{let previous=self.map.replace(candidate).expect("original live key map");self.retirement=Some(previous.retire());if let Some(scope)=update.take_removed(){if self.active_operation.is_some_and(|operation|operation!=scope.operation){scope.cancel();}self.removed_scope=Some(scope);}if let Some(operation)=self.active_operation{self.outcome=Some((operation,true));}}
            update.begin_close();self.update_closing=true;unit(effect)?;return Ok(RetainedCloneStep::Progress(item));
        }
        if self.closing{if let Some(map)=self.map.take(){self.retirement=Some(map.retire());unit(effect)?;return Ok(RetainedCloneStep::Progress(item));}return Ok(RetainedCloneStep::Complete(idle));}
        let map=self.map.as_ref().expect("original live key map");if map.is_empty(){return Ok(RetainedCloneStep::Progress(idle));}self.scan%=map.len();let(_,scope)=map.entry_at_rank(self.scan).expect("bounded original map rank");self.scan+=1;if let Some(remaining)=self.admission_scan.as_mut(){*remaining=remaining.saturating_sub(1);}if scope.claim.is_finished(){self.update=Some(map.begin_remove_shared(scope.key.clone()));self.update_closing=false;}unit(effect)?;Ok(RetainedCloneStep::Progress(item))
    }
    /// 🔒️ Stops admitting new operations; every pending update and unadmitted owner retires through `advance`.
    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(update) = self.update.as_mut() {
            update.begin_close();
            self.update_closing = true;
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.map.is_none() && self.update.is_none() && self.retirement.is_none() && self.removed_scope.is_none()&&self.retired_scope.is_none()&&self.scope_close.is_empty()
    }
}

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
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
    pub command_retirement:RetainedCloneClose,
    pub admission: Option<AdmittedToolCommand>,
    pub meta: ManuallyDrop<Option<ActionMeta>>,
    pub original_caller:ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub original_body:usize,
    pub original_decoder:(NativeAllocationIdentity,u64),
    pub metadata_retirement:RetainedCloneClose,
    pub context_metadata:ManuallyDrop<Option<ActionMeta>>,
    pub context_view:ManuallyDrop<Option<semio_plugin_decl_emission::ViewLease>>,
    pub context_phase:u8,
    pub operation: semio_framework_job::Operation,
    pub source_identity:semio_framework_job::OriginalSourceBinding,
    pub revision: [u8; 32],
    pub lease: Option<ToolCancellationLease>,
    pub cancellation_retirement:Option<Box<semio_plugin_operation::ToolCancellationLeaseRetirement>>,
    pub key_copy: Option<ToolLatestWinsKeyCopy>,
    pub key: Option<Shared<String>>,
    pub construction:OriginalConstructionAdmission,
    pub lookup_started: bool,
    pub accepted: bool,
    pub restarting: bool,
    pub closing: bool,
    pub receipt:Option<(semio_framework_value::RetainedCloneGrant,RetainedCloneProgress)>,
    pub fault:ManuallyDrop<Option<PluginLifecycleFailure>>,
    pub fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub children:ManuallyDrop<Option<semio_framework_value::retirement::arc_custody::original::SharedOriginalStorage<semio_plugin_decl_views::child_view::ChildContentView>>>,
    pub children_generation:u64,
    pub children_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
}

impl<A: ArtifactApp> PendingLatestWinsCommand<A> {
    pub fn metadata(&self)->Result<&ActionMeta,ValueError>{self.meta.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending command lost its original metadata"))}
    pub fn validate_original(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.source_identity.matches(cx)||cx.native_receiving_identity()!=Some(self.original_decoder){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending command requires its original receiving continuation").into())}if let Some(caller)=self.original_caller.as_ref(){caller.validate_retirement_context(cx)?}Ok(())}
    /// 🪪️ Routing metadata and its view lease are prepared in four funded phases before the command may dispatch.
    pub fn prepare_context_metadata(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate_original(cx)?;
        match self.context_phase{
            0=>{
                if self.context_metadata.is_none(){self.meta.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending command lost its original metadata"))?.admit_routing_alias(&mut self.construction,&mut self.context_metadata,cx)?;return Ok(false)}
                stepped(cx,self,|owner|&mut owner.construction,fixed::<u8>(),|owner|{owner.context_phase=1;Ok(receipt(fixed::<u8>()))})?;
            },
            1=>{let d=RetirementDemand{copy_bytes:2*size_of::<Option<semio_plugin_decl_emission::ViewLease>>()+size_of::<u8>(),depth:1,..Default::default()};stepped(cx,self,|owner|&mut owner.construction,d,|owner|{*owner.context_view=owner.context_metadata.as_mut().unwrap().view_state.take().map(semio_plugin_decl_emission::ViewLease::new);owner.context_phase=2;Ok(receipt(d))})?;},
            2=>{let body=self.original_body;if !self.metadata_retirement.is_empty(){let d=self.metadata_retirement.demands(body)?;retired_poll(cx,d,|effect|self.metadata_retirement.step_granted(body,effect))?;}else if self.context_metadata.is_some(){let d=self.metadata_retirement.next_owner_demands::<ActionMeta>(true,&None,body)?;begun(cx,d,|effect|self.metadata_retirement.begin_granted(&mut self.context_metadata,effect))?;}else{stepped(cx,self,|owner|&mut owner.construction,fixed::<u8>(),|owner|{owner.context_phase=3;Ok(receipt(fixed::<u8>()))})?;}},
            3=>return Ok(true),
            _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original command metadata phase changed").into())
        }
        Ok(self.context_phase==3)
    }
    pub fn context_metadata_is_empty(&self)->bool{self.context_metadata.is_none()&&self.context_view.is_none()&&self.metadata_retirement.is_empty()}
    pub fn context_metadata_close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.metadata_retirement.is_empty(){return self.metadata_retirement.demands(self.original_body)}if self.context_metadata.is_some(){return self.metadata_retirement.next_owner_demands::<ActionMeta>(true,&None,self.original_body)}if self.context_view.is_some(){return self.metadata_retirement.next_owner_demands::<semio_plugin_decl_emission::ViewLease>(true,&None,self.original_body)}Ok(Default::default())}
    pub fn close_context_metadata(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate_original(cx)?;if self.context_metadata_is_empty(){return Ok(true)}
        let d=self.context_metadata_close_demands()?;let body=self.original_body;
        if !self.metadata_retirement.is_empty(){retired_poll(cx,d,|effect|self.metadata_retirement.step_granted(body,effect))?;}
        else if self.context_metadata.is_some(){begun(cx,d,|effect|self.metadata_retirement.begin_granted(&mut self.context_metadata,effect))?;}
        else{begun(cx,d,|effect|self.metadata_retirement.begin_granted(&mut self.context_view,effect))?;}
        Ok(false)
    }
    pub fn metadata_close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.context_metadata_is_empty(){return self.context_metadata_close_demands()}if !self.metadata_retirement.is_empty(){return self.metadata_retirement.demands(self.original_body)}if self.meta.is_some(){return self.metadata_retirement.next_owner_demands::<ActionMeta>(true,&None,self.original_body)}if let Some(caller)=self.original_caller.as_ref(){return Ok(caller.retirement_demands())}Ok(Default::default())}
    pub fn close_metadata(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate_original(cx)?;if !self.context_metadata_is_empty(){self.close_context_metadata(cx)?;return Ok(false)}
        if self.meta.is_none()&&self.metadata_retirement.is_empty(){if let Some(caller)=self.original_caller.as_mut(){caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){self.original_caller.take();}return Ok(false)}return unbuilt(cx,&mut self.construction)}
        let d=self.metadata_close_demands()?;let body=self.original_body;
        if self.meta.is_some(){begun(cx,d,|effect|self.metadata_retirement.begin_granted(&mut self.meta,effect))?;}else{retired_poll(cx,d,|effect|self.metadata_retirement.step_granted(body,effect))?;}
        Ok(false)
    }
    pub fn metadata_is_empty(&self)->bool{self.context_metadata_is_empty()&&self.meta.is_none()&&self.metadata_retirement.is_empty()&&self.original_caller.is_none()&&self.construction.terminal_is_empty()}

    pub fn admit_original_lease(&mut self,issuer:&mut semio_plugin_operation::ToolCancellationIssuer,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.closing||self.lease.is_some(){return Ok(self.lease.is_some())}
        if !self.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted pending lease requires its original command caller").into())}
        let key=semio_plugin_operation::ToolOperationKey{app_instance_id:self.metadata()?.instance_id,document:semio_plugin_operation::ArtifactDocumentAuthority(self.metadata()?.instance_id),operation_id:self.operation.operation,base_revision:self.operation.base_revision,generation:self.operation.generation};
        self.lease=issuer.begin_keyed(&key,cx)?;
        Ok(self.lease.is_some())
    }

    pub fn record_original_receipt(&mut self,grant:semio_framework_value::RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),Fault>{
        if progress!=RetainedCloneProgress::default(){if self.receipt.is_some(){return Err(plugin_sdk_fault("latest-wins original receipt awaits its caller handback").with_retained_progress(progress));}self.receipt=Some((grant,progress));}
        if !progress.fits(grant){return Err(plugin_sdk_fault("latest-wins original receipt exceeded its caller").with_retained_progress(progress));}
        Ok(())
    }
    fn key_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.key_copy.as_ref().is_some_and(|copy|copy.key.is_none()){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if !self.command_retirement.is_empty(){return self.command_retirement.demands(self.original_body)}
        if self.key_copy.is_some(){return self.command_retirement.next_owner_demands::<String>(true,&None,self.original_body)}
        if self.key.is_some(){return self.command_retirement.next_owner_demands::<Shared<String>>(true,&None,self.original_body)}
        Ok(Default::default())
    }
    /// 📏️ Quotes the next close turn of the command, key, child or fault owner this pending command still holds.
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.fault_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        if let Some(owner)=self.children_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        if self.children.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<semio_plugin_decl_views::child_view::ChildContentView>(),depth:1,..Default::default()})}
        if self.fault.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<PluginLifecycleFailure>(),depth:1,..Default::default()})}
        if !self.command_retirement.is_empty(){return self.command_retirement.demands(self.original_body)}
        if self.command.is_some(){return self.command_retirement.next_owner_demands::<Box<A::Command>>(true,&None,self.original_body)}
        if self.key_copy.is_some()||self.key.is_some(){return self.key_demands();}
        if self.admission.is_some(){return self.command_retirement.next_owner_demands::<AdmittedToolCommand>(true,&None,self.original_body)}
        Ok(Default::default())
    }
    pub fn permits(grant:semio_framework_value::RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        let grant=effect.grant();let idle=RetainedCloneProgress::default();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;let body=self.original_body;
        if !Self::permits(grant,demand){return Ok(PluginLifecycleStep::Progress(idle))}
        if let Some(owner)=self.fault_retirement.as_mut(){let step=plugin_settle_poll(owner.close_step(grant.maximum_copy_bytes,effect))?;if owner.terminal_is_empty(){self.fault_retirement.take();}return Ok(PluginLifecycleStep::Progress(step.progress()))}
        if self.fault.is_some(){return Ok(PluginLifecycleStep::Progress(match semio_framework_value::retirement::admit_owned_retirement(&mut self.fault,effect).result?{Some((owner,progress))=>{self.fault_retirement=Some(owner);progress},None=>idle}))}
        if let Some(owner)=self.children_retirement.as_mut(){let step=plugin_settle_poll(owner.close_step(grant.maximum_copy_bytes,effect))?;if owner.terminal_is_empty(){self.children_retirement.take();}return Ok(PluginLifecycleStep::Progress(step.progress()))}
        if self.children.is_some(){let(original,retirement)=(&mut *self.children,&mut self.children_retirement);let step=plugin_settle_value(semio_plugin_job::original_steps::replied::<semio_framework_value::OriginalRetirementFailure>(effect,|effect,reply|semio_framework_value::retirement::arc_custody::admit_shared_retirement_into(original,retirement,effect,reply).map(drop)))?;return Ok(PluginLifecycleStep::Progress(step.progress()))}
        if !self.command_retirement.is_empty(){return Ok(PluginLifecycleStep::Progress(plugin_settle_poll(self.command_retirement.step_granted(body,effect))?.progress()))}
        if self.command.is_some(){return Ok(PluginLifecycleStep::Progress(self.command_retirement.begin_granted(&mut self.command,effect).result?.unwrap_or(RetainedCloneStep::Progress(idle)).progress()))}
        if self.key_copy.is_some()||self.key.is_some(){return self.close_key_step(effect)}
        if self.admission.is_some(){return Ok(PluginLifecycleStep::Progress(self.command_retirement.begin_granted(&mut self.admission,effect).result?.unwrap_or(RetainedCloneStep::Progress(idle)).progress()))}
        Ok(PluginLifecycleStep::AwaitingInput{reason:"latest-wins original metadata and cancellation await their native retirement issuer"})
    }
    pub fn close_key_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        let grant=effect.grant();let idle=RetainedCloneProgress::default();if !Self::permits(grant,self.key_demands()?){return Ok(PluginLifecycleStep::Progress(idle))}
        if self.key_copy.as_ref().is_some_and(|copy|copy.key.is_none()){plugin_pay_unit(effect)?;self.key_copy.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}))}
        if !self.command_retirement.is_empty(){return Ok(PluginLifecycleStep::Progress(plugin_settle_poll(self.command_retirement.step_granted(self.original_body,effect))?.progress()))}
        if let Some(copy)=self.key_copy.as_mut(){return Ok(PluginLifecycleStep::Progress(self.command_retirement.begin_granted(&mut copy.key,effect).result?.unwrap_or(RetainedCloneStep::Progress(idle)).progress()))}
        if self.key.is_some(){return Ok(PluginLifecycleStep::Progress(self.command_retirement.begin_granted(&mut self.key,effect).result?.unwrap_or(RetainedCloneStep::Progress(idle)).progress()))}
        Ok(PluginLifecycleStep::Complete(idle))
    }
    pub fn command_owners_are_empty(&self)->bool{self.command.is_none()&&self.command_retirement.is_empty()&&self.admission.is_none()&&self.key_copy.is_none()&&self.key.is_none()&&self.fault.is_none()&&self.fault_retirement.is_none()&&self.children.is_none()&&self.children_retirement.is_none()}
}

impl<A: ArtifactApp> Drop for PendingLatestWinsCommand<A> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(std::thread::panicking() || self.command_owners_are_empty()&&self.metadata_is_empty(), "latest-wins admission must transfer or cursor-retire its exact command/key/raw owners");
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
