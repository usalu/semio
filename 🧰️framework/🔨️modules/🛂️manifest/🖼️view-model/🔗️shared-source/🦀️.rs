//! 🔗️ A leased shared host view projects its one actual allocation as the original clone source.
use super::ViewModel;
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReplyComplete,OriginalProjectionReply,OriginalSourceProjection,RetainedAllocationEffect},retirement::arc_custody::ArcLease};
use std::ptr::NonNull;
impl OriginalSourceProjection<ViewModel> for ArcLease<ViewModel>{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,ViewModel>)->Result<OriginalEffectReplyComplete<'reply,NonNull<ViewModel>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared view projection exceeded its original work"))))}Ok(reply.complete(self.0.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"shared view lease is empty"))))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,ViewModel>)->Result<OriginalEffectReplyComplete<'reply,NonNull<ViewModel>,ValueError>,ValueError>{Ok(reply.pending())}
}
