//! 💾️ Raw bytes and authored hexadecimal text under one cumulative native controller.
use crate::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;
use crate::store;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};
use semio_framework_value::{native_decoding::NativeDecodeProgress,native_encoding::NativeEncodeProgress};
fn native_cells(schema:&str,count:usize,bytes:impl IntoIterator<Item=Result<u8,ValueError>>,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{native.scoped_stage(|native|{let total=count.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Binary row extent overflow"))?;native.begin_stage(total)?;let mut completed=0;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(completed);completed=event.completed;native.advance(delta).is_ok()};let mut control=SqliteSnapshotControl::new(&mut progress,limits);crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native_cells(schema,count,bytes,&mut control)})}
pub(crate) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<BinarySnapshot,ValueError>{
 use super::receiving::{Carrier,count,bind,body};
 let limits=control.limits();let length=match payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};if length>limits.max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary native input exceeds file limit"))}
 let native_before=native_owner.native().owned_bytes();
 control.allocation_stage_native(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let Some(maximum)=native_before.checked_add(remaining) else{return(Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow")),0)};
  let result=native_owner.scoped_native(maximum,&mut |event:NativeDecodeProgress|checkpoint(event.completed,event.total),|owner|{
   let carrier=body(match payload{store::io_schema::IoPayload::Binary(bytes)=>Carrier::Raw(bytes),store::io_schema::IoPayload::Text(text)=>Carrier::Hex(text)},owner.native())?;
   let count=count(carrier,owner.native())?;if count.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Binary native row overflow"))?>limits.max_rows{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Binary native input exceeds row limit"))}
   match carrier{Carrier::Raw(bytes)=>native_cells(crate::STDIO_BINARY_DOCUMENT_SCHEMA,count,bytes.iter().copied().map(Ok),owner.native(),limits)?,Carrier::Hex(text)=>{let mut digits=text.chars().filter(|ch|!ch.is_whitespace());let octets=std::iter::from_fn(||{let first=digits.next()?;Some((||{let second=digits.next().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"odd validated hex extent"))?;Ok(super::receiving::digit(first)?*16+super::receiving::digit(second)?)})())});native_cells(crate::STDIO_BINARY_DOCUMENT_SCHEMA,count,octets,owner.native(),limits)?;}}
   owner.receive::<BinarySnapshot,BinarySnapshot>(|slot,native,body|{bind(carrier,crate::STDIO_BINARY_DOCUMENT_SCHEMA,slot,native,body)?;slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Binary original receiving slot was lost"))})
  });
  (result,native_owner.native().owned_bytes().saturating_sub(native_before))
 })?
}
fn forecast(snapshot:&BinarySnapshot,encoding:SnapshotEncoding,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{ if snapshot.bytes.len().checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Binary native row overflow"))?>limits.max_rows{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Binary native output exceeds row limit"))}
 let output_length=match encoding{SnapshotEncoding::Binary=>snapshot.bytes.len(),SnapshotEncoding::Text=>{let body=snapshot.bytes.len().checked_mul(2).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary hex output size overflow"))?;let prefix=store::semio_format::declared_envelope_prefix_len("stdio.binary",store::semio_format::Component::Dsl,1)?;body.checked_add(prefix).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary hex output size overflow"))?}};if output_length>limits.max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary native output exceeds file limit"))}
 Ok(())
}
/// 📏️ Checks the exact borrowed raw/hex extent before any native ownership admission.
pub(crate) fn preflight(snapshot:&BinarySnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{crate::standards::v_raw::subsets::any::io::sqlite::snapshot::semantic(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;forecast(snapshot,encoding,control.limits())}
pub(crate) fn encode(snapshot:&BinarySnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{
 use semio_framework_value::{RetainedCloneGrant,retained_clone::RetainedCloneProgress};
 use store::io_schema::IoPayload;
 const PREFIX:&str="semio stdio.binary.dsl v1\n";
 const HEX:&[u8;16]=b"0123456789abcdef";
 let limits=control.limits();crate::standards::v_raw::subsets::any::io::sqlite::snapshot::semantic(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;forecast(snapshot,encoding,limits)?;
 let length=match encoding{SnapshotEncoding::Binary=>snapshot.bytes.len(),SnapshotEncoding::Text=>snapshot.bytes.len().checked_mul(2).and_then(|value|value.checked_add(PREFIX.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary hex output size overflow"))?};
 let native_before=native_owner.native().owned_bytes();
 control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{
  let Some(maximum)=native_before.checked_add(remaining) else{return(Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow")),0)};
  let result=native_owner.scoped_native(maximum,&mut |event:NativeEncodeProgress|checkpoint(event.completed,event.total),|owner|owner.receive::<IoPayload,IoPayload>(|slot,native,body|{
   let header=std::mem::size_of::<IoPayload>();let vector_header=if encoding==SnapshotEncoding::Binary{std::mem::size_of::<Vec<u8>>()}else{0};let copy=header.checked_add(vector_header).and_then(|value|value.checked_add(length)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary output receiving extent overflow"))?;
   body.admit_frontier(RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:copy,maximum_capacity_bytes:length,maximum_release_bytes:0,maximum_depth:1})?;native.checkpoint()?;body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:header,retained_capacity_bytes:0,released_bytes:0})?;
   *slot=Some(match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(Vec::new()),SnapshotEncoding::Text=>IoPayload::Text(String::new())});
   match slot.as_mut().unwrap(){
    IoPayload::Binary(output)=>{body.allocate_encode_vec_into(native,snapshot.bytes.len(),output,1)?;native.scoped_stage(|native|{native.begin_stage(snapshot.bytes.len())?;for chunk in snapshot.bytes.chunks(256){output.extend_from_slice(chunk);body.record_progress(RetainedCloneProgress{copied_items:0,copied_bytes:chunk.len(),retained_capacity_bytes:0,released_bytes:0})?;native.advance(chunk.len())?;}Ok::<(),ValueError>(())})?;},
    IoPayload::Text(output)=>{native.scoped_stage(|native|{native.begin_stage(snapshot.bytes.len())?;native.charge(length)?;output.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"Binary output receiving text allocation failed"))?;body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:output.capacity(),released_bytes:0})?;output.push_str(PREFIX);body.record_progress(RetainedCloneProgress{copied_items:0,copied_bytes:PREFIX.len(),retained_capacity_bytes:0,released_bytes:0})?;for byte in &snapshot.bytes{output.push(char::from(HEX[usize::from(byte>>4)]));output.push(char::from(HEX[usize::from(byte&15)]));body.record_progress(RetainedCloneProgress{copied_items:0,copied_bytes:2,retained_capacity_bytes:0,released_bytes:0})?;native.advance(1)?;}Ok::<(),ValueError>(())})?;}
   }
   slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Binary original output receiving slot was lost"))
  }));(result,native_owner.native().owned_bytes().saturating_sub(native_before))
 })?
}
