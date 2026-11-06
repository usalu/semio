//! 📨️ A borrowed original reply writes only into its exact resident result slot.
use super::{ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect reply is already occupied")}
/// 📨️ Deposits the actual producer reply into its resident action.
pub struct OriginalEffectReply<'a,T,E>{slot:&'a mut Option<Result<T,E>>}
/// ✅️ Proves the producer completed or explicitly left the original reply pending.
pub struct OriginalEffectReplyComplete<'a,T,E>{_slot:&'a mut Option<Result<T,E>>}
impl<'a,T,E> OriginalEffectReply<'a,T,E>{
 /// 🫴️ Lends only an empty caller-resident child reply; no action or authority is issued.
 pub fn borrow(slot:&'a mut Option<Result<T,E>>)->Result<Self,ValueError>{if slot.is_some(){return Err(invalid())}Ok(Self{slot})}
 /// 📥️ Records the original result before the producer returns.
 pub fn complete(self,result:Result<T,E>)->OriginalEffectReplyComplete<'a,T,E>{assert!(self.slot.is_none(),"original effect reply occupied");*self.slot=Some(result);OriginalEffectReplyComplete{_slot:self.slot}}
 /// ⏳️ Retains an unresolved original producer reply.
 pub fn pending(self)->OriginalEffectReplyComplete<'a,T,E>{assert!(self.slot.is_none(),"original effect reply occupied");OriginalEffectReplyComplete{_slot:self.slot}}
}
