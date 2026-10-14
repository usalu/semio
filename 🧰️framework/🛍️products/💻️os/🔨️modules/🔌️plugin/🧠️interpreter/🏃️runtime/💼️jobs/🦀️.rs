//! 💼️ Backend execution and raw guest job custody remain separate until original acknowledgement.
use super::TurnFault;
use semio_framework_actor::{OriginalJobSource,OriginalGuestExecutionBudget,OriginalGuestExecutionReceipt};
/// 📬️ The actual raw reply or full trap remains beside its independent backend instruction receipt.
pub struct OriginalGuestJobCall<T>{pub original:Result<T,TurnFault>,pub execution:OriginalGuestExecutionReceipt,pub returned_unsubmitted_source:Option<OriginalJobSource>}
impl<T> OriginalGuestJobCall<T>{
 pub fn unsubmitted(budget:OriginalGuestExecutionBudget,source:Option<OriginalJobSource>,failure:TurnFault)->Self{Self{original:Err(failure),execution:OriginalGuestExecutionReceipt{budget,spent_fuel:Some(0),submitted:false},returned_unsubmitted_source:source}}
}
