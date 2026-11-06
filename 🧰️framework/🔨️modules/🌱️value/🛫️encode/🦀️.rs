//! 🛫️ Cumulative caller admission for owned native construction and physical output.
use crate::{ValueError, ValueRefusalKind,ErasedSnapshotRetirement};
#[path="🔭️observer/🦀️.rs"]
mod observer;
#[path="🎟️action/🦀️.rs"]
mod action;
pub use action::{NativeEncodeWorkRecipient,NativeEncodeWorkIssuer,NativeEncodeWorkLoan};
#[path="🎟️effect/🦀️.rs"]
mod effect;
pub use effect::{native_encode_allocation_outcome_status,NativeEncodeAllocationOutcomeStatus,native_encode_effect_settlement_status,NativeEncodeEffectFailure,NativeEncodeEffectProgress,NativeEncodeEffectState,NativeEncodeEffectSettlementStatus,NativeEncodeEffectSettlement,NativeEncodeEffectPostPhase,NativeEncodeEffectReceipt,NativeEncodeEffectCancellation,NativeEncodeEffectRecovery};
#[path="🏭️construction/🦀️.rs"]
mod construction;
pub use construction::{NativeEncodeConstructionRecipient,NativeEncodeConstructionIssuer,NativeEncodeConstructionLoan};
#[path="📏️maximum/🦀️.rs"]
mod maximum;
#[path="🛂️allocation/🦀️.rs"]
pub mod allocation;
pub use allocation::{NativeEncodeAllocation,NativeEncodeAllocationEvent,NativeEncodeAllocationOutcome,NativeEncodeAllocationProgress,NativeEncodeAllocationPhase,NativeEncodeAllocationRecovery,NativeEncodeAllocationPort,NativeEncodeAllocationLoan,NativeEncodeAllocationIssuer,NativeEncodeScopedAllocationLoan,NativeForwardedEncodeControl,NativeForwardedEncodeContinuation,NativeForwardedEncodeReceipt};

#[path="🫴️recipient/🦀️.rs"]
mod recipient;
pub use recipient::NativeEncodeRetirementRecipient;
#[path="🫴️recipient/🔁️continuation/🦀️.rs"]
mod retirement_continuation;
pub use retirement_continuation::{NativeRetirementEncodeContinuation,NativeDetachedRetirementEncodeContinuation};

/// ⏱️ Known encoding work and cumulative owned storage at a cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeEncodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧾️ Retains a native producer failure independently from its original work-post refusal.
#[derive(Debug)]
pub struct NativeEncodeOperationFailure { pub primary:Option<ValueError>, pub work_refusal:Option<ValueError> }
impl From<ValueError> for NativeEncodeOperationFailure{fn from(error:ValueError)->Self{Self{primary:Some(error),work_refusal:None}}}
impl NativeEncodeOperationFailure{
 /// 🧾️ Preserves the actual physical prefix on each independently retained failure.
 pub fn with_retained_progress(mut self,progress:crate::RetainedCloneProgress)->Self{self.primary=self.primary.map(|error|error.with_retained_progress(progress));self.work_refusal=self.work_refusal.map(|error|error.with_retained_progress(progress));self}
}
impl crate::retirement::RetireOwned for NativeEncodeOperationFailure{
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{use crate::retirement::*;sequence(vec![deferred(self.primary),deferred(self.work_refusal)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use crate::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.primary),deferred_birth_bytes_for(&self.work_refusal)])}
 fn controlled_retirement_supported()->bool{true}
}

/// 🧵️ An opaque consuming receipt retains this operation's admission across callback lifetimes.
#[derive(Debug)]
pub struct NativeEncodeContinuation { actual_work:u64, receiving:bool, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, stage:u64 }
/// 🧵️ Scoped Source encoding retains only cumulative admission and stage data between actual caller turns.
#[derive(Debug)]
pub struct NativeScopedEncodeContinuation{receipt:NativeEncodeContinuation,allocation_identity:u64,recipient_identity:u64}
/// 🧾️ Retains every scalar control field when a scoped pause cannot be qualified.
#[derive(Debug)]
pub struct NativeScopedEncodePauseFailure{scope_maximum:Option<usize>,receiving:bool,maximum_bytes:usize,owned_bytes:usize,completed:usize,total:usize,started:bool,stage:u64,depth:usize,source_depth:Option<usize>,local_allocation:bool,retirement_present:bool,allocation_present:bool,work_present:bool}
impl NativeScopedEncodePauseFailure{pub fn scope_maximum(&self)->Option<usize>{self.scope_maximum}pub fn receiving(&self)->bool{self.receiving}pub fn maximum_bytes(&self)->usize{self.maximum_bytes}pub fn owned_bytes(&self)->usize{self.owned_bytes}pub fn completed(&self)->usize{self.completed}pub fn total(&self)->usize{self.total}pub fn started(&self)->bool{self.started}pub fn stage(&self)->u64{self.stage}pub fn depth(&self)->usize{self.depth}pub fn source_depth(&self)->Option<usize>{self.source_depth}pub fn local_allocation(&self)->bool{self.local_allocation}pub fn retirement_present(&self)->bool{self.retirement_present}pub fn allocation_present(&self)->bool{self.allocation_present}pub fn work_present(&self)->bool{self.work_present}}
impl NativeScopedEncodeContinuation{
 /// 🔭️ Reports committed actions retained by the original source continuation.
 pub fn actual_work(&self)->u64{self.receipt.actual_work}
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
}
impl crate::retirement::RetireOwned for NativeScopedEncodeContinuation{
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{crate::retirement::deferred(self.receipt)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(crate::retirement::deferred_birth_bytes_for(&self.receipt))}
 fn controlled_retirement_supported()->bool{true}
}
struct EncodeContinuationRetirement(Option<NativeEncodeContinuation>);
impl crate::retirement::RetirementCursor for EncodeContinuationRetirement{
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{Ok(crate::RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}
 fn close_step(&mut self,grant:crate::RetainedCloneGrant)->crate::retirement::RetirementStep{if self.0.is_none(){return crate::retirement::RetirementStep::Complete(0)}if grant.maximum_items==0{return crate::retirement::RetirementStep::BudgetExhausted}if grant.maximum_depth==0{return crate::retirement::RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit,"encoding continuation requires original source depth").into(),0)}self.0.take();crate::retirement::RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,..Default::default()},1)}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.0.is_some()))}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl crate::retirement::RetireOwned for NativeEncodeContinuation{
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{Box::new(EncodeContinuationRetirement(Some(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<EncodeContinuationRetirement>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{0}
}

/// 🧮️ One ownership ceiling persists through typed fields, physical output and its envelope.
pub struct NativeEncodeControl<'a> { scope_maximum:Option<usize>, retirement:Option<&'a mut NativeEncodeRetirementRecipient>, construction:Option<&'a mut dyn NativeEncodeConstructionRecipient>, construction_identity:Option<u64>, receiving:bool, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, source_depth:Option<usize>,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool, allocation:allocation::Binding<'a>,scoped_allocation:Option<&'a mut NativeEncodeScopedAllocationLoan<'a>>,work:Option<&'a mut dyn FnMut(usize,usize)->Result<(),ValueError>>,action:action::Action<'a> }

struct StageScope<'owner,'control>{control:&'owner mut NativeEncodeControl<'control>,parent:(usize,usize,bool,u64)}
impl Drop for StageScope<'_, '_>{fn drop(&mut self){if self.control.stage!=self.parent.3{self.control.completed=self.parent.0;self.control.total=self.parent.1;self.control.started=self.parent.2;self.control.stage=self.parent.3;}}}
struct DepthScope<'owner,'control>{control:&'owner mut NativeEncodeControl<'control>,parent:usize}
impl Drop for DepthScope<'_,'_>{fn drop(&mut self){self.control.depth=self.parent}}

impl<'a> NativeEncodeControl<'a> {

    /// 🫴️ Loans exact remaining authority while every foreign allocation debits this original owner.
    pub fn with_encoding_receiver<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(NativeEncodeProgress)->bool,&mut NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>{if self.work.is_some()||self.action.recipient.is_some()||self.construction.is_some(){return Err(E::from(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"scoped encoding cannot detach its original work port")))}self.scoped_stage(|original|crate::scoped_encoding_receiver!(original,operation,NativeEncodeAllocation,NativeEncodeAllocationEvent,NativeEncodeProgress))}
    /// 🔭️ Reborrows the complete caller owner and composes observers without resetting its receipt.
    pub fn scoped_observer<T,E>(&mut self,observer:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{observer::run(self,observer,operation)}

    /// 🪪️ Names the installed original encoder retirement recipient.
    pub fn retirement_recipient_identity(&self)->Option<u64>{self.retirement.as_ref().map(|recipient|recipient.identity)}
    /// 👓️ Reports the original return slot without moving its pending physical owner.
    pub fn has_retirement_owner(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_owner())}
    /// ♻️ Drains one original return turn after forwarding its exact physical capacity demand.
    pub fn close_retirement_recipient(&mut self,grant:crate::RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{
        let preflight=(||{
            let recipient=self.retirement.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"encoder has no original retirement recipient"))?;
            if recipient.terminal_is_empty()||grant.maximum_items==0{return Ok(false)}
            let d=recipient.next_demand(grant.maximum_copy_bytes)?;
            if d.depth>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"encoder retirement exceeds original depth grant"))}
            if !crate::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"encoder retirement exceeds original physical grant"))}
            self.checkpoint()?;self.charge(d.capacity_bytes)?;Ok(true)
        })();
        match preflight{
            Err(error)=>crate::RetainedWorkOutcome{result:Err(error.into()),work:0},
            Ok(true)=>match self.retirement.as_mut(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0}}.close_step(grant),
            Ok(false)=>crate::RetainedWorkOutcome{result:Ok(if self.retirement.as_ref().is_some_and(|r|r.terminal_is_empty()){crate::RetainedCloneStep::Complete(Default::default())}else{crate::RetainedCloneStep::Progress(Default::default())}),work:0}
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
    pub fn pause(self)->Result<NativeEncodeContinuation,ValueError>{
        if !matches!(self.allocation,allocation::Binding::Local)||self.scoped_allocation.is_some()||self.work.is_some()||self.action.recipient.is_some()||self.action.reserved!=0||self.construction.is_some()||self.retirement.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native encoding requires its original allocation continuation"));}
        self.continuation()
    }
    fn continuation(&self)->Result<NativeEncodeContinuation,ValueError>{
        if self.action.recipient.is_some()||self.action.reserved!=0||self.construction.is_some()||self.depth!=0||(self.total!=0&&self.completed>self.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode continuation lacks valid cumulative accounting"));}
        Ok(NativeEncodeContinuation{actual_work:self.action.actual,receiving:self.receiving,maximum_bytes:self.maximum_bytes,owned_bytes:self.owned_bytes,completed:self.completed,total:self.total,stage:self.stage})
    }
    /// ▶️ Rebinds a moved operation receipt to the current hop's cancellation callback.
    pub fn resume(receipt:NativeEncodeContinuation,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Result<Self,ValueError>{
        if (receipt.total!=0&&receipt.completed>receipt.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode continuation has invalid cumulative accounting"));}
        Ok(Self{scope_maximum:(!receipt.receiving).then_some(receipt.maximum_bytes),retirement:None,construction:None,construction_identity:None,receiving:receipt.receiving,maximum_bytes:receipt.maximum_bytes,owned_bytes:receipt.owned_bytes,completed:receipt.completed,total:receipt.total,started:false,stage:receipt.stage,depth:0,source_depth:None,callback,allocation:allocation::Binding::Local,scoped_allocation:None,work:None,action:action::Action::empty(receipt.actual_work)})
    }

    /// 🚦️ Binds the exact caller ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Self{Self{scope_maximum:Some(maximum_bytes),retirement:None,construction:None,construction_identity:None,receiving:false,maximum_bytes,owned_bytes:0,completed:0,total:0,started:false,stage:0,depth:0,source_depth:None,callback,allocation:allocation::Binding::Local,scoped_allocation:None,work:None,action:action::Action::empty(0)}}
    /// 🎟️ A synchronous Source port admits actual reservations and each work span without observing stage counter deltas.
    pub fn new_scoped_source(maximum_bytes:usize,maximum_depth:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut NativeEncodeScopedAllocationLoan<'a>,work:&'a mut dyn FnMut(usize,usize)->Result<(),ValueError>)->Self{let mut control=Self::new_retained(callback);control.maximum_bytes=maximum_bytes;control.scoped_allocation=Some(allocate);control.work=Some(work);control.source_depth=Some(maximum_depth);control}
    /// ⏸️ Ends the current actual port borrows while preserving the complete cumulative admission receipt.
    pub fn pause_scoped_source(self,pending:&Option<NativeEncodeAllocationProgress>)->Result<NativeScopedEncodeContinuation,(ValueError,Self)>{
        if pending.as_ref().is_some_and(NativeEncodeAllocationProgress::pending){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped encoding retains an unsettled allocation"),self))}
        if !matches!(self.allocation,allocation::Binding::Local)||self.scoped_allocation.is_none()||self.work.is_none()||self.source_depth.is_none()||self.retirement.is_none()||!self.receiving||self.scope_maximum.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"scoped encoding continuation requires its complete original Source ports"),self))}
        let Some(allocation_identity)=self.scoped_allocation.as_ref().map(|loan|loan.identity)else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped allocation issuer is absent"),self))};let Some(recipient_identity)=self.retirement.as_ref().map(|recipient|recipient.identity)else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped retirement recipient is absent"),self))};let receipt=match self.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};Ok(NativeScopedEncodeContinuation{receipt,allocation_identity,recipient_identity})
    }
    /// ▶️ Reborrows the same original allocation issuer and retirement recipient without resetting their ledger.
    pub fn resume_scoped_source(receipt:NativeScopedEncodeContinuation,capacity:usize,depth:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut NativeEncodeScopedAllocationLoan<'a>,work:&'a mut dyn FnMut(usize,usize)->Result<(),ValueError>,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<Self,(NativeScopedEncodeContinuation,ValueError)>{
        if receipt.allocation_identity!=allocate.identity||receipt.recipient_identity!=recipient.identity{return Err((receipt,ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped Source original allocation or retirement identity changed")))}
        if !receipt.receipt.receiving||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total){return Err((receipt,ValueError::literal(ValueRefusalKind::InvariantViolated,"scoped encoding continuation changed its original cumulative receipt")))}
        let maximum=match crate::native_receiving::turn::maximum(receipt.receipt.owned_bytes,capacity,None){Ok(maximum)=>maximum,Err(error)=>return Err((receipt,error))};
        let mut control=Self::resume(receipt.receipt,callback).expect("validated scoped Source encoding receipt");control.maximum_bytes=maximum;control.scope_maximum=None;control.source_depth=Some(depth);control.scoped_allocation=Some(allocate);control.work=Some(work);control.retirement=Some(recipient);Ok(control)
    }
    /// 📏️ Returns the complete allowance and cumulative admitted ownership.
    pub fn maximum_bytes(&self)->usize{self.maximum_bytes}
    /// 📊️ Returns storage admitted across every nested stage.
    pub fn owned_bytes(&self)->usize{self.owned_bytes}
    /// 🛂️ Starts with zero storage authority; each original retained turn must supply its grant.
    pub fn new_retained(callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Self{let mut control=Self::new(0,callback);control.receiving=true;control.scope_maximum=None;control}
    /// 🎟️ Binds this original turn's allocation authority without resetting cumulative ownership.
    pub fn admit_turn_capacity(&mut self,maximum_capacity_bytes:usize)->Result<(),ValueError>{let whole=self.scope_maximum;let next=crate::native_receiving::turn::maximum(self.owned_bytes,maximum_capacity_bytes,whole)?;self.maximum_bytes=whole.map_or(next,|maximum|maximum.min(next));Ok(())}

    /// 🪙️ Consumes the same admission owner under an exact checked source capacity.
    pub fn admit_capacity(mut self,source_bytes:usize,multiples:usize,scaffold_bytes:usize)->Result<Self,(Self,ValueError)>{match source_bytes.checked_mul(multiples).and_then(|bytes|bytes.checked_add(scaffold_bytes)).filter(|bytes|*bytes<=isize::MAX as usize&&*bytes>=self.owned_bytes){Some(maximum)=>{self.maximum_bytes=maximum;if !self.receiving{self.scope_maximum=Some(maximum);}Ok(self)},None=>Err((self,ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encoding capacity exceeds source policy or admitted ownership")))}}
    /// 📐️ Applies a narrower physical stage ceiling without resetting ownership.
    pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{maximum::run(self,maximum,operation)}
    /// 🛑️ Checks cancellation before an expensive operation.
    pub fn checkpoint(&mut self)->Result<(),ValueError>{if(self.callback)(NativeEncodeProgress{completed:self.completed,total:self.total,owned_bytes:self.owned_bytes}){self.started=true;Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native encoding canceled"))}}
    /// 🧭️ Begins an exact workload without resetting admitted ownership.
    pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{self.stage=self.stage.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "native encoding stage overflow"))?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🪆️ Restores the enclosing workload while retaining child allocation charges.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{let parent=(self.completed,self.total,self.started,self.stage);let scope=StageScope{control:self,parent};operation(&mut *scope.control)}
    /// 📏️ Checks iterative nesting against the current depth and original source ceiling.
    pub fn check_additional_depth(&self,additional:usize)->Result<(),ValueError>{let depth=self.depth.checked_add(additional).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"native encoding exceeds depth limit"))?;if self.source_depth.is_some_and(|maximum|depth>maximum){return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"native encoding exceeds depth limit"))}Ok(())}
    /// 🪆️ Temporarily enters iterative nesting under the original source ceiling.
    pub fn scoped_additional_depth<T,E:From<ValueError>>(&mut self,additional:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{self.check_additional_depth(additional).map_err(E::from)?;let parent=self.depth;self.depth=parent+additional;let scope=DepthScope{control:self,parent};operation(&mut *scope.control)}
    /// 🌲️ Bounds borrowed recursive projection independently of the input's depth.
    pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{let maximum=self.source_depth.map_or(maximum,|source|source.min(maximum));if self.depth>=maximum{return Err(E::from(ValueError::literal(ValueRefusalKind::DepthLimit, "native encoding exceeds depth limit")))}self.depth+=1;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(self)));self.depth-=1;match result{Ok(result)=>result,Err(payload)=>std::panic::resume_unwind(payload)}}
    /// 📍️ Publishes interior checkpoints at256-unit boundaries and stage completion.
    pub fn advance(&mut self,units:usize)->Result<(),ValueError>{let source_span=self.action.recipient.is_none();self.advance_progress(units,source_span)}
    fn advance_progress(&mut self,units:usize,source_span:bool)->Result<(),ValueError>{if !self.started{self.checkpoint()?;}let previous=self.completed;let next=self.completed.checked_add(units).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native encoding work overflow"))?;if self.total!=0&&next>self.total{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native encoding exceeds declared workload"))}self.completed=next;if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}if source_span{if let Some(work)=self.work.as_mut(){work(units,self.owned_bytes)?;}}Ok(())}
    /// 🔢️ Advances one explicit owned field or collection item.
    pub fn step(&mut self)->Result<(),ValueError>{self.advance(1)}
    /// 📦️ Bounds local storage; original allocation ports require their quoted actual effect.
    pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{
        if !matches!(self.allocation,allocation::Binding::Local)||self.scoped_allocation.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native original allocation requires its quoted actual effect"))}
        let next=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native encoding ownership exceeds caller limit"))?;
        if !self.started||bytes>65536{self.checkpoint()?;}self.owned_bytes=next;Ok(())
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
            if metered{if let Some(source)=control.work.as_mut(){if let Err(error)=source(chunk.len(),control.owned_bytes){control.cancel_work()?;return Err(error)}}}
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
 fn invalid_pause_returns_original_control_state_and_refusal(){
  let mut observe=|_:NativeEncodeProgress|true;let mut port=|_:NativeEncodeAllocationEvent|Ok(());let mut issuer=NativeEncodeAllocationIssuer::new();let mut allocate=issuer.scoped_loan(&mut port);let mut work=|_:usize,_:usize|Ok(());let mut recipient=NativeEncodeRetirementRecipient::new();let mut control=NativeEncodeControl::new_scoped_source(100,7,&mut observe,&mut allocate,&mut work);control.install_retirement_recipient(&mut recipient).unwrap();control.owned_bytes=13;control.completed=2;control.total=5;control.started=true;control.stage=9;control.depth=3;let pending=None;let(error,state)=control.pause_scoped_source(&pending).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!((state.maximum_bytes,state.owned_bytes,state.completed,state.total,state.stage,state.depth),(100,13,2,5,9,3));assert!(state.receiving&&state.started&&matches!(state.allocation,allocation::Binding::Local)&&state.scoped_allocation.is_some()&&state.work.is_some()&&state.retirement.is_some());assert_eq!(state.source_depth,Some(7));assert_eq!(state.scope_maximum,None);
 }
}


#[cfg(test)]
#[path="🧪️tests/🚦️turn-ceiling/🦀️.rs"]
mod turn_ceiling_tests;

