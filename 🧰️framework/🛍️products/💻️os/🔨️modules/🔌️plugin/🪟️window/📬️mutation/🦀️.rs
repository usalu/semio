//! 📬️ Original window mutation payloads carry their captured typed retirement issuer.
use semio_framework_value::{FactoryBoxedValue,retirement::{RetireOwned,RetirementCursor,boxed_owner_birth_bytes}};
use std::any::Any;
pub(crate) trait ErasedWindowMutationValue:Send{
 fn as_any(&self)->&dyn Any;
 fn into_any(self:Box<Self>)->Box<dyn Any+Send>;
 fn clone_value(&self)->Box<dyn ErasedWindowMutationValue>;
 fn retire_payload(self:Box<Self>)->Box<dyn RetirementCursor>;
 fn payload_birth_bytes(&self)->usize;
}
impl<T:Clone+Send+'static> ErasedWindowMutationValue for FactoryBoxedValue<T>{
 fn as_any(&self)->&dyn Any{self.original.as_ref()}
 fn into_any(self:Box<Self>)->Box<dyn Any+Send>{self}
 fn clone_value(&self)->Box<dyn ErasedWindowMutationValue>{Box::new(Self{original:self.original.clone(),factory:self.factory.clone()})}
 fn retire_payload(self:Box<Self>)->Box<dyn RetirementCursor>{self.retirement()}
 fn payload_birth_bytes(&self)->usize{boxed_owner_birth_bytes::<Self>()}
}
impl RetireOwned for Box<dyn ErasedWindowMutationValue>{
 fn retirement(self)->Box<dyn RetirementCursor>{self.retire_payload()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(self.payload_birth_bytes())}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
