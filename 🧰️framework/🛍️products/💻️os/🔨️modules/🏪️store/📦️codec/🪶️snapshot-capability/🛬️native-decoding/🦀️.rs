//! 🛬️ Controlled physical record decoding for explicitly declared snapshot owners.
use semio_framework_value::NativeDecodeControl;
use semio_framework_dsl_record::ParseOptions;
use semio_framework_dsl_record::RecordSpecProducer;
use semio_framework_dsl_record::RecordValue;
use crate::sqlite_snapshot::{SqliteSnapshotControl, SqliteSnapshotPhase};
use crate::sqlite_snapshot::{ValueError, ValueRefusalKind};
use semio_framework_diagnostic::Limits;
use semio_framework_value::native_decoding::NativeDecodeProgress;

/// 🧮️ Preserves one cumulative ownership budget through borrowed envelope, physical parser and typed binding.
pub fn decode_sqlite_snapshot_record_native<T:semio_framework_value::retirement::RetireOwned>(
    payload: &crate::io_schema::IoPayload,
    envelope_id: &str,
    spec: RecordSpecProducer,
    construct: impl FnOnce(&mut RecordValue, &mut Option<T>, &mut NativeDecodeControl<'_>, &mut crate::os_store::NativeSnapshotBodyWallet) -> Result<(), ValueError>,
    control: &mut SqliteSnapshotControl<'_>,
    native_owner: &mut crate::os_store::NativeSnapshotDecodeOwner<'_, '_>,
) -> Result<T, ValueError> {
    let limits = control.limits();
    control.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, 0)?;
    let length = match payload {
        crate::io_schema::IoPayload::Binary(bytes) => bytes.len(),
        crate::io_schema::IoPayload::Text(text) => text.len(),
    };
    if length > limits.max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native snapshot input exceeds file byte limit"));
    }
    let before=native_owner.native().owned_bytes();
    let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot cumulative allowance overflow"))?;
    let mut refused=None;
    let mut observer=|event:NativeDecodeProgress|match control.checkpoint(SqliteSnapshotPhase::DecodeNative,event.completed,event.total){Ok(())=>true,Err(error)=>{refused=Some(error);false}};
    let result=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<(Option<semio_framework_dsl_record::RecordSpec>,Option<RecordValue>,Option<T>),T>(|intermediate,native,body|{
        let result = (|| -> Result<T, ValueError> {
            *intermediate=Some((None,None,None));
            intermediate.as_mut().unwrap().0=Some(spec.decode(native)?);
            let (spec,record,output)=intermediate.as_mut().unwrap();let spec=spec.as_ref().unwrap();
            *record=Some(match payload {
                crate::io_schema::IoPayload::Binary(bytes) => {
                    let body = super::semio_format::unwrap_binary_controlled(bytes, envelope_id, super::semio_format::Component::Pack, 1, native).map_err(super::semio_format::SemioError::into_value_error)?;
                    pack::record::decode_document_controlled(body, &spec, &pack::record::DecodeOptions::default(), native).map_err(super::PackRefusal::into_value_error)?.0
                }
                crate::io_schema::IoPayload::Text(text) => {
                    let body = super::semio_format::split_text_preamble_controlled(text, envelope_id, super::semio_format::Component::Dsl, 1, native).map_err(super::semio_format::SemioError::into_value_error)?;
                    semio_framework_dsl_record::parse_exact_controlled(body, &spec, &ParseOptions { limits: Limits { max_bytes: limits.max_file_bytes, ..Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Document }, native)
                        .map_err(|error| ValueError::new(error.kind, error.message))?
                }
            });
            construct(record.as_mut().unwrap(), output, native, body)?;
            output.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native constructor did not retain its actual output"))
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
