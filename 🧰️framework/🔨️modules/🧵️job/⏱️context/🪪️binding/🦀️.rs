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

impl semio_framework_value_retained_clone::retirement::RetireOwned for OriginalSourceBinding{
 type RetirementCursor=semio_framework_value_retained_clone::retirement::Leaf<Self>;
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value_retained_clone::retirement::leaf(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value_retained_clone::retirement::leaf_birth_bytes::<Self>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{std::mem::size_of::<Self>()}
}
