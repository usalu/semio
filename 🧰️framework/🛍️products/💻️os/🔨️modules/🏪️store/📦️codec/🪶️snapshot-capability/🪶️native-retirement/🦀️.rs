//! 🫴️ Original codec intermediates stay under the caller's admitted receiving frame.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost;
use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotFailure, NativeSnapshotBodyWallet};
use semio_framework_io_schema::{IoError, IoOutcome, IoPayload, IoResult};
use semio_framework_io_sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, SnapshotEncoding};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,retirement::RetireOwned};
use crate::lifted_sqlite_snapshot_workspace::staged;

struct CodecFrame<P:RetireOwned>{pub workspaces:[semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;3],pub snapshot:Option<P>,pub database:Option<SqliteDatabase>,pub payload:Option<IoPayload>,pub validation:Option<IoResult<()>>}
impl<P:RetireOwned> RetireOwned for CodecFrame<P>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{let Self{snapshot,database,payload,validation,workspaces}=self;use semio_framework_value::retirement::{sequence,deferred};sequence(vec![deferred(snapshot),deferred(database),deferred(payload),deferred(validation),deferred(workspaces)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.snapshot),deferred_birth_bytes_for(&self.database),deferred_birth_bytes_for(&self.payload),deferred_birth_bytes_for(&self.validation),deferred_birth_bytes_for(&self.workspaces)])}
 fn controlled_retirement_supported()->bool{P::controlled_retirement_supported()&&SqliteDatabase::controlled_retirement_supported()&&IoPayload::controlled_retirement_supported()&&<IoResult<()>>::controlled_retirement_supported()&&<semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient>::controlled_retirement_supported()}
}

impl<P:RetireOwned> CodecFrame<P>{pub fn empty()->Self{Self{workspaces:std::array::from_fn(|_|semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new()),snapshot:None,database:None,payload:None,validation:None}}}

fn validation(slot:&mut Option<IoResult<()>>)->Result<(),IoError>{
 match slot.as_ref().unwrap(){
  Err(_)=>Err(match slot.take().unwrap(){Err(error)=>error,Ok(_)=>unreachable!()}),
  Ok(outcome)if outcome.diagnostics.iter().any(|diagnostic|matches!(diagnostic.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal))=>{let outcome=slot.take().unwrap().unwrap();Err(IoError{cause:ValueError::literal(ValueRefusalKind::InvalidValue,"owned snapshot violates its requested subset").into(),diagnostics:outcome.diagnostics})},
  Ok(_)=>Ok(())
 }
}


fn take<T,E:Into<NativeSnapshotFailure>>(work:&mut u64,turn:RetainedWorkOutcome<T,E>)->Result<T,NativeSnapshotFailure>{*work=work.checked_add(turn.work).expect("codec child work fits original caller ledger");turn.result.map_err(Into::into)}
fn extent_post(primary:Result<(),ValueError>,physical:Result<(),ValueError>)->Result<(),NativeSnapshotFailure>{match(primary,physical){(Ok(()),Ok(()))=>Ok(()),(Err(error),Ok(()))|(Ok(()),Err(error))=>Err(error.into()),(Err(error),Err(receipt))=>Err(NativeSnapshotFailure::from(error).with_receipt_refusal(receipt).into())}}
macro_rules! movement{
 ($name:ident,$owner:ident)=>{
 fn $name(owner:&mut $owner<'_,'_>,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,bytes:usize,effect:impl FnOnce())->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()};let mut effect=Some(effect);
  let turn=owner.effect(demand,marker,unwind_failure,&mut |performed,physical|{*performed=true;effect.take().unwrap()();*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});
  let mut turn=turn;if turn.result.is_ok(){if let Err(error)=marker.reset_completed(){turn.result=Err(error.into())}}
  turn
 }
 };
}
movement!(decode_move,NativeSnapshotDecodeOwner);
movement!(encode_move,NativeSnapshotEncodeOwner);

pub fn export<P:ArtifactSqliteSnapshotHost+RetireOwned>(_schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,payload:&IoPayload,output:&mut Option<IoResult<SqliteDatabase>>,control:&mut SqliteSnapshotControl<'_>,native:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
 native.receive_nested::<CodecFrame<P>,IoResult<SqliteDatabase>,NativeSnapshotFailure>(output,&mut |slot,produced,marker,unwind_failure,native|{
  let mut work=0;
  let result=(||{
   if slot.is_some(){return Err(ValueError::invariant("codec frame slot must be empty").into())}
   take(&mut work,decode_move(native,marker,unwind_failure,size_of::<CodecFrame<P>>(),||*slot=Some(CodecFrame::empty())))?;
   let frame=slot.as_mut().unwrap();
   control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,1)?;
   take(&mut work,staged(control,&mut frame.workspaces[0],native,&mut |control,native|P::decode_sqlite_snapshot_native(payload,&mut frame.snapshot,control,native)))?;
   control.checkpoint(SqliteSnapshotPhase::DecodeNative,1,1)?;
   take(&mut work,staged(control,&mut frame.workspaces[1],native,&mut |control,native|frame.snapshot.as_ref().unwrap().to_sqlite_database_receiving(&mut frame.database,control,native)))?;
   let mut body=NativeSnapshotBodyWallet::new(native.remaining_grant());let before=control.performed_work();
   let extent=semio_framework_io_sqlite_snapshot::artifact::receiving::check_database(frame.database.as_ref().unwrap(),&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Decode(native.native()),body:&mut body});
   work=work.checked_add(control.performed_work().checked_sub(before).expect("SQL work is monotone")).expect("codec extent work fits");
   let physical=native.record_progress(body.progress());extent_post(extent,physical)?;
   take(&mut work,staged(control,&mut frame.workspaces[2],native,&mut |control,native|frame.snapshot.as_ref().unwrap().validate_sqlite_snapshot_subset_decoding(dialect,frame.database.as_ref().unwrap(),&mut frame.validation,control,native)))?;
   if let Err(error)=validation(&mut frame.validation){*produced=Some(Err(error));return Ok(())}
   take(&mut work,decode_move(native,marker,unwind_failure,size_of::<IoOutcome<SqliteDatabase>>(),||{let diagnostics=frame.validation.take().unwrap().unwrap().diagnostics;*produced=Some(Ok(IoOutcome{value:frame.database.take().unwrap(),diagnostics}));}))?;
   Ok(())
  })();
  RetainedWorkOutcome{result,work}
 })
}

pub fn import<P:ArtifactSqliteSnapshotHost+RetireOwned>(_schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,input:&mut Option<SqliteDatabase>,encoding:SnapshotEncoding,output:&mut Option<IoResult<IoPayload>>,control:&mut SqliteSnapshotControl<'_>,native:&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
 if input.is_none(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite codec original input is absent").into()),work:0}}
 native.receive_nested::<CodecFrame<P>,IoResult<IoPayload>,NativeSnapshotFailure>(output,&mut |slot,produced,marker,unwind_failure,native|{
  let mut work=0;
  let result=(||{
   if slot.is_some(){return Err(ValueError::invariant("codec frame slot must be empty").into())}
   take(&mut work,encode_move(native,marker,unwind_failure,size_of::<CodecFrame<P>>(),||*slot=Some(CodecFrame::empty())))?;
   let frame=slot.as_mut().unwrap();
   take(&mut work,native.move_existing(input,&mut frame.database,marker,unwind_failure))?;marker.reset_completed()?;
   let mut body=NativeSnapshotBodyWallet::new(native.remaining_grant());let before=control.performed_work();
   let extent=semio_framework_io_sqlite_snapshot::artifact::receiving::check_database(frame.database.as_ref().unwrap(),&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body});
   work=work.checked_add(control.performed_work().checked_sub(before).expect("SQL work is monotone")).expect("codec extent work fits");
   let physical=native.record_progress(body.progress());extent_post(extent,physical)?;
   take(&mut work,staged(control,&mut frame.workspaces[0],native,&mut |control,native|P::from_sqlite_database_receiving(frame.database.as_ref().unwrap(),&mut frame.snapshot,control,native)))?;
   take(&mut work,staged(control,&mut frame.workspaces[1],native,&mut |control,native|frame.snapshot.as_ref().unwrap().validate_sqlite_snapshot_subset_encoding(dialect,frame.database.as_ref().unwrap(),&mut frame.validation,control,native)))?;
   if let Err(error)=validation(&mut frame.validation){*produced=Some(Err(error));return Ok(())}
   control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,1)?;
   take(&mut work,staged(control,&mut frame.workspaces[2],native,&mut |control,native|frame.snapshot.as_ref().unwrap().encode_sqlite_snapshot_native(encoding,&mut frame.payload,control,native)))?;
   control.checkpoint(SqliteSnapshotPhase::EncodeNative,1,1)?;
   take(&mut work,encode_move(native,marker,unwind_failure,size_of::<IoOutcome<IoPayload>>(),||{let diagnostics=frame.validation.take().unwrap().unwrap().diagnostics;*produced=Some(Ok(IoOutcome{value:frame.payload.take().unwrap(),diagnostics}));}))?;
   Ok(())
  })();
  RetainedWorkOutcome{result,work}
 })
}
