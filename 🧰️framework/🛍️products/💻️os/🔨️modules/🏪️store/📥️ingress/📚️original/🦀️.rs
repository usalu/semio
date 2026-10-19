//! 📚️ Replay and conflict observations borrow original ledger and incoming edit owners.
use super::{Edit,ReplayEdits,VcsError};
/// 🧷️ Incoming rows shadow ledger rows without duplicating any typed operation or edit.
pub(super) struct OriginalReplayEdits<'a,M,L:ReplayEdits<M>+?Sized>{ledger:&'a L,batch:&'a[Edit<M>]}
impl<'a,M,L:ReplayEdits<M>+?Sized> OriginalReplayEdits<'a,M,L>{
 pub(super) fn new(ledger:&'a L,batch:&'a[Edit<M>])->Self{Self{ledger,batch}}
}
impl<M,L:ReplayEdits<M>+?Sized> ReplayEdits<M> for OriginalReplayEdits<'_,M,L>{
 fn replay_edit(&self,id:&str)->Option<&Edit<M>>{self.batch.iter().find(|edit|edit.id==id).or_else(||self.ledger.replay_edit(id))}
}
/// 🎯️ Conflict observations preserve exact order and refuse every missing original identity.
pub(super) fn original_edits_for_ids<'a,M>(ids:&[String],edits:&'a impl ReplayEdits<M>)->Result<Vec<&'a Edit<M>>,VcsError>{ids.iter().map(|id|edits.replay_edit(id).ok_or_else(||VcsError::UnknownEdit(id.clone()))).collect()}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
