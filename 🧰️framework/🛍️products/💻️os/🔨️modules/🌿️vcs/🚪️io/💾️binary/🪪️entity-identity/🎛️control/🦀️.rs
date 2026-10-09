//! 🎛️ Caller-owned identity admission retained across asynchronous authoring hops.
use semio_framework_value::{serde,NativeEncodeControl,ValueError,ValueRefusalKind,native_encoding::{NativeEncodeProgress,NativeEncodeContinuation,NativeEncodeAllocationPort,NativeForwardedEncodeContinuation,NativeRetirementEncodeContinuation}};

#[cfg(not(target_arch="wasm32"))]
pub type Observer<'a>=dyn FnMut(NativeEncodeProgress)->bool+Send+'a;
#[cfg(target_arch="wasm32")]
pub type Observer<'a>=dyn FnMut(NativeEncodeProgress)->bool+'a;

/// 🧵️ One caller observer and consuming ownership receipt fund every physical authoring stage.
enum Receipt<'a>{Native(NativeEncodeContinuation),Forwarded(NativeForwardedEncodeContinuation<'a>),Retirement(NativeRetirementEncodeContinuation<'a>)}
pub struct EntityIdentityAuthority<'a,O:FnMut(NativeEncodeProgress)->bool+?Sized=Observer<'a>>{continuation:Option<Receipt<'a>>,observer:&'a mut O}
impl<'a,O:FnMut(NativeEncodeProgress)->bool+?Sized> EntityIdentityAuthority<'a,O>{
    /// 🚦️ Starts admission with the caller's explicit ownership ceiling and cancellation authority.
    pub fn new(maximum_bytes:usize,observer:&'a mut O)->Result<Self,ValueError>{let continuation=NativeEncodeControl::new(maximum_bytes,&mut |event|observer(event)).pause()?;Ok(Self{continuation:Some(Receipt::Native(continuation)),observer})}
    /// 🌉️ Retains the original receiving allocation port and observer through every guest authoring hop.
    pub fn new_forwarded(maximum_bytes:usize,observer:&'a mut O,allocate:&'a mut NativeEncodeAllocationPort<'a>)->Result<Self,ValueError>{Ok(Self{continuation:Some(Receipt::Forwarded(NativeForwardedEncodeContinuation::new(maximum_bytes,allocate))),observer})}
    /// 🔗️ Resumes the same caller allocation port and retirement recipient across an asynchronous hop.
    pub fn resume_forwarded(continuation:NativeForwardedEncodeContinuation<'a>,observer:&'a mut O)->Self{Self{continuation:Some(Receipt::Forwarded(continuation)),observer}}
    /// ▶️ Continues the actual previously admitted operation with the current caller observer.
    pub fn resume(continuation:NativeEncodeContinuation,observer:&'a mut O)->Self{Self{continuation:Some(Receipt::Native(continuation)),observer}}
    /// 🫴️ Continues the original local receipt beside its caller-owned retirement recipient.
    pub fn resume_retirement(continuation:NativeRetirementEncodeContinuation<'a>,observer:&'a mut O)->Self{Self{continuation:Some(Receipt::Retirement(continuation)),observer}}
    /// ⏸️ Returns the original cumulative receipt to its next publication or authoring owner.
    pub fn pause(mut self)->Result<NativeEncodeContinuation,ValueError>{match self.continuation.take(){Some(Receipt::Native(receipt))=>Ok(receipt),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native identity authority lacks its original native receipt"))}}
    /// 🚚️ Returns the opaque original receiving port with all admitted guest ownership.
    pub fn pause_forwarded(mut self)->Result<NativeForwardedEncodeContinuation<'a>,ValueError>{match self.continuation.take(){Some(Receipt::Forwarded(receipt))=>Ok(receipt),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"foreign identity authority lacks its original receiving receipt"))}}
    /// 🧾️ Returns the unique native receipt with the same original recipient still borrowed.
    pub fn pause_retirement(mut self)->Result<NativeRetirementEncodeContinuation<'a>,ValueError>{match self.continuation.take(){Some(Receipt::Retirement(receipt))=>Ok(receipt),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"identity authority lacks its original retirement receipt"))}}
    /// 🪆️ Resumes only for synchronous encoding and retains all admitted charges on refusal.
    pub fn encode<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
        let mut observer=|event|(self.observer)(event);
        if let Some(Receipt::Forwarded(receipt))=self.continuation.as_mut(){return receipt.encode(&mut observer,|control|Ok(operation(control))).map_err(E::from)?;}
        if let Some(Receipt::Retirement(receipt))=self.continuation.as_mut(){return receipt.encode(&mut observer,|control|Ok(operation(control))).map_err(E::from)?;}
        let Some(Receipt::Native(receipt))=self.continuation.take()else{return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"identity authority lacks its cumulative receipt")))};
        let mut control=NativeEncodeControl::resume(receipt,&mut observer).map_err(E::from)?;
        let result=control.scoped_stage(operation);
        self.continuation=Some(Receipt::Native(control.pause().map_err(E::from)?));
        result
    }
}

/// 🛂️ Debits the original caller before a foreign operation can reserve physical storage.
pub struct OriginalOperationReceiver<'borrow,'observer,O:FnMut(NativeEncodeProgress)->bool+?Sized=Observer<'observer>>{identity:&'borrow mut EntityIdentityAuthority<'observer,O>,receipt:OriginalOperationReceipt}

/// 🧾️ Carries only the consuming guest ledger; resumption still requires the original caller loan.
#[derive(Debug,serde::Serialize)]
#[serde(crate="semio_framework_value::serde")]
pub struct OriginalOperationReceipt{maximum:Option<usize>,owned:usize,finished:bool,reserved_return:usize,phase_start_owned:usize,phase_start_reserved_return:usize,received_output:usize,received_input:usize,received_retirement:usize}
impl OriginalOperationReceipt{
    /// 👓️ Borrows a tagged measurement view without transferring the original consuming receipt.
    pub fn project_retirement_charge(&self,additional:usize)->Result<measurement::OriginalOperationProjection<'_>,ValueError>{self.validate()?;let maximum=self.maximum.ok_or_else(Self::invariant)?;let ceiling=self.phase_start_owned.checked_add(maximum).ok_or_else(Self::invariant)?;self.source_owned().and_then(|owned|owned.checked_add(additional)).filter(|owned|*owned<=ceiling).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected checkpoint exceeds original admission"))?;Ok(measurement::OriginalOperationProjection{receipt:self,retirement:self.received_retirement.checked_add(additional).ok_or_else(Self::invariant)?})}
    fn invariant()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original operation scalar receipt violates source conservation")}
    fn source_owned(&self)->Option<usize>{self.reserved_return.checked_sub(self.phase_start_reserved_return)?.checked_add(self.phase_start_owned)?.checked_add(self.owned)?.checked_add(self.received_output)?.checked_add(self.received_input)?.checked_add(self.received_retirement)}
    fn validate(&self)->Result<(),ValueError>{match self.maximum{None if self.owned==0&&!self.finished&&self.reserved_return==0&&self.phase_start_owned==0&&self.phase_start_reserved_return==0&&self.received_output==0&&self.received_input==0&&self.received_retirement==0=>Ok(()),Some(maximum)if self.owned<=maximum&&self.phase_start_owned.checked_add(maximum).is_some()&&self.source_owned().is_some()=>Ok(()),_=>Err(Self::invariant())}}
}
#[path="🔎️measurement/🦀️.rs"]
pub mod measurement;
#[derive(serde::Deserialize)]
#[serde(crate="semio_framework_value::serde",deny_unknown_fields)]
struct OperationReceiptFields{#[serde(deserialize_with="required_maximum")] maximum:Option<usize>,owned:usize,finished:bool,reserved_return:usize,phase_start_owned:usize,phase_start_reserved_return:usize,received_output:usize,received_input:usize,received_retirement:usize}
fn required_maximum<'de,D:serde::Deserializer<'de>>(decoder:D)->Result<Option<usize>,D::Error>{serde::Deserialize::deserialize(decoder)}
impl<'de> serde::Deserialize<'de> for OriginalOperationReceipt{
    fn deserialize<D:serde::Deserializer<'de>>(decoder:D)->Result<Self,D::Error>{let fields=OperationReceiptFields::deserialize(decoder)?;let receipt=Self{maximum:fields.maximum,owned:fields.owned,finished:fields.finished,reserved_return:fields.reserved_return,phase_start_owned:fields.phase_start_owned,phase_start_reserved_return:fields.phase_start_reserved_return,received_output:fields.received_output,received_input:fields.received_input,received_retirement:fields.received_retirement};receipt.validate().map_err(serde::de::Error::custom)?;Ok(receipt)}
}
impl semio_framework_value::ToValue for OriginalOperationReceipt{
    fn to_value(&self)->semio_framework_value::DslValue{use semio_framework_value::ToValue;semio_framework_value::DslValue::object([("maximum".into(),self.maximum.to_value()),("owned".into(),self.owned.to_value()),("finished".into(),self.finished.to_value()),("reserved_return".into(),self.reserved_return.to_value()),("phase_start_owned".into(),self.phase_start_owned.to_value()),("phase_start_reserved_return".into(),self.phase_start_reserved_return.to_value()),("received_output".into(),self.received_output.to_value()),("received_input".into(),self.received_input.to_value()),("received_retirement".into(),self.received_retirement.to_value())])}
}
impl semio_framework_value::FromValue for OriginalOperationReceipt{
    fn from_value(value:semio_framework_value::DslValue)->Result<Self,ValueError>{use semio_framework_value::{DslValue,FromValue};let DslValue::Object(fields)=value else{return Err(Self::invariant());};let(mut maximum,mut owned,mut finished,mut reserved,mut start,mut start_reserved,mut received,mut input,mut retirement)=(None,None,None,None,None,None,None,None,None);for(key,value)in fields{match key.as_str(){"maximum" if maximum.is_none()=>maximum=Some(Option::<usize>::from_value(value)?),"owned" if owned.is_none()=>owned=Some(usize::from_value(value)?),"finished" if finished.is_none()=>finished=Some(bool::from_value(value)?),"reserved_return" if reserved.is_none()=>reserved=Some(usize::from_value(value)?),"phase_start_owned" if start.is_none()=>start=Some(usize::from_value(value)?),"phase_start_reserved_return" if start_reserved.is_none()=>start_reserved=Some(usize::from_value(value)?),"received_output" if received.is_none()=>received=Some(usize::from_value(value)?),"received_input" if input.is_none()=>input=Some(usize::from_value(value)?),"received_retirement" if retirement.is_none()=>retirement=Some(usize::from_value(value)?),_=>return Err(Self::invariant())}}let receipt=Self{maximum:maximum.ok_or_else(Self::invariant)?,owned:owned.ok_or_else(Self::invariant)?,finished:finished.ok_or_else(Self::invariant)?,reserved_return:reserved.ok_or_else(Self::invariant)?,phase_start_owned:start.ok_or_else(Self::invariant)?,phase_start_reserved_return:start_reserved.ok_or_else(Self::invariant)?,received_output:received.ok_or_else(Self::invariant)?,received_input:input.ok_or_else(Self::invariant)?,received_retirement:retirement.ok_or_else(Self::invariant)?};receipt.validate()?;Ok(receipt)}
}
impl<'borrow,'observer,O:FnMut(NativeEncodeProgress)->bool+?Sized> OriginalOperationReceiver<'borrow,'observer,O>{
    /// 🔗️ Borrows the same caller without creating another observer or ownership ceiling.
    pub fn new(identity:&'borrow mut EntityIdentityAuthority<'observer,O>)->Self{Self{identity,receipt:OriginalOperationReceipt{maximum:None,owned:0,finished:false,reserved_return:0,phase_start_owned:0,phase_start_reserved_return:0,received_output:0,received_input:0,received_retirement:0}}}
    /// ⏸️ Returns the scalar ledger without moving or replacing the caller authority.
    pub fn pause(self)->OriginalOperationReceipt{self.receipt}
    /// ▶️ Rebinds the consuming ledger to the original caller before guest execution resumes.
    pub fn resume(receipt:OriginalOperationReceipt,identity:&'borrow mut EntityIdentityAuthority<'observer,O>)->Result<Self,(ValueError,OriginalOperationReceipt)>{let validation=receipt.validate().and_then(|_|identity.encode(|control|{control.checkpoint()?;if let Some(maximum)=receipt.maximum{if receipt.phase_start_owned.checked_add(maximum)!=Some(control.maximum_bytes())||receipt.source_owned()!=Some(control.owned_bytes()){return Err(Self::invariant());}}Ok(())}));if let Err(error)=validation{return Err((error,receipt));}Ok(Self{identity,receipt})}
    /// 🔁️ Reopens only the same resumed receiving phase after exact source-conservation validation.
    pub fn continue_receiving_phase(&mut self)->Result<(),ValueError>{if self.receipt.maximum.is_none(){return Err(Self::invariant());}self.receipt.validate()?;let receipt=&self.receipt;self.identity.encode(|control|{control.checkpoint()?;if receipt.phase_start_owned.checked_add(receipt.maximum.unwrap())!=Some(control.maximum_bytes())||receipt.source_owned()!=Some(control.owned_bytes()){return Err(Self::invariant());}Ok(())})?;self.receipt.finished=false;Ok(())}
    pub fn with_host_io<T>(&mut self,operation:impl FnOnce(&mut OriginalOperationHostIo<'_, '_>)->Result<T,ValueError>)->Result<T,ValueError>{if self.receipt.maximum.is_none(){return Err(Self::invariant());}let receipt=&mut self.receipt;self.identity.encode(|control|{receipt.validate()?;if receipt.phase_start_owned.checked_add(receipt.maximum.unwrap())!=Some(control.maximum_bytes())||receipt.source_owned()!=Some(control.owned_bytes()){return Err(Self::invariant());}let before_owned=control.owned_bytes();let before_retirement=receipt.received_retirement;let mut scope=OriginalOperationHostIo{control,receipt,before_owned,before_retirement};operation(&mut scope)})}
    fn invariant()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"foreign operation mismatches its original receiving ledger")}
    fn active(&self)->Result<usize,ValueError>{self.receipt.maximum.filter(|_|!self.receipt.finished).ok_or_else(Self::invariant)}
    /// 🚦️ Exposes only the remaining original admission for this one receiving operation.
    pub fn begin(&mut self)->Result<usize,ValueError>{if self.receipt.maximum.is_some()&&!self.receipt.finished{return Err(Self::invariant());}let(maximum,start)=self.identity.encode(|control|{control.checkpoint()?;let remaining=control.maximum_bytes().checked_sub(control.owned_bytes()).ok_or_else(Self::invariant)?;Ok::<_,ValueError>((remaining,control.owned_bytes()))})?;self.receipt.maximum=Some(maximum);self.receipt.owned=0;self.receipt.finished=false;self.receipt.phase_start_owned=start;self.receipt.phase_start_reserved_return=self.receipt.reserved_return;self.receipt.received_output=0;self.receipt.received_input=0;self.receipt.received_retirement=0;Ok(maximum)}
    /// 📈️ Observes genuine guest progress with the original caller's cancellation authority.
    pub fn progress(&mut self,next:NativeEncodeProgress)->Result<(),ValueError>{self.active()?;if next.owned_bytes!=self.receipt.owned{return Err(Self::invariant());}self.identity.encode(|control|{control.begin_stage(next.total)?;control.advance(next.completed)})}
    /// 🧮️ Admits an exact preallocation request before either ownership ledger advances.
    pub fn allocation(&mut self,next:semio_framework_value::native_encoding::NativeEncodeAllocation)->Result<(),ValueError>{let maximum=self.active()?;if next.maximum_bytes>maximum||next.owned_bytes!=self.receipt.owned||self.receipt.owned.checked_add(next.bytes)!=Some(next.next_owned_bytes){return Err(Self::invariant());}if next.next_owned_bytes>next.maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"foreign allocation exceeds original remaining admission"));}self.identity.encode(|control|control.charge(next.bytes))?;self.receipt.owned=next.next_owned_bytes;Ok(())}
    /// 📬️ Returns the exact admitted guest ledger before the original caller can publish.
    pub fn finish(&mut self,owned:usize)->Result<(),ValueError>{self.active()?;if owned!=self.receipt.owned{return Err(Self::invariant());}self.identity.encode(|control|control.checkpoint())?;self.receipt.finished=true;Ok(())}
    /// 📤️ Reserves native host return storage before canonical lifting can allocate it.
    pub fn reserve_return(&mut self,bytes:usize)->Result<(),ValueError>{self.active()?;let next=self.receipt.reserved_return.checked_add(bytes).ok_or_else(Self::invariant)?;self.identity.encode(|control|control.charge(bytes))?;self.receipt.reserved_return=next;Ok(())}
    /// 📏️ Reports exact host return storage already admitted independently of guest ownership.
    pub fn reserved_return_bytes(&self)->usize{self.receipt.reserved_return}
    /// 🔎️ Validates the returned native shape without charging its admitted storage again.
    pub fn verify_reserved_return(&self,bytes:usize)->Result<(),ValueError>{self.ensure_finished()?;if self.receipt.reserved_return==bytes{Ok(())}else{Err(Self::invariant())}}
    /// 📥️ Measures authentic additional host input through the same resumed source loan.
    pub fn encode_input<T>(&mut self,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{if self.receipt.maximum.is_none(){return Err(Self::invariant());}let (result,charged)=self.identity.encode(|control|{let before=control.owned_bytes();let result=operation(control);let charged=control.owned_bytes().checked_sub(before).ok_or_else(Self::invariant)?;Ok::<_,ValueError>((result,charged))})?;self.receipt.received_input=self.receipt.received_input.checked_add(charged).ok_or_else(Self::invariant)?;result}
    /// ♻️ Records actual host retirement frame admission beside the unchanged guest and return ledgers.
    pub fn retire_host<T>(&mut self,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{self.with_host_io(|scope|operation(scope.control()))}
    /// 📥️ Admits the exact owned-interpreter result copy before its physical reservation.
    pub fn receive_output(&mut self,bytes:usize)->Result<(),ValueError>{if !self.receipt.finished{return Err(Self::invariant());}let next=self.receipt.received_output.checked_add(bytes).ok_or_else(Self::invariant)?;self.identity.encode(|control|control.charge(bytes))?;self.receipt.received_output=next;Ok(())}
    /// 🛑️ Refuses publication while an originally begun authoring phase remains unfinished.
    pub fn ensure_finished(&self)->Result<(),ValueError>{if self.receipt.maximum.is_some()&&!self.receipt.finished{Err(Self::invariant())}else{Ok(())}}
}

/// 👓️ Measures future framing while the real consuming receipt remains with its original owner.
pub struct OriginalOperationReceiptProjection<'a>{receipt:&'a OriginalOperationReceipt,retirement:usize}
impl serde::Serialize for OriginalOperationReceiptProjection<'_>{
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeStruct;let receipt=self.receipt;let mut output=serializer.serialize_struct("OriginalOperationReceipt",9)?;output.serialize_field("maximum",&receipt.maximum)?;output.serialize_field("owned",&receipt.owned)?;output.serialize_field("finished",&receipt.finished)?;output.serialize_field("reserved_return",&receipt.reserved_return)?;output.serialize_field("phase_start_owned",&receipt.phase_start_owned)?;output.serialize_field("phase_start_reserved_return",&receipt.phase_start_reserved_return)?;output.serialize_field("received_output",&receipt.received_output)?;output.serialize_field("received_input",&receipt.received_input)?;output.serialize_field("received_retirement",&self.retirement)?;output.end()}
}
/// 🛂️ A synchronous native loan settles the same unique scalar receipt on every exit.
pub struct OriginalOperationHostIo<'a,'control>{control:&'a mut NativeEncodeControl<'control>,receipt:&'a mut OriginalOperationReceipt,before_owned:usize,before_retirement:usize}
impl<'a,'control> OriginalOperationHostIo<'a,'control>{
    /// 🚦️ Uses the original admitted control without creating another callback or maximum.
    pub fn control(&mut self)->&mut NativeEncodeControl<'control>{self.control}
    /// 📏️ Exposes checked measurement data without admitting the proposed bytes.
    pub fn project_host_charge(&self,additional:usize)->Result<OriginalOperationReceiptProjection<'_>,ValueError>{let actual=self.control.owned_bytes();let charged=actual.checked_sub(self.before_owned).ok_or_else(OriginalOperationReceipt::invariant)?;actual.checked_add(additional).filter(|next|*next<=self.control.maximum_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"projected checkpoint exceeds original admission"))?;let retirement=self.before_retirement.checked_add(charged).and_then(|bytes|bytes.checked_add(additional)).ok_or_else(OriginalOperationReceipt::invariant)?;Ok(OriginalOperationReceiptProjection{receipt:self.receipt,retirement})}
    fn settle(&mut self){let charged=self.control.owned_bytes().checked_sub(self.before_owned).expect("original native host scope lost admitted ownership");self.receipt.received_retirement=self.before_retirement.checked_add(charged).expect("original host scope exceeds its proved native ceiling");}
}
impl Drop for OriginalOperationHostIo<'_, '_>{fn drop(&mut self){self.settle();}}

/// 🛂️ Borrows a genuine receiving operation across host engines without changing its observer ownership.
pub trait OriginalOperationReceiving {
 fn begin(&mut self)->Result<usize,ValueError>;
 fn progress(&mut self,next:NativeEncodeProgress)->Result<(),ValueError>;
 fn allocation(&mut self,next:semio_framework_value::native_encoding::NativeEncodeAllocation)->Result<(),ValueError>;
 fn finish(&mut self,owned:usize)->Result<(),ValueError>;
 fn reserve_return(&mut self,bytes:usize)->Result<(),ValueError>;
 fn reserved_return_bytes(&self)->usize;
 fn verify_reserved_return(&self,bytes:usize)->Result<(),ValueError>;
 fn receive_output(&mut self,bytes:usize)->Result<(),ValueError>;
 fn ensure_finished(&self)->Result<(),ValueError>;
}
impl<O:FnMut(NativeEncodeProgress)->bool+?Sized> OriginalOperationReceiving for OriginalOperationReceiver<'_, '_, O> {
 fn begin(&mut self)->Result<usize,ValueError>{self.begin()}
 fn progress(&mut self,next:NativeEncodeProgress)->Result<(),ValueError>{self.progress(next)}
 fn allocation(&mut self,next:semio_framework_value::native_encoding::NativeEncodeAllocation)->Result<(),ValueError>{self.allocation(next)}
 fn finish(&mut self,owned:usize)->Result<(),ValueError>{self.finish(owned)}
 fn reserve_return(&mut self,bytes:usize)->Result<(),ValueError>{self.reserve_return(bytes)}
 fn reserved_return_bytes(&self)->usize{self.reserved_return_bytes()}
 fn verify_reserved_return(&self,bytes:usize)->Result<(),ValueError>{self.verify_reserved_return(bytes)}
 fn receive_output(&mut self,bytes:usize)->Result<(),ValueError>{self.receive_output(bytes)}
 fn ensure_finished(&self)->Result<(),ValueError>{self.ensure_finished()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod receiving_input_tests;
