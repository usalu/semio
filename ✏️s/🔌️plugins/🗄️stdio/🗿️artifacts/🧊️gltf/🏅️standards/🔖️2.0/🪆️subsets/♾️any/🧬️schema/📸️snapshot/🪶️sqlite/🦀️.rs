//! 🧊️ Explicit GLTF semantic entities retain independently owned document and buffer state.
use super::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,io_schema::{ArtifactDialect,IoError,IoOutcome},sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Cell,Projection,Reconstruction,FloatColumn,FloatRow,insert_ieee754,insert_key_ieee754}}};
use std::collections::{BTreeMap,BTreeSet};

#[path="📄️document/🦀️.rs"]mod document;
#[path="🌳️node/🦀️.rs"]mod node;
#[path="🏔️mesh/🦀️.rs"]mod mesh;
#[path="📦️buffer/🦀️.rs"]mod buffer;
#[path="🖌️material/🦀️.rs"]mod material;
#[path="🖼️texture/🦀️.rs"]mod texture;
#[path="🦴️skin/🦀️.rs"]mod skin;
#[path="🎬️animation/🦀️.rs"]mod animation;
#[path="🎥️camera/🦀️.rs"]mod camera;
#[path="🧩️extras/🦀️.rs"]mod extras;
#[path="📏️encoding/🦀️.rs"]mod encoding;

pub const SQLITE_SCHEMA:&str=concat!(include_str!("📄️document/🗄️.sql"),"\n",include_str!("🧩️extras/🗄️.sql"),"\n",include_str!("🌳️node/🗄️.sql"),"\n",include_str!("🏔️mesh/🗄️.sql"),"\n",include_str!("📦️buffer/🗄️.sql"),"\n",include_str!("🖌️material/🗄️.sql"),"\n",include_str!("🖼️texture/🗄️.sql"),"\n",include_str!("🦴️skin/🗄️.sql"),"\n",include_str!("🎬️animation/🗄️.sql"),"\n",include_str!("🎥️camera/🗄️.sql"));

fn text(value:&Option<String>)->Cell<'_>{value.as_deref().map_or(Cell::Null,Cell::Text)}
fn word(value:u64)->[Cell<'static>;2]{[Cell::Integer((value>>32) as i64),Cell::Integer((value&0xffff_ffff) as i64)]}
fn optional_word(value:Option<u64>)->[Cell<'static>;2]{value.map_or([Cell::Null,Cell::Null],word)}
fn index(value:usize)->[Cell<'static>;2]{word(value as u64)}
fn optional_index(value:Option<usize>)->[Cell<'static>;2]{optional_word(value.map(|value|value as u64))}
fn id(value:Option<i64>)->Cell<'static>{value.map_or(Cell::Null,Cell::Integer)}
fn ordinal(value:usize)->Result<Cell<'static>,String>{Ok(Cell::Integer(i64::try_from(value).map_err(|error|error.to_string())?))}

struct Write<'c,'p>{projection:Projection<'c,'p>,json_next:i64}
impl<'c,'p> Write<'c,'p>{
 fn new(control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,String>{Ok(Self{projection:Projection::new(SQLITE_SCHEMA,control)?,json_next:1})}
 fn extras(&mut self,extensions:&Option<GltfJson>,extras:&Option<GltfJson>)->Result<[Cell<'static>;2],String>{let extension=extras::project(self,extensions.as_ref())?;let extra=extras::project(self,extras.as_ref())?;Ok([id(extension),id(extra)])}
 fn insert(&mut self,table:&'static str,cells:&[Cell<'_>])->Result<i64,String>{self.projection.insert(table,cells)}
 fn insert_key(&mut self,table:&'static str,key:i64,cells:&[Cell<'_>])->Result<(),String>{self.projection.insert_key(table,key,cells)}
 fn floats(&mut self,table:&'static str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,String>{insert_ieee754(&mut self.projection,table,cells,columns)}
 fn float_key(&mut self,table:&'static str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),String>{insert_key_ieee754(&mut self.projection,table,key,cells,columns)}
 fn check(&mut self,count:usize)->Result<(),String>{self.projection.check_rows(count)?;self.projection.checkpoint()}
 fn json_id(&mut self)->Result<i64,String>{let id=self.json_next;self.json_next=self.json_next.checked_add(1).ok_or("GLTF extras identity overflow")?;Ok(id)}
 fn finish(self)->Result<SqliteDatabase,String>{self.projection.finish()}
}

type Group<'a>=BTreeMap<i64,BTreeMap<usize,&'a SqliteRow>>;
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
struct Read<'a,'c,'p>{database:&'a SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>,keys:BTreeMap<&'static str,BTreeMap<i64,&'a SqliteRow>>,groups:BTreeMap<&'static str,Group<'a>>,used:BTreeMap<&'static str,BTreeSet<i64>>,visits:usize}
impl<'a,'c,'p> Read<'a,'c,'p>{
 fn new(database:&'a SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,String>{
  validate_sqlite_database_schema(database,SQLITE_SCHEMA,control.limits()).map_err(|error|error.to_string())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;
  let mut rows=0usize;let mut bytes=0usize;let mut visited=0usize;
  for table in &database.tables{let width=WIDTHS.iter().find(|(name,_)|name.eq_ignore_ascii_case(&table.name)).ok_or("GLTF undeclared table")?.1;rows=rows.checked_add(table.rows.len()).ok_or("GLTF row count overflow")?;control.check_rows(rows)?;for row in &table.rows{if row.values.len()!=width{return Err("GLTF authored row width differs".into());}for value in &row.values{bytes=bytes.checked_add(match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(value)=>value.len(),SqliteValue::Blob(value)=>value.len()}).ok_or("GLTF value byte count overflow")?;control.check_value_bytes(bytes)?;}visited=visited.checked_add(1).ok_or("GLTF traversal overflow")?;if visited%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,visited,0)?;}}}
  Ok(Self{database,control,keys:BTreeMap::new(),groups:BTreeMap::new(),used:BTreeMap::new(),visits:0})
 }
 fn checkpoint(&mut self)->Result<(),String>{self.visits=self.visits.checked_add(1).ok_or("GLTF traversal overflow")?;if self.visits%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.visits,0)?;}Ok(())}
 fn consume(&mut self,table:&'static str,row:&'a SqliteRow)->Result<&'a SqliteRow,String>{if row.rowid<1||row.integer(0)?!=row.rowid||!self.used.entry(table).or_default().insert(row.rowid){return Err("GLTF entity identity or exclusive ownership differs".into());}self.checkpoint()?;Ok(row)}
 fn ensure_keys(&mut self,table:&'static str)->Result<(),String>{
  if !self.keys.contains_key(table){let mut keys=BTreeMap::new();for row in &self.database.table(table)?.rows{if row.rowid<1||row.integer(0)?!=row.rowid||keys.insert(row.rowid,row).is_some(){return Err("GLTF identities must be positive and unique".into());}self.checkpoint()?;}self.keys.insert(table,keys);}
  Ok(())
 }
 fn key(&mut self,table:&'static str,key:i64)->Result<&'a SqliteRow,String>{
  self.ensure_keys(table)?;
  let row=*self.keys.get(table).and_then(|keys|keys.get(&key)).ok_or("GLTF required entity is missing")?;self.consume(table,row)
 }
 fn optional_key(&mut self,table:&'static str,key:i64)->Result<Option<&'a SqliteRow>,String>{self.ensure_keys(table)?;if self.keys.get(table).is_some_and(|keys|keys.contains_key(&key)){self.key(table,key).map(Some)}else{Ok(None)}}
 fn rows(&mut self,table:&'static str,owner_column:usize,owner:i64,ordinal_column:usize)->Result<Vec<&'a SqliteRow>,String>{
  if !self.groups.contains_key(table){let mut groups:Group<'a>=BTreeMap::new();for row in &self.database.table(table)?.rows{let owner=row.integer(owner_column)?;let ordinal=usize::try_from(row.integer(ordinal_column)?).map_err(|error|error.to_string())?;if owner<1||groups.entry(owner).or_default().insert(ordinal,row).is_some(){return Err("GLTF relationship owner or ordinal differs".into());}self.checkpoint()?;}self.groups.insert(table,groups);}
  let group=self.groups.get_mut(table).and_then(|groups|groups.remove(&owner)).unwrap_or_default();self.control.check_rows(group.len())?;let mut result=Vec::with_capacity(group.len());
  for (ordinal,(actual,row)) in group.into_iter().enumerate(){if ordinal!=actual{return Err("GLTF relationship ordinals must be contiguous".into());}result.push(self.consume(table,row)?);}Ok(result)
 }
 fn text(&mut self,row:&SqliteRow,column:usize)->Result<String,String>{Reconstruction::new(self.control)?.text(row.text(column)?)}
 fn optional_text(&mut self,row:&SqliteRow,column:usize)->Result<Option<String>,String>{row.optional_text(column)?.map(|text|Reconstruction::new(self.control)?.text(text)).transpose()}
 fn scalar(&mut self)->Result<(),String>{Reconstruction::new(self.control)?.scalar()}
 fn word(&mut self,row:&SqliteRow,column:usize)->Result<u64,String>{self.scalar()?;let high=u32::try_from(row.integer(column)?).map_err(|error|error.to_string())?;let low=u32::try_from(row.integer(column+1)?).map_err(|error|error.to_string())?;Ok((u64::from(high)<<32)|u64::from(low))}
 fn optional_word(&mut self,row:&SqliteRow,column:usize)->Result<Option<u64>,String>{match(row.values.get(column),row.values.get(column+1)){(Some(SqliteValue::Null),Some(SqliteValue::Null))=>Ok(None),(Some(SqliteValue::Integer(_)),Some(SqliteValue::Integer(_)))=>self.word(row,column).map(Some),_=>Err("GLTF optional unsigned words differ".into())}}
 fn index(&mut self,row:&SqliteRow,column:usize)->Result<usize,String>{usize::try_from(self.word(row,column)?).map_err(|error|error.to_string())}
 fn optional_index(&mut self,row:&SqliteRow,column:usize)->Result<Option<usize>,String>{self.optional_word(row,column)?.map(|value|usize::try_from(value).map_err(|error|error.to_string())).transpose()}
 fn boolean(&mut self,row:&SqliteRow,column:usize)->Result<bool,String>{self.scalar()?;match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err("GLTF boolean must be zero or one".into())}}
 fn json(&mut self,row:&SqliteRow,column:usize)->Result<Option<GltfJson>,String>{match row.values.get(column){Some(SqliteValue::Null)=>Ok(None),Some(SqliteValue::Integer(id)) if *id>0=>extras::reconstruct(self,*id).map(Some),_=>Err("GLTF extras root must be a positive entity or absent".into())}}
 fn finish(self)->Result<(),String>{for table in &self.database.tables{let used=self.used.iter().find(|(name,_)|name.eq_ignore_ascii_case(&table.name)).map(|(_,used)|used);for row in &table.rows{if !used.is_some_and(|used|used.contains(&row.rowid)){return Err(format!("GLTF unowned entity remains in {}",table.name));}}}self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.visits,self.visits)}
}

impl ArtifactSqliteSnapshot for GltfSnapshot{
 const SQLITE_SCHEMA:&'static str=SQLITE_SCHEMA;
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{encoding::check(self,control)}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{let mut write=Write::new(control)?;document::project(&mut write,self)?;node::project(&mut write,&self.document.nodes)?;mesh::project(&mut write,&self.document.meshes)?;buffer::project(&mut write,&self.document)?;material::project(&mut write,&self.document.materials)?;texture::project(&mut write,&self.document)?;skin::project(&mut write,&self.document.skins)?;animation::project(&mut write,&self.document.animations)?;camera::project(&mut write,&self.document.cameras)?;write.finish()}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{let mut read=Read::new(database,control)?;let mut snapshot=document::reconstruct(&mut read)?;snapshot.document.nodes=node::reconstruct(&mut read)?;snapshot.document.meshes=mesh::reconstruct(&mut read)?;buffer::reconstruct(&mut read,&mut snapshot.document)?;snapshot.document.materials=material::reconstruct(&mut read)?;texture::reconstruct(&mut read,&mut snapshot.document)?;snapshot.document.skins=skin::reconstruct(&mut read)?;snapshot.document.animations=animation::reconstruct(&mut read)?;snapshot.document.cameras=camera::reconstruct(&mut read)?;read.finish()?;Ok(snapshot)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<IoOutcome<()>,IoError>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.gltf"||dialect.standard!="2.0"||dialect.subset!="*"{return Err(String::from("GLTF owned SQLite dialect differs").into());}let row=database.table("gltf_document")?.single_row()?;if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("GLTF document identity differs from its semantic projection").into());}Ok(IoOutcome::clean(()))}
}
