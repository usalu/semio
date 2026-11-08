//! 🗂️ Native sorted collections retain their exact entry-vector backing through funded admission.
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::retirement::{RetireOwned,RetirementCursor};
use std::{marker::PhantomData,mem::{ManuallyDrop,size_of}};
mod key_seal{pub trait Sealed{}}
/// 🔑️ Native scalar and UTF-8 keys expose bounded canonical bytes without user callbacks.
pub trait NativeSortedKey:key_seal::Sealed+Send+Sync+'static{fn native_key_len(&self)->usize;fn native_key_byte(&self,index:usize)->u8;}
macro_rules! unsigned_key{($($t:ty),*)=>{$(impl key_seal::Sealed for $t{}impl NativeSortedKey for $t{fn native_key_len(&self)->usize{size_of::<Self>()}fn native_key_byte(&self,index:usize)->u8{self.to_be_bytes()[index]}})*}}
unsigned_key!(u8,u16,u32,u64,u128,usize);
macro_rules! signed_key{($($t:ty),*)=>{$(impl key_seal::Sealed for $t{}impl NativeSortedKey for $t{fn native_key_len(&self)->usize{size_of::<Self>()}fn native_key_byte(&self,index:usize)->u8{let byte=self.to_be_bytes()[index];if index==0{byte^128}else{byte}}})*}}
signed_key!(i8,i16,i32,i64,i128,isize);
impl key_seal::Sealed for String{}
impl NativeSortedKey for String{fn native_key_len(&self)->usize{self.len()}fn native_key_byte(&self,index:usize)->u8{self.as_bytes()[index]}}
pub struct NativeSortedMap<K:NativeSortedKey,V>{entries:ManuallyDrop<Vec<(K,V)>>}
pub struct NativeSortedSet<K:NativeSortedKey>{entries:ManuallyDrop<Vec<K>>}
impl<K:NativeSortedKey,V> Default for NativeSortedMap<K,V>{fn default()->Self{Self::wrap(Vec::new())}}
impl<K:NativeSortedKey> Default for NativeSortedSet<K>{fn default()->Self{Self::wrap(Vec::new())}}
impl<K:NativeSortedKey,V> NativeSortedMap<K,V>{
 pub fn entries(&self)->&[(K,V)]{&self.entries}
 /// 🧷️ Borrows the same immutable native backing for a retained source projection.
 pub fn entries_vector(&self)->&Vec<(K,V)>{&self.entries}
 pub fn len(&self)->usize{self.entries.len()}
 pub fn is_empty(&self)->bool{self.entries.is_empty()}
 fn wrap(entries:Vec<(K,V)>)->Self{Self{entries:ManuallyDrop::new(entries)}}
 pub fn return_entries_demand(&self)->RetirementDemand{return_demand::<(K,V)>()}
 pub fn return_entries_into<'r>(&mut self,output:&mut Option<Vec<(K,V)>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{{let d=self.return_entries_demand();return_entries(&mut self.entries,output,effect,reply,d)}}
}
impl<K:NativeSortedKey> NativeSortedSet<K>{
 pub fn entries(&self)->&[K]{&self.entries}
 /// 🧷️ Borrows the same immutable native backing for a retained source projection.
 pub fn entries_vector(&self)->&Vec<K>{&self.entries}
 pub fn len(&self)->usize{self.entries.len()}
 pub fn is_empty(&self)->bool{self.entries.is_empty()}
 fn wrap(entries:Vec<K>)->Self{Self{entries:ManuallyDrop::new(entries)}}
 pub fn return_entries_demand(&self)->RetirementDemand{return_demand::<K>()}
 pub fn return_entries_into<'r>(&mut self,output:&mut Option<Vec<K>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{{let d=self.return_entries_demand();return_entries(&mut self.entries,output,effect,reply,d)}}
}
fn return_demand<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Vec<T>>()+size_of::<Option<Vec<T>>>(),depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn admitted(effect:&RetainedAllocationEffect<'_>,d:RetirementDemand)->Result<bool,ValueError>{Ok(effect.remaining_work_units()?>0&&retained_clone_grant_admits(effect.grant(),d))}
fn charge(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native sorted ownership exceeded its original Work"))}}
fn no_progress<'r>(reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>{reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))}
fn return_entries<'r,T>(entries:&mut Vec<T>,output:&mut Option<Vec<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>,d:RetirementDemand)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if output.is_some(){return Err(ValueError::invariant("native sorted input return slot is occupied"))}if !admitted(effect,d)?{return Ok(no_progress(reply))}charge(effect)?;*output=Some(std::mem::take(entries));Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress(d)))))}
impl<K:NativeSortedKey+RetireOwned,V:RetireOwned> RetireOwned for NativeSortedMap<K,V>{
 type RetirementCursor=<Vec<(K,V)> as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.entries.retirement_ready()}
 fn retirement_seed(mut self)->Self::RetirementCursor{std::mem::take(&mut*self.entries).retirement_seed()}
 fn controlled_retirement_supported()->bool{Vec::<(K,V)>::controlled_retirement_supported()}
 fn controlled_retirement_supported_in(path:&semio_framework_value_retirement::retirement::RetirementSupport<'_>)->bool{path.supports::<Vec<(K,V)>>()}
}
impl<K:NativeSortedKey+RetireOwned> RetireOwned for NativeSortedSet<K>{
 type RetirementCursor=<Vec<K> as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.entries.retirement_ready()}
 fn retirement_seed(mut self)->Self::RetirementCursor{std::mem::take(&mut*self.entries).retirement_seed()}
 fn controlled_retirement_supported()->bool{Vec::<K>::controlled_retirement_supported()}
 fn controlled_retirement_supported_in(path:&semio_framework_value_retirement::retirement::RetirementSupport<'_>)->bool{path.supports::<Vec<K>>()}
}
impl<K:NativeSortedKey,V> Drop for NativeSortedMap<K,V>{fn drop(&mut self){let empty=self.entries.is_empty()&&self.entries.capacity()==0;assert!(empty||std::thread::panicking(),"native sorted map retains its original entries");if empty{unsafe{ManuallyDrop::drop(&mut self.entries)}}}}
impl<K:NativeSortedKey> Drop for NativeSortedSet<K>{fn drop(&mut self){let empty=self.entries.is_empty()&&self.entries.capacity()==0;assert!(empty||std::thread::panicking(),"native sorted set retains its original entries");if empty{unsafe{ManuallyDrop::drop(&mut self.entries)}}}}
trait Keys<T>{type Key:NativeSortedKey;fn key(value:&T)->&Self::Key;}
struct MapKeys<K,V>(PhantomData<fn()->(K,V)>);struct SetKeys<K>(PhantomData<fn()->K>);
impl<K:NativeSortedKey,V> Keys<(K,V)> for MapKeys<K,V>{type Key=K;fn key(value:&(K,V))->&K{&value.0}}
impl<K:NativeSortedKey> Keys<K> for SetKeys<K>{type Key=K;fn key(value:&K)->&K{value}}
struct Admission<T,M>{input:ManuallyDrop<Option<Vec<T>>>,pair:usize,offset:usize,ordering:i8,phase:u8,last_compared:bool,marker:PhantomData<fn()->M>}
impl<T,M> Default for Admission<T,M>{fn default()->Self{Self{input:ManuallyDrop::new(None),pair:1,offset:0,ordering:0,phase:0,last_compared:false,marker:PhantomData}}}
impl<T,M:Keys<T>> Admission<T,M>{
 fn capture_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Vec<T>>>()+2*size_of::<usize>()+size_of::<i8>()+size_of::<u8>()+size_of::<bool>(),depth:1,..Default::default()}}
 fn capture<'r>(&mut self,input:&mut Option<Vec<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if self.input.is_some()||self.phase!=0{return Err(ValueError::invariant("native sorted admission already owns its input"))}let d=self.capture_demand();if input.is_none()||!admitted(effect,d)?{return Ok(no_progress(reply))}charge(effect)?;*self.input=input.take();self.pair=1;self.offset=0;self.ordering=0;self.last_compared=false;self.phase=if self.input.as_ref().unwrap().len()<2{3}else{1};Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress(d)))))}
 fn advance_demand(&self)->Result<RetirementDemand,ValueError>{if self.phase==3{return Ok(Default::default())}if self.phase!=1&&self.phase!=2{return Err(ValueError::invariant("native sorted admission has no original comparison quote"))}let copy_bytes=if self.phase==1{let input=self.input.as_ref().expect("captured original native entries");let left=M::key(&input[self.pair-1]);let right=M::key(&input[self.pair]);if self.offset<left.native_key_len().min(right.native_key_len()){2+size_of::<usize>()+3}else{2*size_of::<usize>()+3}}else{2*size_of::<usize>()+3};Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 fn advance<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{
  if self.phase==3{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if self.phase!=1&&self.phase!=2{return Err(ValueError::invariant("native sorted admission has no selected order check"))}let d=self.advance_demand()?;if !admitted(effect,d)?{return Ok(no_progress(reply))}charge(effect)?;self.last_compared=false;
  if self.phase==1{let input=self.input.as_ref().expect("captured native input");let left=M::key(&input[self.pair-1]);let right=M::key(&input[self.pair]);let bytes=self.offset<left.native_key_len().min(right.native_key_len());let compared=if bytes{let compared=left.native_key_byte(self.offset).cmp(&right.native_key_byte(self.offset));self.offset+=1;compared}else{left.native_key_len().cmp(&right.native_key_len())};self.ordering=match compared{std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1};self.last_compared=true;if !bytes||self.ordering!=0{self.phase=2}return Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress(d)))))}
  if self.ordering>=0{self.phase=4;return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native sorted entries require unique ascending keys").with_retained_progress(progress(d)))))}self.pair+=1;self.offset=0;self.ordering=0;self.phase=if self.pair>=self.input.as_ref().unwrap().len(){3}else{1};Ok(reply.complete(Ok(if self.phase==3{RetainedCloneStep::Complete(progress(d))}else{RetainedCloneStep::Progress(progress(d))})))
 }
 fn publication_demand<O>(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Vec<T>>>()+size_of::<Option<O>>()+size_of::<u8>(),depth:1,..Default::default()}}
 fn publish<'r,O>(&mut self,output:&mut Option<O>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>,wrap:fn(Vec<T>)->O)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if self.phase!=3{return Err(ValueError::invariant("native sorted publication requires its completed original order check"))}if output.is_some(){return Err(ValueError::invariant("native sorted output is occupied"))}let d=self.publication_demand::<O>();if !admitted(effect,d)?{return Ok(no_progress(reply))}charge(effect)?;let entries=self.input.take().expect("validated original native entries remain resident");*output=Some(wrap(entries));self.phase=0;Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress(d)))))}
 fn return_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Vec<T>>>()+size_of::<u8>(),depth:1,..Default::default()}}
 fn return_input<'r>(&mut self,output:&mut Option<Vec<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if output.is_some(){return Err(ValueError::invariant("native sorted original input return is occupied"))}let d=self.return_demand();if self.input.is_none()||!admitted(effect,d)?{return Ok(no_progress(reply))}charge(effect)?;*output=self.input.take();self.phase=0;Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress(d)))))}
}
impl<T,M> Drop for Admission<T,M>{fn drop(&mut self){let empty=self.input.is_none();assert!(empty||std::thread::panicking(),"native sorted admission retains its original vector");if empty{unsafe{ManuallyDrop::drop(&mut self.input)}}}}
pub struct NativeMapAdmission<K:NativeSortedKey,V>{state:Admission<(K,V),MapKeys<K,V>>}
pub struct NativeSetAdmission<K:NativeSortedKey>{state:Admission<K,SetKeys<K>>}
macro_rules! admission_ports{($name:ident<$($generic:ident),*>,$entry:ty,$output:ty,$wrap:path)=>{impl<K:NativeSortedKey $(,$generic)*> Default for $name<K $(,$generic)*>{fn default()->Self{Self{state:Default::default()}}}impl<K:NativeSortedKey $(,$generic)*> $name<K $(,$generic)*>{
 pub fn capture_demand(&self)->RetirementDemand{self.state.capture_demand()}
 pub fn advance_demand(&self)->Result<RetirementDemand,ValueError>{self.state.advance_demand()}
 pub fn publication_demand(&self)->RetirementDemand{self.state.publication_demand::<$output>()}
 pub fn return_demand(&self)->RetirementDemand{self.state.return_demand()}
 pub fn last_compared(&self)->bool{self.state.last_compared}
 pub fn has_input(&self)->bool{self.state.input.is_some()}
 pub fn is_validated(&self)->bool{self.state.phase==3}
 pub fn terminal_is_empty(&self)->bool{self.state.input.is_none()}
 pub fn capture_from<'r>(&mut self,input:&mut Option<Vec<$entry>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.state.capture(input,effect,reply)}
 pub fn advance_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.state.advance(effect,reply)}
 pub fn publish_into<'r>(&mut self,output:&mut Option<$output>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.state.publish(output,effect,reply,$wrap)}
 pub fn return_input_into<'r>(&mut self,output:&mut Option<Vec<$entry>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.state.return_input(output,effect,reply)}
}}}
admission_ports!(NativeMapAdmission<V>,(K,V),NativeSortedMap<K,V>,NativeSortedMap::wrap);
admission_ports!(NativeSetAdmission<>,K,NativeSortedSet<K>,NativeSortedSet::wrap);

impl<K:NativeSortedKey+RetireOwned,V:RetireOwned> RetireOwned for NativeMapAdmission<K,V>{
 type RetirementCursor=<Vec<(K,V)> as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.state.input.as_ref().is_none_or(RetireOwned::retirement_ready)}
 fn retirement_seed(mut self)->Self::RetirementCursor{let input=self.state.input.take().unwrap_or_default();self.state.phase=0;input.retirement_seed()}
 fn controlled_retirement_supported()->bool{Vec::<(K,V)>::controlled_retirement_supported()}
 fn controlled_retirement_supported_in(path:&semio_framework_value_retirement::retirement::RetirementSupport<'_>)->bool{path.supports::<Vec<(K,V)>>()}
}
impl<K:NativeSortedKey+RetireOwned> RetireOwned for NativeSetAdmission<K>{
 type RetirementCursor=<Vec<K> as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.state.input.as_ref().is_none_or(RetireOwned::retirement_ready)}
 fn retirement_seed(mut self)->Self::RetirementCursor{let input=self.state.input.take().unwrap_or_default();self.state.phase=0;input.retirement_seed()}
 fn controlled_retirement_supported()->bool{Vec::<K>::controlled_retirement_supported()}
 fn controlled_retirement_supported_in(path:&semio_framework_value_retirement::retirement::RetirementSupport<'_>)->bool{path.supports::<Vec<K>>()}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
