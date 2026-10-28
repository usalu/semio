//! 🗂️ Original ordered window addresses retain a private insertion hole until atomic publication.
use std::borrow::Borrow;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_persistent::list::PagedList;
#[doc(hidden)]
pub struct WindowRegistry<K,V>{entries:PagedList<Option<(K,V)>,{usize::MAX}>,insertion:Option<usize>,insertion_target:usize,insertion_source:Option<(usize,usize)>}
impl<K,V> Drop for WindowRegistry<K,V>{fn drop(&mut self){assert!(std::thread::panicking()||self.entries.terminal_is_empty(),"original Window registry retains every address and page until granted closure");}}
impl<K:Ord+Borrow<str>,V> WindowRegistry<K,V>{
 #[doc(hidden)]
 pub fn new()->Self{Self{entries:PagedList::with_payload_page_bytes(4096.max(size_of::<Option<(K,V)>>())).expect("registry page admits its original inline entry"),insertion:None,insertion_target:0,insertion_source:None}}
 fn position(&self,key:&str)->Result<usize,usize>{
  if self.insertion.is_some(){return self.entries.iter().enumerate().find_map(|(index,entry)|entry.as_ref().filter(|entry|entry.0.borrow()==key).map(|_|index)).ok_or(self.entries.len());}
  let(mut low,mut high)=(0,self.entries.len());while low<high{let middle=low+(high-low)/2;match self.entries.get(middle).unwrap().as_ref().unwrap().0.borrow().cmp(key){std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(middle)}}Err(low)
 }
 fn physical_index(&self,index:usize)->usize{index+usize::from(self.insertion.is_some_and(|hole|index>=hole))}
 fn source(key:&K)->(usize,usize){let text=key.borrow();(text.as_ptr()as usize,text.len())}
 fn clear_insertion(&mut self){self.insertion=None;self.insertion_target=0;self.insertion_source=None;}
 fn insertion_copy()->Result<usize,ValueError>{size_of::<Option<(K,V)>>().checked_mul(2).and_then(|n|n.checked_add(size_of::<Option<usize>>()+size_of::<usize>()+size_of::<Option<(usize,usize)>>())).ok_or_else(||invalid("original Window registry exchange extent overflow"))}
 #[doc(hidden)] pub fn contains_key(&self,key:&str)->bool{self.position(key).is_ok()}
 #[doc(hidden)] pub fn get(&self,key:&str)->Option<&V>{self.position(key).ok().and_then(|index|self.entries.get(index)).and_then(Option::as_ref).map(|entry|&entry.1)}
 #[doc(hidden)] pub fn get_mut(&mut self,key:&str)->Option<&mut V>{let index=self.position(key).ok()?;self.entries.get_mut(index).and_then(Option::as_mut).map(|entry|&mut entry.1)}
 #[doc(hidden)] pub fn len(&self)->usize{self.entries.len()-usize::from(self.insertion.is_some())}
 #[doc(hidden)] pub fn is_empty(&self)->bool{self.len()==0}
 #[doc(hidden)] pub fn terminal_is_empty(&self)->bool{self.insertion.is_none()&&self.entries.terminal_is_empty()}
 #[doc(hidden)] pub fn iter(&self)->impl Iterator<Item=(&K,&V)>{self.entries.iter().filter_map(Option::as_ref).map(|entry|(&entry.0,&entry.1))}
 #[doc(hidden)] pub fn values(&self)->impl Iterator<Item=&V>{self.entries.iter().filter_map(Option::as_ref).map(|entry|&entry.1)}
 #[doc(hidden)] pub fn values_mut(&mut self)->impl Iterator<Item=&mut V>{self.entries.iter_mut().filter_map(Option::as_mut).map(|entry|&mut entry.1)}
 #[doc(hidden)] pub fn get_index(&self,index:usize)->Option<&V>{self.entries.get(self.physical_index(index)).and_then(Option::as_ref).map(|entry|&entry.1)}
 #[doc(hidden)] pub fn get_index_mut(&mut self,index:usize)->Option<&mut V>{let physical=self.physical_index(index);self.entries.get_mut(physical).and_then(Option::as_mut).map(|entry|&mut entry.1)}
 #[doc(hidden)] pub fn last(&self)->Option<(&K,&V)>{if self.is_empty(){return None;}self.entries.get(self.physical_index(self.len()-1)).and_then(Option::as_ref).map(|entry|(&entry.0,&entry.1))}
 /// 🌱️ Cold reference registration is separate from native retained insertion.
 #[doc(hidden)]
 pub fn insert(&mut self,key:K,value:V)->Option<V>{assert!(self.insertion.is_none(),"reference registration waits for original native insertion");let index=match self.position(key.borrow()){Ok(index)=>return Some(std::mem::replace(&mut self.entries.get_mut(index).unwrap().as_mut().unwrap().1,value)),Err(index)=>index};while !self.entries.has_reserved_slot(){let bytes=self.entries.next_allocation_bytes().expect("reference registry page quote");assert!(self.entries.reserve_one(bytes).expect("reference registry page allocation").progressed);}self.entries.push_reserved(Some((key,value))).unwrap_or_else(|_|unreachable!("reserved registry entry"));for current in(index..self.entries.len()-1).rev(){self.entries.swap(current,current+1);}None}
 /// 📏️ Quotes one exact page birth, private hole movement or final original publication.
 pub fn insert_original_demands(&self)->Result<RetirementDemand,ValueError>{if self.insertion.is_some()||self.entries.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:Self::insertion_copy()?,depth:1,..Default::default()});}Ok(RetirementDemand{copy_bytes:self.entries.next_reserve_copy_byte_demand().map_err(page_error)?,capacity_bytes:self.entries.next_allocation_bytes().map_err(page_error)?,depth:self.entries.next_reserve_depth_demand().map_err(page_error)?,..Default::default()})}
 /// 📬️ Keeps both original inputs outside the registry until the final atomic publication turn.
 pub fn insert_original(&mut self,key:&mut Option<K>,value:&mut Option<V>,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
  let original=key.as_ref().ok_or_else(||invalid("original registry insertion address absent"))?;if value.is_none(){return Err(invalid("original registry insertion value absent"));}let source=Self::source(original);if self.insertion_source.is_some_and(|held|held!=source){return Err(invalid("registry insertion requires its original address source"));}if self.insertion.is_none()&&self.contains_key(original.borrow()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original registry insertion address already exists"));}
  let demand=self.insert_original_demands()?;if !fits(grant,demand){return Ok((false,Default::default()));}
  if let Some(hole)=self.insertion{if hole>self.insertion_target{self.entries.swap(hole-1,hole);self.insertion=Some(hole-1);return Ok((false,receipt(demand)));}*self.entries.get_mut(hole).unwrap()=Some((key.take().unwrap(),value.take().unwrap()));self.clear_insertion();return Ok((true,receipt(demand)));}
  if !self.entries.has_reserved_slot(){let progress=self.entries.reserve_one_funded(grant).map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;return Ok((false,progress));}
  self.insertion_target=self.position(original.borrow()).unwrap_err();self.insertion_source=Some(source);self.entries.push_reserved(None).unwrap_or_else(|_|unreachable!("original registry reserved hole"));self.insertion=Some(self.entries.len()-1);Ok((false,receipt(demand)))
 }
 /// 📏️ Cancellation moves only the private vacant slot and preserves every original entry.
 pub fn cancel_insert_original_demands(&self)->Result<RetirementDemand,ValueError>{Ok(if self.insertion.is_some(){RetirementDemand{copy_bytes:Self::insertion_copy()?,depth:1,..Default::default()}}else{Default::default()})}
 /// ↩️ Restores the unchanged registry while both original inputs remain with their caller.
 pub fn cancel_insert_original(&mut self,key:&mut Option<K>,value:&mut Option<V>,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{let Some(hole)=self.insertion else{return Ok((true,Default::default()));};let original=key.as_ref().ok_or_else(||invalid("cancelled insertion lost its original address"))?;if value.is_none()||self.insertion_source!=Some(Self::source(original)){return Err(invalid("cancelled insertion requires its original inputs"));}let demand=self.cancel_insert_original_demands()?;if !fits(grant,demand){return Ok((false,Default::default()));}if hole+1<self.entries.len(){self.entries.swap(hole,hole+1);self.insertion=Some(hole+1);return Ok((false,receipt(demand)));}assert!(self.entries.pop().unwrap().is_none());self.clear_insertion();Ok((true,receipt(demand)))}
 #[doc(hidden)] pub fn pop_demand(&self)->Result<RetirementDemand,ValueError>{if self.insertion.is_some(){return Err(invalid("registry insertion hole requires original cancellation before close"));}if self.is_empty(){return self.backing_demand();}Ok(RetirementDemand{copy_bytes:size_of::<Option<(K,V)>>(),depth:self.entries.next_pop_depth_demand().map_err(page_error)?,..Default::default()})}
 #[doc(hidden)] pub fn pop_original(&mut self,grant:RetainedCloneGrant)->Result<Option<((K,V),RetainedCloneProgress)>,ValueError>{let demand=self.pop_demand()?;if self.is_empty()||!fits(grant,demand){return Ok(None);}Ok(self.entries.pop().map(|entry|(entry.expect("published original registry entry"),receipt(demand))))}
 #[doc(hidden)] pub fn backing_demand(&self)->Result<RetirementDemand,ValueError>{if !self.entries.is_empty(){return Err(invalid("registry backing retains original entries or private hole"));}Ok(RetirementDemand{release_bytes:self.entries.next_release_allocation_bytes().map_err(page_error)?,depth:self.entries.next_release_depth_demand().map_err(page_error)?,..Default::default()})}
 #[doc(hidden)] pub fn close_backing_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=self.backing_demand()?;if !fits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}let step=self.entries.release_empty_page(grant.maximum_release_bytes).map_err(page_error)?;let progress=RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()};Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn receipt(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
fn page_error(error:semio_framework_value_persistent::list::PagedListError)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason)}
#[doc(hidden)] pub fn fits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)}
