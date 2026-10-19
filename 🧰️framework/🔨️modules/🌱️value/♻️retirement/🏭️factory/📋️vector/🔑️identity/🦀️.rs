//! 🔑️ Exact borrowed issuer comparison includes both actual data and method-table pointers.
use crate::ArtifactOwnedValueRetirementFactory;
/// 🪪️ A lifetime-erased raw pointer is compared only and never dereferenced or retired.
pub struct FactoryIssuerIdentity<T>{pointer:*const(dyn ArtifactOwnedValueRetirementFactory<T>+'static)}
impl<T> Copy for FactoryIssuerIdentity<T>{}
impl<T> Clone for FactoryIssuerIdentity<T>{fn clone(&self)->Self{*self}}
/// 🧷️ No access to the pointee occurs; the caller retains the actual borrowed issuer through closure.
unsafe impl<T> Send for FactoryIssuerIdentity<T>{}
/// 👁️ Comparison reads only the captured pointer header without accessing its authority.
unsafe impl<T> Sync for FactoryIssuerIdentity<T>{}
impl<T> FactoryIssuerIdentity<T>{
 /// 📏️ First context capture prices the original fat-pointer header and its stored optional header.
 pub const fn capture_copy_bytes()->usize{std::mem::size_of::<Self>()+std::mem::size_of::<Option<Self>>()}
 /// 📥️ Erases only the borrowed lifetime of the original fat pointer, preserving its full native metadata.
 pub fn capture(original:&dyn ArtifactOwnedValueRetirementFactory<T>)->Self{let pointer=std::ptr::from_ref(original);Self{pointer:unsafe{std::mem::transmute(pointer)}}}
 /// 🛂️ Equal data addresses with different concrete method tables are different issuers.
 pub fn matches(self,original:&dyn ArtifactOwnedValueRetirementFactory<T>)->bool{std::ptr::eq(self.pointer,Self::capture(original).pointer)}
}
