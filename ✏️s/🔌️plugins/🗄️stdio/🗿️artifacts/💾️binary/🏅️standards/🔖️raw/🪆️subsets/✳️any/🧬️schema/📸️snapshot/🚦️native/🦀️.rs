//! 💾️ Raw bytes and authored hexadecimal text under one cumulative native controller.
use super::BinarySnapshot;
use crate::store;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
use semio_framework_value::{native_decoding::NativeDecodeProgress,native_encoding::NativeEncodeProgress};
fn digit(ch:char)->Result<u8,ValueError>{ch.to_digit(16).and_then(|v|u8::try_from(v).ok()).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"invalid native Binary hex digit"))}
pub(super) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<BinarySnapshot,ValueError>{
 let limits=control.limits();let length=match payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};if length>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Binary native input exceeds file limit"))}
 control.allocation_stage(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{let mut progress=|event:NativeDecodeProgress|checkpoint(event.completed,event.total);let mut native=NativeDecodeControl::new(remaining,&mut progress);let result=(||->Result<BinarySnapshot,ValueError>{
 let bytes=match payload{
  store::io_schema::IoPayload::Binary(bytes)=>{let rows=bytes.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Binary native row overflow"))?;if rows>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Binary native input exceeds row limit"))}native.begin_stage(bytes.len())?;let mut output=native.allocate_vec(bytes.len())?;for chunk in bytes.chunks(256){output.extend_from_slice(chunk);native.advance(chunk.len())?;}output},
  store::io_schema::IoPayload::Text(text)=>{
   let body=if text.starts_with("semio "){store::semio_format::split_text_preamble_controlled(text,"stdio.binary",store::semio_format::Component::Dsl,1,&mut native).map_err(store::semio_format::SemioError::into_value_error)?}else{text.as_str()};
   native.begin_stage(body.len())?;let mut count=0usize;for ch in body.chars(){if !ch.is_whitespace(){digit(ch)?;count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Binary hex length overflow"))?;}native.advance(ch.len_utf8())?;}if count%2!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"odd hex length"))}let count=count/2;if count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Binary native row overflow"))?>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Binary native input exceeds row limit"))}
   native.begin_stage(body.len())?;let mut output=native.allocate_vec(count)?;let mut high=None;for ch in body.chars(){if !ch.is_whitespace(){let value=digit(ch)?;if let Some(first)=high.take(){output.push(first*16+value);}else{high=Some(value);}}native.advance(ch.len_utf8())?;}output
  }
 };
 Ok(BinarySnapshot{schema:native.copy_text(crate::STDIO_BINARY_DOCUMENT_SCHEMA)?,bytes})
})();(result,native.owned_bytes())})?
}
fn forecast(snapshot:&BinarySnapshot,encoding:SnapshotEncoding,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{ if snapshot.bytes.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Binary native row overflow"))?>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Binary native output exceeds row limit"))}
 let output_length=match encoding{SnapshotEncoding::Binary=>snapshot.bytes.len(),SnapshotEncoding::Text=>{let body=snapshot.bytes.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Binary hex output size overflow"))?;let prefix=store::semio_format::declared_envelope_prefix_len("stdio.binary",store::semio_format::Component::Dsl,1)?;body.checked_add(prefix).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Binary hex output size overflow"))?}};if output_length>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Binary native output exceeds file limit"))}
 Ok(())
}
/// 📏️ Checks the exact borrowed raw/hex extent before any native ownership admission.
pub(super) fn preflight(snapshot:&BinarySnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,snapshot.bytes.len())?;forecast(snapshot,encoding,control.limits())}
pub(super) fn encode(snapshot:&BinarySnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
 let limits=control.limits();forecast(snapshot,encoding,limits)?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{let mut progress=|event:NativeEncodeProgress|checkpoint(event.completed,event.total);let mut native=NativeEncodeControl::new(remaining,&mut progress);let result=(||->Result<store::io_schema::IoPayload,ValueError>{native.begin_stage(snapshot.bytes.len())?;
 match encoding{
  SnapshotEncoding::Binary=>{let mut output=native.allocate_vec(snapshot.bytes.len())?;for chunk in snapshot.bytes.chunks(256){output.extend_from_slice(chunk);native.advance(chunk.len())?;}Ok(store::io_schema::IoPayload::Binary(output))},
  SnapshotEncoding::Text=>{let count=snapshot.bytes.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Binary hex length overflow"))?;let mut output=native.allocate_vec(count)?;const HEX:&[u8;16]=b"0123456789abcdef";for chunk in snapshot.bytes.chunks(256){for byte in chunk{output.push(HEX[usize::from(byte>>4)]);output.push(HEX[usize::from(byte&15)]);}native.advance(chunk.len())?;}let body=String::from_utf8(output).map_err(|error|ValueError::new(ValueRefusalKind::InvariantViolated,error.to_string()))?;Ok(store::io_schema::IoPayload::Text(store::semio_format::wrap_text_controlled("stdio.binary",store::semio_format::Component::Dsl,1,&body,&mut native)?))}
 }
})();(result,native.owned_bytes())})?
}
