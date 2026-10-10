//! 🫴️ Original codec intermediates stay under the caller's admitted receiving frame.
use super::{ArtifactSqliteSnapshot,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner};
use crate::{io_schema::{IoError,IoOutcome,IoPayload,IoResult},sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding}};
use semio_framework_value::{ValueError,ValueRefusalKind,retirement::RetireOwned};

struct CodecFrame<P:RetireOwned>{snapshot:Option<P>,database:Option<SqliteDatabase>,payload:Option<IoPayload>,validation:Option<IoResult<()>>}
impl<P:RetireOwned> RetireOwned for CodecFrame<P>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{let Self{snapshot,database,payload,validation}=self;use semio_framework_value::retirement::{sequence,deferred};sequence(vec![deferred(snapshot),deferred(database),deferred(payload),deferred(validation)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.snapshot),deferred_birth_bytes_for(&self.database),deferred_birth_bytes_for(&self.payload),deferred_birth_bytes_for(&self.validation)])}
 fn controlled_retirement_supported()->bool{P::controlled_retirement_supported()&&SqliteDatabase::controlled_retirement_supported()&&IoPayload::controlled_retirement_supported()&&<IoResult<()>>::controlled_retirement_supported()}
}

impl<P:RetireOwned> CodecFrame<P>{fn empty()->Self{Self{snapshot:None,database:None,payload:None,validation:None}}}

fn validation<P:RetireOwned>(frame:&mut CodecFrame<P>)->Result<(),IoError>{
 match frame.validation.as_ref().unwrap(){
  Err(_)=>Err(match frame.validation.take().unwrap(){Err(error)=>error,Ok(_)=>unreachable!()}),
  Ok(outcome)if outcome.diagnostics.iter().any(|diagnostic|matches!(diagnostic.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal))=>{let outcome=frame.validation.take().unwrap().unwrap();Err(IoError{cause:ValueError::literal(ValueRefusalKind::InvalidValue,"owned snapshot violates its requested subset"),diagnostics:outcome.diagnostics})},
  Ok(_)=>Ok(())
 }
}

pub(super) fn export<P:ArtifactSqliteSnapshot+RetireOwned>(_schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>,native:&mut NativeSnapshotDecodeOwner<'_,'_>)->IoResult<SqliteDatabase>{
 native.receive_nested::<CodecFrame<P>,IoResult<SqliteDatabase>>(|slot,native|{
  *slot=Some(CodecFrame::empty());let frame=slot.as_mut().unwrap();
  let result=(||{
   control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,1).map_err(IoError::from_value_error)?;
   frame.snapshot=Some(P::decode_sqlite_snapshot_native(payload,control,native).map_err(IoError::from_value_error)?);
   control.checkpoint(SqliteSnapshotPhase::DecodeNative,1,1).map_err(IoError::from_value_error)?;
   frame.database=Some(frame.snapshot.as_ref().unwrap().to_sqlite_database_receiving(control,native).map_err(IoError::from_value_error)?);
   let mut body=super::NativeSnapshotBodyWallet::new(native.remaining_grant());let extent=crate::sqlite_snapshot::artifact::receiving::check_database(frame.database.as_ref().unwrap(),&mut crate::sqlite_snapshot::artifact::receiving::Port{control,native:crate::sqlite_snapshot::artifact::receiving::Direction::Decode(native.native()),body:&mut body});native.record_progress(body.progress()).map_err(IoError::from_value_error)?;extent.map_err(IoError::from_value_error)?;
   frame.validation=Some(frame.snapshot.as_ref().unwrap().validate_sqlite_snapshot_subset_decoding(dialect,frame.database.as_ref().unwrap(),control,native));
   validation(frame)?;
   let diagnostics=frame.validation.take().unwrap().unwrap().diagnostics;
   Ok(IoOutcome{value:frame.database.take().unwrap(),diagnostics})
  })();Ok(result)
 }).map_err(IoError::from_value_error)?
}

pub(super) fn import<P:ArtifactSqliteSnapshot+RetireOwned>(_schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,input:&mut Option<SqliteDatabase>,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native:&mut NativeSnapshotEncodeOwner<'_,'_>)->IoResult<IoPayload>{
 if input.is_none(){return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite codec original input is absent")))}
 native.receive_nested::<CodecFrame<P>,IoResult<IoPayload>>(|slot,native|{
  *slot=Some(CodecFrame::empty());let frame=slot.as_mut().unwrap();
  let adopted=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<SqliteDatabase>>(),..Default::default()};
  if !adopted.fits(native.remaining_grant()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"SQLite codec input adoption exceeds original grant"))}
  native.native().checkpoint()?;frame.database=input.take();native.record_progress(adopted)?;
  let result=(||{
   let mut body=super::NativeSnapshotBodyWallet::new(native.remaining_grant());let extent=crate::sqlite_snapshot::artifact::receiving::check_database(frame.database.as_ref().unwrap(),&mut crate::sqlite_snapshot::artifact::receiving::Port{control,native:crate::sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body});native.record_progress(body.progress()).map_err(IoError::from_value_error)?;extent.map_err(IoError::from_value_error)?;
   frame.snapshot=Some(P::from_sqlite_database_receiving(frame.database.as_ref().unwrap(),control,native).map_err(IoError::from_value_error)?);
   frame.validation=Some(frame.snapshot.as_ref().unwrap().validate_sqlite_snapshot_subset_encoding(dialect,frame.database.as_ref().unwrap(),control,native));
   validation(frame)?;
   control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,1).map_err(IoError::from_value_error)?;
   frame.payload=Some(frame.snapshot.as_ref().unwrap().encode_sqlite_snapshot_native(encoding,control,native).map_err(IoError::from_value_error)?);
   control.checkpoint(SqliteSnapshotPhase::EncodeNative,1,1).map_err(IoError::from_value_error)?;
   let diagnostics=frame.validation.take().unwrap().unwrap().diagnostics;
   Ok(IoOutcome{value:frame.payload.take().unwrap(),diagnostics})
  })();Ok(result)
 }).map_err(IoError::from_value_error)?
}
