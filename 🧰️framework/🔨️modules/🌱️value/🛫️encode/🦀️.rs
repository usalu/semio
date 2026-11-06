//! 🛫️ Cumulative caller admission for owned native construction and physical output.
use crate::{ValueError, ValueRefusalKind,ErasedSnapshotRetirement};
use crate::native_receiving::span::{OriginalCheckpointRequest,OriginalRecordedIdentity,OriginalSpanRequest,SendOriginalRecordedLoan};
#[path="🧭️span/🦀️.rs"]
mod span;
pub use span::{OriginalSourceCheckpointState,OriginalSourceProgressState};
#[path="♻️retirement/🦀️.rs"]mod owned_retirement;
pub use owned_retirement::NativeEncodeRetirement;
#[path="🔭️observer/🦀️.rs"]
mod observer;
#[path="🎟️action/🦀️.rs"]
mod action;
pub use action::{OriginalWorkActionIssuer,OriginalWorkIssuerIdentity,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete};
#[path="🎟️effect/🦀️.rs"]
mod effect;
pub use effect::{native_encode_allocation_outcome_status,NativeEncodeAllocationOutcomeStatus,native_encode_effect_settlement_status,NativeEncodeEffectFailure,NativeEncodeProducerOutcome,NativeEncodeEffectProgress,NativeEncodeEffectState,NativeEncodeEffectSettlementStatus,NativeEncodeEffectSettlement,NativeEncodeEffectPostPhase,NativeEncodeEffectReceipt,NativeEncodeEffectCancellation,NativeEncodeEffectRecovery};
#[path="🏭️construction/🦀️.rs"]
mod construction;
pub use construction::{OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionLoan,OriginalConstructionIssuerIdentity,OriginalConstructionAction,OriginalConstructionPhase};
#[path="📏️maximum/🦀️.rs"]
mod maximum;
#[path="🛂️allocation/🦀️.rs"]
pub mod allocation;
pub use allocation::{NativeAllocationRequest,NativeAllocationEvent,NativeEncodeAdmissionPhase,NativeEncodeAdmissionProgress,NativeEncodeAllocationOutcome,NativeEncodeAllocationProgress,NativeEncodeAllocationQuoteStatus,NativeEncodeAllocationPhase,NativeEncodeAllocationRecovery,NativeSendAllocationLoan,NativeForwardedEncodeControl,NativeForwardedEncodeContinuation,NativeForwardedEncodeReceipt};

#[path="🫴️recipient/🦀️.rs"]
mod recipient;
pub use recipient::NativeEncodeRetirementRecipient;
#[path="🫴️recipient/🔁️continuation/🦀️.rs"]
mod retirement_continuation;
pub use retirement_continuation::{NativeRetirementEncodeContinuation,NativeDetachedRetirementEncodeContinuation};

/// ⏱️ Known encoding work and cumulative owned storage at a cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeEncodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

pub use semio_framework_value_native_outcome::NativeEncodeOperationFailure;

/// 🧵️ An opaque consuming receipt retains this operation's admission across callback lifetimes.
#[derive(Debug)]
pub struct NativeEncodeContinuation { work:action::WorkState, receiving:bool, maximum_bytes:usize, ledger:Option<crate::native_receiving::NativeAllocationLedger>, completed:usize, total:usize, stage:u64 }
impl Drop for NativeEncodeContinuation{fn drop(&mut self){assert!(std::thread::panicking()||!self.work.has_live_actions(),"native encoder continuation retains original work")}} 
/// 🧵️ Scoped Source encoding retains its original effect and authorities across the same dispatch.
pub struct NativeScopedEncodeContinuation{receipt:NativeEncodeContinuation,allocation_identity:crate::NativeAllocationIdentity,recipient_identity:u64,work_identity:crate::OriginalWorkIssuerIdentity,construction_identity:OriginalConstructionIssuerIdentity,span_identity:OriginalRecordedIdentity,checkpoint_identity:OriginalRecordedIdentity,source_turn:crate::OriginalSourceTurnIdentity,source_depth:usize,started:bool,effect:Option<NativeEncodeEffectState>,failure:Option<NativeEncodeEffectFailure>,progress:Option<OriginalSourceProgressState>}
impl std::fmt::Debug for NativeScopedEncodeContinuation{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("NativeScopedEncodeContinuation").field("source_turn",&self.source_turn).field("allocation_identity",&self.allocation_identity).field("recipient_identity",&self.recipient_identity).field("work_identity",&self.work_identity).field("construction_identity",&self.construction_identity).field("effect_present",&self.effect.is_some()).field("failure_present",&self.failure.is_some()).finish()}}
/// 🧾️ Retains every scalar control field when a scoped pause cannot be qualified.
#[derive(Debug)]
pub struct NativeScopedEncodePauseFailure{scope_maximum:Option<usize>,receiving:bool,maximum_bytes:usize,owned_bytes:usize,completed:usize,total:usize,started:bool,stage:u64,depth:usize,source_depth:Option<usize>,local_allocation:bool,retirement_present:bool,allocation_present:bool,work_present:bool}
impl NativeScopedEncodePauseFailure{pub fn scope_maximum(&self)->Option<usize>{self.scope_maximum}pub fn receiving(&self)->bool{self.receiving}pub fn maximum_bytes(&self)->usize{self.maximum_bytes}pub fn owned_bytes(&self)->usize{self.owned_bytes}pub fn completed(&self)->usize{self.completed}pub fn total(&self)->usize{self.total}pub fn started(&self)->bool{self.started}pub fn stage(&self)->u64{self.stage}pub fn depth(&self)->usize{self.depth}pub fn source_depth(&self)->Option<usize>{self.source_depth}pub fn local_allocation(&self)->bool{self.local_allocation}pub fn retirement_present(&self)->bool{self.retirement_present}pub fn allocation_present(&self)->bool{self.allocation_present}pub fn work_present(&self)->bool{self.work_present}}
impl NativeScopedEncodeContinuation{
 /// 🔭️ Reports committed actions retained by the original source continuation.
 pub fn actual_work(&self)->u128{self.receipt.work.actual()}
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes()}
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
}
impl crate::retirement::RetireOwned for NativeScopedEncodeContinuation{type RetirementCursor=<(NativeEncodeContinuation,Option<NativeEncodeEffectState>,Option<NativeEncodeEffectFailure>,Option<OriginalSourceProgressState>) as crate::retirement::RetireOwned>::RetirementCursor;fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.receipt)&&crate::retirement::RetireOwned::retirement_ready(&self.effect)&&crate::retirement::RetireOwned::retirement_ready(&self.failure)&&crate::retirement::RetireOwned::retirement_ready(&self.progress)}fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed((self.receipt,self.effect,self.failure,self.progress))}fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(size_of::<Self::RetirementCursor>())}fn controlled_retirement_supported()->bool{true}}
pub struct EncodeContinuationRetirement(Option<NativeEncodeContinuation>);
impl crate::retirement::RetirementCursor for EncodeContinuationRetirement{
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{Ok(crate::RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{let grant=effect.grant();if self.0.is_none(){return crate::retirement::RetirementStep::Complete(0)}if self.0.as_ref().is_some_and(|receipt|!crate::retirement::RetireOwned::retirement_ready(receipt)){return crate::retirement::RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"encoder continuation retains original ledger or work").into(),0)}if grant.maximum_items==0{return crate::retirement::RetirementStep::BudgetExhausted}if grant.maximum_depth==0{return crate::retirement::RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit,"encoding continuation requires original source depth").into(),0)}self.0.take();crate::retirement::RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,..Default::default()},1)}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.0.is_some()))}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl crate::retirement::RetireOwned for NativeEncodeContinuation{
 type RetirementCursor=EncodeContinuationRetirement;
 fn retirement_ready(&self)->bool{!self.work.has_live_actions()&&self.ledger.as_ref().is_some_and(|ledger|!ledger.is_bound()&&!ledger.has_progress())}
 fn retirement_seed(self)->Self::RetirementCursor{EncodeContinuationRetirement(Some(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{std::mem::size_of::<Self>()}
}

/// 🧮️ One ownership ceiling persists through typed fields, physical output and its envelope.
pub struct NativeEncodeControl<'a> { scope_maximum:Option<usize>, retirement:Option<&'a mut NativeEncodeRetirementRecipient>, construction:Option<&'a mut OriginalConstructionLoan<'a>>, construction_identity:Option<OriginalConstructionIssuerIdentity>, receiving:bool, maximum_bytes:usize, ledger:Option<crate::native_receiving::NativeAllocationLedger>, completed:usize, total:usize, started:bool, stage:u64, depth:usize, source_depth:Option<usize>,callback:Option<&'a mut dyn FnMut(NativeEncodeProgress)->bool>, allocation:allocation::NativeEncodeAllocationBinding<'a>,scoped_allocation:Option<&'a mut (dyn NativeSendAllocationLoan+'a)>,work:Option<&'a mut dyn FnMut(usize,usize)->Result<(),ValueError>>,recorded_span:Option<&'a mut (dyn SendOriginalRecordedLoan<OriginalSpanRequest,()>+'a)>,recorded_checkpoint:Option<&'a mut (dyn SendOriginalRecordedLoan<OriginalCheckpointRequest,bool>+'a)>,source_turn:Option<crate::OriginalSourceTurnIdentity>,action:action::Action<'a> }

impl std::fmt::Debug for NativeEncodeControl<'_>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("NativeEncodeControl").field("owned_bytes",&self.owned_bytes()).field("actual_work",&self.action.actual()).field("pending_work",&self.action.has_live_actions()).finish()}}

struct StageScope<'owner,'control>{control:&'owner mut NativeEncodeControl<'control>,parent:(usize,usize,bool,u64)}
impl Drop for StageScope<'_, '_>{fn drop(&mut self){if self.control.stage!=self.parent.3{self.control.completed=self.parent.0;self.control.total=self.parent.1;self.control.started=self.parent.2;self.control.stage=self.parent.3;}}}
struct DepthScope<'owner,'control>{control:&'owner mut NativeEncodeControl<'control>,parent:usize}
impl Drop for DepthScope<'_,'_>{fn drop(&mut self){self.control.depth=self.parent}}

impl<'a> NativeEncodeControl<'a> {

    /// 🫴️ Runs a child against both original authorities with caller-owned unwind custody.
    pub fn with_encoding_receiver<T,E>(&mut self,frame:&mut Option<crate::native_receiving::NativeEncodingReceiverFrame>,result:&mut semio_framework_value_call::NativeCallFailure<T,E>,operation:impl FnOnce(usize,&mut dyn FnMut(NativeEncodeProgress)->bool,&mut dyn NativeSendAllocationLoan,&mut crate::native_receiving::NativeEncodingReceiverWork<'_,'_,'_>,Option<&mut crate::native_receiving::NativeEncodingReceiverConstruction<'_,'_>>,&mut Option<T>,&mut Option<E>))->Result<(),ValueError>{
        if self.callback.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"recorded Source receiver requires recorded progress"))}if frame.is_some()||result.output.is_some()||result.operation.is_some()||result.context.is_some()||result.receipt_refusal.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiver requires empty original custody"))}
        self.checkpoint()?;
        self.scoped_stage(|original|{
            let start=original.owned_bytes();let maximum=original.maximum_bytes.checked_sub(start).ok_or_else(crate::native_receiving::invariant)?;
            *frame=Some(crate::native_receiving::NativeEncodingReceiverFrame::default());let frame=frame.as_mut().expect("original receiver frame installed");
            let forwarded:Option<&dyn NativeSendAllocationLoan>=match &original.allocation{allocation::NativeEncodeAllocationBinding::Local=>None,allocation::NativeEncodeAllocationBinding::Forwarded(port)=>Some(&**port)};
            let scoped=original.scoped_allocation.as_ref().map(|port|&**port as &dyn NativeSendAllocationLoan);
            frame.install_pair(original.ledger.as_mut().ok_or_else(crate::native_receiving::invariant)?,forwarded,scoped,original.maximum_bytes)?;
            frame.begin_operation()?;
            let ledger=original.ledger.as_mut().ok_or_else(crate::native_receiving::invariant)?;
            let forwarded:Option<&mut dyn NativeSendAllocationLoan>=match &mut original.allocation{allocation::NativeEncodeAllocationBinding::Local=>None,allocation::NativeEncodeAllocationBinding::Forwarded(port)=>Some(&mut **port)};
            let scoped=original.scoped_allocation.as_deref_mut();
            let mut construction=original.construction.as_mut().map(|loan|loan.reborrow());let original_work=&mut original.action;let work_observer=original.work.as_mut().map(|port|&mut **port as &mut (dyn FnMut(usize,usize)->Result<(),ValueError>+'_));let callback=original.callback.as_deref_mut().ok_or_else(crate::native_receiving::invariant)?;let completed=&mut original.completed;let total=&mut original.total;let started=&mut original.started;let stage=&mut original.stage;
            let semio_framework_value_call::NativeCallFailure{output,operation:operation_error,context,receipt_refusal}=result;let mut previous=None::<(usize,usize)>;
            let run=frame.with_original_authorities(ledger,forwarded,scoped,original_work,work_observer,construction.as_mut(),|loan,observed,work,physical|{
                let mut observer=|progress:NativeEncodeProgress|{if context.is_some(){return false}let actual=observed.load(std::sync::atomic::Ordering::Relaxed);let valid=start.checked_add(progress.owned_bytes)==Some(actual)&&(progress.total==0||progress.completed<=progress.total);if !valid{*context=Some(crate::native_receiving::invariant());return false}if previous.is_none_or(|(done,all)|all!=progress.total||done>progress.completed){match stage.checked_add(1){Some(next)=>*stage=next,None=>{*context=Some(crate::native_receiving::invariant());return false}}}*completed=progress.completed;*total=progress.total;*started=true;previous=Some((progress.completed,progress.total));if (*callback)(NativeEncodeProgress{completed:*completed,total:*total,owned_bytes:actual}){true}else{*context=Some(ValueError::literal(ValueRefusalKind::Canceled,"original native receiving observer canceled"));false}};
                operation(maximum,&mut observer,loan,work,physical,output,operation_error)
            });
            if let Err(error)=run{if context.is_none(){*context=Some(error)}else{*receipt_refusal=Some(error)}}
            if output.is_some()==operation_error.is_some()&&context.is_none(){*context=Some(crate::native_receiving::invariant())}Ok(())
        })
    }
    /// 🔁️ Recovers only through the same resident child and original parent authorities.
    pub fn recover_encoding_receiver<R>(&mut self,frame:&mut crate::native_receiving::NativeEncodingReceiverFrame,operation:impl FnOnce(&mut dyn NativeSendAllocationLoan,&mut crate::native_receiving::NativeEncodingReceiverWork<'_,'_,'_>,Option<&mut crate::native_receiving::NativeEncodingReceiverConstruction<'_,'_>>)->R)->Result<R,ValueError>{let ledger=self.ledger.as_mut().ok_or_else(crate::native_receiving::invariant)?;let forwarded:Option<&mut dyn NativeSendAllocationLoan>=match &mut self.allocation{allocation::NativeEncodeAllocationBinding::Local=>None,allocation::NativeEncodeAllocationBinding::Forwarded(port)=>Some(&mut **port)};let scoped=self.scoped_allocation.as_deref_mut();let mut construction=self.construction.as_mut().map(|loan|loan.reborrow());let original_work=&mut self.action;let work_observer=self.work.as_mut().map(|port|&mut **port as &mut (dyn FnMut(usize,usize)->Result<(),ValueError>+'_));frame.with_original_authorities(ledger,forwarded,scoped,original_work,work_observer,construction.as_mut(),|loan,observed,work,physical|{let _=observed;operation(loan,work,physical)})}
    /// 🔓️ Releases the settled receiver to its exact original ledger and authorities.
    pub fn release_encoding_receiver(&mut self,frame:&mut crate::native_receiving::NativeEncodingReceiverFrame)->Result<(),ValueError>{let ledger=self.ledger.as_mut().ok_or_else(crate::native_receiving::invariant)?;let forwarded:Option<&dyn NativeSendAllocationLoan>=match &self.allocation{allocation::NativeEncodeAllocationBinding::Local=>None,allocation::NativeEncodeAllocationBinding::Forwarded(port)=>Some(&**port)};let scoped=self.scoped_allocation.as_ref().map(|port|&**port as &dyn NativeSendAllocationLoan);frame.release_pair(ledger,forwarded,scoped)}
    /// 🔭️ Reborrows the complete caller owner and composes observers without resetting its receipt.
    pub fn scoped_observer<T,E:From<ValueError>>(&mut self,observer:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{observer::run(self,observer,operation)}

    /// 🪪️ Names the installed original encoder retirement recipient.
    pub fn retirement_recipient_identity(&self)->Option<u64>{self.retirement.as_ref().map(|recipient|recipient.identity)}
    /// 👓️ Reports the original return slot without moving its pending physical owner.
    pub fn has_retirement_owner(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_owner())}
    /// ♻️ Drains one original return turn after forwarding its exact physical capacity demand.
    pub fn close_retirement_recipient(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{let grant=effect.grant();
        let preflight=(||{
            let recipient=self.retirement.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"encoder has no original retirement recipient"))?;
            if recipient.terminal_is_empty()||grant.maximum_items==0{return Ok(false)}
            let d=recipient.next_demand(grant.maximum_copy_bytes)?;
            if d.depth>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"encoder retirement exceeds original depth grant"))}
            if !crate::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"encoder retirement exceeds original physical grant"))}
            if d.capacity_bytes>self.maximum_bytes.saturating_sub(self.owned_bytes()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"encoder retirement exceeds its original remaining capacity"))}if recipient.next_computational_work(grant.maximum_copy_bytes)?>effect.maximum_work_units(){return Ok(false)}self.checkpoint()?;Ok(true)
        })();
        match preflight{
            Err(error)=>semio_framework_value_control_contract::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0}),
            Ok(true)=>match self.retirement.as_mut(){Some(v)=>v,None=>return semio_framework_value_control_contract::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0})}.close_step(effect),
            Ok(false)=>semio_framework_value_control_contract::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(if self.retirement.as_ref().is_some_and(|r|r.terminal_is_empty()){crate::RetainedCloneStep::Complete(Default::default())}else{crate::RetainedCloneStep::Progress(Default::default())}),work:0})
        }
    }

    /// 🆕️ Installs one explicit caller recipient before any retained encoder ownership is created.
    pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<(),ValueError>{
        if self.retirement.is_some()||!recipient.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encode requires one empty explicit retirement recipient"));}
        self.retirement=Some(recipient);Ok(())
    }
    /// 🪑️ Pre-admits the return slot and exact immediate wrapper allocations, preserving every refusal owner.
    pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{
        if self.retirement.as_ref().is_none_or(|recipient|recipient.reserved||recipient.owner.is_some()){return Err(E::from(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encode has no available explicit retirement slot")));}
        self.checkpoint()?;self.charge(wrapper_bytes)?;
        self.retirement.as_mut().unwrap().reserved=true;
        let(result,owner)=operation(self);
        let recipient=self.retirement.as_mut().unwrap();*recipient.owner=owner;recipient.reserved=false;
        result
    }


    /// ⏸️ Transfers cumulative admission without retaining the current hop's callback.
    pub fn pause(mut self)->Result<NativeEncodeContinuation,(ValueError,Self)>{
        if !matches!(self.allocation,allocation::NativeEncodeAllocationBinding::Local)||self.scoped_allocation.is_some()||self.work.is_some()||self.action.recipient.is_some()||self.action.has_live_actions()||self.construction.is_some()||self.retirement.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native encoding requires its original allocation continuation"),self));}
        match self.continuation(){Ok(receipt)=>Ok(receipt),Err(error)=>Err((error,self))}
    }
    fn validate_continuation(&self)->Result<(),ValueError>{if self.source_turn.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"recorded Source requires its original scoped continuation"))}if self.ledger.as_ref().is_none_or(|ledger|ledger.is_bound())||self.action.recipient.is_some()||self.action.has_live_actions()||self.construction.is_some()||self.depth!=0||(self.total!=0&&self.completed>self.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode continuation lacks valid cumulative accounting"))}Ok(())}
    fn continuation(&mut self)->Result<NativeEncodeContinuation,ValueError>{
        self.validate_continuation()?;
        Ok(NativeEncodeContinuation{work:self.action.state.take(),receiving:self.receiving,maximum_bytes:self.maximum_bytes,ledger:self.ledger.take(),completed:self.completed,total:self.total,stage:self.stage})
    }
    /// ▶️ Rebinds a moved operation receipt to the current hop's cancellation callback.
    pub fn resume(mut receipt:NativeEncodeContinuation,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Result<Self,(ValueError,NativeEncodeContinuation)>{
        if receipt.ledger.is_none(){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode continuation lacks its original ledger"),receipt));}if receipt.work.has_live_actions(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encode continuation requires its original live work loan"),receipt));}if (receipt.total!=0&&receipt.completed>receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode continuation has invalid cumulative accounting"),receipt));}
        Ok(Self{scope_maximum:(!receipt.receiving).then_some(receipt.maximum_bytes),retirement:None,construction:None,construction_identity:None,receiving:receipt.receiving,maximum_bytes:receipt.maximum_bytes,ledger:receipt.ledger.take(),completed:receipt.completed,total:receipt.total,started:false,stage:receipt.stage,depth:0,source_depth:None,callback:Some(callback),allocation:allocation::NativeEncodeAllocationBinding::Local,scoped_allocation:None,work:None,recorded_span:None,recorded_checkpoint:None,source_turn:None,action:action::Action{state:receipt.work.take(),recipient:None}})
    }

    /// 🚦️ Binds the exact caller ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Self{Self{scope_maximum:Some(maximum_bytes),retirement:None,construction:None,construction_identity:None,receiving:false,maximum_bytes,ledger:Some(crate::native_receiving::NativeAllocationLedger::new(0)),completed:0,total:0,started:false,stage:0,depth:0,source_depth:None,callback:Some(callback),allocation:allocation::NativeEncodeAllocationBinding::Local,scoped_allocation:None,work:None,recorded_span:None,recorded_checkpoint:None,source_turn:None,action:action::Action::empty()}}
    /// 🧭️ Binds the actual Source allocation, span and checkpoint producers without a raw callback.
    pub fn new_recorded_source(maximum_bytes:usize,maximum_depth:usize,source:crate::OriginalSourceTurnWitness<'_>,allocate:&'a mut (dyn NativeSendAllocationLoan+'a),span:&'a mut (dyn SendOriginalRecordedLoan<OriginalSpanRequest,()>+'a),checkpoint:&'a mut (dyn SendOriginalRecordedLoan<OriginalCheckpointRequest,bool>+'a))->Result<Self,ValueError>{let turn=source.identity();if allocate.source_turn()!=Some(turn)||span.source_turn()!=Some(turn)||checkpoint.source_turn()!=Some(turn){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"recorded Source authorities do not match the original turn"))}allocate.validate_context()?;span.validate_context()?;checkpoint.validate_context()?;Ok(Self{scope_maximum:None,retirement:None,construction:None,construction_identity:None,receiving:true,maximum_bytes,ledger:Some(crate::native_receiving::NativeAllocationLedger::new(0)),completed:0,total:0,started:false,stage:0,depth:0,source_depth:Some(maximum_depth),callback:None,allocation:allocation::NativeEncodeAllocationBinding::Local,scoped_allocation:Some(allocate),work:None,recorded_span:Some(span),recorded_checkpoint:Some(checkpoint),source_turn:Some(turn),action:action::Action::empty()})}
    /// ⏸️ Transfers the same Source actions, effect and ledger into caller-owned custody.
    pub fn pause_scoped_source(mut self,source:crate::OriginalSourceTurnWitness<'_>,work:&crate::OriginalWorkLoan<'_>,construction:&OriginalConstructionLoan<'_>,effect:&mut Option<NativeEncodeEffectState>,failure:&mut Option<NativeEncodeEffectFailure>,progress:&mut Option<OriginalSourceProgressState>)->Result<NativeScopedEncodeContinuation,(ValueError,Self)>{
        let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped encoding requires its exact original Source authorities and custody");let turn=source.identity();
        if self.source_turn!=Some(turn)||!matches!(self.allocation,allocation::NativeEncodeAllocationBinding::Local)||self.scoped_allocation.is_none()||self.recorded_span.is_none()||self.recorded_checkpoint.is_none()||self.callback.is_some()||self.work.is_some()||self.source_depth.is_none()||self.retirement.is_none()||!self.receiving||self.scope_maximum.is_some()||self.construction.is_some()||self.action.recipient.is_some()||self.depth!=0||self.ledger.as_ref().is_none_or(|ledger|ledger.is_bound())||(self.total!=0&&self.completed>self.total){return Err((invalid(),self))}
        let ledger=self.ledger.as_ref().expect("validated ledger");let allocation=self.scoped_allocation.as_deref_mut().expect("validated allocation");let span=self.recorded_span.as_deref_mut().expect("validated span");let checkpoint=self.recorded_checkpoint.as_deref_mut().expect("validated checkpoint");
        if let Err(error)=crate::native_receiving::span::progress::validate_source_authorities(source,allocation,work,construction){return Err((error,self))}
        if let Some(state)=effect.as_ref().and_then(|effect|effect.checkpoint.as_ref()){if progress.is_some(){return Err((invalid(),self))}if let Err(error)=crate::native_receiving::span::progress::validate_source_checkpoint(source,ledger,self.stage,self.completed,self.total,self.started,span,checkpoint,state){return Err((error,self))}}else if let Err(error)=crate::native_receiving::span::progress::validate_source_progress(source,ledger,self.stage,self.completed,self.total,self.started,span,checkpoint,progress.as_ref()){return Err((error,self))}
        let allocation_identity=allocation.identity();let span_identity=span.identity();let checkpoint_identity=checkpoint.identity();let recipient_identity=self.retirement.as_ref().expect("validated recipient").identity;let work_identity=work.identity();let construction_identity=construction.identity();
        if let Err(error)=self.action.validate_recipient(work){return Err((error,self))}if effect.is_none(){if failure.is_some()||!allocation.matches_transaction(None)||self.action.has_live_actions(){return Err((invalid(),self))}}else if effect.as_ref().is_some_and(|effect|effect.checkpoint.is_some()||(effect.checkpoint_ready.is_some()&&effect.settlement.is_none()&&!effect.quote_started)){if !allocation.matches_transaction(None)||self.action.has_live_actions(){return Err((invalid(),self))}}else if failure.is_none()||effect.as_ref().is_none_or(|effect|effect.settlement.is_none())||effect.as_ref().and_then(|effect|effect.allocation.as_ref()).map_or_else(||!allocation.matches_transaction(None),|state|!state.matches_scoped_authority(&*allocation))||effect.as_ref().and_then(|effect|effect.settlement.as_ref()).is_some_and(|settlement|!settlement.matches_authorities(work_identity,construction_identity)){return Err((invalid(),self))}
        let source_depth=self.source_depth.expect("validated depth");let started=self.started;let receipt=NativeEncodeContinuation{work:self.action.state.take(),receiving:self.receiving,maximum_bytes:self.maximum_bytes,ledger:self.ledger.take(),completed:self.completed,total:self.total,stage:self.stage};Ok(NativeScopedEncodeContinuation{receipt,allocation_identity,recipient_identity,work_identity,construction_identity,span_identity,checkpoint_identity,source_turn:turn,source_depth,started,effect:effect.take(),failure:failure.take(),progress:progress.take()})
    }
    /// ▶️ Restores the same paid Source dispatch, authorities and caller-owned actions.
    pub fn resume_scoped_source(mut receipt:NativeScopedEncodeContinuation,source:crate::OriginalSourceTurnWitness<'_>,allocate:&'a mut (dyn NativeSendAllocationLoan+'a),span:&'a mut (dyn SendOriginalRecordedLoan<OriginalSpanRequest,()>+'a),checkpoint:&'a mut (dyn SendOriginalRecordedLoan<OriginalCheckpointRequest,bool>+'a),recipient:&'a mut NativeEncodeRetirementRecipient,work:&crate::OriginalWorkLoan<'_>,construction:&OriginalConstructionLoan<'_>,effect:&mut Option<NativeEncodeEffectState>,failure:&mut Option<NativeEncodeEffectFailure>,progress:&mut Option<OriginalSourceProgressState>)->Result<Self,(NativeScopedEncodeContinuation,ValueError)>{
        let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped Source continuation changed its original dispatch or authority");let turn=source.identity();
        if !source.matches(receipt.source_turn)||receipt.allocation_identity!=allocate.identity()||receipt.span_identity!=span.identity()||receipt.checkpoint_identity!=checkpoint.identity()||receipt.recipient_identity!=recipient.identity||receipt.work_identity!=work.identity()||receipt.construction_identity!=construction.identity()||effect.is_some()||failure.is_some()||progress.is_some()||receipt.receipt.ledger.is_none()||!receipt.receipt.receiving||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total)||receipt.receipt.work.issuer_identity().is_some_and(|identity|identity!=work.identity().get()){return Err((receipt,invalid()))}
        let ledger=receipt.receipt.ledger.as_ref().expect("validated ledger");if let Some(state)=receipt.effect.as_ref().and_then(|effect|effect.checkpoint.as_ref()){if receipt.progress.is_some(){return Err((receipt,invalid()))}if let Err(error)=crate::native_receiving::span::progress::validate_source_checkpoint(source,ledger,receipt.receipt.stage,receipt.receipt.completed,receipt.receipt.total,receipt.started,span,checkpoint,state){return Err((receipt,error))}}else if let Err(error)=crate::native_receiving::span::progress::validate_source_progress(source,ledger,receipt.receipt.stage,receipt.receipt.completed,receipt.receipt.total,receipt.started,span,checkpoint,receipt.progress.as_ref()){return Err((receipt,error))}
        if receipt.effect.is_none(){if receipt.failure.is_some()||!allocate.matches_transaction(None)||receipt.receipt.work.has_live_actions(){return Err((receipt,invalid()))}}else if receipt.effect.as_ref().is_some_and(|effect|effect.checkpoint.is_some()||(effect.checkpoint_ready.is_some()&&effect.settlement.is_none()&&!effect.quote_started)){if !allocate.matches_transaction(None)||receipt.receipt.work.has_live_actions(){return Err((receipt,invalid()))}}else if receipt.failure.is_none()||receipt.effect.as_ref().is_none_or(|effect|effect.settlement.is_none())||receipt.effect.as_ref().and_then(|effect|effect.allocation.as_ref()).map_or_else(||!allocate.matches_transaction(None),|state|!state.matches_scoped_authority(allocate))||receipt.effect.as_ref().and_then(|effect|effect.settlement.as_ref()).is_some_and(|settlement|!settlement.matches_authorities(work.identity(),construction.identity()))||(receipt.receipt.work.has_live_actions()&&!work.has_live_actions()){return Err((receipt,invalid()))}
        if let Err(error)=crate::native_receiving::span::progress::validate_source_authorities(source,allocate,work,construction){return Err((receipt,error))}
        let control=Self{scope_maximum:None,retirement:Some(recipient),construction:None,construction_identity:None,receiving:receipt.receipt.receiving,maximum_bytes:receipt.receipt.maximum_bytes,ledger:receipt.receipt.ledger.take(),completed:receipt.receipt.completed,total:receipt.receipt.total,started:receipt.started,stage:receipt.receipt.stage,depth:0,source_depth:Some(receipt.source_depth),callback:None,allocation:allocation::NativeEncodeAllocationBinding::Local,scoped_allocation:Some(allocate),work:None,recorded_span:Some(span),recorded_checkpoint:Some(checkpoint),source_turn:Some(turn),action:action::Action{state:receipt.receipt.work.take(),recipient:None}};*effect=receipt.effect.take();*failure=receipt.failure.take();*progress=receipt.progress.take();Ok(control)
    }
    /// 📏️ Returns the complete allowance and cumulative admitted ownership.
    pub fn maximum_bytes(&self)->usize{self.maximum_bytes}
    /// 📊️ Returns storage admitted across every nested stage.
    pub fn owned_bytes(&self)->usize{self.ledger.as_ref().expect("original encode ledger is lent").owned_bytes()}
    /// 🛂️ Starts with zero storage authority; each original retained turn must supply its grant.
    pub fn new_retained(callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Self{let mut control=Self::new(0,callback);control.receiving=true;control.scope_maximum=None;control}
    /// 🎟️ Binds this original turn's allocation authority without resetting cumulative ownership.
    pub fn admit_turn_capacity(&mut self,maximum_capacity_bytes:usize)->Result<(),ValueError>{let whole=self.scope_maximum;let next=crate::native_receiving::turn::maximum(self.owned_bytes(),maximum_capacity_bytes,whole)?;self.maximum_bytes=whole.map_or(next,|maximum|maximum.min(next));Ok(())}

    /// 🪙️ Consumes the same admission owner under an exact checked source capacity.
    pub fn admit_capacity(mut self,source_bytes:usize,multiples:usize,scaffold_bytes:usize)->Result<Self,(Self,ValueError)>{match source_bytes.checked_mul(multiples).and_then(|bytes|bytes.checked_add(scaffold_bytes)).filter(|bytes|*bytes<=isize::MAX as usize&&*bytes>=self.owned_bytes()){Some(maximum)=>{self.maximum_bytes=maximum;if !self.receiving{self.scope_maximum=Some(maximum);}Ok(self)},None=>Err((self,ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encoding capacity exceeds source policy or admitted ownership")))}}
    /// 📐️ Applies a narrower physical stage ceiling without resetting ownership.
    pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{maximum::run(self,maximum,operation)}
    /// 🛑️ Checks cancellation before an expensive operation.
    pub fn checkpoint(&mut self)->Result<(),ValueError>{if self.ledger.as_ref().is_none_or(|ledger|ledger.is_bound()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation ledger is paired or absent"))}let owned=self.owned_bytes();if(self.callback.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"local checkpoint requires its original observer"))?)(NativeEncodeProgress{completed:self.completed,total:self.total,owned_bytes:owned}){self.started=true;Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native encoding canceled"))}}
    /// 🧭️ Begins an exact workload without resetting admitted ownership.
    pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{if self.source_turn.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"recorded Source stage requires its original actions"))}self.stage=self.stage.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "native encoding stage overflow"))?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🪆️ Restores the enclosing workload while retaining child allocation charges.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{let parent=(self.completed,self.total,self.started,self.stage);let scope=StageScope{control:self,parent};operation(&mut *scope.control)}
    /// 📏️ Checks iterative nesting against the current depth and original source ceiling.
    pub fn check_additional_depth(&self,additional:usize)->Result<(),ValueError>{let depth=self.depth.checked_add(additional).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"native encoding exceeds depth limit"))?;if self.source_depth.is_some_and(|maximum|depth>maximum){return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"native encoding exceeds depth limit"))}Ok(())}
    /// 🪆️ Temporarily enters iterative nesting under the original source ceiling.
    pub fn scoped_additional_depth<T,E:From<ValueError>>(&mut self,additional:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{self.check_additional_depth(additional).map_err(E::from)?;let parent=self.depth;self.depth=parent+additional;let scope=DepthScope{control:self,parent};operation(&mut *scope.control)}
    /// 🌲️ Bounds borrowed recursive projection independently of the input's depth.
    pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{let maximum=self.source_depth.map_or(maximum,|source|source.min(maximum));if self.depth>=maximum{return Err(E::from(ValueError::literal(ValueRefusalKind::DepthLimit, "native encoding exceeds depth limit")))}self.depth+=1;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(self)));self.depth-=1;match result{Ok(result)=>result,Err(payload)=>std::panic::resume_unwind(payload)}}
    /// 📍️ Publishes interior checkpoints at256-unit boundaries and stage completion.
    pub fn advance(&mut self,units:usize)->Result<(),ValueError>{if self.source_turn.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"recorded Source progress requires its original actions"))}let source_span=self.action.recipient.is_none();self.advance_progress(units,source_span)}
    fn advance_progress(&mut self,units:usize,source_span:bool)->Result<(),ValueError>{if !self.started{self.checkpoint()?;}let previous=self.completed;let next=self.completed.checked_add(units).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native encoding work overflow"))?;if self.total!=0&&next>self.total{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native encoding exceeds declared workload"))}self.completed=next;if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}if source_span{let owned=self.owned_bytes();if let Some(work)=self.work.as_mut(){work(units,owned)?;}}Ok(())}
    /// 🔢️ Advances one explicit owned field or collection item.
    pub fn step(&mut self)->Result<(),ValueError>{self.advance(1)}
    /// 📦️ Bounds local storage; original allocation ports require their quoted actual effect.
    pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{
        if !matches!(self.allocation,allocation::NativeEncodeAllocationBinding::Local)||self.scoped_allocation.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native original allocation requires its quoted actual effect"))}
        let next=self.owned_bytes().checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encoding ownership exceeds caller limit"))?;
        if !self.started||bytes>65536{self.checkpoint()?;}let request=NativeAllocationRequest{bytes,owned_bytes:self.owned_bytes(),next_owned_bytes:next,maximum_bytes:self.maximum_bytes};self.ledger.as_mut().expect("original encode ledger is lent").commit(request)?;Ok(())
    }
    /// 🗂️ Reserves a known typed frontier before creating its slots.
    pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "native encoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "native encoding collection allocation failed"))?;Ok(output)}
    /// 🔤️ Owns exact UTF-8 through cancellable64KiB spans.
    pub fn copy_text(&mut self,text:&str)->Result<String,ValueError>{if self.work.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Source byte encoding requires its explicit partial receiver"))}self.scoped_stage(|control|{control.begin_stage(text.len())?;control.charge(text.len())?;let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "native encoding text allocation failed"))?;let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);control.advance(end-position)?;position=end;}Ok(output)})}
    /// 🧬️ Owns intrinsic octets through cancellable64KiB spans.
    pub fn copy_bytes(&mut self,bytes:&[u8])->Result<Vec<u8>,ValueError>{if self.work.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Source byte encoding requires its explicit partial receiver"))}self.scoped_stage(|control|{control.begin_stage(bytes.len())?;let mut output=control.allocate_vec(bytes.len())?;for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);control.advance(chunk.len())?;}Ok(output)})}
    /// 📎️ Appends exact bytes with admission and cancellation inside materialization.
    pub fn append_bytes(&mut self,output:&mut Vec<u8>,bytes:&[u8])->Result<(),ValueError>{self.scoped_stage(|control|{
        control.begin_stage(bytes.len())?;
        for chunk in bytes.chunks(65536){
            let capacity=output.len().checked_add(chunk.len()).filter(|capacity|*capacity<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encoding output capacity overflow"))?;
            let metered=control.has_work_recipient();
            if metered&&!control.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native encoding append exceeds original work loan"))}
            let allocation=(||{if capacity>output.capacity(){control.charge(capacity)?;output.try_reserve_exact(chunk.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native encoding output allocation failed"))?;}Ok::<(),ValueError>(())})();
            if let Err(error)=allocation{if metered{control.cancel_work()?;}return Err(error)}
            if metered{let owned=control.owned_bytes();if let Some(source)=control.work.as_mut(){if let Err(error)=source(chunk.len(),owned){control.cancel_work()?;return Err(error)}}}
            else{control.advance(chunk.len())?;}
            output.extend_from_slice(chunk);
            if metered{control.commit_work(1)?;control.advance_progress(chunk.len(),false)?;}
        }
        Ok(())
    })}
}



#[cfg(test)]
mod pause_state_tests{
 use super::*;
 #[test]
 fn receiving_output_and_original_pair_survive_unwind(){
  let mut observer=|_:NativeEncodeProgress|true;let mut original=NativeEncodeControl::new(64,&mut observer);let mut frame=None;let mut result=semio_framework_value_call::NativeCallFailure::<u32,ValueError>{output:None,operation:None,context:None,receipt_refusal:None};
  let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||original.with_encoding_receiver(&mut frame,&mut result,|_,_,_,_,_,output,_|{*output=Some(37);panic!("child interrupted after publishing its owner")}).unwrap()));
  assert!(interrupted.is_err());assert_eq!(result.output,Some(37));assert!(original.checkpoint().is_err());let frame=frame.as_mut().unwrap();frame.settle_pair().unwrap();assert!(frame.take_allocation_failure().unwrap().is_empty());original.release_encoding_receiver(frame).unwrap();assert_eq!(original.owned_bytes(),0);
 }
 #[test]
 fn receiving_preserves_original_work_refusal_and_output_after_unwind(){
  struct OriginalWorkSource;impl semio_framework_value_control_contract::OriginalWorkRecipient for OriginalWorkSource{fn admits<'a>(&mut self,quote:semio_framework_value_control_contract::OriginalWorkQuote<'a>,_:u64,_:u64)->semio_framework_value_control_contract::OriginalWorkQuoteComplete<'a>{quote.complete(Err(ValueError::literal(ValueRefusalKind::Canceled,"actual original work refusal")))}}
  let mut issuer=semio_framework_value_control_contract::OriginalWorkActionIssuer::new();let mut source=OriginalWorkSource;let mut loan=issuer.loan(&mut source);let mut observe=|_:NativeEncodeProgress|true;let mut original=NativeEncodeControl::new(64,&mut observe);let mut frame=None;let mut result=semio_framework_value_call::NativeCallFailure::<u32,ValueError>{output:None,operation:None,context:None,receipt_refusal:None};
  let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||original.with_work_recipient(&mut loan,|control|->Result<(),ValueError>{control.with_encoding_receiver(&mut frame,&mut result,|_,_,_,work,_,output,operation_error|{*output=Some(37);*operation_error=Some(work.prepare(1).unwrap_err());panic!("original child interrupted after owning result and refusal")})?;Ok(())})));
  assert!(interrupted.is_err());assert_eq!(result.output,Some(37));assert_eq!(result.operation.as_ref().unwrap().message.as_ref(),"actual original work refusal");let frame=frame.as_mut().unwrap();frame.settle_pair().unwrap();assert!(frame.take_allocation_failure().unwrap().is_empty());original.release_encoding_receiver(frame).unwrap();drop(loan);assert!(!issuer.has_live_actions());
 }
 #[test]
 fn receiving_keeps_result_and_original_bound_authorities_after_unwind(){
  struct Work;impl semio_framework_value_control_contract::OriginalWorkRecipient for Work{fn admits<'a>(&mut self,q:semio_framework_value_control_contract::OriginalWorkQuote<'a>,_:u64,_:u64)->semio_framework_value_control_contract::OriginalWorkQuoteComplete<'a>{q.complete(Ok(true))}}
  struct Construction;impl semio_framework_value_control_contract::OriginalConstructionRecipient for Construction{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}fn grant<'a,'turn,'source>(&mut self,_:&mut (),q:semio_framework_value_control_contract::OriginalConstructionGrant<'a>)->semio_framework_value_control_contract::OriginalConstructionGrantComplete<'a> where 'source:'turn{let _reply=q.complete(Err(ValueError::literal(ValueRefusalKind::Canceled,"actual construction refusal")));panic!("actual construction producer interrupted after recording its reply")}fn accept<'a,'turn,'source>(&mut self,_:&mut (),q:semio_framework_value_control_contract::OriginalConstructionAccept<'a>)->semio_framework_value_control_contract::OriginalConstructionAcceptComplete<'a> where 'source:'turn{q.complete(Ok(()))}fn submitted<'turn,'source>(&self,_:&())->crate::RetainedCloneProgress where 'source:'turn{Default::default()}}
  let mut physical=semio_framework_value_control_contract::OriginalConstructionAuthority::new(Construction);let mut issuer=semio_framework_value_control_contract::OriginalWorkActionIssuer::new();let mut payer=Work;let mut work_loan=issuer.loan(&mut payer);let mut observe=|_:NativeEncodeProgress|true;let mut original=NativeEncodeControl::new(64,&mut observe);let mut frame=None;let mut result=semio_framework_value_call::NativeCallFailure::<u32,ValueError>{output:None,operation:None,context:None,receipt_refusal:None};
  let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||physical.with_loan(&mut (),|physical_loan|original.with_work_recipient(&mut work_loan,|control|control.with_construction_recipient(physical_loan,|control|{control.with_encoding_receiver(&mut frame,&mut result,|_,_,_,work,construction,output,operation_error|{assert_eq!(work.prepare(1).unwrap(),Some(true));work.cancel().unwrap();let loan=construction.unwrap();*output=Some(41);let _=operation_error;loan.begin(crate::RetirementDemand::default()).unwrap();loan.quote().unwrap()})?;Ok::<_,ValueError>(())}))).unwrap()));
  assert!(interrupted.is_err());assert_eq!(result.output,Some(41));assert!(result.operation.is_none());assert!(frame.as_mut().unwrap().settle_pair().is_err());physical.with_loan(&mut (),|physical_loan|original.with_work_recipient(&mut work_loan,|control|control.with_construction_recipient(physical_loan,|control|{control.recover_encoding_receiver(frame.as_mut().unwrap(),|_,_,construction|{let loan=construction.unwrap();let error=loan.recover_grant().unwrap().unwrap().unwrap_err();loan.finish().unwrap();result.operation=Some(error)})?;Ok::<_,ValueError>(())}))).unwrap().unwrap();assert_eq!(result.operation.as_ref().unwrap().message.as_ref(),"actual construction refusal");let frame=frame.as_mut().unwrap();frame.settle_pair().unwrap();assert!(frame.take_allocation_failure().unwrap().is_empty());original.release_encoding_receiver(frame).unwrap();drop(work_loan);assert!(!issuer.has_live_actions());assert!(physical.into_recipient().is_ok());
 }
 #[test]
 fn invalid_pause_returns_original_control_state_and_refusal(){
  struct Original;impl crate::NativeAllocationAuthorityPort for Original{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{None}fn receive<'turn,'source>(&mut self,_:&mut (),_:crate::NativeAllocationEvent)->Result<(),crate::NativeAllocationRefusal> where 'source:'turn{Ok(())}fn recover<'turn,'source>(&mut self,_:&mut (),_:crate::NativeAllocationEvent)->Result<Option<Result<(),crate::NativeAllocationRefusal>>,ValueError> where 'source:'turn{Ok(None)}}
  let mut authority=crate::NativeAllocationAuthority::new(Original);authority.with_send_loan(&mut (),|loan|{let mut observe=|_:NativeEncodeProgress|true;let mut work=|_:usize,_:usize|Ok(());let mut recipient=NativeEncodeRetirementRecipient::new();let mut control=NativeEncodeControl::new_scoped_source(100,7,&mut observe,loan,&mut work);control.install_retirement_recipient(&mut recipient).unwrap();control.ledger.as_mut().unwrap().commit(NativeAllocationRequest{bytes:13,owned_bytes:0,next_owned_bytes:13,maximum_bytes:100}).unwrap();control.completed=2;control.total=5;control.started=true;control.stage=9;control.depth=3;let pending=None;let source=crate::OriginalSourceTurnIssuer::new();let turn=source.issue().unwrap();let(error,state)=control.pause_scoped_source(turn.witness(),&pending).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!((state.maximum_bytes,state.owned_bytes(),state.completed,state.total,state.stage,state.depth),(100,13,2,5,9,3));assert!(state.receiving&&state.started&&matches!(state.allocation,allocation::NativeEncodeAllocationBinding::Local)&&state.scoped_allocation.is_some()&&state.work.is_some()&&state.retirement.is_some());assert_eq!(state.source_depth,Some(7));assert_eq!(state.scope_maximum,None);}).unwrap();assert!(authority.into_port().is_ok());

 }
}


#[cfg(test)]
#[path="🧪️tests/🚦️turn-ceiling/🦀️.rs"]
mod turn_ceiling_tests;

