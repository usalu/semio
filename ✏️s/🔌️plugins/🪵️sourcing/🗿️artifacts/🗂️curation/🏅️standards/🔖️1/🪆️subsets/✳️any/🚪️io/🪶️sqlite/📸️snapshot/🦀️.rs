//! 🗂️ Individually authored literal catalog and sourcing recipe relational ownership.
use crate::standards::v1::subsets::any::schema::snapshot::CurationSnapshot;
use crate::{ObjectKindExtra,GeometryRecipe,CuratedItem};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{Projection,Cell,NativeEncodingBound,FloatColumn,insert_ieee754,read_binary64,read_binary32}}};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

const BOX:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const FRAME:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
const GLB:&[FloatColumn]=&[FloatColumn::Binary64(3)];
const MESH:&[FloatColumn]=&[FloatColumn::Binary32(3)];
#[path="📏️cells/🦀️.rs"]mod cells;
#[path="💰️backing/🦀️.rs"]mod backing;
fn row_count(snapshot:&CurationSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{
 let mut total=2usize.checked_add(snapshot.curated.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Curation row count overflow"))?;control.check_rows(total)?;
 for(index,extra)in snapshot.stock_extra.iter().enumerate(){if index%256==0{control.checkpoint(phase,index,snapshot.stock_extra.len())?}total=total.checked_add(3).and_then(|n|n.checked_add(extra.typology_path.len())).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Curation row count overflow"))?;if let GeometryRecipe::Mesh{positions,normals,indices}=extra.geometry.as_ref(){total=total.checked_add(positions.len()).and_then(|n|n.checked_add(normals.len())).and_then(|n|n.checked_add(indices.len())).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Curation row count overflow"))?}control.check_rows(total)?}Ok(total)
}
fn list(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<&[semio_framework_dsl_record::FieldValue],semio_framework_value::ValueError>{match value{Some(semio_framework_dsl_record::FieldValue::List(rows))=>Ok(rows),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Curation requires a literal collection").to_string()))}}
fn native_rows(record:&semio_framework_dsl_record::RecordValue,native:&mut semio_framework_value::NativeDecodeControl<'_>,maximum:usize)->Result<(),semio_framework_value::ValueError>{
 let stock=list(record.get(1))?;let mut total=2usize.checked_add(list(record.get(2))?.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Curation row count overflow").to_string()))?;
 native.scoped_stage(|native|->Result<_,semio_framework_value::ValueError>{native.begin_stage(stock.len())?;for value in stock{native.step()?;let row=match value{semio_framework_dsl_record::FieldValue::Record(row)=>row,_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Curation stock requires a literal record").to_string()))};let path=list(row.get(3))?;total=total.checked_add(3).and_then(|n|n.checked_add(path.len())).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Curation row count overflow").to_string()))?;if let Some(semio_framework_dsl_record::FieldValue::Statements(recipes))=row.get(5){if let[(kind,recipe)]=recipes.as_slice(){if kind=="mesh"{for field in[0,1,2]{total=total.checked_add(list(recipe.get(field))?.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Curation row count overflow").to_string()))?}}}}if total>maximum{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Curation native snapshot exceeds row limit").to_string()))}}if total>maximum{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Curation native snapshot exceeds row limit").to_string()))}Ok(())})
}
fn uint(row:&SqliteRow,index:usize)->Result<u32,ValueError>{u32::try_from(row.integer(index)?).map_err(|e|invalid(e.to_string()))}
fn literal_catalog(row:&SqliteRow,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot>,ValueError>{
 if row.integer(1)?!=1{return Err(invalid("Curation catalog has the wrong document owner"))}let child_id=native.copy_text(row.text(2)?)?;let artifact_id=native.copy_text(row.text(3)?)?;let artifact_kind=native.copy_text(row.text(4)?)?;let standard=native.copy_text(row.text(5)?)?;let subset=native.copy_text(row.text(6)?)?;Ok(store::ArtifactChild::new(child_id,semio_framework_artifact_reference::ArtifactRef{artifact_id,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind,standard,subset}}))
}
impl ArtifactSqliteSnapshot for CurationSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  self.validate().map_err(invalid)?;row_count(self,control,SqliteSnapshotPhase::EncodeNative)?;cells::typed(self,control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_rows(2)?;control.check_value_bytes(24)?;let maximum=control.limits().max_rows;let value_maximum=control.limits().max_value_bytes;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{native_rows(record,native,maximum)?;cells::borrowed(record,native,value_maximum)?;let result=Self::__dsl_from_record_controlled(record,native)?;result.validate().map_err(invalid)?;Ok(result)},control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  self.validate().map_err(invalid)?;row_count(self,control,SqliteSnapshotPhase::EncodeNative)?;cells::typed(self,control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=NativeEncodingBound::new(control)?;bound.add(8192)?;let target=&self.catalog.target;for text in[&self.catalog.child_id,&target.artifact_id,&target.dialect.artifact_kind,&target.dialect.standard,&target.dialect.subset]{bound.repeated(text.len(),24)?}
  for extra in &self.stock_extra{bound.add(2048)?;for text in[&extra.id,&extra.name,&extra.module_id]{bound.repeated(text.len(),24)?}for segment in &extra.typology_path{bound.add(64)?;bound.repeated(segment.len(),24)?}match extra.geometry.as_ref(){GeometryRecipe::Mesh{positions,normals,indices}=>{bound.repeated(positions.len(),128)?;bound.repeated(normals.len(),128)?;bound.repeated(indices.len(),64)?},GeometryRecipe::Glb{url,..}=>bound.repeated(url.len(),24)?,_=>{}}}
  for item in &self.curated{bound.add(128)?;bound.repeated(item.object_id.len(),24)?}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;self.validate().map_err(invalid)?;row_count(self,control,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;out.insert_key("curation_document",1,&[])?;let t=&self.catalog.target;out.insert("curation_catalog",&[Cell::Integer(1),Cell::Text(&self.catalog.child_id),Cell::Text(&t.artifact_id),Cell::Text(&t.dialect.artifact_kind),Cell::Text(&t.dialect.standard),Cell::Text(&t.dialect.subset)])?;
  for(ordinal,extra)in self.stock_extra.iter().enumerate(){let id=out.insert("curation_stock_extra",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Text(&extra.id),Cell::Text(&extra.name),Cell::Text(&extra.module_id),Cell::Integer(i64::from(extra.availability))])?;for(ordinal,segment)in extra.typology_path.iter().enumerate(){out.insert("curation_typology_segment",&[Cell::Integer(id),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Text(segment)])?;}
   let kind=match extra.geometry.as_ref(){GeometryRecipe::Box{..}=>"box",GeometryRecipe::Frame{..}=>"frame",GeometryRecipe::Slab{..}=>"slab",GeometryRecipe::Mesh{..}=>"mesh",GeometryRecipe::Glb{..}=>"glb"};let key=out.insert("curation_geometry",&[Cell::Integer(id),Cell::Text(kind)])?;
   match extra.geometry.as_ref(){
    GeometryRecipe::Box{width,height,depth}=>{insert_ieee754(&mut out,"curation_box",&[Cell::Integer(key),Cell::Real(*width),Cell::Real(*height),Cell::Real(*depth)],BOX)?;},
    GeometryRecipe::Frame{width,height,depth,profile}=>{insert_ieee754(&mut out,"curation_frame",&[Cell::Integer(key),Cell::Real(*width),Cell::Real(*height),Cell::Real(*depth),Cell::Real(*profile)],FRAME)?;},
    GeometryRecipe::Slab{width,depth,thickness}=>{insert_ieee754(&mut out,"curation_slab",&[Cell::Integer(key),Cell::Real(*width),Cell::Real(*depth),Cell::Real(*thickness)],BOX)?;},
    GeometryRecipe::Glb{url,extent}=>{insert_ieee754(&mut out,"curation_glb",&[Cell::Integer(key),Cell::Text(url),Cell::Real(*extent)],GLB)?;},
    GeometryRecipe::Mesh{positions,normals,indices}=>{let mesh=out.insert("curation_mesh",&[Cell::Integer(key)])?;for(ordinal,value)in positions.iter().enumerate(){insert_ieee754(&mut out,"curation_mesh_position",&[Cell::Integer(mesh),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Float32(*value)],MESH)?;}for(ordinal,value)in normals.iter().enumerate(){insert_ieee754(&mut out,"curation_mesh_normal",&[Cell::Integer(mesh),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Float32(*value)],MESH)?;}for(ordinal,value)in indices.iter().enumerate(){out.insert("curation_mesh_index",&[Cell::Integer(mesh),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Integer(i64::from(*value))])?;}}
   }
  }
  for(ordinal,item)in self.curated.iter().enumerate(){out.insert("curation_curated",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Text(&item.object_id),Cell::Integer(i64::from(item.count))])?;}out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

