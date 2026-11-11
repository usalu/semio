//! 🪶️ Probe values are relational syntax nodes with explicit collection ordinals.

use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::DslValue;
use semio_framework_value::Number;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use store::sqlite_snapshot::SnapshotEncoding;
use store::sqlite_snapshot::SqliteStageFailure as Stage;
use store::sqlite_snapshot::artifact::receiving::ReceivingSettlement;
use store::os_store::serialization::{SnapshotReceivingOutcome,SnapshotReceivingOutput};
use store::sqlite_snapshot::SqliteDatabase;
use store::sqlite_snapshot::SqliteSnapshotControl;
use store::sqlite_snapshot::SqliteSnapshotPhase;
use crate::semantic;
use crate::probe_document::ProbeSnapshot;
use crate::probe_json::{validate_with, write};

fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

pub(crate) struct NumberWord{pub bytes:[u8;32],pub length:usize}

impl NumberWord{pub fn text(&self)->&str{std::str::from_utf8(&self.bytes[..self.length]).expect("formatted Probe numeric ASCII")}}

impl std::fmt::Write for NumberWord{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}}

pub(crate) fn number_text(number:Number)->Result<NumberWord,ValueError>{
 use std::fmt::Write as _;let mut word=NumberWord{bytes:[0;32],length:0};match number{Number::UInt(value)=>write!(&mut word,"{value}"),Number::Int(value)=>write!(&mut word,"{value}"),Number::Float(value)if value.is_finite()=>semio_framework_pack_json_float::write_float_to(value,&mut word),_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number is not finite"))}.map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe numeric word exceeded bounded scalar storage"))?;Ok(word)
}

pub(crate) fn number_value<P:semantic::Port>(text:&str,scanner:&mut Option<semio_framework_pack_json::JsonNumberCursor>,port:&mut P)->Result<DslValue,ValueError>{
 port.control(|control|{control.check_value_bytes(text.len())?;control.admit_reconstruction_bytes(text.len())})?;
 if !matches!(text.as_bytes().first(),Some(b'-'|b'0'..=b'9')){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number column requires an exact JSON numeric word"))}
 port.work(1,std::mem::size_of::<semio_framework_pack_json::JsonNumberCursor>())?;*scanner=Some(semio_framework_pack_json::JsonNumberCursor::new());
 loop{port.work(1,0)?;let cursor=scanner.as_mut().unwrap();let number=cursor.step(text).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number column must be a finite JSON numeric literal"))?;if let Some(number)=number{if cursor.position()!=text.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe number column has trailing numeric bytes"))}return Ok(DslValue::Number(match number{semio_framework_pack_json::Number::UInt(value)=>Number::UInt(value),semio_framework_pack_json::Number::Int(value)=>Number::Int(value),semio_framework_pack_json::Number::Float(value)=>Number::Float(value)}));}}
}

impl store::os_store::serialization::ArtifactSqliteSnapshotHost for ProbeSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<Self>>)->SnapshotReceivingOutcome{
  let admitted=(||->Result<_,ValueError>{
   if output.is_some(){return Err(invalid("Probe original snapshot output slot is occupied"))}
   owner.native().checkpoint()?;control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
   let text=match payload{store::io_schema::IoPayload::Text(text)=>text.as_str(),store::io_schema::IoPayload::Binary(bytes)=>std::str::from_utf8(bytes).map_err(|_|invalid("Probe JSON input is not UTF8"))?};
   if text.len()>control.limits().max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe input exceeds original file byte ceiling"))}
   let before=owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||invalid("Probe native allocation ceiling overflow"))?;
   Ok((text,before,maximum))
  })();
  let(text,before,maximum)=match admitted{Ok(admitted)=>admitted,Err(error)=>return semio_framework_value::RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
  let settlement=ReceivingSettlement::new(control,before,SqliteSnapshotPhase::DecodeNative);
  let mut observer=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
  let turn=owner.scoped_native(maximum,&mut observer,|owner|owner.receive_nested::<Self,SnapshotReceivingOutput<Self>,store::NativeSnapshotFailure>(output,&mut |slot,result,_marker,_unwind,owner|store::native_snapshot_turn(|work|{
   let status=work.take(<Self as store::ArtifactPackReceiving>::receive_pack(text.as_bytes(),slot,owner));
   match(status,settlement.finish_body(owner.native().owned_bytes())){
    (Err(primary),Ok(()))=>return Err(primary),
    (Ok(()),Err(error))=>return Err(error.into()),
    (Err(primary),Err(error))=>return Err(primary.with_settlement_refusal(error).expect("probe receiving body has its original settlement slot")),
    (Ok(()),Ok(()))=>{}
   }
   let bytes=std::mem::size_of::<SnapshotReceivingOutput<Self>>();let grant=owner.admit_constructed(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;work.admit(owner.native())?;
   let received=slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe receiving lost its retained snapshot"))?;
   *result=Some(Ok(received));
   owner.record_constructed_progress(grant,semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(())
  })));
  semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(|error|Stage::Operation(settlement.refused_failure(owner.native().owned_bytes(),error))),work:turn.work}
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<store::io_schema::IoPayload>>)->SnapshotReceivingOutcome{
  let limits=control.limits();
  let admitted=(||->Result<_,ValueError>{
   if output.is_some(){return Err(invalid("Probe original snapshot output slot is occupied"))}
   owner.native().checkpoint()?;control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
   let before=owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||invalid("Probe native allocation ceiling overflow"))?;
   Ok((before,maximum))
  })();
  let(before,maximum)=match admitted{Ok(admitted)=>admitted,Err(error)=>return semio_framework_value::RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
  let settlement=ReceivingSettlement::new(control,before,SqliteSnapshotPhase::EncodeNative);
  let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
  let turn=owner.scoped_native(maximum,&mut observer,|owner|owner.receive_nested::<String,SnapshotReceivingOutput<store::io_schema::IoPayload>,store::NativeSnapshotFailure>(output,&mut |slot,result,_marker,_unwind,owner|store::native_snapshot_turn(|work|{
   let status=work.take(write(&self.0,limits.max_file_bytes,slot,owner));
   match(status,settlement.finish_body(owner.native().owned_bytes())){
    (Err(primary),Ok(()))=>return Err(primary),
    (Ok(()),Err(error))=>return Err(error.into()),
    (Err(primary),Err(error))=>return Err(primary.with_settlement_refusal(error).expect("probe encoding body has its original settlement slot")),
    (Ok(()),Ok(()))=>{}
   }
   let bytes=std::mem::size_of::<SnapshotReceivingOutput<store::io_schema::IoPayload>>();
   work.admit_encode(owner.native())?;
   let text=slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe encoding lost its retained text"))?;
   *result=Some(Ok(match encoding{SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(text),SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(text.into_bytes())}));
   owner.record_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(())
  })));
  semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(|error|Stage::Operation(settlement.refused_failure(owner.native().owned_bytes(),error))),work:turn.work}
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{validate_with(&self.0,&mut |_|control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0))}
}

impl pack::snapshot::ArtifactSqliteSnapshot for ProbeSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut frame=semantic::Projection::new();let mut port=semantic::Direct{control,phase:SqliteSnapshotPhase::ProjectSnapshot};semantic::project(&self.0,&mut frame,&mut port)?;Ok(frame.database)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let mut frame=semantic::Reconstruction::new();let mut port=semantic::Direct{control,phase:SqliteSnapshotPhase::ReconstructSnapshot};semantic::reconstruct(database,&mut frame,&mut port).map(Self)}
}
