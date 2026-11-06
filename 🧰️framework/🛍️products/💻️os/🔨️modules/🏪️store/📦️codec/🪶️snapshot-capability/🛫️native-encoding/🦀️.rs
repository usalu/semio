//! 🛫️ Declared native record output with one caller-owned cumulative controller.
use semio_framework_dsl_record::{RecordSpecProducer,RecordValue};
use semio_framework_io_sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use semio_framework_io_control::{NativeSnapshotEncodeOwner,NativeSnapshotFailure,NativeSnapshotWork,native_snapshot_turn};
use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome};

fn admit(owner:&mut NativeSnapshotEncodeOwner<'_,'_>,bytes:usize,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let grant=owner.remaining_grant();
 if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"record encoder has no remaining item authority").into())}
 if grant.maximum_copy_bytes<bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"record encoder exceeds original copy authority").into())}
 if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"record encoder exceeds original depth authority").into())}
 work.admit_encode(owner.native())
}
fn receipt(owner:&mut NativeSnapshotEncodeOwner<'_,'_>,bytes:usize)->Result<(),NativeSnapshotFailure>{
 owner.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).map_err(Into::into)
}

/// 🧮️ Projects and emits declared records through the original encoder and retained body.
pub fn encode_sqlite_snapshot_record_native(
 encoding:SnapshotEncoding,
 envelope_id:&str,
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut Option<RecordValue>,&mut bool,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>,
 output:&mut Option<semio_framework_io_schema::IoPayload>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotEncodeOwner<'_,'_>,
)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
 native_snapshot_turn(|work|{
  let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
  let component=match encoding{SnapshotEncoding::Binary=>semio_os_semio::Component::Pack,SnapshotEncoding::Text=>semio_os_semio::Component::Dsl};
  let body_limit=limits.max_file_bytes.checked_sub(semio_os_semio::declared_envelope_prefix_len(envelope_id,component,1)?).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native file ceiling cannot contain its declared envelope"))?;
  let before=native_owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot cumulative allowance overflow"))?;
  let settlement=semio_framework_io_sqlite_snapshot::artifact::receiving::ReceivingSettlement::new(control,before,SqliteSnapshotPhase::EncodeNative);
  let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
  let turn=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive_nested::<(Option<semio_framework_dsl_record::RecordSpec>,Option<RecordValue>,Option<semio_framework_io_schema::IoPayload>,Option<semio_framework_io_schema::IoPayload>,(semio_framework_io_control::NativeSnapshotEffectProgress,Option<NativeSnapshotFailure>)),semio_framework_io_schema::IoPayload,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_failure,owner|native_snapshot_turn(|work|{
   type Frame=(Option<semio_framework_dsl_record::RecordSpec>,Option<RecordValue>,Option<semio_framework_io_schema::IoPayload>,Option<semio_framework_io_schema::IoPayload>,(semio_framework_io_control::NativeSnapshotEffectProgress,Option<NativeSnapshotFailure>));
   let bytes=size_of::<Frame>();work.take(owner.effect(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()},_marker,_failure,&mut |performed,physical|{*slot=Some((None,None,None,None,(Default::default(),None)));*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;_marker.reset_completed()?;
   let frame=slot.as_mut().unwrap();
   let status=(||{
    let bytes=size_of::<Option<semio_framework_dsl_record::RecordSpec>>();admit(owner,bytes,work)?;
    frame.0=Some(work.measured_encode(owner.native(),|native|spec.encode(native))?);receipt(owner,bytes)?;
    let(_,record,_,_,(effect,unwind))=&mut *frame;work.take(construct(record,&mut effect.performed,unwind,owner))?;
    let record=frame.1.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native encoder constructor did not retain its record"))?;
    let spec=frame.0.as_ref().unwrap();let bytes=size_of::<Option<semio_framework_io_schema::IoPayload>>();admit(owner,bytes,work)?;
    frame.2=Some(match encoding{
     SnapshotEncoding::Binary=>{let mut options=pack::record::EncodeOptions::default();options.limits.max_file_len=body_limit as u64;semio_framework_io_schema::IoPayload::Binary(work.measured_encode(owner.native(),|native|pack::record::encode_document_controlled(spec,record,&options,native).map_err(semio_os_pack::PackRefusal::into_value_error))?)},
     SnapshotEncoding::Text=>semio_framework_io_schema::IoPayload::Text(work.measured_encode(owner.native(),|native|semio_framework_dsl_record::print_controlled(record,spec,semio_framework_dsl_record::JoinMode::Document,body_limit,native).map_err(|error|ValueError::new(error.kind,error.message)))?)
    });receipt(owner,bytes)?;
    admit(owner,bytes,work)?;
    frame.3=Some(match frame.2.as_ref().unwrap(){
     semio_framework_io_schema::IoPayload::Binary(body)=>semio_framework_io_schema::IoPayload::Binary(work.measured_encode(owner.native(),|native|semio_os_semio::wrap_binary_controlled(envelope_id,component,1,body,native))?),
     semio_framework_io_schema::IoPayload::Text(body)=>semio_framework_io_schema::IoPayload::Text(work.measured_encode(owner.native(),|native|semio_os_semio::wrap_text_controlled(envelope_id,component,1,body,native))?)
    });receipt(owner,bytes)?;
    let length=match frame.3.as_ref().unwrap(){semio_framework_io_schema::IoPayload::Binary(bytes)=>bytes.len(),semio_framework_io_schema::IoPayload::Text(text)=>text.len()};
    if length>limits.max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot output exceeds file byte limit").into())}
    Ok::<(),NativeSnapshotFailure>(())
   })();
   let settled=settlement.finish_body(owner.native().owned_bytes());
   match(status,settled){
    (Err(primary),Ok(()))=>return Err(primary),
    (Ok(()),Err(error))=>return Err(error.into()),
    (Err(primary),Err(error))=>return Err(primary.with_settlement_refusal(error).expect("record body has its original settlement slot")),
    (Ok(()),Ok(()))=>{}
   }
   if frame.3.is_none()||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native record lost its retained output").into())}
   let(_,_,_,retained,(effect,unwind))=frame;work.take(owner.move_existing(retained,result,effect,unwind))?;Ok(())
  })));
  let turn=RetainedWorkOutcome{result:turn.result.map_err(|error|settlement.refused_failure(native_owner.native().owned_bytes(),error)),work:turn.work};work.take(turn)
 })
}

#[cfg(test)]
#[path="🧪️tests/🚪️control/🦀️.rs"]
mod original_control_tests;
