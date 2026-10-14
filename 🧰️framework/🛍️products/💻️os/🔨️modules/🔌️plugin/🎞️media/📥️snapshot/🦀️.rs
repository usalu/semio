//! 📥️ Original media snapshot and installed retirement supplier remain paired through every denied turn.
use super::{PluginLifecycleStep,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ManuallyDrop};
use semio_framework_job::{OriginalCallerSourceLoan,OriginalTurnFailure,StepContext,InteractiveJobCloseStep};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactorySharedRetirement,ValueRefusalKind};
use std::sync::{Arc,Weak,atomic::{AtomicBool,Ordering}};
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn funds(cx:&StepContext<'_>,demand:RetirementDemand)->bool{let grant=cx.retained_grant();!cx.retirement_should_yield()&&grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes}
fn post(cx:&mut StepContext<'_>,progress:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{cx.consume_retained(progress)?;if progress.copied_items>0{cx.consume_fuel(1)}Ok(())}

pub struct ArtifactSnapshotCloseLease<T>{identity:ManuallyDrop<Option<Weak<T>>>,runtime_retained:ManuallyDrop<Option<Arc<AtomicBool>>>}
impl<T> ArtifactSnapshotCloseLease<T>{
 pub fn can_release(&self,snapshot:&Arc<T>)->bool{self.runtime_retained.as_ref().is_some_and(|flag|flag.load(Ordering::Acquire))&&self.identity.as_ref().is_some_and(|identity|identity.as_ptr()==Arc::as_ptr(snapshot))}
 pub fn terminal_is_empty(&self)->bool{self.identity.is_none()&&self.runtime_retained.is_none()}
 pub fn retirement_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:if self.identity.is_some(){size_of::<Weak<T>>()}else if self.runtime_retained.is_some(){size_of::<Arc<AtomicBool>>()}else{0},depth:usize::from(!self.terminal_is_empty()),..Default::default()}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.retirement_demands();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let flag=self.runtime_retained.as_ref().ok_or_else(||refused("snapshot lease lost its paired original retention"))?;if !flag.load(Ordering::Acquire)||Arc::strong_count(flag)<2{return Err(refused("snapshot lease requires its live original retention witness"))}
  if self.identity.is_some(){self.identity.take();}else{self.runtime_retained.take();}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
}
impl<T> Drop for ArtifactSnapshotCloseLease<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"snapshot lease requires its paid original return")}}

pub(crate) struct ArtifactSnapshotCloseRetention{source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,inner:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,runtime_retained:ManuallyDrop<Option<Arc<AtomicBool>>>}
impl ArtifactSnapshotCloseRetention{
 pub(crate) fn terminal_is_empty(&self)->bool{self.inner.is_none()&&self.runtime_retained.is_none()&&self.source.is_none()}
 pub(crate) fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.source.as_ref().ok_or_else(||refused("published snapshot retention lacks its actual original source"))?.validate_context(cx)}
 pub(crate) fn validate_retirement(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{{let source=self.source.as_ref().ok_or_else(||refused("published snapshot retirement lacks its actual original source"))?;if source.terminal_is_empty(){Ok(())}else{source.validate_retirement_context(cx)}}}
 pub(crate) fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let one=RetirementDemand{depth:1,..Default::default()};if self.inner.is_none()&&self.runtime_retained.is_none(){return Ok(self.source.as_ref().map_or(RetirementDemand::default(),|source|if source.terminal_is_empty(){one}else{source.retirement_demands()}))}let Some(flag)=self.runtime_retained.as_ref()else{return if self.inner.is_none(){Ok(Default::default())}else{Err(refused("snapshot retention lost its paired original flag"))}};
  if Arc::strong_count(flag)!=1||Arc::weak_count(flag)!=0{return Ok(one)}
  if let Some(inner)=self.inner.as_ref(){if inner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(&**inner),..one})}let mut demand=inner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||refused("snapshot supplier close depth overflow"))?;return Ok(demand)}
  Ok(RetirementDemand{release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<AtomicBool>(),..one})
 }
 pub(crate) fn close_step(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,OriginalTurnFailure>{
  if let Some(source)=self.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?}}
  if self.inner.is_none()&&self.runtime_retained.is_none(){if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){if !funds(cx,RetirementDemand{depth:1,..Default::default()}){return Ok(PluginLifecycleStep::Progress(Default::default()))}self.source.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}let parent=cx.original_source_parent()?;return source.close_step(parent,cx).map(|step|PluginLifecycleStep::Progress(step.progress()))}}
  if self.terminal_is_empty(){return Ok(PluginLifecycleStep::Complete(Default::default()))}let flag=self.runtime_retained.as_ref().ok_or_else(||refused("snapshot retention lost original flag"))?;
  if Arc::strong_count(flag)!=1||Arc::weak_count(flag)!=0{return Ok(PluginLifecycleStep::Progress(Default::default()))}let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(PluginLifecycleStep::Progress(Default::default()))}
  if let Some(inner)=self.inner.as_mut(){if inner.terminal_is_empty(){self.inner.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))}return inner.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into)}
  flag.store(false,Ordering::Release);self.runtime_retained.take();let progress=RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()};Ok(if self.source.is_none(){PluginLifecycleStep::Complete(progress)}else{PluginLifecycleStep::Progress(progress)})
 }
}
impl Drop for ArtifactSnapshotCloseRetention{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"snapshot retention requires complete original supplier retirement")}}

pub(crate) struct ArtifactMediaSnapshotSource<T:Send+Sync+'static>{source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,snapshot:ManuallyDrop<Option<Arc<T>>>,factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,flag:ManuallyDrop<Option<Arc<AtomicBool>>>,lease:ManuallyDrop<Option<ArtifactSnapshotCloseLease<T>>>,retention:ManuallyDrop<Option<ArtifactSnapshotCloseRetention>>,partial:ManuallyDrop<Option<FactorySharedRetirement<T>>>}
impl<T:Send+Sync+'static> ArtifactMediaSnapshotSource<T>{
 pub(crate) const fn empty()->Self{Self{source:ManuallyDrop::new(None),snapshot:ManuallyDrop::new(None),factory:ManuallyDrop::new(None),flag:ManuallyDrop::new(None),lease:ManuallyDrop::new(None),retention:ManuallyDrop::new(None),partial:ManuallyDrop::new(None)}}
 pub(crate) fn ready(&self)->bool{self.snapshot.is_some()&&self.lease.is_some()&&self.retention.is_some()}
 pub(crate) fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.source.as_ref().ok_or_else(||refused("media snapshot lacks its admitted original source"))?.validate_context(cx)}
 pub(crate) fn validate_retirement(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{{let source=self.source.as_ref().ok_or_else(||refused("media snapshot retirement lacks its admitted original source"))?;if source.terminal_is_empty(){Ok(())}else{source.validate_retirement_context(cx)}}}
 pub(crate) fn admit_step(&mut self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,capture:impl FnOnce()->Arc<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(source)=self.source.as_ref(){source.validate_context(cx)?}if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  if self.source.is_none(){let parent=cx.original_source_parent()?;*self.source=parent.admit_loan(cx)?;return Ok(false)}
  if self.snapshot.is_none(){let bytes=size_of::<Arc<T>>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>();if !funds(cx,RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}){return Ok(false)}*self.snapshot=Some(capture());*self.factory=Some(Arc::clone(factory));post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(false)}
  if self.flag.is_none(){let bytes=semio_framework_value::shared_retirement_allocation_bytes::<AtomicBool>();if !funds(cx,RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()}){return Ok(false)}*self.flag=Some(Arc::new(AtomicBool::new(true)));post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;return Ok(false)}
  let bytes=FactorySharedRetirement::<T>::constructor_capacity_bytes();let copy=size_of::<Arc<T>>()+size_of::<Weak<T>>()+size_of::<Arc<AtomicBool>>();if !funds(cx,RetirementDemand{copy_bytes:copy,capacity_bytes:bytes,depth:1,..Default::default()}){return Ok(false)}
  let layout=std::alloc::Layout::new::<FactorySharedRetirement<T>>();let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<FactorySharedRetirement<T>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"original snapshot supplier frame allocation refused"))?;
  let snapshot=self.snapshot.as_ref().unwrap();let flag=self.flag.as_ref().unwrap();let owner=unsafe{pointer.as_ptr().write(FactorySharedRetirement::new(Arc::clone(snapshot),self.factory.take().unwrap()));Box::from_raw(pointer.as_ptr())};
  *self.lease=Some(ArtifactSnapshotCloseLease{identity:ManuallyDrop::new(Some(Arc::downgrade(snapshot))),runtime_retained:ManuallyDrop::new(Some(Arc::clone(flag)))});*self.retention=Some(ArtifactSnapshotCloseRetention{source:ManuallyDrop::new(None),inner:ManuallyDrop::new(Some(owner)),runtime_retained:ManuallyDrop::new(self.flag.take())});
  post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:bytes,..Default::default()})?;Ok(false)
 }
 pub(crate) fn handoff(&mut self,snapshot:&mut Option<Arc<T>>,lease:&mut Option<ArtifactSnapshotCloseLease<T>>,retention:&mut Option<ArtifactSnapshotCloseRetention>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if snapshot.is_some()||lease.is_some()||retention.is_some(){return Err(refused("original snapshot receiving slots cannot overwrite another owner").into())}if !self.ready(){return Ok(false)}let bytes=size_of::<Arc<T>>()+size_of::<ArtifactSnapshotCloseLease<T>>()+size_of::<ArtifactSnapshotCloseRetention>()+size_of::<OriginalCallerSourceLoan>();if !funds(cx,RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}){return Ok(false)}*self.retention.as_mut().unwrap().source=self.source.take();*snapshot=self.snapshot.take();*lease=self.lease.take();*retention=self.retention.take();post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(true)
 }
 pub(crate) fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let one=RetirementDemand{depth:1,..Default::default()};if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<T>>()+if self.retention.is_none(){size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>()}else{0},..one})}
  if let Some(lease)=self.lease.as_ref(){return Ok(if lease.terminal_is_empty(){one}else{lease.retirement_demands()})}if let Some(retention)=self.retention.as_ref(){return if retention.terminal_is_empty(){Ok(one)}else{retention.retirement_demands(body)}}
  if let Some(partial)=self.partial.as_ref(){return if partial.terminal_is_empty(){Ok(one)}else{partial.next_demand(body)}}if self.flag.is_some(){return Ok(RetirementDemand{release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<AtomicBool>(),..one})}
  if self.factory.is_some(){return Err(refused("original snapshot supplier cannot outlive its paired snapshot admission"))}if let Some(source)=self.source.as_ref(){return Ok(if source.terminal_is_empty(){one}else{source.retirement_demands()})}Ok(Default::default())
 }
 pub(crate) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}if let Some(source)=self.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?}}
  let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;if !funds(cx,demand){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let grant=cx.retained_grant();
  let progress=if self.snapshot.is_some(){if self.retention.is_some(){if !self.lease.as_ref().is_some_and(|lease|lease.can_release(self.snapshot.as_ref().unwrap())){return Err(refused("prepared snapshot lost original paired retention").into())}self.snapshot.take();}else{*self.partial=Some(FactorySharedRetirement::new(self.snapshot.take().unwrap(),self.factory.take().ok_or_else(||refused("prepared snapshot lost installed supplier"))?));}RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
  else if let Some(lease)=self.lease.as_mut(){if lease.terminal_is_empty(){self.lease.take();RetainedCloneProgress{copied_items:1,..Default::default()}}else{lease.close_step(grant)?.progress()}}
  else if let Some(retention)=self.retention.as_mut(){if retention.terminal_is_empty(){self.retention.take();RetainedCloneProgress{copied_items:1,..Default::default()}}else{retention.close_step(cx,grant)?.progress()}}
  else if let Some(partial)=self.partial.as_mut(){if partial.terminal_is_empty(){self.partial.take();RetainedCloneProgress{copied_items:1,..Default::default()}}else{partial.close_step(grant)?.progress()}}
  else if let Some(flag)=self.flag.as_ref(){if Arc::strong_count(flag)!=1||Arc::weak_count(flag)!=0{return Err(refused("partial snapshot flag acquired an unissued alias").into())}self.flag.take();RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}}
  else if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();RetainedCloneProgress{copied_items:1,..Default::default()}}else{let parent=cx.original_source_parent()?;return source.close_step(parent,cx).map(|step|match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}}
  else{return Err(refused("original snapshot close retains an uncovered supplier").into())};post(cx,progress)?;Ok(InteractiveJobCloseStep::Pending{progress})
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.snapshot.is_none()&&self.factory.is_none()&&self.flag.is_none()&&self.lease.is_none()&&self.retention.is_none()&&self.partial.is_none()}
}
impl<T:Send+Sync+'static> Drop for ArtifactMediaSnapshotSource<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"media snapshot admission retains original source and supplier until paid close")}}
