//! 🌱️ Intrinsic values retain concrete inline retirement seeds.
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
#[doc(hidden)]
pub enum ValueRetirement{Null(<() as RetireOwned>::RetirementCursor),Bool(<bool as RetireOwned>::RetirementCursor),UInt(<u64 as RetireOwned>::RetirementCursor),Int(<i64 as RetireOwned>::RetirementCursor),Float(<f64 as RetireOwned>::RetirementCursor),String(<String as RetireOwned>::RetirementCursor),Bytes(<Vec<u8> as RetireOwned>::RetirementCursor),Array(<Vec<crate::DslValue> as RetireOwned>::RetirementCursor),Object(<Vec<(String,crate::DslValue)> as RetireOwned>::RetirementCursor)}
impl ValueRetirement{fn active(&self)->&dyn RetirementCursor{match self{Self::Null(cursor)=>cursor,Self::Bool(cursor)=>cursor,Self::UInt(cursor)=>cursor,Self::Int(cursor)=>cursor,Self::Float(cursor)=>cursor,Self::String(cursor)=>cursor,Self::Bytes(cursor)=>cursor,Self::Array(cursor)=>cursor,Self::Object(cursor)=>cursor}}fn active_mut(&mut self)->&mut dyn RetirementCursor{match self{Self::Null(cursor)=>cursor,Self::Bool(cursor)=>cursor,Self::UInt(cursor)=>cursor,Self::Int(cursor)=>cursor,Self::Float(cursor)=>cursor,Self::String(cursor)=>cursor,Self::Bytes(cursor)=>cursor,Self::Array(cursor)=>cursor,Self::Object(cursor)=>cursor}}}

type Reply<'r>=semio_framework_value_control_contract::OriginalEffectReply<'r,RetirementStep,semio_framework_value_retirement::OriginalRetirementFailure>;
type Complete<'r>=semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value_retirement::OriginalRetirementFailure>;

impl RetirementCursor for ValueRetirement{
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,crate::ValueError>{self.active_mut().close_into(body,effect,reply)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,crate::ValueError>{self.active_mut().recover_close_into(body,effect,reply)}
 fn close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,crate::ValueError>{self.active_mut().close_original_into(original,body,effect,reply)}
 fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,crate::ValueError>{self.active_mut().recover_close_original_into(original,body,effect,reply)}
 fn requires_original_birth(&self)->bool{self.active().requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,grant:crate::RetainedCloneGrant)->RetirementStep{self.active_mut().close_original_birth_step(original,grant)}
 fn requires_original_control(&self)->bool{self.active().requires_original_control()}

 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.active().next_demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.active().next_computational_work(body)}
 
 fn terminal_is_empty(&self)->bool{self.active().terminal_is_empty()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{self.active().next_depth_demand()}
 fn next_close_byte_demand(&self)->Option<usize>{self.active().next_close_byte_demand()}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{self.active().next_work_byte_demand()}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{self.active().next_selected_work_byte_demand(body)}
 fn allows_admitted_narrow_work(&self)->bool{self.active().allows_admitted_narrow_work()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.active().next_birth_bytes(body)}
}


impl RetireOwned for crate::DslValue{type RetirementCursor=ValueRetirement;fn retirement_seed(self)->Self::RetirementCursor{match self{crate::DslValue::Null=>ValueRetirement::Null(<() as RetireOwned>::retirement_seed(())),crate::DslValue::Bool(value)=>ValueRetirement::Bool(<bool as RetireOwned>::retirement_seed(value)),crate::DslValue::Number(crate::Number::UInt(value))=>ValueRetirement::UInt(<u64 as RetireOwned>::retirement_seed(value)),crate::DslValue::Number(crate::Number::Int(value))=>ValueRetirement::Int(<i64 as RetireOwned>::retirement_seed(value)),crate::DslValue::Number(crate::Number::Float(value))=>ValueRetirement::Float(<f64 as RetireOwned>::retirement_seed(value)),crate::DslValue::String(value)=>ValueRetirement::String(<String as RetireOwned>::retirement_seed(value)),crate::DslValue::Bytes(value)=>ValueRetirement::Bytes(<Vec<u8> as RetireOwned>::retirement_seed(value)),crate::DslValue::Array(value)=>ValueRetirement::Array(<Vec<crate::DslValue> as RetireOwned>::retirement_seed(value)),crate::DslValue::Object(value)=>ValueRetirement::Object(<Vec<(String,crate::DslValue)> as RetireOwned>::retirement_seed(value))}}fn controlled_retirement_supported()->bool{true}}


