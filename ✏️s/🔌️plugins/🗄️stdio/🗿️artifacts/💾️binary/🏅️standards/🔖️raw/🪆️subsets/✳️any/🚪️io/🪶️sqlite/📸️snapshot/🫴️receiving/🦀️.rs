//! 🫴️ Original typed Binary fields before native schema or octet backing is born.
use super::BinarySnapshot;
use semio_os_kernel::NativeSnapshotBodyWallet;
use semio_framework_value::{RetainedCloneGrant, retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
#[derive(Clone,Copy)]
pub(super)enum Carrier<'a>{Raw(&'a[u8]),Hex(&'a str)}
fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary original receiving extent overflow")}
pub(super)fn digit(ch:char)->Result<u8,ValueError>{ch.to_digit(16).and_then(|value|u8::try_from(value).ok()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"invalid native Binary hex digit"))}
/// 📨️ Borrows the authored Binary envelope without creating an owned parser refusal.
pub(super)fn body<'a>(carrier:Carrier<'a>,native:&mut NativeDecodeControl<'_>)->Result<Carrier<'a>,ValueError>{
 let Carrier::Hex(text)=carrier else{return Ok(carrier)};if !text.starts_with("semio "){return Ok(carrier)}native.checkpoint()?;
 const TOKEN:&[u8]=b"stdio.binary.dsl v1";let end=6+TOKEN.len();let bytes=text.as_bytes();if bytes.len()<end{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid semio preamble: truncated envelope token"))}
 let matches=native.scoped_stage(|native|{native.begin_stage(TOKEN.len())?;let mut position=6;for piece in[b"stdio.binary".as_slice(),b".",b"dsl",b" v",b"1"]{if bytes[position..position+piece.len()]!=*piece{return Ok(false)}position+=piece.len();native.advance(piece.len())?;}Ok::<bool,ValueError>(true)})?;
 if !matches{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid semio preamble: declared envelope identity mismatch"))}
 let offset=if end==bytes.len(){end}else if bytes[end]==b'\n'{end+1}else if bytes.get(end..end+2)==Some(b"\r\n"){end+2}else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid semio preamble: canonical preamble requires a line boundary"))};Ok(Carrier::Hex(&text[offset..]))
}
/// 🔍️ Validates borrowed hex before the original recipient can create any physical frame.
pub(super)fn count(carrier:Carrier<'_>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{match carrier{Carrier::Raw(bytes)=>Ok(bytes.len()),Carrier::Hex(text)=>native.scoped_stage(|native|{native.begin_stage(text.len())?;let mut digits=0usize;for ch in text.chars(){if !ch.is_whitespace(){digit(ch)?;digits=digits.checked_add(1).ok_or_else(overflow)?;}native.advance(ch.len_utf8())?;}if digits%2!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"odd hex length"))}Ok(digits/2)})}}
/// 🧾️ Admits every independently supplied body currency before publishing even an empty typed slot.
pub(super)fn bind(carrier:Carrier<'_>,schema:&str,slot:&mut Option<BinarySnapshot>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet)->Result<(),ValueError>{
 if slot.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Binary original receiving slot must be absent"))}
 let carrier=self::body(carrier,native)?;let count=count(carrier,native)?;let vector_header=if matches!(carrier,Carrier::Hex(_)){size_of::<Vec<u8>>()}else{0};let header=size_of::<BinarySnapshot>();let copy=header.checked_add(schema.len()).and_then(|value|value.checked_add(count)).and_then(|value|value.checked_add(vector_header)).ok_or_else(overflow)?;let capacity=schema.len().checked_add(count).ok_or_else(overflow)?;
 body.admit_frontier(RetainedCloneGrant{maximum_items:3,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1})?;
 native.checkpoint()?;body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:header,retained_capacity_bytes:0,released_bytes:0})?;
 *slot=Some(BinarySnapshot{schema:String::new(),bytes:Vec::new()});let output=slot.as_mut().unwrap();body.copy_text_into(native,schema,&mut output.schema,1)?;
 match carrier{
  Carrier::Raw(bytes)=>body.copy_bytes_into(native,bytes,&mut output.bytes,1),
  Carrier::Hex(text)=>{body.allocate_vec_into(native,count,&mut output.bytes,1)?;native.scoped_stage(|native|{native.begin_stage(text.len())?;let mut high=None;for ch in text.chars(){if !ch.is_whitespace(){let value=digit(ch)?;if let Some(first)=high.take(){output.bytes.push(first*16+value);body.record_progress(RetainedCloneProgress{copied_items:0,copied_bytes:1,retained_capacity_bytes:0,released_bytes:0})?;}else{high=Some(value);}}native.advance(ch.len_utf8())?;}Ok(())})}
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
