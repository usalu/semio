//! 🌳️ Native receiving projects genuine heterogeneous children without constructing an intermediate wire owner.
use crate::{DslValue,Number,NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneCursor,RetainedFieldCursor,retained_clone_grant_admits}};
use super::{NativeReceive,NativeReceiver};
use std::mem::{ManuallyDrop,size_of};
/// 🧷️ Every child is a reference into the original immutable input and preserves its enclosing lease.
pub trait NativeValueSource:Sync+'static{
 fn as_source(&self)->&dyn NativeValueSource;
 fn enum_ordinal(&self)->Result<u32,ValueError>{Err(invalid())}
 fn variant_tag(&self)->Option<&str>{None}
 fn variant_payload(&self)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn is_null(&self)->bool{false}
 fn boolean(&self)->Result<bool,ValueError>{Err(invalid())}
 fn number(&self)->Result<Number,ValueError>{Err(invalid())}
 fn text(&self)->Result<&String,ValueError>{Err(invalid())}
 fn record_len(&self)->Result<usize,ValueError>{Err(invalid())}
 fn record_key(&self,_index:usize)->Result<super::NativeRecordKey<'_>,ValueError>{Err(invalid())}
 fn record_name(&self,index:usize)->Result<&str,ValueError>{match self.record_key(index)?{super::NativeRecordKey::Name(name)=>Ok(name),super::NativeRecordKey::Ordinal(_)=>Err(invalid())}}
 fn record_entry(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn record_child(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn sequence_len(&self)->Result<usize,ValueError>{Err(invalid())}
 fn sequence_child(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
}
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid")}
pub fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
pub fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
pub fn nested(mut d:RetirementDemand,copy:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
pub fn child_grant(g:RetainedCloneGrant,copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-copy,maximum_depth:g.maximum_depth-1,..g}}
pub fn add_receipt(mut p:RetainedCloneProgress,copy:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(copy).ok_or_else(invalid)?;Ok(p)}
impl NativeValueSource for DslValue{
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn is_null(&self)->bool{matches!(self,Self::Null)}
 fn boolean(&self)->Result<bool,ValueError>{match self{Self::Bool(value)=>Ok(*value),_=>Err(invalid())}}
 fn number(&self)->Result<Number,ValueError>{match self{Self::Number(value)=>Ok(*value),_=>Err(invalid())}}
 fn text(&self)->Result<&String,ValueError>{match self{Self::String(value)=>Ok(value),_=>Err(invalid())}}
 fn record_len(&self)->Result<usize,ValueError>{match self{Self::Object(entries)=>Ok(entries.len()),_=>Err(invalid())}}
 fn record_key(&self,index:usize)->Result<super::NativeRecordKey<'_>,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|(key,_)|super::NativeRecordKey::Name(key.as_str())).ok_or_else(invalid),_=>Err(invalid())}}
 fn record_entry(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|entry|entry as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|(_,value)|value as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
 fn sequence_len(&self)->Result<usize,ValueError>{match self{Self::Array(entries)=>Ok(entries.len()),_=>Err(invalid())}}
 fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Array(entries)=>entries.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
}
impl NativeValueSource for String{fn as_source(&self)->&dyn NativeValueSource{self}fn text(&self)->Result<&String,ValueError>{Ok(self)}}
impl NativeValueSource for bool{fn as_source(&self)->&dyn NativeValueSource{self}fn boolean(&self)->Result<bool,ValueError>{Ok(*self)}}
impl NativeValueSource for u8{fn as_source(&self)->&dyn NativeValueSource{self}fn number(&self)->Result<Number,ValueError>{Ok(Number::UInt(u64::from(*self)))}}
impl NativeValueSource for Number{fn as_source(&self)->&dyn NativeValueSource{self}fn number(&self)->Result<Number,ValueError>{Ok(*self)}}
impl<T:NativeValueSource> NativeValueSource for (String,T){fn as_source(&self)->&dyn NativeValueSource{self}fn variant_tag(&self)->Option<&str>{Some(&self.0)}fn variant_payload(&self)->Result<&dyn NativeValueSource,ValueError>{Ok(&self.1)}fn sequence_len(&self)->Result<usize,ValueError>{Ok(2)}fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.0),1=>Ok(&self.1),_=>Err(invalid())}}}
impl<T:NativeValueSource> NativeValueSource for Vec<T>{fn as_source(&self)->&dyn NativeValueSource{self}fn sequence_len(&self)->Result<usize,ValueError>{Ok(self.len())}fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{self.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid)}}
/// 🫴️ A typed child retains its receiver and output independently through every normal and cancelled frontier.
pub struct NativeReceiveSlot<I:NativeValueSource,T:NativeReceive<I>>{receiver:ManuallyDrop<Option<Box<T::Receiver>>>,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,closing:bool,subset:bool}
impl<I:NativeValueSource,T:NativeReceive<I>> Default for NativeReceiveSlot<I,T>{fn default()->Self{Self{receiver:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,subset:false}}}
impl<I:NativeValueSource,T:NativeReceive<I>> NativeReceiveSlot<I,T>{
 pub fn original_record_subset()->Self{let mut slot=Self::default();slot.subset=true;slot}
 pub fn ready(&self)->bool{!self.closing&&self.output.is_some()}
 pub fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.output.is_some(){return Err(invalid());}match self.receiver.as_ref(){None=>{let mut d=nested(T::receive_birth_demand(),size_of::<T::Receiver>()+2*size_of::<Option<Box<T::Receiver>>>())?;d.capacity_bytes=d.capacity_bytes.checked_add(size_of::<T::Receiver>()).ok_or_else(invalid)?;Ok(d)},Some(r)if r.output_ready()=>nested(r.take_demand()?,size_of::<Option<T>>()),Some(r)=>nested(r.demands(input,body)?,0)}}
 pub fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.receiver.is_none(){let overhead=size_of::<T::Receiver>()+2*size_of::<Option<Box<T::Receiver>>>();control.charge(d.capacity_bytes)?;let child=RetainedCloneGrant{maximum_capacity_bytes:g.maximum_capacity_bytes-size_of::<T::Receiver>(),..child_grant(g,overhead)};let(r,p)=if self.subset{T::begin_record_subset(child)?}else{T::begin_receive(child)?};*self.receiver=Some(Box::new(r));let mut p=add_receipt(p,overhead)?;p.retained_capacity_bytes=p.retained_capacity_bytes.checked_add(size_of::<T::Receiver>()).ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(p));}let r=self.receiver.as_mut().unwrap();if r.output_ready(){let overhead=size_of::<Option<T>>();let(value,p)=r.take(child_grant(g,overhead))?.ok_or_else(invalid)?;*self.output=Some(value);return Ok(RetainedCloneStep::Complete(add_receipt(p,overhead)?));}r.advance(input,control,body,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()))}
 pub fn take(&mut self)->Option<T>{self.output.take()}
 pub fn begin_close(&mut self){self.closing=true;if let Some(r)=self.receiver.as_mut(){r.begin_close();}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if let Some(r)=self.receiver.as_ref(){return if r.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<T::Receiver>(),..fixed(2*size_of::<Option<Box<T::Receiver>>>())})}else{nested(r.close_demands(body)?,0)};}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}Ok(Default::default())}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(r)=self.receiver.as_mut(){if !r.terminal_is_empty(){return r.close_step(child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()));}self.receiver.take();return Ok(RetainedCloneStep::Progress(receipt(d)));}if !self.retirement.is_empty(){return self.retirement.step_granted(g);}if let Some(s)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(s);}Ok(RetainedCloneStep::Complete(Default::default()))}
 pub fn terminal_is_empty(&self)->bool{self.receiver.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>> Drop for NativeReceiveSlot<I,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native child receiving requires funded closure");}}
/// 🔢️ A scalar or unit enum constructs only its declared native value under original source binding.
pub trait NativeScalarValue<I:NativeValueSource+?Sized>:RetireOwned{fn read(original:&I)->Result<Self,ValueError> where Self:Sized;fn read_copy_bytes(original:&I)->Result<usize,ValueError>;}
pub struct NativeScalarReceiver<I:NativeValueSource+?Sized,T:NativeScalarValue<I>>{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,closing:bool,taken:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized,T:NativeScalarValue<I>> NativeScalarReceiver<I,T>{pub fn birth()->RetirementDemand{fixed(size_of::<Self>())}pub fn begin(g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::birth();if !retained_clone_grant_admits(g,d){return Err(invalid());}Ok((Self{binding:None,output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,taken:false,input:std::marker::PhantomData},receipt(d)))}}
impl<I:NativeValueSource+?Sized,T:NativeScalarValue<I>> NativeReceiver<I,T> for NativeScalarReceiver<I,T>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,_:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.taken||self.output.is_some(){return Err(invalid());}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()));}Ok(fixed(T::read_copy_bytes(input.get())?.checked_add(size_of::<Option<T>>()).ok_or_else(invalid)?))}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;if let Some(p)=input.bind(&mut self.binding,g)?{return Ok(RetainedCloneStep::Progress(p));}*self.output=Some(T::read(input.get())?);Ok(RetainedCloneStep::Complete(receipt(d)))}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(2*size_of::<Option<T>>()+size_of::<bool>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}self.taken=true;Ok(Some((self.output.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.retirement.is_empty(){return self.retirement.step_granted(g);}if let Some(s)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(s);}RetainedCloneBinding::close_one(&mut self.binding,g)}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized,T:NativeScalarValue<I>> Drop for NativeScalarReceiver<I,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native scalar receiving requires funded closure");}}
macro_rules! scalar {($t:ty,$input:ident,$value:expr)=>{impl<I:NativeValueSource+?Sized> NativeScalarValue<I> for $t{fn read($input:&I)->Result<Self,ValueError>{$value}fn read_copy_bytes(_: &I)->Result<usize,ValueError>{Ok(size_of::<Self>())}}impl<I:NativeValueSource+?Sized> NativeReceive<I> for $t{type Receiver=NativeScalarReceiver<I,Self>;fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(g)}}};}
scalar!(bool,input,input.boolean());scalar!((),input,if input.is_null(){Ok(())}else{Err(invalid())});
scalar!(f64,input,{let n=input.number()?.as_f64();if n.is_finite(){Ok(n)}else{Err(invalid())}});
macro_rules! unsigned {($($t:ty),*)=>{$(scalar!($t,input,match input.number()?{Number::UInt(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Int(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Float(n)if n.is_finite()&&n.fract()==0.0&&n>=0.0&&(n as u128)<=<$t>::MAX as u128&&n<18446744073709551616.0=>Ok(n as $t),_=>Err(invalid())});)*};}
unsigned!(u8,u16,u32,u64,usize);
macro_rules! signed {($($t:ty),*)=>{$(scalar!($t,input,match input.number()?{Number::UInt(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Int(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Float(n)if n.is_finite()&&n.fract()==0.0&&n>=<$t>::MIN as f64&&n<=<$t>::MAX as f64&&n<9223372036854775808.0=>Ok(n as $t),_=>Err(invalid())});)*};}
signed!(i8,i16,i32,i64,isize);
pub struct NativeStringReceiver<I:NativeValueSource+?Sized>{cursor:RetainedFieldCursor<String>,closing:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for String{type Receiver=NativeStringReceiver<I>;fn receive_birth_demand()->RetirementDemand{fixed(size_of::<Self::Receiver>())}fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let d=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid());}Ok((NativeStringReceiver{cursor:Default::default(),closing:false,input:std::marker::PhantomData},receipt(d)))}}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,String> for NativeStringReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{input.get().text()?;self.cursor.advance_demands(input.project(0,|v|v.text().unwrap()),body)}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;control.charge(d.capacity_bytes)?;self.cursor.advance(input.project(0,|v|v.text().unwrap()),body,g)}
 fn output_ready(&self)->bool{!self.closing&&self.cursor.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(2*size_of::<String>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(String,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}Ok(Some((self.cursor.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.cursor.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
}


/// ❔️ Optional receiving distinguishes an actual null source from a missing parent field.
pub struct NativeOptionReceiver<I:NativeValueSource,T:NativeReceive<I>>{binding:Option<RetainedCloneBinding>,child:NativeReceiveSlot<I,T>,phase:u8,closing:bool}
impl<I:NativeValueSource,T:NativeReceive<I>> NativeReceive<I> for Option<T>{type Receiver=NativeOptionReceiver<I,T>;fn receive_birth_demand()->RetirementDemand{fixed(size_of::<Self::Receiver>())}fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let d=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid());}Ok((NativeOptionReceiver{binding:None,child:Default::default(),phase:0,closing:false},receipt(d)))}}
impl<I:NativeValueSource,T:NativeReceive<I>> NativeReceiver<I,Option<T>> for NativeOptionReceiver<I,T>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=2{return Err(invalid());}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()));}if self.phase==0{return Ok(fixed(size_of::<u8>()));}nested(self.child.demands(input,body.saturating_sub(size_of::<u8>()))?,size_of::<u8>())}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;if let Some(p)=input.bind(&mut self.binding,g)?{return Ok(RetainedCloneStep::Progress(p));}if self.phase==0{self.phase=if input.get().is_null(){2}else{1};return Ok(RetainedCloneStep::Progress(receipt(d)));}let step=self.child.advance(input,control,body.saturating_sub(size_of::<u8>()),child_grant(g,size_of::<u8>()))?;if self.child.ready(){self.phase=3;}Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),size_of::<u8>())?))}
 fn output_ready(&self)->bool{!self.closing&&matches!(self.phase,2|3)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(2*size_of::<Option<T>>()+size_of::<u8>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Option<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}let value=if self.phase==2{None}else{Some(self.child.take().ok_or_else(invalid)?)};self.phase=4;Ok(Some((value,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.child.terminal_is_empty(){return self.child.close_demands(body);}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.child.terminal_is_empty(){return self.child.close_step(g);}RetainedCloneBinding::close_one(&mut self.binding,g)}
 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.binding.is_none()}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>> Drop for NativeOptionReceiver<I,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native optional receiving requires funded closure");}}

/// 📋️ A sequence declares its actual fixed or allocated native backing independently of wire conversion.
pub trait NativeSequenceCollection<T:RetireOwned>:RetireOwned{
 type Storage:RetireOwned;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>;
 fn reserve_demand(_storage:&Self::Storage,_index:usize)->Result<Option<RetirementDemand>,ValueError>{Ok(None)}
 fn reserve_step(_storage:&mut Self::Storage,_index:usize,_grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{Err(invalid())}
 fn push_depth(_storage:&Self::Storage,_index:usize)->Result<usize,ValueError>{Ok(1)}
 fn begin_storage(count:usize,control:&mut NativeDecodeControl<'_>)->Result<Self::Storage,ValueError>;
 fn push(storage:&mut Self::Storage,index:usize,value:T)->Result<(),T>;
 fn finish(storage:Self::Storage)->Self;
}
impl<T:RetireOwned> NativeSequenceCollection<T> for Vec<T>{type Storage=Vec<T>;fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Self>(),capacity_bytes:count.checked_mul(size_of::<T>()).filter(|n|*n<=isize::MAX as usize).ok_or_else(invalid)?,depth:1,..Default::default()})}fn begin_storage(count:usize,control:&mut NativeDecodeControl<'_>)->Result<Self::Storage,ValueError>{control.allocate_vec(count)}fn push(storage:&mut Self::Storage,_:usize,value:T){assert!(storage.len()<storage.capacity()||size_of::<T>()==0);storage.push(value)}fn finish(storage:Self::Storage)->Self{storage}}
impl<T:RetireOwned,const N:usize> NativeSequenceCollection<T> for [T;N]{type Storage=[Option<T>;N];fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{if count!=N{return Err(invalid());}Ok(fixed(size_of::<Self::Storage>()))}fn begin_storage(_:usize,_:&mut NativeDecodeControl<'_>)->Result<Self::Storage,ValueError>{Ok(std::array::from_fn(|_|None))}fn push(storage:&mut Self::Storage,index:usize,value:T){assert!(storage[index].is_none());storage[index]=Some(value)}fn finish(storage:Self::Storage)->Self{storage.map(|value|value.expect("validated fixed sequence storage"))}}
pub trait NativeSequenceInput<I:NativeValueSource+?Sized>:Send+'static{fn length(input:&I)->Result<usize,ValueError>;fn child(input:&I,index:usize)->Result<&dyn NativeValueSource,ValueError>;}
pub struct SequenceRows;
impl<I:NativeValueSource+?Sized> NativeSequenceInput<I> for SequenceRows{fn length(input:&I)->Result<usize,ValueError>{input.sequence_len()}fn child(input:&I,index:usize)->Result<&dyn NativeValueSource,ValueError>{input.sequence_child(index)}}
pub struct RecordEntries;
impl<I:NativeValueSource+?Sized> NativeSequenceInput<I> for RecordEntries{fn length(input:&I)->Result<usize,ValueError>{input.record_len()}fn child(input:&I,index:usize)->Result<&dyn NativeValueSource,ValueError>{input.record_entry(index)}}
pub struct NativeSequenceReceiver<I:NativeValueSource,T:NativeReceive<I>,C:NativeSequenceCollection<T>>{binding:Option<RetainedCloneBinding>,child:NativeReceiveSlot<I,T>,storage:ManuallyDrop<Option<C::Storage>>,output:ManuallyDrop<Option<C>>,retirement:RetainedCloneClose,count:usize,index:usize,phase:u8,closing:bool}
impl<I:NativeValueSource,T:NativeReceive<I>,C:NativeSequenceCollection<T>> NativeSequenceReceiver<I,T,C>{pub fn birth()->RetirementDemand{fixed(size_of::<Self>())}pub fn begin(g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::birth();if !retained_clone_grant_admits(g,d){return Err(invalid());}Ok((Self{binding:None,child:Default::default(),storage:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),count:0,index:0,phase:0,closing:false},receipt(d)))}fn source<'a>(&self,input:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,I>,ValueError>{input.get().sequence_child(self.index)?;let index=self.index;Ok(input.project(index,move|v|v.sequence_child(index).unwrap()))}}
impl<I:NativeValueSource,T:NativeReceive<I>,C:NativeSequenceCollection<T>> NativeReceiver<I,C> for NativeSequenceReceiver<I,T,C>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=7{return Err(invalid());}input.check_binding(&self.binding)?;match self.phase{0=>Ok(fixed(input.binding_copy_bytes()+size_of::<u8>())),1=>nested(C::storage_demand(input.get().sequence_len()?)?,size_of::<Option<C::Storage>>()+size_of::<usize>()+size_of::<u8>()),2=>nested(self.child.demands(self.source(input)?,body.saturating_sub(size_of::<u8>()))?,size_of::<u8>()),3=>Ok(fixed(3*size_of::<T>()+size_of::<usize>()+size_of::<u8>())),4=>self.child.close_demands(body),5=>Ok(fixed(2*size_of::<NativeReceiveSlot<I,T>>()+size_of::<u8>())),6=>Ok(fixed(2*size_of::<C::Storage>()+size_of::<Option<C>>()+size_of::<u8>())),_=>Err(invalid())}}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;match self.phase{0=>{let p=input.bind(&mut self.binding,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<u8>(),..g})?.ok_or_else(invalid)?;self.phase=1;return Ok(RetainedCloneStep::Progress(add_receipt(p,size_of::<u8>())?));},1=>{self.count=input.get().sequence_len()?;*self.storage=Some(C::begin_storage(self.count,control)?);self.phase=if self.count==0{6}else{2};},2=>{let p=self.child.advance(self.source(input)?,control,body.saturating_sub(size_of::<u8>()),child_grant(g,size_of::<u8>()))?.progress();if self.child.ready(){self.phase=3;}return Ok(RetainedCloneStep::Progress(add_receipt(p,size_of::<u8>())?));},3=>{C::push(self.storage.as_mut().ok_or_else(invalid)?,self.index,self.child.take().ok_or_else(invalid)?);self.index+=1;self.child.begin_close();self.phase=4;},4=>{control.charge(d.capacity_bytes)?;let p=self.child.close_step(g)?.progress();if self.child.terminal_is_empty(){self.phase=5;}return Ok(RetainedCloneStep::Progress(p));},5=>{self.child=Default::default();self.phase=if self.index==self.count{6}else{2};},6=>{*self.output=Some(C::finish(self.storage.take().ok_or_else(invalid)?));self.phase=7;},_=>return Err(invalid())}Ok(if self.phase==7{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(2*size_of::<Option<C>>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(C,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}Ok(Some((self.output.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.child.terminal_is_empty(){return self.child.close_demands(body);}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<C>(false,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<C>(true,&None,body);}if self.storage.is_some(){return self.retirement.next_owner_demands::<C::Storage>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.child.terminal_is_empty(){return self.child.close_step(g);}if !self.retirement.is_empty(){return self.retirement.step_granted(g);}if let Some(s)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(s);}if let Some(s)=self.retirement.begin_granted(&mut self.storage,g)?{return Ok(s);}RetainedCloneBinding::close_one(&mut self.binding,g)}
 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.storage.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,C:NativeSequenceCollection<T>,S:NativeSequenceInput<I>> Drop for NativeSequenceReceiver<I,T,C,S>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native sequence receiving requires funded closure");}}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>> NativeReceive<I> for Vec<T>{type Receiver=NativeSequenceReceiver<I,T,Self>;fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(g)}}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize> NativeReceive<I> for [T;N]{type Receiver=NativeSequenceReceiver<I,T,Self>;fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(g)}}

#[path="👥️tuple/🦀️.rs"]pub mod tuple;

#[cfg(test)]#[path="📖️source/🧪️tests/🦀️.rs"]mod source_tests;

impl<T:RetireOwned,const N:usize> NativeSequenceCollection<T> for semio_framework_value_persistent::list::PagedList<T,N>{
 type Storage=Self;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{if count>N{return Err(invalid())}Ok(fixed(size_of::<Self>()))}
 fn begin_storage(count:usize,_:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{if count>N{return Err(invalid())}Ok(Self::new())}
 fn reserve_demand(storage:&Self,_:usize)->Result<Option<RetirementDemand>,ValueError>{if storage.has_reserved_slot(){return Ok(None)}Ok(Some(RetirementDemand{copy_bytes:storage.next_reserve_copy_byte_demand().map_err(|_|invalid())?,capacity_bytes:storage.next_allocation_bytes().map_err(|_|invalid())?,depth:storage.next_reserve_depth_demand().map_err(|_|invalid())?,..Default::default()}))}
 fn reserve_step(storage:&mut Self,_:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{storage.reserve_one_funded(grant).map_err(|error|invalid().with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}))}
 fn push_depth(storage:&Self,_:usize)->Result<usize,ValueError>{storage.next_push_depth_demand().map_err(|_|invalid())}
 fn push(storage:&mut Self,_:usize,value:T)->Result<(),T>{storage.push_reserved(value)}
 fn finish(storage:Self)->Self{storage}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize> NativeReceive<I> for semio_framework_value_persistent::list::PagedList<T,N>{type Receiver=NativeSequenceReceiver<I,T,Self>;fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(g)}}
