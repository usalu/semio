//! 🔺️ Mesh primitives and indexed, ordered numeric buffers with explicit material relationships.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,insert_ieee754,insert_key_ieee754};
use super::{SemioMeshSnapshot,SemioMesh,SemioPrimitive,SemioTopology,SemioMaterial,SemioTexture,SemioPoint3,SemioUv,SemioRgba};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text,reconstruct_blob},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_framework_os_kernel::sqlite_snapshot::transfer;
fn reference(id:Option<&str>)->Cell<'_>{match id{None=>Cell::Null,Some(id)=>Cell::Text(id)}}
fn optional_text(row:SqliteRow<'_>,column:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Option<String>,ValueError>{if row.is_null(column)?{Ok(None)}else{Ok(Some(reconstruct_text(control,row.text(column)?)?))}}
fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio mesh row identity or columns"))}else{Ok(())}}
fn tick(control:&mut SqliteSnapshotControl<'_>,completed:&mut usize)->Result<(),ValueError>{*completed+=1;if *completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*completed,0)?;}Ok(())}
fn identities(rows:&[SqliteRow<'_>],columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<i64>,ValueError>{
 let mut ids=transfer::reserve(rows.len(),control)?;for(at,row)in rows.iter().enumerate(){identity(row,columns)?;ids.push(row.rowid);if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}
 transfer::heap_sort(&mut ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.cmp(b)))?;for(at,id)in ids.iter().enumerate(){if at>0&&ids[at-1]==*id{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio mesh row identity"));}if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}Ok(ids)
}
fn owned_rows<'a>(db:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{
 let rows=ordered_float_rows(db,table,2,control)?;identities(&rows,columns,control)?;for(at,row)in rows.iter().enumerate(){if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio mesh document parent"));}if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}Ok(rows)
}
fn relationships<'a>(db:&'a SqliteDatabase,table:&str,columns:usize,owners:&[i64],control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{
 let mut rows=float_rows(db,table,control)?;identities(&rows,columns,control)?;for(at,row)in rows.iter().enumerate(){if owners.binary_search(&row.integer(1)?).is_err(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio mesh structural parent"));}if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}
 transfer::heap_sort(&mut rows,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;
 let mut parent=None;let mut ordinal=0usize;for(at,row)in rows.iter().enumerate(){let owner=row.integer(1)?;if parent!=Some(owner){parent=Some(owner);ordinal=0;}if row.integer(2)?!=number(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio mesh relationship ordinals require contiguous occurrences"));}ordinal+=1;if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}Ok(rows)
}
fn group<'a,'b>(rows:&'b[SqliteRow<'a>],parent:i64)->&'b[SqliteRow<'a>]{let start=rows.partition_point(|row|row.integer(1).is_ok_and(|id|id<parent));let end=rows.partition_point(|row|row.integer(1).is_ok_and(|id|id<=parent));&rows[start..end]}
fn topology(value:SemioTopology)->&'static str{match value{SemioTopology::Points=>"points",SemioTopology::Lines=>"lines",SemioTopology::LineStrip=>"line_strip",SemioTopology::Triangles=>"triangles",SemioTopology::TriangleStrip=>"triangle_strip",SemioTopology::TriangleFan=>"triangle_fan"}}
impl ArtifactSqliteSnapshot for SemioMeshSnapshot{
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{super::native_encoding::encode(self,encoding,control)}

 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{super::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="mesh"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_mesh_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioMeshSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;let document=single_float_row(database,"semio_mesh_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio mesh document identifier"));}let mut completed=0usize;
let material_rows=owned_rows(database,"semio_mesh_material",15,control)?;let mut materials=transfer::reserve(material_rows.len(),control)?;
for row in material_rows{materials.push(SemioMaterial{id:reconstruct_text(control,row.text(3)?)?,base_color:SemioRgba{r:row.binary32(4)?,g:row.binary32(5)?,b:row.binary32(6)?,a:row.binary32(7)?},metallic:row.binary32(8)?,roughness:row.binary32(9)?,base_color_texture:optional_text(row,10,control)?,metallic_roughness_texture:optional_text(row,11,control)?,normal_texture:optional_text(row,12,control)?,occlusion_texture:optional_text(row,13,control)?,emissive_texture:optional_text(row,14,control)?});tick(control,&mut completed)?;}
let mesh_rows=owned_rows(database,"semio_mesh_mesh",4,control)?;let mesh_ids=identities(&mesh_rows,4,control)?;let primitives=relationships(database,"semio_mesh_primitive",6,&mesh_ids,control)?;let primitive_ids=identities(&primitives,6,control)?;
let positions=relationships(database,"semio_mesh_position",6,&primitive_ids,control)?;let normals=relationships(database,"semio_mesh_normal",6,&primitive_ids,control)?;let uvs=relationships(database,"semio_mesh_uv",5,&primitive_ids,control)?;let colors=relationships(database,"semio_mesh_color",7,&primitive_ids,control)?;let indices=relationships(database,"semio_mesh_index",4,&primitive_ids,control)?;
let mut meshes=transfer::reserve(mesh_rows.len(),control)?;
for mesh in mesh_rows{let primitive_rows=group(&primitives,mesh.rowid);let mut native_primitives=transfer::reserve(primitive_rows.len(),control)?;
for primitive in primitive_rows{let topology=match primitive.text(4)?{"points"=>SemioTopology::Points,"lines"=>SemioTopology::Lines,"line_strip"=>SemioTopology::LineStrip,"triangles"=>SemioTopology::Triangles,"triangle_strip"=>SemioTopology::TriangleStrip,"triangle_fan"=>SemioTopology::TriangleFan,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio mesh topology"))};
let position_rows=group(&positions,primitive.rowid);let mut native_positions=transfer::reserve(position_rows.len(),control)?;for row in position_rows{native_positions.push(SemioPoint3{x:row.real(3)?,y:row.real(4)?,z:row.real(5)?});tick(control,&mut completed)?;}
let normal_rows=group(&normals,primitive.rowid);let mut native_normals=transfer::reserve(normal_rows.len(),control)?;for row in normal_rows{native_normals.push(SemioPoint3{x:row.real(3)?,y:row.real(4)?,z:row.real(5)?});tick(control,&mut completed)?;}
let uv_rows=group(&uvs,primitive.rowid);let mut native_uvs=transfer::reserve(uv_rows.len(),control)?;for row in uv_rows{native_uvs.push(SemioUv{u:row.real(3)?,v:row.real(4)?});tick(control,&mut completed)?;}
let color_rows=group(&colors,primitive.rowid);let mut native_colors=transfer::reserve(color_rows.len(),control)?;for row in color_rows{native_colors.push(SemioRgba{r:row.binary32(3)?,g:row.binary32(4)?,b:row.binary32(5)?,a:row.binary32(6)?});tick(control,&mut completed)?;}
let index_rows=group(&indices,primitive.rowid);let mut native_indices=transfer::reserve(index_rows.len(),control)?;for row in index_rows{native_indices.push(u32::try_from(row.integer(3)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?);tick(control,&mut completed)?;}
native_primitives.push(SemioPrimitive{id:reconstruct_text(control,primitive.text(3)?)?,topology,positions:native_positions,normals:native_normals,uvs:native_uvs,colors:native_colors,indices:native_indices,material_id:optional_text(*primitive,5,control)?});tick(control,&mut completed)?;}meshes.push(SemioMesh{id:reconstruct_text(control,mesh.text(3)?)?,primitives:native_primitives});tick(control,&mut completed)?;}
let texture_rows=owned_rows(database,"semio_mesh_texture",6,control)?;let mut textures=transfer::reserve(texture_rows.len(),control)?;for row in texture_rows{textures.push(SemioTexture{id:reconstruct_text(control,row.text(3)?)?,mime:reconstruct_text(control,row.text(4)?)?,bytes:reconstruct_blob(control,row.blob(5)?)?});tick(control,&mut completed)?;}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,meshes,materials,textures})
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_mesh_position"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_mesh_normal"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_mesh_uv"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_mesh_color"=>&[FloatColumn::Binary32(3),FloatColumn::Binary32(4),FloatColumn::Binary32(5),FloatColumn::Binary32(6)],"semio_mesh_material"=>&[FloatColumn::Binary32(4),FloatColumn::Binary32(5),FloatColumn::Binary32(6),FloatColumn::Binary32(7),FloatColumn::Binary32(8),FloatColumn::Binary32(9)],_=>&[]}}
trait FloatProjection { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>; fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>; }
impl FloatProjection for Projection<'_,'_> { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{insert_ieee754(self,table,cells,float_columns(table))} fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>{insert_key_ieee754(self,table,key,cells,float_columns(table))} }
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=transfer::reserve(table_rows.rows.len(),control)?;for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let mut result=transfer::reserve(db.table(table)?.rows.len(),control)?;for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioMeshSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.meshes.len())?;for mesh in &self.meshes{b.text(&mesh.id)?;b.entities(mesh.primitives.len())?;for primitive in &mesh.primitives{b.text(&primitive.id)?;b.optional_text(primitive.material_id.as_deref())?;b.scalars(1)?;for(count,width)in[(primitive.positions.len(),3),(primitive.normals.len(),3),(primitive.uvs.len(),2),(primitive.colors.len(),4),(primitive.indices.len(),1)]{b.entities(count)?;b.scalars(count.checked_mul(width).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio mesh native scalar count overflow"))?)?;}}}b.entities(self.materials.len())?;for material in &self.materials{b.text(&material.id)?;b.scalars(6)?;b.optional_text(material.base_color_texture.as_deref())?;b.optional_text(material.metallic_roughness_texture.as_deref())?;b.optional_text(material.normal_texture.as_deref())?;b.optional_text(material.occlusion_texture.as_deref())?;b.optional_text(material.emissive_texture.as_deref())?;}b.entities(self.textures.len())?;for texture in &self.textures{b.text(&texture.id)?;b.text(&texture.mime)?;b.bytes(&texture.bytes)?;}Ok(())}
}

impl SemioMeshSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
let mut p=Projection::new(Self::SQLITE_SCHEMA,control)?;p.insert_key_float("semio_mesh_document",1,&[Cell::Text(&self.schema)])?;
for(ordinal,texture)in self.textures.iter().enumerate(){p.insert_float("semio_mesh_texture",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&texture.id),Cell::Text(&texture.mime),Cell::Blob(&texture.bytes)])?;}
for(ordinal,material)in self.materials.iter().enumerate(){let c=material.base_color;p.insert_float("semio_mesh_material",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&material.id),Cell::Float32(c.r),Cell::Float32(c.g),Cell::Float32(c.b),Cell::Float32(c.a),Cell::Float32(material.metallic),Cell::Float32(material.roughness),reference(material.base_color_texture.as_deref()),reference(material.metallic_roughness_texture.as_deref()),reference(material.normal_texture.as_deref()),reference(material.occlusion_texture.as_deref()),reference(material.emissive_texture.as_deref())])?;}
for(ordinal,mesh)in self.meshes.iter().enumerate(){let mesh_id=p.insert_float("semio_mesh_mesh",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&mesh.id)])?;
for(ordinal,primitive)in mesh.primitives.iter().enumerate(){let id=p.insert_float("semio_mesh_primitive",&[Cell::Integer(mesh_id),Cell::Integer(number(ordinal)?),Cell::Text(&primitive.id),Cell::Text(topology(primitive.topology)),reference(primitive.material_id.as_deref())])?;
for(ordinal,position)in primitive.positions.iter().enumerate(){p.insert_float("semio_mesh_position",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(position.x),Cell::Real(position.y),Cell::Real(position.z)])?;}
for(ordinal,normal)in primitive.normals.iter().enumerate(){p.insert_float("semio_mesh_normal",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(normal.x),Cell::Real(normal.y),Cell::Real(normal.z)])?;}
for(ordinal,uv)in primitive.uvs.iter().enumerate(){p.insert_float("semio_mesh_uv",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(uv.u),Cell::Real(uv.v)])?;}
for(ordinal,color)in primitive.colors.iter().enumerate(){p.insert_float("semio_mesh_color",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Float32(color.r),Cell::Float32(color.g),Cell::Float32(color.b),Cell::Float32(color.a)])?;}
for(ordinal,index)in primitive.indices.iter().enumerate(){p.insert_float("semio_mesh_index",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Integer(i64::from(*index))])?;}}}p.finish()
}
}
