//! 🪪️ Immutable original source identity is independent of construction execution.
use super::{StepContext,Generation,OperationId,OriginalPayloadSourceIdentity};
/// 🧷️ See `README.md`.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalSourceBinding{operation:OperationId,generation:Generation,payload:OriginalPayloadSourceIdentity,clock:Option<u64>,clock_source:usize,deadline:u64}
impl OriginalSourceBinding{
 /// 👁️ See `README.md`.
 pub fn borrow(context:&StepContext<'_>)->Self{Self{operation:context.operation(),generation:context.generation(),payload:context.payload_authority_identity(),clock:context.original_clock_binding(),clock_source:context.clock_source()as usize,deadline:context.original_absolute_deadline_us()}}
 /// 🔒️ See `README.md`.
 pub fn matches(self,context:&StepContext<'_>)->bool{self==Self::borrow(context)}
 pub fn operation(self)->OperationId{self.operation}
 pub fn generation(self)->Generation{self.generation}
}
