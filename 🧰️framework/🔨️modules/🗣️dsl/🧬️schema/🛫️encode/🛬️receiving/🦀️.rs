//! 🛬️ The projection vocabulary borrows every defining construction cause without a diagnostic mirror.
use super::{FieldProjectionSource,FieldProjectionView as V,projection_path_error};
use semio_framework_value::{ValueError,RetainedCloneProgress,Number};
use semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary;
/// 🔭️ A vaulted allocation refusal stays with its issuer and projects as null.
fn owned(refusal:&Option<semio_framework_value_native_owned::NativeAllocationRefusal>)->Option<&ValueError>{match refusal{Some(semio_framework_value_native_owned::NativeAllocationRefusal::Owned(error))=>Some(error),_=>None}}
#[derive(Clone,Copy)]
enum Node<'a>{Receiving(&'a OriginalReceivingPrimary),Allocation(&'a semio_framework_value_native_owned::NativeAllocationFailure),AllocationProgress(semio_framework_value_native_owned::NativeAllocationProgress),Null,Value(&'a ValueError),Progress(RetainedCloneProgress),Text(&'a str),UInt(usize)}
impl<'a> Node<'a>{
 fn key(self,index:usize)->Result<&'static str,ValueError>{match self{
  Self::Receiving(value)=>[value.primary.as_ref().map(|_|"primary"),value.quote_refusal.as_ref().map(|_|"quoteRefusal"),value.allocation_refusal.as_ref().map(|_|"allocationRefusal"),value.allocation_failure.as_ref().map(|_|"allocationFailure"),value.receipt_refusal.as_ref().map(|_|"receiptRefusal"),value.receipt_completion().map(|_|"receiptCompletion"),Some("retainedProgress")].into_iter().flatten().nth(index).ok_or_else(projection_path_error),
  Self::Allocation(_)=>["primary","admission","forwardedAdmission","scopedAdmission","forwardedQuote","scopedQuote","forwardedPost","scopedPost","forwardedCancel","scopedCancel","receipt","extent","progress"].get(index).copied().ok_or_else(projection_path_error),
  Self::AllocationProgress(_)=>["admittedBytes","actualCapacityBytes","priorAdmittedBytes","requestedBytes","maximumBytes"].get(index).copied().ok_or_else(projection_path_error),
  Self::Value(_)=>["kind","message","retainedProgress"].get(index).copied().ok_or_else(projection_path_error),
  Self::Progress(_)=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(projection_path_error),_=>Err(projection_path_error())
 }}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
  Self::Receiving(value)=>match self.key(index)?{"primary"=>Self::Value(value.primary.as_ref().unwrap()),"quoteRefusal"=>Self::Value(value.quote_refusal.as_ref().unwrap()),"allocationRefusal"=>Self::Value(value.allocation_refusal.as_ref().unwrap()),"allocationFailure"=>Self::Allocation(value.allocation_failure.as_ref().unwrap()),"receiptRefusal"=>Self::Value(value.receipt_refusal.as_ref().unwrap()),"receiptCompletion"=>Self::Value(value.receipt_completion().unwrap()),"retainedProgress"=>Self::Progress(value.progress),_=>return Err(projection_path_error())},
  Self::Allocation(value)=>{if index==12{Self::AllocationProgress(value.progress)}else{let cause=match index{0=>value.primary.as_ref(),1=>value.admission.as_ref(),2=>owned(&value.forwarded_admission),3=>owned(&value.scoped_admission),4=>owned(&value.forwarded_quote),5=>owned(&value.scoped_quote),6=>owned(&value.forwarded_post),7=>owned(&value.scoped_post),8=>owned(&value.forwarded_cancel),9=>owned(&value.scoped_cancel),10=>value.receipt.as_ref(),11=>value.extent.as_ref(),_=>return Err(projection_path_error())};cause.map_or(Self::Null,Self::Value)}},
  Self::AllocationProgress(value)=>match index{0=>Self::UInt(value.admitted_bytes),1=>value.actual_capacity_bytes.map_or(Self::Null,Self::UInt),2=>value.prior_admitted_bytes.map_or(Self::Null,Self::UInt),3=>value.requested_bytes.map_or(Self::Null,Self::UInt),4=>value.maximum_bytes.map_or(Self::Null,Self::UInt),_=>return Err(projection_path_error())},
  Self::Value(value)=>match index{0=>Self::Text(value.kind.as_str()),1=>Self::Text(&value.message),2=>Self::Progress(value.retained_progress()),_=>return Err(projection_path_error())},
  Self::Progress(value)=>Self::UInt(*[value.copied_items,value.copied_bytes,value.retained_capacity_bytes,value.released_bytes].get(index).ok_or_else(projection_path_error)?),_=>return Err(projection_path_error())
 })}
 fn view(self)->V<'a>{match self{Self::Receiving(value)=>V::IntrinsicObject(1+usize::from(value.primary.is_some())+usize::from(value.quote_refusal.is_some())+usize::from(value.allocation_refusal.is_some())+usize::from(value.allocation_failure.is_some())+usize::from(value.receipt_refusal.is_some())+usize::from(value.receipt_completion().is_some())),Self::Allocation(_)=>V::IntrinsicObject(13),Self::AllocationProgress(_)=>V::IntrinsicObject(5),Self::Null=>V::IntrinsicNull,Self::Value(_)=>V::IntrinsicObject(3),Self::Progress(_)=>V::IntrinsicObject(4),Self::Text(value)=>V::IntrinsicText(value),Self::UInt(value)=>V::IntrinsicNumber(Number::UInt(value as u64))}}
}
fn locate<'a>(value:&'a OriginalReceivingPrimary,path:&[usize])->Result<Node<'a>,ValueError>{if value.receipt_completion_pending(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original construction completion remains sealed"))}if value.native_allocation.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native allocation outcome lacks its canonical projection"))}if value.allocation_failure.as_ref().is_some_and(|failure|!failure.progress.request_is_complete()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native allocation request is incomplete"))}let mut node=Node::Receiving(value);for &index in path{node=node.child(index)?}Ok(node)}
impl FieldProjectionSource for OriginalReceivingPrimary{
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{Ok(locate(self,path)?.view())}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{locate(self,path)?.key(index)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
