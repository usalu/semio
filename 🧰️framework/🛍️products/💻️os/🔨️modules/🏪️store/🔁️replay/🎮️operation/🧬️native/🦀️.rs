//! 🧬️ Metadata-free semantic preparation uses original snapshot and operation leases.
use super::{ArtifactReplayPreparationContext,ArtifactReplayPrepared,ArtifactReplayPreparationStep};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement};
use semio_framework_value_snapshot_read::{CapturedSnapshotSource,SnapshotRead};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned, retained_clone::{RetainedCloneRef, RetainedCloneSource, RetainedOwnedProjection, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedCloneClose, retained_clone_grant_admits, admit_retained_clone_progress}, retirement::original_vector::OriginalVectorCursor};
use semio_framework_value_persistent::list::PagedList;
use std::{mem::{size_of,ManuallyDrop},sync::Arc};
use semio_framework_os_store_one_item::{OriginalPostRead,PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
#[path="🔌️drivers/🦀️.rs"]
mod drivers;
#[path="🔗️input/🦀️.rs"]
mod input;
pub use input::ArtifactMutationSource;
#[path="🔗️history/🦀️.rs"]
pub mod history;
pub use drivers::{PagedProjection,PagedSemanticEdit,ClonedProjection,ClonedSemanticEdit};

/// 📨️ Captures original authority without publication actor, edit, revision, or sealer metadata.
pub struct ArtifactReplayPreparationRequest<P,M:Sync>{
 pub base:SnapshotRead<P>,
 pub input:RetainedOwnedProjection<M>,
 pub context:ArtifactReplayPreparationContext,
 pub snapshots:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
 pub mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}
/// 📽️ Delegates projection to the installed snapshot or retained clone cursor.
pub trait SemanticProjection<P>:Send+'static{
 fn birth_demand(&self)->RetirementDemand;
 fn begin(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn demands(&self,source:RetainedCloneRef<'_,P>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,source:RetainedCloneRef<'_,P>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(P,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// ✏️ Delegates all semantic stages to the same native edit issuer used by publication.
pub trait SemanticEdit<P,M>:Send+'static{
 fn birth_demand(&self)->RetirementDemand;
 fn begin(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn post_mode(&self)->PagedOneItemPostMode;
 fn demands(&self,before:RetainedCloneRef<'_,P>,input:RetainedCloneRef<'_,M>,post:PagedOneItemPostRead<'_,P>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,before:RetainedCloneRef<'_,P>,input:RetainedCloneRef<'_,M>,post:PagedOneItemPost<'_,P>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<SemanticEditStep<M>,ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,factory:&dyn ArtifactOwnedValueRetirementFactory<M>,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,factory:&dyn ArtifactOwnedValueRetirementFactory<M>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 📤️ Preserves original inverse and diagnostic owners before independent output admission.
pub enum SemanticEditStep<M>{CompleteSingle(RetainedCloneProgress,M),Progress(RetainedCloneProgress),Complete(RetainedCloneProgress,Vec<M>,Vec<semio_framework_os_spr::MutationMessage>,bool),Refused(RetainedCloneProgress,Vec<semio_framework_os_spr::MutationMessage>)}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Stage{RefusalRows,RefusalCode,RefusalTextBirth,RefusalText,SourceCapture,Source,ProjectionBirth,Projection,TakeProjection,CloseProjection,EditBirth,Edit,CloseEdit,InverseBirth,Inverse,CloseInverse,Share,Ready}
/// 🧳️ One retained producer owns every original lease and candidate through cancellation.
pub struct NativeSemanticPreparation<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>>{
 base:ManuallyDrop<Option<SnapshotRead<P>>>,source:ManuallyDrop<Option<RetainedCloneSource<P,CapturedSnapshotSource<P>>>>,source_authority:ManuallyDrop<Option<CapturedSnapshotSource<P>>>,input:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
 projection:C,edit:E,post:ManuallyDrop<Option<P>>,post_read:OriginalPostRead<P>,inverse:ManuallyDrop<Option<Vec<M>>>,rows:ManuallyDrop<Option<OriginalVectorCursor<M>>>,pages:ManuallyDrop<Option<PagedList<M,{usize::MAX}>>>,pending:ManuallyDrop<Option<M>>,messages:ManuallyDrop<Option<Vec<semio_framework_os_spr::MutationMessage>>>,next:ManuallyDrop<Option<Arc<P>>>,
 snapshots:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,mutations:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,factory_close:[Option<semio_framework_value::FactoryAuthority>;2],active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,retirement:RetainedCloneClose,stage:Stage,foreign:bool,refused:bool,closing:bool,refusal:Option<&'static str>,
}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||invalid("semantic producer depth overflow"))?;Ok(d)}
fn list_error(error:semio_framework_value_persistent::list::PagedListError)->ValueError{ValueError::literal(match error.kind{semio_framework_value_persistent::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,semio_framework_value_persistent::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value_persistent::list::PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason)}
impl<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>> NativeSemanticPreparation<P,M,C,E>{
 pub fn new(request:ArtifactReplayPreparationRequest<P,M>,projection:C,edit:E)->Self{base:ManuallyDrop::new(Some(request.base)),source:ManuallyDrop::new(None),source_authority:ManuallyDrop::new(None),input:ManuallyDrop::new(Some(request.input)),projection,edit,post:ManuallyDrop::new(None),post_read:OriginalPostRead::default(),inverse:ManuallyDrop::new(None),rows:ManuallyDrop::new(None),pages:ManuallyDrop::new(Some(Default::default())),pending:ManuallyDrop::new(None),messages:ManuallyDrop::new(None),next:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(request.snapshots)),mutations:ManuallyDrop::new(Some(request.mutations)),factory_close:[None,None],active:ManuallyDrop::new(None),retirement:Default::default(),stage:Stage::SourceCapture,foreign:false,refused:false,closing:false,refusal:None}}
 pub fn refused(request:ArtifactReplayPreparationRequest<P,M>,projection:C,edit:E,reason:&'static str)->Self{let mut owner=Self::new(request,projection,edit);owner.refusal=Some(reason);owner.refused=true;owner.projection.begin_close();owner.edit.begin_close();owner.stage=Stage::RefusalRows;owner}
 fn refusal_fragment(&self,body:usize)->Result<&'static str,ValueError>{let reason=self.refusal.ok_or_else(||invalid("semantic preflight refusal absent"))?;let start=self.messages.as_ref().and_then(|rows|rows.first()).map_or(0,|row|row.message.len());let mut end=start.saturating_add(body).min(reason.len());while !reason.is_char_boundary(end){end-=1;}Ok(&reason[start..end])}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demand(body);}
  Ok(match self.stage{
   Stage::RefusalRows=>RetirementDemand{copy_bytes:size_of::<Vec<semio_framework_os_spr::MutationMessage>>()+size_of::<semio_framework_os_spr::MutationMessage>(),capacity_bytes:size_of::<semio_framework_os_spr::MutationMessage>(),depth:1,..Default::default()},
   Stage::RefusalCode=>RetirementDemand{copy_bytes:size_of::<String>()+"mutation.invariant".len(),capacity_bytes:"mutation.invariant".len(),depth:1,..Default::default()},
   Stage::RefusalTextBirth=>RetirementDemand{copy_bytes:size_of::<String>(),capacity_bytes:self.refusal.ok_or_else(||invalid("semantic preflight refusal absent"))?.len(),depth:1,..Default::default()},
   Stage::RefusalText=>{let fragment=self.refusal_fragment(body)?;if fragment.is_empty(){header::<Stage>()}else{RetirementDemand{copy_bytes:fragment.len(),depth:1,..Default::default()}}},
   Stage::SourceCapture=>CapturedSnapshotSource::<P>::constructor_demand(),
   Stage::Source=>{let d=RetainedCloneSource::<P,CapturedSnapshotSource<P>>::borrowed_constructor_demand::<CapturedSnapshotSource<P>>();RetirementDemand{copy_bytes:RetainedCloneSource::<P,CapturedSnapshotSource<P>>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()}},
   Stage::ProjectionBirth=>self.projection.birth_demand(),
   Stage::Projection=>self.projection.demands(self.source.as_ref().ok_or_else(||invalid("semantic original snapshot source absent"))?.try_borrow()?,body)?,
   Stage::TakeProjection=>self.projection.take_demand()?,
   Stage::CloseProjection=>if self.projection.terminal_is_empty(){header::<Stage>()}else{self.projection.close_demand(body)?},
   Stage::EditBirth=>self.edit.birth_demand(),
   Stage::Edit=>{let mode=self.edit.post_mode();if let Some(demand)=self.post_read.demand(&self.post,mode,body)?{demand}else{self.edit.demands(self.source.as_ref().ok_or_else(||invalid("semantic original snapshot source absent"))?.try_borrow()?,self.input.as_ref().ok_or_else(||invalid("semantic original operation source absent"))?.borrow()?,self.post_read.read(&self.post,mode)?,body)?}},
   Stage::CloseEdit=>if self.edit.terminal_is_empty(){if !self.post_read.terminal_is_empty(){self.post_read.demand(&self.post,PagedOneItemPostMode::Mutable,body)?.ok_or_else(||invalid("completed semantic post return quote absent"))?}else{header::<Stage>()}}else{self.edit.close_demand(self.mutations.as_ref().ok_or_else(||invalid("semantic mutation issuer absent"))?.as_ref(),body)?},
   Stage::InverseBirth=>if self.inverse.is_some(){OriginalVectorCursor::<M>::constructor_demand()}else{header::<Stage>()},
   Stage::Inverse=>{let pages=self.pages.as_ref().ok_or_else(||invalid("semantic inverse pages absent"))?;let rows=self.rows.as_ref();if self.pending.is_some()&&pages.has_reserved_slot(){RetirementDemand{copy_bytes:size_of::<M>(),depth:pages.next_push_depth_demand().map_err(list_error)?,..Default::default()}}else if self.pending.is_some(){RetirementDemand{copy_bytes:pages.next_reserve_copy_byte_demand().map_err(list_error)?,capacity_bytes:pages.next_allocation_bytes().map_err(list_error)?,depth:pages.next_reserve_depth_demand().map_err(list_error)?,..Default::default()}}else if rows.is_none_or(|rows|rows.remaining().is_empty()){header::<Stage>()}else if !pages.has_reserved_slot(){RetirementDemand{copy_bytes:pages.next_reserve_copy_byte_demand().map_err(list_error)?,capacity_bytes:pages.next_allocation_bytes().map_err(list_error)?,depth:pages.next_reserve_depth_demand().map_err(list_error)?,..Default::default()}}else{rows.unwrap().next_take_demand()}},
   Stage::CloseInverse=>if self.rows.as_ref().is_some_and(|rows|!rows.terminal_is_empty()){self.rows.as_ref().unwrap().next_demand(body)?}else{header::<Option<OriginalVectorCursor<M>>>()},
   Stage::Share=>if self.refused{header::<Stage>()}else{RetirementDemand{copy_bytes:size_of::<P>()+size_of::<Arc<P>>(),capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<P>(),depth:1,..Default::default()}},
   Stage::Ready=>header::<ArtifactReplayPrepared<P,M>>(),
  })
 }
 pub fn advance(&mut self,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<ArtifactReplayPreparationStep,ValueError>{
  let d=self.demands(body)?;if !retained_clone_grant_admits(grant,d){return Ok(ArtifactReplayPreparationStep::Blocked);}
  if self.closing{return Ok(ArtifactReplayPreparationStep::Blocked);}receiving.checkpoint()?;if self.stage!=Stage::Edit||self.post_read.demand(&self.post,self.edit.post_mode(),body)?.is_some(){receiving.admit_turn_capacity(grant.maximum_capacity_bytes)?;if d.capacity_bytes!=0{receiving.charge(d.capacity_bytes)?;}}
  let unit=||RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes};
  let p=match self.stage{
   Stage::RefusalRows=>{*self.messages=Some(Vec::new());let rows=self.messages.as_mut().unwrap();rows.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"semantic refusal row allocation failed"))?;rows.push(semio_framework_os_spr::MutationMessage{level:semio_framework_diagnostic::Severity::Fatal,code:semio_framework_diagnostic::FaultCode(String::new()),message:String::new(),target:Vec::new(),op_index:None});self.stage=Stage::RefusalCode;unit()},
   Stage::RefusalCode=>{let code=&mut self.messages.as_mut().unwrap()[0].code.0;code.try_reserve_exact(d.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"semantic refusal code allocation failed"))?;code.push_str("mutation.invariant");self.stage=Stage::RefusalTextBirth;unit()},
   Stage::RefusalTextBirth=>{let text=&mut self.messages.as_mut().unwrap()[0].message;text.try_reserve_exact(d.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"semantic refusal text allocation failed"))?;self.stage=Stage::RefusalText;unit()},
   Stage::RefusalText=>{let fragment=self.refusal_fragment(body)?;if fragment.is_empty()&&self.messages.as_ref().unwrap()[0].message.len()!=self.refusal.unwrap().len(){return Ok(ArtifactReplayPreparationStep::Blocked);}self.messages.as_mut().unwrap()[0].message.push_str(fragment);if self.messages.as_ref().unwrap()[0].message.len()==self.refusal.unwrap().len(){self.stage=Stage::CloseEdit;}unit()},
   Stage::SourceCapture=>{let Some((source,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref().unwrap(),grant)?else{return Ok(ArtifactReplayPreparationStep::Blocked);};*self.source_authority=Some(source);self.stage=Stage::Source;p},
   Stage::Source=>{let authority=self.source_authority.take().ok_or_else(||invalid("semantic original captured snapshot authority absent"))?;match RetainedCloneSource::<P,CapturedSnapshotSource<P>>::admit_borrowed(authority,|source:&CapturedSnapshotSource<P>|source.get(),grant){Ok((source,p))=>{*self.source=Some(source);self.stage=Stage::ProjectionBirth;p},Err((error,authority))=>{*self.source_authority=Some(authority);return Err(error);}}},
   Stage::ProjectionBirth=>{let p=self.projection.begin(grant)?;self.stage=Stage::Projection;p},
   Stage::Projection=>{let step=self.projection.advance(self.source.as_ref().unwrap().try_borrow()?,body,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.stage=Stage::TakeProjection;}step.progress()},
   Stage::TakeProjection=>{let Some((post,p))=self.projection.take(grant)?else{return Err(invalid("funded semantic projection omitted owner"));};*self.post=Some(post);self.projection.begin_close();self.stage=Stage::CloseProjection;p},
   Stage::CloseProjection=>if self.projection.terminal_is_empty(){self.stage=Stage::EditBirth;unit()}else{self.projection.close_step(grant)?.progress()},
   Stage::EditBirth=>{let p=self.edit.begin(grant)?;self.stage=Stage::Edit;p},
   Stage::Edit=>{let mode=self.edit.post_mode();if let Some(p)=self.post_read.prepare(&mut self.post,mode,body,grant)?{return Ok(ArtifactReplayPreparationStep::Pending(p));}let before=self.source.as_ref().unwrap().try_borrow()?;let input=self.input.as_ref().unwrap().borrow()?;match self.edit.advance(before,input,self.post_read.access(&mut self.post,mode)?,body,grant,receiving)?{
    SemanticEditStep::Progress(p)=>p,
    SemanticEditStep::CompleteSingle(p,row)=>{*self.pending=Some(row);*self.messages=Some(Vec::new());self.edit.begin_close();self.stage=Stage::CloseEdit;p},
    SemanticEditStep::Complete(p,inverse,messages,foreign)=>{*self.inverse=Some(inverse);*self.messages=Some(messages);self.foreign=foreign;self.edit.begin_close();self.stage=Stage::CloseEdit;p},
    SemanticEditStep::Refused(p,messages)=>{*self.inverse=Some(Vec::new());*self.messages=Some(messages);self.refused=true;self.edit.begin_close();self.stage=Stage::CloseEdit;p},
   }},
   Stage::CloseEdit=>if self.edit.terminal_is_empty(){if !self.post_read.terminal_is_empty(){self.post_read.prepare(&mut self.post,PagedOneItemPostMode::Mutable,body,grant)?.ok_or_else(||invalid("completed semantic post return absent"))?}else{self.stage=Stage::InverseBirth;unit()}}else{self.edit.close_step(self.mutations.as_ref().unwrap().as_ref(),grant)?.progress()},
   Stage::InverseBirth=>{let Some(inverse)=self.inverse.take()else{self.stage=Stage::Inverse;return Ok(ArtifactReplayPreparationStep::Pending(unit()));};match OriginalVectorCursor::admit(inverse,grant){Ok((rows,p))=>{*self.rows=Some(rows);self.stage=Stage::Inverse;p},Err((error,inverse))=>{*self.inverse=Some(inverse);return Err(error);}}},
   Stage::Inverse=>{let pages=self.pages.as_mut().unwrap();if self.pending.is_some()&&!pages.has_reserved_slot(){pages.reserve_one_funded(grant).map_err(|error|ValueError::from(error.refusal()))?}else if self.pending.is_some(){let original=self.pending.take().unwrap();if let Err(original)=pages.push_reserved(original){*self.pending=Some(original);return Err(invalid("semantic reserved inverse slot absent"));}unit()}else if self.rows.as_ref().is_none_or(|rows|rows.remaining().is_empty()){if let Some(rows)=self.rows.as_mut(){rows.begin_close();}self.stage=Stage::CloseInverse;unit()}else if !pages.has_reserved_slot(){pages.reserve_one_funded(grant).map_err(|error|ValueError::from(error.refusal()))?}else{let Some((row,p))=self.rows.as_mut().unwrap().take_front(grant)?else{return Err(invalid("funded semantic inverse transfer omitted row"));};*self.pending=Some(row);p}},
   Stage::CloseInverse=>if self.rows.as_ref().is_some_and(|rows|!rows.terminal_is_empty()){self.rows.as_mut().unwrap().step(grant)?.progress()}else{self.rows.take();self.stage=Stage::Share;unit()},
   Stage::Share=>{if !self.refused{*self.next=Some(Arc::new(self.post.take().ok_or_else(||invalid("semantic candidate absent"))?));}self.stage=Stage::Ready;unit()},
   Stage::Ready=>return Ok(ArtifactReplayPreparationStep::Prepared(Default::default())),
  };
  let p=admit_retained_clone_progress(grant,p,"metadata-free original semantic producer")?;
  Ok(if self.stage==Stage::Ready{ArtifactReplayPreparationStep::Prepared(p)}else{ArtifactReplayPreparationStep::Pending(p)})
 }
 pub fn take(&mut self)->Option<ArtifactReplayPrepared<P,M>>{if self.closing||self.stage!=Stage::Ready{return None;}self.closing=true;Some(ArtifactReplayPrepared{next:self.next.take(),inverse:Ok(self.pages.take()?),messages:self.messages.take().unwrap_or_default(),apply_refusal:None,input_refusal:None,foreign_steps:self.foreign})}
 pub fn begin_close(&mut self){self.closing=true;self.projection.begin_close();self.edit.begin_close();if let Some(rows)=self.rows.as_mut(){rows.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.projection.terminal_is_empty(){return self.projection.close_demand(body);}
  if !self.edit.terminal_is_empty(){return self.edit.close_demand(self.mutations.as_ref().ok_or_else(||invalid("semantic original mutation issuer absent"))?.as_ref(),body);}
  if !self.post_read.terminal_is_empty(){return self.post_read.demand(&self.post,PagedOneItemPostMode::Mutable,body)?.ok_or_else(||invalid("semantic original post return quote absent"));}
  if let Some(active)=self.active.as_ref(){return semio_framework_value::factory_ticket_demands(active,body);}
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  if let Some(rows)=self.rows.as_ref(){return if rows.terminal_is_empty(){Ok(header::<Option<OriginalVectorCursor<M>>>())}else{rows.next_demand(body)};}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<P,CapturedSnapshotSource<P>>>>())}else{source.next_close_demand(body)};}
  if let Some(input)=self.input.as_ref(){return if input.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{input.close_demand(body)};}
  macro_rules! pending{($field:ident,$kind:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$kind>(true,&None,body);}};}
  if self.base.is_some(){return Ok(CapturedSnapshotSource::<P>::constructor_demand());}
  pending!(source_authority,CapturedSnapshotSource<P>);
  if self.post.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()});}
  pending!(inverse,Vec<M>);pending!(pages,PagedList<M,{usize::MAX}>);
  if self.pending.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<M>::constructor_capacity_bytes(),depth:1,..Default::default()});}
  pending!(messages,Vec<semio_framework_os_spr::MutationMessage>);
  if self.next.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::factory::shared::FactorySharedRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()});}
  if self.snapshots.is_some()||self.mutations.is_some(){return Ok(header::<Arc<dyn semio_framework_value::FactoryRetirement>>());}
  if let Some(factory)=self.factory_close.iter().find_map(Option::as_ref){return factory.demands(body);}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant,receiving:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if d.capacity_bytes!=0{receiving.charge(d.capacity_bytes)?;}
  let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if !self.projection.terminal_is_empty(){return self.projection.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.edit.terminal_is_empty(){return self.edit.close_step(self.mutations.as_ref().unwrap().as_ref(),grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.post_read.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(self.post_read.prepare(&mut self.post,PagedOneItemPostMode::Mutable,grant.maximum_copy_bytes,grant)?.ok_or_else(||invalid("semantic original post return absent"))?));}
  if self.active.is_some(){return semio_framework_value::close_factory_ticket(&mut self.active,grant);}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(rows)=self.rows.as_mut(){if rows.terminal_is_empty(){self.rows.take();return Ok(unit());}return rows.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();return Ok(unit());}return source.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(input)=self.input.as_mut(){if input.terminal_is_empty(){self.input.take();return Ok(unit());}return input.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  macro_rules! close{($field:ident)=>{if let Some(step)=self.retirement.begin_granted(&mut self.$field,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}};}
  if self.base.is_some(){let Some((authority,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref().unwrap(),grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.source_authority=Some(authority);return Ok(RetainedCloneStep::Progress(p));}
  close!(source_authority);
  if self.post.is_some(){let Some((frame,p))=semio_framework_value::FactoryOwnedRetirement::<P>::admit_original(&mut self.post,self.snapshots.as_ref().unwrap(),grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.active=Some(frame);return Ok(RetainedCloneStep::Progress(p));}
  close!(inverse);close!(pages);
  if self.pending.is_some(){let Some((frame,p))=semio_framework_value::FactoryOwnedRetirement::<M>::admit_original(&mut self.pending,self.mutations.as_ref().unwrap(),grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.active=Some(frame);return Ok(RetainedCloneStep::Progress(p));}
  close!(messages);
  if self.next.is_some(){let Some((frame,p))=semio_framework_value::factory::shared::FactorySharedRetirement::<P>::admit_original(&mut self.next,self.snapshots.as_ref().unwrap(),grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.active=Some(frame);return Ok(RetainedCloneStep::Progress(p));}
  if let Some(factory)=self.snapshots.take(){self.factory_close[0]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(unit());}
  if let Some(factory)=self.mutations.take(){self.factory_close[1]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(unit());}
  if let Some(index)=self.factory_close.iter().position(Option::is_some){let owner=self.factory_close[index].as_mut().unwrap();let step=owner.step(grant)?;if owner.terminal_is_empty(){self.factory_close[index]=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.projection.terminal_is_empty()&&self.edit.terminal_is_empty()&&self.retirement.is_empty()&&self.active.is_none()&&self.base.is_none()&&self.source.is_none()&&self.source_authority.is_none()&&self.input.is_none()&&self.post.is_none()&&self.post_read.terminal_is_empty()&&self.inverse.is_none()&&self.rows.is_none()&&self.pages.is_none()&&self.pending.is_none()&&self.messages.is_none()&&self.next.is_none()&&self.snapshots.is_none()&&self.mutations.is_none()&&self.factory_close.iter().all(Option::is_none)}
}
impl<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>> Drop for NativeSemanticPreparation<P,M,C,E>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native semantic producer abandoned original owners");}}

impl<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>> super::ArtifactReplayPreparation<P,M> for NativeSemanticPreparation<P,M,C,E>{
 fn advance_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{self.demands(b)}
 fn advance(&mut self,body:usize,g:semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<ArtifactReplayPreparationStep,ValueError>{NativeSemanticPreparation::advance(self,body,g.retained_grant(),control)}
 fn take_prepared(&mut self)->Option<ArtifactReplayPrepared<P,M>>{self.take()}
 fn cancel(&mut self){NativeSemanticPreparation::begin_close(self);}
 fn begin_close(&mut self){NativeSemanticPreparation::begin_close(self);}
 fn close_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{self.close_demand(b)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{Ok(self.close_demand(b)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.depth)}
 fn close_step(&mut self,g:RetainedCloneGrant,receiving:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<RetainedCloneStep,ValueError>{NativeSemanticPreparation::close_step(self,g,receiving)}
 fn terminal_is_empty(&self)->bool{NativeSemanticPreparation::terminal_is_empty(self)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
