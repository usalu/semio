//! 🔗️ Original history source retains its exact ledger slot, captured issuer and projected forward owners.
use semio_framework_os_vcs::{ArtifactHistoryLedger,ArtifactHistoryReadKey};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,FactoryAuthority,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedOwnedProjection,retained_clone_grant_admits}};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
/// 🧷️ One granted source binds the original ledger and issuer until every retained projection returns.
pub struct OriginalHistorySource<T:Sync>{root:ManuallyDrop<Option<RetainedOwnedProjection<T>>>,key:Option<ArtifactHistoryReadKey>,issuer:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,returned:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,closing_factory:[Option<FactoryAuthority>;2],ready:bool,closing:bool}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
impl<T:Send+Sync+'static> OriginalHistorySource<T>{
 /// 📏️ Captures only the original slot key and exact issuer alias under caller authority.
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 /// 🧷️ Keeps the same indexed source throughout one original edit's operation projections.
 pub fn key(&self)->Option<ArtifactHistoryReadKey>{self.key}
 /// 🎟️ Admits source custody without reading, cloning or reconstructing any history operation.
 pub fn admit(key:ArtifactHistoryReadKey,issuer:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}Ok(Some((Self{root:ManuallyDrop::new(None),key:Some(key),issuer:ManuallyDrop::new(Some(Arc::clone(issuer))),returned:ManuallyDrop::new(None),closing_factory:[None,None],ready:false,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 /// 🧮️ Quotes one original source birth or projection with no grant synthesis.
 pub fn demands(&self,ledger:&ArtifactHistoryLedger<T>)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid("closing history source cannot project another operation"));}let key=self.key.ok_or_else(||invalid("original history key absent"))?;if self.ready{if self.root.is_none(){ledger.retained_read(key)?;}Ok(RetainedOwnedProjection::<T>::constructor_demand())}else{ledger.retained_read_demands(key)}}
 /// 📽️ Projects the actual original field only after independent ledger read births complete.
 pub fn advance<M:Sync>(&mut self,ledger:&mut ArtifactHistoryLedger<T>,output:&mut Option<RetainedOwnedProjection<M>>,projection:usize,project:impl FnOnce(&T)->&M,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if output.is_some(){return Err(invalid("history projection output already occupied"));}let d=self.demands(ledger)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let key=self.key.unwrap();if !self.ready{let step=ledger.advance_retained_read(key,self.issuer.as_ref().ok_or_else(||invalid("original history issuer absent"))?,grant)?;self.ready=matches!(step,RetainedCloneStep::Complete(_));return Ok(RetainedCloneStep::Progress(step.progress()));}if self.root.is_none(){let(root,p)=ledger.retained_read(key)?.retain_projection(grant)?;*self.root=Some(root);return Ok(RetainedCloneStep::Progress(p));}let(root,p)=self.root.as_ref().unwrap().borrow()?.project(projection,project).retain_projection(grant)?;*output=Some(root);Ok(RetainedCloneStep::Complete(p))
 }
 /// 🛑️ Stops original reads before producer cancellation returns its projection aliases.
 pub fn begin_close(&mut self){self.closing=true;}
 /// 📏️ Quotes the exact original return and independently controlled issuer aliases.
 pub fn close_demands(&self,ledger:&ArtifactHistoryLedger<T>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(root)=self.root.as_ref(){return if root.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<T>>>())}else{root.close_demand(body)};}if let Some(key)=self.key{return ledger.retained_read_return_demands(key,body);}if self.returned.is_some()||self.issuer.is_some(){return Ok(header::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>());}if let Some(factory)=self.closing_factory.iter().find_map(Option::as_ref){return factory.demands(body);}Ok(Default::default())}
 /// ↩️ Restores the same slot before closing the aliases of its exact captured issuer.
 pub fn close_step(&mut self,ledger:&mut ArtifactHistoryLedger<T>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demands(ledger,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(root)=self.root.as_mut(){if root.terminal_is_empty(){self.root.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}return root.close_step(grant);}
  if let Some(key)=self.key{let step=match ledger.return_retained_read(key,&mut self.returned,grant){Ok(step)=>step,Err(error)if error.kind==ValueRefusalKind::WorkLimit&&error.retained_progress()==RetainedCloneProgress::default()=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>return Err(error)};if matches!(step,RetainedCloneStep::Complete(_)){self.key=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(factory)=self.returned.take(){self.closing_factory[0]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if let Some(factory)=self.issuer.take(){self.closing_factory[1]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  if let Some(index)=self.closing_factory.iter().position(Option::is_some){let factory=self.closing_factory[index].as_mut().unwrap();let step=factory.step(grant)?;if factory.terminal_is_empty(){self.closing_factory[index]=None;}return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<T:Sync> OriginalHistorySource<T>{
 /// 🫙️ Confirms original slot restoration and physical closure of every source issuer alias.
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.root.is_none()&&self.key.is_none()&&self.issuer.is_none()&&self.returned.is_none()&&self.closing_factory.iter().all(Option::is_none)}
}
impl<T:Sync> Drop for OriginalHistorySource<T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original history source requires original ledger return and funded issuer closure");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
