//! 🧊️ Explicit GLTF semantic entities retain independently owned document and buffer state.
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,io_schema::{ArtifactDialect,IoError,IoOutcome},sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Cell,Projection,Reconstruction,FloatColumn,FloatRow,insert_ieee754,insert_key_ieee754}}};

#[path="📄️document/💰️backing/🦀️.rs"]mod document;
#[path="🌳️node/💰️backing/🦀️.rs"]mod node;
#[path="🏔️mesh/💰️backing/🦀️.rs"]mod mesh;
#[path="📦️buffer/💰️backing/🦀️.rs"]mod buffer;
#[path="🖌️material/💰️backing/🦀️.rs"]mod material;
#[path="🖼️texture/💰️backing/🦀️.rs"]mod texture;
#[path="🦴️skin/💰️backing/🦀️.rs"]mod skin;
#[path="🎬️animation/💰️backing/🦀️.rs"]mod animation;
#[path="🎥️camera/💰️backing/🦀️.rs"]mod camera;
#[path="🧩️extras/💰️backing/🦀️.rs"]mod extras;
#[path="📏️encoding/💰️backing/🦀️.rs"]mod encoding;

pub const SQLITE_SCHEMA:&str=concat!(include_str!("📄️document/🗄️.sql"),"\n",include_str!("🧩️extras/🗄️.sql"),"\n",include_str!("🌳️node/🗄️.sql"),"\n",include_str!("🏔️mesh/🗄️.sql"),"\n",include_str!("📦️buffer/🗄️.sql"),"\n",include_str!("🖌️material/🗄️.sql"),"\n",include_str!("🖼️texture/🗄️.sql"),"\n",include_str!("🦴️skin/🗄️.sql"),"\n",include_str!("🎬️animation/🗄️.sql"),"\n",include_str!("🎥️camera/🗄️.sql"));

fn text(value:&Option<String>)->Cell<'_>{value.as_deref().map_or(Cell::Null,Cell::Text)}
fn word(value:u64)->[Cell<'static>;2]{[Cell::Integer((value>>32) as i64),Cell::Integer((value&0xffff_ffff) as i64)]}
fn optional_word(value:Option<u64>)->[Cell<'static>;2]{value.map_or([Cell::Null,Cell::Null],word)}
fn index(value:usize)->[Cell<'static>;2]{word(value as u64)}
fn optional_index(value:Option<usize>)->[Cell<'static>;2]{optional_word(value.map(|value|value as u64))}
fn id(value:Option<i64>)->Cell<'static>{value.map_or(Cell::Null,Cell::Integer)}
fn ordinal(value:usize)->Result<Cell<'static>, ValueError>{Ok(Cell::Integer(i64::try_from(value).map_err(|error| ValueError::new(ValueRefusalKind::WorkLimit, error.to_string()))?))}

struct Write<'c,'p>{projection:Projection<'c,'p>,json_next:i64}
impl<'c,'p> Write<'c,'p>{
 fn new(control:&'c mut SqliteSnapshotControl<'p>)->Result<Self, ValueError>{Ok(Self{projection:Projection::new(SQLITE_SCHEMA,control)?,json_next:1})}
 fn extras(&mut self,extensions:&Option<GltfJson>,extras:&Option<GltfJson>)->Result<[Cell<'static>;2], ValueError>{let extension=extras::project(self,extensions.as_ref())?;let extra=extras::project(self,extras.as_ref())?;Ok([id(extension),id(extra)])}
 fn insert(&mut self,table:&'static str,cells:&[Cell<'_>])->Result<i64, ValueError>{self.projection.insert(table,cells)}
 fn insert_key(&mut self,table:&'static str,key:i64,cells:&[Cell<'_>])->Result<(), ValueError>{self.projection.insert_key(table,key,cells)}
 fn floats(&mut self,table:&'static str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64, ValueError>{insert_ieee754(&mut self.projection,table,cells,columns)}
 fn float_key(&mut self,table:&'static str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(), ValueError>{insert_key_ieee754(&mut self.projection,table,key,cells,columns)}
 fn check(&mut self,count:usize)->Result<(), ValueError>{self.projection.check_rows(count)?;self.projection.checkpoint()}
 fn json_id(&mut self)->Result<i64, ValueError>{let id=self.json_next;self.json_next=self.json_next.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "GLTF extras identity overflow"))?;Ok(id)}
 fn finish(self)->Result<SqliteDatabase, ValueError>{self.projection.finish()}
}

const WIDTHS:&[(&str,usize)]=&[
 ("gltf_document",7),("gltf_asset",8),("gltf_scene",6),("gltf_scene_node",5),("gltf_extension_used",4),("gltf_extension_required",4),("gltf_resolved_buffer",3),("gltf_resolved_byte",4),
 ("gltf_json_value",7),("gltf_json_array_element",4),("gltf_json_object_member",5),
 ("gltf_node",12),("gltf_node_child",5),("gltf_node_matrix",1),("gltf_node_matrix_component",6),("gltf_node_translation",1),("gltf_node_translation_component",6),("gltf_node_rotation",1),("gltf_node_rotation_component",6),("gltf_node_scale",1),("gltf_node_scale_component",6),("gltf_node_weight",6),
 ("gltf_mesh",6),("gltf_mesh_weight",6),("gltf_primitive",11),("gltf_primitive_attribute",6),("gltf_morph_target",3),("gltf_morph_attribute",6),
 ("gltf_buffer",9),("gltf_buffer_view",16),("gltf_accessor",15),("gltf_accessor_max",1),("gltf_accessor_max_component",6),("gltf_accessor_min",1),("gltf_accessor_min_component",6),("gltf_sparse_accessor",3),("gltf_sparse_indices",6),("gltf_sparse_values",5),
 ("gltf_material",20),("gltf_pbr_metallic_roughness",21),("gltf_base_color_texture",7),("gltf_metallic_roughness_texture",7),("gltf_emissive_texture",7),("gltf_normal_texture",10),("gltf_occlusion_texture",10),
 ("gltf_texture",10),("gltf_image",10),("gltf_sampler",14),("gltf_skin",10),("gltf_skin_joint",5),
 ("gltf_animation",6),("gltf_animation_sampler",10),("gltf_animation_channel",7),("gltf_animation_channel_target",6),("gltf_camera",7),("gltf_camera_orthographic",15),("gltf_camera_perspective",15)
];
fn owned<T:semio_framework_dsl_record::DslField>(value:T)->semio_framework_dsl_record::__rt::DecodedFieldOwner<T>{semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,T::retire_decoded)}
fn owned_option<T:semio_framework_dsl_record::DslField>(value:Option<T>)->semio_framework_dsl_record::__rt::DecodedFieldOwner<Option<T>>{semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,|value|{if let Some(value)=value{T::retire_decoded(value);}})}
#[path="💰️backing/🦀️.rs"]mod backing;
use backing::Read;
#[path="🚦️native/🦀️.rs"]mod native;

use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};
impl ArtifactSqliteSnapshot for GltfSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{native::decode(payload,crate::standards::v2_0::subsets::any::schema::snapshot::owned_pack::controlled_spec_producer(),crate::standards::v2_0::subsets::any::schema::snapshot::owned_pack::reconstruct_record_controlled,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{encoding::native_footprint(self,control)?;crate::standards::v2_0::subsets::any::schema::snapshot::owned_pack::encode_native(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<GltfDocument as semio_framework_dsl_record::DslField>::retire_decoded(self.document);}
 const SQLITE_SCHEMA:&'static str=SQLITE_SCHEMA;
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{encoding::native_footprint(self,control)}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
        let mut write=Write::new(control)?;document::project(&mut write,self)?;node::project(&mut write,&self.document.nodes)?;mesh::project(&mut write,&self.document.meshes)?;buffer::project(&mut write,&self.document)?;material::project(&mut write,&self.document.materials)?;texture::project(&mut write,&self.document)?;skin::project(&mut write,&self.document.skins)?;animation::project(&mut write,&self.document.animations)?;camera::project(&mut write,&self.document.cameras)?;write.finish()
        
    }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
        let mut read=Read::new(database,control)?;let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(document::reconstruct(&mut read)?,GltfSnapshot::retire_sqlite_snapshot);let snapshot=owner.as_mut();snapshot.document.nodes=node::reconstruct(&mut read)?;snapshot.document.meshes=mesh::reconstruct(&mut read)?;buffer::reconstruct(&mut read,&mut snapshot.document)?;snapshot.document.materials=material::reconstruct(&mut read)?;texture::reconstruct(&mut read,&mut snapshot.document)?;snapshot.document.skins=skin::reconstruct(&mut read)?;snapshot.document.animations=animation::reconstruct(&mut read)?;snapshot.document.cameras=camera::reconstruct(&mut read)?;read.finish()?;Ok(owner.take())
        
    }
 fn validate_sqlite_snapshot_subset(&self,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<IoOutcome<()>,IoError>{
  (|| -> Result<IoOutcome<()>,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
  if dialect.artifact_kind!="s.stdio.gltf"||dialect.standard!="2.0"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF owned SQLite dialect differs"));}
  validate_sqlite_database_schema(database,SQLITE_SCHEMA,control.limits())?;
  let expected=self.to_sqlite_database(control)?;let mut candidate=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self::from_sqlite_database(database,control)?,Self::retire_sqlite_snapshot);let canonical=candidate.as_mut().to_sqlite_database(control)?;drop(candidate);let total=expected.tables.iter().map(|table|table.rows.len()).sum();let mut completed=0;
  for table in &expected.tables{
   let actual=canonical.tables.iter().find(|actual|actual.name==table.name).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"GLTF owned table missing"))?;
   if actual.rows.len()!=table.rows.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF owned state differs from semantic projection"));}
   for (row,actual) in table.rows.iter().zip(&actual.rows){
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,total)?;completed+=1;
    if actual.rowid!=row.rowid||actual.values!=row.values{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF owned state differs from semantic projection"));}
   }
  }
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,total,total)?;Ok(IoOutcome::clean(()))
  })().map_err(IoError::from_value_error)
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

