//! ☁️ Handwritten full LAS domain projection and exact typed reconstruction.

use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,artifact::NativeEncodingBound};
use crate::standards::v1_0::subsets::any::schema::snapshot::{LasSnapshot,LasHeader,LasVlr,LasPoint};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,io_schema::{ArtifactDialect,IoError,IoOutcome},sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Cell,RowWriter,FloatColumn,FloatRow}}};
#[path="💰️backing/🦀️.rs"]
mod backing;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
const HEADER_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(16),FloatColumn::Binary64(17),FloatColumn::Binary64(18),FloatColumn::Binary64(19),FloatColumn::Binary64(20),FloatColumn::Binary64(21),FloatColumn::Binary64(22),FloatColumn::Binary64(23),FloatColumn::Binary64(24),FloatColumn::Binary64(25)];
const POINT_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
const GPS_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(1)];
fn scalar_bytes(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn identity(row:&SqliteRow)->Result<i64, ValueError>{let id=row.integer(0)?;if id<1||id!=row.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "LAS entity identity is invalid"));}Ok(id)}
fn document(row:&SqliteRow)->Result<(), ValueError>{if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "LAS entity has an unknown document"));}Ok(())}
fn u8_at(row:&SqliteRow,index:usize)->Result<u8, ValueError>{u8::try_from(row.integer(index)?).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "LAS integer exceeds its native width"))}
fn u16_at(row:&SqliteRow,index:usize)->Result<u16, ValueError>{u16::try_from(row.integer(index)?).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "LAS integer exceeds its native width"))}
fn u32_at(row:&SqliteRow,index:usize)->Result<u32, ValueError>{u32::try_from(row.integer(index)?).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "LAS integer exceeds its native width"))}
fn boolean(row:&SqliteRow,index:usize)->Result<bool, ValueError>{match row.integer(index)?{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, "LAS boolean requires 0 or 1"))}}

use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};
fn visit_rows(snapshot:&LasSnapshot,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let h=&snapshot.header;p.insert("las_document",&[Cell::Text(&snapshot.schema)])?;
        p.insert_float("las_header",&[Cell::Integer(1),Cell::Integer(h.version_major.into()),Cell::Integer(h.version_minor.into()),Cell::Text(&h.system_identifier),Cell::Text(&h.generating_software),Cell::Integer(h.creation_day_of_year.into()),Cell::Integer(h.creation_year.into()),Cell::Integer(h.header_size.into()),Cell::Integer(h.offset_to_point_data.into()),Cell::Integer(h.number_of_vlrs.into()),Cell::Integer(h.point_data_format_id.into()),Cell::Integer(h.point_data_record_length.into()),Cell::Integer(h.number_of_point_records.into()),Cell::Real(h.x_scale),Cell::Real(h.y_scale),Cell::Real(h.z_scale),Cell::Real(h.x_offset),Cell::Real(h.y_offset),Cell::Real(h.z_offset),Cell::Real(h.max_x),Cell::Real(h.min_x),Cell::Real(h.max_y),Cell::Real(h.min_y),Cell::Real(h.max_z),Cell::Real(h.min_z)],HEADER_FLOATS)?;
        for(ordinal,count)in h.points_by_return.iter().enumerate(){p.insert("las_return_histogram",&[Cell::Integer(1),Cell::Integer(ordinal as i64),Cell::Integer((*count).into())])?;}
        for(ordinal,v)in snapshot.vlrs.iter().enumerate(){let id=p.insert("las_vlr",&[Cell::Integer(1),Cell::Integer(ordinal as i64),Cell::Text(&v.user_id),Cell::Integer(v.record_id.into()),Cell::Text(&v.description)])?;for(ordinal,value)in v.data.iter().enumerate(){p.insert("las_vlr_octet",&[Cell::Integer(id),Cell::Integer(ordinal as i64),Cell::Integer((*value).into())])?;}}
        for(ordinal,v)in snapshot.points.iter().enumerate(){let id=p.insert_float("las_point",&[Cell::Integer(1),Cell::Integer(ordinal as i64),Cell::Real(v.x),Cell::Real(v.y),Cell::Real(v.z),Cell::Integer(v.intensity.into()),Cell::Integer(v.return_number.into()),Cell::Integer(v.number_of_returns.into()),Cell::Integer(v.scan_direction_flag.into()),Cell::Integer(v.edge_of_flight_line.into()),Cell::Integer(v.classification.into()),Cell::Integer(v.scan_angle_rank.into()),Cell::Integer(v.user_data.into()),Cell::Integer(v.point_source_id.into())],POINT_FLOATS)?;if let Some(time)=v.gps_time{p.insert_key_float("las_point_gps",id,&[Cell::Real(time)],GPS_FLOATS)?;}if let Some((red,green,blue))=v.rgb{p.insert_key("las_point_rgb",id,&[Cell::Integer(red.into()),Cell::Integer(green.into()),Cell::Integer(blue.into())])?;}}
Ok(())}
impl LasSnapshot{pub(in crate::standards::v1_0::subsets::any::schema::snapshot::super)fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{semantic::extent(control.limits())?;let mut p=RowWriter::borrowed(control,phase)?;visit_rows(self,&mut p)?;p.finish_borrowed()}pub(in crate::standards::v1_0::subsets::any::schema::snapshot::super)fn admit_sqlite_record(value:&semio_framework_dsl_record::RecordValue,limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{semantic::admit_record(value,limits,native)}}
impl ArtifactSqliteSnapshot for LasSnapshot{
    fn decode_sqlite_snapshot_native(payload:&semio_framework_os_kernel::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1_0::subsets::any::schema::snapshot::pack::native::decode(payload,control)}
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_os_kernel::io_schema::IoPayload,ValueError>{self.preflight_sqlite_snapshot_encoding(encoding,control)?;crate::standards::v1_0::subsets::any::schema::snapshot::pack::native::encode(self,encoding,control)}
    fn retire_sqlite_snapshot(self){crate::standards::v1_0::subsets::any::schema::snapshot::pack::native::retire(self)}
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
        self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=NativeEncodingBound::file_only(control)?;bound.add(4096)?;bound.repeated(self.schema.len(),6)?;bound.repeated(self.header.system_identifier.len(),6)?;bound.repeated(self.header.generating_software.len(),6)?;for record in &self.vlrs{bound.add(256)?;bound.repeated(record.user_id.len(),6)?;bound.repeated(record.description.len(),6)?;bound.repeated(record.data.len(),8)?;}for _ in &self.points{bound.add(1024)?;}bound.finish()
        
    }
    const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
    fn validate_sqlite_snapshot_subset(&self,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<IoOutcome<()>,IoError>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(IoError::from_value_error)?;if dialect.artifact_kind!="s.stdio.las"||dialect.standard!="1.0"||dialect.subset!="*"{return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"LAS owned SQLite dialect differs")));}let row=database.table("las_document").map_err(IoError::from_value_error)?.single_row().map_err(IoError::from_value_error)?;if row.rowid!=1||row.integer(0).map_err(IoError::from_value_error)?!=1||row.text(1).map_err(IoError::from_value_error)?!=self.schema{return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"LAS document identity differs from its semantic projection")));}Ok(IoOutcome::clean(()))}
    fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::extent(control.limits())?;let mut p=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut p)?;p.finish()}
    fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{semantic::extent(control.limits())?;backing::reconstruct(database,control)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

