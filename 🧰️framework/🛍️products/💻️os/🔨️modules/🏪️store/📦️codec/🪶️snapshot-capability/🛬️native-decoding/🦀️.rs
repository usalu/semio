//! 🛬️ Controlled physical record decoding for explicitly declared snapshot owners.
use semio_framework_dsl_record::{ParseOptions,RecordSpecProducer,RecordValue};
use semio_framework_io_sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use semio_framework_io_control::{NativeSnapshotDecodeOwner,NativeSnapshotFailure,native_snapshot_turn};
use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome};
use semio_framework_diagnostic::Limits;

fn admit(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,bytes:usize,work:&mut semio_framework_io_control::NativeSnapshotWork)->Result<semio_framework_value::RetainedCloneGrant,NativeSnapshotFailure>{
 let grant=owner.admit_constructed(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
 work.admit(owner.native())?;Ok(grant)
}
fn receipt(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,grant:semio_framework_value::RetainedCloneGrant,bytes:usize)->Result<(),NativeSnapshotFailure>{
 owner.record_constructed_progress(grant,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})
}

/// 🧮️ Preserves the original work, ownership and every refusal through native record reconstruction.
pub fn decode_sqlite_snapshot_record_native<T:semio_framework_value::retirement::RetireOwned>(
 payload:&semio_framework_io_schema::IoPayload,
 envelope_id:&str,
 spec:RecordSpecProducer,
 construct:&mut impl FnMut(&mut RecordValue,&mut Option<T>,&mut bool,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotDecodeOwner<'_,'_>,&mut SqliteSnapshotControl<'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>,
 output:&mut Option<T>,
 control:&mut SqliteSnapshotControl<'_>,
 native_owner:&mut NativeSnapshotDecodeOwner<'_,'_>,
)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
 native_snapshot_turn(|work|{
  let limits=control.limits();
  control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
  let length=match payload{semio_framework_io_schema::IoPayload::Binary(bytes)=>bytes.len(),semio_framework_io_schema::IoPayload::Text(text)=>text.len()};
  if length>limits.max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot input exceeds file byte limit").into())}
  let before=native_owner.native().owned_bytes();
  let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot cumulative allowance overflow"))?;
  let settlement=semio_framework_io_sqlite_snapshot::artifact::receiving::ReceivingSettlement::new(control,before,SqliteSnapshotPhase::DecodeNative);
  let mut observer=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
  let turn=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive_nested::<(Option<semio_framework_dsl_record::RecordSpec>,Option<(RecordValue,Option<pack::record::DecodeReport>)>,Option<T>,semio_framework_io_control::NativeSnapshotEffectProgress,Option<NativeSnapshotFailure>),T,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_failure,owner|native_snapshot_turn(|work|{
   type Frame<T>=(Option<semio_framework_dsl_record::RecordSpec>,Option<(RecordValue,Option<pack::record::DecodeReport>)>,Option<T>,semio_framework_io_control::NativeSnapshotEffectProgress,Option<NativeSnapshotFailure>);
   let bytes=size_of::<Frame<T>>();work.take(owner.effect(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()},_marker,_failure,&mut |performed,physical|{*slot=Some((None,None,None,Default::default(),None));*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;_marker.reset_completed()?;
   let frame=slot.as_mut().unwrap();
   let status=(||{
    let bytes=size_of::<Option<semio_framework_dsl_record::RecordSpec>>();let grant=admit(owner,bytes,work)?;
    frame.0=Some(work.measured(owner.native(),|native|spec.decode(native))?);receipt(owner,grant,bytes)?;
    let spec=frame.0.as_ref().unwrap();
    let bytes=size_of::<Option<(RecordValue,Option<pack::record::DecodeReport>)>>();let grant=admit(owner,bytes,work)?;
    frame.1=Some(match payload{
     semio_framework_io_schema::IoPayload::Binary(bytes)=>{
      let source=work.measured(owner.native(),|native|semio_os_semio::unwrap_binary_controlled(bytes,envelope_id,semio_os_semio::Component::Pack,1,native))?;
      {let (record,report)=work.measured(owner.native(),|native|pack::record::decode_document_controlled(source,spec,&pack::record::DecodeOptions::default(),native).map_err(semio_os_pack::PackRefusal::into_value_error))?;(record,Some(report))}
     },
     semio_framework_io_schema::IoPayload::Text(text)=>{
      let source=work.measured(owner.native(),|native|semio_os_semio::split_text_preamble_controlled(text,envelope_id,semio_os_semio::Component::Dsl,1,native).map_err(semio_os_semio::SemioError::into_value_error))?;
      (work.measured(owner.native(),|native|semio_framework_dsl_record::parse_exact_controlled(source,spec,&ParseOptions{limits:Limits{max_bytes:limits.max_file_bytes,..Limits::default()},mode:semio_framework_dsl_record::SourceMode::Document},native).map_err(|error|ValueError::new(error.kind,error.message)))?,None)
     }
    });receipt(owner,grant,bytes)?;
    let(_,record,retained,effect,unwind)=&mut *frame;work.take(settlement.with_control(|control|construct(&mut record.as_mut().unwrap().0,retained,&mut effect.performed,unwind,owner,control)))?;
    if frame.2.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native constructor did not retain its actual output").into())}
    Ok::<_,NativeSnapshotFailure>(())
   })();
   let settled=settlement.finish_body(owner.native().owned_bytes());
   match(status,settled){
    (Err(primary),Ok(()))=>return Err(primary),
    (Ok(()),Err(error))=>return Err(error.into()),
    (Err(primary),Err(error))=>return Err(primary.with_settlement_refusal(error).expect("record body has its original settlement slot")),
    (Ok(()),Ok(()))=>{}
   }
   if frame.2.is_none()||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native record lost its retained output").into())}
   let(_,_,retained,effect,unwind)=frame;work.take(owner.move_existing(retained,result,effect,unwind))?;Ok(())
  })));
  let turn=RetainedWorkOutcome{result:turn.result.map_err(|error|settlement.refused_failure(native_owner.native().owned_bytes(),error)),work:turn.work};
  work.take(turn)
 })
}

#[cfg(test)]
#[path="🧪️tests/🚪️control/🦀️.rs"]
mod original_control_tests;
