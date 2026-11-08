//! 🧺️ Native sorted cloning retains typed vector, admission and child reply custody.
#[macro_export]
macro_rules! collection_cursor_engine{($d:tt)=>{
use super::*;
pub use semio_framework_value_native_sorted::{NativeSortedKey,NativeSortedMap,NativeSortedSet,NativeMapAdmission,NativeSetAdmission};
mod collection_seal{pub trait Sealed{}}
pub trait NativeCollectionClone:RetainedClone+collection_seal::Sealed{
 type Entries:RetainedClone;type Admission:Default+Send;
 fn original_entries(&self)->&Self::Entries;
 fn entry_count(&self)->usize;
 fn admission_has_input(a:&Self::Admission)->bool;
 fn admission_validated(a:&Self::Admission)->bool;
 fn capture_demand(a:&Self::Admission)->RetirementDemand;
 fn validate_demand(a:&Self::Admission)->Result<RetirementDemand,ValueError>;
 fn publication_demand(a:&Self::Admission)->RetirementDemand;
 fn return_demand(a:&Self::Admission)->RetirementDemand;
 fn capture<'r>(a:&mut Self::Admission,input:&mut Option<Self::Entries>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn validate<'r>(a:&mut Self::Admission,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn publish<'r>(a:&mut Self::Admission,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn return_input<'r>(a:&mut Self::Admission,output:&mut Option<Self::Entries>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
}

impl<K:NativeSortedKey+RetainedClone,V:RetainedClone> collection_seal::Sealed for NativeSortedMap<K,V>{}
impl<K:NativeSortedKey+RetainedClone,V:RetainedClone> NativeCollectionClone for NativeSortedMap<K,V>{
 type Entries=Vec<(K,V)>;type Admission=NativeMapAdmission<K,V>;
 fn original_entries(&self)->&Self::Entries{self.entries_vector()}
 fn entry_count(&self)->usize{self.len()}
 fn admission_has_input(a:&Self::Admission)->bool{a.has_input()}
 fn admission_validated(a:&Self::Admission)->bool{a.is_validated()}
 fn capture_demand(a:&Self::Admission)->RetirementDemand{a.capture_demand()}
 fn validate_demand(a:&Self::Admission)->Result<RetirementDemand,ValueError>{a.advance_demand()}
 fn publication_demand(a:&Self::Admission)->RetirementDemand{a.publication_demand()}
 fn return_demand(a:&Self::Admission)->RetirementDemand{a.return_demand()}
 fn capture<'r>(a:&mut Self::Admission,input:&mut Option<Self::Entries>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.capture_from(input,effect,reply)}
 fn validate<'r>(a:&mut Self::Admission,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.advance_into(effect,reply)}
 fn publish<'r>(a:&mut Self::Admission,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.publish_into(output,effect,reply)}
 fn return_input<'r>(a:&mut Self::Admission,output:&mut Option<Self::Entries>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.return_input_into(output,effect,reply)}
}
impl<K:NativeSortedKey+RetainedClone,V:RetainedClone> RetainedClone for NativeSortedMap<K,V>{type Cursor=NativeCollectionCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}

impl<K:NativeSortedKey+RetainedClone> collection_seal::Sealed for NativeSortedSet<K>{}
impl<K:NativeSortedKey+RetainedClone> NativeCollectionClone for NativeSortedSet<K>{
 type Entries=Vec<K>;type Admission=NativeSetAdmission<K>;
 fn original_entries(&self)->&Self::Entries{self.entries_vector()}
 fn entry_count(&self)->usize{self.len()}
 fn admission_has_input(a:&Self::Admission)->bool{a.has_input()}
 fn admission_validated(a:&Self::Admission)->bool{a.is_validated()}
 fn capture_demand(a:&Self::Admission)->RetirementDemand{a.capture_demand()}
 fn validate_demand(a:&Self::Admission)->Result<RetirementDemand,ValueError>{a.advance_demand()}
 fn publication_demand(a:&Self::Admission)->RetirementDemand{a.publication_demand()}
 fn return_demand(a:&Self::Admission)->RetirementDemand{a.return_demand()}
 fn capture<'r>(a:&mut Self::Admission,input:&mut Option<Self::Entries>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.capture_from(input,effect,reply)}
 fn validate<'r>(a:&mut Self::Admission,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.advance_into(effect,reply)}
 fn publish<'r>(a:&mut Self::Admission,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.publish_into(output,effect,reply)}
 fn return_input<'r>(a:&mut Self::Admission,output:&mut Option<Self::Entries>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{a.return_input_into(output,effect,reply)}
}
impl<K:NativeSortedKey+RetainedClone> RetainedClone for NativeSortedSet<K>{type Cursor=NativeCollectionCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}

type CollectionKey=(semio_framework_value_control_contract::OriginalEffectInvocationIdentity,u8);
pub struct NativeCollectionCursor<M:NativeCollectionClone>{child:ManuallyDrop<RetainedFieldCursor<M::Entries>>,value:ManuallyDrop<Option<M::Entries>>,admission:ManuallyDrop<M::Admission>,output:ManuallyDrop<Option<M>>,source:ManuallyDrop<Option<RetainedCloneBinding>>,close:ManuallyDrop<RetainedCloneClose>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,closing:bool,spent:bool,route:Option<CollectionKey>,selected:Option<(RetirementDemand,u64)>}
impl<M:NativeCollectionClone> Default for NativeCollectionCursor<M>{fn default()->Self{Self{child:ManuallyDrop::new(Default::default()),value:ManuallyDrop::new(None),admission:ManuallyDrop::new(Default::default()),output:ManuallyDrop::new(None),source:ManuallyDrop::new(None),close:ManuallyDrop::new(Default::default()),reply:ManuallyDrop::new(None),closing:false,spent:false,route:None,selected:None}}}
impl<M:NativeCollectionClone> NativeCollectionCursor<M>{
 fn error(message:&'static str)->ValueError{ValueError::invariant(message)}
 fn projected<'a>(source:RetainedCloneRef<'a,M>)->RetainedCloneRef<'a,M::Entries>{source.project(1,|owner|owner.original_entries())}
 fn key(&self,effect:&RetainedAllocationEffect<'_>,code:u8)->Result<CollectionKey,ValueError>{let key=(effect.invocation().ok_or_else(||Self::error("native collection child requires its actual invocation"))?.identity(),code);if self.route.is_some_and(|saved|saved!=key){return Err(Self::error("native collection child changed its actual invocation"))}Ok(key)}
 fn code(&self)->u8{if M::admission_has_input(&self.admission){if M::admission_validated(&self.admission){4}else{3}}else if self.value.is_some(){2}else if self.child.output_ready(){1}else{0}}
 fn advance_reply<'r>(&mut self,source:RetainedCloneRef<'_,M>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  let code=if let Some((_,code))=self.route{code}else{
   if recover{return Err(Self::error("native collection recovery has no selected actual action"))}if self.output_ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let demand=self.advance_demands(source,body,maximum)?;let work=self.advance_work_demand(source,body,maximum)?;if work==0||work>effect.remaining_work_units()?||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if self.source.is_none(){let turn=source.bind(&mut self.source,effect);return Ok(reply.complete(turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)))}
   self.selected=Some((demand,work));let code=self.code();if code==0{match source.check_binding_paid(&self.source,effect).result{Ok(Some(true))=>{},Ok(None)=>return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))),Ok(Some(false))=>return Err(Self::error("native collection paid binding is absent")),Err(error)=>return Ok(reply.complete(Err(error.into())))}}code
  };let key=self.key(effect,code)?;
  if code==0{let mut scope=reply.progress().scope(&mut self.route,key)?;let maximum=maximum.min(effect.remaining_work_units()?);if recover{self.child.recover_advance_into(Self::projected(source),body,maximum,effect,scope.borrow()?)?;}else{self.child.advance_into(Self::projected(source),body,maximum,effect,scope.borrow()?)?;}return Ok(scope.forwarded())}
  self.route=Some(key);if self.reply.is_none(){let child=OriginalEffectReply::borrow(&mut self.reply)?;match code{1=>{if recover{self.child.recover_take(&mut self.value,effect,child)?;}else{self.child.take(&mut self.value,effect,child)?;}},2=>{M::capture(&mut self.admission,&mut self.value,effect,child)?;},3=>{M::validate(&mut self.admission,effect,child)?;},4=>{M::publish(&mut self.admission,&mut self.output,effect,child)?;},_=>return Err(Self::error("native collection advance route is invalid"))}}
  let mut scope=reply.progress().scope(&mut self.route,key)?;forward_original_value_reply(&mut self.reply,scope.borrow()?)?;Ok(scope.forwarded())
 }
 fn close_reply<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if !self.closing{return Err(Self::error("native collection close has not started"))}let code=if let Some((_,code))=self.route{code}else{
   if recover{return Err(Self::error("native collection close recovery has no selected actual action"))}let demand=self.close_demands(body)?;let work=self.close_work_demand(body)?;if work>effect.remaining_work_units()?||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if M::admission_has_input(&self.admission){self.selected=Some((demand,work));14}
   else if !self.child.terminal_is_empty(){self.selected=Some((demand,work));if self.child.close_started(){11}else{10}}
   else if !self.close.is_empty(){self.selected=Some((demand,work));12}
   else if self.value.is_some(){let turn=self.close.begin_granted(&mut self.value,effect);return Ok(reply.complete(turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
   else if self.output.is_some(){let turn=self.close.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
   else{self.selected=Some((demand,work));13}
  };let key=self.key(effect,code)?;
  if code==14{self.route=Some(key);if self.reply.is_none(){M::return_input(&mut self.admission,&mut self.value,effect,OriginalEffectReply::borrow(&mut self.reply)?)?;}let mut scope=reply.progress().scope(&mut self.route,key)?;forward_original_value_reply(&mut self.reply,scope.borrow()?)?;return Ok(scope.forwarded())}
  let reply=if code==13{reply}else{reply.progress()};let mut scope=reply.scope(&mut self.route,key)?;let child=scope.borrow()?;match(code,original){
   (10,_)=>{if recover{self.child.recover_begin_close_into(effect,child)?;}else{self.child.begin_close_into(effect,child)?;}},
   (11,Some(original))=>{if recover{self.child.recover_close_original_into(original,body,effect,child)?;}else{self.child.close_original_into(original,body,effect,child)?;}},
   (11,None)=>{if recover{self.child.recover_close_into(body,effect,child)?;}else{self.child.close_into(body,effect,child)?;}},
   (12,Some(original))=>{if recover{self.close.recover_step_original_into(original,body,effect,child)?;}else{self.close.step_original_into(original,body,effect,child)?;}},
   (12,None)=>{if recover{self.close.recover_step_into(body,effect,child)?;}else{self.close.step_into(body,effect,child)?;}},
   (13,Some(original))=>{if recover{RetainedCloneBinding::recover_close_original_into(&mut self.source,original,body,effect,child)?;}else{RetainedCloneBinding::close_original_into(&mut self.source,original,body,effect,child)?;}},
   (13,None)=>{if recover{RetainedCloneBinding::recover_close_into(&mut self.source,body,effect,child)?;}else{RetainedCloneBinding::close_into(&mut self.source,body,effect,child)?;}},
   _=>return Err(Self::error("native collection close route is invalid"))
  }Ok(scope.forwarded())
 }
}
impl<M:NativeCollectionClone> RetainedCloneCursor<M> for NativeCollectionCursor<M>{
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(|cursor:&Self|cursor.terminal_is_empty())})}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 fn advance_demands(&self,source:RetainedCloneRef<'_,M>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.route.is_some(){return self.selected.map(|s|s.0).ok_or_else(||Self::error("native collection frozen demand absent"))}if self.closing||self.spent{return Err(Self::error("native collection cannot advance a closing or spent cursor"))}if let Some(d)=source.binding_demand(&self.source)?{return Ok(d)}if self.output_ready(){return Ok(Default::default())}match self.code(){0=>self.child.advance_demands(Self::projected(source),body,maximum.checked_sub(RetainedCloneRef::<M>::check_binding_work(&self.source)).unwrap_or(0)),1=>self.child.take_demand(),2=>Ok(M::capture_demand(&self.admission)),3=>M::validate_demand(&self.admission),4=>Ok(M::publication_demand(&self.admission)),_=>unreachable!()}}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,M>,body:usize,maximum:u64)->Result<u64,ValueError>{if self.route.is_some(){return self.selected.map(|s|s.1).ok_or_else(||Self::error("native collection frozen Work absent"))}source.binding_demand(&self.source)?;if self.output_ready(){return Ok(0)}if self.source.is_none(){return source.binding_work_demand(&self.source)}let work=match self.code(){0=>{let checked=RetainedCloneRef::<M>::check_binding_work(&self.source);let native=self.child.advance_work_demand(Self::projected(source),body,maximum.checked_sub(checked).unwrap_or(0))?;if native==0{return Ok(0)}native.checked_add(checked).ok_or_else(||Self::error("native collection Work overflow"))?},1=>self.child.take_work_demand()?,2|3|4=>1,_=>unreachable!()};Ok(if work<=maximum{work}else{0})}
 fn advance_into<'r>(&mut self,s:RetainedCloneRef<'_,M>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.advance_reply(s,b,m,e,r,false)}
 fn recover_advance_into<'r>(&mut self,s:RetainedCloneRef<'_,M>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.advance_reply(s,b,m,e,r,true)}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<Option<M>>()+size_of::<bool>(),depth:1,..Default::default()})}
 fn take<'r>(&mut self,output:&mut Option<M>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if output.is_some(){return Err(Self::error("native collection caller output is occupied"))}let d=self.take_demand()?;if !self.output_ready()||effect.remaining_work_units()?==0||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}crate::record_retirement_work(effect,1).map_err(|(e,_)|e)?;*output=self.output.take();self.spent=true;Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(crate::retained_close_start_demand(self.closing))}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(crate::retained_close_start_work_demand(self.closing))}
 fn begin_close_into<'r>(&mut self,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let t=crate::start_retained_close(&mut self.closing,e);Ok(r.complete(t.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  #[cfg(test)] selected_close_tests::record(self as*const Self as usize,body);
  if self.route.is_some(){return self.selected.map(|s|s.0).ok_or_else(||Self::error("native collection frozen close demand absent"))}if !self.closing{return Ok(Default::default())}if M::admission_has_input(&self.admission){return Ok(M::return_demand(&self.admission))}if !self.child.terminal_is_empty(){return if self.child.close_started(){self.child.close_demands(body)}else{self.child.begin_close_demand()}}if !self.close.is_empty(){return self.close.next_owner_demands::<M>(false,&None,body)}if self.value.is_some(){return self.close.next_owner_demands::<M::Entries>(true,&None,body)}self.close.next_owner_demands::<M>(self.output.is_some(),&self.source,body)
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.route.is_some(){return self.selected.map(|s|s.1).ok_or_else(||Self::error("native collection frozen close Work absent"))}if !self.closing{return Ok(0)}if M::admission_has_input(&self.admission){return Ok(1)}if !self.child.terminal_is_empty(){return if self.child.close_started(){self.child.close_work_demand(body)}else{self.child.begin_close_work_demand()}}if !self.close.is_empty(){return self.close.next_owner_work::<M>(false,&None,body)}if self.value.is_some(){return self.close.next_owner_work::<M::Entries>(true,&None,body)}self.close.next_owner_work::<M>(self.output.is_some(),&self.source,body)}
 fn close_into<'r>(&mut self,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,e,r,false)}
 fn recover_close_into<'r>(&mut self,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,e,r,true)}
 fn close_original_into<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(o),body,e,r,false)}
 fn recover_close_original_into<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(o),body,e,r,true)}
 fn requires_original_birth(&self)->bool{if self.route.is_some()||M::admission_has_input(&self.admission){return false}if !self.child.terminal_is_empty(){return self.child.requires_original_birth()}if !self.close.is_empty(){return self.close.requires_original_birth()}RetainedCloneBinding::requires_original_birth(&self.source)}
 fn close_original_birth_step(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.child.terminal_is_empty(){return self.child.close_original_birth_step(o,g)}if !self.close.is_empty(){return self.close.step_original_birth(o,g)}RetainedCloneBinding::close_original_birth_step(&mut self.source,o,g)}
 fn requires_original_control(&self)->bool{if !self.child.terminal_is_empty(){return self.child.requires_original_control()}if !self.close.is_empty(){return self.close.requires_original_control()}RetainedCloneBinding::requires_original_control(&self.source)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.child.native_terminal_release().is_some_and(|release|release.validates(&*self.child))&&!M::admission_has_input(&self.admission)&&self.value.is_none()&&self.output.is_none()&&self.source.is_none()&&self.close.is_empty()&&self.reply.is_none()&&self.route.is_none()}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
}
impl<M:NativeCollectionClone> Drop for NativeCollectionCursor<M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"native collection cursor retains original custody");if empty{unsafe{ManuallyDrop::drop(&mut self.child);ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.close);ManuallyDrop::drop(&mut self.reply)}}}}
};}
