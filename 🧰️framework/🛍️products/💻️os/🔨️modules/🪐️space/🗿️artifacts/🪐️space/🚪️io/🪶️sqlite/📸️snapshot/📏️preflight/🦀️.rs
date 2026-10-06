//! 📏️ Complete semantic cells and exact canonical wire borrow the actual retained owner.
use super::*;
#[path="./🫳️borrowed/🦀️.rs"]mod borrowed;
pub(super)fn check(value:&SpaceSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 semantic(value,SqliteSnapshotPhase::EncodeNative,control)?;let limits=control.limits();
 let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};
 let prefix=store::semio_format::declared_envelope_prefix_len(SpaceSnapshot::__DSL_ENVELOPE_ID,component,1)?;
 let maximum=limits.max_file_bytes.checked_sub(prefix).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native preflight ceiling cannot contain declared envelope"))?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{
  let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|checkpoint(event.completed,event.total);
  let mut native=semio_framework_value::NativeEncodeControl::new(remaining,&mut progress);
  let result=(||{
   let body=match encoding{
    SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(value,&borrowed::spec(),usize::MAX,&mut native)?,
    SnapshotEncoding::Binary=>{let mut options=store::os_pack::record::EncodeOptions::default();options.limits.max_file_len=u64::MAX;store::os_pack::record::measure_document_borrowed(value,&borrowed::spec(),&options,&mut native)?},
   };
   if body>maximum{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"exact canonical native wire exceeds caller file limit"))}Ok(())
  })();(result,native.owned_bytes())
 })?
}
