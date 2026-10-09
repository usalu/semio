//! 🛬️ Cumulative ownership admission shared by native parsers and typed field construction.
use crate::{ValueError, ValueRefusalKind, ErasedSnapshotRetirement};
#[path="🔭️observer/🦀️.rs"]
mod observer;
#[path="📏️maximum/🦀️.rs"]
mod maximum;
#[cfg(test)]
#[path="📏️maximum/🧪️tests/🦀️.rs"]
mod maximum_tests;
#[path="🛂️allocation/🦀️.rs"]
pub mod allocation;
pub use allocation::{NativeDecodeAllocation,NativeDecodeAllocationPort,NativeForwardedDecodeControl,NativeForwardedDecodeContinuation};
#[path = "🫴️recipient/🦀️.rs"]
mod recipient;
pub use recipient::NativeDecodeRetirementRecipient;
#[path="🫴️recipient/🔁️continuation/🦀️.rs"]
mod retirement_continuation;
pub use retirement_continuation::{NativeRetirementDecodeContinuation,NativeDetachedRetirementDecodeContinuation};

/// ⏱️ Decoder work units and admitted owned bytes at one cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧵️ An opaque consuming receipt retains this operation's admission across callback lifetimes.
pub struct NativeDecodeContinuation { receiving:bool, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, stage:u64 }

/// 🧮️ One caller-owned budget persists from input scanning through final typed construction.
pub struct NativeDecodeControl<'a> { scope_maximum:Option<usize>, allocation:allocation::Binding<'a>, receiving:bool, retirement:Option<&'a mut NativeDecodeRetirementRecipient>, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool }
struct StageScope<'owner,'control>{control:&'owner mut NativeDecodeControl<'control>,parent:(usize,usize,bool,u64)}
impl Drop for StageScope<'_, '_>{fn drop(&mut self){if self.control.stage!=self.parent.3{self.control.completed=self.parent.0;self.control.total=self.parent.1;self.control.started=self.parent.2;self.control.stage=self.parent.3;}}}

impl<'a> NativeDecodeControl<'a> {

    /// 🫴️ Loans the original decoder's exact remaining capacity to a foreign construction phase.
    pub fn with_encoding_receiver<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(crate::native_encoding::NativeEncodeProgress)->bool,&mut crate::native_encoding::NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>{self.scoped_stage(|original|crate::value::native_receiving::scoped_encoding_receiver!(original,operation,NativeDecodeAllocation,NativeDecodeProgress))}
    /// 🔭️ Reborrows the complete caller owner and composes observers without resetting its receipt.
    pub fn scoped_observer<T,E>(&mut self,observer:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{observer::run(self,observer,operation)}

    /// 👓️ Reports the original return slot without moving its pending physical owner.
    pub fn has_retirement_owner(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_owner())}
    /// ♻️ Drains one original return turn after forwarding its exact physical capacity demand.
    pub fn close_retirement_recipient(&mut self,grant:crate::retained_clone::RetainedCloneGrant)->Result<crate::retained_clone::RetainedCloneStep,ValueError>{
        let recipient=self.retirement.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"decoder has no original retirement recipient"))?;
        if recipient.terminal_is_empty(){return Ok(crate::retained_clone::RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0{return Ok(crate::retained_clone::RetainedCloneStep::Progress(Default::default()))}
        if recipient.next_depth_demand()?>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"decoder retirement exceeds original depth grant"))}
        let copy=recipient.next_copy_byte_demand()?;let capacity=recipient.next_capacity_byte_demand(grant.maximum_copy_bytes)?;let release=recipient.next_release_byte_demand()?;
        if copy>grant.maximum_copy_bytes||capacity>grant.maximum_capacity_bytes||release>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"decoder retirement exceeds original physical grant"))}
        self.checkpoint()?;self.charge(capacity)?;self.retirement.as_mut().unwrap().close_step(grant)
    }

    /// 🫴️ Installs one explicit caller recipient before any retained decoder ownership is created.
    pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<(),ValueError>{
        if self.retirement.is_some()||!recipient.terminal_is_empty(){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native decode requires one empty explicit retirement recipient"));}
        self.retirement=Some(recipient);Ok(())
    }
    /// 🪑️ Pre-admits the return slot and exact immediate wrapper allocations, preserving every refusal owner.
    pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn crate::ErasedSnapshotRetirement>>))->Result<T,E>{
        if self.retirement.as_ref().is_none_or(|recipient|recipient.reserved||recipient.owner.is_some()){return Err(E::from(ValueError::new(ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot")));}
        self.charge(wrapper_bytes)?;self.checkpoint()?;
        self.retirement.as_mut().unwrap().reserved=true;
        let(result,owner)=operation(self);
        let recipient=self.retirement.as_mut().unwrap();*recipient.owner=owner;recipient.reserved=false;
        result
    }


    /// ⏸️ Transfers cumulative admission without retaining the current hop's callback.
    pub fn pause(self)->Result<NativeDecodeContinuation,ValueError>{
        if matches!(self.allocation,allocation::Binding::Forwarded(_))||self.retirement.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native decode continuation requires original allocation and retirement authority"));}
        self.continuation()
    }
    fn continuation(&self)->Result<NativeDecodeContinuation,ValueError>{
        if self.depth!=0||self.owned_bytes>self.maximum_bytes||(self.total!=0&&self.completed>self.total){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native decode continuation lacks valid cumulative accounting"));}
        Ok(NativeDecodeContinuation{receiving:self.receiving,maximum_bytes:self.maximum_bytes,owned_bytes:self.owned_bytes,completed:self.completed,total:self.total,stage:self.stage})
    }
    /// ▶️ Rebinds a moved operation receipt to the current hop's cancellation callback.
    pub fn resume(receipt:NativeDecodeContinuation,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Result<Self,ValueError>{
        if receipt.owned_bytes>receipt.maximum_bytes||(receipt.total!=0&&receipt.completed>receipt.total){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native decode continuation has invalid cumulative accounting"));}
        Ok(Self{scope_maximum:None,allocation:allocation::Binding::Local,receiving:receipt.receiving,retirement:None,maximum_bytes:receipt.maximum_bytes,owned_bytes:receipt.owned_bytes,completed:receipt.completed,total:receipt.total,started:false,stage:receipt.stage,depth:0,callback})
    }

    /// 🚦️ Binds the explicit allocation ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self { Self{scope_maximum:None,allocation:allocation::Binding::Local,receiving:false,retirement:None,maximum_bytes,owned_bytes:0,completed:0,total:0,started:false,stage:0,depth:0,callback} }
    /// 🪆️ Bounds recursive typed construction independently of physical input parsing.
    pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        if self.depth>=maximum{return Err(E::from(ValueError::new(ValueRefusalKind::DepthLimit, "native typed construction exceeds depth limit")))}
        self.depth+=1;let result=operation(self);self.depth-=1;result
    }
    /// 📊️ Returns the cumulative ownership already admitted for this operation.
    pub fn owned_bytes(&self)->usize { self.owned_bytes }
    /// 🛂️ Starts with zero storage authority; each original retained turn must supply its grant.
    pub fn new_retained(callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self{let mut control=Self::new(0,callback);control.receiving=true;control}
    /// 🎟️ Binds this original turn's allocation authority without resetting cumulative ownership.
    pub fn admit_turn_capacity(&mut self,maximum_capacity_bytes:usize)->Result<(),ValueError>{let next=self.owned_bytes.checked_add(maximum_capacity_bytes).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native turn capacity exceeds address space"))?;self.maximum_bytes=if self.receiving{self.scope_maximum.map_or(next,|maximum|maximum.min(next))}else{self.maximum_bytes.min(next)};Ok(())}

    /// 📏️ Returns the caller's complete native allocation allowance.
    pub fn maximum_bytes(&self)->usize { self.maximum_bytes }
    /// 🛑️ Checks cancellation before an explicitly owned expensive operation.
    pub fn checkpoint(&mut self)->Result<(),ValueError> { let owned_bytes=self.owned_bytes();if(self.callback)(NativeDecodeProgress{completed:self.completed,total:self.total,owned_bytes}){self.started=true;Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native decoding canceled"))} }
    /// 🧭️ Begins a known stage workload while retaining every previously admitted byte.
    pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{self.stage=self.stage.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native decoding stage overflow"))?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🪆️ Restores a parent workload after a child begins its own stage, preserving cumulative ownership.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        let parent=(self.completed,self.total,self.started,self.stage);let mut scope=StageScope{control:self,parent};operation(&mut *scope.control)
    }
    /// 📐️ Restricts one owned decoder stage to its domain ceiling, retaining cumulative charges afterward.
    pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{maximum::run(self,maximum,operation)}
    /// 📍️ Advances consumed stage units and checks cancellation across256-unit boundaries.
    pub fn advance(&mut self,units:usize)->Result<(),ValueError>{if !self.started{self.checkpoint()?;}let previous=self.completed;self.completed=self.completed.checked_add(units).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native decoding work overflow"))?;if self.total!=0&&self.completed>self.total{return Err(ValueError::new(ValueRefusalKind::WorkLimit, "native decoding exceeded declared stage workload"));}if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}Ok(())}
    /// 🔢️ Advances decoder work and publishes a checkpoint every256units.
    pub fn step(&mut self)->Result<(),ValueError> {self.advance(1)}
    /// 📦️ Admits cumulative storage before a caller copies or reserves it.
    pub fn charge(&mut self,bytes:usize)->Result<(),ValueError> { let next=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native decoding ownership exceeds caller limit"))?;if !self.started||bytes>65536 {self.checkpoint()?;}if let allocation::Binding::Forwarded(port)=&mut self.allocation{port(NativeDecodeAllocation{bytes,owned_bytes:self.owned_bytes,next_owned_bytes:next,maximum_bytes:self.maximum_bytes})?;}self.owned_bytes=next;Ok(()) }
    /// 🗂️ Reserves typed collection slots after overflow and caller-bound admission.
    pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native decoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native decoding collection allocation failed"))?;Ok(output) }
    fn copy_checkpoint(&mut self,completed:usize,total:usize)->Result<(),ValueError>{let owned_bytes=self.owned_bytes();if(self.callback)(NativeDecodeProgress{completed,total,owned_bytes}){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native decoding canceled"))}}
    /// 🔎️ Validates borrowed UTF-8 in bounded spans without owning a second text buffer.
    pub fn borrow_text<'text>(&mut self,bytes:&'text[u8])->Result<&'text str,ValueError>{
        self.copy_checkpoint(0,bytes.len())?;let mut position=0;
        while position<bytes.len(){
            let mut end=position.saturating_add(65536).min(bytes.len());
            loop{match std::str::from_utf8(&bytes[position..end]){Ok(_)=>break,Err(error) if error.error_len().is_none()&&end<bytes.len()=>{end+=1;},Err(_)=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "invalid native UTF-8"))}}
            position=end;self.copy_checkpoint(position,bytes.len())?;
        }
        Ok(unsafe{std::str::from_utf8_unchecked(bytes)})
    }
    /// 🔤️ Copies owned UTF-8 in cancellable64KiB spans, preserving the outer work stage.
    pub fn copy_text(&mut self,text:&str)->Result<String,ValueError> {
        self.charge(text.len())?;self.copy_checkpoint(0,text.len())?;
        let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native decoding text allocation failed"))?;
        let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);position=end;self.copy_checkpoint(position,text.len())?;}Ok(output)
    }
    /// 🧵️ Preserves the caller's original UTF8 destination and each cancellable copied prefix.
    pub fn copy_text_into(&mut self,text:&str,output:&mut String)->Result<(),ValueError>{
        if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native copied text slot must be empty"))}
        self.charge(text.len())?;self.copy_checkpoint(0,text.len())?;
        output.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native decoding text allocation failed"))?;
        let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);position=end;self.copy_checkpoint(position,text.len())?;}Ok(())
    }
    /// 🫴️ Copies into the caller's original slot so cancellation retains every born buffer.
    pub fn copy_bytes_into(&mut self,bytes:&[u8],output:&mut Vec<u8>)->Result<(),ValueError>{
        if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native copied buffer slot must be empty"))}
        self.charge(bytes.len())?;self.copy_checkpoint(0,bytes.len())?;
        output.try_reserve_exact(bytes.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native decoding collection allocation failed"))?;
        for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);self.copy_checkpoint(output.len(),bytes.len())?;}Ok(())
    }
    /// 🧬️ Copies intrinsic octets in cancellable64KiB spans after complete ownership admission.
    pub fn copy_bytes(&mut self,bytes:&[u8])->Result<Vec<u8>,ValueError> {
        self.charge(bytes.len())?;self.copy_checkpoint(0,bytes.len())?;
        let mut output=Vec::new();output.try_reserve_exact(bytes.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native decoding collection allocation failed"))?;
        for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);self.copy_checkpoint(output.len(),bytes.len())?;}Ok(output)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
