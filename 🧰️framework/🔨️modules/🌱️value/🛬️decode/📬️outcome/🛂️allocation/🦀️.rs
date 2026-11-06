//! 🛂️ One original native allocation outcome keeps every physical and admission refusal owner.
use crate::{ValueError,NativeAllocationRequest};
/// 🧳️ Retains operation, extent and independent original allocation settlement refusals.
#[derive(Debug,PartialEq)]
pub struct NativeDecodeAllocationOutcome{pub request:Option<NativeAllocationRequest>,pub admitted:bool,pub primary:Option<ValueError>,pub quote_refusal:Option<crate::NativeAllocationRefusal>,pub extent_refusal:Option<ValueError>,pub settlement_refusal:Option<crate::NativeAllocationRefusal>,pub actual:Option<usize>}
type NativeDecodeAllocationOutcomeParts=(Option<ValueError>,Option<crate::NativeAllocationRefusal>,Option<ValueError>,Option<crate::NativeAllocationRefusal>);
impl crate::retirement::RetireOwned for NativeDecodeAllocationOutcome{
 type RetirementCursor=<NativeDecodeAllocationOutcomeParts as crate::retirement::RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.quote_refusal)&&crate::retirement::RetireOwned::retirement_ready(&self.settlement_refusal)}
 fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed((self.primary,self.quote_refusal,self.extent_refusal,self.settlement_refusal))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}

impl NativeDecodeAllocationOutcome{
 /// 🧳️ Transfers every directional cause into canonical caller-owned failure custody.
 pub fn into_allocation_failure(self,prior:usize)->Option<crate::NativeAllocationFailure>{let Self{request,admitted,primary,quote_refusal,extent_refusal,settlement_refusal,actual}=self;let mut failure=crate::NativeAllocationFailure::default();failure.primary=primary;failure.extent=extent_refusal;failure.forwarded_quote=quote_refusal;if actual.is_some(){failure.forwarded_post=settlement_refusal}else{failure.forwarded_cancel=settlement_refusal}let admitted_bytes=request.map_or(prior,|request|if admitted{request.next_owned_bytes}else{request.owned_bytes});failure.progress=semio_framework_value_control_contract::NativeAllocationProgress{prior_admitted_bytes:request.map(|request|request.owned_bytes),requested_bytes:request.map(|request|request.bytes),maximum_bytes:request.map(|request|request.maximum_bytes),admitted_bytes,actual_capacity_bytes:actual};if failure.is_empty(){None}else{Some(failure)}}
}
