//! 🔌️ Shared semantic preparation delegates to original native projection and edit cursors.
use super::*;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor};
use semio_framework_value_snapshot_projection::snapshot_projection::{SnapshotProjection, SnapshotProjectionCursor, ProjectionOutput};
use semio_framework_os_store_one_item::{PagedOneItemEdit,PagedOneItemEditStep,PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
use semio_framework_os_store_snapshot_clone::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory};
use super::super::{ArtifactReplayPreparation,ArtifactReplayPreparationFactory};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_os_store_command::HistoryLane;

pub struct PagedProjection<P:SnapshotProjection>{cursor:Option<P::Projection>,closing:bool}
impl<P:SnapshotProjection> Default for PagedProjection<P>{fn default()->Self{Self{cursor:None,closing:false}}}
impl<P:SnapshotProjection<Output=P>> SemanticProjection<P> for PagedProjection<P>{
 fn birth_demand(&self)->RetirementDemand{P::projection_birth_demand()}
 fn begin(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let(c,p)=P::begin_projection(g)?;self.cursor=Some(c);Ok(p)}
 fn demands(&self,s:RetainedCloneRef<'_,P>,b:usize)->Result<RetirementDemand,ValueError>{self.cursor.as_ref().ok_or_else(||invalid("semantic projection cursor absent"))?.demands(s,b)}
 fn advance(&mut self,s:RetainedCloneRef<'_,P>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.cursor.as_mut().ok_or_else(||invalid("semantic projection cursor absent"))?.advance(s,body,g)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{let a=self.cursor.as_ref().ok_or_else(||invalid("semantic projection cursor absent"))?.take_demand()?;let b=<P::Output as ProjectionOutput<P>>::placement_demand();Ok(RetirementDemand{copy_bytes:a.copy_bytes.checked_add(b.copy_bytes).ok_or_else(||invalid("semantic projection transfer copy overflow"))?,capacity_bytes:a.capacity_bytes.checked_add(b.capacity_bytes).ok_or_else(||invalid("semantic projection transfer capacity overflow"))?,release_bytes:a.release_bytes.checked_add(b.release_bytes).ok_or_else(||invalid("semantic projection transfer release overflow"))?,depth:a.depth.max(b.depth)})}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(P,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None)}let placement=<P::Output as ProjectionOutput<P>>::placement_demand();let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-placement.copy_bytes,maximum_capacity_bytes:g.maximum_capacity_bytes-placement.capacity_bytes,maximum_release_bytes:g.maximum_release_bytes-placement.release_bytes,..g};let Some((output,progress))=self.cursor.as_mut().ok_or_else(||invalid("semantic projection cursor absent"))?.take(child)?else{return Ok(None)};let mut target=std::mem::MaybeUninit::uninit();let placed=output.place(&mut target,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:placement.copy_bytes,maximum_capacity_bytes:placement.capacity_bytes,maximum_release_bytes:placement.release_bytes,maximum_depth:placement.depth}).unwrap_or_else(|_|panic!("sealed original projection placement changed its admitted demand"));let mut total=progress.checked_add(placed)?;total.copied_items=1;Ok(Some((unsafe{target.assume_init()},total)))}
 fn begin_close(&mut self){self.closing=true;if let Some(c)=self.cursor.as_mut(){c.begin_close();}}
 fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{match self.cursor.as_ref(){Some(c)if !c.terminal_is_empty()=>c.close_demands(b),Some(_)=>Ok(header::<Option<P::Projection>>()),None=>Ok(Default::default())}}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}match self.cursor.as_mut(){Some(c)if !c.terminal_is_empty()=>c.close_step(g),Some(_)=>{self.cursor=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))},None=>Ok(RetainedCloneStep::Complete(Default::default()))}}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()}
}
pub struct ClonedProjection<P:RetainedClone>{cursor:Option<Box<P::Cursor>>,birth:semio_framework_value::retained_clone::RetainedCloneBirthDemand,closing:bool}
impl<P:RetainedClone> ClonedProjection<P>{pub fn new(birth:semio_framework_value::retained_clone::RetainedCloneBirthDemand)->Self{Self{cursor:None,birth,closing:false}}}
impl<P:RetainedClone> SemanticProjection<P> for ClonedProjection<P>{
 fn birth_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<P::Cursor>()+size_of::<Box<P::Cursor>>(),capacity_bytes:self.birth.capacity_bytes+size_of::<P::Cursor>(),depth:self.birth.depth.max(1),..Default::default()}}
 fn begin(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid("semantic retained clone cursor birth not funded"));}self.cursor=Some(Box::new(P::retained_clone_cursor()));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})}
 fn demands(&self,s:RetainedCloneRef<'_,P>,b:usize)->Result<RetirementDemand,ValueError>{self.cursor.as_ref().ok_or_else(||invalid("semantic retained clone cursor absent"))?.advance_demands(s,b)}
 fn advance(&mut self,s:RetainedCloneRef<'_,P>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.cursor.as_mut().ok_or_else(||invalid("semantic retained clone cursor absent"))?.advance(s,body,g)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<P>()*2,depth:1,..Default::default()})}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(P,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}Ok(self.cursor.as_mut().ok_or_else(||invalid("semantic retained clone cursor absent"))?.take().map(|p|(p,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 fn begin_close(&mut self){self.closing=true;if let Some(c)=self.cursor.as_mut(){let _=c.begin_close();}}
 fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{match self.cursor.as_ref(){Some(c)if !c.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:c.next_close_copy_byte_demand()?,capacity_bytes:c.next_close_capacity_byte_demand(b)?,release_bytes:c.next_close_release_byte_demand()?,depth:c.next_close_depth_demand()?}),Some(_)=>Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<P::Cursor>>>(),release_bytes:size_of::<P::Cursor>(),depth:1,..Default::default()}),None=>Ok(Default::default())}}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}match self.cursor.as_mut(){Some(c)if !c.terminal_is_empty()=>c.close_step(g),Some(_)=>{self.cursor=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))},None=>Ok(RetainedCloneStep::Complete(Default::default()))}}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()}
}
pub struct PagedSemanticEdit<P,M,E:PagedOneItemEdit<P,M>>{edit:E,closing:bool,marker:std::marker::PhantomData<fn()->(P,M)>}
impl<P,M,E:PagedOneItemEdit<P,M>> Default for PagedSemanticEdit<P,M,E>{fn default()->Self{Self{edit:E::default(),closing:false,marker:Default::default()}}}
impl<P:'static,M:'static,E:PagedOneItemEdit<P,M>> SemanticEdit<P,M> for PagedSemanticEdit<P,M,E>{
 fn birth_demand(&self)->RetirementDemand{header::<bool>()}
 fn begin(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid("semantic edit stage not funded"));}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
 fn post_mode(&self)->PagedOneItemPostMode{self.edit.post_mode()}
 fn demands(&self,b:RetainedCloneRef<'_,P>,m:RetainedCloneRef<'_,M>,p:PagedOneItemPostRead<'_,P>,body:usize)->Result<RetirementDemand,ValueError>{self.edit.advance_demands(b,m,p,body)}
 fn advance(&mut self,b:RetainedCloneRef<'_,P>,m:RetainedCloneRef<'_,M>,p:PagedOneItemPost<'_,P>,body:usize,g:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<SemanticEditStep<M>,ValueError>{Ok(match self.edit.advance(b,m,p,body,g,receiving)?{PagedOneItemEditStep::Progress(r)=>SemanticEditStep::Progress(r),PagedOneItemEditStep::Complete(r,row)=>SemanticEditStep::CompleteSingle(r,row),PagedOneItemEditStep::CompleteRows(r,rows)=>SemanticEditStep::Complete(r,rows,Vec::new(),false),PagedOneItemEditStep::CompleteOutcome(r,rows,messages)=>SemanticEditStep::Complete(r,rows,messages,false),PagedOneItemEditStep::RefusedOutcome(r,messages)=>SemanticEditStep::Refused(r,messages)})}
 fn begin_close(&mut self){self.closing=true;self.edit.begin_close();}
 fn close_demand(&self,f:&dyn ArtifactOwnedValueRetirementFactory<M>,b:usize)->Result<RetirementDemand,ValueError>{self.edit.close_demands(f,b)}
 fn close_step(&mut self,f:&dyn ArtifactOwnedValueRetirementFactory<M>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.edit.close_step(f,g)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.edit.terminal_is_empty()}
}
pub struct ClonedSemanticEdit<P:RetainedClone,M,E:RetainedCloneEdit<P,M>>{factory:Option<Arc<E>>,cursor:Option<Box<E::Cursor>>,factory_close:Option<semio_framework_value::FactoryAuthority>,closing:bool}
impl<P:RetainedClone,M,E:RetainedCloneEdit<P,M>> ClonedSemanticEdit<P,M,E>{pub fn new(factory:Arc<E>)->Self{Self{factory:Some(factory),cursor:None,factory_close:None,closing:false}}}
impl<P:RetainedClone,M:'static,E:RetainedCloneEdit<P,M>> SemanticEdit<P,M> for ClonedSemanticEdit<P,M,E>{
 fn birth_demand(&self)->RetirementDemand{let d=self.factory.as_ref().unwrap().begin_demand();RetirementDemand{copy_bytes:size_of::<E::Cursor>()+size_of::<Box<E::Cursor>>(),capacity_bytes:d.capacity_bytes+size_of::<E::Cursor>(),depth:d.depth.max(1),..Default::default()}}
 fn begin(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid("semantic retained edit birth not funded"));}let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<Box<E::Cursor>>(),maximum_capacity_bytes:g.maximum_capacity_bytes-size_of::<E::Cursor>(),..g};let(c,mut p)=self.factory.as_ref().unwrap().begin(child)?;self.cursor=Some(Box::new(c));p.copied_bytes=p.copied_bytes.max(d.copy_bytes);p.retained_capacity_bytes+=size_of::<E::Cursor>();Ok(p)}
 fn post_mode(&self)->PagedOneItemPostMode{PagedOneItemPostMode::Mutable}
 fn demands(&self,b:RetainedCloneRef<'_,P>,m:RetainedCloneRef<'_,M>,p:&P,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.as_ref().ok_or_else(||invalid("semantic retained edit cursor absent"))?.advance_demands(b,p,m,body)}
 fn advance(&mut self,b:RetainedCloneRef<'_,P>,m:RetainedCloneRef<'_,M>,p:&mut P,body:usize,g:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<SemanticEditStep<M>,ValueError>{let c=self.cursor.as_mut().ok_or_else(||invalid("semantic retained edit cursor absent"))?;let d=c.advance_demands(b,p,m,body)?;if !retained_clone_grant_admits(g,d){return Ok(SemanticEditStep::Progress(Default::default()));}receiving.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0{receiving.charge(d.capacity_bytes)?;}Ok(match c.advance(b,p,m,body,g)?{RetainedCloneEditStep::Progress(r)=>SemanticEditStep::Progress(r),RetainedCloneEditStep::Complete(r)=>{let rows=c.take_inverse().ok_or_else(||invalid("semantic retained edit omitted inverse"))?;SemanticEditStep::Complete(r,rows,Vec::new(),c.foreign_step_presence())}})}
 fn begin_close(&mut self){self.closing=true;if let Some(c)=self.cursor.as_mut(){c.cancel();let _=c.begin_close();}}
 fn close_demand(&self,_:&dyn ArtifactOwnedValueRetirementFactory<M>,b:usize)->Result<RetirementDemand,ValueError>{if let Some(c)=self.cursor.as_ref(){return if c.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<E::Cursor>>>(),release_bytes:size_of::<E::Cursor>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:c.next_close_copy_byte_demand()?,capacity_bytes:c.next_close_capacity_byte_demand(b)?,release_bytes:c.next_close_release_byte_demand()?,depth:c.next_close_depth_demand()?})};}if self.factory.is_some(){return Ok(header::<Arc<E>>());}self.factory_close.as_ref().map_or(Ok(Default::default()),|f|f.demands(b))}
 fn close_step(&mut self,f:&dyn ArtifactOwnedValueRetirementFactory<M>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demand(f,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(c)=self.cursor.as_mut(){if !c.terminal_is_empty(){return c.close_step(g);}self.cursor=None;}else if let Some(f)=self.factory.take(){self.factory_close=Some(semio_framework_value::FactoryAuthority::new(f));}else if let Some(f)=self.factory_close.as_mut(){let s=f.step(g)?;if f.terminal_is_empty(){self.factory_close=None;}return Ok(s);}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}));}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()&&self.factory.is_none()&&self.factory_close.is_none()}
}

/// 🪢 Funds the metadata-free native semantic producer from the retained clone preparation factory.
impl<P,M,E> ArtifactReplayPreparationFactory<P,M> for RetainedClonePreparationFactory<P,M,E>
where P:RetainedClone,M:ArtifactCanonicalJsonTree+RetireOwned+Send+Sync+'static,E:RetainedCloneEdit<P,M>{
 fn preparation_birth_demand(&self)->semio_framework_value::RetirementDemand{
  type Producer<P,M,E>=NativeSemanticPreparation<P,M,ClonedProjection<P>,ClonedSemanticEdit<P,M,E>>;
  semio_framework_value::RetirementDemand{copy_bytes:size_of::<ArtifactReplayPreparationRequest<P,M>>()+size_of::<Producer<P,M,E>>()+size_of::<Arc<E>>(),capacity_bytes:size_of::<Producer<P,M,E>>(),depth:1,..Default::default()}
 }
 fn begin(&self,request:ArtifactReplayPreparationRequest<P,M>,grant:RetainedCloneGrant)->Result<(Box<dyn ArtifactReplayPreparation<P,M>>,RetainedCloneProgress),(ValueError,ArtifactReplayPreparationRequest<P,M>)>{
  let d=ArtifactReplayPreparationFactory::preparation_birth_demand(self);
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"semantic retained clone producer birth not funded"),request));}
  let refusal=match request.input.borrow(){Ok(input)=>match self.edit.preflight(input.get(),HistoryLane::Document){Ok(footprint)if footprint.is_admissible()=>None,Ok(_)=>Some("semantic retained clone operation exceeds the one-item footprint"),Err(reason)=>Some(reason)},Err(error)=>return Err((error,request))};
  let projection=ClonedProjection::<P>::new(self.edit.snapshot_cursor_birth_demand());
  let edit=ClonedSemanticEdit::<P,M,E>::new(Arc::clone(&self.edit));
  let producer=match refusal{Some(reason)=>NativeSemanticPreparation::refused(request,projection,edit,reason),None=>NativeSemanticPreparation::new(request,projection,edit)};
  Ok((Box::new(producer),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
 }
}
