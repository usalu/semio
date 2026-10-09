//! 🌊️ Flow carrier bridge using its existing record schema authority.
use super::*;
use semio_framework_value::{ValueError,NativeDecodeControl,NativeEncodeControl};
use crate::os_store::{TextError,TextSpan};
use crate::store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl};
/// 🛬️ Decodes the physical carrier under the shared cumulative controller, then binds actual fields.
pub(super) fn decode(payload:&crate::store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut crate::os_store::NativeSnapshotDecodeOwner<'_,'_>)->Result<FlowHostSnapshot,ValueError>{crate::os_store::decode_sqlite_snapshot_record_native(payload,<FlowHostSnapshot as crate::os_store::ArtifactDsl>::envelope_id(),FlowHostSnapshotDsl::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {flow_native_decoding::decode_record(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_owner)}
/// 🛫️ Projects actual borrowed owner fields through the shared physical record producer.
pub(super) fn encode(value:&FlowHostSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut crate::os_store::NativeSnapshotEncodeOwner<'_,'_>)->Result<crate::store::io_schema::IoPayload,ValueError>{crate::os_store::encode_sqlite_snapshot_record_native(encoding,<FlowHostSnapshot as crate::os_store::ArtifactDsl>::envelope_id(),FlowHostSnapshotDsl::__dsl_spec_producer(),|native|flow_native_encoding::encode_record(value,native),control,native_owner)}
/// 📦️ Direct controlled root field projection without the ordinary metadata mirror.
pub(crate) fn encode_field(value:&FlowHostSnapshot,control:&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{flow_native_encoding::encode_record(value,control).map(semio_framework_dsl_record::FieldValue::Record)}
/// 📥️ Direct borrowed root field binding with the actual typed partial retirement owner.
pub(crate) fn decode_field(value:&semio_framework_dsl_record::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<FlowHostSnapshot,ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(record)=>flow_native_decoding::decode_record(record,control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Flow native root record required"))}}
