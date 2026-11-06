//! 🎟️ SQL producers borrow the same original native work recipient across stages.
use super::{SqliteStageWorkFailure,ValueError,ValueRefusalKind};
pub use semio_framework_value::native_encoding::NativeEncodeWorkRecipient as SqliteSnapshotWorkRecipient;
/// 🪢️ One stage lends the original work loan and cumulative SQL receipt without replacing either.
pub struct SqliteSnapshotStageWork<'a>{pub(super) original:Option<&'a mut dyn SqliteSnapshotWorkRecipient>,pub(super) actual:&'a mut u64,pub(super) baseline:u64,pub(super) failure:Option<&'a mut Option<SqliteStageWorkFailure>>}
impl SqliteSnapshotWorkRecipient for SqliteSnapshotStageWork<'_>{
 fn admits(&mut self,_:u64,additional:u64)->Result<bool,ValueError>{
  let failure=self.failure.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite work requires its original failure slot"))?;if failure.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite work has an unsettled original receipt"))}
  if additional==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite work reservation must be nonzero"))}
  self.actual.checked_add(additional).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"SQLite work reservation overflow"))?;
  let observed=self.actual.checked_sub(self.baseline).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite work baseline exceeds its actual effects"))?;
  self.original.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite stage requires its original work recipient"))?.admits(observed,additional)
 }
 fn committed(&mut self,actual:u64)->Result<(),ValueError>{
  let failure=self.failure.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite work lost its original failure slot"))?;if failure.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite work has an unsettled original receipt"))}let before=*self.actual;let next=before.checked_add(actual);
  if let Some(next)=next{*self.actual=next;}
  let failure=self.failure.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite work lost its original failure slot"))?;*failure=Some(SqliteStageWorkFailure{before,actual,post_completed:false,post_refusal:None});
  let posted=match self.original.as_deref_mut(){Some(original)=>{let result=original.committed(actual);failure.as_mut().unwrap().post_completed=true;result},None=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite stage lost its original work recipient"))};
  match(next,posted){(Some(_),Ok(()))=>{failure.take();Ok(())},(next,posted)=>{failure.as_mut().unwrap().post_refusal=posted.err();Err(ValueError::literal(if next.is_some(){ValueRefusalKind::InvariantViolated}else{ValueRefusalKind::WorkLimit},"SQLite original work receipt remains unsettled"))}}
 }
}
