//! 🔐️ Exposes strong cancellation custody while sealing the original Arc and every Weak capability.
use super::super::CancelNode;
use std::sync::Arc;
pub(crate) struct CancelHandle { inner: Arc<CancelNode> }
impl CancelHandle {
 pub(crate) fn new(value: CancelNode) -> Self { Self { inner: Arc::new(value) } }
 pub(crate) fn frame_bytes() -> usize { semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<CancelNode>() }
 pub(crate) fn return_original(self) -> Option<CancelNode> { Arc::into_inner(self.inner) }
}
impl Clone for CancelHandle { fn clone(&self) -> Self { Self { inner: Arc::clone(&self.inner) } } }
impl std::ops::Deref for CancelHandle { type Target = CancelNode; fn deref(&self) -> &CancelNode { &self.inner } }
