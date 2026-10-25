//! 🧳️ Original JSON operation bytes become typed native projections without a cold conversion.
use crate::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor,JsonGrammarCursor,JsonMemberPolicy,JsonError};
use semio_framework_value::{DslValue, RetirementDemand, ValueError, ValueRefusalKind, ArtifactOwnedValueRetirementFactory, FactoryAuthority, retained_clone::{RetainedCloneSource, RetainedOwnedProjection, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::owned::NativeReceivingOriginal};
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{Header,Parse,Source,Shell,Capture,Birth,Receive,TakeReceived,CloseReceived,Authority,TypedSource,TypedProjection,CanonicalProjection,CanonicalBirth,Canonical,Ready}

/// 🛡️ The installed wire projection determines canonical operation bytes; its original decoder ledger remains fixed.
pub struct OriginalJsonReceiving<M:NativeReceive<DslValue>+Sync>{
 ledger:u64,header:&'static[u8],wire:ManuallyDrop<Option<RetainedOwnedProjection<Vec<u8>>>>,
 parser:ManuallyDrop<Option<JsonGrammarCursor<DslValue>>>,parsed:ManuallyDrop<Option<DslValue>>,
 parsed_source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,M>>>,
 typed:ManuallyDrop<Option<M>>,authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,typed_source:ManuallyDrop<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>,
 output:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,canonical_projection:ManuallyDrop<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>,canonical:ManuallyDrop<Option<ArtifactCanonicalJsonTreeCursor>>,
 factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,factory_close:Option<FactoryAuthority>,
 error:ManuallyDrop<Option<JsonError>>,retirement:RetainedCloneClose,source:for<'a>fn(&'a M)->&'a dyn ArtifactCanonicalJsonTree,offset:usize,phase:Phase,closing:bool,faulted:bool,
}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn phase(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(size_of::<Phase>()).ok_or_else(||invalid("original JSON receiving extent overflow"))?;Ok(d)}
impl<M:NativeReceive<DslValue>+Sync> OriginalJsonReceiving<M>{
 /// 📐️ Captures only original wire custody, codec source and issuer aliases before reading any bytes.
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 /// 🎟️ A denied constructor hands the same wire projection back without decoding or materialization.
 pub fn admit(wire:RetainedOwnedProjection<Vec<u8>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,header:&'static[u8],source:for<'a>fn(&'a M)->&'a dyn ArtifactCanonicalJsonTree,control:&NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,RetainedOwnedProjection<Vec<u8>>)>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Err((invalid("original JSON receiving shell was not funded"),wire));}
  Ok((Self{ledger:control.ledger_identity(),header,wire:ManuallyDrop::new(Some(wire)),parser:ManuallyDrop::new(Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject))),parsed:ManuallyDrop::new(None),parsed_source:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),typed:ManuallyDrop::new(None),authority:ManuallyDrop::new(None),typed_source:ManuallyDrop::new(None),output:ManuallyDrop::new(None),canonical_projection:ManuallyDrop::new(None),canonical:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(Arc::clone(factory))),factory_close:None,error:ManuallyDrop::new(None),retirement:Default::default(),source,offset:header.len(),phase:Phase::Header,closing:false,faulted:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 /// 📏️ The next physical frontier is quoted from the same original parser, receiver or canonical source.
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body);}if self.faulted{return Err(invalid("faulted original JSON receiving requires cleanup"));}
  phase(match self.phase{
   Phase::Header=>RetirementDemand{copy_bytes:self.header.len(),depth:1,..Default::default()},
   Phase::Parse=>{let input=self.wire.as_ref().ok_or_else(||invalid("original JSON wire is absent"))?.borrow()?;let mut d=self.parser.as_ref().unwrap().normal_step_demands(input.get().get(self.header.len()..).ok_or_else(||invalid("original JSON wire header is truncated"))?).map_err(|error|ValueError::literal(error.kind(),"original JSON grammar demand refused"))?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<Option<DslValue>>()).ok_or_else(||invalid("original JSON result capture overflow"))?;d},
   Phase::Source=>{let d=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();RetirementDemand{copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()}},
   Phase::Shell=>NativeReceivingOriginal::<DslValue,M>::shell_demand(),
   Phase::Capture=>self.receiver.as_ref().unwrap().original_capture_demand()?,
   Phase::Birth=>self.receiver.as_ref().unwrap().receiver_birth_demand()?,
   Phase::Receive=>self.receiver.as_ref().unwrap().demands(body)?,
   Phase::TakeReceived=>self.receiver.as_ref().unwrap().take_demand()?,
   Phase::CloseReceived=>if self.receiver.as_ref().unwrap().terminal_is_empty(){header::<Option<NativeReceivingOriginal<DslValue,M>>>()}else{self.receiver.as_ref().unwrap().close_demands(body)?},
   Phase::Authority=>header::<FactoryOwnedSourceAuthority<M>>(),
   Phase::TypedSource=>{let d=RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::borrowed_constructor_demand::<FactoryOwnedSourceAuthority<M>>();RetirementDemand{copy_bytes:RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()}},
   Phase::TypedProjection|Phase::CanonicalProjection=>RetainedOwnedProjection::<M>::constructor_demand(),
   Phase::CanonicalBirth=>ArtifactCanonicalJsonTreeCursor::constructor_demand(),
   Phase::Canonical=>{let mut d=self.canonical.as_ref().unwrap().next_demand()?;d.copy_bytes=d.copy_bytes.checked_add(64+size_of::<usize>()).ok_or_else(||invalid("original JSON canonical comparison extent overflow"))?;d},
   Phase::Ready=>header::<Option<RetainedOwnedProjection<M>>>(),
  })
 }
 /// ⏯️ Each granted action preserves the same decode ledger and original byte source.
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if control.ledger_identity()!=self.ledger{return Err(invalid("original JSON receiving changed its decoder ledger"));}
  let d=self.demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.closing{return control.with_retirement(g,|retirement|self.close_step(retirement,g));}
  control.checkpoint()?;if !matches!(self.phase,Phase::Parse|Phase::Receive){control.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?;}}let child=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:g.maximum_copy_bytes-size_of::<Phase>(),..g};let unit=||RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  let mut p=match self.phase{
   Phase::Header=>{let input=self.wire.as_ref().unwrap().borrow()?;if input.get().get(..self.header.len())!=Some(self.header){self.faulted=true;return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original JSON operation wire header differs").with_retained_progress(unit()));}self.phase=Phase::Parse;let mut p=unit();p.copied_bytes-=size_of::<Phase>();p},
   Phase::Parse=>{let input=self.wire.as_ref().unwrap().borrow()?;let parse=self.parser.as_mut().unwrap();let result=parse.step_source(input.get().get(self.header.len()..).ok_or_else(||invalid("original JSON wire header is truncated"))?,1,control,RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-size_of::<Option<DslValue>>(),..child},false);let mut p=parse.normal_step_progress();p.copied_bytes=p.copied_bytes.checked_add(size_of::<Option<DslValue>>()).ok_or_else(||invalid("original JSON parser receipt overflow"))?;match result{Ok(Some(value))=>{*self.parsed=Some(value);self.phase=Phase::Source;},Ok(None)=>{},Err(error)=>{let kind=error.kind();*self.error=Some(error);self.faulted=true;p.copied_bytes+=size_of::<Phase>();return Err(ValueError::literal(kind,"original JSON syntax refused").with_retained_progress(p));}}p},
   Phase::Source=>{let value=self.parsed.take().unwrap();match RetainedCloneSource::admit_owned(value,(),child){Ok((source,p))=>{*self.parsed_source=Some(source);self.phase=Phase::Shell;p},Err((error,value,()))=>{*self.parsed=Some(value);return Err(error);}}},
   Phase::Shell=>{let(receiver,p)=NativeReceivingOriginal::begin_shell(child)?;*self.receiver=Some(receiver);self.phase=Phase::Capture;p},
   Phase::Capture=>{let p=self.receiver.as_mut().unwrap().admit_original(&mut self.parsed_source,child)?;self.phase=Phase::Birth;p},
   Phase::Birth=>{let p=self.receiver.as_mut().unwrap().begin_receiver(child)?;self.phase=Phase::Receive;p},
   Phase::Receive=>{let owner=self.receiver.as_mut().unwrap();let p=owner.advance(control,body,child)?.progress();if owner.output_ready(){self.phase=Phase::TakeReceived;}p},
   Phase::TakeReceived=>{let(value,p)=self.receiver.as_mut().unwrap().take(child)?.ok_or_else(||invalid("funded original JSON receiver omitted its output"))?;*self.typed=Some(value);self.receiver.as_mut().unwrap().begin_close();self.phase=Phase::CloseReceived;p},
   Phase::CloseReceived=>{if self.receiver.as_ref().unwrap().terminal_is_empty(){self.receiver.take();self.phase=Phase::Authority;let mut p=unit();p.copied_bytes-=size_of::<Phase>();p}else{self.receiver.as_mut().unwrap().close_step(child)?.progress()}},
   Phase::Authority=>{*self.authority=Some(FactoryOwnedSourceAuthority::new(self.typed.take().unwrap(),Arc::clone(self.factory.as_ref().unwrap())));self.phase=Phase::TypedSource;let mut p=unit();p.copied_bytes-=size_of::<Phase>();p},
   Phase::TypedSource=>{let authority=self.authority.take().unwrap();match RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::admit_borrowed(authority,|original:&FactoryOwnedSourceAuthority<M>|original.get(),child){Ok((source,p))=>{*self.typed_source=Some(source);self.phase=Phase::TypedProjection;p},Err((error,authority))=>{*self.authority=Some(authority);return Err(error);}}},
   Phase::TypedProjection=>{let(output,p)=self.typed_source.as_ref().unwrap().retain_original(child)?;*self.output=Some(output);self.phase=Phase::CanonicalProjection;p},
   Phase::CanonicalProjection=>{let(source,p)=self.output.as_ref().unwrap().project(0,self.source,child)?;*self.canonical_projection=Some(source);self.phase=Phase::CanonicalBirth;p},
   Phase::CanonicalBirth=>{let source=self.canonical_projection.take().unwrap();match ArtifactCanonicalJsonTreeCursor::admit(source,child){Ok((cursor,p))=>{*self.canonical=Some(cursor);self.phase=Phase::Canonical;p},Err((error,source))=>{*self.canonical_projection=Some(source);return Err(error);}}},
   Phase::Canonical=>{let mut bytes=[0;64];let copy=64+size_of::<usize>();let step=self.canonical.as_mut().unwrap().advance(&mut bytes,RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-copy,..child})?;let input=self.wire.as_ref().unwrap().borrow()?;let next=self.offset.checked_add(step.written_bytes).ok_or_else(||invalid("original JSON canonical offset overflow"))?;let equal=input.get().get(self.offset..next)==Some(&bytes[..step.written_bytes]);self.offset=next;let mut p=step.ownership.progress();p.copied_bytes=p.copied_bytes.checked_add(step.written_bytes+size_of::<usize>()).ok_or_else(||invalid("original JSON canonical receipt overflow"))?;if !equal||self.canonical.as_ref().unwrap().is_complete()&&self.offset!=input.get().len(){self.faulted=true;p.copied_bytes+=size_of::<Phase>();return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original JSON operation is not canonically encoded").with_retained_progress(p));}if self.canonical.as_ref().unwrap().is_complete(){self.phase=Phase::Ready;}p},
   Phase::Ready=>return Ok(RetainedCloneStep::Complete(Default::default())),
  };
  p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.checked_add(size_of::<Phase>()).ok_or_else(||invalid("original JSON receiving receipt overflow"))?;semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original JSON receiving")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 /// 📤️ The returned native operation projection keeps its original typed source and issuer alive.
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Ok(None);}let d=header::<Option<RetainedOwnedProjection<M>>>();if !retained_clone_grant_admits(g,d){return Ok(None);}let output=self.output.take().ok_or_else(||invalid("original JSON receiving output already transferred"))?;self.begin_close();Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn ready(&self)->bool{!self.closing&&!self.faulted&&self.phase==Phase::Ready&&self.output.is_some()}
 /// 🛑️ Cancellation retains parser, typed result, canonical cursor and every original source alias.
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close();}if let Some(canonical)=self.canonical.as_mut(){canonical.begin_close();}}
 /// 📪️ Cleanup quotes its defining typed owners instead of retiring payloads through a cold fallback.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body);}
  if let Some(canonical)=self.canonical.as_ref(){return if canonical.terminal_is_empty(){Ok(header::<Option<ArtifactCanonicalJsonTreeCursor>>())}else{canonical.next_demand()};}
  if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(header::<Option<NativeReceivingOriginal<DslValue,M>>>())}else{receiver.close_demands(body)};}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{output.close_demand(body)};}
  if let Some(output)=self.canonical_projection.as_ref(){return if output.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>())}else{output.close_demand(body)};}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{source.next_close_demand(body)};}
  if let Some(source)=self.parsed_source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<DslValue>>>())}else{source.next_close_demand(body)};}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)};}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body);}};}
  owner!(parser,JsonGrammarCursor<DslValue>);owner!(parsed,DslValue);owner!(typed,M);owner!(authority,FactoryOwnedSourceAuthority<M>);owner!(error,JsonError);
  if self.factory.is_some(){return Ok(header::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>());}if let Some(factory)=self.factory_close.as_ref(){return factory.demands(body);}Ok(Default::default())
 }
 pub fn close_step(&mut self,control:&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, '_>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if control.ledger_identity()!=self.ledger{return Err(invalid("original JSON cleanup changed its decoder ledger"));}if !self.closing{return Err(invalid("original JSON cleanup was not started"));}let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?;}let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if !self.retirement.is_empty(){return self.retirement.step_granted(g);}
  if let Some(canonical)=self.canonical.as_mut(){if canonical.terminal_is_empty(){self.canonical.take();return Ok(unit());}return canonical.advance(&mut[],g).map(|step|step.ownership);}
  if let Some(receiver)=self.receiver.as_mut(){if receiver.terminal_is_empty(){self.receiver.take();return Ok(unit());}return receiver.close_step(g);}
  macro_rules! close{($field:ident,$demand:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field.take();return Ok(unit());}return owner.close_step(g);}};}
  close!(output,close_demand);close!(canonical_projection,close_demand);close!(typed_source,next_close_demand);close!(parsed_source,next_close_demand);close!(wire,close_demand);
  macro_rules! owned{($field:ident)=>{if let Some(step)=self.retirement.begin_granted(&mut self.$field,g)?{return Ok(step);}};}
  owned!(parser);owned!(parsed);owned!(typed);owned!(authority);owned!(error);
  if let Some(factory)=self.factory.take(){*(&mut self.factory_close)=Some(FactoryAuthority::new(factory));return Ok(unit());}
  if let Some(factory)=self.factory_close.as_mut(){let step=factory.step(g)?;if factory.terminal_is_empty(){self.factory_close=None;}return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.wire.is_none()&&self.parser.is_none()&&self.parsed.is_none()&&self.parsed_source.is_none()&&self.receiver.is_none()&&self.typed.is_none()&&self.authority.is_none()&&self.typed_source.is_none()&&self.output.is_none()&&self.canonical_projection.is_none()&&self.canonical.is_none()&&self.factory.is_none()&&self.factory_close.is_none()&&self.error.is_none()&&self.retirement.is_empty()}
}
impl<M:NativeReceive<DslValue>+Sync> Drop for OriginalJsonReceiving<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original JSON receiving requires funded closure of all source and typed owners");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
