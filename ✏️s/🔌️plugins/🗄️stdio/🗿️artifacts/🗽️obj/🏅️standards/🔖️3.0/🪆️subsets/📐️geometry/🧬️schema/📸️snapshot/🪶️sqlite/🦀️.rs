
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
fn optional_identity(index: Option<u32>) -> SqliteValue { index.map(|index| SqliteValue::Integer(i64::from(index) + 1)).unwrap_or(SqliteValue::Null) }
fn optional_number(number: Option<f64>) -> SqliteValue { number.map(SqliteValue::Real).unwrap_or(SqliteValue::Null) }

fn append(projection: &mut Projection<'_,'_>, name: &str, values: Vec<SqliteValue>) -> Result<(), String> {
    let cells:Vec<_>=values.iter().map(|value|match value{SqliteValue::Null=>Cell::Null,SqliteValue::Integer(value)=>Cell::Integer(*value),SqliteValue::Real(value)=>Cell::Real(*value),SqliteValue::Text(value)=>Cell::Text(value),SqliteValue::Blob(value)=>Cell::Blob(value)}).collect();
    let columns=match name{"obj_vertex"=>VERTEX_COLUMNS,"obj_texcoord"=>TEXCOORD_COLUMNS,"obj_normal"=>NORMAL_COLUMNS,_=>&[]};
    insert_ieee754(projection,name,&cells,columns).map(|_|())
}

fn entities<'a>(database: &'a SqliteDatabase, name: &str) -> Result<Vec<&'a SqliteRow>, String> {
    let table = database.table(name)?;
    let mut ids = BTreeSet::new();
    for row in &table.rows {
        let id = row.integer(0)?;
        if id < 1 || id != row.rowid || !ids.insert(id) { return Err(format!("{name} requires distinct positive identifiers")); }
        if row.integer(1)? != 1 { return Err(format!("{name} has an unknown document")); }
    }
    table.ordered_rows(2)
}

fn indices(rows: &[&SqliteRow]) -> Result<BTreeMap<i64, usize>, String> { rows.iter().enumerate().map(|(ordinal, row)| Ok((row.integer(0)?, ordinal))).collect() }
fn reference(row: &SqliteRow, column: usize, index: &BTreeMap<i64, usize>) -> Result<usize, String> { index.get(&row.integer(column)?).copied().ok_or_else(|| "OBJ relationship references an unknown entity".into()) }
fn optional_reference(row: &SqliteRow, column: usize, index: &BTreeMap<i64, usize>) -> Result<Option<u32>, String> { if row.values.get(column) == Some(&SqliteValue::Null) { Ok(None) } else { u32::try_from(reference(row, column, index)?).map(Some).map_err(|error| error.to_string()) } }

fn children<'a>(database: &'a SqliteDatabase, name: &str, owners: &BTreeMap<i64, usize>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<&'a SqliteRow>>, String> {
    let table = database.table(name)?;
    let mut result = BTreeMap::<i64, Vec<&SqliteRow>>::new();
    let mut ids = BTreeSet::new();
    for (ordinal, row) in table.rows.iter().enumerate() {
        if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, table.rows.len())?; }
        let id = row.integer(0)?;
        if id < 1 || id != row.rowid || !ids.insert(id) { return Err(format!("{name} requires distinct positive identifiers")); }
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
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut bound=NativeEncodingBound::new(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;for _ in &self.vertices{bound.add(256+4*1100)?;}for _ in &self.texcoords{bound.add(192+3*1100)?;}for _ in &self.normals{bound.add(192+3*1100)?;}for face in &self.faces{bound.add(64)?;for _ in &face.vertices{bound.add(192)?;}}for group in &self.groups{bound.add(128)?;bound.repeated(group.name.len(),6)?;for _ in &group.faces{bound.add(32)?;}}for object in &self.objects{bound.add(128)?;bound.repeated(object.name.len(),6)?;for _ in &object.faces{bound.add(32)?;}}if let Some(name)=&self.mtllib{bound.repeated(name.len(),6)?;}for range in &self.usemtl{bound.add(192)?;bound.repeated(range.material.len(),6)?;}for _ in &self.smoothing_groups{bound.add(128)?;}for statement in &self.unknown_statements{bound.add(192)?;bound.repeated(statement.raw.len(),6)?;}bound.finish()}

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.obj"||dialect.standard!="3.0"||dialect.subset!="geometry"{return Err(String::from("geometry owned SQLite dialect differs").into());}
        let row=database.table("obj_document")?.single_row()?;
        if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("geometry document identity differs from its semantic projection").into());}
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
            if face.vertices.len() < 3 { return Err("OBJ faces must have at least three vertices".into()); }
            account(control, &mut budget, 24)?;
            for vertex in &face.vertices {
                if vertex.vertex as usize >= self.vertices.len() || vertex.texcoord.is_some_and(|index| index as usize >= self.texcoords.len()) || vertex.normal.is_some_and(|index| index as usize >= self.normals.len()) { return Err("OBJ face references unknown geometry".into()); }
                account(control, &mut budget, 32 + usize::from(vertex.texcoord.is_some()) * 8 + usize::from(vertex.normal.is_some()) * 8)?;
            }
        }
        for group in &self.groups {
            account(control, &mut budget, 24usize.checked_add(group.name.len()).ok_or("OBJ value size overflow")?)?;
            for &face in &group.faces { if face >= self.faces.len() { return Err("OBJ group references an unknown face".into()); } account(control, &mut budget, 32)?; }
        }
        for object in &self.objects {
            account(control, &mut budget, 24usize.checked_add(object.name.len()).ok_or("OBJ value size overflow")?)?;
            for &face in &object.faces { if face >= self.faces.len() { return Err("OBJ object references an unknown face".into()); } account(control, &mut budget, 32)?; }
        }
        for boundary in 0..=self.faces.len() { account(control, &mut budget, if boundary < self.faces.len() { 32 } else { 24 })?; }
        for range in &self.usemtl { if range.face_index_from > self.faces.len() { return Err("OBJ material range starts beyond the face boundary".into()); } account(control, &mut budget, 32usize.checked_add(range.material.len()).ok_or("OBJ value size overflow")?)?; }
        for range in &self.smoothing_groups { if range.face_index_from > self.faces.len() { return Err("OBJ smoothing range starts beyond the face boundary".into()); } account(control, &mut budget, 32 + usize::from(range.group.is_some()) * 8)?; }
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
            for (ordinal, vertex) in face.vertices.iter().enumerate() { append(&mut database, "obj_face_vertex", vec![SqliteValue::Integer(face_id), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Integer(i64::from(vertex.vertex) + 1), optional_identity(vertex.texcoord), optional_identity(vertex.normal)])?; }
        }
        for (ordinal, group) in self.groups.iter().enumerate() {
            let group_id = identity(ordinal)?;
            append(&mut database, "obj_group", vec![SqliteValue::Integer(1), SqliteValue::Integer(group_id - 1), SqliteValue::Text(group.name.clone())])?;
            for (ordinal, &face) in group.faces.iter().enumerate() { append(&mut database, "obj_group_face", vec![SqliteValue::Integer(group_id), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Integer(identity(face)?)])?; }
        }
        for (ordinal, object) in self.objects.iter().enumerate() {
            let object_id = identity(ordinal)?;
            append(&mut database, "obj_object", vec![SqliteValue::Integer(1), SqliteValue::Integer(object_id - 1), SqliteValue::Text(object.name.clone())])?;
            for (ordinal, &face) in object.faces.iter().enumerate() { append(&mut database, "obj_object_face", vec![SqliteValue::Integer(object_id), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Integer(identity(face)?)])?; }
        }
        for ordinal in 0..=self.faces.len() { append(&mut database, "obj_face_boundary", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), if ordinal < self.faces.len() { SqliteValue::Integer(identity(ordinal)?) } else { SqliteValue::Null }])?; }
        for (ordinal, range) in self.usemtl.iter().enumerate() { append(&mut database, "obj_material_range", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Integer(identity(range.face_index_from)?), SqliteValue::Text(range.material.clone())])?; }
        for (ordinal, range) in self.smoothing_groups.iter().enumerate() { append(&mut database, "obj_smoothing_range", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Integer(identity(range.face_index_from)?), range.group.map(|group| SqliteValue::Integer(i64::from(group))).unwrap_or(SqliteValue::Null)])?; }
        for (ordinal, statement) in self.unknown_statements.iter().enumerate() { let position = statement.line_index; append(&mut database, "obj_unknown_statement", vec![SqliteValue::Integer(1), SqliteValue::Integer(identity(ordinal)? - 1), SqliteValue::Integer((position >> 32) as i64), SqliteValue::Integer((position & u32::MAX as u64) as i64), SqliteValue::Text(statement.raw.clone())])?; }
        database.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("obj_document")?.single_row()?;
        if document.integer(0)? != 1 || document.rowid != 1 { return Err("OBJ document identifier must be 1".into()); }
        let vertex_rows = entities(database, "obj_vertex")?;
        let texcoord_rows = entities(database, "obj_texcoord")?;
        let normal_rows = entities(database, "obj_normal")?;
        let face_rows = entities(database, "obj_face")?;
        let group_rows = entities(database, "obj_group")?;
        let object_rows = entities(database, "obj_object")?;
        let boundary_rows = entities(database, "obj_face_boundary")?;
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
            if owned.len() < 3 { return Err("OBJ faces must have at least three vertices".into()); }
            let mut vertices = Vec::new();
            for vertex in owned {
                if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
                vertices.push(ObjFaceVertex { vertex: u32::try_from(reference(vertex, 3, &vertex_indices)?).map_err(|error| error.to_string())?, texcoord: optional_reference(vertex, 4, &texcoord_indices)?, normal: optional_reference(vertex, 5, &normal_indices)? });
            }
            snapshot.faces.push(ObjFace { vertices });
        }
        for row in group_rows {
            if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
            let mut faces = Vec::new();
            for member in group_children.remove(&row.integer(0)?).unwrap_or_default() { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; faces.push(reference(member, 3, &face_indices)?); }
            snapshot.groups.push(ObjGroup { name: row.text(3)?.into(), faces });
        }
        for row in object_rows {
            if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1;
            let mut faces = Vec::new();
            for member in object_children.remove(&row.integer(0)?).unwrap_or_default() { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; faces.push(reference(member, 3, &face_indices)?); }
            snapshot.objects.push(ObjObject { name: row.text(3)?.into(), faces });
        }
        for row in entities(database, "obj_material_range")? { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; snapshot.usemtl.push(ObjUsemtlRange { face_index_from: reference(row, 3, &boundary_indices)?, material: row.text(4)?.into() }); }
        for row in entities(database, "obj_smoothing_range")? { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; let group = if row.values.get(4) == Some(&SqliteValue::Null) { None } else { Some(u32::try_from(row.integer(4)?).map_err(|error| error.to_string())?) }; snapshot.smoothing_groups.push(ObjSmoothingRange { face_index_from: reference(row, 3, &boundary_indices)?, group }); }
        for row in entities(database, "obj_unknown_statement")? { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; let high = u32::try_from(row.integer(3)?).map_err(|error| error.to_string())?; let low = u32::try_from(row.integer(4)?).map_err(|error| error.to_string())?; snapshot.unknown_statements.push(ObjUnknownStatement { line_index: (u64::from(high) << 32) | u64::from(low), raw: row.text(5)?.into() }); }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(snapshot)
    }
}
