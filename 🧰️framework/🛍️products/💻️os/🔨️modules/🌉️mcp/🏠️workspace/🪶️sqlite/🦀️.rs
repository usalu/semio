//! 🪶️ Probe values are relational syntax nodes with explicit collection ordinals.
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
#[path="🫴️semantic/🦀️.rs"]
mod semantic;
struct NumberWord{bytes:[u8;32],length:usize}
impl NumberWord{fn text(&self)->&str{std::str::from_utf8(&self.bytes[..self.length]).expect("formatted Probe numeric ASCII")}}
impl std::fmt::Write for NumberWord{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}}
fn number_text(number:Number)->Result<NumberWord,ValueError>{
 use std::fmt::Write as _;let mut word=NumberWord{bytes:[0;32],length:0};match number{Number::UInt(value)=>write!(&mut word,"{value}"),Number::Int(value)=>write!(&mut word,"{value}"),Number::Float(value)if value.is_finite()=>semio_framework_pack_json::write_float_to(value,&mut word),_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number is not finite"))}.map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe numeric word exceeded bounded scalar storage"))?;Ok(word)
}
fn number_value<P:semantic::Port>(text:&str,scanner:&mut Option<semio_framework_pack_json::JsonNumberCursor>,port:&mut P)->Result<DslValue,ValueError>{
 port.control(|control|{control.check_value_bytes(text.len())?;control.admit_reconstruction_bytes(text.len())})?;
 if !matches!(text.as_bytes().first(),Some(b'-'|b'0'..=b'9')){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number column requires an exact JSON numeric word"))}
 port.work(1,std::mem::size_of::<semio_framework_pack_json::JsonNumberCursor>())?;*scanner=Some(semio_framework_pack_json::JsonNumberCursor::new());
 loop{port.work(1,0)?;let cursor=scanner.as_mut().unwrap();let number=cursor.step(text).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number column must be a finite JSON numeric literal"))?;if let Some(number)=number{if cursor.position()!=text.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number column has trailing numeric bytes"))}return Ok(DslValue::Number(match number{semio_framework_pack_json::Number::UInt(value)=>Number::UInt(value),semio_framework_pack_json::Number::Int(value)=>Number::Int(value),semio_framework_pack_json::Number::Float(value)=>Number::Float(value)}));}}
}

impl store::ArtifactSqliteSnapshot for super::ProbeSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut frame=semantic::Projection::new();let mut port=semantic::Direct{control,phase:SqliteSnapshotPhase::ProjectSnapshot};semantic::project(&self.0,&mut frame,&mut port)?;Ok(frame.database)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let mut frame=semantic::Reconstruction::new();let mut port=semantic::Direct{control,phase:SqliteSnapshotPhase::ReconstructSnapshot};semantic::reconstruct(database,&mut frame,&mut port).map(Self)}
 fn to_sqlite_database_receiving(&self,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{
  let result=(||{
  if control.allocation_remaining_bytes()==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe SQL projection has no original backing authority"))}
  let before=owner.native().owned_bytes();let remaining=control.allocation_remaining_bytes();let maximum=before.checked_add(remaining).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe semantic allocation ceiling overflow"))?;
  let result=owner.scoped_native(maximum,&mut |_|true,|owner|owner.receive::<semantic::Projection,SqliteDatabase>(|slot,native,body|{
   let mut port=semantic::Decode{control,native,body};semantic::Port::work(&mut port,1,std::mem::size_of::<semantic::Projection>())?;*slot=Some(semantic::Projection::new());let frame=slot.as_mut().unwrap();semantic::project(&self.0,frame,&mut port)?;semantic::Port::work(&mut port,1,std::mem::size_of::<SqliteDatabase>())?;Ok(std::mem::replace(&mut frame.database,SqliteDatabase{tables:Vec::new()}))
  }));
  let admitted=remaining-control.allocation_remaining_bytes();let actual=owner.native().owned_bytes().checked_sub(before).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe semantic allocation receipt regressed"))?;control.admit_native_allocation_bytes(actual.checked_sub(admitted).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe semantic SQL receipt exceeded native ownership"))?)?;result
  })();result.map_err(|error|error.with_retained_progress(owner.progress()))
 }
 fn from_sqlite_database_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError>{
  let result=(||{
  if control.allocation_remaining_bytes()==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe SQL reconstruction has no original backing authority"))}
  let before=owner.native().owned_bytes();let remaining=control.allocation_remaining_bytes();let maximum=before.checked_add(remaining).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe semantic allocation ceiling overflow"))?;
  let result=owner.scoped_native(maximum,&mut |_|true,|owner|owner.receive::<semantic::Reconstruction,Self>(|slot,native,body|{
   let mut port=semantic::Encode{control,native,body};semantic::Port::work(&mut port,1,std::mem::size_of::<semantic::Reconstruction>())?;*slot=Some(semantic::Reconstruction::new());semantic::reconstruct(database,slot.as_mut().unwrap(),&mut port).map(Self)
  }));
  let admitted=remaining-control.allocation_remaining_bytes();let actual=owner.native().owned_bytes().checked_sub(before).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe semantic allocation receipt regressed"))?;control.admit_native_allocation_bytes(actual.checked_sub(admitted).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe semantic SQL receipt exceeded native ownership"))?)?;result
  })();result.map_err(|error|error.with_retained_progress(owner.progress()))
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{
  let text=match payload{store::io_schema::IoPayload::Text(text)=>text.as_str(),store::io_schema::IoPayload::Binary(bytes)=>std::str::from_utf8(bytes).map_err(|_|invalid("Probe JSON input is not UTF8"))?};
  if text.len()>control.limits().max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe input exceeds original file byte ceiling"))}
  let before=owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||invalid("Probe native allocation ceiling overflow"))?;
  let mut refused=None;
  let mut observer=|event:semio_framework_value::native_decoding::NativeDecodeProgress|match control.checkpoint(SqliteSnapshotPhase::DecodeNative,event.completed,event.total){Ok(())=>true,Err(error)=>{refused=Some(error);false}};
  let result=owner.scoped_native(maximum,&mut observer,|owner|<Self as store::ArtifactPackReceiving>::receive_pack(text.as_bytes(),owner));
  drop(observer);
  control.admit_native_allocation_bytes(owner.native().owned_bytes().checked_sub(before).ok_or_else(||invalid("Probe original allocation receipt regressed"))?)?;
  if let Some(error)=refused{Err(error)}else{result}
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<store::io_schema::IoPayload,ValueError>{
  let limits=control.limits();let before=owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||invalid("Probe native allocation ceiling overflow"))?;
  let mut refused=None;
  let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|match control.checkpoint(SqliteSnapshotPhase::EncodeNative,event.completed,event.total){Ok(())=>true,Err(error)=>{refused=Some(error);false}};
  let result=owner.scoped_native(maximum,&mut observer,|owner|super::probe_json::write(&self.0,limits.max_file_bytes,owner)).map(|text|match encoding{SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(text),SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(text.into_bytes())});
  drop(observer);
  control.admit_native_allocation_bytes(owner.native().owned_bytes().checked_sub(before).ok_or_else(||invalid("Probe original allocation receipt regressed"))?)?;
  if let Some(error)=refused{Err(error)}else{result}
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{super::probe_json::validate_with(&self.0,&mut |_|control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0))}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="🫴️native/🧪️tests/🦀️.rs"]
mod original_io;
#[cfg(test)]
#[path="🫴️semantic/🧪️tests/🦀️.rs"]
mod original_semantic;
