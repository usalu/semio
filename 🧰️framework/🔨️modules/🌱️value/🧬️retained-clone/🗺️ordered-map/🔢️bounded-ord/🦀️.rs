//! 🔢️ Native bounded comparison retains actual bindings and records original Work before source reads.
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind,RetainedWorkOutcome,RetainedWorkPoll,retained_clone_grant_admits};
use semio_framework_value_clone_source::{RetainedCloneBinding,RetainedCloneRef};
use std::{cmp::Ordering,mem::size_of};
use semio_framework_value_control_contract::{OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value_retirement::{OriginalRetirementFailure,OriginalRetirementCloseLoan};
use semio_framework_value_native_clone_engine::RetainedCursorRelease;
use std::mem::ManuallyDrop;
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct BoundedOrdGrant{pub maximum_items:usize,pub maximum_bytes:usize}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct BoundedOrdProgress{pub compared_items:usize,pub compared_bytes:usize}
impl BoundedOrdProgress{pub fn fits(self,g:BoundedOrdGrant)->bool{self.compared_items<=g.maximum_items&&self.compared_bytes<=g.maximum_bytes}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BoundedOrdStep{Authority(RetainedCloneProgress),Progress(BoundedOrdProgress),Complete{ordering:Ordering,progress:BoundedOrdProgress}}
/// 🧭️ Its fixed first-party primitives must be bounded, immutable, nonallocating, callback-free and nonunwinding.
pub struct NativeBoundedOrdering<T>{scalar:Option<fn(&T,&T)->Ordering>,bytes:Option<for<'a>fn(&'a T)->&'a[u8]>}
impl<T> NativeBoundedOrdering<T>{
 /// 🔢️ The comparison must satisfy the native primitive contract of this witness.
 pub const unsafe fn scalar(compare:fn(&T,&T)->Ordering)->Self{Self{scalar:Some(compare),bytes:None}}
 /// 🔠️ The accessor must expose the actual immutable native byte representation without callbacks or unwind.
 pub const unsafe fn bytes(access:for<'a>fn(&'a T)->&'a[u8])->Self{Self{scalar:None,bytes:Some(access)}}
}
pub trait BoundedOrd:Ord+Send+Sync+Sized+'static{
 type Cursor:BoundedOrdCursor<Self>;
 fn bounded_ord_birth_demand()->crate::RetirementDemand;
 fn bounded_ord_birth_work_demand()->u64;
 fn admit_bounded_ord_cursor_into<'r>(output:&mut Option<Self::Cursor>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_admit_bounded_ord_cursor_into<'r>(output:&mut Option<Self::Cursor>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 const NATIVE_ORDERING:Option<NativeBoundedOrdering<Self>>=None;
}
pub trait BoundedOrdCursor<T:BoundedOrd>:Send+Sized{
 fn compare_demands(&self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,grant:BoundedOrdGrant,body:usize)->Result<crate::RetirementDemand,ValueError>;
 fn compare_work_demand(&self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,grant:BoundedOrdGrant,body:usize)->Result<u64,ValueError>;
 fn compare_into<'r>(&mut self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,grant:BoundedOrdGrant,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_compare_into<'r>(&mut self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,grant:BoundedOrdGrant,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn ordering(&self)->Option<Ordering>;
 fn comparison_progress(&self)->BoundedOrdProgress;
 /// 🧮️ Observes actual logical comparisons, independently of native Work and copied headers.
 fn comparison_totals(&self)->BoundedOrdProgress;
 fn take_ordering_demand(&self)->crate::RetirementDemand;
 fn take_ordering_work_demand(&self)->u64;
 fn take_ordering_into<'r>(&mut self,output:&mut Option<Ordering>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_take_ordering_into<'r>(&mut self,output:&mut Option<Ordering>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn close_started(&self)->bool;
 fn begin_close_demand(&self)->crate::RetirementDemand;
 fn begin_close_work_demand(&self)->u64;
 fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn requires_original_birth(&self)->bool;
 fn requires_original_control(&self)->bool;
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
 fn terminal_release(&self)->Option<RetainedCursorRelease<Self>>{None}
}
fn invalid(message:&'static str)->ValueError{ValueError::invariant(message)}
fn demand(copy_bytes:usize)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes,depth:usize::from(copy_bytes!=0),..Default::default()}}
fn permits(effect:&crate::RetainedAllocationEffect<'_>,d:crate::RetirementDemand,work:u64)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),d)&&effect.remaining_work_units()?>=work)}
fn progress(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:usize::from(copy_bytes!=0),copied_bytes:copy_bytes,..Default::default()}}
pub struct NativeBoundedOrdCursor<T>{
 pub(super) initialized:bool,pub(super) closing:bool,pub(super) complete:Option<Ordering>,
 pub(super) left:ManuallyDrop<Option<RetainedCloneBinding>>,pub(super) right:ManuallyDrop<Option<RetainedCloneBinding>>,
 offset:usize,last:BoundedOrdProgress,total:BoundedOrdProgress,taken:bool,marker:std::marker::PhantomData<fn()->T>,
}
pub type ScalarBoundedOrdCursor<T>=NativeBoundedOrdCursor<T>;
pub type StringBoundedOrdCursor=NativeBoundedOrdCursor<String>;
impl<T> Default for NativeBoundedOrdCursor<T>{fn default()->Self{Self{initialized:false,closing:false,complete:None,left:ManuallyDrop::new(None),right:ManuallyDrop::new(None),offset:0,last:Default::default(),total:Default::default(),taken:false,marker:std::marker::PhantomData}}}
impl<T> NativeBoundedOrdCursor<T>{
 fn binding(&self)->&Option<RetainedCloneBinding>{if self.left.is_some(){&self.left}else{&self.right}}
 fn binding_mut(&mut self)->&mut Option<RetainedCloneBinding>{if self.left.is_some(){&mut self.left}else{&mut self.right}}
 fn empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
 fn birth<'r>(output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{if output.is_some(){return Err(invalid("bounded comparison output is occupied"))}let d=demand(size_of::<Option<Self>>());if !permits(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(invalid("native comparison admission lost original Work"))}*output=Some(Self::default());Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress(d.copy_bytes)))))}
}
impl<T> Drop for NativeBoundedOrdCursor<T>{fn drop(&mut self){let empty=self.empty();assert!(empty||std::thread::panicking(),"bounded comparator requires paid binding close");if empty{unsafe{ManuallyDrop::drop(&mut self.left);ManuallyDrop::drop(&mut self.right)}}}}
impl<T:BoundedOrd> NativeBoundedOrdCursor<T>{
 fn selected(&self,left:RetainedCloneRef<'_,T>,right:RetainedCloneRef<'_,T>,g:BoundedOrdGrant,body:usize)->Result<(crate::RetirementDemand,u64,usize),ValueError>{
  if self.closing{return Err(invalid("bounded comparator is closing"))}left.binding_demand(&self.left)?;right.binding_demand(&self.right)?;
  if self.complete.is_some()||self.taken{return Ok((Default::default(),0,0))}
  if g.maximum_items==0{return Ok((Default::default(),0,0))}
  if let Some(d)=left.binding_demand(&self.left)?{return Ok((d,left.binding_work_demand(&self.left)?,0))}
  if let Some(d)=right.binding_demand(&self.right)?{return Ok((d,right.binding_work_demand(&self.right)?,0))}
  let native=T::NATIVE_ORDERING.ok_or_else(||invalid("type has no native bounded ordering witness"))?;
  let remaining=if let Some(access)=native.bytes{access(left.get()).len().min(access(right.get()).len()).checked_sub(self.offset).ok_or_else(||invalid("native comparison offset exceeds its immutable inputs"))?}else{size_of::<T>()};let reads=if native.scalar.is_some(){if g.maximum_bytes<size_of::<T>()||body<size_of::<T>(){return Ok((Default::default(),0,0))}size_of::<T>()}else{remaining.min(g.maximum_bytes).min(body).min(64)};
  if self.initialized&&reads==0&&remaining!=0{return Ok((Default::default(),0,0))}
  let copy=size_of::<BoundedOrdProgress>()+if self.initialized{size_of::<usize>()+size_of::<Option<Ordering>>()+size_of::<BoundedOrdProgress>()+reads.checked_mul(2).ok_or_else(||invalid("native comparison extent overflow"))?}else{size_of::<bool>()+size_of::<BoundedOrdProgress>()};
  let native_work=if self.initialized{if native.scalar.is_some(){1}else{u64::try_from(reads).map_err(|_|invalid("comparison Work overflow"))?.max(1)}}else{1};
  let work=RetainedCloneRef::<T>::check_binding_work(&self.left).checked_add(RetainedCloneRef::<T>::check_binding_work(&self.right)).and_then(|n|n.checked_add(native_work)).and_then(|n|n.checked_add(1)).ok_or_else(||invalid("comparison Work overflow"))?;
  Ok((demand(copy),work,reads))
 }
}
pub(super) fn flatten_poll<T>(poll:RetainedWorkPoll<T,OriginalRetirementFailure>)->RetainedWorkOutcome<T,ValueError>{
    match poll{
        RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"ordered-map cleanup left its producer reply pending inside an advance")),work},
        RetainedWorkPoll::Complete(turn)=>RetainedWorkOutcome{result:turn.result.map_err(|failure|{let(primary,work,receipt,..)=failure.into_parts();primary.or(work).or(receipt).unwrap_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"ordered-map cleanup failed with an unrepresentable owner"))}),work:turn.work},
    }
}
impl<T:BoundedOrd> BoundedOrdCursor<T> for NativeBoundedOrdCursor<T>{
 fn compare_demands(&self,l:RetainedCloneRef<'_,T>,r:RetainedCloneRef<'_,T>,g:BoundedOrdGrant,b:usize)->Result<crate::RetirementDemand,ValueError>{Ok(self.selected(l,r,g,b)?.0)}
 fn compare_work_demand(&self,l:RetainedCloneRef<'_,T>,r:RetainedCloneRef<'_,T>,g:BoundedOrdGrant,b:usize)->Result<u64,ValueError>{Ok(self.selected(l,r,g,b)?.1)}
 fn compare_into<'r>(&mut self,l:RetainedCloneRef<'_,T>,r:RetainedCloneRef<'_,T>,g:BoundedOrdGrant,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{
  let(d,work,reads)=match self.selected(l,r,g,body){Ok(selected)=>selected,Err(error)=>return Ok(reply.complete(Err(error.into())))};if self.complete.is_some()||self.taken{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if work==0||!permits(effect,d,work)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if self.left.is_none(){let turn=l.bind(&mut self.left,effect);return Ok(reply.complete(turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)))}
  if self.right.is_none(){let turn=r.bind(&mut self.right,effect);return Ok(reply.complete(turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)))}
  self.total.compared_items.checked_add(1).ok_or_else(||invalid("comparison item count overflow"))?;self.total.compared_bytes.checked_add(reads).ok_or_else(||invalid("comparison byte count overflow"))?;
  for result in [l.check_binding_paid(&self.left,effect).result,r.check_binding_paid(&self.right,effect).result]{if result?!=Some(true){return Err(invalid("bounded comparison lost its admitted source checks"))}}
  if !self.initialized{if !effect.performed(2)?{return Err(invalid("comparison initialization lost original Work"))}self.initialized=true;self.last=BoundedOrdProgress{compared_items:1,compared_bytes:0};self.total.compared_items+=1;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress(d.copy_bytes)))))}
  let native=T::NATIVE_ORDERING.ok_or_else(||invalid("native comparison witness disappeared"))?;let mut compared=0usize;let ordering=if let Some(compare)=native.scalar{if !effect.performed(1)?{return Err(invalid("native scalar comparison lost original Work"))}compared=size_of::<T>();Some(compare(l.get(),r.get()))}else{let access=native.bytes.ok_or_else(||invalid("native comparison bytes absent"))?;if !effect.performed(1)?{return Err(invalid("native byte or length comparison lost original Work"))}let(left,right)=(access(l.get()),access(r.get()));let common=left.len().min(right.len());if self.offset==common{Some(left.len().cmp(&right.len()))}else{let mut ordering=None;while compared<reads&&self.offset+compared<common{if compared!=0&&!effect.performed(1)?{return Err(invalid("native byte comparison lost original Work"))}let index=self.offset+compared;compared+=1;let next=left[index].cmp(&right[index]);if next!=Ordering::Equal{ordering=Some(next);break}}ordering}};
  if !effect.performed(1)?{return Err(invalid("comparison totals lost original Work"))}
  self.total.compared_items+=usize::from(compared==0);self.total.compared_bytes+=compared;self.offset=self.offset.checked_add(compared).ok_or_else(||invalid("native comparison offset overflow"))?;self.last=BoundedOrdProgress{compared_items:usize::from(compared==0),compared_bytes:compared};self.complete=ordering;let copied=d.copy_bytes-2*reads+2*compared;Ok(reply.complete(Ok(if ordering.is_some(){RetainedCloneStep::Complete(progress(copied))}else{RetainedCloneStep::Progress(progress(copied))})))
 }
 fn recover_compare_into<'r>(&mut self,_l:RetainedCloneRef<'_,T>,_r:RetainedCloneRef<'_,T>,_g:BoundedOrdGrant,_body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:Reply<'r>)->Result<Complete<'r>,ValueError>{Err(invalid("native comparison has no unrecorded callback"))}
 fn ordering(&self)->Option<Ordering>{self.complete}
 fn comparison_progress(&self)->BoundedOrdProgress{self.last}
 fn comparison_totals(&self)->BoundedOrdProgress{self.total}
 fn take_ordering_demand(&self)->crate::RetirementDemand{demand(2*size_of::<Option<Ordering>>()+size_of::<bool>())}
 fn take_ordering_work_demand(&self)->u64{1}
 fn take_ordering_into<'r>(&mut self,output:&mut Option<Ordering>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{if output.is_some()||self.complete.is_none(){return Err(invalid("native ordering output is not vacant and ready"))}let d=self.take_ordering_demand();if !permits(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(invalid("ordering transfer lost original Work"))}*output=self.complete.take();self.taken=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress(d.copy_bytes)))))}
 fn recover_take_ordering_into<'r>(&mut self,_output:&mut Option<Ordering>,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:Reply<'r>)->Result<Complete<'r>,ValueError>{Err(invalid("native ordering transfer has no unrecorded callback"))}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->crate::RetirementDemand{demand(usize::from(!self.closing)*size_of::<bool>())}
 fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{if self.closing{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let d=self.begin_close_demand();if !permits(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(invalid("native close selection lost original Work"))}self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress(d.copy_bytes)))))}
 fn recover_begin_close_into<'r>(&mut self,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:Reply<'r>)->Result<Complete<'r>,ValueError>{Err(invalid("native close selection has no unrecorded callback"))}
 fn close_demands(&self,b:usize)->Result<crate::RetirementDemand,ValueError>{RetainedCloneBinding::close_demand(self.binding(),b)}
 fn close_work_demand(&self,b:usize)->Result<u64,ValueError>{RetainedCloneBinding::close_work_demand(self.binding(),b)}
 fn close_into<'r>(&mut self,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{if !self.closing{return Err(invalid("comparator must begin paid close"))}RetainedCloneBinding::close_into(self.binding_mut(),b,e,r.progress())}
 fn recover_close_into<'r>(&mut self,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{RetainedCloneBinding::recover_close_into(self.binding_mut(),b,e,r.progress())}
 fn close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{if !self.closing{return Err(invalid("comparator must begin paid close"))}RetainedCloneBinding::close_original_into(self.binding_mut(),o,b,e,r.progress())}
 fn recover_close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{RetainedCloneBinding::recover_close_original_into(self.binding_mut(),o,b,e,r.progress())}
 fn requires_original_birth(&self)->bool{RetainedCloneBinding::requires_original_birth(self.binding())}
 fn requires_original_control(&self)->bool{RetainedCloneBinding::requires_original_control(self.binding())}
 fn close_original_birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->crate::OriginalRetirementPoll{RetainedCloneBinding::close_original_birth_step(self.binding_mut(),o,g)}
 fn terminal_is_empty(&self)->bool{self.empty()}
 fn terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.empty().then(||unsafe{RetainedCursorRelease::native(Self::empty)})}
}
macro_rules! bounded_ord_scalar{($($t:ty),+$(,)?)=>{$(
 impl BoundedOrd for $t{
 type Cursor=NativeBoundedOrdCursor<Self>;
 fn bounded_ord_birth_demand()->crate::RetirementDemand{demand(size_of::<Option<Self::Cursor>>())}
 fn bounded_ord_birth_work_demand()->u64{1}
 fn admit_bounded_ord_cursor_into<'r>(o:&mut Option<Self::Cursor>,e:&mut crate::RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{NativeBoundedOrdCursor::birth(o,e,r)}
 fn recover_admit_bounded_ord_cursor_into<'r>(_o:&mut Option<Self::Cursor>,_e:&mut crate::RetainedAllocationEffect<'_>,_r:Reply<'r>)->Result<Complete<'r>,ValueError>{Err(invalid("native cursor admission has no unrecorded callback"))}
 const NATIVE_ORDERING:Option<NativeBoundedOrdering<Self>>=Some(unsafe{NativeBoundedOrdering::scalar(|l,r|l.cmp(r))});
 }
)+}}
bounded_ord_scalar!(bool,char,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize);
impl BoundedOrd for String{
 type Cursor=StringBoundedOrdCursor;
 fn bounded_ord_birth_demand()->crate::RetirementDemand{demand(size_of::<Option<Self::Cursor>>())}
 fn bounded_ord_birth_work_demand()->u64{1}
 fn admit_bounded_ord_cursor_into<'r>(o:&mut Option<Self::Cursor>,e:&mut crate::RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{NativeBoundedOrdCursor::birth(o,e,r)}
 fn recover_admit_bounded_ord_cursor_into<'r>(_o:&mut Option<Self::Cursor>,_e:&mut crate::RetainedAllocationEffect<'_>,_r:Reply<'r>)->Result<Complete<'r>,ValueError>{Err(invalid("native cursor admission has no unrecorded callback"))}
 const NATIVE_ORDERING:Option<NativeBoundedOrdering<Self>>=Some(unsafe{NativeBoundedOrdering::bytes(String::as_bytes)});
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub(crate) mod actual_work_tests;
