//! 🛬️ Controlled physical record decoding for explicitly declared snapshot owners.
use semio_framework_dsl_record::{ParseOptions,RecordSpec,RecordSpecProducer,RecordValue};
use semio_framework_io_sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SqliteStageFailure as Stage,ValueError,ValueRefusalKind};
use semio_framework_io_control::{NativeSnapshotConstruction,NativeSnapshotDecodeOwner,NativeSnapshotFailure,NativeSnapshotWork,native_snapshot_turn};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeProgress};
use semio_framework_diagnostic::Limits;
use semio_os_store_space_history::lifted_serialization::{SnapshotReceivingOutcome,SnapshotReceivingOutput};

type Frame<T>=(Option<RecordSpec>,Option<(RecordValue,Option<pack::record::DecodeReport>)>,Option<T>);

#[derive(Clone,Copy)]
enum Framing<'a>{Envelope(&'a str),Prefix(&'a [u8])}

fn admit(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,bytes:usize,work:&mut NativeSnapshotWork)->Result<NativeSnapshotConstruction,NativeSnapshotFailure>{
 let grant=owner.admit_constructed(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
 work.admit(owner.native())?;Ok(grant)
}
fn receipt(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,grant:NativeSnapshotConstruction,bytes:usize)->Result<(),NativeSnapshotFailure>{
 owner.record_constructed_progress(grant,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})
}

/// 🧮️ Reconstructs a record with a constructor that only needs the cumulative native decoder.
pub fn decode_sqlite_snapshot_record_native<T:semio_framework_value::retirement::RetireOwned>(
 payload:&semio_framework_io_schema::IoPayload,
 envelope_id:&str,
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut RecordValue,&mut Option<T>,&mut NativeDecodeControl<'_>)->Result<(),ValueError>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotDecodeOwner<'_,'_>,
 output:&mut Option<SnapshotReceivingOutput<T>>,
)->SnapshotReceivingOutcome{
 decode_sqlite_snapshot_record_receiving(payload,envelope_id,spec,&mut |record,retained,owner,_control|native_snapshot_turn(|work|work.measured(owner.native(),|native|construct(record,retained,native))),control,native_owner,output)
}

/// 🧮️ Reconstructs a record with a constructor that receives the original owner and the SQL control, preserving its work, ownership and every refusal.
pub fn decode_sqlite_snapshot_record_receiving<T:semio_framework_value::retirement::RetireOwned>(
 payload:&semio_framework_io_schema::IoPayload,
 envelope_id:&str,
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut RecordValue,&mut Option<T>,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut SqliteSnapshotControl<'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotDecodeOwner<'_,'_>,
 output:&mut Option<SnapshotReceivingOutput<T>>,
)->SnapshotReceivingOutcome{
 decode_framed(payload,Framing::Envelope(envelope_id),spec,construct,control,native_owner,output)
}

/// 🏷️ Decodes a binary record behind an exact owner-declared discriminator instead of a semio envelope.
pub fn decode_sqlite_snapshot_prefixed_record_native<T:semio_framework_value::retirement::RetireOwned>(
 payload:&semio_framework_io_schema::IoPayload,
 prefix:&[u8],
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut RecordValue,&mut Option<T>,&mut NativeDecodeControl<'_>)->Result<(),ValueError>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotDecodeOwner<'_,'_>,
 output:&mut Option<SnapshotReceivingOutput<T>>,
)->SnapshotReceivingOutcome{
 decode_framed(payload,Framing::Prefix(prefix),spec,&mut |record,retained,owner,_control|native_snapshot_turn(|work|work.measured(owner.native(),|native|construct(record,retained,native))),control,native_owner,output)
}

fn decode_framed<T:semio_framework_value::retirement::RetireOwned>(
 payload:&semio_framework_io_schema::IoPayload,
 framing:Framing<'_>,
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut RecordValue,&mut Option<T>,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut SqliteSnapshotControl<'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotDecodeOwner<'_,'_>,
 output:&mut Option<SnapshotReceivingOutput<T>>,
)->SnapshotReceivingOutcome{
 let limits=control.limits();
 let admitted=(||{
  if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))}
  native_owner.native().checkpoint()?;
  control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
  let length=match payload{semio_framework_io_schema::IoPayload::Binary(bytes)=>bytes.len(),semio_framework_io_schema::IoPayload::Text(text)=>text.len()};
  if length>limits.max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot input exceeds file byte limit"))}
  let before=native_owner.native().owned_bytes();
  let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot cumulative allowance overflow"))?;
  Ok((before,maximum))
 })();
 let(before,maximum)=match admitted{Ok(admitted)=>admitted,Err(error)=>return RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
 let settlement=semio_framework_io_sqlite_snapshot::artifact::receiving::ReceivingSettlement::new(control,before,SqliteSnapshotPhase::DecodeNative);
 let mut observer=|event:NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
 let turn=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive_nested::<Frame<T>,SnapshotReceivingOutput<T>,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_unwind,owner|native_snapshot_turn(|work|{
  *slot=Some((None,None,None));
  let frame=slot.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native record frame is absent"))?;
  let status=(||{
   let bytes=size_of::<Option<RecordSpec>>();let grant=admit(owner,bytes,work)?;
   frame.0=Some(work.measured(owner.native(),|native|spec.decode(native))?);receipt(owner,grant,bytes)?;
   let record_spec=frame.0.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native record spec is absent"))?;
   let bytes=size_of::<Option<(RecordValue,Option<pack::record::DecodeReport>)>>();let grant=admit(owner,bytes,work)?;
   frame.1=Some(match payload{
    semio_framework_io_schema::IoPayload::Binary(bytes)=>{
     let source=match framing{
      Framing::Envelope(envelope_id)=>work.measured(owner.native(),|native|semio_os_semio::unwrap_binary_controlled(bytes,envelope_id,semio_os_semio::Component::Pack,1,native))?,
      Framing::Prefix(prefix)=>bytes.strip_prefix(prefix).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"native snapshot discriminator mismatch"))?
     };
     let(record,report)=work.measured(owner.native(),|native|pack::record::decode_document_controlled(source,record_spec,&pack::record::DecodeOptions::default(),native).map_err(semio_os_pack::PackRefusal::into_value_error))?;(record,Some(report))
    },
    semio_framework_io_schema::IoPayload::Text(text)=>{
     let Framing::Envelope(envelope_id)=framing else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native snapshot discriminator frames only binary payloads").into())};
     let source=work.measured(owner.native(),|native|semio_os_semio::split_text_preamble_controlled(text,envelope_id,semio_os_semio::Component::Dsl,1,native).map_err(semio_os_semio::SemioError::into_value_error))?;
     (work.measured(owner.native(),|native|semio_framework_dsl_record::parse_exact_controlled(source,record_spec,&ParseOptions{limits:Limits{max_bytes:limits.max_file_bytes,..Limits::default()},mode:semio_framework_dsl_record::SourceMode::Document},native).map_err(|error|ValueError::new(error.kind,error.message)))?,None)
    }
   });receipt(owner,grant,bytes)?;
   let(_,record,retained)=&mut *frame;
   let record=record.as_mut().map(|record|&mut record.0).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native record is absent"))?;
   work.take(settlement.with_control(|control|construct(record,retained,owner,control)))?;
   if retained.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native constructor did not retain its actual output").into())}
   Ok::<(),NativeSnapshotFailure>(())
  })();
  let settled=settlement.finish_body(owner.native().owned_bytes());
  match(status,settled){
   (Err(primary),Ok(()))=>return Err(primary),
   (Ok(()),Err(error))=>return Err(error.into()),
   (Err(primary),Err(error))=>return Err(primary.with_settlement_refusal(error).expect("record body has its original settlement slot")),
   (Ok(()),Ok(()))=>{}
  }
  let bytes=size_of::<SnapshotReceivingOutput<T>>();let grant=admit(owner,bytes,work)?;
  let retained=frame.2.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native record lost its retained output"))?;
  *result=Some(Ok(retained));receipt(owner,grant,bytes)?;Ok(())
 })));
 RetainedWorkOutcome{result:turn.result.map_err(|error|Stage::Operation(settlement.refused_failure(native_owner.native().owned_bytes(),error))),work:turn.work}
}

#[cfg(test)]
#[path="🧪️tests/🚪️control/🦀️.rs"]
mod original_control_tests;
