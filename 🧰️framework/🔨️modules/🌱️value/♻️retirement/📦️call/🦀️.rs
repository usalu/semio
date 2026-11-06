//! 🧳️ Typed native call outputs and independent refusals retain their original custody.
use semio_framework_value_retirement::{ValueError,retirement::{self,RetireOwned}};
/// 🧳️ Retains an owned output or operation error together with an independent context refusal.
#[derive(Debug)]
pub struct NativeCallFailure<T,E>{pub output:Option<T>,pub operation:Option<E>,pub context:Option<ValueError>,pub receipt_refusal:Option<ValueError>}
impl<T:retirement::RetireOwned,E:retirement::RetireOwned> retirement::RetireOwned for NativeCallFailure<T,E>{
 fn retirement_ready(&self)->bool{retirement::RetireOwned::retirement_ready(&self.output)&&retirement::RetireOwned::retirement_ready(&self.operation)}
 type RetirementCursor=<(Option<T>,Option<E>,Option<ValueError>,Option<ValueError>) as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{(self.output,self.operation,self.context,self.receipt_refusal).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{retirement::support::controlled_retirement_supported::<Self>()}
 fn controlled_retirement_supported_in(path:&retirement::RetirementSupport<'_>)->bool{path.supports::<T>()&&path.supports::<E>()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
