//! 🗂️ Individually authored literal catalog and sourcing recipe relational ownership.
use super::CurationSnapshot;
use crate::{ObjectKindExtra,GeometryRecipe,CuratedItem};
use std::collections::{BTreeMap,BTreeSet};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound,FloatColumn,insert_ieee754,read_binary64,read_binary32}}};
const BOX:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const FRAME:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
const GLB:&[FloatColumn]=&[FloatColumn::Binary64(3)];
const MESH:&[FloatColumn]=&[FloatColumn::Binary32(3)];
type Single<'a>=BTreeMap<i64,&'a SqliteRow>;
type Members<'a>=BTreeMap<i64,BTreeMap<usize,&'a SqliteRow>>;
fn row_count(snapshot:&CurationSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,String>{
 let mut total=2usize.checked_add(snapshot.curated.len()).ok_or("Curation row count overflow")?;control.check_rows(total)?;
 for(index,extra)in snapshot.stock_extra.iter().enumerate(){if index%256==0{control.checkpoint(phase,index,snapshot.stock_extra.len())?}total=total.checked_add(3).and_then(|n|n.checked_add(extra.typology_path.len())).ok_or("Curation row count overflow")?;if let GeometryRecipe::Mesh{positions,normals,indices}=extra.geometry.as_ref(){total=total.checked_add(positions.len()).and_then(|n|n.checked_add(normals.len())).and_then(|n|n.checked_add(indices.len())).ok_or("Curation row count overflow")?}control.check_rows(total)?}Ok(total)
}
fn list(value:Option<&dsl::FieldValue>)->Result<&[dsl::FieldValue],dsl::TextError>{match value{Some(dsl::FieldValue::List(rows))=>Ok(rows),None|Some(dsl::FieldValue::Absent)=>Ok(&[]),_=>Err(dsl::__rt::field_error("Curation requires a literal collection"))}}
fn native_rows(record:&dsl::RecordValue,native:&mut dsl::NativeDecodeControl<'_>,maximum:usize)->Result<(),dsl::TextError>{
 let stock=list(record.get(1))?;let mut total=2usize.checked_add(list(record.get(2))?.len()).ok_or_else(||dsl::__rt::field_error("Curation row count overflow"))?;
 native.scoped_stage(|native|->Result<_,dsl::TextError>{native.begin_stage(stock.len()).map_err(dsl::__rt::field_error)?;for value in stock{native.step().map_err(dsl::__rt::field_error)?;let row=match value{dsl::FieldValue::Record(row)=>row,_=>return Err(dsl::__rt::field_error("Curation stock requires a literal record"))};let path=list(row.get(3))?;total=total.checked_add(3).and_then(|n|n.checked_add(path.len())).ok_or_else(||dsl::__rt::field_error("Curation row count overflow"))?;if let Some(dsl::FieldValue::Statements(recipes))=row.get(5){if let[(kind,recipe)]=recipes.as_slice(){if kind=="mesh"{for field in[0,1,2]{total=total.checked_add(list(recipe.get(field))?.len()).ok_or_else(||dsl::__rt::field_error("Curation row count overflow"))?}}}}if total>maximum{return Err(dsl::__rt::field_error("Curation native snapshot exceeds row limit"))}}if total>maximum{return Err(dsl::__rt::field_error("Curation native snapshot exceeds row limit"))}Ok(())})
}
fn table<'a>(database:&'a SqliteDatabase,name:&str,columns:usize,native:&mut dsl::NativeDecodeControl<'_>)->Result<&'a[SqliteRow],String>{
 let rows=&database.table(name)?.rows;native.charge(rows.len().checked_mul(128).ok_or("Curation identity workspace overflow")?)?;
 native.scoped_stage(|native|->Result<_,String>{native.begin_stage(rows.len())?;let mut ids=BTreeSet::new();for row in rows{native.step()?;if row.rowid<=0||row.values.len()!=columns||row.integer(0)?!=row.rowid||!ids.insert(row.rowid){return Err("Curation requires complete positive aliased unique entities".into())}}Ok(())})?;Ok(rows)
}
fn singles<'a>(rows:&'a[SqliteRow],native:&mut dsl::NativeDecodeControl<'_>)->Result<Single<'a>,String>{
 native.charge(rows.len().checked_mul(128).ok_or("Curation relationship workspace overflow")?)?;native.scoped_stage(|native|->Result<_,String>{native.begin_stage(rows.len())?;let mut result=BTreeMap::new();for row in rows{native.step()?;if result.insert(row.integer(1)?,row).is_some(){return Err("Curation requires one matching recipe entity".into())}}Ok(result)})
}
fn members<'a>(rows:&'a[SqliteRow],native:&mut dsl::NativeDecodeControl<'_>)->Result<Members<'a>,String>{
 native.charge(rows.len().checked_mul(256).ok_or("Curation ordinal workspace overflow")?)?;native.scoped_stage(|native|->Result<_,String>{native.begin_stage(rows.len())?;let mut result:Members<'a>=BTreeMap::new();for row in rows{native.step()?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|e.to_string())?;if result.entry(row.integer(1)?).or_default().insert(ordinal,row).is_some(){return Err("Curation relationships require unique ordinals".into())}}Ok(result)})
}
fn one<'a>(rows:&mut Single<'a>,parent:i64)->Result<&'a SqliteRow,String>{rows.remove(&parent).ok_or_else(||"Curation requires one complete matching recipe".into())}
fn ordered<'a>(rows:&mut Members<'a>,parent:i64,native:&mut dsl::NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,String>{
 let rows=rows.remove(&parent).unwrap_or_default();native.scoped_stage(|native|->Result<_,String>{native.begin_stage(rows.len())?;let mut result=native.allocate_vec(rows.len())?;for(index,(ordinal,row))in rows.into_iter().enumerate(){native.step()?;if ordinal!=index{return Err("Curation relationship ordinals must be dense".into())}result.push(row)}Ok(result)})
}
fn uint(row:&SqliteRow,index:usize)->Result<u32,String>{u32::try_from(row.integer(index)?).map_err(|e|e.to_string())}
fn literal_catalog(row:&SqliteRow,native:&mut dsl::NativeDecodeControl<'_>)->Result<store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot>,String>{
 if row.integer(1)?!=1{return Err("Curation catalog has the wrong document owner".into())}let child_id=native.copy_text(row.text(2)?)?;let artifact_id=native.copy_text(row.text(3)?)?;let artifact_kind=native.copy_text(row.text(4)?)?;let standard=native.copy_text(row.text(5)?)?;let subset=native.copy_text(row.text(6)?)?;Ok(store::ArtifactChild::new(child_id,store::io_schema::ArtifactRef{artifact_id,dialect:store::io_schema::ArtifactDialect{artifact_kind,standard,subset}}))
}
impl ArtifactSqliteSnapshot for CurationSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{
  self.validate()?;row_count(self,control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  control.check_rows(2)?;let maximum=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{native_rows(record,native,maximum)?;let result=Self::__dsl_from_record_controlled(record,native)?;result.validate().map_err(dsl::__rt::field_error)?;Ok(result)},control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{
  self.validate()?;row_count(self,control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=NativeEncodingBound::new(control)?;bound.add(8192)?;let target=&self.catalog.target;for text in[&self.catalog.child_id,&target.artifact_id,&target.dialect.artifact_kind,&target.dialect.standard,&target.dialect.subset]{bound.repeated(text.len(),24)?}
  for extra in &self.stock_extra{bound.add(2048)?;for text in[&extra.id,&extra.name,&extra.module_id]{bound.repeated(text.len(),24)?}for segment in &extra.typology_path{bound.add(64)?;bound.repeated(segment.len(),24)?}match extra.geometry.as_ref(){GeometryRecipe::Mesh{positions,normals,indices}=>{bound.repeated(positions.len(),128)?;bound.repeated(normals.len(),128)?;bound.repeated(indices.len(),64)?},GeometryRecipe::Glb{url,..}=>bound.repeated(url.len(),24)?,_=>{}}}
  for item in &self.curated{bound.add(128)?;bound.repeated(item.object_id.len(),24)?}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;self.validate()?;row_count(self,control,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;out.insert_key("curation_document",1,&[])?;let t=&self.catalog.target;out.insert("curation_catalog",&[Cell::Integer(1),Cell::Text(&self.catalog.child_id),Cell::Text(&t.artifact_id),Cell::Text(&t.dialect.artifact_kind),Cell::Text(&t.dialect.standard),Cell::Text(&t.dialect.subset)])?;
  for(ordinal,extra)in self.stock_extra.iter().enumerate(){let id=out.insert("curation_stock_extra",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Text(&extra.id),Cell::Text(&extra.name),Cell::Text(&extra.module_id),Cell::Integer(i64::from(extra.availability))])?;for(ordinal,segment)in extra.typology_path.iter().enumerate(){out.insert("curation_typology_segment",&[Cell::Integer(id),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Text(segment)])?;}
   let kind=match extra.geometry.as_ref(){GeometryRecipe::Box{..}=>"box",GeometryRecipe::Frame{..}=>"frame",GeometryRecipe::Slab{..}=>"slab",GeometryRecipe::Mesh{..}=>"mesh",GeometryRecipe::Glb{..}=>"glb"};let key=out.insert("curation_geometry",&[Cell::Integer(id),Cell::Text(kind)])?;
   match extra.geometry.as_ref(){
    GeometryRecipe::Box{width,height,depth}=>{insert_ieee754(&mut out,"curation_box",&[Cell::Integer(key),Cell::Real(*width),Cell::Real(*height),Cell::Real(*depth)],BOX)?;},
    GeometryRecipe::Frame{width,height,depth,profile}=>{insert_ieee754(&mut out,"curation_frame",&[Cell::Integer(key),Cell::Real(*width),Cell::Real(*height),Cell::Real(*depth),Cell::Real(*profile)],FRAME)?;},
    GeometryRecipe::Slab{width,depth,thickness}=>{insert_ieee754(&mut out,"curation_slab",&[Cell::Integer(key),Cell::Real(*width),Cell::Real(*depth),Cell::Real(*thickness)],BOX)?;},
    GeometryRecipe::Glb{url,extent}=>{insert_ieee754(&mut out,"curation_glb",&[Cell::Integer(key),Cell::Text(url),Cell::Real(*extent)],GLB)?;},
    GeometryRecipe::Mesh{positions,normals,indices}=>{let mesh=out.insert("curation_mesh",&[Cell::Integer(key)])?;for(ordinal,value)in positions.iter().enumerate(){insert_ieee754(&mut out,"curation_mesh_position",&[Cell::Integer(mesh),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Float32(*value)],MESH)?;}for(ordinal,value)in normals.iter().enumerate(){insert_ieee754(&mut out,"curation_mesh_normal",&[Cell::Integer(mesh),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Float32(*value)],MESH)?;}for(ordinal,value)in indices.iter().enumerate(){out.insert("curation_mesh_index",&[Cell::Integer(mesh),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Integer(i64::from(*value))])?;}}
   }
  }
  for(ordinal,item)in self.curated.iter().enumerate(){out.insert("curation_curated",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Text(&item.object_id),Cell::Integer(i64::from(item.count))])?;}out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|e|e.to_string())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;let maximum=control.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=dsl::NativeDecodeControl::new(maximum,&mut progress);native.charge(std::mem::size_of::<Self>())?;
  let documents=table(database,"curation_document",1,&mut native)?;let catalogs=table(database,"curation_catalog",7,&mut native)?;let stock=table(database,"curation_stock_extra",7,&mut native)?;let path=table(database,"curation_typology_segment",4,&mut native)?;let geometry=table(database,"curation_geometry",3,&mut native)?;
  let bx=table(database,"curation_box",11,&mut native)?;let fr=table(database,"curation_frame",14,&mut native)?;let sl=table(database,"curation_slab",11,&mut native)?;let me=table(database,"curation_mesh",2,&mut native)?;let gl=table(database,"curation_glb",6,&mut native)?;let p=table(database,"curation_mesh_position",6,&mut native)?;let n=table(database,"curation_mesh_normal",6,&mut native)?;let i=table(database,"curation_mesh_index",4,&mut native)?;let selection=table(database,"curation_curated",5,&mut native)?;
  if documents.len()!=1||documents[0].rowid!=1||catalogs.len()!=1{return Err("Curation requires one document and one mandatory Kit catalog".into())}let catalog=literal_catalog(&catalogs[0],&mut native)?;
  let mut recipes=singles(geometry,&mut native)?;let mut boxes=singles(bx,&mut native)?;let mut frames=singles(fr,&mut native)?;let mut slabs=singles(sl,&mut native)?;let mut meshes=singles(me,&mut native)?;let mut glbs=singles(gl,&mut native)?;
  let mut paths=members(path,&mut native)?;let mut positions=members(p,&mut native)?;let mut normals=members(n,&mut native)?;let mut indices=members(i,&mut native)?;let mut stocks=members(stock,&mut native)?;let mut selections=members(selection,&mut native)?;
  let mut stock_extra=native.allocate_vec::<ObjectKindExtra>(stock.len())?;
  for row in ordered(&mut stocks,1,&mut native)?{let recipe=one(&mut recipes,row.rowid)?;let geometry=match recipe.text(2)?{
   "box"=>{let r=one(&mut boxes,recipe.rowid)?;GeometryRecipe::Box{width:read_binary64(r,2,BOX)?,height:read_binary64(r,3,BOX)?,depth:read_binary64(r,4,BOX)?}},
   "frame"=>{let r=one(&mut frames,recipe.rowid)?;GeometryRecipe::Frame{width:read_binary64(r,2,FRAME)?,height:read_binary64(r,3,FRAME)?,depth:read_binary64(r,4,FRAME)?,profile:read_binary64(r,5,FRAME)?}},
   "slab"=>{let r=one(&mut slabs,recipe.rowid)?;GeometryRecipe::Slab{width:read_binary64(r,2,BOX)?,depth:read_binary64(r,3,BOX)?,thickness:read_binary64(r,4,BOX)?}},
   "glb"=>{let r=one(&mut glbs,recipe.rowid)?;GeometryRecipe::Glb{url:native.copy_text(r.text(2)?)?,extent:read_binary64(r,3,GLB)?}},
   "mesh"=>{let r=one(&mut meshes,recipe.rowid)?;let pr=ordered(&mut positions,r.rowid,&mut native)?;let nr=ordered(&mut normals,r.rowid,&mut native)?;let ir=ordered(&mut indices,r.rowid,&mut native)?;let mut positions=native.allocate_vec(pr.len())?;let mut normals=native.allocate_vec(nr.len())?;let mut indices=native.allocate_vec(ir.len())?;native.scoped_stage(|native|->Result<_,String>{native.begin_stage(pr.len()+nr.len()+ir.len())?;for row in pr{native.step()?;positions.push(read_binary32(row,3,MESH)?)}for row in nr{native.step()?;normals.push(read_binary32(row,3,MESH)?)}for row in ir{native.step()?;indices.push(uint(row,3)?)}Ok(())})?;GeometryRecipe::Mesh{positions,normals,indices}},
   _=>return Err("Curation recipe discriminant is undeclared".into())
  };let path=ordered(&mut paths,row.rowid,&mut native)?;let mut typology_path=native.allocate_vec(path.len())?;native.scoped_stage(|native|->Result<_,String>{native.begin_stage(path.len())?;for row in path{native.step()?;typology_path.push(native.copy_text(row.text(3)?)?)}Ok(())})?;native.charge(std::mem::size_of::<GeometryRecipe>())?;stock_extra.push(ObjectKindExtra{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,module_id:native.copy_text(row.text(5)?)?,availability:uint(row,6)?,typology_path,geometry:Box::new(geometry)});native.step()?;}
  let mut curated=native.allocate_vec::<CuratedItem>(selection.len())?;for row in ordered(&mut selections,1,&mut native)?{native.step()?;curated.push(CuratedItem{object_id:native.copy_text(row.text(3)?)?,count:uint(row,4)?})}
  if !recipes.is_empty()||!boxes.is_empty()||!frames.is_empty()||!slabs.is_empty()||!meshes.is_empty()||!glbs.is_empty()||!paths.is_empty()||!positions.is_empty()||!normals.is_empty()||!indices.is_empty()||!stocks.is_empty()||!selections.is_empty(){return Err("Curation contains unmatched owned entities".into())}let result=Self{catalog,stock_extra,curated};result.validate()?;native.checkpoint()?;Ok(result)
 }
}
