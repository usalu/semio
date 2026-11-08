//! 🛬️ Cumulative ownership admission shared by native parsers and typed field construction.
use crate::{ValueError, ValueRefusalKind, ErasedSnapshotRetirement};
#[path="🔭️observer/🦀️.rs"]
mod observer;
#[path="🎟️work/🦀️.rs"]
mod work;
pub use work::{OriginalWorkAuthority,OriginalWorkIssuerIdentity,OriginalWorkActionIssuer,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity};
#[path="📏️maximum/🦀️.rs"]
mod maximum;
#[path="🛂️allocation/🦀️.rs"]
pub mod allocation;
pub use crate::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationRefusal,NativeAllocationReply,NativeAllocationReplyComplete};
pub use allocation::{NativeAllocationRequest,NativeAllocationEvent,NativeDecodeAdmissionPhase,NativeDecodeAdmissionProgress,NativeDecodeAllocationOutcome,NativeDecodeAllocationProgress,NativeDecodeAllocationPhase,NativeDecodeAllocationRecovery,NativeSendAllocationLoan,NativeForwardedDecodeControl,NativeForwardedDecodeContinuation,NativeForwardedDecodeFrame};
#[path="🏭️construction/🦀️.rs"]
pub mod construction;
pub use construction::{OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionLoan,OriginalConstructionIssuerIdentity,OriginalConstructionAction,OriginalConstructionPhase,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,NativeBackingTurn,allocate_empty_vec,OriginalEffectAction,OriginalEffectAuthority,OriginalEffectCallbackMode,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,NativeReceivingEffect,NativeReceivingEffectPoll,NativeReceivingEffectError,NativeReceivingEffectFailure,NativeReceivingEffectPhase,NativeSharedAdmission,NativeOwnedDecodeControl,NativeConstructionStepResult,NativeConstructionStepOutcome,NativeConstructionStepFailure,OriginalReceivingPrimary};
#[path = "🫴️recipient/🦀️.rs"]
mod recipient;
#[path="🧬️child/🦀️.rs"]
mod child;
pub use recipient::NativeDecodeRetirementRecipient;
#[path="🫴️recipient/🔁️continuation/🦀️.rs"]
mod retirement_continuation;
pub use retirement_continuation::{NativeRetirementDecodeContinuation,NativeDetachedRetirementDecodeContinuation};
pub use semio_framework_value_call::NativeCallFailure;
#[path="♻️retirement/🦀️.rs"]mod retirement_loan;
pub use retirement_loan::{NativeRetirementDecodeControl,NativeDecodeCloseControl,NativePreparedDecodeControl};

/// ⏱️ Decoder work units and admitted owned bytes at one cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧵️ An opaque consuming receipt retains this operation's admission across callback lifetimes.
#[derive(Debug)]
pub struct NativeDecodeContinuation { work:work::WorkState, ledger:Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>, receiving:bool, maximum_bytes:usize, completed:usize, total:usize, stage:u64 }

impl NativeDecodeContinuation{
 /// 📏️ Observes the admission retained by this same receipt.
 pub fn owned_bytes(&self)->usize{self.ledger.as_ref().expect("original decode ledger present").owned_bytes()}
 /// 🪪️ Names the ledger retained by this same receipt.
 pub fn ledger_identity(&self)->crate::NativeAllocationIdentity{self.ledger.as_ref().expect("original decode ledger present").identity()}
}

impl Drop for NativeDecodeContinuation{fn drop(&mut self){assert!(std::thread::panicking()||self.ledger.as_ref().is_none_or(|ledger|!ledger.is_bound())&&!self.work.has_live_actions(),"native decoder continuation retains original paired ledger or pending work")}}


/// 🧮️ One caller-owned budget persists from input scanning through final typed construction.
pub struct NativeDecodeControl<'a> { work:work::Work<'a>, ledger:Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>, scope_maximum:Option<usize>, construction:Option<&'a mut OriginalConstructionLoan<'a>>, allocation:allocation::Binding<'a>, receiving:bool, retirement:Option<&'a mut NativeDecodeRetirementRecipient>, maximum_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool }
struct StageScope<'owner,'control>{control:&'owner mut NativeDecodeControl<'control>,parent:(usize,usize,bool,u64)}
impl Drop for StageScope<'_, '_>{fn drop(&mut self){if self.control.stage!=self.parent.3{self.control.completed=self.parent.0;self.control.total=self.parent.1;self.control.started=self.parent.2;self.control.stage=self.parent.3;}}}

impl std::fmt::Debug for NativeDecodeControl<'_>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("NativeDecodeControl").field("ledger",&self.ledger).field("actual_work",&self.work.state.actual()).field("pending_work",&self.work.state.has_live_actions()).finish()}}
impl Drop for NativeDecodeControl<'_>{fn drop(&mut self){assert!(std::thread::panicking()||self.ledger.as_ref().is_none_or(|ledger|!ledger.is_bound())&&!self.work.state.has_live_actions(),"native decoder control retains original paired ledger or pending work")}}

impl<'a> NativeDecodeControl<'a> {

    /// 🫴️ Runs a child against both original authorities with caller-owned unwind custody.
    pub fn with_encoding_receiver<T,E>(&mut self,frame:&mut Option<semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverFrame>,result:&mut semio_framework_value_call::NativeCallFailure<T,E>,operation:impl FnOnce(usize,&mut dyn FnMut(crate::native_encoding::NativeEncodeProgress)->bool,&mut dyn NativeSendAllocationLoan,&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverWork<'_,'_,'_>,Option<&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverConstruction<'_,'_>>,&mut Option<T>,&mut Option<E>))->Result<(),ValueError>{
        if frame.is_some()||result.output.is_some()||result.operation.is_some()||result.context.is_some()||result.receipt_refusal.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiver requires empty original custody"))}
        self.checkpoint()?;
        self.scoped_stage(|original|{
            let start=original.owned_bytes();let maximum=original.maximum_bytes.checked_sub(start).ok_or_else(semio_framework_value_native_encoding::native_receiving::invariant)?;
            *frame=Some(semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverFrame::default());let frame=frame.as_mut().expect("original receiver frame installed");
            let forwarded:Option<&dyn NativeSendAllocationLoan>=match &original.allocation{allocation::Binding::Local=>None,allocation::Binding::Forwarded(port)=>Some(&**port)};
            let scoped=None;
            frame.install_pair(original.ledger.as_mut().ok_or_else(semio_framework_value_native_encoding::native_receiving::invariant)?,forwarded,scoped,original.maximum_bytes)?;
            frame.begin_operation()?;
            let ledger=original.ledger.as_mut().ok_or_else(semio_framework_value_native_encoding::native_receiving::invariant)?;
            let forwarded:Option<&mut dyn NativeSendAllocationLoan>=match &mut original.allocation{allocation::Binding::Local=>None,allocation::Binding::Forwarded(port)=>Some(&mut **port)};
            let scoped=None;
            let mut construction=original.construction.as_mut().map(|loan|loan.reborrow());let original_work=&mut original.work;let work_observer=None;let callback=&mut original.callback;let completed=&mut original.completed;let total=&mut original.total;let started=&mut original.started;let stage=&mut original.stage;
            let semio_framework_value_call::NativeCallFailure{output,operation:operation_error,context,receipt_refusal}=result;let mut previous=None::<(usize,usize)>;
            let run=frame.with_original_authorities(ledger,forwarded,scoped,original_work,work_observer,construction.as_mut(),|loan,observed,work,physical|{
                let mut observer=|progress:crate::native_encoding::NativeEncodeProgress|{if context.is_some(){return false}let actual=observed.load(std::sync::atomic::Ordering::Relaxed);let valid=start.checked_add(progress.owned_bytes)==Some(actual)&&(progress.total==0||progress.completed<=progress.total);if !valid{*context=Some(semio_framework_value_native_encoding::native_receiving::invariant());return false}if previous.is_none_or(|(done,all)|all!=progress.total||done>progress.completed){match stage.checked_add(1){Some(next)=>*stage=next,None=>{*context=Some(semio_framework_value_native_encoding::native_receiving::invariant());return false}}}*completed=progress.completed;*total=progress.total;*started=true;previous=Some((progress.completed,progress.total));if (**callback)(NativeDecodeProgress{completed:*completed,total:*total,owned_bytes:actual}){true}else{*context=Some(ValueError::literal(ValueRefusalKind::Canceled,"original native receiving observer canceled"));false}};
                operation(maximum,&mut observer,loan,work,physical,output,operation_error)
            });
            if let Err(error)=run{if context.is_none(){*context=Some(error)}else{*receipt_refusal=Some(error)}}
            if output.is_some()==operation_error.is_some()&&context.is_none(){*context=Some(semio_framework_value_native_encoding::native_receiving::invariant())}Ok(())
        })
    }
    /// 🔁️ Recovers only through the same resident child and original parent authorities.
    pub fn recover_encoding_receiver<R>(&mut self,frame:&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverFrame,operation:impl FnOnce(&mut dyn NativeSendAllocationLoan,&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverWork<'_,'_,'_>,Option<&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverConstruction<'_,'_>>)->R)->Result<R,ValueError>{let ledger=self.ledger.as_mut().ok_or_else(semio_framework_value_native_encoding::native_receiving::invariant)?;let forwarded:Option<&mut dyn NativeSendAllocationLoan>=match &mut self.allocation{allocation::Binding::Local=>None,allocation::Binding::Forwarded(port)=>Some(&mut **port)};let scoped=None;let mut construction=self.construction.as_mut().map(|loan|loan.reborrow());let original_work=&mut self.work;let work_observer=None;frame.with_original_authorities(ledger,forwarded,scoped,original_work,work_observer,construction.as_mut(),|loan,observed,work,physical|{let _=observed;operation(loan,work,physical)})}
    /// 🔓️ Releases the settled receiver to its exact original ledger and authorities.
    pub fn release_encoding_receiver(&mut self,frame:&mut semio_framework_value_native_encoding::native_receiving::NativeEncodingReceiverFrame)->Result<(),ValueError>{let ledger=self.ledger.as_mut().ok_or_else(semio_framework_value_native_encoding::native_receiving::invariant)?;let forwarded:Option<&dyn NativeSendAllocationLoan>=match &self.allocation{allocation::Binding::Local=>None,allocation::Binding::Forwarded(port)=>Some(&**port)};let scoped=None;frame.release_pair(ledger,forwarded,scoped)}
    /// 🔭️ Reborrows the complete caller owner and composes observers without resetting its receipt.
    pub fn scoped_observer<T,E>(&mut self,observer:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{observer::run(self,observer,operation)}

    /// 🪪️ Names the one cumulative ownership ledger shared by this decoder and every scoped child.
    pub fn ledger_identity(&self)->crate::NativeAllocationIdentity{self.ledger.as_ref().expect("original decode ledger present").identity()}
    /// 🔑️ Names the one cancellation callback shared by this decoder and every scoped child.
    pub fn callback_identity(&self)->usize{std::ptr::from_ref(&*self.callback).cast::<()>() as usize}
    /// 👤️ Names the installed explicit recipient, if any, without borrowing its owner.
    pub fn retirement_recipient_identity(&self)->Option<u64>{self.retirement.as_ref().map(|recipient|recipient.funding.identity)}
    /// 🧬️ Lends the same ledger, callback and stage to a nested decoder that retires into its own explicit recipient.
    pub fn with_retirement_child<T,E:From<ValueError>>(&mut self,recipient:&mut NativeDecodeRetirementRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{child::run(self,recipient,operation)}
    /// 👓️ Reports the original return slot without moving its pending physical owner.
    pub fn has_retirement_owner(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_owner())}
    /// 🚧️ Reports retirement work other than idle spare failure custody.
    pub fn has_retirement_pending_close(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_pending_close())}
    /// 🧺️ Observes spare failure custody separately from a pending returned owner.
    pub fn has_retirement_failure_storage(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_failure_storage())}
    /// 🚦️ Observes a resident failure-node birth without admitting another turn.
    pub fn failure_storage_birth_pending(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|!recipient.funding.failure_birth.terminal_is_empty())}
    /// ♻️ Drains one original return turn after forwarding its exact physical capacity demand.
    pub fn close_retirement_recipient(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{let grant=effect.grant();if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Pending{work:0}}match self.with_retirement::<_,ValueError>(grant,|original|Ok(original.close_retirement_recipient(effect))){Ok(poll)=>poll,Err(error)=>crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})}}

    /// 🆕️ Installs one explicit caller recipient before any retained decoder ownership is created.
    pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<(),ValueError>{
        if self.retirement.is_some()||!recipient.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decode requires one empty explicit retirement recipient"));}
        self.retirement=Some(recipient);Ok(())
    }
    /// 🪑️ Pre-admits the return slot and exact immediate wrapper allocations, preserving every refusal owner.
    pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{
        if self.retirement.as_ref().is_none_or(|recipient|recipient.funding.reserved||recipient.owner.is_some()){return Err(E::from(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot")));}
        self.checkpoint()?;self.charge(wrapper_bytes)?;
        self.retirement.as_mut().unwrap().funding.reserved=true;
        let(result,owner)=operation(self);
        let recipient=self.retirement.as_mut().unwrap();*recipient.owner=owner;recipient.funding.reserved=false;
        result
    }


    /// ⏸️ Transfers cumulative admission without retaining the current hop's callback.
    pub fn pause(mut self)->Result<NativeDecodeContinuation,(ValueError,Self)>{
        if matches!(self.allocation,allocation::Binding::Forwarded(_))||self.retirement.is_some()||self.construction.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native decode continuation requires original allocation and retirement authority"),self));}
        match self.continuation(){Ok(receipt)=>{Ok(receipt)},Err(error)=>Err((error,self))}
    }
    fn validate_continuation(&self)->Result<(),ValueError>{
        if self.ledger.as_ref().is_none_or(|ledger|ledger.is_bound())||self.work.recipient.is_some()||self.work.state.has_live_actions()||self.depth!=0||(self.total!=0&&self.completed>self.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode continuation lacks valid cumulative accounting"));}
        Ok(())
    }
    fn continuation(&mut self)->Result<NativeDecodeContinuation,ValueError>{self.validate_continuation()?;
        let receipt=NativeDecodeContinuation{work:self.work.state.take(),ledger:self.ledger.take(),receiving:self.receiving,maximum_bytes:self.maximum_bytes,completed:self.completed,total:self.total,stage:self.stage};Ok(receipt)
    }
    /// ▶️ Rebinds a moved operation receipt to the current hop's cancellation callback.
    pub fn resume(receipt:NativeDecodeContinuation,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Result<Self,(ValueError,NativeDecodeContinuation)>{
        if receipt.ledger.is_none()||(receipt.total!=0&&receipt.completed>receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode continuation has invalid cumulative accounting"),receipt));}
        Ok(Self::resume_validated(receipt,callback))
    }
    fn resume_validated(mut receipt:NativeDecodeContinuation,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self{Self{work:work::Work{state:receipt.work.take(),recipient:None},ledger:receipt.ledger.take(),scope_maximum:None,construction:None,allocation:allocation::Binding::Local,receiving:receipt.receiving,retirement:None,maximum_bytes:receipt.maximum_bytes,completed:receipt.completed,total:receipt.total,started:false,stage:receipt.stage,depth:0,callback}}

    /// 🚦️ Binds the explicit allocation ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self { Self{work:work::Work::empty(),ledger:Some(semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger::new(0)),scope_maximum:None,construction:None,allocation:allocation::Binding::Local,receiving:false,retirement:None,maximum_bytes,completed:0,total:0,started:false,stage:0,depth:0,callback} }
    /// 🪆️ Bounds recursive typed construction independently of physical input parsing.
    pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        if self.depth>=maximum{return Err(E::from(ValueError::literal(ValueRefusalKind::DepthLimit, "native typed construction exceeds depth limit")))}
        self.depth+=1;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(self)));self.depth-=1;match result{Ok(result)=>result,Err(payload)=>std::panic::resume_unwind(payload)}
    }
    /// 📊️ Returns the cumulative ownership already admitted for this operation.
    pub fn owned_bytes(&self)->usize { self.ledger.as_ref().expect("original decode ledger present").owned_bytes() }
    /// 🛂️ Starts with zero storage authority; each original retained turn must supply its grant.
    pub fn new_retained(callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self{let mut control=Self::new(0,callback);control.receiving=true;control}
    /// 🎟️ Binds this original turn's allocation authority without resetting cumulative ownership.
    pub fn admit_turn_capacity(&mut self,maximum_capacity_bytes:usize)->Result<(),ValueError>{let whole=if self.receiving{self.scope_maximum}else{Some(self.maximum_bytes)};let next=semio_framework_value_native_encoding::native_receiving::turn::maximum(self.owned_bytes(),maximum_capacity_bytes,whole)?;self.maximum_bytes=if self.receiving{self.scope_maximum.map_or(next,|maximum|maximum.min(next))}else{self.maximum_bytes.min(next)};Ok(())}

    /// 📏️ Returns the caller's complete native allocation allowance.
    pub fn maximum_bytes(&self)->usize { self.maximum_bytes }
    /// 🛑️ Checks cancellation before an explicitly owned expensive operation.
    pub fn checkpoint(&mut self)->Result<(),ValueError> { if self.ledger.as_ref().is_none_or(|ledger|ledger.is_bound()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation ledger is paired or absent"))}let owned_bytes=self.owned_bytes();if(self.callback)(NativeDecodeProgress{completed:self.completed,total:self.total,owned_bytes}){self.started=true;Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native decoding canceled"))} }
    /// 🧭️ Begins a known stage workload while retaining every previously admitted byte.
    pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{self.stage=self.stage.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "native decoding stage overflow"))?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🎪️ Restores a parent workload after a child begins its own stage, preserving cumulative ownership.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        let parent=(self.completed,self.total,self.started,self.stage);let scope=StageScope{control:self,parent};operation(&mut *scope.control)
    }
    /// 📐️ Restricts one owned decoder stage to its domain ceiling, retaining cumulative charges afterward.
    pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{maximum::run(self,maximum,operation)}
    /// 📍️ Advances consumed stage units and checks cancellation across256-unit boundaries.
    pub fn advance(&mut self,units:usize)->Result<(),ValueError>{if !self.started{self.checkpoint()?;}let previous=self.completed;let next=self.completed.checked_add(units).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native decoding work overflow"))?;if self.total!=0&&next>self.total{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native decoding exceeded declared stage workload"))}self.completed=next;if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}Ok(())}
    /// 🔢️ Advances decoder work and publishes a checkpoint every256units.
    pub fn step(&mut self)->Result<(),ValueError> {self.advance(1)}
    /// 📦️ Admits cumulative storage before a caller copies or reserves it.
    pub fn charge(&mut self,bytes:usize)->Result<(),ValueError> { let prior=self.owned_bytes();let next=prior.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "native decoding ownership exceeds caller limit"))?;if !self.started||bytes>65536 {self.checkpoint()?;}if matches!(&self.allocation,allocation::Binding::Forwarded(_)){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native allocation requires an actual quoted effect"))}self.ledger.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original decode ledger absent"))?.commit(NativeAllocationRequest{bytes,owned_bytes:prior,next_owned_bytes:next,maximum_bytes:self.maximum_bytes})?;Ok(()) }
    /// 🪢️ Reports whether this decoder lends its allocation to a caller-quoted effect instead of charging its own ledger.
    pub fn is_forwarded(&self)->bool{matches!(&self.allocation,allocation::Binding::Forwarded(_))}
    /// 🗂️ Reserves typed collection slots after overflow and caller-bound admission.
    pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "native decoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "native decoding collection allocation failed"))?;Ok(output) }
    fn copy_checkpoint(&mut self,completed:usize,total:usize)->Result<(),ValueError>{let owned_bytes=self.owned_bytes();if(self.callback)(NativeDecodeProgress{completed,total,owned_bytes}){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native decoding canceled"))}}
    /// 🔎️ Validates borrowed UTF-8 in bounded spans without owning a second text buffer.
    pub fn borrow_text<'text>(&mut self,bytes:&'text[u8])->Result<&'text str,ValueError>{
        self.copy_checkpoint(0,bytes.len())?;let mut position=0;
        while position<bytes.len(){
            let mut end=position.saturating_add(65536).min(bytes.len());
            loop{match std::str::from_utf8(&bytes[position..end]){Ok(_)=>break,Err(error) if error.error_len().is_none()&&end<bytes.len()=>{end+=1;},Err(_)=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "invalid native UTF-8"))}}
            position=end;self.copy_checkpoint(position,bytes.len())?;
        }
        Ok(unsafe{std::str::from_utf8_unchecked(bytes)})
    }
    /// 🔤️ Copies owned UTF-8 in cancellable64KiB spans, preserving the outer work stage.
    pub fn copy_text(&mut self,text:&str)->Result<String,ValueError> {
        self.charge(text.len())?;self.copy_checkpoint(0,text.len())?;
        let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "native decoding text allocation failed"))?;
        let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);position=end;self.copy_checkpoint(position,text.len())?;}Ok(output)
    }
    /// 🦜️ Preserves the caller's original UTF8 destination and each cancellable copied prefix.
    pub fn copy_text_into(&mut self,text:&str,output:&mut String)->Result<(),ValueError>{
        if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native copied text slot must be empty"))}
        self.charge(text.len())?;self.copy_checkpoint(0,text.len())?;
        output.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native decoding text allocation failed"))?;
        let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);position=end;self.copy_checkpoint(position,text.len())?;}Ok(())
    }
    /// 🪞️ Copies into the caller's original slot so cancellation retains every born buffer.
    pub fn copy_bytes_into(&mut self,bytes:&[u8],output:&mut Vec<u8>)->Result<(),ValueError>{
        if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native copied buffer slot must be empty"))}
        self.charge(bytes.len())?;self.copy_checkpoint(0,bytes.len())?;
        output.try_reserve_exact(bytes.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native decoding collection allocation failed"))?;
        for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);self.copy_checkpoint(output.len(),bytes.len())?;}Ok(())
    }
    /// 🧬️ Copies intrinsic octets in cancellable64KiB spans after complete ownership admission.
    pub fn copy_bytes(&mut self,bytes:&(impl semio_framework_value_control_contract::ByteReadSource+?Sized))->Result<Vec<u8>,ValueError> {
        self.charge(bytes.byte_len())?;self.copy_checkpoint(0,bytes.byte_len())?;
        let mut output=Vec::new();output.try_reserve_exact(bytes.byte_len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native decoding collection allocation failed"))?;
        let mut fragment=[0;256];let mut offset=0;while offset<bytes.byte_len(){let count=bytes.read_bytes(offset,&mut fragment);if count==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native byte source changed"))}output.extend_from_slice(&fragment[..count]);offset+=count;self.copy_checkpoint(offset,bytes.byte_len())?;}Ok(output)
    }
}




#[path="♻️retirement/🎟️continuation/🦀️.rs"]
mod continuation_retirement;

pub use semio_framework_value_native_number::OriginalFloatWord;
