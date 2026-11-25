//! 🏗️ The native record receiving machine is compiled once per owner shape instead of once per derived record; a record contributes only its field descriptor and small typed field arms.
use super::*;
use crate::native_decoding::{NativeReceivingFailure,NativeSeedTemplate};
use crate::retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,retained_clone_grant_admits};
use crate::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,ValueRefusalKind};
use super::tree::{fixed,invalid,receipt,child_grant,NativeValueSource,NativeFieldPlanSlot,NativeScalarValue,plan_field_stage,plan_leaf_stage};
use super::{NativeDefault,NativeDefaultSlot,SharedFrame,NativeReceivingChild};
use std::mem::{size_of,ManuallyDrop,MaybeUninit};

type Projection=RetainedOwnedProjection<dyn NativeValueSource>;
pub type Source=ManuallyDrop<Option<Projection>>;
pub type Frame=NativeReceivingChild<OriginalReceivingPrimary>;
pub type Step=RetainedCloneStep;
pub type Close=RetainedWorkPoll<Step,OriginalRetirementFailure>;
type Default_<T>=NativeDefaultSlot<T,SharedFrame>;

/// 📜️ The static shape of one record: its arity, key table and per-field flags.
pub struct RecordSpec{pub count:usize,pub tuple:bool,pub flatten:&'static [bool],pub required:&'static [bool],pub leaves:&'static [NativeSchemaLeaf],pub any_flatten:bool}

/// 🏷️ The static flags of one field of a record.
#[derive(Clone,Copy)]
pub struct FieldShape{pub leaf:bool,pub flat:bool,pub text:bool,pub pure_default:bool}

/// 🪙️ A vacant part is written as its empty discriminant and defaults, so one seed part quotes at most this many bytes however large the empty owner is.
pub const SEED_WRITE_BYTES:usize=8;

/// 🧩️ The resident state of one record receiver that does not depend on its field types.
pub struct NativeRecordCore<I:NativeValueSource+?Sized,N,const C:usize>{
 pub binding:Option<RetainedCloneBinding>,pub positions:[Option<usize>;C],pub initialized:[bool;C],pub partial:NativeRecordStorage<N>,pub output:ManuallyDrop<Option<N>>,pub source:Source,
 pub matcher:Option<NativeRecordMatcher<I>>,pub projection:RetainedOwnedProjectionAdmission<dyn NativeValueSource>,pub retirement:RetainedCloneClose,pub total:Option<usize>,
 pub scan:usize,pub field:usize,pub close_field:usize,pub phase:u8,pub stage:u8,pub closing:bool,pub subset:bool}

/// 🔭️ The slot operations that do not depend on a field type, so they run through one erased view.
pub trait RecordSlotView{
 fn view_empty(&self)->bool;
 fn view_birth(&self)->bool;
 fn view_control(&self)->bool;
 fn view_pending(&self)->bool;
 fn view_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll;
}

/// 🧱️ The slot operations of one field type shared by its present slot and its phased default.
pub trait RecordSlot<T>:RecordSlotView{
 fn slot_work(&self,source:Option<RetainedCloneRef<'_,dyn NativeValueSource>>,body:usize)->Result<u64,ValueError>;
 fn slot_nested(&mut self,source:Option<RetainedCloneRef<'_,dyn NativeValueSource>>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>;
 fn slot_retire(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close;
 fn slot_begin(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Close;
 fn slot_close_plan(&self,body:usize)->Option<Result<(RetirementDemand,u64),ValueError>>;
 fn slot_close_step(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Option<Close>;
 fn slot_take(&mut self,pending:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<Step,OriginalReceivingPrimary>;
 fn slot_reset(&mut self,frame:*mut Frame);
}

/// 🚫️ The absent slot of a field that has no present slot or no default.
pub struct NoSlot;
impl RecordSlotView for NoSlot{fn view_empty(&self)->bool{true}fn view_birth(&self)->bool{false}fn view_control(&self)->bool{false}fn view_pending(&self)->bool{false}fn view_birth_step(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:RetainedCloneGrant)->OriginalRetirementPoll{unreachable!()}}
impl<T> RecordSlot<T> for NoSlot{
 fn slot_work(&self,_:Option<RetainedCloneRef<'_,dyn NativeValueSource>>,_:usize)->Result<u64,ValueError>{Err(invalid())}
 fn slot_nested(&mut self,_:Option<RetainedCloneRef<'_,dyn NativeValueSource>>,_:&mut NativeDecodeControl<'_>,_:OriginalSourceTurnWitness<'_>,_:usize,_:u64,_:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn slot_retire(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>)->Close{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn slot_begin(&mut self,_:&mut RetainedAllocationEffect<'_>)->Close{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn slot_close_plan(&self,_:usize)->Option<Result<(RetirementDemand,u64),ValueError>>{None}
 fn slot_close_step(&mut self,_:Option<&mut OriginalRetirementCloseLoan<'_>>,_:usize,_:&mut RetainedAllocationEffect<'_>)->Option<Close>{None}
 fn slot_take(&mut self,_:&mut Option<T>,_:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<Step,OriginalReceivingPrimary>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn slot_reset(&mut self,_:*mut Frame){}
}

/// 🔖️ Short names of the arm signatures a derived record spells out.
pub type Effect<'a>=RetainedAllocationEffect<'a>;
pub type Control<'a>=NativeDecodeControl<'a>;
pub type Witness<'a>=OriginalSourceTurnWitness<'a>;
pub type Grant=RetainedCloneGrant;
pub type Loan<'a>=OriginalRetirementCloseLoan<'a>;
pub type Demand=RetirementDemand;
pub type PlanOut=Result<(RetirementDemand,u64,u8),ValueError>;
pub type ClosePlanOut=Result<(RetirementDemand,u64),ValueError>;
pub type WorkOut=Result<u64,ValueError>;
pub type ApplyOut=Result<Option<Option<Step>>,OriginalReceivingPrimary>;
pub type LeafOut=Result<(),OriginalReceivingPrimary>;
pub type NestedOut=RetainedWorkPoll<Step,NativeReceivingFailure>;
pub type CloseOut=Close;
pub type FrameMark=*mut Frame;

pub fn progress(step:Step)->Step{Step::Progress(step.progress())}
pub fn close_plan_of(empty:bool,closing:bool,demands:impl FnOnce()->Result<(RetirementDemand,u64),ValueError>,begin:impl FnOnce()->Result<(RetirementDemand,u64),ValueError>)->Option<Result<(RetirementDemand,u64),ValueError>>{if empty{None}else if closing{Some(demands())}else{Some(begin())}}

impl<T:NativeDefault> RecordSlotView for Default_<T>{
 fn view_empty(&self)->bool{self.terminal_is_empty()}
 fn view_birth(&self)->bool{self.requires_original_birth()}
 fn view_control(&self)->bool{self.requires_original_control()}
 fn view_pending(&self)->bool{self.advance_pending()}
 fn view_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.close_original_birth_step(original,grant)}
}
impl<T:NativeDefault> RecordSlot<T> for Default_<T>{
 fn slot_work(&self,_:Option<RetainedCloneRef<'_,dyn NativeValueSource>>,body:usize)->Result<u64,ValueError>{self.advance_work_demand(body)}
 fn slot_nested(&mut self,_:Option<RetainedCloneRef<'_,dyn NativeValueSource>>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>{self.advance(control,witness,body,maximum,grant).map(|s|Step::Progress(s.progress()))}
 fn slot_retire(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{native_close_settle(effect,|effect,reply|self.close_step(body,effect,reply)).map(progress)}
 fn slot_begin(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Close{native_close_settle(effect,|effect,reply|self.begin_close_into(effect,reply)).map(progress)}
 fn slot_close_plan(&self,body:usize)->Option<Result<(RetirementDemand,u64),ValueError>>{close_plan_of(self.terminal_is_empty(),self.is_closing(),||Ok((self.close_demands(body)?,self.close_work_demand(body)?)),||Ok((self.begin_close_demand()?,self.begin_close_work_demand()?)))}
 fn slot_close_step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Option<Close>{
  if self.terminal_is_empty(){return None}
  Some(if self.is_closing(){native_close_settle(effect,|effect,reply|match original.as_deref_mut(){Some(loan)=>self.close_original_step(loan,body,effect,reply),None=>self.close_step(body,effect,reply)})}else{native_close_settle(effect,|effect,reply|self.begin_close_into(effect,reply))}.map(progress))}
 fn slot_take(&mut self,pending:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<Step,OriginalReceivingPrimary>{self.take(pending,effect)}
 fn slot_reset(&mut self,_:*mut Frame){}
}

/// 🔤️ Selects the exact page of one text leaf the next funded step copies: whole short text at once, otherwise a char-aligned page.
fn text_page(text:&str,copied:usize,body:usize,maximum:u64)->Result<usize,ValueError>{
 let remaining=text.len().checked_sub(copied).ok_or_else(invalid)?;
 let page=body.saturating_sub(size_of::<Option<String>>()).min(usize::try_from(maximum.saturating_sub(1)).unwrap_or(usize::MAX)).min(semio_framework_value_control_contract::RETAINED_CLONE_BULK_PAGE_BYTES);
 let mut end=copied+remaining.min(page.max(usize::from(remaining!=0)));
 while end>copied&&!text.is_char_boundary(end){end-=1}
 if end==copied&&remaining>0{end=copied+text[copied..].chars().next().map_or(0,char::len_utf8)}
 Ok(end-copied)
}

/// 🔤️ Quotes one step of a text leaf: the page copy and, on the first step, the exact byte buffer reservation.
fn plan_text_stage<I:NativeValueSource+?Sized>(spec:&RecordSpec,input:RetainedCloneRef<'_,I>,index:usize,body:usize,maximum:u64,copied:Option<usize>)->Result<(RetirementDemand,u64,u8),ValueError>{
 let child=input.try_project::<dyn NativeValueSource,ValueError,_>(index,|original|if spec.tuple{original.sequence_child(index)}else{original.record_child(index)})?;
 let text=child.get().text()?;let count=text_page(text,copied.unwrap_or(0),body,maximum)?;
 let copy_bytes=count.checked_add(size_of::<Option<String>>()).ok_or_else(invalid)?;
 Ok((RetirementDemand{copy_bytes,capacity_bytes:if copied.is_none(){text.len()}else{0},depth:1,..Default::default()},u64::try_from(count).map_err(|_|invalid())?.checked_add(1).ok_or_else(invalid)?,27))
}

/// 🔤️ Copies one quoted page of a text leaf into its pending string; the first page reserves the whole buffer once under the allocation marker.
pub fn field_text<I:NativeValueSource+?Sized,N,const C:usize>(c:&mut NativeRecordCore<I,N,C>,spec:&RecordSpec,i:usize,input:RetainedCloneRef<'_,I>,pending:&mut ManuallyDrop<Option<String>>,effect:&mut RetainedAllocationEffect<'_>,demand:RetirementDemand,work:&mut u128)->Result<(),OriginalReceivingPrimary>{
 let index=c.positions[i].ok_or_else(invalid)?;
 let child=input.try_project::<dyn NativeValueSource,ValueError,_>(index,|original|if spec.tuple{original.sequence_child(index)}else{original.record_child(index)})?;
 let text=child.get().text()?;let count=demand.copy_bytes.checked_sub(size_of::<Option<String>>()).ok_or_else(invalid)?;
 if pending.is_none(){
  if !text.is_empty()&&effect.actual().is_some(){return Err(ValueError::invariant("string allocation marker occupied").into())}
  let mut buffer=String::new();if !text.is_empty(){*effect.actual()=Some(0)}
  if buffer.try_reserve_exact(text.len()).is_err(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"string byte allocation failed").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<String>>(),..Default::default()}).into())}
  if !text.is_empty(){*effect.actual()=Some(buffer.capacity())}
  if buffer.capacity()!=text.len(){return Err(ValueError::invariant("string byte allocator changed its exact original capacity").into())}
  **pending=Some(buffer);
 }
 let buffer=pending.as_mut().ok_or_else(invalid)?;let copied=buffer.len();
 buffer.push_str(text.get(copied..copied+count).ok_or_else(invalid)?);
 if buffer.len()==text.len(){c.stage=3}
 *work=u128::try_from(count).map_err(|_|invalid())?+1;Ok(())
}

fn project<'a,I:NativeValueSource+?Sized>(input:RetainedCloneRef<'a,I>,spec:&RecordSpec,shape:FieldShape,i:usize,index:usize,flat_key:bool)->Result<RetainedCloneRef<'a,dyn NativeValueSource>,ValueError>{
 let key=if flat_key&&shape.flat&&index==usize::MAX{usize::MAX-i}else{index};
 input.try_project::<dyn NativeValueSource,ValueError,_>(key,|original|if shape.flat&&index==usize::MAX{Ok(original.as_source())}else if spec.tuple{original.sequence_child(index)}else{original.record_child(index)})
}

/// 📐️ Plans one field stage of a record.
pub fn field_plan<I:NativeValueSource+?Sized,N,T,const C:usize>(c:&NativeRecordCore<I,N,C>,spec:&RecordSpec,shape:FieldShape,i:usize,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64,slot:Option<&dyn NativeFieldPlanSlot>,default:Option<&dyn NativeFieldPlanSlot>,text_copied:Option<usize>)->Result<(RetirementDemand,u64,u8),ValueError>{
 let option_bytes=size_of::<Option<T>>();let demand=NativeRecordStorage::<N>::field_demand::<T>;
 let stage=|slot:Option<&dyn NativeFieldPlanSlot>,present:bool,child:Option<RetainedCloneRef<'_,dyn NativeValueSource>>|plan_field_stage(c.stage,present,slot.ok_or_else(invalid)?,&c.source,child,&c.projection,body,maximum,option_bytes,demand);
 let Some(index)=c.positions[i]else{return if spec.required[i]{Err(invalid())}else{if shape.pure_default&&c.stage==0{Ok((fixed(size_of::<Option<T>>()+size_of::<u8>()),1,13))}else{stage(default,false,None)}}};
 if shape.text&&c.stage==0{return plan_text_stage(spec,input,index,body,maximum,text_copied)}
 if shape.leaf{return plan_leaf_stage(c.stage,option_bytes,demand)}
 if c.stage==0{return Ok((fixed(size_of::<u8>()),1,13))}
 let child=if c.stage==1{Some(project(input,spec,shape,i,index,true)?)}else{None};
 stage(slot,true,child)
}

/// ⏱️ Quotes the work of the next child action of one field.
pub fn field_work<I:NativeValueSource+?Sized,N,T,P:RecordSlot<T>,M:RecordSlot<T>,const C:usize>(c:&NativeRecordCore<I,N,C>,spec:&RecordSpec,shape:FieldShape,input:RetainedCloneRef<'_,I>,i:usize,body:usize,slot:Option<&P>,default:Option<&M>)->Result<u64,ValueError>{
 if let Some(index)=c.positions[i]{match slot{Some(slot)=>slot.slot_work(Some(project(input,spec,shape,i,index,true)?),body),None=>Err(invalid())}}else{match default{Some(default)=>default.slot_work(None,body),None=>Err(invalid())}}
}

/// 🪆️ Advances the nested child action of one field.
pub fn field_nested<I:NativeValueSource+?Sized,N,T,P:RecordSlot<T>,M:RecordSlot<T>,const C:usize>(c:&mut NativeRecordCore<I,N,C>,spec:&RecordSpec,shape:FieldShape,input:RetainedCloneRef<'_,I>,i:usize,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant,slot:Option<&mut P>,default:Option<&mut M>)->RetainedWorkPoll<Step,NativeReceivingFailure>{
 let failure=||RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});
 if let Some(index)=c.positions[i]{match(slot,project(input,spec,shape,i,index,true)){(Some(slot),Ok(child))=>slot.slot_nested(Some(child),control,witness,body,maximum,grant),(_,Err(error))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0}),_=>failure()}}else{match default{Some(default)=>default.slot_nested(None,control,witness,body,maximum,grant),None=>failure()}}
}

/// ♻️ Retires the nested child of one field.
pub fn field_retire<I:NativeValueSource+?Sized,N,T,P:RecordSlot<T>,M:RecordSlot<T>,const C:usize>(c:&NativeRecordCore<I,N,C>,i:usize,body:usize,effect:&mut RetainedAllocationEffect<'_>,slot:Option<&mut P>,default:Option<&mut M>)->Close{
 let failure=||RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});
 if c.positions[i].is_some(){match slot{Some(slot)=>slot.slot_retire(body,effect),None=>failure()}}else{match default{Some(default)=>default.slot_retire(body,effect),None=>failure()}}
}

/// 🚪️ Begins the close of the nested child of one field.
pub fn field_begin<I:NativeValueSource+?Sized,N,T,P:RecordSlot<T>,M:RecordSlot<T>,const C:usize>(c:&NativeRecordCore<I,N,C>,i:usize,effect:&mut RetainedAllocationEffect<'_>,slot:Option<&mut P>,default:Option<&mut M>)->Close{
 let failure=||RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});
 if c.positions[i].is_some(){match slot{Some(slot)=>slot.slot_begin(effect),None=>failure()}}else{match default{Some(default)=>default.slot_begin(effect),None=>failure()}}
}

/// 📥️ Admits the projection of one present field.
pub fn field_admit<'reply,I:NativeValueSource+?Sized,N,const C:usize>(c:&mut NativeRecordCore<I,N,C>,spec:&RecordSpec,shape:FieldShape,i:usize,input:RetainedCloneRef<'_,I>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,ValueError>)->Result<OriginalEffectReplyComplete<'reply,Step,ValueError>,ValueError>{
 let index=c.positions[i].ok_or_else(invalid)?;let child=project(input,spec,shape,i,index,true)?;c.projection.admit(child,effect,reply)
}

/// 🍃️ Reads the scalar of one present leaf field.
pub fn field_leaf<I:NativeValueSource+?Sized,N,T:NativeScalarValue<dyn NativeValueSource>,const C:usize>(c:&mut NativeRecordCore<I,N,C>,spec:&RecordSpec,i:usize,input:RetainedCloneRef<'_,I>,pending:&mut ManuallyDrop<Option<T>>,work:&mut u128)->Result<(),OriginalReceivingPrimary>{
 if pending.is_some(){return Err(invalid().into())}
 let index=c.positions[i].ok_or_else(invalid)?;
 let child=input.try_project::<dyn NativeValueSource,ValueError,_>(index,|original|if spec.tuple{original.sequence_child(index)}else{original.record_child(index)})?;
 let value=<T as NativeScalarValue<dyn NativeValueSource>>::read(child.get())?;**pending=Some(value);c.stage=3;*work=1;Ok(())
}

fn place_step<I:NativeValueSource+?Sized,N,T,const C:usize>(c:&mut NativeRecordCore<I,N,C>,offset:usize,i:usize,pending:&mut ManuallyDrop<Option<T>>,work:&mut u128,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,OriginalReceivingPrimary>{
 let turn=unsafe{c.partial.place_field::<T>(offset,pending,size_of::<bool>()+size_of::<u8>(),effect)};*work=turn.work;let placed=turn.result?.is_some();if placed{c.initialized[i]=true}Ok(placed)
}

fn take_step<I:NativeValueSource+?Sized,N,T,S:RecordSlot<T>,const C:usize>(c:&mut NativeRecordCore<I,N,C>,pending:&mut ManuallyDrop<Option<T>>,work:&mut u128,effect:&mut RetainedAllocationEffect<'_>,slot:&mut S)->Result<Option<Option<Step>>,OriginalReceivingPrimary>{
 match slot.slot_take(&mut **pending,effect){
  RetainedWorkPoll::Pending{..}=>return Ok(Some(None)),
  RetainedWorkPoll::Complete(turn)=>{*work=turn.work;turn.result?;if pending.is_none(){return Ok(Some(Some(Step::Progress(Default::default()))))}c.stage=3;*work+=2}}
 Ok(None)
}

/// 🪄️ Applies one step of one field: its projection, receipt, placement and slot close.
pub fn field_apply<I:NativeValueSource+?Sized,N,T,P:RecordSlot<T>,M:RecordSlot<T>,const C:usize>(c:&mut NativeRecordCore<I,N,C>,pure:fn()->Option<T>,shape:FieldShape,shared:bool,offset:usize,i:usize,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128,pending:&mut ManuallyDrop<Option<T>>,slot:Option<&mut P>,default:Option<&mut M>,frame:*mut Frame)->Result<Option<Option<Step>>,OriginalReceivingPrimary>{
 let Some(_)=c.positions[i]else{
  let default=default.ok_or_else(invalid)?;
  match c.stage{
   0=>{if shape.pure_default{if let Some(value)=pure(){**pending=Some(value);c.stage=3;*work=1}else{return Err(invalid().into())}}else{c.stage=1;*work=1}},1=>{c.stage=2;*work=1},
   2=>return take_step(c,pending,work,effect,default),
   3=>{if place_step(c,offset,i,pending,work,effect)?{c.stage=4;*work+=2}},
   4=>{c.stage=if default.view_empty(){5}else{7};*work=1},
   5=>{c.stage=6;*work=1},6=>{c.field+=1;c.stage=0;*work=2},
   7=>{c.stage=5;*work=1},_=>return Err(invalid().into())}
  return Ok(None)};
 if shape.leaf{match c.stage{3=>{if place_step(c,offset,i,pending,work,effect)?{c.field+=1;c.stage=0;*work+=2}},_=>return Err(invalid().into())}return Ok(None)}
 let slot=slot.ok_or_else(invalid)?;
 match c.stage{
  0=>{c.stage=1;*work=1},1=>{c.stage=2;*work=1},
  2=>return take_step(c,pending,work,effect,slot),
  3=>{if place_step(c,offset,i,pending,work,effect)?{c.stage=4;*work+=2}},
  4=>{c.stage=if slot.view_empty(){6}else{7};*work=1},
  5=>{if c.source.is_some(){c.source.take();}else{c.stage=6;}*work=1},
  6=>{if shared{slot.slot_reset(frame)}c.field+=1;c.stage=0;*work=2},
  7=>{c.stage=6;*work=1},_=>return Err(invalid().into())}
 Ok(None)
}

/// 📏️ Quotes the close of one field including its pending value and placed storage.
pub fn field_close_plan<I:NativeValueSource+?Sized,N,T:RetireOwned,P:RecordSlot<T>,M:RecordSlot<T>,const C:usize>(c:&NativeRecordCore<I,N,C>,i:usize,body:usize,pending:&ManuallyDrop<Option<T>>,slot:Option<&P>,default:Option<&M>)->Result<(RetirementDemand,u64),ValueError>{
 if let Some(plan)=slot.and_then(|slot|slot.slot_close_plan(body)){return plan}
 if let Some(plan)=default.and_then(|default|default.slot_close_plan(body)){return plan}
 if pending.is_some(){return Ok((c.retirement.next_owner_demands::<T>(true,&None,body)?,c.retirement.next_owner_work::<T>(true,&None,body)?))}
 if !c.partial.taken()&&c.initialized[i]{let mut d=NativeRecordStorage::<N>::field_demand::<T>(0)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(invalid)?;return Ok((d,2))}
 Ok((fixed(size_of::<usize>()),1))
}

/// 🔒️ Closes one field including its pending value and placed storage.
pub fn field_close_step<I:NativeValueSource+?Sized,N,T:RetireOwned,P:RecordSlot<T>,M:RecordSlot<T>,const C:usize>(c:&mut NativeRecordCore<I,N,C>,offset:usize,i:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,d:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>,pending:&mut ManuallyDrop<Option<T>>,slot:Option<&mut P>,default:Option<&mut M>)->Close{
 let mut original=original;
 if let Some(slot)=slot{if let Some(poll)=slot.slot_close_step(original.as_deref_mut(),body,effect){return poll}}
 if let Some(default)=default{if let Some(poll)=default.slot_close_step(original.as_deref_mut(),body,effect){return poll}}
 if pending.is_some(){let turn=c.retirement.begin_granted(pending,effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(Step::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
 if !c.partial.taken()&&c.initialized[i]{let turn=unsafe{c.partial.remove_field::<T>(offset,pending,size_of::<bool>(),effect)};let mut work=turn.work;let result=turn.result;if result.is_ok()&&pending.is_some(){c.initialized[i]=false;work+=1}return finish(effect,work,result.map_err(Into::into))}
 c.close_field+=1;finish(effect,1,Ok(Step::Progress(receipt(d))))
}

fn finish(effect:&mut RetainedAllocationEffect<'_>,work:u128,result:Result<Step,OriginalRetirementFailure>)->Close{let recorded=effect.performed_work();let result=if work>recorded{match u64::try_from(work-recorded){Ok(owed)=>match effect.performed(owed){Ok(true)=>result,Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native record close exceeded its original work").into()),Err(error)=>Err(error.into())},Err(_)=>Err(ValueError::invariant("native record close work overflow").into())}}else{result};RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()})}

/// 🧬️ The field arms of one derived record: only these depend on the record's field types.
pub trait NativeRecordArms<I:NativeValueSource+?Sized,N,const C:usize>:Send+'static{
 fn spec(&self)->&'static RecordSpec;
 fn schema(&self)->(NativeSchemaFields,usize);
 fn static_spec()->&'static RecordSpec where Self:Sized;
 fn born()->Self where Self:Sized;
 fn plan_field(&self,c:&NativeRecordCore<I,N,C>,i:usize,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<(RetirementDemand,u64,u8),ValueError>;
 fn work_field(&self,c:&NativeRecordCore<I,N,C>,input:RetainedCloneRef<'_,I>,i:usize,body:usize)->Result<u64,ValueError>;
 fn close_plan_field(&self,c:&NativeRecordCore<I,N,C>,i:usize,body:usize)->Result<(RetirementDemand,u64),ValueError>;
 fn apply_field(&mut self,c:&mut NativeRecordCore<I,N,C>,i:usize,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<Option<Step>>,OriginalReceivingPrimary>;
 fn leaf_field(&mut self,c:&mut NativeRecordCore<I,N,C>,i:usize,input:RetainedCloneRef<'_,I>,effect:&mut RetainedAllocationEffect<'_>,demand:RetirementDemand,work:&mut u128)->Result<(),OriginalReceivingPrimary>;
 fn nested_field(&mut self,c:&mut NativeRecordCore<I,N,C>,input:RetainedCloneRef<'_,I>,i:usize,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>;
 fn retire_field(&mut self,c:&mut NativeRecordCore<I,N,C>,i:usize,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close;
 fn begin_field(&mut self,c:&mut NativeRecordCore<I,N,C>,i:usize,effect:&mut RetainedAllocationEffect<'_>)->Close;
 fn close_field(&mut self,c:&mut NativeRecordCore<I,N,C>,i:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,d:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>)->Close;
 fn slot(&self,k:usize)->Option<&dyn RecordSlotView>;
 fn slot_mut(&mut self,k:usize)->Option<&mut dyn RecordSlotView>;
 fn pending_empty(&self)->bool;
 fn slots_empty(&self)->bool;
 fn frame_empty(&self)->bool;
 fn anchor(&mut self);
 fn seed_count()->usize where Self:Sized;
 unsafe fn seed_size(target:*mut Self,index:usize)->Option<usize> where Self:Sized;
 unsafe fn seed_write(target:*mut Self,index:usize)->bool where Self:Sized;
}

impl<I:NativeValueSource+?Sized,N:'static,const C:usize> NativeRecordCore<I,N,C>{
 fn new(spec:&RecordSpec)->Self{Self{binding:None,positions:std::array::from_fn(|i|if spec.flatten[i]{Some(usize::MAX)}else{None}),initialized:[false;C],partial:NativeRecordStorage::empty(C),output:ManuallyDrop::new(None),source:ManuallyDrop::new(None),matcher:None,projection:Default::default(),retirement:Default::default(),total:None,scan:0,field:0,close_field:0,phase:0,stage:0,closing:false,subset:false}}
}

fn scan_plan<I:NativeValueSource+?Sized,N,const C:usize>(c:&NativeRecordCore<I,N,C>,spec:&RecordSpec,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<(RetirementDemand,u64,u8),ValueError>{
 if spec.tuple{return Ok((fixed(size_of::<Option<usize>>()+size_of::<usize>()),2,2))}
 if let Some(matcher)=c.matcher.as_ref(){
  if c.stage==1{return if matcher.terminal_is_empty(){Ok((fixed(size_of::<Option<NativeRecordMatcher<I>>>()+size_of::<usize>()+size_of::<u8>()),3,6))}else{Ok((matcher.close_demands(body)?,matcher.close_work_demand(body)?,5))}}
  if matcher.result().is_some(){let mut d=matcher.begin_close_demand();let field=matches!(matcher.result(),Some(NativeFieldMatch::Field(_)));d.copy_bytes=d.copy_bytes.checked_add(if field{size_of::<Option<usize>>()+size_of::<u8>()}else{size_of::<u8>()}).ok_or_else(invalid)?;return Ok((d,tree::work_sum(matcher.begin_close_work_demand(),if field{2}else{1})?,4))}
  return match matcher.select_advance(input,body,maximum)?{NativeAdvanceSelection::Nested=>Ok((matcher.demands(input,body,maximum)?,matcher.nested_work_demand(input)?,3)),NativeAdvanceSelection::Local{work}=>Ok((matcher.demands(input,body,maximum)?,work,22))}
 }
 if spec.any_flatten{return Ok((fixed(size_of::<Option<NativeRecordMatcher<I>>>()),1,2))}
 match input.get().record_key(c.scan){Ok(NativeRecordKey::Ordinal(_))=>Ok((fixed(size_of::<Option<usize>>()+size_of::<usize>()),2,26)),_=>Ok((fixed(size_of::<Option<NativeRecordMatcher<I>>>()),1,2))}
}

fn plan<I:NativeValueSource+?Sized,N:'static,const C:usize>(c:&NativeRecordCore<I,N,C>,a:&dyn NativeRecordArms<I,N,C>,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<(RetirementDemand,u64,u8),ValueError>{
 let spec=a.spec();
 if c.closing||c.phase>=5{return Err(invalid())}
 if c.binding.is_none(){return Ok((fixed(input.binding_copy_bytes()),input.binding_work_demand(&c.binding)?,0))}
 input.check_binding(&c.binding)?;
 if c.total.is_none(){return Ok((fixed(size_of::<Option<usize>>()+size_of::<u8>()),2u64,1))}
 match c.phase{
  1=>if c.scan==c.total.ok_or_else(invalid)?&&c.matcher.is_none(){Ok((fixed(size_of::<u8>()+size_of::<usize>()),2,7))}else{scan_plan(c,spec,input,body,maximum)},
  2=>match c.stage{
   0=>if c.field==spec.count{Ok((fixed(size_of::<u8>()+size_of::<usize>()),2,9))}else{Ok((fixed(size_of::<usize>()),1,8))},
   1=>if c.partial.reserved(){Ok((fixed(size_of::<u8>()),1,19))}else{Ok((NativeRecordStorage::<N>::reserve_demand(),c.partial.reserve_work_demand(),10))},
   2=>if c.partial.initialized(){Ok((fixed(size_of::<u8>()),1,20))}else{Ok((NativeRecordStorage::<N>::initialize_demand(),1u64,11))},
   3=>Ok((fixed(2*size_of::<u8>()+size_of::<usize>()),3,12)),_=>Err(invalid())},
  3=>if c.field==spec.count{Ok((fixed(size_of::<u8>()),1,17))}else{a.plan_field(c,c.field,input,body,maximum)},
  4=>{let mut d=NativeRecordStorage::<N>::take_initialized_demand(0)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<Option<N>>()+size_of::<u8>()).ok_or_else(invalid)?;Ok((d,tree::work_sum(1u64,2)?,18))},
  _=>Err(invalid())}
}

fn apply<I:NativeValueSource+?Sized,N:'static,const C:usize>(c:&mut NativeRecordCore<I,N,C>,a:&mut dyn NativeRecordArms<I,N,C>,action:u8,input:RetainedCloneRef<'_,I>,effect:&mut RetainedAllocationEffect<'_>,d:RetirementDemand)->Result<Option<Step>,OriginalReceivingPrimary>{
 let spec=a.spec();let mut work:u128=0;
 let step=(||->Result<Option<Step>,OriginalReceivingPrimary>{match action{
  0=>{let turn=input.bind(&mut c.binding,effect);work=turn.work;return turn.result.map(|p|Some(Step::Progress(p.unwrap_or_default()))).map_err(Into::into)},
  1=>{let total=if spec.tuple{input.get().sequence_len()?}else{input.get().record_len()?};if spec.tuple&&total!=spec.count{return Err(invalid().into())}c.total=Some(total);c.phase=1;work=2;},
  2=>{if spec.tuple{c.positions[c.scan]=Some(c.scan);c.scan+=1;work=2;}else if spec.any_flatten{c.matcher=Some(NativeRecordMatcher::new(a.schema().0,a.schema().1,NativeSchemaKeySource::Record(c.scan)));work=1;}else{c.matcher=Some(NativeRecordMatcher::new_flat(spec.leaves,NativeSchemaKeySource::Record(c.scan)));work=1;}},
  4=>{let matcher=c.matcher.as_mut().ok_or_else(invalid)?;match matcher.result().ok_or_else(invalid)?{NativeFieldMatch::Missing if !c.subset=>return Err(invalid().into()),NativeFieldMatch::Ambiguous=>return Err(invalid().into()),NativeFieldMatch::Field(field)=>{let flatten=*spec.flatten.get(field).ok_or_else(invalid)?;if !flatten||matcher.ordinal_key(){if c.positions[field].is_some_and(|position|!flatten||position!=usize::MAX){return Err(invalid().into())}c.positions[field]=Some(c.scan)}work=1},_=>{}}let previous=work;let turn=matcher.begin_close_paid(effect);work=previous+turn.work;turn.result?;c.stage=1;work+=1;},
  6=>{c.matcher=None;c.scan+=1;c.stage=0;work=3},7=>{c.phase=2;c.field=0;work=2},
  8=>{if c.field>=spec.count{return Err(invalid().into())}if spec.required[c.field]&&c.positions[c.field].is_none(){return Err(invalid().into())}c.field+=1;work=1},
  9=>{c.field=0;c.stage=1;work=2},
  10=>match c.partial.reserve_empty(effect){RetainedWorkPoll::Pending{..}=>return Ok(None),RetainedWorkPoll::Complete(turn)=>{work=turn.work;return turn.result.map(Some).map_err(Into::into)}},
  11=>{let turn=c.partial.initialize(effect);work=turn.work;return turn.result.map(Some).map_err(Into::into)},
  12=>{c.phase=3;c.stage=0;c.field=0;work=3},
  13=>{let field=c.field;if let Some(early)=a.apply_field(c,field,effect,&mut work)?{return Ok(early)}},
  17=>{c.phase=4;work=1},
  18=>{let extra=size_of::<Option<N>>()+size_of::<u8>();match unsafe{c.partial.take_initialized_into(&mut c.output,extra,effect)}{RetainedWorkPoll::Pending{..}=>return Ok(None),RetainedWorkPoll::Complete(turn)=>{work=turn.work;turn.result?;if c.output.is_some(){c.phase=5;work+=2;}}}},
  19=>{c.stage=2;work=1},20=>{c.stage=3;work=1},
  26=>{if spec.tuple||spec.any_flatten{return Err(invalid().into())}let NativeRecordKey::Ordinal(ordinal)=input.get().record_key(c.scan)?else{return Err(invalid().into())};match spec.leaves.iter().position(|leaf|leaf.ordinal==u32::from(ordinal)){Some(field)=>{if c.positions[field].is_some(){return Err(invalid().into())}c.positions[field]=Some(c.scan)},None if c.subset=>{},None=>return Err(invalid().into())}c.scan+=1;work=2;},
  25|27=>{let field=c.field;a.leaf_field(c,field,input,effect,d,&mut work)?},
  _=>return Err(invalid().into())}Ok(Some(Step::Progress(receipt(d))))})()?;
 let Some(step)=step else{return Ok(None)};
 let recorded=effect.performed_work();
 if work>recorded{let owed=u64::try_from(work-recorded).map_err(|_|ValueError::invariant("native record action work overflow"))?;if !effect.performed(owed)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native record action exceeded its original work").into())}}else if work<recorded{return Err(ValueError::invariant("native record action recorded more work than it quoted").into())}
 Ok(Some(step))
}

fn retire_normal<I:NativeValueSource+?Sized,N:'static,const C:usize>(c:&mut NativeRecordCore<I,N,C>,a:&mut dyn NativeRecordArms<I,N,C>,action:u8,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{
 let failure=||RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});
 let field=c.field;match action{
  5=>{let matcher=c.matcher.as_mut().expect("selected retained matcher");native_close_settle(effect,|effect,reply|matcher.close_into(body,effect,reply)).map(progress)},
  15=>a.retire_field(c,field,body,effect),
  16=>c.source.as_mut().expect("selected retained source").close_step(body,effect).map(progress),
  21=>a.begin_field(c,field,effect),
  _=>failure()}
}

fn close_plan<I:NativeValueSource+?Sized,N:RetireOwned,const C:usize>(c:&NativeRecordCore<I,N,C>,a:&dyn NativeRecordArms<I,N,C>,body:usize)->Result<(RetirementDemand,u64),ValueError>{
 let spec=a.spec();
 if !c.closing{return Err(invalid())}
 if let Some(m)=c.matcher.as_ref(){return if m.terminal_is_empty(){Ok((fixed(size_of::<Option<NativeRecordMatcher<I>>>()),1))}else if !m.close_started(){Ok((m.begin_close_demand(),m.begin_close_work_demand()))}else{Ok((m.close_demands(body)?,m.close_work_demand(body)?))}}
 if !c.projection.terminal_is_empty(){return Ok((c.projection.close_demand(body)?,c.projection.close_work_demand(body)?))}
 if let Some(source)=c.source.as_ref(){return if source.terminal_is_empty(){Ok((fixed(size_of::<Option<Projection>>()),1))}else{Ok((source.close_demand(body)?,source.close_work_demand(body)?))}}
 if !c.retirement.is_empty(){return Ok((c.retirement.demands(body)?,c.retirement.next_owner_work::<N>(false,&None,body)?))}
 if c.close_field<spec.count{return a.close_plan_field(c,c.close_field,body)}
 if c.output.is_some(){return Ok((c.retirement.next_owner_demands::<N>(true,&None,body)?,c.retirement.next_owner_work::<N>(true,&None,body)?))}
 if c.partial.reserved(){return Ok((c.partial.release_demand(),c.partial.release_work_demand()))}
 if c.binding.is_some(){return Ok((RetainedCloneBinding::close_demand(&c.binding,body)?,RetainedCloneBinding::close_work_demand(&c.binding,body)?))}
 Ok((fixed(size_of::<u8>()),u64::from(c.phase!=7)))
}

fn close_poll<I:NativeValueSource+?Sized,N:RetireOwned,const C:usize>(c:&mut NativeRecordCore<I,N,C>,a:&mut dyn NativeRecordArms<I,N,C>,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Close{
 use RetainedWorkPoll::Complete;
 let spec=a.spec();let g=effect.grant();
 if is_empty(c,a){return Complete(RetainedWorkOutcome{result:Ok(Step::Complete(Default::default())),work:0})}
 let d=match close_plan(c,a,g.maximum_copy_bytes){Ok(plan)=>plan.0,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
 if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(Step::Progress(Default::default())),work:0})}
 if let Some(m)=c.matcher.as_mut(){
  if m.terminal_is_empty(){c.matcher=None;return finish(effect,1,Ok(Step::Progress(receipt(d))))}
  if !m.close_started(){let turn=m.begin_close_paid(effect);return Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
  let control=m.requires_original_control();
  return native_close_settle(effect,|effect,reply|match original.as_deref_mut().filter(|_|control){Some(loan)=>m.close_original_into(loan,body,effect,reply),None=>m.close_into(body,effect,reply)}).map(progress)}
 if !c.projection.terminal_is_empty(){if c.projection.requires_original_control(){if let Some(loan)=original.as_deref_mut(){return c.projection.close_original_step(loan,body,effect).map(progress)}}return native_close_settle(effect,|effect,reply|c.projection.close_into(body,effect,reply)).map(progress)}
 if let Some(source)=c.source.as_mut(){if source.terminal_is_empty(){c.source.take();return finish(effect,1,Ok(Step::Progress(receipt(d))))}if source.requires_original_control(){if let Some(loan)=original.as_deref_mut(){return source.close_original_step(loan,body,effect).map(progress)}}return source.close_step(body,effect).map(progress)}
 if !c.retirement.is_empty(){if c.retirement.requires_original_control(){if let Some(loan)=original.as_deref_mut(){return c.retirement.step_original(loan,body,effect).map(progress)}}return c.retirement.step_granted(body,effect).map(progress)}
 if c.close_field<spec.count{let field=c.close_field;return a.close_field(c,field,original,body,d,effect)}
 if c.output.is_some(){let turn=c.retirement.begin_granted(&mut c.output,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(Step::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
 if c.partial.reserved(){let turn=c.partial.release(effect);return Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
 if c.binding.is_some(){if RetainedCloneBinding::requires_original_control(&c.binding){if let Some(loan)=original.as_deref_mut(){return RetainedCloneBinding::close_original_step(&mut c.binding,loan,body,effect)}}return RetainedCloneBinding::close_one(&mut c.binding,body,effect)}
 c.phase=7;finish(effect,1,Ok(Step::Complete(receipt(d))))
}

fn is_empty<I:NativeValueSource+?Sized,N:'static,F:NativeRecordArms<I,N,C>+?Sized,const C:usize>(c:&NativeRecordCore<I,N,C>,a:&F)->bool{
 c.binding.is_none()&&c.source.is_none()&&c.matcher.is_none()&&c.projection.terminal_is_empty()&&c.retirement.is_empty()&&c.partial.terminal_is_empty()&&c.output.is_none()&&a.frame_empty()&&a.pending_empty()&&a.slots_empty()&&(c.phase==7||c.phase==0)
}

fn requires_original<I:NativeValueSource+?Sized,N:'static,const C:usize>(c:&NativeRecordCore<I,N,C>,a:&dyn NativeRecordArms<I,N,C>,birth:bool)->bool{
 if let Some(m)=c.matcher.as_ref(){if !m.terminal_is_empty(){return if birth{m.requires_original_birth()}else{m.requires_original_control()}}}
 if !c.projection.terminal_is_empty(){return if birth{c.projection.requires_original_birth()}else{c.projection.requires_original_control()}}
 if let Some(source)=c.source.as_ref(){if !source.terminal_is_empty(){return if birth{source.requires_original_birth()}else{source.requires_original_control()}}}
 if !c.retirement.is_empty(){return if birth{c.retirement.requires_original_birth()}else{c.retirement.requires_original_control()}}
 for k in 0.. {let Some(slot)=a.slot(k)else{break};if !slot.view_empty(){return if birth{slot.view_birth()}else{slot.view_control()}}}
 if birth{RetainedCloneBinding::requires_original_birth(&c.binding)}else{RetainedCloneBinding::requires_original_control(&c.binding)}
}

fn close_original_birth_step<I:NativeValueSource+?Sized,N:'static,const C:usize>(c:&mut NativeRecordCore<I,N,C>,a:&mut dyn NativeRecordArms<I,N,C>,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
 if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(Step::Progress(Default::default())),work:0})}
 let inner=child_grant(grant,0);
 if let Some(m)=c.matcher.as_mut(){if !m.terminal_is_empty(){return m.close_original_birth_step(original,inner)}}
 if !c.projection.terminal_is_empty(){return c.projection.close_original_birth_step(original,inner)}
 if let Some(source)=c.source.as_mut(){if !source.terminal_is_empty(){return source.close_original_birth_step(original,inner)}}
 if !c.retirement.is_empty(){return c.retirement.step_original_birth(original,grant)}
 for k in 0.. {let Some(slot)=a.slot_mut(k)else{break};if !slot.view_empty(){return slot.view_birth_step(original,inner)}}
 RetainedCloneBinding::close_original_birth_step(&mut c.binding,original,grant)
}

/// 🏗️ One native record receiver: the shared core plus the field arms of its derived record.
pub struct NativeRecordReceiver<I:NativeValueSource+?Sized,N:'static,F:NativeRecordArms<I,N,C>,const C:usize>{core:NativeRecordCore<I,N,C>,fields:F}

impl<I:NativeValueSource+?Sized,N:'static,F:NativeRecordArms<I,N,C>,const C:usize> NativeRecordReceiver<I,N,F,C>{
 /// 🔢️ Counts the field slots that currently own a live child.
 pub fn native_field_owner_count(&self)->usize{(0..).map_while(|k|self.fields.slot(k)).filter(|slot|!slot.view_empty()).count()}
 /// 🌱️ Births the receiver around its born field arms.
 pub fn birth(subset:bool)->Self{let mut fields=F::born();fields.anchor();let mut core=NativeRecordCore::new(fields.spec());core.subset=subset;Self{core,fields}}
 const CORE_SEEDS:usize=17;
 unsafe fn seed_size(target:*mut MaybeUninit<Self>,index:usize)->Option<usize>{
  fn size<T>(_:*mut T)->usize{size_of::<T>().min(SEED_WRITE_BYTES)}
  let target=target.cast::<Self>();
  Some(unsafe{match index{
   0=>size(std::ptr::addr_of_mut!((*target).core.binding)),1=>size(std::ptr::addr_of_mut!((*target).core.positions)),2=>size(std::ptr::addr_of_mut!((*target).core.initialized)),
   3=>size(std::ptr::addr_of_mut!((*target).core.partial)),4=>size(std::ptr::addr_of_mut!((*target).core.output)),5=>size(std::ptr::addr_of_mut!((*target).core.source)),
   6=>size(std::ptr::addr_of_mut!((*target).core.matcher)),7=>size(std::ptr::addr_of_mut!((*target).core.projection)),8=>size(std::ptr::addr_of_mut!((*target).core.retirement)),
   9=>size(std::ptr::addr_of_mut!((*target).core.total)),10=>size(std::ptr::addr_of_mut!((*target).core.scan)),11=>size(std::ptr::addr_of_mut!((*target).core.field)),
   12=>size(std::ptr::addr_of_mut!((*target).core.close_field)),13=>size(std::ptr::addr_of_mut!((*target).core.phase)),14=>size(std::ptr::addr_of_mut!((*target).core.stage)),
   15=>size(std::ptr::addr_of_mut!((*target).core.closing)),16=>size(std::ptr::addr_of_mut!((*target).core.subset)),
   field=>return F::seed_size(std::ptr::addr_of_mut!((*target).fields),field-Self::CORE_SEEDS)}}) }
 unsafe fn seed_write<const SUBSET:bool>(target:*mut MaybeUninit<Self>,index:usize)->bool{
  let target=target.cast::<Self>();
  unsafe{match index{
   0=>std::ptr::addr_of_mut!((*target).core.binding).write(None),
   1=>std::ptr::addr_of_mut!((*target).core.positions).write(std::array::from_fn(|i|if F::static_spec().flatten[i]{Some(usize::MAX)}else{None})),
   2=>std::ptr::addr_of_mut!((*target).core.initialized).write([false;C]),
   3=>std::ptr::addr_of_mut!((*target).core.partial).write(NativeRecordStorage::empty(C)),
   4=>std::ptr::addr_of_mut!((*target).core.output).write(ManuallyDrop::new(None)),
   5=>std::ptr::addr_of_mut!((*target).core.source).write(ManuallyDrop::new(None)),
   6=>std::ptr::addr_of_mut!((*target).core.matcher).write(None),
   7=>std::ptr::addr_of_mut!((*target).core.projection).write(Default::default()),
   8=>std::ptr::addr_of_mut!((*target).core.retirement).write(Default::default()),
   9=>std::ptr::addr_of_mut!((*target).core.total).write(None),
   10=>std::ptr::addr_of_mut!((*target).core.scan).write(0),
   11=>std::ptr::addr_of_mut!((*target).core.field).write(0),
   12=>std::ptr::addr_of_mut!((*target).core.close_field).write(0),
   13=>std::ptr::addr_of_mut!((*target).core.phase).write(0),
   14=>std::ptr::addr_of_mut!((*target).core.stage).write(0),
   15=>std::ptr::addr_of_mut!((*target).core.closing).write(false),
   16=>std::ptr::addr_of_mut!((*target).core.subset).write(SUBSET),
   field=>return F::seed_write(std::ptr::addr_of_mut!((*target).fields),field-Self::CORE_SEEDS)}}
  true}
 /// 🧬️ Returns the factory seed template of this record shape.
 unsafe fn seed_all_size(target:*mut MaybeUninit<Self>,index:usize)->Option<usize>{if index!=0{return None}let mut total=0usize;for part in 0..Self::CORE_SEEDS+F::seed_count(){total=total.checked_add(unsafe{Self::seed_size(target,part)}?)?}Some(total)}
 unsafe fn seed_all_write<const SUBSET:bool>(target:*mut MaybeUninit<Self>,index:usize)->bool{index==0&&(0..Self::CORE_SEEDS+F::seed_count()).all(|part|unsafe{Self::seed_write::<SUBSET>(target,part)})}
 /// 🪙️ One funded step writes every vacant part of the receiver: the parts carry no custody, so they need no separate receipts.
 pub fn template(subset:bool)->Option<NativeSeedTemplate<Self>>{let write:unsafe fn(*mut MaybeUninit<Self>,usize)->bool=if subset{Self::seed_all_write::<true>}else{Self::seed_all_write::<false>};Some(unsafe{NativeSeedTemplate::fields(1,Self::seed_all_size,write)})}
}

impl<I:NativeValueSource+?Sized,N:RetireOwned+'static,F:NativeRecordArms<I,N,C>,const C:usize> NativeReceiver<I,N> for NativeRecordReceiver<I,N,F,C>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{let(d,q,_)=plan(&self.core,&self.fields,input,body,maximum)?;Ok(if q<=maximum{d}else{Default::default()})}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<NativeAdvanceSelection,ValueError>{let(_,q,action)=plan(&self.core,&self.fields,input,body,maximum)?;Ok(if matches!(action,3|14){NativeAdvanceSelection::Nested}else{NativeAdvanceSelection::Local{work:q}})}
 fn advance_work_demand(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<u64,ValueError>{let(_,q,action)=plan(&self.core,&self.fields,input,body,u64::MAX)?;if action!=14{return Ok(q)}self.fields.work_field(&self.core,input,self.core.field,body)}
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,NativeReceivingFailure>{
  use RetainedWorkPoll::Complete;
  let(d,q,action)=match plan(&self.core,&self.fields,input,body,maximum){Ok(plan)=>plan,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if (q==0&&action!=3)||q>maximum||!retained_clone_grant_admits(grant,d){return Complete(RetainedWorkOutcome{result:Ok(Step::Progress(Default::default())),work:0})}
  if action==3{return self.core.matcher.as_mut().expect("selected retained matcher").advance_nested(input,control,witness,body,maximum,grant)}
  if action==14{let field=self.core.field;return self.fields.nested_field(&mut self.core,input,field,control,witness,body,maximum,grant)}
  Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalReceivingPrimary>,ValueError>{
  let maximum=effect.maximum_work_units();
  if let Ok((d,q,action@(23|24)))=plan(&self.core,&self.fields,input,body,maximum){if q==0||q>maximum||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}
   let done=if action==23{let field=self.core.field;let spec=self.fields.spec();let core=&mut self.core;native_relay(reply.progress(),|reply:OriginalEffectReply<'_,Step,ValueError>|field_admit(core,spec,FieldShape{leaf:false,flat:spec.flatten[field],text:false,pure_default:false},field,input,effect,reply))}else{let core=&mut self.core;native_relay(reply.progress(),|reply|core.projection.take(&mut *core.source,effect,reply))};
   if action==23&&self.core.projection.borrow().is_ok(){self.core.stage=8}else if action==24&&self.core.source.is_some(){self.core.stage=1}
   return done}
  if let Ok((d,q,22))=plan(&self.core,&self.fields,input,body,maximum){if q==0||q>maximum||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}return self.core.matcher.as_mut().expect("selected retained matcher").advance_local(input,body,effect,reply.progress())}
  let result=(||->Result<Option<Step>,OriginalReceivingPrimary>{
   let(d,q,action)=plan(&self.core,&self.fields,input,body,maximum)?;
   if q==0||q>maximum||!retained_clone_grant_admits(effect.grant(),d){return Ok(Some(Step::Progress(Default::default())))}
   if matches!(action,5|15|16|21){return match retire_normal(&mut self.core,&mut self.fields,action,body,effect){RetainedWorkPoll::Pending{..}=>Ok(None),RetainedWorkPoll::Complete(turn)=>turn.result.map(Some).map_err(Into::into)}}
   let step=apply(&mut self.core,&mut self.fields,action,input,effect,d)?;
   Ok(step.map(|step|if self.output_ready(){Step::Complete(step.progress())}else{step}))
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})}
 fn output_ready(&self)->bool{!self.core.closing&&self.core.output.is_some()&&self.core.phase==5}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if self.output_ready(){Ok(fixed(0))}else{Err(invalid())}}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 fn take<'reply>(&mut self,output:&mut Option<N>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalReceivingPrimary>,ValueError>{
  let turn=(||->RetainedWorkOutcome<Step,OriginalReceivingPrimary>{
   let d=match self.take_demand(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
   if output.is_some()||self.core.output.is_none(){return RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
   if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return RetainedWorkOutcome{result:Ok(Step::Progress(Default::default())),work:0}}
   match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:1}}
   *output=self.core.output.take();self.core.phase=6;RetainedWorkOutcome{result:Ok(Step::Complete(receipt(d))),work:1}})();
  Ok(reply.complete(turn.result))}
 fn advance_pending(&self)->bool{self.core.matcher.as_ref().is_some_and(|m|m.advance_pending())||(0..).map_while(|k|self.fields.slot(k)).any(|slot|slot.view_pending())}
 fn is_closing(&self)->bool{self.core.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(if self.core.closing{Default::default()}else{fixed(size_of::<bool>())})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.core.closing))}
 fn begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{
  if self.core.closing{return Ok(reply.complete(Ok(Step::Complete(Default::default()))))}
  let d=fixed(size_of::<bool>());
  if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(Step::Progress(Default::default()))))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native record close start exceeded its work authority").into())))}
  self.core.closing=true;Ok(reply.complete(Ok(Step::Complete(receipt(d)))))}
 fn recover_begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{self.begin_close_into(effect,reply)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{close_plan(&self.core,&self.fields,body).map(|p|p.0)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{close_plan(&self.core,&self.fields,body).map(|p|p.1)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{Ok(match close_poll(&mut self.core,&mut self.fields,None,body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})}
 fn recover_close_step<'reply>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{Ok(match close_poll(&mut self.core,&mut self.fields,Some(original),body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})}
 fn requires_original_birth(&self)->bool{requires_original(&self.core,&self.fields,true)}
 fn requires_original_control(&self)->bool{requires_original(&self.core,&self.fields,false)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{close_original_birth_step(&mut self.core,&mut self.fields,original,grant)}
 fn recover_close_original_step<'reply>(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Step,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,Step,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn terminal_is_empty(&self)->bool{is_empty(&self.core,&self.fields)}
 fn flat_close_demand(&self)->Option<(RetirementDemand,u64)>{
  let c=&self.core;let a=&self.fields;
  if !matches!(c.phase,5|6)||c.matcher.is_some()||c.source.is_some()||!c.projection.terminal_is_empty()||!c.retirement.is_empty()||(c.partial.initialized()&&!c.partial.taken())||c.output.is_some()||!a.frame_empty()||!a.pending_empty()||!a.slots_empty(){return None}
  let alias=RetainedCloneBinding::flat_release_quote(&c.binding).ok()?;let count=a.spec().count;
  let mut copy_bytes=size_of::<bool>()+count*size_of::<usize>();let mut work=1+u64::try_from(count).ok()?;let mut release_bytes=0;
  if let Some(demand)=alias{copy_bytes+=demand.copy_bytes;work+=1}
  if c.partial.reserved(){let release=c.partial.release_demand();copy_bytes+=release.copy_bytes;release_bytes=release.release_bytes;work+=c.partial.release_work_demand()}
  Some((RetirementDemand{copy_bytes,release_bytes,depth:1,..Default::default()},work))
 }
 fn flat_close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
  let(demand,work)=self.flat_close_demand().ok_or_else(invalid)?;
  if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(Default::default())}
  let release_work=if self.core.partial.reserved(){self.core.partial.release_work_demand()}else{0};
  if !effect.performed(work-release_work)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native record flat close exceeded its original work"))}
  if self.core.partial.reserved(){self.core.partial.release(effect).result?;}
  if !RetainedCloneBinding::flat_release(&mut self.core.binding){return Err(ValueError::invariant("native record alias changed during its flat close"))}
  self.core.closing=true;self.core.close_field=self.fields.spec().count;self.core.phase=7;Ok(receipt(demand))
 }
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(|owner:&Self|owner.terminal_is_empty())})}
}

impl<I:NativeValueSource+?Sized,N:'static,F:NativeRecordArms<I,N,C>,const C:usize> Drop for NativeRecordReceiver<I,N,F,C>{fn drop(&mut self){assert!(std::thread::panicking()||is_empty(&self.core,&self.fields),"native record receiving requires funded closure")}}
