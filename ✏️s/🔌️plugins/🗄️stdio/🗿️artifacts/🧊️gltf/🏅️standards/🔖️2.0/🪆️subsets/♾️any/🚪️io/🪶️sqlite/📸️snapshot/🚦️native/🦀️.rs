//! 🛬️ Adjacent GLTF input admission awaits the owning capability baseline.
use super::*;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_dsl_record::RecordSpecProducer;

/// 📥️ Decodes through a supplied actual schema and typed owner constructor with cumulative settlement.
pub(super) fn decode(payload:&store::io::IoPayload,spec:RecordSpecProducer,reconstruct:impl FnOnce(&semio_framework_dsl_record::RecordValue,&mut NativeDecodeControl<'_>)->Result<GltfSnapshot,ValueError>,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<GltfSnapshot>>)->store::os_store::serialization::SnapshotReceivingOutcome{
 let length=match payload{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};
 if length>control.limits().max_file_bytes{return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(ValueError::new(ValueRefusalKind::OwnershipLimit,"GLTF native input exceeds file byte limit"))),work:0}}
 let mut reconstruct=Some(reconstruct);
 store::decode_sqlite_snapshot_record_receiving(payload,"stdio.gltf",spec,&mut |record,snapshot_output,owner,control|semio_os_kernel::native_snapshot_turn(|work|work.measured(owner.native(),|native|{
  let snapshot=reconstruct.take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"GLTF native reconstruction was already consumed"))?(record,native)?;
  let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(snapshot,GltfSnapshot::retire_sqlite_snapshot);owner.as_mut().to_sqlite_database(control)?;
  *snapshot_output=Some(owner.take());Ok::<(),ValueError>(())
 })),control,native_control,output)
}

