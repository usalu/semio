//! 📋️ Primitive sequences compare paid bulk pages; variable children retain one funded original traversal body.
use super::*;
struct SequenceChildBirth<T:RetainedEquality>(std::marker::PhantomData<fn()->T>);
impl<T:RetainedEquality> RetainedEqualityBirth for SequenceChildBirth<T>{type Output=T::Equality;fn work()->Result<u64,ValueError>{Ok(T::CURSOR_BIRTH_WORK)}fn birth()->Self::Output{T::equality_cursor()}}

pub struct RetainedSequenceEquality<T:RetainedEquality>{index:usize,active:Option<Box<T::Equality>>,pending:Option<bool>,result:Option<bool>,inspection:crate::slice_equality::RetainedSliceInspection,closing:bool}
impl<T:RetainedEquality> RetainedSequenceEquality<T>{
 pub fn new()->Self{Self{index:0,active:None,pending:None,result:None,inspection:crate::slice_equality::RetainedSliceInspection::new(),closing:false}}
 fn fixed()->usize{0}
 fn inline_count(&self,remaining:usize,width:usize,body:usize,maximum_work_units:u64)->Result<usize,ValueError>{let unit=T::INLINE_PAIR_WORK.filter(|work|*work>0).ok_or_else(||invalid("typed inline comparison has no fixed work"))?;let affordable=usize::try_from(maximum_work_units.saturating_sub(1)/unit).unwrap_or(usize::MAX);let page=if width==0{1}else{(body.saturating_sub(Self::fixed())/width).min((4096/width).max(1))};Ok(remaining.min(page).min(affordable))}
 pub fn demand(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.result.is_some(){return Err(invalid("typed sequence comparison is closing or settled"))}if !self.inspection.ready(){return self.inspection.inspect_demand()}if maximum_work_units<4{return Ok(Default::default())}
  let(left,right)=self.inspection.lengths()?;let fixed=Self::fixed();let work=maximum_work_units-3;if left!=right||self.index==left{return Ok(scalar(fixed+size_of::<Option<bool>>()*2))}
  if let Some(width)=T::INLINE_PAIR_BYTES{let count=self.inline_count(left-self.index,width,body,work.saturating_sub(2))?;if count==0{return Ok(Default::default())}return Ok(scalar(fixed+size_of::<usize>()*2+size_of::<Option<bool>>()*2+width.checked_mul(count).ok_or_else(||invalid("typed sequence page size overflow"))?))}
  if let Some(active)=self.active.as_ref(){if self.pending.is_some(){if active.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of::<T::Equality>(),..scalar(fixed+size_of::<Option<Box<T::Equality>>>()*2+size_of::<Option<bool>>()*4+size_of::<usize>()*2)})}let selected=active.close_work_demand(body.saturating_sub(fixed))?;if selected.checked_add(1).ok_or_else(||invalid("typed sequence work overflow"))?>work{return Ok(Default::default())}let mut d=active.close_demand(body.saturating_sub(fixed))?;d.copy_bytes+=fixed;d.depth+=1;return Ok(d)}let selected=active.work_demand(body.saturating_sub(fixed),work.saturating_sub(5))?;if selected==0{return Ok(Default::default())}let mut d=active.demand(body.saturating_sub(fixed),work-5)?;d.copy_bytes+=fixed+size_of::<Option<bool>>()*2;d.depth+=1;return Ok(d)}
  Ok(RetirementDemand{copy_bytes:fixed+size_of::<Option<Box<T::Equality>>>()*2+size_of::<T::Equality>()*2,capacity_bytes:size_of::<T::Equality>(),depth:1,..Default::default()})
 }
 pub fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  if !self.inspection.ready(){self.inspection.inspect_demand()?;return Ok(if maximum_work_units>=3{3}else{0})}if maximum_work_units<4{return Ok(0)}let(left,right)=self.inspection.lengths()?;let work=maximum_work_units-3;
  let inner=if left!=right||self.index==left{1}else if let Some(width)=T::INLINE_PAIR_BYTES{let count=self.inline_count(left-self.index,width,body,work.saturating_sub(2))?;if count==0{return Ok(0)}(count as u64).checked_mul(T::INLINE_PAIR_WORK.ok_or_else(||invalid("typed inline work absent"))?).and_then(|v|v.checked_add(3)).ok_or_else(||invalid("typed sequence work overflow"))?}else if let Some(active)=self.active.as_ref(){let selected=if self.pending.is_some(){if active.terminal_is_empty(){3}else{active.close_work_demand(body.saturating_sub(Self::fixed()))?}}else{active.work_demand(body.saturating_sub(Self::fixed()),work.saturating_sub(5))?};if self.pending.is_none()&&selected==0{return Ok(0)}selected.checked_add(if self.pending.is_some(){1}else{5}).ok_or_else(||invalid("typed sequence work overflow"))?}else{equality_box_work::<SequenceChildBirth<T>>()?.checked_add(1).ok_or_else(||invalid("sequence birth work overflow"))?};let total=inner.checked_add(3).ok_or_else(||invalid("typed sequence work overflow"))?;Ok(if total<=maximum_work_units{total}else{0})
 }
 pub fn advance(&mut self,left:&[T],right:&[T],body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let mut work:u64=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if actual.is_some(){return Err(invalid("equality advance needs an empty allocation marker").into())}
  if self.closing{return Err(invalid("typed comparison is closing").into())}if self.result.is_some(){return Ok(RetainedEqualityStep{equal:self.result,progress:Default::default()})}
  if !self.inspection.ready(){let turn=self.inspection.inspect(left,right,maximum_work_units,g);work=turn.work;return turn.result.map(|step|RetainedEqualityStep{equal:None,progress:step.progress()}).map_err(Into::into)}
  if self.work_demand(body,maximum_work_units)?==0{return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}let d=self.demand(body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}
  let checked=self.inspection.verify(left,right,maximum_work_units);work=checked.work;if checked.result?!=Some(true){return Err(invalid("typed sequence source header changed").into())}let(left_len,right_len)=self.inspection.lengths()?;let maximum_work_units=maximum_work_units-3;
  let fixed=Self::fixed();let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if left_len!=right_len{self.result=Some(false);work+=1;}
  else if self.index==left_len{self.result=Some(true);work+=1;}
  else if let Some(width)=T::INLINE_PAIR_BYTES{let count=self.inline_count(left_len-self.index,width,body,maximum_work_units.saturating_sub(2))?;let end=self.index+count;let mut read=0;let mut equal=true;let unit=T::INLINE_PAIR_WORK.ok_or_else(||invalid("typed inline work absent"))?;work+=2;for at in self.index..end{read+=width;work=work.checked_add(unit).ok_or_else(||invalid("typed sequence work overflow"))?;if !T::inline_equal(&left[at],&right[at]).ok_or_else(||invalid("typed inline capability omitted equality"))?{equal=false;work+=1;break;}}self.index=end;p.copied_bytes=fixed+read+size_of::<usize>()*2+size_of::<Option<bool>>()*2;self.result=if !equal{Some(false)}else if self.index==left_len{Some(true)}else{None};}
  else if self.active.is_none(){let turn=equality_box::<SequenceChildBirth<T>>(maximum_work_units-1,g,actual);work+=turn.work;if let Some((child,receipt))=turn.result?{self.active=Some(child);work+=1;p=receipt;p.copied_bytes+=size_of::<Option<Box<T::Equality>>>()*2;let expected=if p.retained_capacity_bytes==0{None}else{Some(p.retained_capacity_bytes)};if *actual!=expected||p.retained_capacity_bytes!=size_of::<T::Equality>(){return Err(crate::OriginalRetirementFailure::from_primary(invalid("sequence cursor allocation marker disagrees")).with_retained_progress(p))}}else{return Ok(RetainedEqualityStep{equal:None,progress:Default::default()})}}
  else if self.pending.is_none(){let turn=self.active.as_mut().ok_or_else(||ValueError::invariant("active absent"))?.advance(&left[self.index],&right[self.index],body.saturating_sub(fixed),maximum_work_units-5,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed-size_of::<Option<bool>>()*2,maximum_depth:g.maximum_depth-1,..g},actual);work+=3+turn.work;let step=turn.result?;p=step.progress;p.copied_bytes+=fixed;if let Some(equal)=step.equal{self.pending=Some(equal);work+=2;p.copied_bytes+=size_of::<Option<bool>>()*2;self.active.as_mut().ok_or_else(||ValueError::invariant("active absent"))?.begin_close();}}
  else if !self.active.as_ref().ok_or_else(||ValueError::invariant("active absent"))?.terminal_is_empty(){let turn=self.active.as_mut().ok_or_else(||ValueError::invariant("active absent"))?.close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});work+=1+turn.work;p=turn.result?.progress();p.copied_bytes+=fixed;}
  else{drop(self.active.take());work+=4;p.released_bytes=size_of::<T::Equality>();let equal=self.pending.take().ok_or_else(||ValueError::invariant("pending absent"))?;self.index+=1;self.result=if !equal{Some(false)}else if self.index==left_len{Some(true)}else{None};}
  Ok(RetainedEqualityStep{equal:self.result,progress:p})
 })();crate::RetainedWorkOutcome{result,work}}

 pub fn begin_close(&mut self){self.closing=true;self.inspection.begin_close();if let Some(active)=self.active.as_mut(){active.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("typed sequence close has not begun"))}if let Some(active)=self.active.as_ref(){if !active.terminal_is_empty(){let mut d=active.close_demand(body.saturating_sub(Self::fixed()))?;d.copy_bytes+=Self::fixed();d.depth+=1;return Ok(d)}return Ok(RetirementDemand{release_bytes:size_of::<T::Equality>(),..scalar(Self::fixed()+size_of::<Option<Box<T::Equality>>>()*2)})}if !self.inspection.is_empty(){return self.inspection.close_demand()}if self.index!=0||self.pending.is_some()||self.result.is_some(){return Ok(scalar(size_of::<usize>()*2+size_of::<Option<bool>>()*4))}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid("typed sequence close has not begun"))}if let Some(active)=self.active.as_ref(){if !active.terminal_is_empty(){return active.close_work_demand(body.saturating_sub(Self::fixed()))}return Ok(1)}if !self.inspection.is_empty(){return Ok(1)}if self.index!=0||self.pending.is_some()||self.result.is_some(){return Ok(3)}Ok(0)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{if !self.closing{return Err(invalid("typed sequence close has not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(active)=self.active.as_mut(){if !active.terminal_is_empty(){let turn=active.close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g});work=turn.work;let mut p=turn.result?.progress();p.copied_bytes+=Self::fixed();return Ok(RetainedCloneStep::Progress(p))}drop(self.active.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
  if !self.inspection.is_empty(){let turn=self.inspection.close_step(1,g);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()))}
  self.index=0;self.pending=None;self.result=None;work=3;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}
 pub fn terminal_is_empty(&self)->bool{self.active.is_none()&&self.inspection.is_empty()&&self.index==0&&self.pending.is_none()&&self.result.is_none()}
}
impl<T:RetainedEquality> Drop for RetainedSequenceEquality<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"typed sequence retains original frontier or header")}}
impl<T:RetainedEquality> RetainedEquality for Vec<T>{
 type Equality=RetainedSequenceEquality<T>;
 const CURSOR_BIRTH_WORK:u64=8;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 const INLINE_PAIR_WORK:Option<u64>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality> RetainedEqualityCursor<Vec<T>> for RetainedSequenceEquality<T>{
 fn demand(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}RetainedSequenceEquality::demand(self,body,work-2)}
 fn work_demand(&self,body:usize,work:u64)->Result<u64,ValueError>{if work<2{return Ok(0)}let selected=RetainedSequenceEquality::work_demand(self,body,work-2)?;if selected==0{return Ok(0)}selected.checked_add(2).ok_or_else(||invalid("typed sequence source view work overflow"))}
 fn advance(&mut self,l:&Vec<T>,r:&Vec<T>,body:usize,work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{if actual.is_some(){return crate::RetainedWorkOutcome{result:Err(invalid("equality advance needs an empty allocation marker").into()),work:0}}let selected=match <Self as RetainedEqualityCursor<Vec<T>>>::work_demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let d=match <Self as RetainedEqualityCursor<Vec<T>>>::demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let turn=RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),body,work-2,g,actual);crate::RetainedWorkOutcome{result:turn.result,work:turn.work+2}}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{RetainedSequenceEquality::close_work_demand(self,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{self.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}
impl<T:RetainedEquality,const N:usize> RetainedEquality for [T;N]{
 type Equality=RetainedSequenceEquality<T>;
 const CURSOR_BIRTH_WORK:u64=8;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 const INLINE_PAIR_WORK:Option<u64>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality,const N:usize> RetainedEqualityCursor<[T;N]> for RetainedSequenceEquality<T>{
 fn demand(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}RetainedSequenceEquality::demand(self,body,work-2)}
 fn work_demand(&self,body:usize,work:u64)->Result<u64,ValueError>{if work<2{return Ok(0)}let selected=RetainedSequenceEquality::work_demand(self,body,work-2)?;if selected==0{return Ok(0)}selected.checked_add(2).ok_or_else(||invalid("typed sequence source view work overflow"))}
 fn advance(&mut self,l:&[T;N],r:&[T;N],body:usize,work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{if actual.is_some(){return crate::RetainedWorkOutcome{result:Err(invalid("equality advance needs an empty allocation marker").into()),work:0}}let selected=match <Self as RetainedEqualityCursor<[T;N]>>::work_demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let d=match <Self as RetainedEqualityCursor<[T;N]>>::demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let turn=RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),body,work-2,g,actual);crate::RetainedWorkOutcome{result:turn.result,work:turn.work+2}}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{RetainedSequenceEquality::close_work_demand(self,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{self.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}

impl<T:RetainedEquality> RetainedEquality for Option<T>{
 type Equality=RetainedSequenceEquality<T>;
 const CURSOR_BIRTH_WORK:u64=8;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 const INLINE_PAIR_WORK:Option<u64>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality> RetainedEqualityCursor<Option<T>> for RetainedSequenceEquality<T>{
 fn demand(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}RetainedSequenceEquality::demand(self,body,work-2)}
 fn work_demand(&self,body:usize,work:u64)->Result<u64,ValueError>{if work<2{return Ok(0)}let selected=RetainedSequenceEquality::work_demand(self,body,work-2)?;if selected==0{return Ok(0)}selected.checked_add(2).ok_or_else(||invalid("typed sequence source view work overflow"))}
 fn advance(&mut self,l:&Option<T>,r:&Option<T>,body:usize,work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{if actual.is_some(){return crate::RetainedWorkOutcome{result:Err(invalid("equality advance needs an empty allocation marker").into()),work:0}}let selected=match <Self as RetainedEqualityCursor<Option<T>>>::work_demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let d=match <Self as RetainedEqualityCursor<Option<T>>>::demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let turn=RetainedSequenceEquality::advance(self,l.as_slice(),r.as_slice(),body,work-2,g,actual);crate::RetainedWorkOutcome{result:turn.result,work:turn.work+2}}
 fn begin_close(&mut self){RetainedSequenceEquality::begin_close(self)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::close_demand(self,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{RetainedSequenceEquality::close_work_demand(self,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{RetainedSequenceEquality::close_step(self,g)}
 fn terminal_is_empty(&self)->bool{RetainedSequenceEquality::terminal_is_empty(self)}
}
impl<T:RetainedEquality> RetainedEquality for Box<T>{
 type Equality=RetainedSequenceEquality<T>;
 const CURSOR_BIRTH_WORK:u64=8;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 const INLINE_PAIR_WORK:Option<u64>=None;
 fn equality_cursor()->Self::Equality{RetainedSequenceEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<T:RetainedEquality> RetainedEqualityCursor<Box<T>> for RetainedSequenceEquality<T>{
 fn demand(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}RetainedSequenceEquality::demand(self,body,work-2)}
 fn work_demand(&self,body:usize,work:u64)->Result<u64,ValueError>{if work<2{return Ok(0)}let selected=RetainedSequenceEquality::work_demand(self,body,work-2)?;if selected==0{return Ok(0)}selected.checked_add(2).ok_or_else(||invalid("typed sequence source view work overflow"))}
 fn advance(&mut self,l:&Box<T>,r:&Box<T>,body:usize,work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{if actual.is_some(){return crate::RetainedWorkOutcome{result:Err(invalid("equality advance needs an empty allocation marker").into()),work:0}}let selected=match <Self as RetainedEqualityCursor<Box<T>>>::work_demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let d=match <Self as RetainedEqualityCursor<Box<T>>>::demand(self,body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let turn=RetainedSequenceEquality::advance(self,std::slice::from_ref(&**l),std::slice::from_ref(&**r),body,work-2,g,actual);crate::RetainedWorkOutcome{result:turn.result,work:turn.work+2}}
 fn begin_close(&mut self){RetainedSequenceEquality::begin_close(self)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedSequenceEquality::close_demand(self,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{RetainedSequenceEquality::close_work_demand(self,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{RetainedSequenceEquality::close_step(self,g)}
 fn terminal_is_empty(&self)->bool{RetainedSequenceEquality::terminal_is_empty(self)}
}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn inline_sequence_respects_neutral_zero_body_vectors(){
  let rows:serde_json::Value=serde_json::from_str(include_str!("../../../📋️list/🟰️equal/🧪️tests/🧫️fixtures/⛽️work.json")).unwrap();
  for row in rows.as_array().unwrap().iter().filter(|row|row["payloadBodyBytes"].as_u64().unwrap()<2){
   let left:Vec<u8>=row["left"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let right:Vec<u8>=row["right"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let mut c=RetainedSequenceEquality::<u8>::new();let body=row["payloadBodyBytes"].as_u64().unwrap()as usize;let work=row["maximumWork"].as_u64().unwrap();let d=c.demand(body,work).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let mut actual=None;assert_eq!(c.advance(&left,&right,body,work,g,&mut actual).work,3);assert!(actual.is_none());assert_eq!(c.demand(body,work).unwrap(),RetirementDemand::default());assert_eq!(c.work_demand(body,work).unwrap(),row["work"].as_u64().unwrap());c.begin_close();while !c.terminal_is_empty(){let d=c.close_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};c.close_step(g).result.unwrap();}
  }
 }
}
