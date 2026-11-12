//! 🎟️ Exact diagnostic parameter backing closes independently of logical payload copies.
use crate::FaultParams;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress as P,RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep,original_box::BoxOriginal}};
use std::mem::size_of;

enum Action{Take,Text(bool,usize),Row,Vector(usize),Shell,Clear,Done}
struct ParamsBoxRetirement{source:Option<BoxOriginal<FaultParams>>,values:Vec<(String,String)>,taken:bool}
impl ParamsBoxRetirement{
 fn action(&self)->Action{
  let Some(source)=self.source.as_ref()else{return Action::Done};
  if !self.taken{return Action::Take}
  if let Some((key,value))=self.values.last(){if key.capacity()!=0{return Action::Text(true,key.capacity())}if value.capacity()!=0{return Action::Text(false,value.capacity())}return Action::Row}
  if self.values.capacity()!=0{return Action::Vector(self.values.capacity()*size_of::<(String,String)>())}
  if !source.terminal_is_empty(){Action::Shell}else{Action::Clear}
 }
 fn demand(&self)->RetirementDemand{let action=self.action();RetirementDemand{release_bytes:match action{Action::Text(_,bytes)|Action::Vector(bytes)=>bytes,Action::Shell=>BoxOriginal::<FaultParams>::allocation_bytes(),_=>0},depth:usize::from(!matches!(action,Action::Done)),..Default::default()}}
}
impl RetirementCursor for ParamsBoxRetirement{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(self.demand())}
 semio_framework_value_retirement::native_retirement_reply!();
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(!self.terminal_is_empty()))}
 fn terminal_is_empty(&self)->bool{self.source.is_none()}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.demand().release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl ParamsBoxRetirement{
 fn close_step(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{
  let grant=effect.grant();
  let action=self.action();if matches!(action,Action::Done){return RetirementStep::Complete(0)}
  let demand=self.demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  let progress=match action{
   Action::Take=>{let Some(source)=self.source.as_mut()else{return RetirementStep::Failure(ValueError::invariant("diagnostic retirement source absent").into(),0)};self.values=unsafe{source.take_field(|root|std::ptr::addr_of_mut!((*root).0))};self.taken=true;P{copied_items:1,..Default::default()}},
   Action::Text(key,bytes)=>{let Some(pair)=self.values.last_mut()else{return RetirementStep::Failure(ValueError::invariant("diagnostic retirement row absent").into(),0)};let text=if key{&mut pair.0}else{&mut pair.1};*text=String::new();P{copied_items:1,released_bytes:bytes,..Default::default()}},
   Action::Row=>{self.values.pop();P{copied_items:1,..Default::default()}},
   Action::Vector(bytes)=>{self.values=Vec::new();P{copied_items:1,released_bytes:bytes,..Default::default()}},
   Action::Shell=>{let Some(source)=self.source.as_mut()else{return RetirementStep::Failure(ValueError::invariant("diagnostic retirement source absent").into(),0)};unsafe{source.release_empty(grant)}},
   Action::Clear=>{self.source=None;P{copied_items:1,..Default::default()}},
   Action::Done=>return RetirementStep::Complete(0),
  };RetirementStep::Progress(progress,1)
 }
}
impl Drop for ParamsBoxRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"diagnostic parameter cursor retains original backing")}}
pub(super) fn demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<ParamsBoxRetirement>(),depth:1,..Default::default()}}
/// 🛡️ The funded cursor owns the original Box before its sole Vec field transfers exactly once.
pub(super) fn admit(original:&mut Option<BoxOriginal<FaultParams>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,P)>>{
 if original.is_none(){return RetainedWorkOutcome{result:Ok(None),work:0}}
 if !match original.as_ref(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("original absent").into()),work:0}}.is_initialized(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"diagnostic parameters require their initialized original Box")),work:0}}
 let grant=effect.grant();let demand=demands();if effect.maximum_work_units()==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
 if effect.actual().is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"diagnostic cursor allocation marker is already occupied")),work:0}}
 let work=match semio_framework_value::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}};
 let layout=std::alloc::Layout::new::<ParamsBoxRetirement>();*effect.actual()=Some(0);let raw=unsafe{std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}let Some(pointer)=std::ptr::NonNull::new(raw.cast::<ParamsBoxRetirement>())else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"diagnostic parameter cursor allocation refused")),work}};
 let mut cursor=unsafe{pointer.as_ptr().write(ParamsBoxRetirement{source:None,values:Vec::new(),taken:false});Box::from_raw(pointer.as_ptr())};cursor.source=original.take();
 RetainedWorkOutcome{result:Ok(Some((cursor,P{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()}))),work}
}
