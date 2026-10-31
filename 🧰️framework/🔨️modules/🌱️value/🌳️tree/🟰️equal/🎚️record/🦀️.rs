//! 🎚️ Typed record and enum cursors birth only their currently visited original field.
use super::*;
/// 🧬️ Derivation visits every declared typed field, independently of codec roles and skip annotations.
pub trait RetainedEqualityFields:RetainedEquality{
 fn equality_shape(left:&Self,right:&Self)->Option<usize>;
 fn equality_field_inline_bytes(left:&Self,index:usize)->Option<usize>;
 fn equality_field_inline(left:&Self,right:&Self,index:usize)->Option<bool>;
 fn equality_field_bytes(left:&Self,index:usize)->usize;
 fn equality_field(left:&Self,index:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Box<dyn RetainedEqualityCursor<Self>>,RetainedCloneProgress)>>;
}
/// 🚶️ One funded field body and its genuine child frontier remain live until child-first closure.
pub struct RetainedRecordEquality<T:RetainedEqualityFields>{index:usize,active:Option<Box<dyn RetainedEqualityCursor<T>>>,bytes:usize,pending:Option<bool>,result:Option<bool>,closing:bool}
impl<T:RetainedEqualityFields> RetainedRecordEquality<T>{
 pub fn new()->Self{Self{index:0,active:None,bytes:0,pending:None,result:None,closing:false}}
 fn fixed()->usize{0}
}
impl<T:RetainedEqualityFields> RetainedEqualityCursor<T> for RetainedRecordEquality<T>{
 fn demand(&self,l:&T,r:&T,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.result.is_some(){return Err(invalid("typed record comparison is closing or settled"));}
  let fixed=Self::fixed();let count=T::equality_shape(l,r);
  if count.is_none()||count==Some(self.index){return Ok(scalar(fixed));}
  if let Some(active)=self.active.as_ref(){
   if self.pending.is_some(){if active.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:self.bytes,..scalar(fixed)});}let mut d=active.close_demand(body.saturating_sub(fixed))?;d.copy_bytes+=fixed;d.depth+=1;return Ok(d);}
   let mut d=active.demand(l,r,body.saturating_sub(fixed))?;d.copy_bytes+=fixed;d.depth+=1;return Ok(d);
  }
  if let Some(width)=T::equality_field_inline_bytes(l,self.index){return Ok(scalar(fixed+width));}
  let bytes=T::equality_field_bytes(l,self.index);Ok(RetirementDemand{capacity_bytes:bytes,..scalar(fixed)})
 }
 fn advance(&mut self,l:&T,r:&T,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedEqualityStep>{let mut work=0;let result=(||{
  if self.closing{return Err(invalid("typed comparison is closing"))}if self.result.is_some(){return Ok(RetainedEqualityStep{equal:self.result,progress:Default::default()})}
  let d=self.demand(l,r,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}
  let fixed=Self::fixed();let count=T::equality_shape(l,r);let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if count.is_none(){self.result=Some(false);work=1;}
  else if count==Some(self.index){self.result=Some(true);work=1;}
  else if self.active.is_none()&&T::equality_field_inline_bytes(l,self.index).is_some(){let equal=T::equality_field_inline(l,r,self.index).ok_or_else(||invalid("typed record inline field omitted equality"))?;work=1;self.index+=1;if !equal{self.result=Some(false);}else if count==Some(self.index){self.result=Some(true);}}
  else if self.active.is_none(){let turn=T::equality_field(l,self.index,g);work=turn.work;if let Some((child,receipt))=turn.result?{self.active=Some(child);self.bytes=receipt.retained_capacity_bytes;p=receipt;}else{return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}}
  else if self.pending.is_none(){let turn=self.active.as_mut().unwrap().advance(l,r,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});work=turn.work;let step=turn.result?;p=step.progress;p.copied_bytes+=fixed;if let Some(equal)=step.equal{self.pending=Some(equal);self.active.as_mut().unwrap().begin_close();}}
  else if !self.active.as_ref().unwrap().terminal_is_empty(){let turn=self.active.as_mut().unwrap().close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});work=turn.work;p=turn.result?.progress();p.copied_bytes+=fixed;}
  else{drop(self.active.take());work=1;p.released_bytes=self.bytes;self.bytes=0;let equal=self.pending.take().unwrap();self.index+=1;if !equal{self.result=Some(false);}else if count==Some(self.index){self.result=Some(true);}}
  Ok(RetainedEqualityStep{equal:self.result,progress:p})
 })();crate::RetainedWorkOutcome{result,work}}

 fn begin_close(&mut self){self.closing=true;if let Some(active)=self.active.as_mut(){active.begin_close();}}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("typed record close has not begun"));}if let Some(active)=self.active.as_ref(){if !active.terminal_is_empty(){let mut d=active.close_demand(body.saturating_sub(Self::fixed()))?;d.copy_bytes+=Self::fixed();d.depth+=1;return Ok(d);}return Ok(RetirementDemand{release_bytes:self.bytes,..scalar(Self::fixed())});}Ok(Default::default())}
 fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(active)=self.active.as_mut(){if !active.terminal_is_empty(){let turn=active.close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g});work=turn.work;let mut p=turn.result?.progress();p.copied_bytes+=Self::fixed();return Ok(RetainedCloneStep::Progress(p))}drop(self.active.take());work=1;self.bytes=0;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
  Ok(RetainedCloneStep::Complete(Default::default()))
 })();crate::RetainedWorkOutcome{result,work}}

 fn terminal_is_empty(&self)->bool{self.active.is_none()}
}
impl<T:RetainedEqualityFields> Drop for RetainedRecordEquality<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.active.is_none(),"typed record retains original field frontier");}}
