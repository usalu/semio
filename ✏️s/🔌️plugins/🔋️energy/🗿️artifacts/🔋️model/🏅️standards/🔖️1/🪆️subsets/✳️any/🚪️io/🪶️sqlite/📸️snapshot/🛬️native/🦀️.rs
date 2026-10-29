//! 🛬️ Explicit Energy native record construction under cumulative input and output controls.
use crate::reference::EnergyModelSnapshot;
use crate::reference::binary::EnergyModelPackRecord;
use semio_framework_value::{FromValue,ToValue};
use store::sqlite_snapshot::{SqliteSnapshotControl,SnapshotEncoding};
use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordValue;
use semio_framework_diagnostic::TextError;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}
fn error(error:ValueError)->TextError{TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}

fn construct(record:&RecordValue,c:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<EnergyModelSnapshot,ValueError>{
 semio_s_artifact_energy_model_snapshot_sqlite_admission::admit_record(record,c,limits,super::SCHEMA.len())?;let record=EnergyModelPackRecord::__dsl_from_record_controlled(record,c)?;let model=crate::model::Model::from_value_controlled(&record.model,c)?;
 Ok(EnergyModelSnapshot{schema:record.schema,model,structure:record.structure,zones:record.zones,referenced_model:record.referenced_model,weather_link:record.weather_link})
}
pub(crate)fn decode(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<EnergyModelSnapshot,ValueError>{let limits=c.limits();store::decode_sqlite_snapshot_record_native(payload,<EnergyModelSnapshot as pack::snapshot::ArtifactDsl>::envelope_id(),EnergyModelPackRecord::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {construct(record,native,limits)})(); *snapshot_output = Some(constructed?); Ok(()) },c,native_control)}
fn field<T:DslField>(value:&T,c:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{c.scoped_stage(|c|{c.begin_stage(0)?;value.to_value_controlled(c)})}
fn optional<T:DslField>(value:&Option<T>,c:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{match value{Some(value)=>field(value,c),None=>{c.checkpoint()?;Ok(FieldValue::Absent)}}}
fn project(snapshot:&EnergyModelSnapshot,c:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 c.scoped_stage(|c|->Result<RecordValue,ValueError>{c.begin_stage(6)?;let mut out=semio_framework_dsl_record::native_encoding::EncodedRecord::new(6,c)?;
 out.insert(0,field(&snapshot.schema,c)?)?;c.step()?;
 let model=c.scoped_stage(|c|{c.begin_stage(0)?;snapshot.model.to_value_controlled(c)})?;out.insert(1,FieldValue::Value(model))?;c.step()?;
 out.insert(2,field(&snapshot.structure,c)?)?;c.step()?;out.insert(3,field(&snapshot.zones,c)?)?;c.step()?;out.insert(4,optional(&snapshot.referenced_model,c)?)?;c.step()?;out.insert(5,optional(&snapshot.weather_link,c)?)?;c.step()?;Ok(out.take())
 })
}
pub(crate)fn encode(snapshot:&EnergyModelSnapshot,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{super::admit(snapshot,c)?;store::encode_sqlite_snapshot_record_native(encoding,<EnergyModelSnapshot as pack::snapshot::ArtifactDsl>::envelope_id(),EnergyModelPackRecord::__dsl_spec_producer(),|native|project(snapshot,native),c,native_owner)}
