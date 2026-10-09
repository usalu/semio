//! 📬️ Returns actual completion aliases before retiring the original last cell and payload.
use crate::app::{ArtifactApp,ArtifactToolCompletion,ArtifactToolCompletionValue};
use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::{RetireOwned,RetirementCursor,RetirementStep},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::ManuallyDrop,sync::{Arc,Mutex,MutexGuard,TryLockResult}};
struct Cell<A:ArtifactApp>{value:Mutex<Option<ArtifactToolCompletionValue<A>>>,authoring:Mutex<Option<crate::app::OriginalAuthoringEffect>>}
pub(crate) struct CompletionCell<A:ArtifactApp>{inner:ManuallyDrop<Arc<Cell<A>>>,returned:bool}
impl<A:ArtifactApp> CompletionCell<A>{
    pub(crate) fn new()->Self{Self{inner:ManuallyDrop::new(Arc::new(Cell{value:Mutex::new(None),authoring:Mutex::new(None)})),returned:false}}
    pub(crate) fn try_lock(&self)->TryLockResult<MutexGuard<'_,Option<ArtifactToolCompletionValue<A>>>>{self.inner.value.try_lock()}
    pub(crate) fn lock(&self)->std::sync::LockResult<MutexGuard<'_,Option<ArtifactToolCompletionValue<A>>>>{self.inner.value.lock()}
    pub(crate) fn try_lock_authoring(&self)->TryLockResult<MutexGuard<'_,Option<crate::app::OriginalAuthoringEffect>>>{self.inner.authoring.try_lock()}
    pub(crate) fn strong_count(&self)->usize{Arc::strong_count(&self.inner)}
    pub(crate) fn same_cell(&self,other:&Self)->bool{Arc::ptr_eq(&self.inner,&other.inner)}
    fn return_alias(mut self)->Option<Cell<A>>{let inner=unsafe{ManuallyDrop::take(&mut self.inner)};self.returned=true;Arc::into_inner(inner)}
}
impl<A:ArtifactApp> Clone for CompletionCell<A>{fn clone(&self)->Self{Self{inner:ManuallyDrop::new(Arc::clone(&self.inner)),returned:false}}}
impl<A:ArtifactApp> Drop for CompletionCell<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.returned,"original completion alias reached Drop without its explicit retirement return");}}
pub(crate) struct CompletionRetirement<A:ArtifactApp>{source:ManuallyDrop<Option<CompletionCell<A>>>,original:ManuallyDrop<Option<ArtifactToolCompletionValue<A>>>,child:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}
impl<A:ArtifactApp> CompletionRetirement<A>{
    pub(crate) fn new(source:CompletionCell<A>)->Self{Self{source:ManuallyDrop::new(Some(source)),original:ManuallyDrop::new(None),child:ManuallyDrop::new(None)}}
    fn error()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original completion payload has no declared physical retirement authority")}
    fn child_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{let child=self.child.as_mut().unwrap();if child.terminal_is_empty(){let bytes=std::mem::size_of_val(&**child);if grant.maximum_release_bytes<bytes{return RetirementStep::BudgetExhausted;}drop(self.child.take());return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()});}match child.close_step(grant){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}
}
impl<A:ArtifactApp> RetirementCursor for CompletionRetirement<A>{
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
        if self.terminal_is_empty(){return RetirementStep::Complete;}if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}
        if self.source.is_some(){let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Cell<A>>();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return RetirementStep::BudgetExhausted;}
            let original=self.source.take().unwrap().return_alias();let released=if let Some(original)=original{*self.original=original.value.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);drop(original.authoring.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner));bytes}else{0};return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()});
        }
        if self.child.is_some(){return self.child_step(grant);}
        if self.original.is_some(){match A::admit_completion_retirement(&mut self.original,grant){Ok(Some((child,progress)))=>{*self.child=Some(child);return RetirementStep::Progress(progress);},Ok(None)=>return RetirementStep::BudgetExhausted,Err(error)=>return RetirementStep::Failure(error)}}
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.original.is_none()&&self.child.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.child.as_ref().map_or(Ok(0),|child|child.next_copy_byte_demand())}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{if let Some(child)=self.child.as_ref(){return child.next_capacity_byte_demand(copy).ok();}self.original.as_ref().map_or(Some(0),A::completion_retirement_birth_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{if self.source.is_some(){return Some(semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Cell<A>>());}self.child.as_ref().map_or(Some(0),|child|if child.terminal_is_empty(){Some(std::mem::size_of_val(&**child))}else{child.next_release_byte_demand().ok()})}
    fn next_depth_demand(&self)->Result<usize,ValueError>{if let Some(child)=self.child.as_ref(){return child.next_depth_demand();}if let Some(original)=self.original.as_ref(){if A::completion_retirement_birth_bytes(original).is_none(){return Err(Self::error());}}Ok(usize::from(!self.terminal_is_empty()))}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl<A:ArtifactApp> Drop for CompletionRetirement<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original completion reached Drop before its exact cell and payload returned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.child);}}}}
impl<A:ArtifactApp> RetireOwned for ArtifactToolCompletion<A>{
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self.retirement_cursor())}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<CompletionRetirement<A>>())}
    fn controlled_retirement_supported()->bool{true}
}
