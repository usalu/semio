//! 🛫️ Native encoding outcomes retain their original independent causes.
use crate::{NativeAllocationRequest,ValueError};
use std::mem::size_of;
/// 🧾️ Retains a native producer failure independently from its original work-post refusal.
#[derive(Debug)]
pub struct NativeEncodeOperationFailure { pub primary:Option<ValueError>, pub work_refusal:Option<ValueError> }
impl From<ValueError> for NativeEncodeOperationFailure{fn from(error:ValueError)->Self{Self{primary:Some(error),work_refusal:None}}}
impl NativeEncodeOperationFailure{
 /// 🧾️ Preserves the actual physical prefix on each independently retained failure.
 pub fn with_retained_progress(mut self,progress:crate::RetainedCloneProgress)->Self{self.primary=self.primary.map(|error|error.with_retained_progress(progress));self.work_refusal=self.work_refusal.map(|error|error.with_retained_progress(progress));self}
}
type NativeEncodeOperationFailureParts=(Option<ValueError>,Option<ValueError>);
impl crate::retirement::RetireOwned for NativeEncodeOperationFailure{type RetirementCursor=<NativeEncodeOperationFailureParts as crate::retirement::RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed((self.primary,self.work_refusal))}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}fn controlled_retirement_supported()->bool{true}}

/// 🧳️ Owns the operation, extent and both original allocation settlement refusals.
#[derive(Debug,PartialEq)]
pub struct NativeEncodeAllocationOutcome{pub request:Option<NativeAllocationRequest>,pub admitted:bool,pub primary:Option<ValueError>,pub forwarded_quote:Option<crate::NativeAllocationRefusal>,pub scoped_quote:Option<crate::NativeAllocationRefusal>,pub extent_refusal:Option<ValueError>,pub forwarded_refusal:Option<crate::NativeAllocationRefusal>,pub scoped_refusal:Option<crate::NativeAllocationRefusal>,pub actual:Option<usize>}
type NativeEncodeAllocationOutcomeParts=(Option<ValueError>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<ValueError>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>);
impl crate::retirement::RetireOwned for NativeEncodeAllocationOutcome{
 type RetirementCursor=<NativeEncodeAllocationOutcomeParts as crate::retirement::RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{crate::retirement::RetireOwned::retirement_ready(&self.forwarded_quote)&&crate::retirement::RetireOwned::retirement_ready(&self.scoped_quote)&&crate::retirement::RetireOwned::retirement_ready(&self.forwarded_refusal)&&crate::retirement::RetireOwned::retirement_ready(&self.scoped_refusal)}
 fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::RetireOwned::retirement_seed((self.primary,self.forwarded_quote,self.scoped_quote,self.extent_refusal,self.forwarded_refusal,self.scoped_refusal))}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeEncodeAllocationOutcome{
 /// 🧳️ Transfers every directional cause into canonical caller-owned failure custody.
 pub fn into_allocation_failure(self,prior:usize)->Option<crate::NativeAllocationFailure>{let Self{request,admitted,primary,forwarded_quote,scoped_quote,extent_refusal,forwarded_refusal,scoped_refusal,actual}=self;let mut failure=crate::NativeAllocationFailure::default();failure.primary=primary;failure.extent=extent_refusal;failure.forwarded_quote=forwarded_quote;failure.scoped_quote=scoped_quote;if actual.is_some(){failure.forwarded_post=forwarded_refusal;failure.scoped_post=scoped_refusal}else{failure.forwarded_cancel=forwarded_refusal;failure.scoped_cancel=scoped_refusal}let admitted_bytes=request.map_or(prior,|request|if admitted{request.next_owned_bytes}else{request.owned_bytes});failure.progress=crate::NativeAllocationProgress{prior_admitted_bytes:request.map(|request|request.owned_bytes),requested_bytes:request.map(|request|request.bytes),maximum_bytes:request.map(|request|request.maximum_bytes),admitted_bytes,actual_capacity_bytes:actual};if failure.is_empty(){None}else{Some(failure)}}
}

