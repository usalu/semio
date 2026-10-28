//! 📨️ The declared Drawing operation wire receives one original native mutation before entering the shared semantic producer.
use super::super::{DrawingSnapshot as P,DrawingMutation as M};
use store::os_store::one_item_publication::{ArtifactStoreOneItemPreparation,ArtifactStoreOneItemPreparationFactory,ArtifactStoreOneItemPreparationRequest,ArtifactStoreOneItemPrepared,ArtifactStoreOneItemLiveAuthority,MemberStoreOneItemWire as Wire,MemberStoreOneItemWirePreparationFactory};
use store::os_store::one_item::{ArtifactStoreOneItemGrant as Grant,ArtifactStoreOneItemFootprint,ArtifactStoreOneItemPreparationStep as Step,ArtifactStoreOneItemCheckpoint};
use store::os_store::HistoryLane;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,OriginalRetirementCloseLoan,ErasedSnapshotRetirement,ArtifactOwnedValueRetirementFactory,SnapshotRetirementFactory,FactoryAuthority,FactoryRetirement,retained_clone::{RetainedCloneSource,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress as Progress,RetainedCloneStep,RetainedCloneBirthDemand,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value_snapshot_read::SnapshotRead;
use semio_framework_pack_json::OriginalJsonReceiving;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
type Request=ArtifactStoreOneItemPreparationRequest<P,Wire,M>;
type TypedRequest=ArtifactStoreOneItemPreparationRequest<P,M,M>;
type TypedFactory=Arc<dyn ArtifactStoreOneItemPreparationFactory<P,M>>;
/// 🏷️ The mutation aggregate's declared schema accompanies its canonical op envelope.
pub const DRAWING_MEMBER_OPERATION_SCHEMA:&str="drawing.drawing";
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DrawingMemberWirePreparationFactory{#[factory_child]pub(super) typed:TypedFactory}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn frame<T>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>(),depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||invalid("drawing wire depth overflow"))?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_depth:g.maximum_depth-1,..g}}
fn paid(d:RetirementDemand)->Progress{Progress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[derive(Clone,Copy,PartialEq,Eq)]enum Phase{Bytes,Projection,Json,Receive,Take,CloseJson,CloseBytes,Typed}
/// 🧳️ Every decoded or undecoded owner remains in this original request until a funded transfer or close.
pub struct DrawingMemberWirePreparation{
 request:ManuallyDrop<Option<Request>>,typed_request:ManuallyDrop<Option<TypedRequest>>,
 bytes:ManuallyDrop<Option<RetainedCloneSource<Vec<u8>>>>,projection:ManuallyDrop<Option<RetainedOwnedProjection<Vec<u8>>>>,
 receiver:ManuallyDrop<Option<OriginalJsonReceiving<M>>>,received:ManuallyDrop<Option<M>>,
 received_factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
 typed:ManuallyDrop<Option<TypedFactory>>,inner:ManuallyDrop<Option<Box<dyn ArtifactStoreOneItemPreparation<P,M>>>>,
 wire:ManuallyDrop<Option<Wire>>,base:ManuallyDrop<Option<SnapshotRead<P>>>,authority:ManuallyDrop<Option<Arc<ArtifactStoreOneItemLiveAuthority>>>,
 mutations:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,snapshots:ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
 active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,factory:ManuallyDrop<Option<FactoryAuthority>>,
 ledger:Option<u64>,recipient:Option<u64>,phase:Phase,closing:bool,turns:u32,
}
impl MemberStoreOneItemWirePreparationFactory<P,M> for DrawingMemberWirePreparationFactory{
 fn preflight(&self,w:&Wire,lane:HistoryLane)->Result<ArtifactStoreOneItemFootprint,String>{if w.schema!=DRAWING_MEMBER_OPERATION_SCHEMA||lane!=HistoryLane::Document||w.bytes.len()<3{return Err("drawing original operation schema, lane or envelope is not admitted".into())}Ok(ArtifactStoreOneItemFootprint{work_items:(w.bytes.len()/3).saturating_add(2),retained_bytes:w.bytes.len().saturating_mul(16).saturating_add((w.bytes.len()/3).saturating_add(1).saturating_mul(semio_s_artifact_draw_drawing_mutations::retained_drag::retained_target_row_bytes())).saturating_add(size_of::<DrawingMemberWirePreparation>())})}
 fn begin_demand(&self,_:&Wire,_:HistoryLane)->Result<RetainedCloneBirthDemand,ValueError>{Ok(RetainedCloneBirthDemand{capacity_bytes:size_of::<DrawingMemberWirePreparation>(),depth:1})}
 fn begin_copy_byte_demand(&self,_:&Wire,_:HistoryLane)->Result<usize,ValueError>{size_of::<Request>().checked_mul(2).and_then(|n|n.checked_add(size_of::<DrawingMemberWirePreparation>())).ok_or_else(||invalid("drawing original wire constructor extent overflow"))}
 fn begin(&self,request:Request,g:Grant)->Result<(Box<dyn ArtifactStoreOneItemPreparation<P,M>>,Progress),(ValueError,Request)>{
  if request.mutation.schema!=DRAWING_MEMBER_OPERATION_SCHEMA||request.lane!=HistoryLane::Document||request.mutation.bytes.len()<3{return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"drawing original wire schema is not admitted"),request))}
  let d=match self.begin_demand(&request.mutation,request.lane){Ok(d)=>d,Err(e)=>return Err((e,request))};let mut p=match d.admit(g.retained_grant()){Ok(p)=>p,Err(e)=>return Err((e,request))};let copy=match self.begin_copy_byte_demand(&request.mutation,request.lane){Ok(n)=>n,Err(e)=>return Err((e,request))};if g.maximum_copy_bytes<copy{return Err((invalid("drawing original wire header transfer is not funded"),request))}p.copied_bytes=copy;
  Ok((Box::new(DrawingMemberWirePreparation{request:ManuallyDrop::new(Some(request)),typed_request:ManuallyDrop::new(None),bytes:ManuallyDrop::new(None),projection:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),received:ManuallyDrop::new(None),received_factory:ManuallyDrop::new(None),typed:ManuallyDrop::new(Some(Arc::clone(&self.typed))),inner:ManuallyDrop::new(None),wire:ManuallyDrop::new(None),base:ManuallyDrop::new(None),authority:ManuallyDrop::new(None),mutations:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(None),active:ManuallyDrop::new(None),factory:ManuallyDrop::new(None),ledger:None,recipient:None,phase:Phase::Bytes,closing:false,turns:0}),p))
 }
}
impl DrawingMemberWirePreparation{
 fn original(&self,c:&NativeDecodeControl<'_>)->Result<(),ValueError>{if c.retirement_recipient_identity().is_none()||self.ledger.is_some_and(|id|id!=c.ledger_identity()||self.recipient!=c.retirement_recipient_identity()){return Err(invalid("drawing wire changed its original decoder ledger"))}Ok(())}
 fn advance_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  Ok(match self.phase{
   Phase::Bytes=>{let b=RetainedCloneSource::<Vec<u8>>::owned_constructor_demand::<()>();RetirementDemand{capacity_bytes:b.capacity_bytes,copy_bytes:RetainedCloneSource::<Vec<u8>>::constructor_copy_bytes()+frame::<Option<RetainedCloneSource<Vec<u8>>>>().copy_bytes+frame::<Vec<u8>>().copy_bytes,depth:b.depth.max(1),..Default::default()}}
   Phase::Projection=>{let mut d=RetainedOwnedProjection::<Vec<u8>>::constructor_demand();d.copy_bytes+=frame::<Option<RetainedOwnedProjection<Vec<u8>>>>().copy_bytes;d}
   Phase::Json=>{let mut d=OriginalJsonReceiving::<M>::constructor_demand();d.copy_bytes+=frame::<Option<OriginalJsonReceiving<M>>>().copy_bytes;d}
   Phase::Receive=>self.receiver.as_ref().ok_or_else(||invalid("drawing wire lost its original JSON owner"))?.demands(body)?,
   Phase::Take=>{let mut d=self.receiver.as_ref().ok_or_else(||invalid("drawing wire lost its original JSON output"))?.take_original_demand(body)?;d.copy_bytes=d.copy_bytes.checked_add(frame::<Option<M>>().copy_bytes+frame::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>().copy_bytes).ok_or_else(||invalid("drawing original output header extent overflow"))?;d}
   Phase::CloseJson=>{let r=self.receiver.as_ref().ok_or_else(||invalid("drawing wire lost its closing JSON owner"))?;if r.terminal_is_empty(){frame::<Option<OriginalJsonReceiving<M>>>()}else{r.close_demands(body)?}}
   Phase::CloseBytes=>{let b=self.bytes.as_ref().ok_or_else(||invalid("drawing wire lost its original byte owner"))?;if b.terminal_is_empty(){frame::<Option<RetainedCloneSource<Vec<u8>>>>()}else{b.next_close_demand(body)?}}
   Phase::Typed=>{if let Some(r)=self.typed_request.as_ref(){let f=self.typed.as_ref().ok_or_else(||invalid("drawing wire lost its shared typed issuer"))?;let b=f.begin_demand(&r.mutation,r.lane)?;RetirementDemand{capacity_bytes:b.capacity_bytes,copy_bytes:f.begin_copy_byte_demand(&r.mutation,r.lane)?,depth:b.depth,..Default::default()}}else{frame::<(Request,TypedRequest,Option<M>)>()}}
  })
 }
 fn progress(&mut self,p:Progress)->Step{self.turns=self.turns.saturating_add(1);Step::Progress(self.checkpoint(),p)}
 fn wire_demand(w:&Wire)->RetirementDemand{RetirementDemand{release_bytes:if w.bytes.capacity()!=0{w.bytes.capacity()}else{w.schema.capacity()},depth:1,..Default::default()}}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(inner)=self.inner.as_ref(){return if inner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(inner.as_ref()),depth:1,..Default::default()})}else{nested(RetirementDemand{copy_bytes:inner.next_close_copy_byte_demand()?,capacity_bytes:inner.next_close_capacity_byte_demand(body)?,release_bytes:inner.next_close_release_byte_demand()?,depth:inner.next_close_depth_demand()?})}}
  if let Some(r)=self.receiver.as_ref(){return if r.terminal_is_empty(){Ok(frame::<Option<OriginalJsonReceiving<M>>>())}else{nested(r.close_demands(body)?)}}
  if let Some(p)=self.projection.as_ref(){return if p.terminal_is_empty(){Ok(frame::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{nested(p.close_demand(body)?)}}
  if let Some(b)=self.bytes.as_ref(){return if b.terminal_is_empty(){Ok(frame::<Option<RetainedCloneSource<Vec<u8>>>>())}else{nested(b.next_close_demand(body)?)}}
  if self.request.is_some(){return Ok(frame::<Request>())}if self.typed_request.is_some(){return Ok(frame::<TypedRequest>())}
  if let Some(a)=self.active.as_ref(){return if a.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(a.as_ref()),depth:1,..Default::default()})}else{nested(a.next_demand(body)?)}}
  if self.received.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::factory::owned::FactoryOwnedRetirement::<M>::constructor_capacity_bytes(),depth:2,..Default::default()})}
  if let Some(w)=self.wire.as_ref(){return Ok(if w.bytes.capacity()!=0||w.schema.capacity()!=0{Self::wire_demand(w)}else{frame::<Option<Wire>>()})}
  if self.base.is_some(){return Ok(frame::<Option<SnapshotRead<P>>>())}
  if let Some(a)=self.authority.as_ref(){let d=a.retirement_birth_demand();return Ok(RetirementDemand{capacity_bytes:d.capacity_bytes,depth:d.depth+1,..Default::default()})}
  if let Some(f)=self.factory.as_ref(){return nested(f.demands(body)?)}
  if self.received_factory.is_some()||self.mutations.is_some()||self.snapshots.is_some()||self.typed.is_some(){return Ok(frame::<Option<FactoryAuthority>>())}Ok(Default::default())
 }
 fn close_original(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.ledger.is_some_and(|id|id!=o.ledger_identity()||self.recipient!=o.retirement_recipient_identity()){return Err(invalid("drawing wire cancellation changed its original decoder ledger"))}self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let c=child(g);
  if let Some(i)=self.inner.as_mut(){if i.terminal_is_empty(){self.inner.take();return Ok(RetainedCloneStep::Progress(paid(d)))}let s=i.close_original_step(o,Grant::from_retained(c))?;return Ok(RetainedCloneStep::Progress(s.progress()))}
  if let Some(r)=self.receiver.as_mut(){if r.terminal_is_empty(){self.receiver.take();return Ok(RetainedCloneStep::Progress(paid(d)))}return r.close_step(o,c).map(|s|RetainedCloneStep::Progress(s.progress()))}
  o.charge(d.capacity_bytes)?;
  if let Some(p)=self.projection.as_mut(){if p.terminal_is_empty(){self.projection.take();return Ok(RetainedCloneStep::Progress(paid(d)))}return p.close_step(c).map(|s|RetainedCloneStep::Progress(s.progress()))}
  if let Some(b)=self.bytes.as_mut(){if b.terminal_is_empty(){self.bytes.take();return Ok(RetainedCloneStep::Progress(paid(d)))}return b.close_step(c).map(|s|RetainedCloneStep::Progress(s.progress()))}
  if let Some(r)=self.request.take(){let Request{operation:_,generation:_,base_revision:_,lane:_,authority,base,mutation,mutation_retirement,snapshot_retirement}=r;*self.wire=Some(mutation);*self.base=Some(base);*self.authority=Some(authority);*self.mutations=Some(mutation_retirement);*self.snapshots=Some(snapshot_retirement);return Ok(RetainedCloneStep::Progress(paid(d)))}
  if let Some(r)=self.typed_request.take(){let TypedRequest{operation:_,generation:_,base_revision:_,lane:_,authority,base,mutation,mutation_retirement,snapshot_retirement}=r;*self.received=Some(mutation);*self.base=Some(base);*self.authority=Some(authority);*self.mutations=Some(mutation_retirement);*self.snapshots=Some(snapshot_retirement);return Ok(RetainedCloneStep::Progress(paid(d)))}
  if let Some(a)=self.active.as_mut(){if a.terminal_is_empty(){self.active.take();return Ok(RetainedCloneStep::Progress(paid(d)))}return a.close_step(c).map(|s|RetainedCloneStep::Progress(s.progress()))}
  if self.received.is_some(){let f=self.received_factory.as_ref().or(self.mutations.as_ref()).ok_or_else(||invalid("drawing received mutation lost its original issuer"))?;if let Some((owner,p))=semio_framework_value::factory::owned::FactoryOwnedRetirement::admit_original(&mut self.received,f,c)?{*self.active=Some(owner);return Ok(RetainedCloneStep::Progress(p))}return Err(invalid("drawing received mutation retirement lost its original"))}
  if let Some(w)=self.wire.as_mut(){if w.bytes.capacity()!=0{w.bytes=Vec::new()}else if w.schema.capacity()!=0{w.schema=String::new()}else{self.wire.take();}return Ok(RetainedCloneStep::Progress(paid(d)))}
  if let Some(b)=self.base.take(){return match b.try_return_to_registry_witness(){Ok(_)=>Ok(RetainedCloneStep::Progress(paid(d))),Err(b)=>{*self.base=Some(b);Ok(RetainedCloneStep::Progress(Default::default()))}}}
  if let Some(a)=self.authority.take(){return match a.retire(c){Ok((owner,p))=>{*self.active=Some(owner);Ok(RetainedCloneStep::Progress(p))},Err((e,a))=>{*self.authority=Some(a);Err(e)}}}
  if let Some(f)=self.factory.as_mut(){let s=f.step(c)?;if f.terminal_is_empty(){self.factory.take();}return Ok(RetainedCloneStep::Progress(s.progress()))}
  let f:Option<Arc<dyn FactoryRetirement>>=if let Some(f)=self.received_factory.take(){Some(f)}else if let Some(f)=self.mutations.take(){Some(f)}else if let Some(f)=self.snapshots.take(){Some(f)}else{self.typed.take().map(|f|f as Arc<dyn FactoryRetirement>)};if let Some(f)=f{*self.factory=Some(FactoryAuthority::new(f));return Ok(RetainedCloneStep::Progress(paid(d)))}Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl ArtifactStoreOneItemPreparation<P,M> for DrawingMemberWirePreparation{
 fn advance(&mut self,body:usize,g:Grant,c:&mut NativeDecodeControl<'_>)->Result<Step,ValueError>{
  self.original(c)?;if self.closing||!g.permits_one(){return Ok(Step::Blocked)}if let Some(inner)=self.inner.as_mut(){if g.maximum_depth<2{return Ok(Step::Blocked)}return match inner.advance(body,Grant{maximum_depth:g.maximum_depth-1,..g},c)?{Step::Progress(_,p)=>Ok(Step::Progress(self.checkpoint(),p)),Step::Prepared(_,p)=>Ok(Step::Prepared(self.checkpoint(),p)),Step::Blocked=>Ok(Step::Blocked)}}let d=self.advance_demand(body)?;if !retained_clone_grant_admits(g.retained_grant(),d){return Ok(Step::Blocked)}self.ledger=Some(c.ledger_identity());self.recipient=c.retirement_recipient_identity();let reserve=match self.phase{Phase::Bytes=>frame::<Option<RetainedCloneSource<Vec<u8>>>>().copy_bytes+frame::<Vec<u8>>().copy_bytes,Phase::Projection=>frame::<Option<RetainedOwnedProjection<Vec<u8>>>>().copy_bytes,Phase::Json=>frame::<Option<OriginalJsonReceiving<M>>>().copy_bytes,Phase::Take=>frame::<Option<M>>().copy_bytes+frame::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>().copy_bytes,_=>0};let cg=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-reserve,..g.retained_grant()};
  let mut p=match self.phase{
   Phase::Bytes=>{c.checkpoint()?;c.admit_turn_capacity(g.maximum_capacity_bytes)?;c.charge(d.capacity_bytes)?;let r=self.request.as_mut().ok_or_else(||invalid("drawing wire lost its original request"))?;let bytes=std::mem::take(&mut r.mutation.bytes);match RetainedCloneSource::admit_owned(bytes,(),cg){Ok((source,p))=>{*self.bytes=Some(source);self.phase=Phase::Projection;p},Err((e,bytes,()))=>{r.mutation.bytes=bytes;return Err(e)}}}
   Phase::Projection=>{c.checkpoint()?;let (p,receipt)=self.bytes.as_ref().ok_or_else(||invalid("drawing wire lost its original source"))?.retain_original(cg)?;*self.projection=Some(p);self.phase=Phase::Json;receipt}
   Phase::Json=>{c.checkpoint()?;let p=self.projection.take().ok_or_else(||invalid("drawing wire lost its original projection"))?;let f=&self.request.as_ref().ok_or_else(||invalid("drawing wire lost its mutation issuer"))?.mutation_retirement;match OriginalJsonReceiving::admit(p,f,b"op",|m:&M|m,c,cg){Ok((receiver,p))=>{*self.receiver=Some(receiver);self.phase=Phase::Receive;p},Err((e,p))=>{*self.projection=Some(p);return Err(e)}}}
   Phase::Receive=>{let r=self.receiver.as_mut().ok_or_else(||invalid("drawing original JSON owner vanished"))?;let s=r.advance(c,body,cg)?;if r.ready(){self.phase=Phase::Take}s.progress()}
   Phase::Take=>{let r=self.receiver.as_mut().ok_or_else(||invalid("drawing original JSON output vanished"))?;let (s,out)=r.take_original(c,body,cg)?;if let Some((m,f))=out{*self.received=Some(m);*self.received_factory=Some(f);self.phase=Phase::CloseJson;}s.progress()}
   Phase::CloseJson=>{let r=self.receiver.as_mut().ok_or_else(||invalid("drawing closing original JSON owner vanished"))?;if r.terminal_is_empty(){self.receiver.take();self.phase=Phase::CloseBytes;paid(d)}else{c.with_retirement(cg,|o|o.with_original_close(|o|r.close_step(o,cg)))?.progress()}}
   Phase::CloseBytes=>{let b=self.bytes.as_mut().ok_or_else(||invalid("drawing original byte owner vanished"))?;if b.terminal_is_empty(){self.bytes.take();self.phase=Phase::Typed;paid(d)}else{c.with_retirement(cg,|o|o.with_original_close(|o|{let d=b.next_close_demand(body)?;o.charge(d.capacity_bytes)?;b.close_step(cg)}))?.progress()}}
   Phase::Typed=>{if self.typed_request.is_none(){c.checkpoint()?;let r=self.request.take().ok_or_else(||invalid("drawing wire lost its original request before typed handoff"))?;let m=self.received.take().ok_or_else(||invalid("drawing wire lost its original native mutation"))?;let Request{operation,generation,base_revision,lane,authority,base,mutation,mutation_retirement,snapshot_retirement}=r;*self.wire=Some(mutation);*self.typed_request=Some(TypedRequest{operation,generation,base_revision,lane,authority,base,mutation:m,mutation_retirement,snapshot_retirement});paid(d)}else{c.checkpoint()?;c.admit_turn_capacity(g.maximum_capacity_bytes)?;c.charge(d.capacity_bytes)?;let f=self.typed.as_ref().ok_or_else(||invalid("drawing wire lost its shared semantic issuer"))?;let r=self.typed_request.take().ok_or_else(||invalid("drawing typed handoff vanished"))?;match f.begin(r,g){Ok((inner,p))=>{*self.inner=Some(inner);p},Err((e,r))=>{*self.typed_request=Some(r);self.closing=true;return Err(e)}}}}
  };p.copied_bytes=p.copied_bytes.checked_add(reserve).ok_or_else(||invalid("drawing original wire receipt extent overflow"))?;if !p.fits(g.retained_grant()){self.closing=true;return Err(invalid("drawing original wire child exceeded its caller grant").with_retained_progress(p))}Ok(self.progress(p))
 }
 fn outcome_messages(&self)->&[semio_framework_os_spr::MutationMessage]{self.inner.as_ref().map_or(&[],|i|i.outcome_messages())}
 fn checkpoint(&self)->ArtifactStoreOneItemCheckpoint{let mut c=self.inner.as_ref().map_or_else(ArtifactStoreOneItemCheckpoint::default,|i|i.checkpoint());c.cursor=c.cursor.saturating_add(self.turns);c.completed_items=c.completed_items.saturating_add(self.turns);c}
 fn prepared(&self)->Option<&ArtifactStoreOneItemPrepared<P,M>>{self.inner.as_ref().and_then(|i|i.prepared())}
 fn take_prepared(&mut self)->Option<ArtifactStoreOneItemPrepared<P,M>>{self.inner.as_mut().and_then(|i|i.take_prepared())}
 fn cancel(&mut self){self.begin_close();if let Some(i)=self.inner.as_mut(){i.cancel()}}
 fn begin_close(&mut self){self.closing=true;if let Some(r)=self.receiver.as_mut(){r.begin_close()}if let Some(i)=self.inner.as_mut(){i.begin_close()}}
 fn close_original_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:Grant)->Result<RetainedCloneStep,ValueError>{self.close_original(o,g.retained_grant())}
 fn close_step(&mut self,_:Grant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){Ok(RetainedCloneStep::Complete(Default::default()))}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"drawing wire close requires its actual original decoder cleanup loan"))}}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.request.is_none()&&self.typed_request.is_none()&&self.bytes.is_none()&&self.projection.is_none()&&self.receiver.is_none()&&self.received.is_none()&&self.received_factory.is_none()&&self.typed.is_none()&&self.inner.is_none()&&self.wire.is_none()&&self.base.is_none()&&self.authority.is_none()&&self.mutations.is_none()&&self.snapshots.is_none()&&self.active.is_none()&&self.factory.is_none()}
}

impl Drop for DrawingMemberWirePreparation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Drawing wire request, decoder, typed value and exact issuers require funded closure")}}
