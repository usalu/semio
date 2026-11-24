//! 🪞️ A shell cursor erases the one cursor a derived owner retires through, so every retirement port is compiled once instead of once per derived owner.
use super::{RetirementCursor,RetirementStep};
use crate::retained_clone::RetainedCloneGrant;

/// 🫧️ Exposes the one erased cursor of a derived owner's retirement.
pub trait RetirementShell:Send{
    /// 🔭️ Borrows the erased cursor.
    fn shell(&self)->&dyn RetirementCursor;
    /// 🔧️ Borrows the erased cursor exclusively.
    fn shell_mut(&mut self)->&mut dyn RetirementCursor;
}

/// 🪞️ Retires exactly as the cursor its shell erases.
pub struct ShellCursor<S:RetirementShell>(pub S);

impl<S:RetirementShell> RetirementCursor for ShellCursor<S>{
    fn requires_original_birth(&self)->bool{self.0.shell().requires_original_birth()}
    fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{self.0.shell_mut().close_original_birth_step(original,grant)}
    fn requires_original_control(&self)->bool{self.0.shell().requires_original_control()}
    fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{self.0.shell_mut().close_step_controlled(body,effect,control)}
    fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{self.0.shell_mut().close_original_step(original,body,effect)}
    fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.0.shell().next_computational_work(body)}
    fn next_demand(&self,maximum_body_bytes:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.0.shell().next_demand(maximum_body_bytes)}
    fn close_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetirementStep,crate::OriginalRetirementFailure>,crate::ValueError>{self.0.shell_mut().close_into(body,effect,reply)}
    fn recover_close_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetirementStep,crate::OriginalRetirementFailure>,crate::ValueError>{self.0.shell_mut().recover_close_into(body,effect,reply)}
    fn close_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetirementStep,crate::OriginalRetirementFailure>,crate::ValueError>{self.0.shell_mut().close_original_into(original,body,effect,reply)}
    fn recover_close_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetirementStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetirementStep,crate::OriginalRetirementFailure>,crate::ValueError>{self.0.shell_mut().recover_close_original_into(original,body,effect,reply)}
    fn terminal_is_empty(&self)->bool{self.0.shell().terminal_is_empty()}
    fn next_depth_demand(&self)->Result<usize,crate::ValueError>{self.0.shell().next_depth_demand()}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.shell().next_close_byte_demand()}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{self.0.shell().next_work_byte_demand()}
    fn next_selected_work_byte_demand(&self,maximum_body_bytes:usize)->Result<usize,crate::ValueError>{self.0.shell().next_selected_work_byte_demand(maximum_body_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{self.0.shell().allows_admitted_narrow_work()}
    fn next_birth_bytes(&self,maximum_bytes:usize)->Option<usize>{self.0.shell().next_birth_bytes(maximum_bytes)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
