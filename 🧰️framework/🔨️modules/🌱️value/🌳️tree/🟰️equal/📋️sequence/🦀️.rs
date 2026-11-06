//! 📋️ Primitive sequences compare paid bulk pages; variable children retain one funded original traversal body.
use super::*;

pub struct RetainedSequenceEquality<T:RetainedEquality>{index:usize,active:Option<Box<T::Equality>>,pending:Option<bool>,result:Option<bool>,closing:bool}
impl<T:RetainedEquality> RetainedSequenceEquality<T>{
 pub fn new()->Self{Self{index:0,active:None,pending:None,result:None,closing:false}}
 fn fixed()->usize{0}
 pub fn demand(&self,left:&[T],right:&[T],body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.result.is_some(){return Err(invalid("typed sequence comparison is closing or settled"));}let fixed=Self::fixed();if left.len()!=right.len()||self.index==left.len(){return Ok(scalar(fixed));}if let Some(width)=T::INLINE_PAIR_BYTES{return Ok(scalar(fixed+width));}if let Some(active)=self.active.as_ref(){if self.pending.is_some(){if active.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of::<T::Equality>(),..scalar(fixed)});}let mut d=active.close_demand(body.saturating_sub(fixed))?;d.copy_bytes+=fixed;d.depth+=1;return Ok(d);}let mut d=active.demand(&left[self.index],&right[self.index],body.saturating_sub(fixed))?;d.copy_bytes+=fixed;d.depth+=1;return Ok(d);}Ok(RetirementDemand{copy_bytes:fixed,capacity_bytes:size_of::<T::Equality>(),depth:1,..Default::default()})}
 pub fn advance(&mut self,left:&[T],right:&[T],g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedEqualityStep>{let mut work=0;let result=(||{
  if self.closing{return Err(invalid("typed comparison is closing"))}if self.result.is_some(){return Ok(RetainedEqualityStep{equal:self.result,progress:Default::default()})}
  let d=self.demand(left,right,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}
  let fixed=Self::fixed();let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if left.len()!=right.len(){self.result=Some(false);work=1;}
  else if self.index==left.len(){self.result=Some(true);work=1;}
  else if let Some(width)=T::INLINE_PAIR_BYTES{let count=if width==0{1}else{((g.maximum_copy_bytes-fixed)/width).min(4096/width.max(1)).max(1)}.min(left.len()-self.index);let end=self.index+count;let mut read=0;work=1;for at in self.index..end{read+=width;let equal=T::inline_equal(&left[at],&right[at]).ok_or_else(||invalid("typed inline capability omitted equality"))?;if !equal{self.result=Some(false);break;}}self.index=end;p.copied_bytes=fixed+read;if self.result.is_none()&&self.index==left.len(){self.result=Some(true);}}
  else if self.active.is_none(){let turn=equality_box(T::equality_cursor(),g);work=turn.work;if let Some((child,receipt))=turn.result?{self.active=Some(child);p=receipt;}else{return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}}
  else if self.pending.is_none(){let turn=self.active.as_mut().ok_or_else(||ValueError::invariant("active absent"))?.advance(&left[self.index],&right[self.index],RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});work=turn.work;let step=turn.result?;p=step.progress;p.copied_bytes+=fixed;if let Some(equal)=step.equal{self.pending=Some(equal);self.active.as_mut().ok_or_else(||ValueError::invariant("active absent"))?.begin_close();}}
  else if !self.active.as_ref().ok_or_else(||ValueError::invariant("active absent"))?.terminal_is_empty(){let turn=self.active.as_mut().ok_or_else(||ValueError::invariant("active absent"))?.close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});work=turn.work;p=turn.result?.progress();p.copied_bytes+=fixed;}
  else{drop(self.active.take());work=1;p.released_bytes=size_of::<T::Equality>();let equal=self.pending.take().ok_or_else(||ValueError::invariant("pending absent"))?;self.index+=1;if !equal{self.result=Some(false);}else if self.index==left.len(){self.result=Some(true);}}
  Ok(RetainedEqualityStep{equal:self.result,progress:p})
 })();crate::RetainedWorkOutcome{result,work}}

 pub fn begin_close(&mut self){self.closing=true;if let Some(active)=self.active.as_mut(){active.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(active)=self.active.as_ref(){if !active.terminal_is_empty(){let mut d=active.close_demand(body.saturating_sub(Self::fixed()))?;d.copy_bytes+=Self::fixed();d.depth+=1;return Ok(d);}return Ok(RetirementDemand{release_bytes:size_of::<T::Equality>(),..scalar(Self::fixed())});}Ok(Default::default())}
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let g=effect.grant();let mut work=0;let result=(||{
  if !self.closing{return Err(invalid("typed sequence close has not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(active)=self.active.as_mut(){if !active.terminal_is_empty(){let turn=active.close_step(&mut effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g},effect.maximum_work_units()).expect("retirement child exceeds original effect"));work=turn.work;let mut p=turn.result?.progress();p.copied_bytes+=Self::fixed();return Ok(RetainedCloneStep::Progress(p))}drop(self.active.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
  Ok(RetainedCloneStep::Complete(Default::default()))
 })();crate::RetainedWorkOutcome{result,work}}

 pub fn terminal_is_empty(&self)->bool{self.active.is_none()}
}
impl<T:RetainedEquality> Drop for RetainedSequenceEquality<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.active.is_none(),"typed sequence retains original child frontier");}}
impl<T:RetainedEquality> RetainedEquality for Vec<T>{
 type Equality=RetainedSequenceEquality<T>;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality> RetainedEqualityCursor<Vec<T>> for RetainedSequenceEquality<T>{
 fn demand(&self,l:&Vec<T>,r:&Vec<T>,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::demand(self,l.as_slice(),r.as_slice(),body)}
 fn advance(&mut self,l:&Vec<T>,r:&Vec<T>,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedEqualityStep>{RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),g)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let g=effect.grant();self.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}
impl<T:RetainedEquality,const N:usize> RetainedEquality for [T;N]{
 type Equality=RetainedSequenceEquality<T>;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality,const N:usize> RetainedEqualityCursor<[T;N]> for RetainedSequenceEquality<T>{
 fn demand(&self,l:&[T;N],r:&[T;N],body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::demand(self,l.as_slice(),r.as_slice(),body)}
 fn advance(&mut self,l:&[T;N],r:&[T;N],g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedEqualityStep>{RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),g)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let g=effect.grant();self.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}

impl<T:RetainedEquality> RetainedEquality for Option<T>{
 type Equality=RetainedSequenceEquality<T>;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality> RetainedEqualityCursor<Option<T>> for RetainedSequenceEquality<T>{
 fn demand(&self,l:&Option<T>,r:&Option<T>,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::demand(self,l.as_slice(),r.as_slice(),body)}
 fn advance(&mut self,l:&Option<T>,r:&Option<T>,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedEqualityStep>{RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),g)}
 fn begin_close(&mut self){RetainedSequenceEquality::begin_close(self)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::close_demand(self,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let g=effect.grant();RetainedSequenceEquality::close_step(self,&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
 fn terminal_is_empty(&self)->bool{RetainedSequenceEquality::terminal_is_empty(self)}
}
impl<T:RetainedEquality> RetainedEquality for Box<T>{
 type Equality=RetainedSequenceEquality<T>;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality> RetainedEqualityCursor<Box<T>> for RetainedSequenceEquality<T>{
 fn demand(&self,l:&Box<T>,r:&Box<T>,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::demand(self,std::slice::from_ref(&**l),std::slice::from_ref(&**r),body)}
 fn advance(&mut self,l:&Box<T>,r:&Box<T>,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedEqualityStep>{RetainedSequenceEquality::advance(self,std::slice::from_ref(&**l),std::slice::from_ref(&**r),g)}
 fn begin_close(&mut self){RetainedSequenceEquality::begin_close(self)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::close_demand(self,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let g=effect.grant();RetainedSequenceEquality::close_step(self,&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
 fn terminal_is_empty(&self)->bool{RetainedSequenceEquality::terminal_is_empty(self)}
}
