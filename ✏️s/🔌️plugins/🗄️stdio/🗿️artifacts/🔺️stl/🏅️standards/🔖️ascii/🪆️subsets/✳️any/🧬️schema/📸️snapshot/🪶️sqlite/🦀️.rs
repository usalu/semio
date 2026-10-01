
use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,artifact::NativeEncodingBound};
use super::{StlSnapshot, StlTriangle};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{Projection,Cell,FloatColumn,FloatRow,insert_ieee754};
const COORDINATES:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
fn scalar_bytes(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}

fn coordinates(row: &SqliteRow) -> Result<[f64; 3], String> {
    let row=FloatRow::new(row,COORDINATES)?;
    Ok([row.real(3)?, row.real(4)?, row.real(5)?])
}

impl ArtifactSqliteSnapshot for StlSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut bound=NativeEncodingBound::new(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;bound.repeated(self.solid_name.len(),6)?;for _ in &self.triangles{bound.add(256+12*1100)?;}bound.finish()}

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.stl"||dialect.standard!="ascii"||dialect.subset!="*"{return Err(String::from("geometry owned SQLite dialect differs").into());}
        let row=database.table("stl_solid")?.single_row()?;
        if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("geometry document identity differs from its semantic projection").into());}
        Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
    }
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        let total = self.triangles.len().checked_mul(4).ok_or("STL entity count overflow")?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or("STL entity count overflow")?)?;
        let mut bytes=8usize.checked_add(self.schema.len()).and_then(|n|n.checked_add(self.solid_name.len())).ok_or("STL value size overflow")?;
        for(ordinal,triangle)in self.triangles.iter().enumerate(){
            bytes=bytes.checked_add(96).ok_or("STL value size overflow")?;
            for value in triangle.normal.iter().chain(triangle.vertices.iter().flatten()){bytes=bytes.checked_add(scalar_bytes(*value)).ok_or("STL value size overflow")?;}
            control.check_value_bytes(bytes)?;
            if ordinal%64==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,total)?;}
        }
        let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;
        projection.insert("stl_solid",&[Cell::Text(&self.schema),Cell::Text(&self.solid_name)])?;
        for (ordinal, triangle) in self.triangles.iter().enumerate() {
            let ordinal = i64::try_from(ordinal).map_err(|error| error.to_string())?;
            let facet_id = ordinal.checked_add(1).ok_or("STL facet identifier overflow")?;
            insert_ieee754(&mut projection,"stl_facet",&[Cell::Integer(1),Cell::Integer(ordinal),Cell::Real(triangle.normal[0]),Cell::Real(triangle.normal[1]),Cell::Real(triangle.normal[2])],COORDINATES)?;
            for (ordinal, vertex) in triangle.vertices.iter().enumerate() {
                insert_ieee754(&mut projection,"stl_vertex",&[Cell::Integer(facet_id),Cell::Integer(ordinal as i64),Cell::Real(vertex[0]),Cell::Real(vertex[1]),Cell::Real(vertex[2])],COORDINATES)?;
            }
        }
        projection.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let solid = database.table("stl_solid")?.single_row()?;
        if solid.integer(0)? != 1 { return Err("STL solid identifier must be 1".into()); }
        let facet_table = database.table("stl_facet")?;
        let vertex_table = database.table("stl_vertex")?;
        let total = facet_table.rows.len().checked_add(vertex_table.rows.len()).ok_or("STL entity count overflow")?;
        let mut facet_ids = BTreeSet::new();
        for (ordinal, row) in facet_table.rows.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let id = row.integer(0)?;
            if id < 1 || !facet_ids.insert(id) { return Err("STL facet identifiers must be distinct positive integers".into()); }
        }
        let mut vertices = BTreeMap::<i64, Vec<&SqliteRow>>::new();
        for (ordinal, row) in vertex_table.rows.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let facet_id = row.integer(1)?;
            if !facet_ids.contains(&facet_id) { return Err("STL vertex has an unknown facet".into()); }
            vertices.entry(facet_id).or_default().push(row);
        }
        let mut triangles = Vec::new();
        for row in facet_table.ordered_rows(2)? {
            if triangles.len() % 64 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, triangles.len() * 4, total)?; }
            if row.integer(1)? != 1 { return Err("STL facet has an unknown solid".into()); }
            let mut owned = vertices.remove(&row.integer(0)?).unwrap_or_default();
            if owned.len() != 3 { return Err("STL facets must own exactly three vertices".into()); }
            owned.sort_by_key(|row| row.integer(2).unwrap_or(i64::MIN));
            for (ordinal, vertex) in owned.iter().enumerate() {
                if vertex.integer(2)? != ordinal as i64 { return Err("STL vertex ordinals must be 0, 1, 2".into()); }
            }
            triangles.push(StlTriangle { normal: coordinates(row)?, vertices: [coordinates(owned[0])?, coordinates(owned[1])?, coordinates(owned[2])?] });
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(Self { schema: solid.text(1)?.into(), solid_name: solid.text(2)?.into(), triangles })
    }
}
