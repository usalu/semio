//! 🛫️ Declared native record output with one caller-owned cumulative controller.
use crate::os_dsl::{NativeEncodeControl,RecordSpecProducer,RecordValue,TextError,JoinMode};
use crate::os_dsl::native_encoding::EncodedRecord;
use crate::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
use semio_framework_value::native_encoding::NativeEncodeProgress;

/// 🧮️ Projects and emits only the owner's declared controlled record factories.
pub fn encode_sqlite_snapshot_record_native(
    encoding:SnapshotEncoding,
    envelope_id:&str,
    spec:RecordSpecProducer,
    construct:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<RecordValue,TextError>,
    control:&mut SqliteSnapshotControl<'_>,
)->Result<crate::io_schema::IoPayload,String>{
    let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let component=match encoding{SnapshotEncoding::Binary=>super::semio_format::Component::Pack,SnapshotEncoding::Text=>super::semio_format::Component::Dsl};let body_limit=limits.max_file_bytes.checked_sub(super::semio_format::declared_envelope_prefix_len(envelope_id,component,1)?).ok_or("native file ceiling cannot contain its declared envelope")?;let mut progress=|event:NativeEncodeProgress|control.checkpoint(SqliteSnapshotPhase::EncodeNative,event.completed,event.total).is_ok();let mut native=NativeEncodeControl::new(limits.max_value_bytes,&mut progress);let spec=spec.encode(&mut native)?;let record=EncodedRecord::from_record(construct(&mut native).map_err(|error|error.message)?);
    let output=match encoding{
        SnapshotEncoding::Binary=>{let mut options=super::PackEncodeOptions::default();options.limits.max_file_len=body_limit as u64;let body=crate::os_pack::encode_document_controlled(&spec,record.as_record(),&options,&mut native).map_err(|error|error.to_string())?;crate::io_schema::IoPayload::Binary(super::semio_format::wrap_binary_controlled(envelope_id,component,1,&body,&mut native)?)},
        SnapshotEncoding::Text=>{let body=crate::os_dsl::schema::print_controlled(record.as_record(),&spec,JoinMode::Document,body_limit,&mut native).map_err(|error|error.message)?;crate::io_schema::IoPayload::Text(super::semio_format::wrap_text_controlled(envelope_id,component,1,&body,&mut native)?)},
    };let length=match &output{crate::io_schema::IoPayload::Binary(bytes)=>bytes.len(),crate::io_schema::IoPayload::Text(text)=>text.len()};if length>limits.max_file_bytes{return Err("native snapshot output exceeds file byte limit".into())}Ok(output)
}
