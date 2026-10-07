use semio_framework_value::{ValueError,ValueRefusalKind};

use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,artifact::NativeEncodingBound};
use crate::standards::v_ascii::subsets::any::schema::snapshot::{StlSnapshot, StlTriangle};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{RowWriter,Cell,FloatColumn,FloatRow};
const COORDINATES:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
fn coordinates(row: &SqliteRow) -> Result<[f64; 3], ValueError> {
    let row=FloatRow::new(row,COORDINATES)?;
    Ok([row.real(3)?, row.real(4)?, row.real(5)?])
}

impl StlSnapshot{
 fn sqlite_extent(limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{const TABLES:[(&str,usize);3]=[("stl_solid",3),("stl_facet",12),("stl_vertex",12)];let mut bytes=0usize;for statement in Self::SQLITE_SCHEMA.split(';'){bytes=bytes.checked_add(statement.trim().len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"STL schema bytes overflow"))?;}for(name,_)in TABLES{bytes=bytes.checked_add(name.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"STL schema names overflow"))?;}if bytes.max(Self::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"STL authored schema exceeds caller bytes"))}if limits.max_tables<TABLES.len()||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"STL authored relational extent exceeds caller limits"))}Ok(())}
 fn admit_sqlite_record(record:&semio_framework_dsl_record::RecordValue,limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};type Result<T>=std::result::Result<T,ValueError>;fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"STL native field differs from authored role")}fn add(left:usize,right:usize)->Result<usize>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"STL semantic bytes overflow"))}fn exact(record:&R,count:usize)->Result<()>{if record.fields.len()!=count||record.fields.keys().any(|key|usize::from(*key)>=count){return Err(invalid())}Ok(())}fn nested(value:Option<&F>)->Result<&R>{match value{Some(F::Record(record))=>Ok(record),_=>Err(invalid())}}fn text(value:Option<&F>,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize>{match value{Some(F::Text(value))=>{native.borrow_text(value.as_bytes())?;Ok(value.len())},_=>Err(invalid())}}fn point(value:Option<&F>,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize>{let record=nested(value)?;exact(record,3)?;let mut bytes=24usize;for id in 0..3{let Some(F::Float(value))=record.fields.get(&id)else{return Err(invalid())};let class=if value.is_nan(){"nan"}else if *value==f64::INFINITY{"positiveInfinity"}else if *value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};bytes=add(bytes,8+class.len()+if value.is_nan(){0}else{8})?;native.step()?;}Ok(bytes)}fn row(rows:&mut usize,bytes:&mut usize,payload:usize,limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<()>{let count=rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STL semantic rows overflow"))?;let total=add(*bytes,payload)?;if count>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"STL semantic rows exceed caller limit"))}if total>limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"STL semantic values exceed caller limit"))}native.step()?;*rows=count;*bytes=total;Ok(())}Self::sqlite_extent(limits)?;native.scoped_stage(|native|{native.begin_stage(0)?;exact(record,3)?;let schema=text(record.fields.get(&0),native)?;let name=text(record.fields.get(&1),native)?;let Some(F::List(facets))=record.fields.get(&2)else{return Err(invalid())};let mut rows=0;let mut bytes=0;row(&mut rows,&mut bytes,add(add(8,schema)?,name)?,limits,native)?;for facet in facets{let facet=nested(Some(facet))?;exact(facet,4)?;for id in 0..4{let payload=point(facet.fields.get(&id),native)?;row(&mut rows,&mut bytes,payload,limits,native)?;}}native.checkpoint()})}
 
fn preflight_sqlite_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=NativeEncodingBound::file_only(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;bound.repeated(self.solid_name.len(),6)?;for _ in &self.triangles{bound.add(256+12*1100)?;}bound.finish()}
fn write_sqlite_rows(&self,projection:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
        projection.insert("stl_solid",&[Cell::Text(&self.schema),Cell::Text(&self.solid_name)])?;
        for (ordinal, triangle) in self.triangles.iter().enumerate() {
            let ordinal = i64::try_from(ordinal).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))?;
            let facet_id = ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STL facet identifier overflow"))?;
            projection.insert_float("stl_facet",&[Cell::Integer(1),Cell::Integer(ordinal),Cell::Real(triangle.normal[0]),Cell::Real(triangle.normal[1]),Cell::Real(triangle.normal[2])],COORDINATES)?;
            for (ordinal, vertex) in triangle.vertices.iter().enumerate() {
                projection.insert_float("stl_vertex",&[Cell::Integer(facet_id),Cell::Integer(ordinal as i64),Cell::Real(vertex[0]),Cell::Real(vertex[1]),Cell::Real(vertex[2])],COORDINATES)?;
            }
        }
        Ok(())
    }
fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{Self::sqlite_extent(control.limits())?;let mut writer=RowWriter::borrowed(control,phase)?;self.write_sqlite_rows(&mut writer)?;writer.finish_borrowed()}
fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{Self::sqlite_extent(control.limits())?;let mut writer=RowWriter::new(Self::SQLITE_SCHEMA,control)?;self.write_sqlite_rows(&mut writer)?;writer.finish()}
fn reconstruct_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError>{
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        for(name,width)in[("stl_solid",3),("stl_facet",12),("stl_vertex",12)]{let mut ids=BTreeSet::new();for row in&database.table(name)?.rows{if row.values.len()!=width||row.integer(0)?!=row.rowid||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL entity shape or identity differs"));}}}
        let solid = database.table("stl_solid")?.single_row()?;
        if solid.integer(0)? != solid.rowid { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL solid identifier differs")); }
        let facet_table = database.table("stl_facet")?;
        let vertex_table = database.table("stl_vertex")?;
        let total = facet_table.rows.len().checked_add(vertex_table.rows.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STL entity count overflow"))?;
        let mut facet_ids = BTreeSet::new();
        for (ordinal, row) in facet_table.rows.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let id = row.integer(0)?;
            if id!=row.rowid || !facet_ids.insert(id) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL facet identifiers must be distinct positive integers")); }
        }
        let mut vertices = BTreeMap::<i64, Vec<&SqliteRow>>::new();
        for (ordinal, row) in vertex_table.rows.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let facet_id = row.integer(1)?;
            if !facet_ids.contains(&facet_id) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL vertex has an unknown facet")); }
            vertices.entry(facet_id).or_default().push(row);
        }
        let mut triangles = Vec::new();
        for row in facet_table.ordered_rows(2)? {
            if triangles.len() % 64 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, triangles.len() * 4, total)?; }
            if row.integer(1)? != solid.rowid { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL facet has an unknown solid")); }
            let mut owned = vertices.remove(&row.integer(0)?).unwrap_or_default();
            if owned.len() != 3 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL facets must own exactly three vertices")); }
            owned.sort_by_key(|row| row.integer(2).unwrap_or(i64::MIN));
            for (ordinal, vertex) in owned.iter().enumerate() {
                if vertex.integer(2)? != ordinal as i64 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL vertex ordinals must be 0, 1, 2")); }
            }
            triangles.push(StlTriangle { normal: coordinates(row)?, vertices: [coordinates(owned[0])?, coordinates(owned[1])?, coordinates(owned[2])?] });
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(Self { schema: solid.text(1)?.into(), solid_name: solid.text(2)?.into(), triangles })
    }
}

impl ArtifactSqliteSnapshot for StlSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{(|| -> Result<(),ValueError>{control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;control.check_rows(self.triangles.len().checked_mul(4).and_then(|count|count.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STL native row count overflow"))?)?;Ok(())})()?;self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,"stdio.stl",crate::standards::v_ascii::subsets::any::io::binary::snapshot::native_pack::spec_producer(),|native|crate::standards::v_ascii::subsets::any::io::binary::snapshot::native_pack::record_controlled(self,native),control)}
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.preflight_sqlite_encoding(_encoding,control)}

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{( || -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.stl"||dialect.standard!="ascii"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"geometry owned SQLite dialect differs"));}
        let candidate=Self::reconstruct_sqlite_database(database,control)?;
        if candidate.project_sqlite_database(control)?!=self.project_sqlite_database(control)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"STL document identity differs from its semantic projection"));}
        Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
    })().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let limits=control.limits();Self::sqlite_extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,"stdio.stl",crate::standards::v_ascii::subsets::any::io::binary::snapshot::native_pack::spec_producer(),|record,native|{Self::admit_sqlite_record(record,limits,native)?;crate::standards::v_ascii::subsets::any::io::binary::snapshot::native_pack::reconstruct_record_controlled(record,native,limits.max_rows)},control)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {self.project_sqlite_database(control)}

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> {Self::reconstruct_sqlite_database(database,control)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

