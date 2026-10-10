//! 🔺️ Mesh primitives and indexed, ordered numeric buffers with explicit material relationships.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot,SemioMesh,SemioPrimitive,SemioTopology,SemioMaterial,SemioTexture};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3,SemioUv,SemioRgba};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text,reconstruct_blob},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_framework_os_kernel::sqlite_snapshot::transfer;
fn reference(id:Option<&str>)->Cell<'_>{match id{None=>Cell::Null,Some(id)=>Cell::Text(id)}}
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🫳️ Visits every Mesh buffer, exact IEEE word and literal reference through the same row writer.
pub(crate)fn visit_rows(snapshot:&SemioMeshSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
out.insert_key_float("semio_mesh_document",1,&[Cell::Text(&snapshot.schema)],float_columns("semio_mesh_document"))?;
for(ordinal,texture)in snapshot.textures.iter().enumerate(){out.insert_float("semio_mesh_texture",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&texture.id),Cell::Text(&texture.mime),Cell::Blob(&texture.bytes)],float_columns("semio_mesh_texture"))?;}
for(ordinal,material)in snapshot.materials.iter().enumerate(){let c=material.base_color;out.insert_float("semio_mesh_material",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&material.id),Cell::Float32(c.r),Cell::Float32(c.g),Cell::Float32(c.b),Cell::Float32(c.a),Cell::Float32(material.metallic),Cell::Float32(material.roughness),reference(material.base_color_texture.as_deref()),reference(material.metallic_roughness_texture.as_deref()),reference(material.normal_texture.as_deref()),reference(material.occlusion_texture.as_deref()),reference(material.emissive_texture.as_deref())],float_columns("semio_mesh_material"))?;}
for(ordinal,mesh)in snapshot.meshes.iter().enumerate(){let mesh_id=out.insert_float("semio_mesh_mesh",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&mesh.id)],float_columns("semio_mesh_mesh"))?;
for(ordinal,primitive)in mesh.primitives.iter().enumerate(){let id=out.insert_float("semio_mesh_primitive",&[Cell::Integer(mesh_id),Cell::Integer(number(ordinal)?),Cell::Text(&primitive.id),Cell::Text(topology(primitive.topology)),reference(primitive.material_id.as_deref())],float_columns("semio_mesh_primitive"))?;
for(ordinal,position)in primitive.positions.iter().enumerate(){out.insert_float("semio_mesh_position",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(position.x),Cell::Real(position.y),Cell::Real(position.z)],float_columns("semio_mesh_position"))?;}
for(ordinal,normal)in primitive.normals.iter().enumerate(){out.insert_float("semio_mesh_normal",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(normal.x),Cell::Real(normal.y),Cell::Real(normal.z)],float_columns("semio_mesh_normal"))?;}
for(ordinal,uv)in primitive.uvs.iter().enumerate(){out.insert_float("semio_mesh_uv",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(uv.u),Cell::Real(uv.v)],float_columns("semio_mesh_uv"))?;}
for(ordinal,color)in primitive.colors.iter().enumerate(){out.insert_float("semio_mesh_color",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Float32(color.r),Cell::Float32(color.g),Cell::Float32(color.b),Cell::Float32(color.a)],float_columns("semio_mesh_color"))?;}
for(ordinal,index)in primitive.indices.iter().enumerate(){out.insert_float("semio_mesh_index",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Integer(i64::from(*index))],float_columns("semio_mesh_index"))?;}}}
Ok(())
}
/// 🎟️ Admits complete typed cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioMeshSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before allocating typed fields.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts actual document primitive cells before allocating typed fields.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio mesh row identity or columns"))}else{Ok(())}}
fn tick(control:&mut SqliteSnapshotControl<'_>,completed:&mut usize)->Result<(),ValueError>{*completed+=1;if *completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*completed,0)?;}Ok(())}
fn topology(value:SemioTopology)->&'static str{match value{SemioTopology::Points=>"points",SemioTopology::Lines=>"lines",SemioTopology::LineStrip=>"line_strip",SemioTopology::Triangles=>"triangles",SemioTopology::TriangleStrip=>"triangle_strip",SemioTopology::TriangleFan=>"triangle_fan"}}
impl ArtifactSqliteSnapshot for SemioMeshSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::mesh::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}

 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::mesh::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="mesh"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_mesh_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioMeshSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_mesh_position"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_mesh_normal"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_mesh_uv"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_mesh_color"=>&[FloatColumn::Binary32(3),FloatColumn::Binary32(4),FloatColumn::Binary32(5),FloatColumn::Binary32(6)],"semio_mesh_material"=>&[FloatColumn::Binary32(4),FloatColumn::Binary32(5),FloatColumn::Binary32(6),FloatColumn::Binary32(7),FloatColumn::Binary32(8),FloatColumn::Binary32(9)],_=>&[]}}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioMeshSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.meshes.len())?;for mesh in &self.meshes{b.text(&mesh.id)?;b.entities(mesh.primitives.len())?;for primitive in &mesh.primitives{b.text(&primitive.id)?;b.optional_text(primitive.material_id.as_deref())?;b.scalars(1)?;for(count,width)in[(primitive.positions.len(),3),(primitive.normals.len(),3),(primitive.uvs.len(),2),(primitive.colors.len(),4),(primitive.indices.len(),1)]{b.entities(count)?;b.scalars(count.checked_mul(width).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio mesh native scalar count overflow"))?)?;}}}b.entities(self.materials.len())?;for material in &self.materials{b.text(&material.id)?;b.scalars(6)?;b.optional_text(material.base_color_texture.as_deref())?;b.optional_text(material.metallic_roughness_texture.as_deref())?;b.optional_text(material.normal_texture.as_deref())?;b.optional_text(material.occlusion_texture.as_deref())?;b.optional_text(material.emissive_texture.as_deref())?;}b.entities(self.textures.len())?;for texture in &self.textures{b.text(&texture.id)?;b.text(&texture.mime)?;b.bytes(&texture.bytes)?;}Ok(())}
}

impl SemioMeshSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::project_rows_owned(Self::SQLITE_SCHEMA,control,|out|visit_rows(self,out))
}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;

#[path="💰️reconstruction/🦀️.rs"]mod reconstruction;
