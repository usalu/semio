//! 📦️ SQL octets remain in their original field across native and SQL settlement failures.
use super::{SnapshotOwner,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use semio_framework_io_control::{NativeSnapshotEffectProgress,NativeSnapshotFailure};
use super::super::{SqliteNativeStageOutcome,SqliteStageFailure,SqliteStageWorkReceipt};
/// 🧊️ Copies into an empty caller-owned field under the initiating original native owner.
pub fn copy_into(output:&mut Vec<u8>,blob:&[u8],control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,owner:&mut SnapshotOwner<'_,'_,'_>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,stage_unwind:&mut Option<SqliteNativeStageOutcome<(),NativeSnapshotFailure>>,allocation:&mut semio_framework_io_control::NativeSnapshotAllocation,depth:usize)->SqliteNativeStageOutcome<(),NativeSnapshotFailure>{
 if output.capacity()!=0||stage_unwind.is_some(){return SqliteNativeStageOutcome{result:Err(SqliteStageFailure::NotStarted(ValueError::literal(ValueRefusalKind::InvalidValue,"SQLite blob requires an empty destination and unwind receipt"))),work:Some(0)}}
 let mut payload=None;
 let turn=control.allocation_stage_native(phase,|remaining,progress|{
  let before=owner.actual_work();
  let turn=if blob.len()>remaining{semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"SQLite blob exceeds its remaining original allocation allowance").into()),work:0}}else{
   match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||owner.copy_blob(blob,output,marker,unwind,allocation,depth,progress))){
    Ok(turn)=>turn,
    Err(error)=>{payload=Some(error);let work=u128::from(output.capacity()!=0)+(output.len().div_ceil(65536)as u128);semio_framework_value::RetainedWorkOutcome{result:Ok(()),work}},
   }
  };
  let after=owner.actual_work();let receipt=SqliteStageWorkReceipt{declared:turn.work,before,after,actual:after.checked_sub(before)};
  (turn,output.capacity(),receipt)
 });
 if let Some(payload)=payload{*stage_unwind=Some(turn);std::panic::resume_unwind(payload)}turn
}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
