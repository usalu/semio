//! 🔐️ Seals the original session Arc behind strong-only first-party custody.
use super::super::WorkerJobSessionInner;
use std::sync::Arc;
pub(crate) struct SessionHandle<J>{inner:Arc<WorkerJobSessionInner<J>>}
impl<J> SessionHandle<J>{
 pub(crate) fn new(value:WorkerJobSessionInner<J>)->Self{Self{inner:Arc::new(value)}}
 pub(crate) fn frame_bytes()->usize{semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>()}
 pub(crate) fn is_unique(&self)->bool{Arc::strong_count(&self.inner)==1&&Arc::weak_count(&self.inner)==0}
 pub(crate) fn try_return(self)->Result<WorkerJobSessionInner<J>,Self>{Arc::try_unwrap(self.inner).map_err(|inner|Self{inner})}
}
impl<J> Clone for SessionHandle<J>{fn clone(&self)->Self{Self{inner:Arc::clone(&self.inner)}}}
impl<J> std::ops::Deref for SessionHandle<J>{type Target=WorkerJobSessionInner<J>;fn deref(&self)->&Self::Target{&self.inner}}
