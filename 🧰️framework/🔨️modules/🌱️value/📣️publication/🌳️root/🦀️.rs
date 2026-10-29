//! 🌳️ One original publication decision selects staged typed roots; every displaced root returns to its own caller.
use crate::{FactoryAuthority,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use crate::retained_clone::original_slot::{RetainedOriginalSlot,RetainedOriginalSlotKey};
use std::{sync::Arc,mem::{ManuallyDrop,size_of},ops::{Deref,DerefMut}};
/// 🪟️ The defining publisher owns the decision and its genuine retirement authority.
pub trait RetainedPublicationDecision:Send+Sync{
 fn committed(&self)->bool;
 fn pending(&self)->bool;
 fn into_retirement(self:Arc<Self>)->FactoryAuthority;
}
struct Candidate<T>{root:Option<T>,decision:Option<Arc<dyn RetainedPublicationDecision>>}
/// 🧳️ Staged roots retain original data and decision aliases until funded settlement returns every displaced owner.
pub struct RetainedPublicationRoot<T>{original:Option<RetainedOriginalSlot<T>>,candidate:ManuallyDrop<Option<Box<Candidate<T>>>>}
fn demand<T>(release:bool)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>()*2+size_of::<RetainedOriginalSlot<T>>()*2+size_of::<RetainedPublicationRoot<T>>()+size_of::<FactoryAuthority>(),capacity_bytes:if release{0}else{size_of::<Candidate<T>>()},release_bytes:if release{size_of::<Candidate<T>>()}else{0},depth:1}}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items!=0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl<T> RetainedPublicationRoot<T>{
 /// 🌱️ Captures the existing original root without allocating or cloning its payload.
 pub fn new(original:T)->Self{Self{original:Some(RetainedOriginalSlot::new(original)),candidate:ManuallyDrop::new(None)}}
 /// 📏️ Quotes the separately allocated candidate frame before any original owner moves.
 pub fn stage_demands()->RetirementDemand{demand::<T>(false)}
 /// 📏️ Returns both original payload ownership and the exact decision alias before releasing its candidate frame.
 pub fn settlement_demands(&self)->RetirementDemand{if self.candidate.is_some(){demand::<T>(true)}else{Default::default()}}
 /// 🎟️ Whether the actual original root has no live source loan or staged candidate.
 pub fn accepts_candidate(&self)->bool{self.candidate.is_none()&&self.original.as_ref().is_some_and(|original|!original.has_read())}
 /// 👁️ Reports real staged custody independently of which root readers currently observe.
 pub fn has_staged_root(&self)->bool{self.candidate.is_some()}
 /// 🪪️ Validates the decision by original allocation identity before publication or cancellation.
 pub fn bound_to(&self,decision:&Arc<dyn RetainedPublicationDecision>)->bool{self.candidate.as_ref().and_then(|candidate|candidate.decision.as_ref()).is_some_and(|original|Arc::ptr_eq(original,decision))}
 /// 🪪️ Borrows the actual allocation identity of the retained publication decision without making another alias.
 pub fn decision_identity(&self)->Option<usize>{self.candidate.as_ref().and_then(|candidate|candidate.decision.as_ref()).map(|decision|Arc::as_ptr(decision)as*const()as usize)}
 /// 🎟️ Transfers the candidate and exact decision once; denied grants preserve all source slots unchanged.
 pub fn stage(&mut self,candidate:&mut Option<T>,decision:&mut Option<Arc<dyn RetainedPublicationDecision>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.candidate.is_some()||self.original.is_none()||self.original.as_ref().is_some_and(RetainedOriginalSlot::has_read){return Err(invalid("retained publication root already holds a candidate"))}let Some(view)=decision.as_ref()else{return Err(invalid("retained publication root requires its original decision"))};if !view.pending(){return Err(invalid("retained publication root requires a pending decision"))}if candidate.is_none(){return Err(invalid("retained publication root requires its original candidate"))}let d=Self::stage_demands();if !permits(d,grant){return Ok(Default::default())}*self.candidate=Some(Box::new(Candidate{root:candidate.take(),decision:decision.take()}));Ok(progress(d))
 }
 /// 🤝️ The committed candidate replaces the root; an aborted candidate returns unchanged for original retirement.
 pub fn settle(&mut self,displaced:&mut Option<T>,decision_retirement:&mut Option<FactoryAuthority>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if displaced.is_some()||decision_retirement.is_some(){return Err(invalid("retained publication settlement requires empty original recipient slots"))}let Some(candidate)=self.candidate.as_ref()else{return Ok(Default::default())};let decision=candidate.decision.as_ref().unwrap();if decision.pending(){return Ok(Default::default())}let d=self.settlement_demands();if !permits(d,grant){return Ok(Default::default())}let mut candidate=self.candidate.take().unwrap();if candidate.decision.as_ref().unwrap().committed(){let(previous,_)=self.original.as_mut().unwrap().replace(&mut candidate.root,grant)?.ok_or_else(||invalid("publication root settlement must admit original slot exchange"))?;*displaced=Some(previous);}else{*displaced=candidate.root.take();}*decision_retirement=Some(candidate.decision.take().unwrap().into_retirement());drop(candidate);Ok(progress(d))
 }
 /// 🪪️ Identifies the actual original root slot independently of its staged publication candidate.
 pub fn original_slot_key(&self)->Result<RetainedOriginalSlotKey,ValueError>{if self.candidate.is_some(){return Err(invalid("staged publication root cannot issue another original source"))}Ok(self.original.as_ref().ok_or_else(||invalid("original publication root is retiring"))?.key())}
 /// 📏️ Quotes the source birth of the original root using its existing retained slot protocol.
 pub fn original_read_demands(&self,key:RetainedOriginalSlotKey)->Result<RetirementDemand,ValueError>where T:Send+Sync+'static{self.original.as_ref().ok_or_else(||invalid("original publication root is retiring"))?.demands(key)}
 /// 🎟️ Lends the actual owned root through its supplied defining issuer before any projection is available.
 pub fn advance_original_read(&mut self,key:RetainedOriginalSlotKey,factory:&Arc<dyn crate::ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->Result<crate::RetainedCloneStep,ValueError>where T:Send+Sync+'static{if self.candidate.is_some(){return Err(invalid("staged publication root cannot issue another original source"))}self.original.as_mut().ok_or_else(||invalid("original publication root is retiring"))?.advance_read(key,factory,grant)}
 /// 👁️ Projects only the exact admitted original slot and its current issuance.
 pub fn borrow_original(&self,key:RetainedOriginalSlotKey)->Result<crate::RetainedCloneRef<'_,T>,ValueError>{self.original.as_ref().ok_or_else(||invalid("original publication root is retiring"))?.borrow(key)}
 /// 📏️ Quotes original return and captured source cleanup before root mutation or publication.
 pub fn original_return_demands(&self,key:RetainedOriginalSlotKey,body:usize)->Result<RetirementDemand,ValueError>{self.original.as_ref().ok_or_else(||invalid("original publication root is retiring"))?.return_demands(key,body)}
 /// 🤝️ Returns the same original root and exact defining issuer after all retained projections close.
 pub fn return_original_read(&mut self,key:RetainedOriginalSlotKey,factory:&mut Option<Arc<dyn crate::ArtifactOwnedValueRetirementFactory<T>>>,grant:RetainedCloneGrant)->Result<crate::RetainedCloneStep,ValueError>{self.original.as_mut().ok_or_else(||invalid("original publication root is retiring"))?.return_read(key,factory,grant)}
 /// 📦️ Hands off the final root only after every staged candidate has settled.
 pub fn into_original(self)->Result<T,Self>{if self.candidate.is_some()||self.original.as_ref().is_some_and(RetainedOriginalSlot::has_read){return Err(self)}let mut owner=ManuallyDrop::new(self);Ok(owner.original.take().unwrap().try_into_original().ok().unwrap())}
}
impl<T> Deref for RetainedPublicationRoot<T>{type Target=T;fn deref(&self)->&T{if let Some(candidate)=self.candidate.as_ref(){if candidate.decision.as_ref().unwrap().committed(){return candidate.root.as_ref().unwrap()}}self.original.as_ref().unwrap().get()}}
impl<T> DerefMut for RetainedPublicationRoot<T>{fn deref_mut(&mut self)->&mut T{if self.candidate.as_ref().is_some_and(|candidate|candidate.decision.as_ref().unwrap().committed()){return self.candidate.as_mut().unwrap().root.as_mut().unwrap()}self.original.as_mut().unwrap().get_mut_unissued().expect("retained publication root mutation requires every original read returned")}}
impl<T> Drop for RetainedPublicationRoot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.candidate.is_none(),"retained publication root requires funded original candidate settlement");if self.candidate.is_none(){unsafe{ManuallyDrop::drop(&mut self.candidate)}}}}


impl<T:Default> Default for RetainedPublicationRoot<T>{fn default()->Self{Self::new(T::default())}}
impl<T:Clone> Clone for RetainedPublicationRoot<T>{fn clone(&self)->Self{Self::new((**self).clone())}}
impl<T:std::fmt::Debug> std::fmt::Debug for RetainedPublicationRoot<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Debug::fmt(&**self,f)}}
impl<T:PartialEq> PartialEq for RetainedPublicationRoot<T>{fn eq(&self,other:&Self)->bool{**self==**other}}
impl<T:Eq> Eq for RetainedPublicationRoot<T>{}
impl<T:crate::ToValue> crate::ToValue for RetainedPublicationRoot<T>{fn to_value(&self)->crate::DslValue{(**self).to_value()}}
impl<T:crate::FromValue> crate::FromValue for RetainedPublicationRoot<T>{fn from_value(value:crate::DslValue)->Result<Self,ValueError>{T::from_value(value).map(Self::new)}}
impl<T> From<T> for RetainedPublicationRoot<T>{fn from(value:T)->Self{Self::new(value)}}
impl<T,const N:usize> From<Vec<T>> for RetainedPublicationRoot<semio_framework_value_persistent::list::PagedList<T,N>>{fn from(value:Vec<T>)->Self{Self::new(value.into())}}
#[path="♻️retirement/🦀️.rs"]
mod original_retirement;

impl<'a,T> IntoIterator for &'a RetainedPublicationRoot<T> where &'a T:IntoIterator{type Item=<&'a T as IntoIterator>::Item;type IntoIter=<&'a T as IntoIterator>::IntoIter;fn into_iter(self)->Self::IntoIter{(&**self).into_iter()}}
impl<'a,T> IntoIterator for &'a mut RetainedPublicationRoot<T> where &'a mut T:IntoIterator{type Item=<&'a mut T as IntoIterator>::Item;type IntoIter=<&'a mut T as IntoIterator>::IntoIter;fn into_iter(self)->Self::IntoIter{(&mut **self).into_iter()}}
impl<T:IntoIterator> IntoIterator for RetainedPublicationRoot<T>{type Item=T::Item;type IntoIter=T::IntoIter;fn into_iter(self)->Self::IntoIter{match self.into_original(){Ok(original)=>original.into_iter(),Err(_original)=>panic!("published roots require settlement before owned iteration")}}}
