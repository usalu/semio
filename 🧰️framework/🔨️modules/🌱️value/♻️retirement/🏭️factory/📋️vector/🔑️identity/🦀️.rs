//! 🔑️ Borrowed issuer comparison uses actual data and concrete implementation identity.
use crate::ArtifactOwnedValueRetirementFactory;
/// 🪪️ Data and concrete type are compared without retaining or dereferencing the borrowed issuer.
pub struct FactoryIssuerIdentity<T>{data:usize,implementation:std::any::TypeId,marker:std::marker::PhantomData<fn(T)>}
impl<T> Copy for FactoryIssuerIdentity<T>{}
impl<T> Clone for FactoryIssuerIdentity<T>{fn clone(&self)->Self{*self}}
impl<T:'static> FactoryIssuerIdentity<T>{
 /// 📏️ Prices the original binding and its stored optional header.
 pub const fn capture_copy_bytes()->usize{size_of::<Self>()+size_of::<Option<Self>>()}
 /// 📥️ Captures the live address and the concrete implementation without compiler vtable assumptions.
 pub fn capture(original:&dyn ArtifactOwnedValueRetirementFactory<T>)->Self{Self{data:std::ptr::from_ref(original).cast::<()>()as usize,implementation:std::any::Any::type_id(original),marker:std::marker::PhantomData}}
 /// 🛂️ Requires the same live data and concrete implementation through the original borrowed lifetime.
 pub fn matches(self,original:&dyn ArtifactOwnedValueRetirementFactory<T>)->bool{let current=Self::capture(original);self.data==current.data&&self.implementation==current.implementation}
}
