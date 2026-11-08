//! 🛫️ Declared native record output with one caller-owned cumulative controller.
use semio_framework_dsl_record::{RecordSpec,RecordSpecProducer,RecordValue};
use semio_framework_io_sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteStageFailure as Stage,ValueError,ValueRefusalKind};
use semio_framework_io_control::{NativeSnapshotBodyWallet,NativeSnapshotEncodeOwner,NativeSnapshotFailure,NativeSnapshotWork,native_snapshot_turn};
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome};
use semio_framework_value::native_encoding::NativeEncodeProgress;
use semio_framework_io_schema::IoPayload;
use semio_os_store_space_history::lifted_serialization::{SnapshotReceivingOutcome,SnapshotReceivingOutput};

type Frame=(Option<RecordSpec>,Option<RecordValue>,Option<IoPayload>,Option<IoPayload>);

fn admit(body:&NativeSnapshotBodyWallet,native:&mut NativeEncodeControl<'_>,work:&mut NativeSnapshotWork,bytes:usize)->Result<(),NativeSnapshotFailure>{
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;work.admit_encode(native)
}
fn receipt(body:&mut NativeSnapshotBodyWallet,bytes:usize)->Result<(),NativeSnapshotFailure>{
 body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).map_err(Into::into)
}

#[derive(Clone,Copy)]
enum Framing<'a>{Envelope(&'a str),Prefix(&'a [u8])}

/// 🧮️ Projects and emits declared records through the original encoder and retained body into the caller's output slot.
pub fn encode_sqlite_snapshot_record_native(
 encoding:SnapshotEncoding,
 envelope_id:&str,
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotEncodeOwner<'_,'_>,
 output:&mut Option<SnapshotReceivingOutput<IoPayload>>,
)->SnapshotReceivingOutcome{
 encode_framed(encoding,Framing::Envelope(envelope_id),spec,construct,control,native_owner,output)
}

/// 🏷️ Emits a binary record behind an exact owner-declared discriminator instead of a semio envelope.
pub fn encode_sqlite_snapshot_prefixed_record_native(
 prefix:&[u8],
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotEncodeOwner<'_,'_>,
 output:&mut Option<SnapshotReceivingOutput<IoPayload>>,
)->SnapshotReceivingOutcome{
 encode_framed(SnapshotEncoding::Binary,Framing::Prefix(prefix),spec,construct,control,native_owner,output)
}

fn encode_framed(
 encoding:SnapshotEncoding,
 framing:Framing<'_>,
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotEncodeOwner<'_,'_>,
 output:&mut Option<SnapshotReceivingOutput<IoPayload>>,
)->SnapshotReceivingOutcome{
 let limits=control.limits();
 let component=match encoding{SnapshotEncoding::Binary=>semio_os_semio::Component::Pack,SnapshotEncoding::Text=>semio_os_semio::Component::Dsl};
 let admitted=(||{
  if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))}
  native_owner.native().checkpoint()?;
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
  let frame_len=match framing{Framing::Envelope(envelope_id)=>semio_os_semio::declared_envelope_prefix_len(envelope_id,component,1)?,Framing::Prefix(prefix)=>prefix.len()};
  let body_limit=limits.max_file_bytes.checked_sub(frame_len).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native file ceiling cannot contain its declared envelope"))?;
  let before=native_owner.native().owned_bytes();
  let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot cumulative allowance overflow"))?;
  Ok((body_limit,before,maximum))
 })();
 let(body_limit,before,maximum)=match admitted{Ok(admitted)=>admitted,Err(error)=>return RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
 let settlement=semio_framework_io_sqlite_snapshot::artifact::receiving::ReceivingSettlement::new(control,before,SqliteSnapshotPhase::EncodeNative);
 let mut observer=|event:NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
 let turn=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Frame,SnapshotReceivingOutput<IoPayload>,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_unwind,native,body|native_snapshot_turn(|work|{
  *slot=Some((None,None,None,None));
  let frame=slot.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native record frame is absent"))?;
  let status=(||{
   let bytes=size_of::<Option<RecordSpec>>();admit(body,native,work,bytes)?;
   frame.0=Some(work.measured_encode(native,|native|spec.encode(native))?);receipt(body,bytes)?;
   work.measured_encode(native,|native|construct(native).map(|record|frame.1=Some(record)))?;
   let record_spec=frame.0.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native record spec is absent"))?;
   let record=frame.1.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native encoder constructor did not retain its record"))?;
   let bytes=size_of::<Option<IoPayload>>();admit(body,native,work,bytes)?;
   frame.2=Some(match encoding{
    SnapshotEncoding::Binary=>{let mut options=pack::record::EncodeOptions::default();options.limits.max_file_len=body_limit as u64;IoPayload::Binary(work.measured_encode(native,|native|pack::record::encode_document_controlled(record_spec,record,&options,native).map_err(semio_os_pack::PackRefusal::into_value_error))?)}
    SnapshotEncoding::Text=>IoPayload::Text(work.measured_encode(native,|native|semio_framework_dsl_record::print_controlled(record,record_spec,semio_framework_dsl_record::JoinMode::Document,body_limit,native).map_err(|error|ValueError::new(error.kind,error.message)))?)
   });receipt(body,bytes)?;
   admit(body,native,work,bytes)?;
   frame.3=Some(match(framing,frame.2.as_ref()){
    (Framing::Envelope(envelope_id),Some(IoPayload::Binary(body_bytes)))=>IoPayload::Binary(work.measured_encode(native,|native|semio_os_semio::wrap_binary_controlled(envelope_id,component,1,body_bytes,native))?),
    (Framing::Envelope(envelope_id),Some(IoPayload::Text(body_text)))=>IoPayload::Text(work.measured_encode(native,|native|semio_os_semio::wrap_text_controlled(envelope_id,component,1,body_text,native))?),
    (Framing::Prefix(prefix),Some(IoPayload::Binary(body_bytes)))=>IoPayload::Binary(work.measured_encode(native,|native|{
     let length=prefix.len().checked_add(body_bytes.len()).filter(|length|*length<=limits.max_file_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot output exceeds file byte limit"))?;
     let mut framed=native.allocate_vec::<u8>(length)?;native.begin_stage(length)?;framed.extend_from_slice(prefix);native.advance(prefix.len())?;
     for chunk in body_bytes.chunks(65536){framed.extend_from_slice(chunk);native.advance(chunk.len())?}
     Ok::<_,ValueError>(framed)
    })?),
    (Framing::Prefix(_),Some(IoPayload::Text(_)))|(_,None)=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native record body is not framable").into())
   });receipt(body,bytes)?;
   let length=match frame.3.as_ref(){Some(IoPayload::Binary(bytes))=>bytes.len(),Some(IoPayload::Text(text))=>text.len(),None=>0};
   if length>limits.max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot output exceeds file byte limit").into())}
   Ok::<(),NativeSnapshotFailure>(())
  })();
  let settled=settlement.finish_body(native.owned_bytes());
  match(status,settled){
   (Err(primary),Ok(()))=>return Err(primary),
   (Ok(()),Err(error))=>return Err(error.into()),
   (Err(primary),Err(error))=>return Err(primary.with_settlement_refusal(error).expect("record body has its original settlement slot")),
   (Ok(()),Ok(()))=>{}
  }
  let bytes=size_of::<SnapshotReceivingOutput<IoPayload>>();admit(body,native,work,bytes)?;
  let retained=frame.3.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native record lost its retained output"))?;
  *result=Some(Ok(retained));receipt(body,bytes)?;Ok(())
 })));
 RetainedWorkOutcome{result:turn.result.map_err(|error|Stage::Operation(settlement.refused_failure(native_owner.native().owned_bytes(),error))),work:turn.work}
}

#[cfg(test)]
#[path="🧪️tests/🚪️control/🦀️.rs"]
mod original_control_tests;
