//! 🛬️ Cumulative ownership admission shared by native parsers and typed field construction.
use crate::{ValueError, ValueRefusalKind, ErasedSnapshotRetirement};
#[path="🔭️observer/🦀️.rs"]
mod observer;
#[path="🎟️work/🦀️.rs"]
mod work;
pub use work::{NativeDecodeWorkIdentity,NativeDecodeWorkIssuer,NativeDecodeWorkLoan,NativeDecodeWorkRecipient};
#[path="📏️maximum/🦀️.rs"]
mod maximum;
#[path="🛂️allocation/🦀️.rs"]
pub mod allocation;
pub use allocation::{NativeDecodeAllocation,NativeDecodeAllocationEvent,NativeDecodeAllocationOutcome,NativeDecodeAllocationProgress,NativeDecodeAllocationPhase,NativeDecodeAllocationRecovery,NativeDecodeAllocationPort,NativeDecodeAllocationLoan,NativeDecodeAllocationIssuer,NativeForwardedDecodeControl,NativeForwardedDecodeContinuation,NativeForwardedDecodeFrame};
#[path="🏭️construction/🦀️.rs"]
pub mod construction;
pub use construction::{NativeConstructionRecipientIssuer,NativeConstructionRecipientLoan,NativeBackingTurn,NativeReceivingEffect,NativeReceivingEffectError,NativeReceivingEffectFailure,NativeReceivingEffectPhase,NativeSharedAdmission,NativeDecodeConstructionRecipient,NativeOwnedDecodeControl,NativeConstructionStepResult,NativeConstructionStepOutcome,NativeConstructionStepFailure,OriginalReceivingPrimary};
#[path = "🫴️recipient/🦀️.rs"]
mod recipient;
#[path="🧬️child/🦀️.rs"]
mod child;
pub use recipient::NativeDecodeRetirementRecipient;
#[path="🫴️recipient/🔁️continuation/🦀️.rs"]
mod retirement_continuation;
pub use retirement_continuation::{NativeRetirementDecodeContinuation,NativeDetachedRetirementDecodeContinuation,NativeDecodeCallFailure};
#[path="♻️retirement/🦀️.rs"]mod retirement_loan;
pub use retirement_loan::{NativeRetirementDecodeControl,NativeDecodeCloseControl,NativePreparedDecodeControl};

/// ⏱️ Decoder work units and admitted owned bytes at one cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧵️ An opaque consuming receipt retains this operation's admission across callback lifetimes.
#[derive(Debug)]
pub struct NativeDecodeContinuation { actual_work:u64, reserved_work:u64, reserved_issuer:Option<u64>, reserved_action:Option<u64>, work_quote_pending:bool, ledger:u64, receiving:bool, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, stage:u64 }

impl Drop for NativeDecodeContinuation{fn drop(&mut self){assert!(std::thread::panicking()||self.reserved_issuer.is_none()&&self.reserved_action.is_none(),"native decoder continuation retains original pending work")}}

static LEDGERS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
fn next_ledger()->u64{LEDGERS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|ledger|ledger.checked_add(1)).expect("native decode ledger identity space exhausted")}

/// 🧮️ One caller-owned budget persists from input scanning through final typed construction.
pub struct NativeDecodeControl<'a> { work:work::Work<'a>, ledger:u64, scope_maximum:Option<usize>, construction:Option<&'a mut NativeConstructionRecipientLoan<'a>>, allocation:allocation::Binding<'a>, receiving:bool, retirement:Option<&'a mut NativeDecodeRetirementRecipient>, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool }
struct StageScope<'owner,'control>{control:&'owner mut NativeDecodeControl<'control>,parent:(usize,usize,bool,u64)}
impl Drop for StageScope<'_, '_>{fn drop(&mut self){if self.control.stage!=self.parent.3{self.control.completed=self.parent.0;self.control.total=self.parent.1;self.control.started=self.parent.2;self.control.stage=self.parent.3;}}}

impl std::fmt::Debug for NativeDecodeControl<'_>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("NativeDecodeControl").field("ledger",&self.ledger).field("actual_work",&self.work.actual).field("pending_work",&self.work.reserved_issuer.is_some()).finish()}}
impl Drop for NativeDecodeControl<'_>{fn drop(&mut self){assert!(std::thread::panicking()||self.work.reserved_issuer.is_none()&&self.work.reserved_action.is_none(),"native decoder control retains original pending work")}}

impl<'a> NativeDecodeControl<'a> {

    /// 🫴️ Loans the original decoder's exact remaining capacity to a foreign construction phase.
    pub fn with_encoding_receiver<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(crate::native_encoding::NativeEncodeProgress)->bool,&mut crate::native_encoding::NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>{self.scoped_stage(|original|semio_framework_value_native_encoding::scoped_encoding_receiver!(original,operation,NativeDecodeAllocation,NativeDecodeAllocationEvent,NativeDecodeProgress))}
    /// 🔭️ Reborrows the complete caller owner and composes observers without resetting its receipt.
    pub fn scoped_observer<T,E>(&mut self,observer:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{observer::run(self,observer,operation)}

    /// 🪪️ Names the one cumulative ownership ledger shared by this decoder and every scoped child.
    pub fn ledger_identity(&self)->u64{self.ledger}
    /// 🔑️ Names the one cancellation callback shared by this decoder and every scoped child.
    pub fn callback_identity(&self)->usize{std::ptr::from_ref(&*self.callback).cast::<()>() as usize}
    /// 👤️ Names the installed explicit recipient, if any, without borrowing its owner.
    pub fn retirement_recipient_identity(&self)->Option<u64>{self.retirement.as_ref().map(|recipient|recipient.identity)}
    /// 🧬️ Lends the same ledger, callback and stage to a nested decoder that retires into its own explicit recipient.
    pub fn with_retirement_child<T,E:From<ValueError>>(&mut self,recipient:&mut NativeDecodeRetirementRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{child::run(self,recipient,operation)}
    /// 👓️ Reports the original return slot without moving its pending physical owner.
    pub fn has_retirement_owner(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_owner())}
    /// 🧺️ Observes spare failure custody separately from a pending returned owner.
    pub fn has_retirement_failure_storage(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_failure_storage())}
    /// ♻️ Drains one original return turn after forwarding its exact physical capacity demand.
    pub fn close_retirement_recipient(&mut self,grant:crate::retained_clone::RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkOutcome{result:Ok(crate::retained_clone::RetainedCloneStep::Progress(Default::default())),work:0};}match self.with_retirement::<_,ValueError>(grant,|original|Ok(original.close_retirement_recipient(grant))){Ok(turn)=>turn,Err(error)=>crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}}

    /// 🆕️ Installs one explicit caller recipient before any retained decoder ownership is created.
    pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<(),ValueError>{
        if self.retirement.is_some()||!recipient.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decode requires one empty explicit retirement recipient"));}
        self.retirement=Some(recipient);Ok(())
    }
    /// 🪑️ Pre-admits the return slot and exact immediate wrapper allocations, preserving every refusal owner.
    pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{
        if self.retirement.as_ref().is_none_or(|recipient|recipient.reserved||recipient.owner.is_some()){return Err(E::from(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot")));}
        self.checkpoint()?;self.charge(wrapper_bytes)?;
        self.retirement.as_mut().unwrap().reserved=true;
        let(result,owner)=operation(self);
        let recipient=self.retirement.as_mut().unwrap();*recipient.owner=owner;recipient.reserved=false;
        result
    }


    /// ⏸️ Transfers cumulative admission without retaining the current hop's callback.
    pub fn pause(mut self)->Result<NativeDecodeContinuation,(ValueError,Self)>{
        if matches!(self.allocation,allocation::Binding::Forwarded(_))||self.retirement.is_some()||self.construction.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native decode continuation requires original allocation and retirement authority"),self));}
        match self.continuation(){Ok(receipt)=>{self.work.reserved=0;self.work.reserved_issuer=None;self.work.reserved_action=None;self.work.quote_pending=false;Ok(receipt)},Err(error)=>Err((error,self))}
    }
    fn continuation(&self)->Result<NativeDecodeContinuation,ValueError>{
        if self.work.recipient.is_some()||(self.work.reserved_issuer.is_some()!=self.work.reserved_action.is_some())||(self.work.reserved_issuer.is_none()&&(self.work.reserved!=0||self.work.quote_pending))||self.depth!=0||(self.total!=0&&self.completed>self.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode continuation lacks valid cumulative accounting"));}
        Ok(NativeDecodeContinuation{actual_work:self.work.actual,reserved_work:self.work.reserved,reserved_issuer:self.work.reserved_issuer,reserved_action:self.work.reserved_action,work_quote_pending:self.work.quote_pending,ledger:self.ledger,receiving:self.receiving,maximum_bytes:self.maximum_bytes,owned_bytes:self.owned_bytes,completed:self.completed,total:self.total,stage:self.stage})
    }
    /// ▶️ Rebinds a moved operation receipt to the current hop's cancellation callback.
    pub fn resume(receipt:NativeDecodeContinuation,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Result<Self,(ValueError,NativeDecodeContinuation)>{
        if (receipt.reserved_issuer.is_some()!=receipt.reserved_action.is_some())||(receipt.reserved_issuer.is_none()&&(receipt.reserved_work!=0||receipt.work_quote_pending))||(receipt.total!=0&&receipt.completed>receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode continuation has invalid cumulative accounting"),receipt));}
        Ok(Self::resume_validated(receipt,callback))
    }
    fn resume_validated(mut receipt:NativeDecodeContinuation,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self{let mut work=work::Work::empty(receipt.actual_work);work.reserved=receipt.reserved_work;work.reserved_issuer=receipt.reserved_issuer;work.reserved_action=receipt.reserved_action;work.quote_pending=receipt.work_quote_pending;receipt.reserved_work=0;receipt.reserved_issuer=None;receipt.reserved_action=None;receipt.work_quote_pending=false;Self{work,ledger:receipt.ledger,scope_maximum:None,construction:None,allocation:allocation::Binding::Local,receiving:receipt.receiving,retirement:None,maximum_bytes:receipt.maximum_bytes,owned_bytes:receipt.owned_bytes,completed:receipt.completed,total:receipt.total,started:false,stage:receipt.stage,depth:0,callback}}

    /// 🚦️ Binds the explicit allocation ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self { Self{work:work::Work::empty(0),ledger:next_ledger(),scope_maximum:None,construction:None,allocation:allocation::Binding::Local,receiving:false,retirement:None,maximum_bytes,owned_bytes:0,completed:0,total:0,started:false,stage:0,depth:0,callback} }
    /// 🪆️ Bounds recursive typed construction independently of physical input parsing.
    pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        if self.depth>=maximum{return Err(E::from(ValueError::literal(ValueRefusalKind::DepthLimit, "native typed construction exceeds depth limit")))}
        self.depth+=1;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(self)));self.depth-=1;match result{Ok(result)=>result,Err(payload)=>std::panic::resume_unwind(payload)}
    }
    /// 📊️ Returns the cumulative ownership already admitted for this operation.
    pub fn owned_bytes(&self)->usize { self.owned_bytes }
    /// 🛂️ Starts with zero storage authority; each original retained turn must supply its grant.
    pub fn new_retained(callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self{let mut control=Self::new(0,callback);control.receiving=true;control}
    /// 🎟️ Binds this original turn's allocation authority without resetting cumulative ownership.
    pub fn admit_turn_capacity(&mut self,maximum_capacity_bytes:usize)->Result<(),ValueError>{let whole=if self.receiving{self.scope_maximum}else{Some(self.maximum_bytes)};let next=semio_framework_value_native_encoding::native_receiving::turn::maximum(self.owned_bytes,maximum_capacity_bytes,whole)?;self.maximum_bytes=if self.receiving{self.scope_maximum.map_or(next,|maximum|maximum.min(next))}else{self.maximum_bytes.min(next)};Ok(())}

    /// 📏️ Returns the caller's complete native allocation allowance.
    pub fn maximum_bytes(&self)->usize { self.maximum_bytes }
    /// 🛑️ Checks cancellation before an explicitly owned expensive operation.
    pub fn checkpoint(&mut self)->Result<(),ValueError> { let owned_bytes=self.owned_bytes();if(self.callback)(NativeDecodeProgress{completed:self.completed,total:self.total,owned_bytes}){self.started=true;Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native decoding canceled"))} }
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
    pub fn charge(&mut self,bytes:usize)->Result<(),ValueError> { let next=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "native decoding ownership exceeds caller limit"))?;if !self.started||bytes>65536 {self.checkpoint()?;}if matches!(&self.allocation,allocation::Binding::Forwarded(_)){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native allocation requires an actual quoted effect"))}self.owned_bytes=next;Ok(()) }
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

#[path="🔢️number/🦀️.rs"]
mod number;
pub use number::OriginalFloatWord;
