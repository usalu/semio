//! 🔗️ Canonical shared request field retirement retains its live issuer.
use super::*;

/// 🪢️ A request releases its shared alias while the original Store may remain live.
pub struct OriginalRequestLease<T:RetireOwned+Sync>(pub Arc<T>);
impl<T:RetireOwned+Sync> RetireOwned for OriginalRequestLease<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.0))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedControlledRetirement<T>>())}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
