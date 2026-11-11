//! 📥️ Latest-wins command admission and the typed command wire writer.
use semio_framework::Fault;
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{NativeAllocationIdentity,RetainedCloneClose};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_ordered::ordered::{OrderedMap,SharedOwner,UpdateCursor};
use semio_framework_async::PublicationClaimHandle;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_emission::{ActionMeta,ViewLease};
use semio_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS,PluginLifecycleFailure,plugin_typed_owner_demand,plugin_typed_owner_original};
use semio_plugin_job::original_steps::{acted,begin,fixed,forward,retired,settle,produced};
use semio_plugin_job::reserved_jobs::QualifiedToolProof;
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_operation::ToolCancellationLease;
use std::mem::{ManuallyDrop,size_of};

#[doc(hidden)]
#[path="../🗝️latest/🦀️.rs"]
pub mod original_latest_key;

#[path="../🎟️typed/🦀️.rs"]
pub mod original_typed_command;

use crate::command_admission::original_latest_key::ToolLatestWinsKeyCopy;

fn invalid(detail:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,detail)}
fn duplicated(key:&SharedOwner<String>,output:&mut Option<SharedOwner<String>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
    let result=(||{
        if effect.remaining_work_units()?<SharedOwner::<String>::DUPLICATE_WORK{return Ok(RetainedCloneStep::Progress(Default::default()))}
        key.duplicate_into(output,effect).result
    })();
    settle(effect,result)
}

/// 🎫️ One live latest-wins entry: the operation, its cancellation handles and the key alias that removes it.
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolLatestWinsScope {
    pub key: SharedOwner<String>,
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

/// 🪆️ One resident owner custody: the original waits in `value` until its controlled retirement frame is born in `owner`.
struct Slot<T:RetireOwned>{value:Option<T>,owner:Option<ControlledRetirement<T>>}
impl<T:RetireOwned> Default for Slot<T>{fn default()->Self{Self{value:None,owner:None}}}
impl<T:RetireOwned> Slot<T>{
    fn is_empty(&self)->bool{self.value.is_none()&&self.owner.is_none()}
    fn retire(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let d=plugin_typed_owner_demand(&self.value,&self.owner,cx.retained_grant().maximum_copy_bytes)?;
        retired(cx,d,|effect|plugin_typed_owner_original(&mut self.value,&mut self.owner,effect))?;
        Ok(())
    }
}

/// 🗝️ The bounded latest-wins registry: every turn performs exactly one funded unit against the ordered live-scope map.
#[doc(hidden)]
pub struct ToolLatestWinsRegistry {
    map: Option<OrderedMap<ToolLatestWinsScope>>,
    update: Slot<UpdateCursor<ToolLatestWinsScope>>,
    candidate: Option<OrderedMap<ToolLatestWinsScope>>,
    previous: Slot<OrderedMap<ToolLatestWinsScope>>,
    removed: Slot<SharedOwner<ToolLatestWinsScope>>,
    alias: Slot<SharedOwner<String>>,
    source: Slot<ToolLatestWinsScope>,
    shared: Slot<SharedOwner<ToolLatestWinsScope>>,
    pub active_operation: Option<u64>,
    pub outcome: Option<(u64, bool)>,
    pub scan: usize,
    admission_scan: Option<usize>,
    phase: u8,
    removal: Option<usize>,
    taken: bool,
    update_closing: bool,
    pub closing: bool,
}

impl ToolLatestWinsRegistry {
    pub fn new() -> Self {
        Self { map: Some(OrderedMap::new()), update: Slot::default(), candidate: None, previous: Slot::default(), removed: Slot::default(), alias: Slot::default(), source: Slot::default(), shared: Slot::default(), active_operation: None, outcome: None, scan: 0, admission_scan: None, phase: 0, removal: None, taken: false, update_closing: false, closing: false }
    }

    fn idle(&self)->bool{
        self.phase==0&&!self.update_closing&&self.candidate.is_none()&&self.update.is_empty()&&self.previous.is_empty()&&self.removed.is_empty()&&self.alias.is_empty()&&self.source.is_empty()&&self.shared.is_empty()&&self.active_operation.is_none()
    }

    fn abandoning(&self)->bool{self.closing||self.update_closing}

    /// 🚦️ A new operation may start only while the whole frontier of the previous update has returned its owners.
    pub fn can_begin(&self) -> bool {
        !self.closing&&self.outcome.is_none()&&self.idle()
    }

    fn live(&self)->Result<&OrderedMap<ToolLatestWinsScope>,ValueError>{self.map.as_ref().ok_or_else(||invalid("latest-wins registry lost its live map"))}

    fn alias_key(&mut self,key:&SharedOwner<String>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let d=SharedOwner::<String>::duplicate_demand();
        forward(cx,d,|effect|duplicated(key,&mut self.alias.value,effect))?;
        Ok(())
    }

    fn admit_update(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let Self{map,alias,shared,update,..}=self;
        let map=map.as_ref().ok_or_else(||invalid("latest-wins registry lost its live map"))?;
        let d=map.update_constructor_demand();
        forward(cx,d,|effect|{let turn=map.admit_update_cursor(&mut alias.value,&mut shared.value,&mut update.value,effect);settle(effect,turn.result)})?;
        if self.update.value.is_some(){self.phase=0;self.removal=None;}
        Ok(())
    }

    /// 🌱️ Starts or continues the insertion of one operation; `true` once its update cursor is admitted. While another frontier is busy this turn advances that frontier instead.
    pub fn begin(&mut self,operation:u64,key:&SharedOwner<String>,lease:&ToolCancellationLease,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let beginning=self.active_operation==Some(operation)&&(1..=4).contains(&self.phase);
        if self.active_operation==Some(operation)&&self.phase==0&&self.update.value.is_some(){return Ok(true);}
        if beginning&&self.abandoning()||!beginning&&!self.can_begin(){self.advance(cx)?;return Ok(false);}
        match self.phase{
            0=>{
                if self.live()?.len()>=ARTIFACT_LIVE_OUTPUT_SLOTS{match self.admission_scan{
                    None=>{acted(cx,fixed::<Option<usize>>(),||{self.admission_scan=Some(ARTIFACT_LIVE_OUTPUT_SLOTS);Ok(())})?;return Ok(false);},
                    Some(0)=>{},
                    Some(_)=>{self.advance(cx)?;return Ok(false);},
                }}
                self.alias_key(key,cx)?;
                if self.alias.value.is_some(){self.phase=1;self.active_operation=Some(operation);}
            },
            1=>{
                let d=RetirementDemand{copy_bytes:2*size_of::<ToolLatestWinsScope>()+size_of::<Option<SharedOwner<String>>>(),depth:1,..Default::default()};
                let Self{alias,source,..}=self;
                produced(cx,d,&mut source.value,||Ok(ToolLatestWinsScope{key:alias.value.take().ok_or_else(||invalid("latest-wins scope lost its key alias"))?,operation,token:lease.cancel_token().clone(),claim:lease.publication_claim().clone()}))?;
                if self.source.value.is_some(){self.phase=2;}
            },
            2=>{
                let d=SharedOwner::<ToolLatestWinsScope>::source_admission_demand()?;
                let Self{source,shared,..}=self;
                forward(cx,d,|effect|{let turn=SharedOwner::admit_from(&mut source.value,&mut shared.value,effect);settle(effect,turn.result)})?;
                if self.shared.value.is_some(){self.phase=3;}
            },
            3=>{
                self.alias_key(key,cx)?;
                if self.alias.value.is_some(){self.phase=4;}
            },
            _=>{
                self.admit_update(cx)?;
                return Ok(self.update.value.is_some());
            },
        }
        Ok(false)
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
            self.update_closing = true;
            self.outcome = Some((operation, false));
        }
    }

    fn settle_update(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let d=RetirementDemand{copy_bytes:3*size_of::<Option<OrderedMap<ToolLatestWinsScope>>>()+size_of::<Option<SharedOwner<ToolLatestWinsScope>>>()+size_of::<Option<(u64,bool)>>(),depth:1,..Default::default()};
        acted(cx,d,||{
            let candidate=self.candidate.take().ok_or_else(||invalid("complete original key candidate is absent"))?;
            self.admission_scan=None;self.taken=false;
            let operation=self.active_operation;
            if candidate.len()>ARTIFACT_LIVE_OUTPUT_SLOTS{
                self.previous.value=Some(candidate);
                if let Some(operation)=operation{self.outcome=Some((operation,false));}
            }else{
                let previous=self.map.replace(candidate).ok_or_else(||invalid("latest-wins registry lost its live map"))?;
                self.previous.value=Some(previous);
                if let Some(scope)=self.update.value.as_mut().and_then(UpdateCursor::take_removed){
                    if operation.is_some_and(|operation|operation!=scope.operation){scope.cancel();}
                    self.removed.value=Some(scope);
                }
                if let Some(operation)=operation{self.outcome=Some((operation,true));}
            }
            self.update_closing=true;
            Ok(())
        })?;
        Ok(())
    }

    fn advance_update(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let body=cx.retained_grant().maximum_copy_bytes;
        let Self{update,candidate,taken,..}=self;
        let cursor=update.value.as_mut().ok_or_else(||invalid("latest-wins registry lost its update cursor"))?;
        if !cursor.is_complete(){
            let d=cursor.native_demands(body)?;
            forward(cx,d,|effect|{let turn=cursor.advance_native(body,effect);settle(effect,turn.result)})?;
        }else if !*taken{
            let d=cursor.result_transfer_demand();
            forward(cx,d,|effect|{let turn=cursor.take_result_into(candidate,effect);if matches!(turn.result,Ok(RetainedCloneStep::Complete(_))){*taken=true;}settle(effect,turn.result)})?;
        }else{
            return self.settle_update(cx);
        }
        Ok(())
    }

    fn scan_unit(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        if self.phase==8{
            let Self{map,removal,alias,..}=self;
            let map=map.as_ref().ok_or_else(||invalid("latest-wins registry lost its live map"))?;
            let rank=removal.ok_or_else(||invalid("latest-wins removal lost its scanned rank"))?;
            let(_,scope)=map.entry_at_rank(rank).ok_or_else(||invalid("latest-wins removal lost its scanned entry"))?;
            let d=SharedOwner::<String>::duplicate_demand();
            forward(cx,d,|effect|duplicated(&scope.key,&mut alias.value,effect))?;
            if self.alias.value.is_some(){self.phase=9;}
            return Ok(());
        }
        if self.phase==9{return self.admit_update(cx);}
        let Self{map,scan,admission_scan,removal,phase,..}=self;
        let map=map.as_ref().ok_or_else(||invalid("latest-wins registry lost its live map"))?;
        if map.is_empty(){return Ok(());}
        acted(cx,fixed::<(usize,usize,u8)>(),||{
            *scan%=map.len();
            let(_,scope)=map.entry_at_rank(*scan).ok_or_else(||invalid("latest-wins scan lost its ranked entry"))?;
            let rank=*scan;*scan+=1;
            if let Some(remaining)=admission_scan.as_mut(){*remaining=remaining.saturating_sub(1);}
            if scope.claim.is_finished(){*removal=Some(rank);*phase=8;}
            Ok(())
        })?;
        Ok(())
    }

    /// ⏩️ Performs exactly one funded unit of the registry frontier; an unfunded unit leaves every owner in place.
    pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        if !self.removed.is_empty(){return self.removed.retire(cx);}
        if self.abandoning()&&self.phase!=0&&self.update.value.is_none(){
            if !self.alias.is_empty(){return self.alias.retire(cx);}
            if !self.source.is_empty(){return self.source.retire(cx);}
            if !self.shared.is_empty(){return self.shared.retire(cx);}
            acted(cx,fixed::<(u8,Option<usize>)>(),||{self.phase=0;self.removal=None;Ok(())})?;
            return Ok(());
        }
        if !self.previous.is_empty(){return self.previous.retire(cx);}
        if self.update_closing{
            if self.candidate.is_some(){acted(cx,fixed::<[Option<OrderedMap<ToolLatestWinsScope>>;2]>(),||{self.previous.value=self.candidate.take();self.taken=false;Ok(())})?;return Ok(());}
            if !self.update.is_empty(){return self.update.retire(cx);}
            acted(cx,fixed::<(Option<u64>,bool)>(),||{self.active_operation=None;self.update_closing=false;self.taken=false;Ok(())})?;
            return Ok(());
        }
        if self.update.value.is_some(){return self.advance_update(cx);}
        if (1..=4).contains(&self.phase){return Ok(());}
        if self.closing{
            if self.map.is_some(){acted(cx,fixed::<[Option<OrderedMap<ToolLatestWinsScope>>;2]>(),||{self.previous.value=self.map.take();Ok(())})?;}
            return Ok(());
        }
        self.scan_unit(cx)
    }

    /// 🔒️ Stops admitting new operations; every pending update and unadmitted owner retires through `advance`.
    pub fn begin_close(&mut self) {
        self.closing = true;
        if self.update.value.is_some()||self.phase!=0{self.update_closing=true;}
    }

    /// ♻️ One retirement turn of the whole registry; `true` once every owner returned.
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if !self.closing{self.begin_close();}
        if self.terminal_is_empty(){return Ok(true);}
        self.advance(cx)?;
        Ok(self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing&&self.map.is_none()&&self.idle_owners()
    }

    fn idle_owners(&self)->bool{
        self.phase==0&&self.candidate.is_none()&&self.update.is_empty()&&self.previous.is_empty()&&self.removed.is_empty()&&self.alias.is_empty()&&self.source.is_empty()&&self.shared.is_empty()
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

/// 🎟️ One pending latest-wins command keeps every original owner until the registry accepts or retires it.
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
    pub context_view:ManuallyDrop<Option<ViewLease>>,
    pub context_phase:u8,
    pub operation: semio_framework_job::Operation,
    pub source_identity:semio_framework_job::OriginalSourceBinding,
    pub revision: [u8; 32],
    pub lease: Option<ToolCancellationLease>,
    pub cancellation_retirement:Option<Box<semio_plugin_operation::ToolCancellationLeaseRetirement>>,
    pub key_copy: Option<ToolLatestWinsKeyCopy>,
    pub key: Option<SharedOwner<String>>,
    pub key_retirement: Option<ControlledRetirement<SharedOwner<String>>>,
    pub lookup_started: bool,
    pub accepted: bool,
    pub restarting: bool,
    pub closing: bool,
    pub fault:ManuallyDrop<Option<PluginLifecycleFailure>>,
    pub fault_retirement:Option<ControlledRetirement<PluginLifecycleFailure>>,
}

impl<A: ArtifactApp> PendingLatestWinsCommand<A> {
    pub fn metadata(&self)->Result<&ActionMeta,ValueError>{self.meta.as_ref().ok_or_else(||invalid("pending command lost its original metadata"))}
    pub fn validate_original(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.source_identity.matches(cx)||cx.native_receiving_identity()!=Some(self.original_decoder){return Err(invalid("pending command requires its original receiving continuation").into())}if let Some(caller)=self.original_caller.as_ref(){caller.validate_retirement_context(cx)?}Ok(())}
    /// 🪪️ Routing metadata and its view lease are prepared in four funded phases before the command may dispatch.
    pub fn prepare_context_metadata(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate_original(cx)?;
        match self.context_phase{
            0=>{
                if self.context_metadata.is_none(){self.meta.as_ref().ok_or_else(||invalid("pending command lost its original metadata"))?.admit_routing_alias(&mut self.context_metadata,cx)?;return Ok(false)}
                acted(cx,fixed::<u8>(),||{self.context_phase=1;Ok(())})?;
            },
            1=>{
                let d=RetirementDemand{copy_bytes:2*size_of::<Option<ViewLease>>()+size_of::<u8>(),depth:1,..Default::default()};
                acted(cx,d,||{*self.context_view=self.context_metadata.as_mut().ok_or_else(||invalid("pending command lost its routing metadata"))?.view_state.take().map(ViewLease::new);self.context_phase=2;Ok(())})?;
            },
            2=>{
                let body=self.original_body;
                if !self.metadata_retirement.is_empty(){let d=self.metadata_retirement.demands(body)?;retired(cx,d,|effect|self.metadata_retirement.step_granted(body,effect))?;}
                else if self.context_metadata.is_some(){let d=self.metadata_retirement.next_owner_demands::<ActionMeta>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.metadata_retirement,&mut self.context_metadata,effect))?;}
                else{acted(cx,fixed::<u8>(),||{self.context_phase=3;Ok(())})?;}
            },
            3=>return Ok(true),
            _=>return Err(invalid("original command metadata phase changed").into()),
        }
        Ok(self.context_phase==3)
    }
    pub fn context_metadata_is_empty(&self)->bool{self.context_metadata.is_none()&&self.context_view.is_none()&&self.metadata_retirement.is_empty()}
    pub fn context_metadata_close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.metadata_retirement.is_empty(){return self.metadata_retirement.demands(self.original_body)}if self.context_metadata.is_some(){return self.metadata_retirement.next_owner_demands::<ActionMeta>(true,&None,self.original_body)}if self.context_view.is_some(){return self.metadata_retirement.next_owner_demands::<ViewLease>(true,&None,self.original_body)}Ok(Default::default())}
    pub fn close_context_metadata(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate_original(cx)?;if self.context_metadata_is_empty(){return Ok(true)}
        let d=self.context_metadata_close_demands()?;let body=self.original_body;
        retired(cx,d,|effect|{
            if !self.metadata_retirement.is_empty(){return self.metadata_retirement.step_granted(body,effect)}
            if self.context_metadata.is_some(){return begin(&mut self.metadata_retirement,&mut self.context_metadata,effect)}
            begin(&mut self.metadata_retirement,&mut self.context_view,effect)
        })?;
        Ok(false)
    }
    pub fn metadata_close_demands(&self)->Result<RetirementDemand,ValueError>{if !self.context_metadata_is_empty(){return self.context_metadata_close_demands()}if !self.metadata_retirement.is_empty(){return self.metadata_retirement.demands(self.original_body)}if self.meta.is_some(){return self.metadata_retirement.next_owner_demands::<ActionMeta>(true,&None,self.original_body)}if let Some(caller)=self.original_caller.as_ref(){return Ok(caller.retirement_demands())}Ok(Default::default())}
    pub fn close_metadata(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate_original(cx)?;
        if !self.context_metadata_is_empty(){self.close_context_metadata(cx)?;return Ok(false)}
        if self.meta.is_none()&&self.metadata_retirement.is_empty(){if let Some(caller)=self.original_caller.as_mut(){caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){self.original_caller.take();}return Ok(false)}return Ok(true)}
        let d=self.metadata_close_demands()?;let body=self.original_body;
        retired(cx,d,|effect|{
            if self.meta.is_some(){return begin(&mut self.metadata_retirement,&mut self.meta,effect)}
            self.metadata_retirement.step_granted(body,effect)
        })?;
        Ok(false)
    }
    pub fn metadata_is_empty(&self)->bool{self.context_metadata_is_empty()&&self.meta.is_none()&&self.metadata_retirement.is_empty()&&self.original_caller.is_none()}

    pub fn admit_original_lease(&mut self,issuer:&mut semio_plugin_operation::ToolCancellationIssuer,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.closing||self.lease.is_some(){return Ok(self.lease.is_some())}
        if !self.source_identity.matches(cx){return Err(invalid("mounted pending lease requires its original command caller").into())}
        let key=semio_plugin_operation::ToolOperationKey{app_instance_id:self.metadata()?.instance_id,document:semio_plugin_operation::ArtifactDocumentAuthority(self.metadata()?.instance_id),operation_id:self.operation.operation,base_revision:self.operation.base_revision,generation:self.operation.generation};
        self.lease=issuer.begin_keyed(&key,cx)?;
        Ok(self.lease.is_some())
    }

    /// 📏️ Quotes the next close turn of the command, key or fault owner this pending command still holds.
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.fault.is_some()||self.fault_retirement.is_some(){return plugin_typed_owner_demand(&self.fault,&self.fault_retirement,body)}
        if !self.command_retirement.is_empty(){return self.command_retirement.demands(self.original_body)}
        if self.command.is_some(){return self.command_retirement.next_owner_demands::<Box<A::Command>>(true,&None,self.original_body)}
        if self.key_copy.is_some()||self.key.is_some()||self.key_retirement.is_some(){return self.key_demands(body)}
        if self.admission.is_some(){return self.command_retirement.next_owner_demands::<AdmittedToolCommand>(true,&None,self.original_body)}
        Ok(Default::default())
    }
    fn key_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(copy)=self.key_copy.as_ref(){return Ok(if copy.key.is_none(){fixed::<Option<ToolLatestWinsKeyCopy>>()}else{copy.close_demands()});}
        plugin_typed_owner_demand(&self.key,&self.key_retirement,body)
    }
    /// ♻️ One funded retirement turn of the command, key or fault; `true` once the command owners are empty and only metadata and cancellation remain for their issuers.
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let body=cx.retained_grant().maximum_copy_bytes;let original=self.original_body;
        if self.fault.is_some()||self.fault_retirement.is_some(){let d=plugin_typed_owner_demand(&self.fault,&self.fault_retirement,body)?;retired(cx,d,|effect|plugin_typed_owner_original(&mut self.fault,&mut self.fault_retirement,effect))?;return Ok(false)}
        if !self.command_retirement.is_empty(){let d=self.command_retirement.demands(original)?;retired(cx,d,|effect|self.command_retirement.step_granted(original,effect))?;return Ok(false)}
        if self.command.is_some(){let d=self.command_retirement.next_owner_demands::<Box<A::Command>>(true,&None,original)?;retired(cx,d,|effect|begin(&mut self.command_retirement,&mut self.command,effect))?;return Ok(false)}
        if self.key_copy.is_some()||self.key.is_some()||self.key_retirement.is_some(){self.close_key_step(cx)?;return Ok(false)}
        if self.admission.is_some(){let d=self.command_retirement.next_owner_demands::<AdmittedToolCommand>(true,&None,original)?;retired(cx,d,|effect|begin(&mut self.command_retirement,&mut self.admission,effect))?;return Ok(false)}
        Ok(true)
    }
    /// 🗝️ One funded retirement turn of the key byte cursor, its shared alias or that alias's controlled frame; `true` once the key is gone.
    pub fn close_key_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let body=cx.retained_grant().maximum_copy_bytes;
        if let Some(empty)=self.key_copy.as_ref().map(|copy|copy.key.is_none()){
            if empty{acted(cx,fixed::<Option<ToolLatestWinsKeyCopy>>(),||{self.key_copy=None;Ok(())})?;}
            else if let Some(copy)=self.key_copy.as_mut(){copy.close_step(cx)?;}
            return Ok(false);
        }
        if self.key.is_some()||self.key_retirement.is_some(){let d=plugin_typed_owner_demand(&self.key,&self.key_retirement,body)?;retired(cx,d,|effect|plugin_typed_owner_original(&mut self.key,&mut self.key_retirement,effect))?;return Ok(false)}
        Ok(true)
    }
    pub fn command_owners_are_empty(&self)->bool{self.command.is_none()&&self.command_retirement.is_empty()&&self.admission.is_none()&&self.key_copy.is_none()&&self.key.is_none()&&self.key_retirement.is_none()&&self.fault.is_none()&&self.fault_retirement.is_none()}
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
