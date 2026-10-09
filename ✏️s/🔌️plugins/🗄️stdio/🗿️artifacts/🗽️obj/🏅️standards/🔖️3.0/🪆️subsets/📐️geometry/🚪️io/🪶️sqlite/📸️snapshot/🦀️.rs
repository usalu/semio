use semio_framework_value::{ValueError,ValueRefusalKind};

use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,artifact::NativeEncodingBound};
use crate::standards::v3_0::subsets::any::schema::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{Cell,RowWriter,FloatColumn,FloatRow};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
const TABLES:[(&str,usize);14]=[("obj_document",3),("obj_vertex",15),("obj_texcoord",12),("obj_normal",12),("obj_face",3),("obj_face_vertex",9),("obj_group",4),("obj_group_face",6),("obj_object",4),("obj_object_face",6),("obj_face_boundary",4),("obj_material_range",7),("obj_smoothing_range",7),("obj_unknown_statement",6)];
fn extent(limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{let bytes=ObjSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).filter(|sql|!sql.is_empty()).try_fold(0usize,|sum,sql|sum.checked_add(sql.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ schema byte overflow")))?;let bytes=TABLES.iter().try_fold(bytes,|sum,(name,_)|sum.checked_add(name.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ table-name byte overflow")))?;if bytes>limits.max_schema_bytes||TABLES.len()>limits.max_tables||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<2{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"OBJ copied authored schema extent exceeds caller limits"))}Ok(())}
const VERTEX_COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const TEXCOORD_COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
const NORMAL_COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
fn optional_real(row: FloatRow<'_>, column: usize) -> Result<Option<f64>, ValueError> { if row.is_null(column)? { Ok(None) } else { row.real(column).map(Some) } }
fn identity(index: usize) -> Result<i64, ValueError> { i64::try_from(index).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))?.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ identifier overflow")) }
fn resolved_source(index:u64,size:usize)->Result<Cell<'static>,ValueError>{if index<size as u64{Ok(Cell::Integer(identity(usize::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"OBJ occurrence width differs"))?)?))}else{Ok(Cell::Null)}}
fn optional_identity(index:Option<u32>,size:usize)->Result<Cell<'static>,ValueError>{match index{Some(index)=>resolved_source(u64::from(index),size),None=>Ok(Cell::Null)}}
fn optional_number(number:Option<f64>)->Cell<'static>{number.map(Cell::Real).unwrap_or(Cell::Null)}
fn append(projection:&mut RowWriter<'_,'_>,name:&str,cells:&[Cell<'_>])->Result<(),ValueError>{let columns=match name{"obj_vertex"=>VERTEX_COLUMNS,"obj_texcoord"=>TEXCOORD_COLUMNS,"obj_normal"=>NORMAL_COLUMNS,_=>&[]};projection.insert_float(name,cells,columns).map(|_|())}

fn entities<'a>(database: &'a SqliteDatabase, name: &str,document:i64) -> Result<Vec<&'a SqliteRow>, ValueError> {
    let table = database.table(name)?;
    let mut ids = BTreeSet::new();
    for row in &table.rows {
        let id = row.integer(0)?;
        if id != row.rowid || !ids.insert(id) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{name} requires distinct identifiers"))); }
        if row.integer(1)? != document { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{name} has an unknown document"))); }
    }
    table.ordered_rows(2)
}

fn indices(rows: &[&SqliteRow]) -> Result<BTreeMap<i64, usize>, ValueError> { rows.iter().enumerate().map(|(ordinal, row)| Ok((row.integer(0)?, ordinal))).collect() }
fn reference(row: &SqliteRow, column: usize, index: &BTreeMap<i64, usize>) -> Result<usize, ValueError> { index.get(&row.integer(column)?).copied().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"OBJ relationship references an unknown entity")) }
fn source32(row:&SqliteRow,column:usize)->Result<u32,ValueError>{u32::try_from(row.integer(column)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"OBJ source word must be unsigned32"))}
fn optional_source32(row:&SqliteRow,column:usize)->Result<Option<u32>,ValueError>{if row.values.get(column)==Some(&SqliteValue::Null){Ok(None)}else{source32(row,column).map(Some)}}
fn source64(row:&SqliteRow,column:usize)->Result<u64,ValueError>{Ok((u64::from(source32(row,column)?)<<32)|u64::from(source32(row,column+1)?))}
fn check_resolved(row:&SqliteRow,column:usize,index:&BTreeMap<i64,usize>,source:Option<u64>)->Result<(),ValueError>{if row.values.get(column)==Some(&SqliteValue::Null){return Ok(())}if Some(reference(row,column,index)? as u64)!=source{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ resolved relationship differs from its source index"))}Ok(())}

fn children<'a>(database: &'a SqliteDatabase, name: &str, owners: &BTreeMap<i64, usize>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<&'a SqliteRow>>, ValueError> {
    let table = database.table(name)?;
    let mut result = BTreeMap::<i64, Vec<&SqliteRow>>::new();
    let mut ids = BTreeSet::new();
    for (ordinal, row) in table.rows.iter().enumerate() {
        if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, table.rows.len())?; }
        let id = row.integer(0)?;
        if id != row.rowid || !ids.insert(id) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{name} requires distinct identifiers"))); }
        let owner = row.integer(1)?;
        if !owners.contains_key(&owner) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{name} has an unknown owner"))); }
        result.entry(owner).or_default().push(row);
    }
    for rows in result.values_mut() {
        rows.sort_by_key(|row| row.integer(2).unwrap_or(i64::MIN));
        for (ordinal, row) in rows.iter().enumerate() { if row.integer(2)? != ordinal as i64 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{name} requires contiguous ordinals"))); } }
    }
    Ok(result)
}

impl ObjSnapshot{
fn preflight_sqlite_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=NativeEncodingBound::file_only(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;for _ in &self.vertices{bound.add(256+4*1100)?;}for _ in &self.texcoords{bound.add(192+3*1100)?;}for _ in &self.normals{bound.add(192+3*1100)?;}for face in &self.faces{bound.add(64)?;for _ in &face.vertices{bound.add(192)?;}}for group in &self.groups{bound.add(128)?;bound.repeated(group.name.len(),6)?;for _ in &group.faces{bound.add(32)?;}}for object in &self.objects{bound.add(128)?;bound.repeated(object.name.len(),6)?;for _ in &object.faces{bound.add(32)?;}}if let Some(name)=&self.mtllib{bound.repeated(name.len(),6)?;}for range in &self.usemtl{bound.add(192)?;bound.repeated(range.material.len(),6)?;}for _ in &self.smoothing_groups{bound.add(128)?;}for statement in &self.unknown_statements{bound.add(192)?;bound.repeated(statement.raw.len(),6)?;}bound.finish()}
fn write_sqlite_rows(&self,database:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
        append(database, "obj_document", &[Cell::Text(&self.schema), self.mtllib.as_ref().map(|name| Cell::Text(name)).unwrap_or(Cell::Null)])?;
        for (ordinal, vertex) in self.vertices.iter().enumerate() { append(database, "obj_vertex", &[Cell::Integer(1), Cell::Integer(identity(ordinal)? - 1), Cell::Real(vertex.x), Cell::Real(vertex.y), Cell::Real(vertex.z), optional_number(vertex.w)])?; }
        for (ordinal, coordinate) in self.texcoords.iter().enumerate() { append(database, "obj_texcoord", &[Cell::Integer(1), Cell::Integer(identity(ordinal)? - 1), Cell::Real(coordinate.u), Cell::Real(coordinate.v), optional_number(coordinate.w)])?; }
        for (ordinal, normal) in self.normals.iter().enumerate() { append(database, "obj_normal", &[Cell::Integer(1), Cell::Integer(identity(ordinal)? - 1), Cell::Real(normal.x), Cell::Real(normal.y), Cell::Real(normal.z)])?; }
        for (ordinal, face) in self.faces.iter().enumerate() {
            let face_id = identity(ordinal)?;
            append(database, "obj_face", &[Cell::Integer(1), Cell::Integer(face_id - 1)])?;
            for (ordinal, vertex) in face.vertices.iter().enumerate() {append(database,"obj_face_vertex",&[Cell::Integer(face_id),Cell::Integer(identity(ordinal)?-1),Cell::Integer(i64::from(vertex.vertex)),vertex.texcoord.map(|value|Cell::Integer(i64::from(value))).unwrap_or(Cell::Null),vertex.normal.map(|value|Cell::Integer(i64::from(value))).unwrap_or(Cell::Null),resolved_source(u64::from(vertex.vertex),self.vertices.len())?,optional_identity(vertex.texcoord,self.texcoords.len())?,optional_identity(vertex.normal,self.normals.len())?])?;}
        }
        for (ordinal, group) in self.groups.iter().enumerate() {
            let group_id = identity(ordinal)?;
            append(database, "obj_group", &[Cell::Integer(1), Cell::Integer(group_id - 1), Cell::Text(&group.name)])?;
            for(ordinal,&face)in group.faces.iter().enumerate(){append(database,"obj_group_face",&[Cell::Integer(group_id),Cell::Integer(identity(ordinal)?-1),Cell::Integer((face>>32)as i64),Cell::Integer((face&0xffffffff)as i64),resolved_source(face,self.faces.len())?])?;}
        }
        for (ordinal, object) in self.objects.iter().enumerate() {
            let object_id = identity(ordinal)?;
            append(database, "obj_object", &[Cell::Integer(1), Cell::Integer(object_id - 1), Cell::Text(&object.name)])?;
            for(ordinal,&face)in object.faces.iter().enumerate(){append(database,"obj_object_face",&[Cell::Integer(object_id),Cell::Integer(identity(ordinal)?-1),Cell::Integer((face>>32)as i64),Cell::Integer((face&0xffffffff)as i64),resolved_source(face,self.faces.len())?])?;}
        }
        for ordinal in 0..=self.faces.len() { append(database, "obj_face_boundary", &[Cell::Integer(1), Cell::Integer(identity(ordinal)? - 1), if ordinal < self.faces.len() { Cell::Integer(identity(ordinal)?) } else { Cell::Null }])?; }
        for(ordinal,range)in self.usemtl.iter().enumerate(){let index=range.face_index_from;append(database,"obj_material_range",&[Cell::Integer(1),Cell::Integer(identity(ordinal)?-1),Cell::Integer((index>>32)as i64),Cell::Integer((index&0xffffffff)as i64),resolved_source(index,self.faces.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ boundary overflow"))?)?,Cell::Text(&range.material)])?;}
        for(ordinal,range)in self.smoothing_groups.iter().enumerate(){let index=range.face_index_from;append(database,"obj_smoothing_range",&[Cell::Integer(1),Cell::Integer(identity(ordinal)?-1),Cell::Integer((index>>32)as i64),Cell::Integer((index&0xffffffff)as i64),resolved_source(index,self.faces.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ boundary overflow"))?)?,range.group.map(|group|Cell::Integer(i64::from(group))).unwrap_or(Cell::Null)])?;}
        for (ordinal, statement) in self.unknown_statements.iter().enumerate() { let position = statement.line_index; append(database, "obj_unknown_statement", &[Cell::Integer(1), Cell::Integer(identity(ordinal)? - 1), Cell::Integer((position >> 32) as i64), Cell::Integer((position & u32::MAX as u64) as i64), Cell::Text(&statement.raw)])?; }
        Ok(())
    }
fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{extent(control.limits())?;let mut writer=RowWriter::borrowed(control,phase)?;self.write_sqlite_rows(&mut writer)?;writer.finish_borrowed()}
fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{extent(control.limits())?;let mut writer=RowWriter::new(Self::SQLITE_SCHEMA,control)?;self.write_sqlite_rows(&mut writer)?;writer.finish()}
fn reconstruct_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError>{
        extent(control.limits())?;
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        for(name,width)in[("obj_document",3),("obj_vertex",15),("obj_texcoord",12),("obj_normal",12),("obj_face",3),("obj_face_vertex",9),("obj_group",4),("obj_group_face",6),("obj_object",4),("obj_object_face",6),("obj_face_boundary",4),("obj_material_range",7),("obj_smoothing_range",7),("obj_unknown_statement",6)]{for row in&database.table(name)?.rows{if row.values.len()!=width{return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{name} row width differs")));}}}
        let document = database.table("obj_document")?.single_row()?;
        if document.integer(0)?!=document.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ document identity alias differs"))}let document_id=document.rowid;
        let vertex_rows = entities(database, "obj_vertex",document_id)?;
        let texcoord_rows = entities(database, "obj_texcoord",document_id)?;
        let normal_rows = entities(database, "obj_normal",document_id)?;
        let face_rows = entities(database, "obj_face",document_id)?;
        let group_rows = entities(database, "obj_group",document_id)?;
        let object_rows = entities(database, "obj_object",document_id)?;
        let boundary_rows = entities(database, "obj_face_boundary",document_id)?;
        let vertex_indices = indices(&vertex_rows)?;
        let texcoord_indices = indices(&texcoord_rows)?;
        let normal_indices = indices(&normal_rows)?;
        let face_indices = indices(&face_rows)?;
        let boundary_indices = indices(&boundary_rows)?;
        if boundary_rows.len() != face_rows.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ boundary count overflow"))? { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ requires one boundary for every face and its end")); }
        for (ordinal, boundary) in boundary_rows.iter().enumerate() {
            if ordinal < face_rows.len() { if reference(boundary, 3, &face_indices)? != ordinal { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ face boundary references the wrong face")); } }
            else if boundary.values.get(3) != Some(&SqliteValue::Null) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ final boundary must not reference a face")); }
        }
        let mut face_children = children(database, "obj_face_vertex", &face_indices, control)?;
        let mut group_children = children(database, "obj_group_face", &indices(&group_rows)?, control)?;
        let mut object_children = children(database, "obj_object_face", &indices(&object_rows)?, control)?;
        let mut snapshot = Self { schema: document.text(1)?.into(), mtllib: document.optional_text(2)?.map(str::to_owned), ..Self::default() };
        let total = database.tables.iter().map(|table| table.rows.len()).sum();
        let mut completed = 0;
        for row in vertex_rows { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; let row=FloatRow::new(row,VERTEX_COLUMNS)?; snapshot.vertices.push(ObjVertex { x: row.real(3)?, y: row.real(4)?, z: row.real(5)?, w: optional_real(row, 6)? }); }
        for row in texcoord_rows { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; let row=FloatRow::new(row,TEXCOORD_COLUMNS)?; snapshot.texcoords.push(ObjTexCoord { u: row.real(3)?, v: row.real(4)?, w: optional_real(row, 5)? }); }
        for row in normal_rows { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; let row=FloatRow::new(row,NORMAL_COLUMNS)?; snapshot.normals.push(ObjNormal { x: row.real(3)?, y: row.real(4)?, z: row.real(5)? }); }
        for row in face_rows {
            let owned = face_children.remove(&row.integer(0)?).unwrap_or_default();
            let mut vertices = Vec::new();
            for vertex in owned {
                if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
                let source=source32(vertex,3)?;let texcoord=optional_source32(vertex,4)?;let normal=optional_source32(vertex,5)?;check_resolved(vertex,6,&vertex_indices,Some(u64::from(source)))?;check_resolved(vertex,7,&texcoord_indices,texcoord.map(u64::from))?;check_resolved(vertex,8,&normal_indices,normal.map(u64::from))?;
                vertices.push(ObjFaceVertex{vertex:source,texcoord,normal});
            }
            snapshot.faces.push(ObjFace { vertices });
        }
        for row in group_rows {
            if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
            let mut faces = Vec::new();
            for member in group_children.remove(&row.integer(0)?).unwrap_or_default() { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;let index=source64(member,3)?;check_resolved(member,5,&face_indices,Some(index))?;faces.push(index);}
            snapshot.groups.push(ObjGroup { name: row.text(3)?.into(), faces });
        }
        for row in object_rows {
            if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
            let mut faces = Vec::new();
            for member in object_children.remove(&row.integer(0)?).unwrap_or_default() { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;let index=source64(member,3)?;check_resolved(member,5,&face_indices,Some(index))?;faces.push(index);}
            snapshot.objects.push(ObjObject { name: row.text(3)?.into(), faces });
        }
        for row in entities(database,"obj_material_range",document_id)?{if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,total)?;}completed+=1;let index=source64(row,3)?;check_resolved(row,5,&boundary_indices,Some(index))?;snapshot.usemtl.push(ObjUsemtlRange{face_index_from:index,material:row.text(6)?.into()});}
        for row in entities(database,"obj_smoothing_range",document_id)?{if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,total)?;}completed+=1;let index=source64(row,3)?;check_resolved(row,5,&boundary_indices,Some(index))?;let group=optional_source32(row,6)?;snapshot.smoothing_groups.push(ObjSmoothingRange{face_index_from:index,group});}
        for row in entities(database,"obj_unknown_statement",document_id)?{if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,total)?;}completed+=1;snapshot.unknown_statements.push(ObjUnknownStatement{line_index:source64(row,3)?,raw:row.text(5)?.into()});}
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(snapshot)
    }
}

impl ArtifactSqliteSnapshot for ObjSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{(|| -> Result<(),ValueError>{
        let units=self.faces.len().checked_add(self.groups.len()).and_then(|count|count.checked_add(self.objects.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native work count overflow"))?;control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,units)?;let mut rows=2usize.checked_add(self.faces.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native row count overflow"))?;
        for count in[self.vertices.len(),self.texcoords.len(),self.normals.len(),self.faces.len(),self.groups.len(),self.objects.len(),self.usemtl.len(),self.smoothing_groups.len(),self.unknown_statements.len()]{rows=rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native row count overflow"))?;}control.check_rows(rows)?;let mut completed=0usize;
        for face in&self.faces{rows=rows.checked_add(face.vertices.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native row count overflow"))?;control.check_rows(rows)?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;}}
        for group in&self.groups{rows=rows.checked_add(group.faces.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native row count overflow"))?;control.check_rows(rows)?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;}}
        for object in&self.objects{rows=rows.checked_add(object.faces.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ native row count overflow"))?;control.check_rows(rows)?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;}}control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;
        Ok(())})()?;self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,"stdio.obj",crate::standards::v3_0::subsets::any::io::binary::snapshot::native_pack::spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
    }
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let limits=control.limits();extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,"stdio.obj",crate::standards::v3_0::subsets::any::io::binary::snapshot::native_pack::spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {semantic::admit_record(record,limits,native)?;crate::standards::v3_0::subsets::any::io::binary::snapshot::native_pack::reconstruct_record_controlled(record,native,limits.max_rows)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.preflight_sqlite_encoding(_encoding,control)}

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{( || -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.obj"||dialect.standard!="3.0"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ owned SQLite dialect differs"));}
        let candidate=Self::reconstruct_sqlite_database(database,control)?;
        if candidate.project_sqlite_database(control)?!=self.project_sqlite_database(control)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OBJ document identity differs"));}
        Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
    })().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {self.project_sqlite_database(control)}

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> {Self::reconstruct_sqlite_database(database,control)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
