//! 📏️ Complete semantic cells and exact canonical wire borrow the actual retained owner.
use super::*;
#[path="🫳️borrowed/🦀️.rs"]mod borrowed;
pub(super)fn check(value:&SpaceHistorySnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 semantic(value,SqliteSnapshotPhase::EncodeNative,control)?;let limits=control.limits();
 let prefix=0usize;
 let maximum=limits.max_file_bytes.checked_sub(prefix).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native preflight ceiling cannot contain declared envelope"))?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint,allocation|{
  let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|checkpoint(event.completed,event.total);
  let mut native_allocation=|request:semio_framework_value::native_encoding::NativeEncodeAllocation|allocation(request.bytes);let mut native=semio_framework_value::NativeEncodeControl::new_forwarded(remaining,&mut progress,&mut native_allocation);
  let result=(||{
   let body=borrowed::measure(value,encoding,usize::MAX,&mut native)?;
   if body>maximum{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"exact canonical native wire exceeds caller file limit"))}Ok(())
  })();(result,native.owned_bytes())
 })?
}
