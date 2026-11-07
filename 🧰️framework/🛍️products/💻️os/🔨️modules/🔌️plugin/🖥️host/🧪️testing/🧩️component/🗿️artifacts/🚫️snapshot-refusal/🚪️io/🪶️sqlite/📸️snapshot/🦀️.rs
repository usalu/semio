//! 🛑️ Selected compiled refusal ownership with real native and individually authored relational routes.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::super::super::Snapshot;
use semio_os_kernel as store;
use semio_os_kernel::os_pack as pack;
use semio_os_kernel::sqlite_snapshot::{*,artifact::{Cell,Projection}};
use semio_framework_diagnostic::{Diagnostic,ExpectedSet,FaultCode,FaultScope,Severity,TextSpan};
const SQL:&str=include_str!("🗄️.sql");
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn selected(subset:&str)->Result<ValueRefusalKind,ValueError>{
 match subset{
  "invalid-value"=>Ok(ValueRefusalKind::InvalidValue),
  "canceled"=>Ok(ValueRefusalKind::Canceled),
  "ownership-limit"=>Ok(ValueRefusalKind::OwnershipLimit),
  "allocation-failed"=>Ok(ValueRefusalKind::AllocationFailed),
  "work-limit"=>Ok(ValueRefusalKind::WorkLimit),
  "depth-limit"=>Ok(ValueRefusalKind::DepthLimit),
  "unsupported-owner"=>Ok(ValueRefusalKind::UnsupportedOwner),
  "invariant-violated"=>Ok(ValueRefusalKind::InvariantViolated),
  _=>Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"undeclared compiled refusal subset"))
 }
}
fn refusal(kind:ValueRefusalKind)->store::io_schema::IoError{
 store::io_schema::IoError{
  cause:ValueError::new(kind,"selected typed cause\0引用😀").into(),
  diagnostics:vec![Diagnostic{
   code:FaultCode::new("fixture.snapshot\0code"),severity:Severity::Error,
   span:TextSpan{line:7,column:9,length:4},message:"diagnostic\0引用😀".into(),
   expected:Some(ExpectedSet{tokens:vec!["typed\0token".into()],keywords:vec!["word\0引用".into()],keys:vec!["count\0key".into()]}),
   scope:FaultScope{plugin_id:Some("neutral-host-fixture".into()),app_id:Some("fixture\0app".into()),instance_id:Some("lease\0instance".into()),module:Some("sqlite\0module".into()),body_key:Some("count\0body".into())}
  }]
 }
}
fn text_size(value:i32)->usize{let mut magnitude=value.unsigned_abs();let mut digits=1;while magnitude>=10{magnitude/=10;digits+=1;}10+digits+usize::from(value<0)}
fn deflate_bound(bytes:usize)->usize{(bytes*9+10).div_ceil(8)}
fn compressed_bound(raw:usize)->usize{2+20+deflate_bound(raw)+4}
fn binary_bound()->usize{
 let manifest=1+32+20+10+20+20+20+10+10+10+10;
 pack::format::HEADER_SIZE+compressed_bound(1)+compressed_bound(10)+compressed_bound(manifest)+2+10+4+pack::format::FOOTER_SIZE
}
impl store::serialization::ArtifactSqliteSnapshotHost for Snapshot{
 
 
 
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<Self,store::NativeSnapshotFailure>{
  store::native_snapshot_turn(|work|{
   c.checkpoint(SqliteSnapshotPhase::DecodeNative,0,1)?;c.check_rows(1)?;c.check_value_bytes(8)?;
   work.take(store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,output,native,_body|{*output=Some(Self::__dsl_from_record_controlled(record,native)?);Ok(())},c,native_owner))
  })
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_, '_>)->semio_framework_value::RetainedWorkOutcome<store::io_schema::IoPayload,store::NativeSnapshotFailure>{
  store::native_snapshot_turn(|work|{
   c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,1)?;c.check_rows(1)?;c.check_value_bytes(8)?;
   work.take(store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c,owner))
  })
 }
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,1)?;c.check_rows(1)?;
  if SQL.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"refusal snapshot schema exceeds caller ceiling"))}
  let bytes=match encoding{SnapshotEncoding::Text=>text_size(self.value),SnapshotEncoding::Binary=>binary_bound()};
  if bytes>c.limits().max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"refusal snapshot borrowed forecast exceeds file ceiling"))}
  if bytes>c.allocation_remaining_bytes(){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"refusal snapshot forecast exceeds caller allowance"))}
  c.check_value_bytes(8)?;
  c.checkpoint(SqliteSnapshotPhase::EncodeNative,1,1)
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  use store::io_schema::IoError;
  c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(IoError::from_value_error)?;
  if dialect.artifact_kind!=super::super::super::KIND||dialect.standard!="1"{return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"undeclared compiled refusal dialect")))}
  let row=database.table("fixture_refusal").map_err(IoError::from_value_error)?.single_row().map_err(IoError::from_value_error)?;
  if row.integer(1).map_err(IoError::from_value_error)?!=i64::from(self.value){return Err(IoError::from_value_error(invalid("refusal snapshot differs from projected value")))}
  let kind=selected(&dialect.subset).map_err(IoError::from_value_error)?;
  c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(IoError::from_value_error)?;
  Err(refusal(kind))
 }
}
impl pack::snapshot::ArtifactSqliteSnapshot for Snapshot {
const SQLITE_SCHEMA:&'static str=SQL;
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;c.check_rows(1)?;c.check_value_bytes(16)?;
  let mut out=Projection::new(SQL,c)?;out.insert_key("fixture_refusal",1,&[Cell::Integer(i64::from(self.value))])?;out.checkpoint_total(1)?;out.finish()
 }
fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,1)?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;
  validate_sqlite_database_schema_controlled(d,SQL,SqliteSnapshotPhase::ReconstructSnapshot,c)?;
  let row=d.table("fixture_refusal")?.single_row()?;
  if row.rowid!=1||row.values.len()!=2||row.integer(0)?!=1{return Err(invalid("refusal snapshot requires exact singleton identity"))}
  let value=i32::try_from(row.integer(1)?).map_err(|_|invalid("refusal snapshot value exceeds signed i32"))?;
  c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(Self{value})
 }
}
