//! 🧾️ The open original receiving scope: the issued grant and the construction spend its semantic receipt must still cover.
use super::*;
/// 🧾️ Holds one issued receiving grant and the accepted physical spend inside it.
#[derive(Default)]
pub struct OriginalReceivingScope{issued:Option<RetainedCloneGrant>,spent:RetainedCloneProgress}
impl OriginalReceivingScope{
 /// 🧾️ Returns the grant issued for the open scope.
 pub fn issued(&self)->Option<RetainedCloneGrant>{self.issued}
 /// 📥️ Returns physical spend already accepted inside the open scope.
 pub fn spent(&self)->RetainedCloneProgress{self.spent}
 /// 📥️ Lends the accepted-spend accumulator to the construction recipient of the open scope.
 pub fn spent_mut(&mut self)->&mut RetainedCloneProgress{&mut self.spent}
 /// 🚪️ Opens a scope only when the previous one owes nothing.
 pub fn open(&mut self,grant:RetainedCloneGrant)->Result<(),ValueError>{if self.spent!=RetainedCloneProgress::default(){return Err(ValueError::invariant("original receiving scope still owes its enclosing semantic receipt"))}self.issued=Some(grant);Ok(())}
 /// 📩️ Closes the scope against its semantic receipt and returns the spend the receipt still owes.
 pub fn settle(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{
  let original=self.issued.take().ok_or_else(||ValueError::invariant("semantic receipt lacks its original receiving scope"))?;
  if grant.maximum_items>original.maximum_items||grant.maximum_copy_bytes>original.maximum_copy_bytes||grant.maximum_capacity_bytes>original.maximum_capacity_bytes||grant.maximum_release_bytes>original.maximum_release_bytes||grant.maximum_depth>original.maximum_depth||!progress.fits(grant){return Err(ValueError::invariant("semantic receipt differs from original receiving grant").with_retained_progress(progress))}
  let child=std::mem::take(&mut self.spent);
  let mismatch=||ValueError::invariant("semantic receipt omitted accepted original construction spend").with_retained_progress(progress);
  Ok(RetainedCloneProgress{copied_items:progress.copied_items.checked_sub(child.copied_items).ok_or_else(mismatch)?,copied_bytes:progress.copied_bytes.checked_sub(child.copied_bytes).ok_or_else(mismatch)?,retained_capacity_bytes:progress.retained_capacity_bytes.checked_sub(child.retained_capacity_bytes).ok_or_else(mismatch)?,released_bytes:progress.released_bytes.checked_sub(child.released_bytes).ok_or_else(mismatch)?})
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
