//! 💰️ Complete typed BMP semantic row ownership.
use super::BmpSnapshot;
use crate::schema::snapshot::{BmpImage,BmpNativeSample,BmpPaletteEntry,BmpPixels,BmpProfile,BmpRowOrder};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteTable,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,validate_sqlite_database_schema_controlled};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{Cell,RowWriter,reconstruct_text,ordered_row_refs};
use semio_framework_value::NativeDecodeControl;
type Result<T> = std::result::Result<T,ValueError>;
const SQL: &str = include_str!("../🗄️.sql");
const PROJECT: SqliteSnapshotPhase = SqliteSnapshotPhase::ProjectSnapshot;
const RECONSTRUCT: SqliteSnapshotPhase = SqliteSnapshotPhase::ReconstructSnapshot;
#[path="📤️projection/🦀️.rs"] mod projection;
#[path="📥️reconstruction/🦀️.rs"] mod reconstruction;
pub(super) use projection::project;
pub(super) use reconstruction::reconstruct;
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue,message) }
fn ordinal(index: usize) -> Result<i64> { i64::try_from(index).map_err(|_|invalid("BMP ordinal exceeds INTEGER")) }
fn uint(row: &SqliteRow, column: usize, maximum: u32) -> Result<u32> { let value = row.integer(column)?; u32::try_from(value).ok().filter(|value| *value <= maximum).ok_or_else(||invalid("BMP integer exceeds its native precision")) }
fn signed(row: &SqliteRow, column: usize) -> Result<i32> { i32::try_from(row.integer(column)?).map_err(|_|invalid("BMP resolution exceeds signed native precision")) }
fn ordered<'a>(table: &'a SqliteTable, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>> { let rows = ordered_row_refs(table,2,control)?; for row in &rows { if row.integer(1)? != 1 { return Err(invalid("BMP sample belongs to another image")); } } Ok(rows) }
fn allocate<T>(count: usize, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<T>> { control.allocation_stage(RECONSTRUCT,|remaining,checkpoint,allocation|{ let mut progress = |event:semio_framework_value::native_decoding::NativeDecodeProgress| checkpoint(event.completed,event.total); let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut progress,&mut native_allocation); let result = native.allocate_vec(count); (result,native.owned_bytes()) })? }
pub(super) fn admit(snapshot:&BmpSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()> {
 snapshot.validate().map_err(invalid)?;let limits=control.limits();
 let schema_bytes=SQL.split(';').filter(|statement|!statement.trim().is_empty()).map(|statement|{let statement=statement.trim();statement.len()+statement.split_whitespace().nth(2).unwrap_or("").len()}).sum::<usize>().max(SQL.len());
 if schema_bytes>limits.max_schema_bytes {return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Bmp authored schema exceeds caller bytes"));}
 if limits.max_tables<6||limits.max_columns<16 {return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Bmp authored tables or columns exceed caller extent"));}
 let mut output=RowWriter::borrowed(control,phase)?;projection::write(snapshot,&mut output)?;output.finish_borrowed()
}
