//! 🔎️ Duplicate names compare bounded bytes from each original row while excluding the addressed identifier.
use super::*;
use semio_framework_value_persistent::list::persistent::PersistentListRead;
use semio_framework_value::retained_clone::{RetainedOwnedProjectionAdmission,RetainedCloneBinding};
use crate::custody::{self,Cursor,Bound};
use crate::scalar::{Quote,bind};
use crate::retained::{Done,Flight,Reply,conclude,native_child};
use semio_framework_value::{RetainedAllocationEffect,RetirementPollExit};
crate::custody::std_cursor!([T:EntityRow] RetainedOwnedProjectionAdmission<T>,[],|_|{});
pub(super) struct NameLookup<S:TextRowPath>{reader:PersistentListRead<S::Row>,admission:RetainedOwnedProjectionAdmission<S::Row>,row:Option<Box<RetainedOwnedProjection<S::Row>>>,equality:RetainedEqualityOwner<String>,input:Bound,key:Bound,phase:u8,result:Option<bool>,closing:bool,flight:Flight}
impl<S:TextRowPath> NameLookup<S>{
 pub fn new()->Self{Self{reader:PersistentListRead::new(),admission:Default::default(),row:None,equality:RetainedEqualityOwner::new(),input:Default::default(),key:Default::default(),phase:0,result:None,closing:false,flight:Default::default()}}
 pub fn result(&self)->Option<bool>{self.result}
 fn old_of(row:&Option<Box<RetainedOwnedProjection<S::Row>>>)->Result<RetainedCloneRef<'_,String>,ValueError>{Ok(row.as_ref().ok_or_else(invalid)?.borrow()?.project(0,|r|S::read(r).get()))}
 fn fixed()->usize{2*size_of::<Option<RetainedOwnedProjection<S::Row>>>()+size_of::<u32>()+2}
 fn admit_worst()->RetirementDemand{RetirementDemand{copy_bytes:RetainedCloneBinding::alias_copy_bytes()+size_of::<Option<std::ptr::NonNull<S::Row>>>(),depth:1,..Default::default()}}
 pub fn quote(&self,input:RetainedCloneRef<'_,String>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{if self.closing||self.result.is_some()||!self.flight.idle(){return Err(invalid());}input.check_binding(&self.input.0)?;key.check_binding(&self.key.0)?;match self.phase{
  0=>Ok((nested(scalar(input.binding_copy_bytes()),1)?,input.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(key.binding_copy_bytes()),1)?,key.binding_work_demand(&self.key.0)?)),
  2=>{let mut d=self.reader.demand(S::rows(original))?;let a=Self::admit_worst();d.copy_bytes+=a.copy_bytes;Ok((nested(d,Self::fixed())?,self.reader.advance_work_demand(S::rows(original))?.saturating_add(RetainedCloneBinding::alias_work_demand())))},
  6=>Ok((nested(self.admission.take_demand()?,Self::fixed())?,self.admission.take_work_demand()?)),
  3=>Ok((nested(self.equality.demand(input,Self::old_of(&self.row)?,body.saturating_sub(2),max)?,2)?,self.equality.work_demand(input,Self::old_of(&self.row)?,body.saturating_sub(2),max)?)),
  4=>Ok((Cursor::demand(&self.equality,body)?,Cursor::work(&self.equality,body)?)),
  5=>Ok((Cursor::demand(&self.row,body)?,Cursor::work(&self.row,body)?)),
  _=>Err(invalid())}}
 pub fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,String>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.quote(input,key,original,body,max))};let g=effect.grant();
  custody::gated(effect,reply,quote,max,|effect,work|{let mut p=RetainedCloneProgress::default();match self.phase{
   0=>{p=add(bind(input,&mut self.input,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=1;},1=>{p=add(bind(key,&mut self.key,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;},
   2=>{let extra=Self::fixed();let mut row=None;let reader=&mut self.reader;let step=native_child(effect,work,|e,r|reader.advance_into(S::rows(original),&mut row,&mut e.nested(child(g,extra)),r))??;p=add(step.progress(),extra)?;
    if let Some((_,projected))=row{if projected.get().entity_id()!=*key.get(){let admission=&mut self.admission;let admitted=native_child(effect,work,|e,r|admission.admit(projected,e,r))??;p=add(p,admitted.progress().copied_bytes)?;self.phase=6;}}else if matches!(step,RetainedCloneStep::Complete(_)){self.result=Some(false);}},
   6=>{let mut taken=None;let admission=&mut self.admission;let step=native_child(effect,work,|e,r|admission.take(&mut taken,e,r))??;self.row=taken.map(Box::new);p=add(step.progress(),Self::fixed())?;if self.row.is_some(){self.phase=3;}},
   3=>{let old=Self::old_of(&self.row)?;let nb=body.saturating_sub(2);let equality=&mut self.equality;
    let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,*work,|r| equality.advance_into(input,old,nb,max,&mut effect.nested(child(g,2)),r),equality.recover_advance_into(input,old,nb,max,&mut effect.nested(child(g,2)),r));
    let step=step?;p=add(step.progress,2)?;if let Some(equal)=step.equal{if equal{self.result=Some(true);}else{self.phase=4;}}},
   4=>{let step=custody::chain_step(&mut self.flight,&mut [&mut self.equality as &mut dyn Cursor],None,body,effect,work,recover)?;p=step.progress();if self.equality.terminal_is_empty(){self.phase=5;}},
   5=>{let step=custody::finish(&mut self.flight,&mut self.row,body,effect,work,recover)?;p=step.progress();if self.row.is_none(){self.equality=RetainedEqualityOwner::new();self.phase=2;}},
   _=>return Err(invalid().into())}
   Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 pub fn begin_close(&mut self){self.closing=true;}
 custody_impl!(equality,row,admission,reader,input,key);
}
crate::custody::std_cursor!([S:TextRowPath] NameLookup<S>,[],|s|s.begin_close());
