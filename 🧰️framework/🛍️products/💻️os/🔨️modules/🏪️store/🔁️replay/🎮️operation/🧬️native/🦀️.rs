//! 🧬️ Metadata-free semantic preparation uses original snapshot and operation leases.
use super::{ArtifactReplayPreparationContext,ArtifactReplayPrepared,ArtifactReplayPreparationStep};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement};
use semio_framework_value_snapshot_read::{CapturedSnapshotSource,SnapshotRead};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned, retained_clone::{RetainedCloneRef, RetainedOwnedProjection, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedCloneClose, retained_clone_grant_admits, admit_retained_clone_progress}};
use semio_framework_value_original_vector::OriginalVectorCursor;
use semio_framework_value_persistent::list::PagedList;
use std::{mem::{size_of,ManuallyDrop},sync::Arc};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
#[path="🔌️drivers/🦀️.rs"]
mod drivers;
#[path="👁️post/🦀️.rs"]
pub mod post;
pub use post::{PostCustody,PostWant};
pub use semio_os_store_snapshot_clone::ReplyStash;
use semio_framework_value::factory::ArcRootRetirement;
#[path="🔗️input/🦀️.rs"]
mod input;
use input::HoldCustody;
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
 fn birth_work_demand(&self)->Result<u64,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn birth_demand(&self)->RetirementDemand;
 fn begin(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>;
 fn demands(&self,source:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 /// ⛽️ Quotes selected original work when the projection has a native work contract.
 fn advance_work_demand(&self,_source:RetainedCloneRef<'_,P>,_body:usize,_maximum_work_units:u64)->Result<Option<u64>,ValueError>{Ok(None)}
 fn advance(&mut self,source:RetainedCloneRef<'_,P>,body:usize,effect:Effect<'_,'_>)->Result<RetainedCloneStep,Failure>;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,effect:Effect<'_,'_>)->Result<Option<(P,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 /// ⛽️ Quotes selected cleanup work before the original owner moves.
 fn close_work_demand(&self,_body:usize)->Result<Option<u64>,ValueError>{Ok(None)}
 fn close_step(&mut self,effect:Effect<'_,'_>)->semio_framework_value::OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
}
/// ✏️ Delegates all semantic stages to the same native edit issuer used by publication.
pub trait SemanticEdit<P,M>:Send+'static{
 fn birth_work_demand(&self)->u64;
 fn birth_demand(&self)->RetirementDemand;
 fn begin(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>;
 fn post_mode(&self)->PagedOneItemPostMode;
 fn demands(&self,before:RetainedCloneRef<'_,P>,input:RetainedCloneRef<'_,M>,post:PagedOneItemPostRead<'_,P>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,before:RetainedCloneRef<'_,P>,input:RetainedCloneRef<'_,M>,post:PagedOneItemPost<'_,P>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<SemanticEditStep<M>,ValueError>;
 /// ⛽️ Reports the work of the latest `advance`, which funds its own original actions.
 fn last_turn_work(&self)->u64;
 fn begin_close(&mut self);
 fn close_demand(&self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,effect:Effect<'_,'_>)->semio_framework_value::OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
}
/// 📤️ Preserves original inverse and diagnostic owners before independent output admission.
pub enum SemanticEditStep<M>{CompleteSingle(RetainedCloneProgress,M),Progress(RetainedCloneProgress),Complete(RetainedCloneProgress,Vec<M>,Vec<semio_os_spr::MutationMessage>,bool),Refused(RetainedCloneProgress,Vec<semio_os_spr::MutationMessage>)}
std::thread_local!{pub(crate) static DEBUG_STAGE:std::cell::Cell<u8>=const{std::cell::Cell::new(0)};}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Stage{RefusalRows,RefusalCode,RefusalTextBirth,RefusalText,SourceCapture,Source,ProjectionBirth,Projection,TakeProjection,CloseProjection,EditBirth,PostCapture,Edit,CloseEdit,InverseBirth,Inverse,CloseInverse,Share,Ready}
/// 🧳️ One retained producer owns every original lease and candidate through cancellation.
pub struct NativeSemanticPreparation<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>>{
 base:ManuallyDrop<Option<SnapshotRead<P>>>,source:HoldCustody<P,CapturedSnapshotSource<P>>,source_authority:ManuallyDrop<Option<CapturedSnapshotSource<P>>>,input:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
 projection:C,edit:E,post:ManuallyDrop<Option<P>>,custody:PostCustody<P>,inverse:ManuallyDrop<Option<Vec<M>>>,rows:ManuallyDrop<Option<OriginalVectorCursor<M>>>,pages:ManuallyDrop<Option<PagedList<M,{usize::MAX}>>>,pending:ManuallyDrop<Option<M>>,messages:ManuallyDrop<Option<Vec<semio_os_spr::MutationMessage>>>,next:ManuallyDrop<Option<Arc<P>>>,
 snapshots:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,mutations:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,factory_close:[Option<semio_framework_value::FactoryAuthority>;2],active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,retirement:RetainedCloneClose,stage:Stage,foreign:bool,refused:bool,closing:bool,refusal:Option<&'static str>,last_work:u64,original_control:Option<(semio_framework_value::retained_clone::NativeAllocationIdentity,Option<u64>)>,
}
pub(crate) type Effect<'a,'b>=&'a mut semio_framework_value::RetainedAllocationEffect<'b>;
pub(crate) type Failure=semio_framework_value::OriginalRetirementFailure;
pub(crate) type Exit=semio_framework_value::RetirementPollExit;
/// 🪜️ The smallest work ceiling the probe opens; a loan below it plus the invocation handoff opens no effect.
pub(crate) const PROBE_CEILING_FLOOR:u64=8;
/// 🪙️ Opens one original work effect at the largest ceiling the job admits, backing off by quarters.
pub(crate) fn probe_effect<R>(control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,Effect<'_,'_>)->R)->Result<Option<R>,ValueError>{
 let mut run=Some(run);let mut maximum:u64=1<<20;
 loop{
  let attempt=control.with_effect(grant,maximum,|control,effect|(run.take().expect("probe closure runs once"))(control,effect))?;
  if attempt.is_some(){return Ok(attempt)}
  if maximum<=PROBE_CEILING_FLOOR{return Ok(None)}maximum=(maximum/4).max(PROBE_CEILING_FLOOR);
 }
}
/// 🧷️ Runs one reply-shaped child that must record its answer inside the caller's native action; a suspended child violates the native owner contract.
pub(crate) fn complete<S,E>(call:impl for<'r> FnOnce(semio_framework_value::retained_clone::OriginalEffectReply<'r,S,E>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,S,E>,ValueError>)->Result<Result<S,E>,ValueError>{let mut slot=None;drop(call(semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut slot)?)?);slot.ok_or_else(||invalid("semantic child suspended inside a native action"))}
pub(crate) fn one(effect:Effect<'_,'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"semantic preparation action exceeds its work ceiling"))}}
pub(crate) fn pay(effect:Effect<'_,'_>,work:u128)->Result<(),ValueError>{if work==0{return Ok(())}let work=u64::try_from(work).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"semantic preparation work exceeds its ceiling"))?;if effect.performed(work)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"semantic preparation work exceeds its ceiling"))}}
pub(crate) fn paid<T,E>(effect:Effect<'_,'_>,turn:semio_framework_value::RetainedWorkOutcome<T,E>)->Result<T,Failure> where Failure:From<E>{pay(effect,turn.work)?;turn.result.map_err(Failure::from)}
pub(crate) fn lifted(poll:semio_framework_value::RetainedWorkPoll<RetainedCloneStep,ValueError>)->semio_framework_value::OriginalRetirementPoll{match poll{semio_framework_value::RetainedWorkPoll::Pending{work}=>semio_framework_value::RetainedWorkPoll::Pending{work},semio_framework_value::RetainedWorkPoll::Complete(turn)=>semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(Failure::from),work:turn.work})}}
pub(crate) fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneStep,Failure>{match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result}}
fn reserved<M:RetireOwned+Sync>(pages:&mut PagedList<M,{usize::MAX}>,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>{use semio_framework_value_persistent::list::{PagedListReservationOutcome,PagedListReservationFailure};match pages.reserve_one_funded(effect).result{Ok(PagedListReservationOutcome::Ready(progress))=>Ok(progress),Ok(PagedListReservationOutcome::Denied)=>Ok(Default::default()),Err(PagedListReservationFailure::Work(error))=>Err(error),Err(PagedListReservationFailure::Allocation{error,..})=>Err(ValueError::from(error.refusal()))}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||invalid("semantic producer depth overflow"))?;Ok(d)}
fn list_error(error:semio_framework_value_persistent::list::PagedListError)->ValueError{ValueError::literal(match error.kind{semio_framework_value_persistent::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,semio_framework_value_persistent::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value_persistent::list::PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason)}
impl<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>> NativeSemanticPreparation<P,M,C,E>{
 pub fn new(request:ArtifactReplayPreparationRequest<P,M>,projection:C,edit:E)->Self{Self{base:ManuallyDrop::new(Some(request.base)),source:HoldCustody::vacant(),source_authority:ManuallyDrop::new(None),input:ManuallyDrop::new(Some(request.input)),projection,edit,post:ManuallyDrop::new(None),custody:Default::default(),inverse:ManuallyDrop::new(None),rows:ManuallyDrop::new(None),pages:ManuallyDrop::new(Some(Default::default())),pending:ManuallyDrop::new(None),messages:ManuallyDrop::new(None),next:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(request.snapshots)),mutations:ManuallyDrop::new(Some(request.mutations)),factory_close:[None,None],active:ManuallyDrop::new(None),retirement:Default::default(),stage:Stage::SourceCapture,foreign:false,refused:false,closing:false,refusal:None,last_work:0,original_control:None}}
 pub fn refused(request:ArtifactReplayPreparationRequest<P,M>,projection:C,edit:E,reason:&'static str)->Self{let mut owner=Self::new(request,projection,edit);owner.refusal=Some(reason);owner.refused=true;owner.projection.begin_close();owner.edit.begin_close();owner.stage=Stage::RefusalRows;owner}
 fn refusal_fragment(&self,body:usize)->Result<&'static str,ValueError>{let reason=self.refusal.ok_or_else(||invalid("semantic preflight refusal absent"))?;let start=self.messages.as_ref().and_then(|rows|rows.first()).map_or(0,|row|row.message.len());let mut end=start.saturating_add(body).min(reason.len());while !reason.is_char_boundary(end){end-=1;}Ok(&reason[start..end])}
 pub fn demands(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demand(body);}
  Ok(match self.stage{
   Stage::RefusalRows=>RetirementDemand{copy_bytes:size_of::<Vec<semio_os_spr::MutationMessage>>()+size_of::<semio_os_spr::MutationMessage>(),capacity_bytes:size_of::<semio_os_spr::MutationMessage>(),depth:1,..Default::default()},
   Stage::RefusalCode=>RetirementDemand{copy_bytes:size_of::<String>()+"mutation.invariant".len(),capacity_bytes:"mutation.invariant".len(),depth:1,..Default::default()},
   Stage::RefusalTextBirth=>RetirementDemand{copy_bytes:size_of::<String>(),capacity_bytes:self.refusal.ok_or_else(||invalid("semantic preflight refusal absent"))?.len(),depth:1,..Default::default()},
   Stage::RefusalText=>{let fragment=self.refusal_fragment(body)?;if fragment.is_empty(){header::<Stage>()}else{RetirementDemand{copy_bytes:fragment.len(),depth:1,..Default::default()}}},
   Stage::SourceCapture=>CapturedSnapshotSource::<P>::constructor_demand(),
   Stage::Source=>if self.source.ready(){header::<Stage>()}else{self.source.quote(body)?},
   Stage::ProjectionBirth=>self.projection.birth_demand(),
   Stage::Projection=>self.projection.demands(self.source.original()?,body,maximum_work_units.checked_sub(1).unwrap_or(0))?,
   Stage::TakeProjection=>self.projection.take_demand()?,
   Stage::CloseProjection=>if self.projection.terminal_is_empty(){header::<Stage>()}else{self.projection.close_demand(body)?},
   Stage::EditBirth=>self.edit.birth_demand(),
   Stage::PostCapture=>PostCustody::<P>::capture_quote().0,
   Stage::Edit=>{let mode=self.edit.post_mode();if let Some((demand,_))=self.custody.due(mode,body)?{demand}else{self.edit.demands(self.source.original()?,self.input.as_ref().ok_or_else(||invalid("semantic original operation source absent"))?.borrow()?,self.custody.read(mode)?,body)?}},
   Stage::CloseEdit=>if self.edit.terminal_is_empty(){if !self.custody.terminal_is_empty(){self.custody.closing_quote(body)?.0}else{header::<Stage>()}}else{self.edit.close_demand(self.mutations.as_ref().ok_or_else(||invalid("semantic mutation issuer absent"))?,body)?},
   Stage::InverseBirth=>if self.inverse.is_some(){OriginalVectorCursor::<M>::constructor_demand()}else{header::<Stage>()},
   Stage::Inverse=>{let pages=self.pages.as_ref().ok_or_else(||invalid("semantic inverse pages absent"))?;let rows=self.rows.as_ref();if self.pending.is_some()&&pages.has_reserved_slot(){RetirementDemand{copy_bytes:size_of::<M>(),depth:pages.next_push_depth_demand().map_err(list_error)?,..Default::default()}}else if self.pending.is_some(){RetirementDemand{copy_bytes:pages.next_reserve_copy_byte_demand().map_err(list_error)?,capacity_bytes:pages.next_allocation_bytes().map_err(list_error)?,depth:pages.next_reserve_depth_demand().map_err(list_error)?,..Default::default()}}else if rows.is_none_or(|rows|rows.remaining().is_empty()){header::<Stage>()}else if !pages.has_reserved_slot(){RetirementDemand{copy_bytes:pages.next_reserve_copy_byte_demand().map_err(list_error)?,capacity_bytes:pages.next_allocation_bytes().map_err(list_error)?,depth:pages.next_reserve_depth_demand().map_err(list_error)?,..Default::default()}}else{rows.unwrap().next_take_demand()}},
   Stage::CloseInverse=>if !self.retirement.is_empty(){RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?}}else if self.rows.is_some(){self.retirement.next_owner_demands::<OriginalVectorCursor<M>>(true,&None,body)?}else{header::<Option<OriginalVectorCursor<M>>>()},
   Stage::Share=>if self.refused{header::<Stage>()}else{RetirementDemand{copy_bytes:size_of::<P>()+size_of::<Arc<P>>(),capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<P>(),depth:1,..Default::default()}},
   Stage::Ready=>header::<ArtifactReplayPrepared<P,M>>(),
  })
 }

 pub fn last_turn_work(&self)->u64{self.last_work}
 pub fn advance(&mut self,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<ArtifactReplayPreparationStep,Failure>{
  self.last_work=0;let identity=(receiving.ledger_identity(),receiving.retirement_recipient_identity());if self.original_control.is_some_and(|original|original!=identity){return Err((invalid("native semantic preparation changed its original decoder or recipient")).into())}
  if self.closing||!retained_clone_grant_admits(grant,self.demands(body,u64::MAX)?){return Ok(ArtifactReplayPreparationStep::Blocked)}
  self.original_control=Some(identity);
  if self.stage==Stage::Edit&&self.custody.due(self.edit.post_mode(),body)?.is_none(){return self.advance_edit(body,grant,receiving)}
  let turn=probe_effect(receiving,grant,|control,effect|{let result=self.advance_effect(body,control,effect);(result,effect.performed_work())})?;
  let Some((result,work))=turn else{return Ok(ArtifactReplayPreparationStep::Blocked)};
  self.last_work=u64::try_from(work).unwrap_or(u64::MAX);result
 }
 fn advance_edit(&mut self,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<ArtifactReplayPreparationStep,Failure>{
  let d=self.demands(body,u64::MAX)?;receiving.checkpoint()?;
  let unit=||RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes};let _=unit;
  let mode=self.edit.post_mode();let before=self.source.original()?;let input=self.input.as_ref().unwrap().borrow()?;let result=self.edit.advance(before,input,self.custody.access(mode)?,body,grant,receiving);self.last_work=self.edit.last_turn_work();
  let p=match result?{
   SemanticEditStep::Progress(p)=>p,
   SemanticEditStep::CompleteSingle(p,row)=>{*self.pending=Some(row);*self.messages=Some(Vec::new());self.edit.begin_close();self.stage=Stage::CloseEdit;p},
   SemanticEditStep::Complete(p,inverse,messages,foreign)=>{*self.inverse=Some(inverse);*self.messages=Some(messages);self.foreign=foreign;self.edit.begin_close();self.stage=Stage::CloseEdit;p},
   SemanticEditStep::Refused(p,messages)=>{*self.inverse=Some(Vec::new());*self.messages=Some(messages);self.refused=true;self.edit.begin_close();self.stage=Stage::CloseEdit;p},
  };
  let p=admit_retained_clone_progress(grant,p,"metadata-free original semantic producer")?;Ok(ArtifactReplayPreparationStep::Pending(p))
 }
 fn advance_effect(&mut self,body:usize,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,effect:Effect<'_,'_>)->Result<ArtifactReplayPreparationStep,Failure>{
  DEBUG_STAGE.with(|cell|cell.set(self.stage as u8));
  let grant=effect.grant();let maximum_work_units=effect.maximum_work_units();
  let d=self.demands(body,maximum_work_units)?;if !retained_clone_grant_admits(grant,d){return Ok(ArtifactReplayPreparationStep::Blocked);}
  if self.closing{return Ok(ArtifactReplayPreparationStep::Blocked);}control.checkpoint()?;if self.stage!=Stage::Edit||self.custody.due(self.edit.post_mode(),body)?.is_some(){control.admit_turn_capacity(grant.maximum_capacity_bytes)?;if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?;}}
  let unit=||RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes};
  if matches!(self.stage,Stage::RefusalRows|Stage::RefusalCode|Stage::RefusalTextBirth|Stage::RefusalText|Stage::Share){one(effect)?;}
  let p=match self.stage{
   Stage::RefusalRows=>{*self.messages=Some(Vec::new());let rows=self.messages.as_mut().unwrap();rows.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"semantic refusal row allocation failed"))?;rows.push(semio_os_spr::MutationMessage{level:semio_framework_diagnostic::Severity::Fatal,code:semio_framework_diagnostic::FaultCode(String::new()),message:String::new(),target:Vec::new(),op_index:None});self.stage=Stage::RefusalCode;unit()},
   Stage::RefusalCode=>{let code=&mut self.messages.as_mut().unwrap()[0].code.0;code.try_reserve_exact(d.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"semantic refusal code allocation failed"))?;code.push_str("mutation.invariant");self.stage=Stage::RefusalTextBirth;unit()},
   Stage::RefusalTextBirth=>{let text=&mut self.messages.as_mut().unwrap()[0].message;text.try_reserve_exact(d.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"semantic refusal text allocation failed"))?;self.stage=Stage::RefusalText;unit()},
   Stage::RefusalText=>{let fragment=self.refusal_fragment(body)?;if fragment.is_empty()&&self.messages.as_ref().unwrap()[0].message.len()!=self.refusal.unwrap().len(){return Ok(ArtifactReplayPreparationStep::Blocked);}self.messages.as_mut().unwrap()[0].message.push_str(fragment);if self.messages.as_ref().unwrap()[0].message.len()==self.refusal.unwrap().len(){self.stage=Stage::CloseEdit;}unit()},
   Stage::SourceCapture=>{let Some((source,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref().ok_or_else(||semio_framework_value::ValueError::invariant("replay snapshot issuer is absent"))?,effect).result?else{return Ok(ArtifactReplayPreparationStep::Blocked);};self.source.fill(source);one(effect)?;self.stage=Stage::Source;p},
   Stage::Source=>if self.source.ready(){one(effect)?;self.stage=Stage::ProjectionBirth;unit()}else{let body=self.source.pending_body().unwrap_or(body);self.source.drive(body,d,effect)?.progress()},
   Stage::ProjectionBirth=>{let p=self.projection.begin(effect)?;one(effect)?;self.stage=Stage::Projection;p},
   Stage::Projection=>{let step=self.projection.advance(self.source.original()?,body,effect)?;if matches!(step,RetainedCloneStep::Complete(_)){one(effect)?;self.stage=Stage::TakeProjection;}step.progress()},
   Stage::TakeProjection=>{let Some((post,p))=self.projection.take(effect)?else{return Err((invalid("funded semantic projection omitted owner")).into());};*self.post=Some(post);one(effect)?;self.projection.begin_close();self.stage=Stage::CloseProjection;p},
   Stage::CloseProjection=>if self.projection.terminal_is_empty(){one(effect)?;self.stage=Stage::EditBirth;unit()}else{settled(self.projection.close_step(effect))?.progress()},
   Stage::EditBirth=>{let p=self.edit.begin(effect)?;one(effect)?;self.stage=Stage::PostCapture;p},
   Stage::PostCapture=>{let p=self.custody.run(PostWant::Capture,body,&mut self.post,effect)?.unwrap_or_default();if self.custody.captured(){self.stage=Stage::Edit;}p},
   Stage::Edit=>{let mode=self.edit.post_mode();let p=self.custody.run(PostWant::Edit(mode),body,&mut self.post,effect)?.unwrap_or_default();return Ok(ArtifactReplayPreparationStep::Pending(p));},
   Stage::CloseEdit=>if self.edit.terminal_is_empty(){if !self.custody.terminal_is_empty(){self.custody.run(PostWant::Close,body,&mut self.post,effect)?.unwrap_or_default()}else{one(effect)?;self.stage=Stage::InverseBirth;unit()}}else{settled(self.edit.close_step(self.mutations.as_ref().unwrap(),effect))?.progress()},
   Stage::InverseBirth=>{if self.inverse.is_none(){one(effect)?;self.stage=Stage::Inverse;return Ok(ArtifactReplayPreparationStep::Pending(unit()));}let mut rows=None;let step=complete(|reply|OriginalVectorCursor::admit_into(&mut self.inverse,&mut rows,effect,reply))??;if let Some(rows)=rows{*self.rows=Some(rows);self.stage=Stage::Inverse;}step.progress()},
   Stage::Inverse=>{one(effect)?;let pages=self.pages.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("replay inverse pages are absent"))?;if self.pending.is_some()&&!pages.has_reserved_slot(){reserved(pages,effect)?}else if self.pending.is_some(){let original=self.pending.take().ok_or_else(||semio_framework_value::ValueError::invariant("replay pending inverse is absent"))?;if let Err(original)=pages.push_reserved(original){*self.pending=Some(original);return Err((invalid("semantic reserved inverse slot absent")).into());}unit()}else if self.rows.as_ref().is_none_or(|rows|rows.remaining().is_empty()){self.stage=Stage::CloseInverse;unit()}else if !pages.has_reserved_slot(){reserved(pages,effect)?}else{let rows=self.rows.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("replay inverse rows are absent"))?;let step=complete(|reply|rows.take_front_into(&mut self.pending,effect,reply))??;if self.pending.is_none(){return Err((invalid("funded semantic inverse transfer omitted row")).into())}step.progress()}},
   Stage::CloseInverse=>if !self.retirement.is_empty(){settled(self.retirement.step_granted(body,effect))?.progress()}else if self.rows.is_some(){match self.retirement.begin_granted(&mut self.rows,effect).result?{Some(step)=>step.progress(),None=>Default::default()}}else{one(effect)?;self.stage=Stage::Share;unit()},
   Stage::Share=>{if !self.refused{*self.next=Some(Arc::new(self.post.take().ok_or_else(||invalid("semantic candidate absent"))?));}self.stage=Stage::Ready;unit()},
   Stage::Ready=>return Ok(ArtifactReplayPreparationStep::Prepared(Default::default())),
  };
  let p=admit_retained_clone_progress(grant,p,"metadata-free original semantic producer")?;
  Ok(if self.stage==Stage::Ready{ArtifactReplayPreparationStep::Prepared(p)}else{ArtifactReplayPreparationStep::Pending(p)})
 }
 pub fn take(&mut self)->Option<ArtifactReplayPrepared<P,M>>{if self.closing||self.stage!=Stage::Ready{return None;}self.closing=true;Some(ArtifactReplayPrepared{next:self.next.take(),inverse:Ok(self.pages.take()?),messages:self.messages.take().unwrap_or_default(),apply_refusal:None,input_refusal:None,foreign_steps:self.foreign})}
 pub fn begin_close(&mut self){self.closing=true;self.projection.begin_close();self.edit.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.projection.terminal_is_empty(){return self.projection.close_demand(body);}
  if !self.edit.terminal_is_empty(){return self.edit.close_demand(self.mutations.as_ref().ok_or_else(||invalid("semantic original mutation issuer absent"))?,body);}
  if !self.custody.terminal_is_empty(){return Ok(self.custody.closing_quote(body)?.0);}
  if let Some(active)=self.active.as_ref(){return semio_framework_value::factory_ticket_demands(active,body);}
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  if self.rows.is_some(){return self.retirement.next_owner_demands::<OriginalVectorCursor<M>>(true,&None,body);}
  if !self.source.terminal_is_empty(){return self.source.close_demand(body);}
  if let Some(input)=self.input.as_ref(){return if input.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{input.close_demand(body)};}
  macro_rules! pending{($field:ident,$kind:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$kind>(true,&None,body);}};}
  if self.base.is_some(){return Ok(CapturedSnapshotSource::<P>::constructor_demand());}
  pending!(source_authority,CapturedSnapshotSource<P>);
  if self.post.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()});}
  pending!(inverse,Vec<M>);pending!(pages,PagedList<M,{usize::MAX}>);
  if self.pending.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<M>::constructor_capacity_bytes(),depth:1,..Default::default()});}
  pending!(messages,Vec<semio_os_spr::MutationMessage>);
  if self.next.is_some(){return Ok(ArcRootRetirement::<P>::original_admission_demands(&self.next));}
  if self.snapshots.is_some()||self.mutations.is_some(){return Ok(header::<Arc<dyn semio_framework_value::FactoryRetirement>>());}
  if let Some(factory)=self.factory_close.iter().find_map(Option::as_ref){return if factory.terminal_is_empty(){Ok(header::<Option<semio_framework_value::FactoryAuthority>>())}else{factory.demands(body)};}
  Ok(Default::default())
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.closing||self.terminal_is_empty(){return Ok(0)}
  if !self.projection.terminal_is_empty(){return self.projection.close_work_demand(body)?.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"semantic projection has no original close work quote"))}
  if !self.edit.terminal_is_empty(){return self.edit.close_work_demand(self.mutations.as_ref().ok_or_else(||invalid("semantic mutation issuer absent"))?,body)}
  if !self.custody.terminal_is_empty(){return Ok(self.custody.closing_quote(body)?.1)}
  if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<P>(false,&None,body)}
  if self.rows.is_some(){return self.retirement.next_owner_work::<OriginalVectorCursor<M>>(true,&None,body)}
  if !self.source.terminal_is_empty(){return self.source.close_work_demand(body)}
  if let Some(input)=self.input.as_ref(){return if input.terminal_is_empty(){Ok(1)}else{input.close_work_demand(body)}}
  if self.base.is_some(){return CapturedSnapshotSource::<P>::constructor_work_demand(&self.base).checked_add(1).ok_or_else(||invalid("semantic source capture work overflow"))}
  macro_rules! pending{($field:ident,$kind:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_work::<$kind>(true,&None,body)}};}
  pending!(source_authority,CapturedSnapshotSource<P>);
  if self.post.is_some(){return Ok(semio_framework_value::FactoryOwnedRetirement::<P>::admit_original_source_work_demand(&self.post))}
  pending!(inverse,Vec<M>);pending!(pages,PagedList<M,{usize::MAX}>);
  if self.pending.is_some(){return Ok(semio_framework_value::FactoryOwnedRetirement::<M>::admit_original_source_work_demand(&self.pending))}
  pending!(messages,Vec<semio_os_spr::MutationMessage>);
  if self.next.is_some(){return Ok(ArcRootRetirement::<P>::admit_original_source_work_demand(&self.next))}
  if self.snapshots.is_some()||self.mutations.is_some(){return Ok(1)}
  self.factory_close.iter().find_map(Option::as_ref).map_or(Ok(0),|factory|if factory.terminal_is_empty(){Ok(1)}else{factory.next_computational_work(body)})
 }
 fn delegates_original_close(&self)->bool{
  if !self.projection.terminal_is_empty()||!self.edit.terminal_is_empty()||!self.custody.terminal_is_empty(){return false}
  if self.active.is_some()||!self.retirement.is_empty()||self.source.delegates(){return true}
  if !self.source.terminal_is_empty()||self.input.is_some()||self.base.is_some()||self.source_authority.is_some()||self.rows.is_some()||self.post.is_some()||self.inverse.is_some()||self.pages.is_some()||self.pending.is_some()||self.messages.is_some()||self.next.is_some()||self.snapshots.is_some()||self.mutations.is_some(){return false}
  self.factory_close.iter().any(Option::is_some)
 }

 pub fn close_step(&mut self,effect:Effect<'_,'_>,receiving:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::OriginalRetirementPoll{
  let result=(||->Result<RetainedCloneStep,Exit>{
   let grant=effect.grant();let identity=(receiving.ledger_identity(),receiving.retirement_recipient_identity());if self.original_control.is_some_and(|original|original!=identity){return Err(invalid("native semantic retirement changed its original decoder or recipient").into())}
   if self.closing&&!self.terminal_is_empty()&&receiving.failure_storage_pool().is_empty(){return Exit::resume(semio_framework_value::refill_original_failure_storage(receiving,grant))}
   if !self.closing||self.terminal_is_empty()||!retained_clone_grant_admits(grant,self.close_demand(grant.maximum_copy_bytes)?){return self.close_original(effect,receiving)}
   if !self.delegates_original_close(){let demand=self.close_demand(grant.maximum_copy_bytes)?;receiving.charge(demand.capacity_bytes)?;}
   self.close_original(effect,receiving)
  })();
  Exit::settle(effect,result)
 }
 fn close_original(&mut self,effect:Effect<'_,'_>,receiving:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<RetainedCloneStep,Exit>{
  let grant=effect.grant();
  if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  let progress=|step:RetainedCloneStep|RetainedCloneStep::Progress(step.progress());
  if !self.projection.terminal_is_empty(){return Exit::resume(self.projection.close_step(effect)).map(progress);}
  if !self.edit.terminal_is_empty(){return Exit::resume(self.edit.close_step(self.mutations.as_ref().ok_or_else(||invalid("semantic mutation issuer absent"))?,effect)).map(progress);}
  if !self.custody.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(self.custody.run(PostWant::Close,grant.maximum_copy_bytes,&mut self.post,effect)?.unwrap_or_default()));}
  if self.active.is_some(){return Exit::resume(semio_framework_value::close_factory_ticket_controlled(&mut self.active,grant.maximum_copy_bytes,effect,receiving));}
  if !self.retirement.is_empty(){let selected=self.retirement.select(grant.maximum_copy_bytes)?;return Exit::resume(selected.step_controlled(effect,receiving)).map(progress);}
  if self.rows.is_some(){return Ok(RetainedCloneStep::Progress(match self.retirement.begin_granted(&mut self.rows,effect).result?{Some(step)=>step.progress(),None=>Default::default()}));}
  if !self.source.terminal_is_empty(){return self.source.close_step_controlled(grant.maximum_copy_bytes,effect,receiving);}
  if let Some(input)=self.input.as_mut(){if input.terminal_is_empty(){self.input.take();one(effect)?;return Ok(unit());}return Exit::resume(input.close_step(grant.maximum_copy_bytes,effect)).map(progress);}
  macro_rules! close{($field:ident)=>{if let Some(step)=self.retirement.begin_granted(&mut self.$field,effect).result?{return Ok(RetainedCloneStep::Progress(step.progress()));}};}
  if self.base.is_some(){let Some((authority,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref().ok_or_else(||semio_framework_value::ValueError::invariant("replay snapshot issuer is absent"))?,effect).result?else{return Ok(RetainedCloneStep::Progress(Default::default()));};*self.source_authority=Some(authority);one(effect)?;return Ok(RetainedCloneStep::Progress(p));}
  close!(source_authority);
  if self.post.is_some(){return Exit::resume(lifted(semio_framework_value::FactoryOwnedRetirement::<P>::admit_original(&mut self.post,&mut self.snapshots,&mut self.active,effect))).map(progress);}
  close!(inverse);close!(pages);
  if self.pending.is_some(){return Exit::resume(lifted(semio_framework_value::FactoryOwnedRetirement::<M>::admit_original(&mut self.pending,&mut self.mutations,&mut self.active,effect))).map(progress);}
  close!(messages);
  if self.next.is_some(){return Exit::resume(ArcRootRetirement::<P>::admit_original(&mut self.next,&mut self.snapshots,&mut self.active,effect)).map(progress);}
  if self.snapshots.is_some(){let factory=self.snapshots.take().expect("quoted original factory alias");self.factory_close[0]=Some(semio_framework_value::FactoryAuthority::new(factory));one(effect)?;return Ok(unit())}
  if self.mutations.is_some(){let factory=self.mutations.take().expect("quoted original factory alias");self.factory_close[1]=Some(semio_framework_value::FactoryAuthority::new(factory));one(effect)?;return Ok(unit())}
  if let Some(index)=self.factory_close.iter().position(Option::is_some){
   if self.factory_close[index].as_ref().unwrap().terminal_is_empty(){self.factory_close[index]=None;one(effect)?;return Ok(unit())}
   let owner=self.factory_close[index].as_mut().unwrap();let poll=if owner.requires_original_control(){owner.step_controlled(grant.maximum_copy_bytes,effect,receiving)}else{owner.step(grant.maximum_copy_bytes,effect)};return Exit::resume(poll).map(progress);
  }
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.projection.terminal_is_empty()&&self.edit.terminal_is_empty()&&self.retirement.is_empty()&&self.active.is_none()&&self.base.is_none()&&self.source.terminal_is_empty()&&self.source_authority.is_none()&&self.input.is_none()&&self.post.is_none()&&self.custody.terminal_is_empty()&&self.inverse.is_none()&&self.rows.is_none()&&self.pages.is_none()&&self.pending.is_none()&&self.messages.is_none()&&self.next.is_none()&&self.snapshots.is_none()&&self.mutations.is_none()&&self.factory_close.iter().all(Option::is_none)}
}
impl<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>> Drop for NativeSemanticPreparation<P,M,C,E>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native semantic producer abandoned original owners");}}

impl<P:RetireOwned+Sync,M:RetireOwned+Sync,C:SemanticProjection<P>,E:SemanticEdit<P,M>> super::ArtifactReplayPreparation<P,M> for NativeSemanticPreparation<P,M,C,E>{
 fn advance_demands(&self,b:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.demands(b,maximum_work_units)}
 fn last_turn_work(&self)->u64{self.last_work}
 fn advance(&mut self,body:usize,g:semio_os_store_one_item::ArtifactStoreOneItemGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<ArtifactReplayPreparationStep,semio_framework_value::OriginalRetirementFailure>{NativeSemanticPreparation::advance(self,body,g.retained_grant(),control)}
 fn take_prepared(&mut self)->Option<ArtifactReplayPrepared<P,M>>{self.take()}
 fn cancel(&mut self){NativeSemanticPreparation::begin_close(self);}
 fn begin_close(&mut self){NativeSemanticPreparation::begin_close(self);}
 fn close_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{self.close_demand(b)}
 fn close_work_demand(&self,b:usize)->Result<u64,ValueError>{NativeSemanticPreparation::close_work_demand(self,b)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{Ok(self.close_demand(b)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.depth)}
 fn close_step(&mut self,effect:Effect<'_,'_>,receiving:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::OriginalRetirementPoll{NativeSemanticPreparation::close_step(self,effect,receiving)}
 fn terminal_is_empty(&self)->bool{NativeSemanticPreparation::terminal_is_empty(self)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
