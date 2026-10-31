//! 🪪️ Immutable original source identity is independent of construction execution.
use super::{StepContext,Generation,OperationId,OriginalPayloadSourceIdentity};
/// 🧷️ A zero-effect witness borrows original source identity without issuing any construction or work authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalSourceBinding{operation:OperationId,generation:Generation,payload:OriginalPayloadSourceIdentity,clock:Option<u64>,clock_source:usize,deadline:u64}
impl OriginalSourceBinding{
 /// 👁️ Captures only identity already supplied by the original caller and its native source ledger.
 pub fn borrow(context:&StepContext<'_>)->Self{Self{operation:context.operation(),generation:context.generation(),payload:context.payload_authority_identity(),clock:context.original_clock_binding(),clock_source:context.clock_source()as usize,deadline:context.original_absolute_deadline_us()}}
 /// 🔒️ Checks the same roots and original clock without deriving or enlarging a turn grant.
 pub fn matches(self,context:&StepContext<'_>)->bool{self==Self::borrow(context)}
 pub fn operation(self)->OperationId{self.operation}
 pub fn generation(self)->Generation{self.generation}
}
