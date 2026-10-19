//! 📮️ A sealed builder lends one original page per turn into a pre-reserved return owner in place.
use super::*;

/// 📦️ Supplies the last page of a sealed builder into a page return reservation.
pub trait RetainedPayloadBuilderSupply{
 /// 📦️ A valid sealed prefix remains in the same header while the last page keeps its original ordinal.
 fn supply_last_page<const N:usize>(&mut self,reservation:&mut OriginalJobPageReservation<N>,cx:&mut StepContext<'_>)->Result<Option<OriginalSuppliedJobPage>,OriginalTurnFailure>;
}
impl RetainedPayloadBuilderSupply for RetainedPayloadBuilder{
 fn supply_last_page<const N:usize>(&mut self,reservation:&mut OriginalJobPageReservation<N>,cx:&mut StepContext<'_>)->Result<Option<OriginalSuppliedJobPage>,OriginalTurnFailure>{
  let Some(original)=self.sealed_original_mut() else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original supplied page requires its sealed builder").into())};
  reservation.publish_last(original,cx)
 }
}
