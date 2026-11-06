//! 🛂️ Original admission and physical allocation failure custody.
use crate::ValueError;

/// 📏️ Separates historical admission from a performed physical allocation.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct NativeAllocationProgress{pub prior_admitted_bytes:Option<usize>,pub requested_bytes:Option<usize>,pub maximum_bytes:Option<usize>,pub admitted_bytes:usize,pub actual_capacity_bytes:Option<usize>}

impl NativeAllocationProgress{
 /// 🧩️ Verifies that an original allocation request is either complete or absent.
 pub fn request_is_complete(&self)->bool{self.prior_admitted_bytes.is_some()==self.requested_bytes.is_some()&&self.requested_bytes.is_some()==self.maximum_bytes.is_some()}
}

/// 🧳️ Retains each independently invoked allocation and receipt refusal inline.
#[derive(Debug,Default,PartialEq)]
pub struct NativeAllocationFailure{pub primary:Option<ValueError>,pub admission:Option<ValueError>,pub forwarded_admission:Option<ValueError>,pub scoped_admission:Option<ValueError>,pub forwarded_quote:Option<ValueError>,pub scoped_quote:Option<ValueError>,pub forwarded_post:Option<ValueError>,pub scoped_post:Option<ValueError>,pub forwarded_cancel:Option<ValueError>,pub scoped_cancel:Option<ValueError>,pub receipt:Option<ValueError>,pub extent:Option<ValueError>,pub progress:NativeAllocationProgress}
impl NativeAllocationFailure{
 /// 🔎️ Reports whether any original refusal is present.
 pub fn is_empty(&self)->bool{self.primary.is_none()&&self.admission.is_none()&&self.forwarded_admission.is_none()&&self.scoped_admission.is_none()&&self.forwarded_quote.is_none()&&self.scoped_quote.is_none()&&self.forwarded_post.is_none()&&self.scoped_post.is_none()&&self.forwarded_cancel.is_none()&&self.scoped_cancel.is_none()&&self.receipt.is_none()&&self.extent.is_none()}
}

impl From<ValueError> for NativeAllocationFailure{fn from(primary:ValueError)->Self{Self{primary:Some(primary),..Default::default()}}}
impl std::fmt::Display for NativeAllocationFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self.primary.as_ref().or(self.admission.as_ref()).or(self.forwarded_admission.as_ref()).or(self.scoped_admission.as_ref()).or(self.forwarded_quote.as_ref()).or(self.scoped_quote.as_ref()).or(self.forwarded_post.as_ref()).or(self.scoped_post.as_ref()).or(self.forwarded_cancel.as_ref()).or(self.scoped_cancel.as_ref()).or(self.receipt.as_ref()).or(self.extent.as_ref()){Some(error)=>std::fmt::Display::fmt(error,f),None=>f.write_str("native allocation failed without a cause")}}}
impl std::error::Error for NativeAllocationFailure{}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
