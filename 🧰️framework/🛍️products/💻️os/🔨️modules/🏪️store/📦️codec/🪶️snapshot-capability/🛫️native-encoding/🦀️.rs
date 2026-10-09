//! 🛫️ Declared native record output with one caller-owned cumulative controller.
use semio_framework_value::NativeEncodeControl;
use semio_framework_dsl_record::RecordSpecProducer;
use semio_framework_dsl_record::RecordValue;
use crate::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotControl, SqliteSnapshotPhase};
use crate::sqlite_snapshot::{ValueError, ValueRefusalKind};
use semio_framework_value::native_encoding::NativeEncodeProgress;

/// 🧮️ Projects and emits only the owner's declared controlled record factories.
pub fn encode_sqlite_snapshot_record_native(
    encoding: SnapshotEncoding,
    envelope_id: &str,
    spec: RecordSpecProducer,
    construct: impl FnOnce(&mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError>,
    control: &mut SqliteSnapshotControl<'_>,
    native_owner: &mut crate::os_store::NativeSnapshotEncodeOwner<'_, '_>,
) -> Result<crate::io_schema::IoPayload, ValueError> {
    let limits = control.limits();
    control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, 0)?;
    let component = match encoding {
        SnapshotEncoding::Binary => super::semio_format::Component::Pack,
        SnapshotEncoding::Text => super::semio_format::Component::Dsl,
    };
    let body_limit =
        limits.max_file_bytes.checked_sub(super::semio_format::declared_envelope_prefix_len(envelope_id, component, 1)?).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native file ceiling cannot contain its declared envelope"))?;
    let before=native_owner.native().owned_bytes();
    let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot cumulative allowance overflow"))?;
    let mut refused=None;
    let mut observer=|event:NativeEncodeProgress|match control.checkpoint(SqliteSnapshotPhase::EncodeNative,event.completed,event.total){Ok(())=>true,Err(error)=>{refused=Some(error);false}};
    let result=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<(Option<semio_framework_dsl_record::RecordSpec>,Option<RecordValue>,Option<crate::io_schema::IoPayload>,Option<crate::io_schema::IoPayload>),crate::io_schema::IoPayload>(|intermediate,native,_body|{
        let result = (|| -> Result<crate::io_schema::IoPayload, ValueError> {
            *intermediate=Some((None,None,None,None));
            intermediate.as_mut().unwrap().0=Some(spec.encode(native)?);
            intermediate.as_mut().unwrap().1=Some(construct(native)?);
            let (spec,record,body,candidate)=intermediate.as_mut().unwrap();let spec=spec.as_ref().unwrap();let record=record.as_ref().unwrap();
            let output = match encoding {
                SnapshotEncoding::Binary => {
                    let mut options = pack::record::EncodeOptions::default();
                    options.limits.max_file_len = body_limit as u64;
                    *body=Some(crate::io_schema::IoPayload::Binary(pack::record::encode_document_controlled(spec, record, &options, native).map_err(super::PackRefusal::into_value_error)?));
                    let Some(crate::io_schema::IoPayload::Binary(bytes))=body.as_ref()else{unreachable!()};crate::io_schema::IoPayload::Binary(super::semio_format::wrap_binary_controlled(envelope_id, component, 1, bytes, native)?)
                }
                SnapshotEncoding::Text => {
                    *body=Some(crate::io_schema::IoPayload::Text(semio_framework_dsl_record::print_controlled(record, spec, semio_framework_dsl_record::JoinMode::Document, body_limit, native).map_err(|error| ValueError::new(error.kind, error.message))?));
                    let Some(crate::io_schema::IoPayload::Text(text))=body.as_ref()else{unreachable!()};crate::io_schema::IoPayload::Text(super::semio_format::wrap_text_controlled(envelope_id, component, 1, text, native)?)
                }
            };
            *candidate=Some(output);
            let length = match candidate.as_ref().unwrap() {
                crate::io_schema::IoPayload::Binary(bytes) => bytes.len(),
                crate::io_schema::IoPayload::Text(text) => text.len(),
            };
            if length > limits.max_file_bytes {
                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native snapshot output exceeds file byte limit"));
            }
            Ok(candidate.take().unwrap())
        })();
        result
        }));
    drop(observer);
    control.admit_native_allocation_bytes(native_owner.native().owned_bytes().saturating_sub(before))?;
    if let Some(error)=refused{Err(error)}else{result}
}

#[cfg(test)]
#[path="🧪️tests/🚪️control/🦀️.rs"]
mod original_control_tests;
