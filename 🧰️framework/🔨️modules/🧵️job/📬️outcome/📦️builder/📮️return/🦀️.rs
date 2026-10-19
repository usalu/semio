//! 📮️ A sealed builder lends one original page per turn into a pre-reserved return owner in place.
use super::*;
impl RetainedPayloadBuilder{
 /// 📦️ A valid sealed prefix remains in the same header while the last page keeps its original ordinal.
 pub fn supply_last_page<const N:usize>(&mut self,reservation:&mut OriginalJobPageReservation<N>,cx:&mut StepContext<'_>)->Result<Option<OriginalSuppliedJobPage>,OriginalTurnFailure>{
  if !self.sealed||!self.ready||self.staged.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original supplied page requires its sealed builder").into())}
  reservation.publish_last(unsafe{self.original.assume_init_mut()},cx)
 }
}
