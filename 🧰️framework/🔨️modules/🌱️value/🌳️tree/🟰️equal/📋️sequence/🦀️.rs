//! 📋️ Primitive sequences compare paid bulk pages; variable children retain one funded original traversal body.
use super::*;

pub struct RetainedSequenceEquality<T:RetainedEquality>{index:usize,active:Option<Box<T::Equality>>,pending:Option<bool>,result:Option<bool>,closing:bool}
impl<T:RetainedEquality> RetainedSequenceEquality<T>{
 pub fn new()->Self{Self{index:0,active:None,pending:None,result:None,closing:false}}
 fn fixed()->usize{size_of::<Self>()*2+size_of::<usize>()*4+size_of::<Vec<T>>()*2}
 pub fn demand(&self,left:&[T],right:&[T],body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.result.is_some(){return Err(invalid("typed sequence comparison is closing or settled"));}let fixed=Self::fixed();if left.len()!=right.len()||self.index==left.len(){return Ok(scalar(fixed));}if let Some(width)=T::INLINE_PAIR_BYTES{return Ok(scalar(fixed+width));}if let Some(active)=self.active.as_ref(){if self.pending.is_some(){if active.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of::<T::Equality>(),..scalar(fixed+size_of::<Box<T::Equality>>()*2)});}let mut d=active.close_demand(body.saturating_sub(fixed))?;d.copy_bytes+=fixed;d.depth+=1;return Ok(d);}let mut d=active.demand(&left[self.index],&right[self.index],body.saturating_sub(fixed))?;d.copy_bytes+=fixed;d.depth+=1;return Ok(d);}Ok(RetirementDemand{copy_bytes:fixed+size_of::<T::Equality>()*2+size_of::<Box<T::Equality>>()*2,capacity_bytes:size_of::<T::Equality>(),depth:1,..Default::default()})}
 pub fn advance(&mut self,left:&[T],right:&[T],g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{let d=self.demand(left,right,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()});}let fixed=Self::fixed();let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if left.len()!=right.len(){self.result=Some(false);}else if self.index==left.len(){self.result=Some(true);}else if let Some(width)=T::INLINE_PAIR_BYTES{let count=if width==0{1}else{((g.maximum_copy_bytes-fixed)/width).min(4096/width.max(1)).max(1)}.min(left.len()-self.index);let end=self.index+count;let mut read=0;for at in self.index..end{read+=width;let equal=T::inline_equal(&left[at],&right[at]).ok_or_else(||invalid("typed inline capability omitted equality"))?;if !equal{self.result=Some(false);break;}}self.index=end;p.copied_bytes=fixed+read;if self.result.is_none()&&self.index==left.len(){self.result=Some(true);}}else if self.active.is_none(){self.active=Some(equality_box(T::equality_cursor())?);p.retained_capacity_bytes=size_of::<T::Equality>();}else if self.pending.is_none(){let step=self.active.as_mut().unwrap().advance(&left[self.index],&right[self.index],RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g})?;p=step.progress;p.copied_bytes+=fixed;if let Some(equal)=step.equal{self.pending=Some(equal);self.active.as_mut().unwrap().begin_close();}}else if !self.active.as_ref().unwrap().terminal_is_empty(){p=self.active.as_mut().unwrap().close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g})?.progress();p.copied_bytes+=fixed;}else{drop(self.active.take());p.released_bytes=size_of::<T::Equality>();let equal=self.pending.take().unwrap();self.index+=1;if !equal{self.result=Some(false);}else if self.index==left.len(){self.result=Some(true);}}Ok(RetainedEqualityStep{equal:self.result,progress:p})}
 pub fn begin_close(&mut self){self.closing=true;if let Some(active)=self.active.as_mut(){active.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(active)=self.active.as_ref(){if !active.terminal_is_empty(){let mut d=active.close_demand(body.saturating_sub(Self::fixed()))?;d.copy_bytes+=Self::fixed();d.depth+=1;return Ok(d);}return Ok(RetirementDemand{release_bytes:size_of::<T::Equality>(),..scalar(Self::fixed()+size_of::<Box<T::Equality>>()*2)});}Ok(Default::default())}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(invalid("typed sequence close has not begun"));}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(active)=self.active.as_mut(){if !active.terminal_is_empty(){let mut p=active.close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g})?.progress();p.copied_bytes+=Self::fixed();return Ok(RetainedCloneStep::Progress(p));}drop(self.active.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}));}Ok(RetainedCloneStep::Complete(Default::default()))}
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
 fn advance(&mut self,l:&Vec<T>,r:&Vec<T>,g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),g)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}
impl<T:RetainedEquality,const N:usize> RetainedEquality for [T;N]{
 type Equality=RetainedSequenceEquality<T>;
 const INLINE_PAIR_BYTES:Option<usize>=match T::INLINE_PAIR_BYTES{Some(_)=>Some(size_of::<Self>()*2),None=>None};
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(l:&Self,r:&Self)->Option<bool>{let _=Self::INLINE_PAIR_BYTES?;for at in 0..N{if !T::inline_equal(&l[at],&r[at])?{return Some(false);}}Some(true)}
}
impl<T:RetainedEquality,const N:usize> RetainedEqualityCursor<[T;N]> for RetainedSequenceEquality<T>{
 fn demand(&self,l:&[T;N],r:&[T;N],body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::demand(self,l.as_slice(),r.as_slice(),body)}
 fn advance(&mut self,l:&[T;N],r:&[T;N],g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),g)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}

impl<T:RetainedEquality> RetainedEquality for Option<T>{
 type Equality=RetainedSequenceEquality<T>;
 const INLINE_PAIR_BYTES:Option<usize>=match T::INLINE_PAIR_BYTES{Some(_)=>Some(size_of::<Self>()*2),None=>None};
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(l:&Self,r:&Self)->Option<bool>{let _=Self::INLINE_PAIR_BYTES?;match(l,r){(None,None)=>Some(true),(Some(l),Some(r))=>T::inline_equal(l,r),_=>Some(false)}}
}
impl<T:RetainedEquality> RetainedEqualityCursor<Option<T>> for RetainedSequenceEquality<T>{
 fn demand(&self,l:&Option<T>,r:&Option<T>,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::demand(self,l.as_slice(),r.as_slice(),body)}
 fn advance(&mut self,l:&Option<T>,r:&Option<T>,g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),g)}
 fn begin_close(&mut self){RetainedSequenceEquality::begin_close(self)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::close_demand(self,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{RetainedSequenceEquality::close_step(self,g)}
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
 fn advance(&mut self,l:&Box<T>,r:&Box<T>,g:RetainedCloneGrant)->Result<RetainedEqualityStep,ValueError>{RetainedSequenceEquality::advance(self,std::slice::from_ref(&**l),std::slice::from_ref(&**r),g)}
 fn begin_close(&mut self){RetainedSequenceEquality::begin_close(self)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::close_demand(self,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{RetainedSequenceEquality::close_step(self,g)}
 fn terminal_is_empty(&self)->bool{RetainedSequenceEquality::terminal_is_empty(self)}
}
