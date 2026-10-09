//! 🛫️ Cumulative caller admission for owned native construction and physical output.
use crate::{ValueError, ValueRefusalKind,ErasedSnapshotRetirement};
#[path="🔭️observer/🦀️.rs"]
mod observer;
#[path="📏️maximum/🦀️.rs"]
mod maximum;
#[path="🛂️allocation/🦀️.rs"]
pub mod allocation;
pub use allocation::{NativeEncodeAllocation,NativeEncodeAllocationPort,NativeForwardedEncodeControl,NativeForwardedEncodeContinuation};

#[path="🫴️recipient/🦀️.rs"]
mod recipient;
pub use recipient::NativeEncodeRetirementRecipient;
#[path="🫴️recipient/🔁️continuation/🦀️.rs"]
mod retirement_continuation;
pub use retirement_continuation::{NativeRetirementEncodeContinuation,NativeDetachedRetirementEncodeContinuation};

/// ⏱️ Known encoding work and cumulative owned storage at a cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeEncodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧵️ An opaque consuming receipt retains this operation's admission across callback lifetimes.
#[derive(Debug)]
pub struct NativeEncodeContinuation { receiving:bool, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, stage:u64 }

/// 🧮️ One ownership ceiling persists through typed fields, physical output and its envelope.
pub struct NativeEncodeControl<'a> { scope_maximum:Option<usize>, retirement:Option<&'a mut NativeEncodeRetirementRecipient>, receiving:bool, maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool, allocation:allocation::Binding<'a> }

struct StageScope<'owner,'control>{control:&'owner mut NativeEncodeControl<'control>,parent:(usize,usize,bool,u64)}
impl Drop for StageScope<'_, '_>{fn drop(&mut self){if self.control.stage!=self.parent.3{self.control.completed=self.parent.0;self.control.total=self.parent.1;self.control.started=self.parent.2;self.control.stage=self.parent.3;}}}

impl<'a> NativeEncodeControl<'a> {

    /// 🫴️ Loans exact remaining authority while every foreign allocation debits this original owner.
    pub fn with_encoding_receiver<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(NativeEncodeProgress)->bool,&mut NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>{self.scoped_stage(|original|crate::value::native_receiving::scoped_encoding_receiver!(original,operation,NativeEncodeAllocation,NativeEncodeProgress))}
    /// 🔭️ Reborrows the complete caller owner and composes observers without resetting its receipt.
    pub fn scoped_observer<T,E>(&mut self,observer:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{observer::run(self,observer,operation)}

    /// 👓️ Reports the original return slot without moving its pending physical owner.
    pub fn has_retirement_owner(&self)->bool{self.retirement.as_ref().is_some_and(|recipient|recipient.has_owner())}
    /// ♻️ Drains one original return turn after forwarding its exact physical capacity demand.
    pub fn close_retirement_recipient(&mut self,grant:crate::retained_clone::RetainedCloneGrant)->Result<crate::retained_clone::RetainedCloneStep,ValueError>{
        let recipient=self.retirement.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"encoder has no original retirement recipient"))?;
        if recipient.terminal_is_empty(){return Ok(crate::retained_clone::RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0{return Ok(crate::retained_clone::RetainedCloneStep::Progress(Default::default()))}
        if recipient.next_depth_demand()?>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"encoder retirement exceeds original depth grant"))}
        let copy=recipient.next_copy_byte_demand()?;let capacity=recipient.next_capacity_byte_demand(grant.maximum_copy_bytes)?;let release=recipient.next_release_byte_demand()?;
        if copy>grant.maximum_copy_bytes||capacity>grant.maximum_capacity_bytes||release>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"encoder retirement exceeds original physical grant"))}
        self.checkpoint()?;self.charge(capacity)?;self.retirement.as_mut().unwrap().close_step(grant)
    }

    /// 🫴️ Installs one explicit caller recipient before any retained encoder ownership is created.
    pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<(),ValueError>{
        if self.retirement.is_some()||!recipient.terminal_is_empty(){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native encode requires one empty explicit retirement recipient"));}
        self.retirement=Some(recipient);Ok(())
    }
    /// 🪑️ Pre-admits the return slot and exact immediate wrapper allocations, preserving every refusal owner.
    pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn crate::ErasedSnapshotRetirement>>))->Result<T,E>{
        if self.retirement.as_ref().is_none_or(|recipient|recipient.reserved||recipient.owner.is_some()){return Err(E::from(ValueError::new(ValueRefusalKind::OwnershipLimit,"native encode has no available explicit retirement slot")));}
        self.charge(wrapper_bytes)?;self.checkpoint()?;
        self.retirement.as_mut().unwrap().reserved=true;
        let(result,owner)=operation(self);
        let recipient=self.retirement.as_mut().unwrap();*recipient.owner=owner;recipient.reserved=false;
        result
    }


    /// ⏸️ Transfers cumulative admission without retaining the current hop's callback.
    pub fn pause(self)->Result<NativeEncodeContinuation,ValueError>{
        if matches!(self.allocation,allocation::Binding::Forwarded(_))||self.retirement.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native encoding requires its original allocation continuation"));}
        self.continuation()
    }
    fn continuation(&self)->Result<NativeEncodeContinuation,ValueError>{
        if self.depth!=0||self.owned_bytes>self.maximum_bytes||(self.total!=0&&self.completed>self.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode continuation lacks valid cumulative accounting"));}
        Ok(NativeEncodeContinuation{receiving:self.receiving,maximum_bytes:self.maximum_bytes,owned_bytes:self.owned_bytes,completed:self.completed,total:self.total,stage:self.stage})
    }
    /// ▶️ Rebinds a moved operation receipt to the current hop's cancellation callback.
    pub fn resume(receipt:NativeEncodeContinuation,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Result<Self,ValueError>{
        if receipt.owned_bytes>receipt.maximum_bytes||(receipt.total!=0&&receipt.completed>receipt.total){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native encode continuation has invalid cumulative accounting"));}
        Ok(Self{scope_maximum:None,retirement:None,receiving:receipt.receiving,maximum_bytes:receipt.maximum_bytes,owned_bytes:receipt.owned_bytes,completed:receipt.completed,total:receipt.total,started:false,stage:receipt.stage,depth:0,callback,allocation:allocation::Binding::Local})
    }

    /// 🚦️ Binds the exact caller ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Self{Self{scope_maximum:None,retirement:None,receiving:false,maximum_bytes,owned_bytes:0,completed:0,total:0,started:false,stage:0,depth:0,callback,allocation:allocation::Binding::Local}}
    /// 📏️ Returns the complete allowance and cumulative admitted ownership.
    pub fn maximum_bytes(&self)->usize{self.maximum_bytes}
    /// 📊️ Returns storage admitted across every nested stage.
    pub fn owned_bytes(&self)->usize{self.owned_bytes}
    /// 🛂️ Starts with zero storage authority; each original retained turn must supply its grant.
    pub fn new_retained(callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Self{let mut control=Self::new(0,callback);control.receiving=true;control}
    /// 🎟️ Binds this original turn's allocation authority without resetting cumulative ownership.
    pub fn admit_turn_capacity(&mut self,maximum_capacity_bytes:usize)->Result<(),ValueError>{let next=self.owned_bytes.checked_add(maximum_capacity_bytes).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native turn capacity exceeds address space"))?;self.maximum_bytes=if self.receiving{self.scope_maximum.map_or(next,|maximum|maximum.min(next))}else{self.maximum_bytes.min(next)};Ok(())}

    /// 🪙️ Consumes the same admission owner under an exact checked source capacity.
    pub fn admit_capacity(mut self,source_bytes:usize,multiples:usize,scaffold_bytes:usize)->Result<Self,(Self,ValueError)>{match source_bytes.checked_mul(multiples).and_then(|bytes|bytes.checked_add(scaffold_bytes)).filter(|bytes|*bytes<=isize::MAX as usize&&*bytes>=self.owned_bytes){Some(maximum)=>{self.maximum_bytes=maximum;Ok(self)},None=>Err((self,ValueError::new(ValueRefusalKind::OwnershipLimit,"native encoding capacity exceeds source policy or admitted ownership")))}}
    /// 📐️ Applies a narrower physical stage ceiling without resetting ownership.
    pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{maximum::run(self,maximum,operation)}
    /// 🛑️ Checks cancellation before an expensive operation.
    pub fn checkpoint(&mut self)->Result<(),ValueError>{if(self.callback)(NativeEncodeProgress{completed:self.completed,total:self.total,owned_bytes:self.owned_bytes}){self.started=true;Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled, "native encoding canceled"))}}
    /// 🧭️ Begins an exact workload without resetting admitted ownership.
    pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{self.stage=self.stage.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native encoding stage overflow"))?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🪆️ Restores the enclosing workload while retaining child allocation charges.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{let parent=(self.completed,self.total,self.started,self.stage);let mut scope=StageScope{control:self,parent};operation(&mut *scope.control)}
    /// 🌲️ Bounds borrowed recursive projection independently of the input's depth.
    pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{if self.depth>=maximum{return Err(E::from(ValueError::new(ValueRefusalKind::DepthLimit, "native encoding exceeds depth limit")))}self.depth+=1;let result=operation(self);self.depth-=1;result}
    /// 📍️ Publishes interior checkpoints at256-unit boundaries and stage completion.
    pub fn advance(&mut self,units:usize)->Result<(),ValueError>{if !self.started{self.checkpoint()?;}let previous=self.completed;self.completed=self.completed.checked_add(units).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native encoding work overflow"))?;if self.total!=0&&self.completed>self.total{return Err(ValueError::new(ValueRefusalKind::WorkLimit, "native encoding exceeds declared workload"))}if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}Ok(())}
    /// 🔢️ Advances one explicit owned field or collection item.
    pub fn step(&mut self)->Result<(),ValueError>{self.advance(1)}
    /// 📦️ Admits storage before an allocation, preserving all previous charges.
    pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{let next=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "native encoding ownership exceeds caller limit"))?;if !self.started||bytes>65536{self.checkpoint()?;}if let allocation::Binding::Forwarded(port)=&mut self.allocation{port(NativeEncodeAllocation{bytes,owned_bytes:self.owned_bytes,next_owned_bytes:next,maximum_bytes:self.maximum_bytes})?;}self.owned_bytes=next;Ok(())}
    /// 🗂️ Reserves a known typed frontier before creating its slots.
    pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding collection allocation failed"))?;Ok(output)}
    /// 🔤️ Owns exact UTF-8 through cancellable64KiB spans.
    pub fn copy_text(&mut self,text:&str)->Result<String,ValueError>{self.scoped_stage(|control|{control.begin_stage(text.len())?;control.charge(text.len())?;let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding text allocation failed"))?;let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);control.advance(end-position)?;position=end;}Ok(output)})}
    /// 🧬️ Owns intrinsic octets through cancellable64KiB spans.
    pub fn copy_bytes(&mut self,bytes:&[u8])->Result<Vec<u8>,ValueError>{self.scoped_stage(|control|{control.begin_stage(bytes.len())?;let mut output=control.allocate_vec(bytes.len())?;for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);control.advance(chunk.len())?;}Ok(output)})}
    /// 📎️ Appends exact bytes with admission and cancellation inside materialization.
    pub fn append_bytes(&mut self,output:&mut Vec<u8>,bytes:&[u8])->Result<(),ValueError>{self.scoped_stage(|control|{control.begin_stage(bytes.len())?;for chunk in bytes.chunks(65536){let capacity=output.len().checked_add(chunk.len()).filter(|capacity|*capacity<=isize::MAX as usize).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native encoding output capacity overflow"))?;if capacity>output.capacity(){control.charge(capacity)?;output.try_reserve_exact(chunk.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"native encoding output allocation failed"))?;}output.extend_from_slice(chunk);control.advance(chunk.len())?;}Ok(())})}
}
