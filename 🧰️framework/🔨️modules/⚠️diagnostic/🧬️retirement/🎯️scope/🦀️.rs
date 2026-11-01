//! 🎟️ Scope fields leave the exact original Box on independently paid metadata and release events.
use crate::FaultScope;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress as P,RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::{RetirementCursor,RetirementStep,original_box::BoxOriginal}};
use std::mem::size_of;
struct ScopeBoxRetirement{source:Option<BoxOriginal<FaultScope>>,ordinal:usize,text:Option<String>}
impl ScopeBoxRetirement{
 fn demand(&self)->RetirementDemand{RetirementDemand{release_bytes:self.text.as_ref().map_or_else(||if self.ordinal==5&&self.source.as_ref().is_some_and(|source|!source.terminal_is_empty()){BoxOriginal::<FaultScope>::allocation_bytes()}else{0},String::capacity),depth:usize::from(self.source.is_some()),..Default::default()}}
}
impl RetirementCursor for ScopeBoxRetirement{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(self.demand())}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if self.source.is_none(){return RetirementStep::Complete(0)}
  let demand=self.demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  let progress=if let Some(text)=self.text.as_mut(){if text.capacity()!=0{*text=String::new();P{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}}else{self.text=None;self.ordinal+=1;P{copied_items:1,..Default::default()}}}
  else if self.ordinal<5{
   let Some(source)=self.source.as_mut()else{return RetirementStep::Failure(ValueError::invariant("diagnostic retirement source absent"),0)};self.text=unsafe{match self.ordinal{0=>source.take_field(|root|std::ptr::addr_of_mut!((*root).plugin_id)),1=>source.take_field(|root|std::ptr::addr_of_mut!((*root).app_id)),2=>source.take_field(|root|std::ptr::addr_of_mut!((*root).instance_id)),3=>source.take_field(|root|std::ptr::addr_of_mut!((*root).module)),4=>source.take_field(|root|std::ptr::addr_of_mut!((*root).body_key)),_=>return RetirementStep::Failure(ValueError::invariant("diagnostic scope ordinal out of range"),0)}};if self.text.is_none(){self.ordinal+=1}P{copied_items:1,..Default::default()}
  }else if let Some(source)=self.source.as_mut().filter(|source|!source.terminal_is_empty()){unsafe{source.release_empty(grant)}}else{self.source=None;P{copied_items:1,..Default::default()}};
  RetirementStep::Progress(progress,1)
 }
 fn terminal_is_empty(&self)->bool{self.source.is_none()}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.demand().release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for ScopeBoxRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"diagnostic scope cursor retains its original allocation")}}
pub(super) fn demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<ScopeBoxRetirement>(),depth:1,..Default::default()}}
/// 🛡️ A fallible exact cursor owns the Box before transferring each distinct initialized field once.
pub(super) fn admit(original:&mut Option<BoxOriginal<FaultScope>>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,P)>>{
 if original.is_none(){return RetainedWorkOutcome{result:Ok(None),work:0}}
 if !match original.as_ref(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("original absent").into()),work:0}}.is_initialized(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"diagnostic scope requires its initialized original Box")),work:0}}
 let demand=demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
 let layout=std::alloc::Layout::new::<ScopeBoxRetirement>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<ScopeBoxRetirement>())else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"diagnostic scope cursor allocation refused")),work:1}};
 let mut cursor=unsafe{pointer.as_ptr().write(ScopeBoxRetirement{source:None,ordinal:0,text:None});Box::from_raw(pointer.as_ptr())};cursor.source=original.take();
 RetainedWorkOutcome{result:Ok(Some((cursor,P{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()}))),work:1}
}
