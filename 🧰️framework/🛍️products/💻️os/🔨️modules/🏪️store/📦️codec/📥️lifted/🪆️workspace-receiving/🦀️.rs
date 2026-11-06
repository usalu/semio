//! 📥️ A codec stage borrows the cleanup recipient already retained in its admitted parent frame.
use semio_framework_io_sqlite_snapshot::SqliteSnapshotControl;
use semio_framework_value::RetainedWorkOutcome;
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
/// 🧳️ Keeps stage cleanup with its caller on success, refusal and unwind, forwarding the child's actual work.
pub fn staged<T,O,E:From<semio_framework_value::ValueError>>(control:&mut SqliteSnapshotControl<'_>,workspace:&mut NativeDecodeRetirementRecipient,owner:&mut O,stage:&mut impl FnMut(&mut SqliteSnapshotControl<'_>,&mut O)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,E>{
 let mut work=0;
 let result=control.with_workspace_recipient(workspace,|control|{let turn=stage(control,owner);work=turn.work;turn.result});
 RetainedWorkOutcome{result,work}
}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_io_control::NativeSnapshotFailure;
 use semio_framework_value::{ErasedSnapshotRetirement,RetainedCloneGrant,ValueError,ValueRefusalKind};
 use semio_framework_io_sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotPhase};
 #[test]
 fn stage_forwards_actual_work_and_both_original_failures(){
  let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut workspace=NativeDecodeRetirementRecipient::new();let mut calls=0;
  let turn=staged::<(),_,_>(&mut control,&mut workspace,&mut calls,&mut |_,calls|{*calls+=1;RetainedWorkOutcome{work:7,result:Err(NativeSnapshotFailure::primary_with_work(ValueError::literal(ValueRefusalKind::InvalidValue,"original stage"),ValueError::literal(ValueRefusalKind::WorkLimit,"original work post")))}});
  assert_eq!(turn.work,7);let failure=turn.result.unwrap_err();assert_eq!(failure.message(),"original stage");assert_eq!(failure.work_refusal().unwrap().message,"original work post");assert_eq!(calls,1);assert!(workspace.terminal_is_empty());
 }
 #[test]
 fn stage_unwind_keeps_its_reserved_workspace_for_paid_cleanup(){
  let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut workspace=NativeDecodeRetirementRecipient::new();
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||staged::<(),_,_>(&mut control,&mut workspace,&mut (),&mut |control,_|{
   let result=control.with_retirement_owner::<(),NativeSnapshotFailure>(SqliteSnapshotPhase::ProjectSnapshot,0,|_|panic!("stage after reserved workspace"));
   RetainedWorkOutcome{result,work:0}
  })));
  assert!(result.is_err());assert!(workspace.is_reserved());let mut called=false;
  let denied=staged::<(),_,_>(&mut control,&mut workspace,&mut (),&mut |_,_|{called=true;panic!("denied stage was invoked")});
  assert!(denied.result.is_err());assert_eq!(denied.work,0);assert!(!called);
  let turn=ErasedSnapshotRetirement::close_step(&mut workspace,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()});assert_eq!(turn.work,1);turn.result.unwrap();assert!(workspace.terminal_is_empty());
 }
}
