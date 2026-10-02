
use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,artifact::NativeEncodingBound};
use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{Cell,Projection,FloatColumn,FloatRow,insert_ieee754};
const VERTEX_COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const TEXCOORD_COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
const NORMAL_COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
fn scalar_bytes(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}

fn account(control: &mut SqliteSnapshotControl<'_>, budget: &mut (usize, usize), bytes: usize) -> Result<(), String> {
    budget.0 = budget.0.checked_add(1).ok_or("OBJ entity count overflow")?;
    budget.1 = budget.1.checked_add(bytes).ok_or("OBJ value size overflow")?;
    control.check_rows(budget.0)?;
    control.check_value_bytes(budget.1)?;
    if budget.0 % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, budget.0)?; }
    Ok(())
}

fn optional_real(row: FloatRow<'_>, column: usize) -> Result<Option<f64>, String> { if row.is_null(column)? { Ok(None) } else { row.real(column).map(Some) } }
fn identity(index: usize) -> Result<i64, String> { i64::try_from(index).map_err(|error| error.to_string())?.checked_add(1).ok_or_else(|| "OBJ identifier overflow".into()) }
fn resolved_source(index:u64,size:usize)->Result<SqliteValue,String>{if index<size as u64{Ok(SqliteValue::Integer(identity(usize::try_from(index).map_err(|_|"OBJ occurrence width differs")?)?))}else{Ok(SqliteValue::Null)}}
fn optional_identity(index:Option<u32>,size:usize)->Result<SqliteValue,String>{match index{Some(index)=>resolved_source(u64::from(index),size),None=>Ok(SqliteValue::Null)}}
fn optional_number(number: Option<f64>) -> SqliteValue { number.map(SqliteValue::Real).unwrap_or(SqliteValue::Null) }

fn append(projection: &mut Projection<'_,'_>, name: &str, values: Vec<SqliteValue>) -> Result<(), String> {
    let cells:Vec<_>=values.iter().map(|value|match value{SqliteValue::Null=>Cell::Null,SqliteValue::Integer(value)=>Cell::Integer(*value),SqliteValue::Real(value)=>Cell::Real(*value),SqliteValue::Text(value)=>Cell::Text(value),SqliteValue::Blob(value)=>Cell::Blob(value)}).collect();
    let columns=match name{"obj_vertex"=>VERTEX_COLUMNS,"obj_texcoord"=>TEXCOORD_COLUMNS,"obj_normal"=>NORMAL_COLUMNS,_=>&[]};
    insert_ieee754(projection,name,&cells,columns).map(|_|())
}

fn entities<'a>(database: &'a SqliteDatabase, name: &str,document:i64) -> Result<Vec<&'a SqliteRow>, String> {
    let table = database.table(name)?;
    let mut ids = BTreeSet::new();
    for row in &table.rows {
        let id = row.integer(0)?;
        if id != row.rowid || !ids.insert(id) { return Err(format!("{name} requires distinct identifiers")); }
        if row.integer(1)? != document { return Err(format!("{name} has an unknown document")); }
    }
    table.ordered_rows(2)
}

fn indices(rows: &[&SqliteRow]) -> Result<BTreeMap<i64, usize>, String> { rows.iter().enumerate().map(|(ordinal, row)| Ok((row.integer(0)?, ordinal))).collect() }
fn reference(row: &SqliteRow, column: usize, index: &BTreeMap<i64, usize>) -> Result<usize, String> { index.get(&row.integer(column)?).copied().ok_or_else(|| "OBJ relationship references an unknown entity".into()) }
fn source32(row:&SqliteRow,column:usize)->Result<u32,String>{u32::try_from(row.integer(column)?).map_err(|_|"OBJ source word must be unsigned32".into())}
fn optional_source32(row:&SqliteRow,column:usize)->Result<Option<u32>,String>{if row.values.get(column)==Some(&SqliteValue::Null){Ok(None)}else{source32(row,column).map(Some)}}
fn source64(row:&SqliteRow,column:usize)->Result<u64,String>{Ok((u64::from(source32(row,column)?)<<32)|u64::from(source32(row,column+1)?))}
fn check_resolved(row:&SqliteRow,column:usize,index:&BTreeMap<i64,usize>,source:Option<u64>)->Result<(),String>{if row.values.get(column)==Some(&SqliteValue::Null){return Ok(())}if Some(reference(row,column,index)? as u64)!=source{return Err("OBJ resolved relationship differs from its source index".into())}Ok(())}

fn children<'a>(database: &'a SqliteDatabase, name: &str, owners: &BTreeMap<i64, usize>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<&'a SqliteRow>>, String> {
    let table = database.table(name)?;
    let mut result = BTreeMap::<i64, Vec<&SqliteRow>>::new();
    let mut ids = BTreeSet::new();
    for (ordinal, row) in table.rows.iter().enumerate() {
        if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, table.rows.len())?; }
        let id = row.integer(0)?;
        if id != row.rowid || !ids.insert(id) { return Err(format!("{name} requires distinct identifiers")); }
        let owner = row.integer(1)?;
        if !owners.contains_key(&owner) { return Err(format!("{name} has an unknown owner")); }
        result.entry(owner).or_default().push(row);
    }
    for rows in result.values_mut() {
        rows.sort_by_key(|row| row.integer(2).unwrap_or(i64::MIN));
        for (ordinal, row) in rows.iter().enumerate() { if row.integer(2)? != ordinal as i64 { return Err(format!("{name} requires contiguous ordinals")); } }
    }
    Ok(result)
}

impl ArtifactSqliteSnapshot for ObjSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{
        let units=self.faces.len().checked_add(self.groups.len()).and_then(|count|count.checked_add(self.objects.len())).ok_or("OBJ native work count overflow")?;control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,units)?;let mut rows=2usize.checked_add(self.faces.len()).ok_or("OBJ native row count overflow")?;
        for count in[self.vertices.len(),self.texcoords.len(),self.normals.len(),self.faces.len(),self.groups.len(),self.objects.len(),self.usemtl.len(),self.smoothing_groups.len(),self.unknown_statements.len()]{rows=rows.checked_add(count).ok_or("OBJ native row count overflow")?;}control.check_rows(rows)?;let mut completed=0usize;
        for face in&self.faces{rows=rows.checked_add(face.vertices.len()).ok_or("OBJ native row count overflow")?;control.check_rows(rows)?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;}}
        for group in&self.groups{rows=rows.checked_add(group.faces.len()).ok_or("OBJ native row count overflow")?;control.check_rows(rows)?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;}}
        for object in&self.objects{rows=rows.checked_add(object.faces.len()).ok_or("OBJ native row count overflow")?;control.check_rows(rows)?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;}}control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,units)?;
        store::encode_sqlite_snapshot_record_native(encoding,"stdio.obj",super::native_pack::spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
    }
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{let maximum_rows=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,"stdio.obj",super::native_pack::spec_producer(),|record,native|super::native_pack::reconstruct_record_controlled(record,native,maximum_rows),control)}
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut bound=NativeEncodingBound::new(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;for _ in &self.vertices{bound.add(256+4*1100)?;}for _ in &self.texcoords{bound.add(192+3*1100)?;}for _ in &self.normals{bound.add(192+3*1100)?;}for face in &self.faces{bound.add(64)?;for _ in &face.vertices{bound.add(192)?;}}for group in &self.groups{bound.add(128)?;bound.repeated(group.name.len(),6)?;for _ in &group.faces{bound.add(32)?;}}for object in &self.objects{bound.add(128)?;bound.repeated(object.name.len(),6)?;for _ in &object.faces{bound.add(32)?;}}if let Some(name)=&self.mtllib{bound.repeated(name.len(),6)?;}for range in &self.usemtl{bound.add(192)?;bound.repeated(range.material.len(),6)?;}for _ in &self.smoothing_groups{bound.add(128)?;}for statement in &self.unknown_statements{bound.add(192)?;bound.repeated(statement.raw.len(),6)?;}bound.finish()}

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.obj"||dialect.standard!="3.0"||dialect.subset!="*"{return Err(String::from("OBJ owned SQLite dialect differs").into());}
        let candidate=Self::from_sqlite_database(database,control)?;
        if candidate.to_sqlite_database(control)?!=self.to_sqlite_database(control)?{return Err(String::from("OBJ document identity differs").into());}
        Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
    }
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
        let mut budget = (0usize, 0usize);
        account(control, &mut budget, 8usize.checked_add(self.schema.len()).and_then(|count| count.checked_add(self.mtllib.as_ref().map_or(0, String::len))).ok_or("OBJ value size overflow")?)?;
        for vertex in &self.vertices { account(control, &mut budget, 24+scalar_bytes(vertex.x)+scalar_bytes(vertex.y)+scalar_bytes(vertex.z)+vertex.w.map_or(0,scalar_bytes))?; }
        for coordinate in &self.texcoords { account(control, &mut budget, 24+scalar_bytes(coordinate.u)+scalar_bytes(coordinate.v)+coordinate.w.map_or(0,scalar_bytes))?; }
        for normal in &self.normals { account(control, &mut budget, 24+scalar_bytes(normal.x)+scalar_bytes(normal.y)+scalar_bytes(normal.z))?; }
        for face in &self.faces {
            account(control, &mut budget, 24)?;
            for vertex in &face.vertices {
                account(control,&mut budget,32+usize::from(vertex.texcoord.is_some())*8+usize::from(vertex.normal.is_some())*8+usize::from((vertex.vertex as usize)<self.vertices.len())*8+usize::from(vertex.texcoord.is_some_and(|index|(index as usize)<self.texcoords.len()))*8+usize::from(vertex.normal.is_some_and(|index|(index as usize)<self.normals.len()))*8)?;
            }
        }
        for group in &self.groups {
            account(control, &mut budget, 24usize.checked_add(group.name.len()).ok_or("OBJ value size overflow")?)?;
            for &face in &group.faces {account(control,&mut budget,40+usize::from(face<self.faces.len() as u64)*8)?;}
        }
        for object in &self.objects {
            account(control, &mut budget, 24usize.checked_add(object.name.len()).ok_or("OBJ value size overflow")?)?;
            for &face in &object.faces {account(control,&mut budget,40+usize::from(face<self.faces.len() as u64)*8)?;}
        }
        for boundary in 0..=self.faces.len() { account(control, &mut budget, if boundary < self.faces.len() { 32 } else { 24 })?; }
        for range in &self.usemtl {account(control,&mut budget,40usize.checked_add(range.material.len()).and_then(|value|value.checked_add(usize::from(range.face_index_from<=self.faces.len() as u64)*8)).ok_or("OBJ value size overflow")?)?;}
        for range in &self.smoothing_groups {account(control,&mut budget,40+usize::from(range.group.is_some())*8+usize::from(range.face_index_from<=self.faces.len() as u64)*8)?;}
        for statement in &self.unknown_statements { account(control, &mut budget, 40usize.checked_add(statement.raw.len()).ok_or("OBJ value size overflow")?)?; }
        let mut database = Projection::new(Self::SQLITE_SCHEMA,control)?;
        let mut completed = 0;
        append(&mut database, "obj_document", vec![SqliteValue::Text(self.schema.clone()), self.mtllib.as_ref().map(|name| SqliteValue::Text(name.clone())).unwrap_or(SqliteValue::Null)])?;
        for (ordinal, vertex) in self.vertices.iter().enumerate() { append(&mut database, "obj_vertex", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Real(vertex.x), SqliteValue::Real(vertex.y), SqliteValue::Real(vertex.z), optional_number(vertex.w)])?; }
        for (ordinal, coordinate) in self.texcoords.iter().enumerate() { append(&mut database, "obj_texcoord", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Real(coordinate.u), SqliteValue::Real(coordinate.v), optional_number(coordinate.w)])?; }
        for (ordinal, normal) in self.normals.iter().enumerate() { append(&mut database, "obj_normal", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Real(normal.x), SqliteValue::Real(normal.y), SqliteValue::Real(normal.z)])?; }
        for (ordinal, face) in self.faces.iter().enumerate() {
            let face_id = identity(ordinal)?;
            append(&mut database, "obj_face", vec![SqliteValue::Integer(1), SqliteValue::Integer(face_id - 1)])?;
            for (ordinal, vertex) in face.vertices.iter().enumerate() {append(&mut database,"obj_face_vertex",vec![SqliteValue::Integer(face_id),SqliteValue::Integer(identity(ordinal)?-1),SqliteValue::Integer(i64::from(vertex.vertex)),vertex.texcoord.map(|value|SqliteValue::Integer(i64::from(value))).unwrap_or(SqliteValue::Null),vertex.normal.map(|value|SqliteValue::Integer(i64::from(value))).unwrap_or(SqliteValue::Null),resolved_source(u64::from(vertex.vertex),self.vertices.len())?,optional_identity(vertex.texcoord,self.texcoords.len())?,optional_identity(vertex.normal,self.normals.len())?])?;}
        }
        for (ordinal, group) in self.groups.iter().enumerate() {
            let group_id = identity(ordinal)?;
            append(&mut database, "obj_group", vec![SqliteValue::Integer(1), SqliteValue::Integer(group_id - 1), SqliteValue::Text(group.name.clone())])?;
            for(ordinal,&face)in group.faces.iter().enumerate(){append(&mut database,"obj_group_face",vec![SqliteValue::Integer(group_id),SqliteValue::Integer(identity(ordinal)?-1),SqliteValue::Integer((face>>32)as i64),SqliteValue::Integer((face&0xffffffff)as i64),resolved_source(face,self.faces.len())?])?;}
        }
        for (ordinal, object) in self.objects.iter().enumerate() {
            let object_id = identity(ordinal)?;
            append(&mut database, "obj_object", vec![SqliteValue::Integer(1), SqliteValue::Integer(object_id - 1), SqliteValue::Text(object.name.clone())])?;
            for(ordinal,&face)in object.faces.iter().enumerate(){append(&mut database,"obj_object_face",vec![SqliteValue::Integer(object_id),SqliteValue::Integer(identity(ordinal)?-1),SqliteValue::Integer((face>>32)as i64),SqliteValue::Integer((face&0xffffffff)as i64),resolved_source(face,self.faces.len())?])?;}
        }
        for ordinal in 0..=self.faces.len() { append(&mut database, "obj_face_boundary", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), if ordinal < self.faces.len() { SqliteValue::Integer(identity(ordinal)?) } else { SqliteValue::Null }])?; }
        for(ordinal,range)in self.usemtl.iter().enumerate(){let index=range.face_index_from;append(&mut database,"obj_material_range",vec![SqliteValue::Integer(1),SqliteValue::Integer(identity(ordinal)?-1),SqliteValue::Integer((index>>32)as i64),SqliteValue::Integer((index&0xffffffff)as i64),resolved_source(index,self.faces.len().checked_add(1).ok_or("OBJ boundary overflow")?)?,SqliteValue::Text(range.material.clone())])?;}
        for(ordinal,range)in self.smoothing_groups.iter().enumerate(){let index=range.face_index_from;append(&mut database,"obj_smoothing_range",vec![SqliteValue::Integer(1),SqliteValue::Integer(identity(ordinal)?-1),SqliteValue::Integer((index>>32)as i64),SqliteValue::Integer((index&0xffffffff)as i64),resolved_source(index,self.faces.len().checked_add(1).ok_or("OBJ boundary overflow")?)?,range.group.map(|group|SqliteValue::Integer(i64::from(group))).unwrap_or(SqliteValue::Null)])?;}
        for (ordinal, statement) in self.unknown_statements.iter().enumerate() { let position = statement.line_index; append(&mut database, "obj_unknown_statement", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Integer((position >> 32) as i64), SqliteValue::Integer((position & u32::MAX as u64) as i64), SqliteValue::Text(statement.raw.clone())])?; }
        database.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        for(name,width)in[("obj_document",3),("obj_vertex",15),("obj_texcoord",12),("obj_normal",12),("obj_face",3),("obj_face_vertex",9),("obj_group",4),("obj_group_face",6),("obj_object",4),("obj_object_face",6),("obj_face_boundary",4),("obj_material_range",7),("obj_smoothing_range",7),("obj_unknown_statement",6)]{for row in&database.table(name)?.rows{if row.values.len()!=width{return Err(format!("{name} row width differs"));}}}
        let document = database.table("obj_document")?.single_row()?;
        if document.integer(0)?!=document.rowid{return Err("OBJ document identity alias differs".into())}let document_id=document.rowid;
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
        if boundary_rows.len() != face_rows.len().checked_add(1).ok_or("OBJ boundary count overflow")? { return Err("OBJ requires one boundary for every face and its end".into()); }
        for (ordinal, boundary) in boundary_rows.iter().enumerate() {
            if ordinal < face_rows.len() { if reference(boundary, 3, &face_indices)? != ordinal { return Err("OBJ face boundary references the wrong face".into()); } }
            else if boundary.values.get(3) != Some(&SqliteValue::Null) { return Err("OBJ final boundary must not reference a face".into()); }
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
