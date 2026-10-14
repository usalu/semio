//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v_rfc1950::subsets::any::schema::snapshot::{DeflateSnapshot,DeflateLevelHint};
use semio_framework_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,ValueError,ValueRefusalKind,artifact::RowWriter}};
#[path="💰️backing/🦀️.rs"] mod backing;
#[path="🫴️receiving/🦀️.rs"] mod receiving;
impl ArtifactSqliteSnapshotHost for DeflateSnapshot{
 fn to_sqlite_database_receiving(&self,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{receiving::project_receiving(Some(self),None,control,owner)}
 fn from_sqlite_database_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError>{receiving::reconstruct_receiving(None,Some(database),control,owner)}
 fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner<'_,'_>)->store::io_schema::IoResult<()>{(||{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.deflate"||dialect.standard!="rfc1950"||dialect.subset!="*"{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Deflate owned SQLite dialect differs"))}receiving::validate_decoding(Some(self),Some(database),control,owner)})().map(|()|store::io_schema::IoOutcome::clean(())).map_err(|error|store::io_schema::IoError::from_value_error(error.with_retained_progress(owner.progress())))}
 fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>)->store::io_schema::IoResult<()>{(||{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.deflate"||dialect.standard!="rfc1950"||dialect.subset!="*"{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Deflate owned SQLite dialect differs"))}receiving::validate_encoding(Some(self),Some(database),control,owner)})().map(|()|store::io_schema::IoOutcome::clean(())).map_err(|error|store::io_schema::IoError::from_value_error(error.with_retained_progress(owner.progress())))}
 fn validate_sqlite_snapshot_subset(&self,_dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,_control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{Err(store::io_schema::IoError::from_value_error(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Deflate subset validation requires supplied directional owner")))}
 
 
 
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{native::decode(payload,control,native_control.native())}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{native::encode(self,encoding,control,native_owner)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}
impl pack::snapshot::ArtifactSqliteSnapshot for DeflateSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
}

/// 🧮️ Checks the actual authored relational metadata without allocating its SQL recipe.
pub(super)fn admit_layout(limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 const WIDTHS:[(&str,usize);2]=[("deflate_document",6),("deflate_payload_byte",4)];
 let mut schema=0usize;let mut count=0usize;for statement in DeflateSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).filter(|statement|!statement.is_empty()){let(name,width)=WIDTHS.get(count).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Deflate authored table extent changed"))?;if *width>limits.max_columns{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Deflate columns exceed caller limit"))}schema=schema.checked_add(statement.len()).and_then(|value|value.checked_add(name.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate schema extent overflow"))?;count+=1;}
 if count!=WIDTHS.len(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Deflate authored table extent changed"))}if schema>limits.max_schema_bytes||DeflateSnapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes||count>limits.max_tables||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Deflate relational metadata exceeds caller limit"))}Ok(())
}
/// 🫳️ Visits all actual document and payload cells through the original semantic control.
pub(super)fn admit_values(snapshot:&DeflateSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{admit_layout(control.limits())?;let mut output=RowWriter::borrowed(control,phase)?;backing::visit_rows(snapshot,&mut output)}
/// 🫧️ Bounds measured RFC1950 expansion before the final payload allocation.
pub(super)fn native_payload_capacity(limits:SqliteDatabaseLimits)->Result<usize,ValueError>{admit_layout(limits)?;let header=32usize.checked_add(crate::STDIO_DEFLATE_DOCUMENT_SCHEMA.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate document cell extent overflow"))?;let payload=limits.max_value_bytes.checked_sub(header).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate document values exceed caller limit"))?;Ok(payload/32)}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
