//! 💬️ Handwritten topic, viewpoint, camera and component relationships.
use crate::standards::v2_1::subsets::any::schema::snapshot::*;
use std::collections::BTreeMap;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{NativeEncodingBound,Cell,RowWriter,FloatColumn,FloatRow,reconstruct_text,reconstruct_blob},validate_sqlite_database_schema,SnapshotEncoding,SqliteDatabase,SqliteDatabaseLimits,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase},ArtifactSqliteSnapshot};

fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}

type Entities<'a>=BTreeMap<i64,&'a SqliteRow>;
const CAMERA_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10)];
const PARAMETER_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(1)];
fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(),ValueError>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn entities<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{let rows=&database.table(table)?.rows;let mut result=BTreeMap::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||result.insert(row.rowid,row).is_some(){return Err(invalid(format!("{table} requires unique positive aliased identities and exact columns")));}}Ok(result)}
fn same_owner(rows:&Entities<'_>,parents:&Entities<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{for(position,&id)in rows.keys().enumerate(){checkpoint(control,position,rows.len())?;if !parents.contains_key(&id){return Err(invalid("BCF one-to-one entity has an unknown owner"));}}Ok(())}
fn groups<'a>(rows:&Entities<'a>,parents:&Entities<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<&'a SqliteRow>>,ValueError>{let mut result=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(position,&row)in rows.values().enumerate(){checkpoint(control,position,rows.len())?;let owner=row.integer(1)?;if !parents.contains_key(&owner){return Err(invalid("BCF ordered entity has an unknown owner"));}result.entry(owner).or_default().push(row);}for(group,rows)in result.values_mut().enumerate(){checkpoint(control,group,0)?;let mut slots=vec![None;rows.len()];for(position,&row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error|invalid(error.to_string()))?;if slots.get_mut(ordinal).ok_or_else(||invalid("BCF ordinals must be dense"))?.replace(row).is_some(){return Err(invalid("BCF ordinals must be unique"));}}let mut ordered=Vec::new();for(position,row)in slots.into_iter().enumerate(){checkpoint(control,position,rows.len())?;ordered.push(row.ok_or_else(||invalid("BCF ordinals must be dense"))?);}*rows=ordered;}Ok(result)}
fn write_strings(out:&mut RowWriter<'_,'_>,table:&str,owner:i64,strings:&[String])->Result<(),ValueError>{for(ordinal,value)in strings.iter().enumerate(){out.insert(table,&[Cell::Integer(owner),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Text(value)])?;}Ok(())}
fn read_strings(rows:Option<&Vec<&SqliteRow>>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<String>,ValueError>{let mut strings=Vec::new();for row in rows.into_iter().flatten(){checkpoint(control,strings.len(),0)?;strings.push(reconstruct_text(control,row.text(3)?)?);}Ok(strings)}
fn boolean(row:&SqliteRow,column:usize)->Result<bool,ValueError>{match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("BCF visibility must be boolean"))}}
fn point(row:FloatRow<'_>,column:usize)->Result<BcfPoint3,ValueError>{Ok(BcfPoint3{x:row.real(column)?,y:row.real(column+1)?,z:row.real(column+2)?})}


#[path="🧮️semantic/🦀️.rs"]mod semantic;
/// 🧮️ Admits the complete authored static relational recipe before owned construction.
fn admit_layout(limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 const WIDTHS:[(&str,usize);15]=[("bcf_document",3),("bcf_topic",10),("bcf_topic_label",4),("bcf_comment",8),("bcf_viewpoint",4),("bcf_camera",29),("bcf_perspective_camera",4),("bcf_orthogonal_camera",4),("bcf_components",2),("bcf_selection",4),("bcf_visibility_exception",4),("bcf_coloring",4),("bcf_coloring_component",4),("bcf_snapshot_image",2),("bcf_raw_part",5)];
 let mut schema=0usize;let mut count=0usize;for statement in BcfSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).filter(|statement|!statement.is_empty()){schema=schema.checked_add(statement.len()).ok_or_else(||work("BCF schema extent overflow"))?;count+=1;}for(name,width)in WIDTHS{schema=schema.checked_add(name.len()).ok_or_else(||work("BCF schema extent overflow"))?;if width>limits.max_columns{return Err(work("BCF columns exceed caller limit"))}}
 if count!=WIDTHS.len(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"BCF authored table extent changed"))}if schema>limits.max_schema_bytes||BcfSnapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes||count>limits.max_tables||limits.max_rows<1{return Err(work("BCF relational metadata exceeds caller limit"))}Ok(())
}
/// 🫳️ Uses every actual authored cell for typed native admission without output ownership.
fn admit_native(snapshot:&BcfSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{admit_layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
fn visit_rows(snapshot:&BcfSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
  out.insert("bcf_document",&[Cell::Text(&snapshot.schema),Cell::Text(&snapshot.version)])?;
  for(ordinal,topic)in snapshot.topics.iter().enumerate(){let id=out.insert("bcf_topic",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Text(&topic.guid),Cell::Text(&topic.title),Cell::Text(&topic.description),Cell::Text(&topic.status),Cell::Text(&topic.priority),Cell::Text(&topic.creation_date),Cell::Text(&topic.creation_author)])?;write_strings(out,"bcf_topic_label",id,&topic.labels)?;
   for(ordinal,comment)in topic.comments.iter().enumerate(){out.insert("bcf_comment",&[Cell::Integer(id),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Text(&comment.guid),Cell::Text(&comment.date),Cell::Text(&comment.author),Cell::Text(&comment.text),comment.viewpoint_ref.as_deref().map_or(Cell::Null,Cell::Text)])?;}
   for(ordinal,view)in topic.viewpoints.iter().enumerate(){let view_id=out.insert("bcf_viewpoint",&[Cell::Integer(id),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Text(&view.guid)])?;
    if let Some(camera)=&view.camera{let(kind,p,d,u,parameter,table)=match camera{BcfCamera::Perspective{view_point,direction,up_vector,field_of_view}=>("perspective",view_point,direction,up_vector,*field_of_view,"bcf_perspective_camera"),BcfCamera::Orthogonal{view_point,direction,up_vector,view_to_world_scale}=>("orthogonal",view_point,direction,up_vector,*view_to_world_scale,"bcf_orthogonal_camera")};out.insert_key_float("bcf_camera",view_id,&[Cell::Text(kind),Cell::Real(p.x),Cell::Real(p.y),Cell::Real(p.z),Cell::Real(d.x),Cell::Real(d.y),Cell::Real(d.z),Cell::Real(u.x),Cell::Real(u.y),Cell::Real(u.z)],CAMERA_FLOATS)?;out.insert_key_float(table,view_id,&[Cell::Real(parameter)],PARAMETER_FLOATS)?;}
    if let Some(components)=&view.components{out.insert_key("bcf_components",view_id,&[Cell::Integer(i64::from(components.visibility.default_visibility))])?;write_strings(out,"bcf_selection",view_id,&components.selection)?;write_strings(out,"bcf_visibility_exception",view_id,&components.visibility.exceptions)?;for(ordinal,coloring)in components.coloring.iter().enumerate(){let color_id=out.insert("bcf_coloring",&[Cell::Integer(view_id),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Text(&coloring.color)])?;write_strings(out,"bcf_coloring_component",color_id,&coloring.components)?;}}
    if let Some(image)=&view.snapshot{out.insert_key("bcf_snapshot_image",view_id,&[Cell::Blob(image)])?;}
   }
  }
  for(ordinal,part)in snapshot.parts.iter().enumerate(){out.insert("bcf_raw_part",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Text(&part.name),Cell::Blob(&part.data)])?;}out.checkpoint()
 }

impl ArtifactSqliteSnapshot for BcfSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{semantic::admit_record(record,limits,native)?;Self::__dsl_from_record_controlled(record,native)},control)
 }
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  admit_native(self,SqliteSnapshotPhase::EncodeNative,control)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self, _encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> {
  admit_native(self,SqliteSnapshotPhase::EncodeNative,control)?;
  let mut bound = NativeEncodingBound::file_only(control)?;
  let scalar = 2 * std::mem::size_of::<semio_framework_dsl_record::FieldValue>() + 64;
  bound.add(32768)?;
  for text in [&self.schema, &self.version] { bound.repeated(text.len(), 24)?; }
  for topic in &self.topics {
   bound.add(8192)?;
   for text in [&topic.guid, &topic.title, &topic.description, &topic.status, &topic.priority, &topic.creation_date, &topic.creation_author] { bound.repeated(text.len(), 24)?; }
   for label in &topic.labels { bound.add(1024)?; bound.repeated(label.len(), 24)?; }
   for comment in &topic.comments {
    bound.add(2048)?;
    for text in [&comment.guid, &comment.date, &comment.author, &comment.text] { bound.repeated(text.len(), 24)?; }
    if let Some(reference) = &comment.viewpoint_ref { bound.repeated(reference.len(), 24)?; }
   }
   for view in &topic.viewpoints {
    bound.add(8192)?;
    bound.repeated(view.guid.len(), 24)?;
    if view.camera.is_some() { bound.add(2048)?; bound.repeated(10, scalar)?; }
    if let Some(components) = &view.components {
     bound.add(8192)?;
     for name in &components.selection { bound.add(1024)?; bound.repeated(name.len(), 24)?; }
     for name in &components.visibility.exceptions { bound.add(1024)?; bound.repeated(name.len(), 24)?; }
     for color in &components.coloring {
      bound.add(2048)?;
      bound.repeated(color.color.len(), 24)?;
      for name in &color.components { bound.add(1024)?; bound.repeated(name.len(), 24)?; }
     }
    }
    if let Some(image) = &view.snapshot { bound.repeated(image.len(), scalar)?; }
   }
  }
  for part in &self.parts { bound.add(2048)?; bound.repeated(part.name.len(), 24)?; bound.repeated(part.data.len(), 16)?; }
  bound.finish()
 }

 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{admit_layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  admit_layout(control.limits())?;validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let documents=entities(database,"bcf_document",3,control)?;if documents.len()!=1||!documents.contains_key(&1){return Err(invalid("BCF requires document identity one"));}let topic_entities=entities(database,"bcf_topic",10,control)?;let topics_order=groups(&topic_entities,&documents,control)?;let labels=groups(&entities(database,"bcf_topic_label",4,control)?,&topic_entities,control)?;let comments=groups(&entities(database,"bcf_comment",8,control)?,&topic_entities,control)?;let view_entities=entities(database,"bcf_viewpoint",4,control)?;let views=groups(&view_entities,&topic_entities,control)?;
  let cameras=entities(database,"bcf_camera",29,control)?;same_owner(&cameras,&view_entities,control)?;let perspectives=entities(database,"bcf_perspective_camera",4,control)?;same_owner(&perspectives,&cameras,control)?;let orthogonals=entities(database,"bcf_orthogonal_camera",4,control)?;same_owner(&orthogonals,&cameras,control)?;let component_entities=entities(database,"bcf_components",2,control)?;same_owner(&component_entities,&view_entities,control)?;let selections=groups(&entities(database,"bcf_selection",4,control)?,&component_entities,control)?;let exceptions=groups(&entities(database,"bcf_visibility_exception",4,control)?,&component_entities,control)?;let coloring_entities=entities(database,"bcf_coloring",4,control)?;let colorings=groups(&coloring_entities,&component_entities,control)?;let colored=groups(&entities(database,"bcf_coloring_component",4,control)?,&coloring_entities,control)?;let images=entities(database,"bcf_snapshot_image",2,control)?;same_owner(&images,&view_entities,control)?;let parts_order=groups(&entities(database,"bcf_raw_part",5,control)?,&documents,control)?;
  let mut topics=Vec::new();for row in topics_order.get(&1).into_iter().flatten(){checkpoint(control,topics.len(),0)?;let mut topic_comments=Vec::new();for comment in comments.get(&row.rowid).into_iter().flatten(){checkpoint(control,topic_comments.len(),0)?;topic_comments.push(BcfComment{guid:reconstruct_text(control,comment.text(3)?)?,date:reconstruct_text(control,comment.text(4)?)?,author:reconstruct_text(control,comment.text(5)?)?,text:reconstruct_text(control,comment.text(6)?)?,viewpoint_ref:comment.optional_text(7)?.map(|text|reconstruct_text(control,text)).transpose()?});}
   let mut viewpoints=Vec::new();for view in views.get(&row.rowid).into_iter().flatten(){checkpoint(control,viewpoints.len(),0)?;let camera=if let Some(camera)=cameras.get(&view.rowid){let camera=FloatRow::new(camera,CAMERA_FLOATS)?;let view_point=point(camera,2)?;let direction=point(camera,5)?;let up_vector=point(camera,8)?;Some(match camera.text(1)?{"perspective"=>{if orthogonals.contains_key(&view.rowid){return Err(invalid("BCF camera has conflicting choice parameters"));}let parameter=FloatRow::new(perspectives.get(&view.rowid).ok_or_else(||invalid("BCF perspective camera is missing its parameter"))?,PARAMETER_FLOATS)?;BcfCamera::Perspective{view_point,direction,up_vector,field_of_view:parameter.real(1)?}},"orthogonal"=>{if perspectives.contains_key(&view.rowid){return Err(invalid("BCF camera has conflicting choice parameters"));}let parameter=FloatRow::new(orthogonals.get(&view.rowid).ok_or_else(||invalid("BCF orthogonal camera is missing its parameter"))?,PARAMETER_FLOATS)?;BcfCamera::Orthogonal{view_point,direction,up_vector,view_to_world_scale:parameter.real(1)?}},_=>return Err(invalid("BCF camera kind is unknown"))})}else{None};
    let components=if let Some(component)=component_entities.get(&view.rowid){let mut coloring=Vec::new();for color in colorings.get(&view.rowid).into_iter().flatten(){checkpoint(control,coloring.len(),0)?;coloring.push(BcfColoring{color:reconstruct_text(control,color.text(3)?)?,components:read_strings(colored.get(&color.rowid),control)?});}Some(BcfComponents{selection:read_strings(selections.get(&view.rowid),control)?,visibility:BcfVisibility{default_visibility:boolean(component,1)?,exceptions:read_strings(exceptions.get(&view.rowid),control)?},coloring})}else{None};let snapshot=images.get(&view.rowid).map(|image|reconstruct_blob(control,image.blob(1)?)).transpose()?;viewpoints.push(BcfViewpoint{guid:reconstruct_text(control,view.text(3)?)?,camera,components,snapshot});
   }
   topics.push(BcfTopic{guid:reconstruct_text(control,row.text(3)?)?,title:reconstruct_text(control,row.text(4)?)?,description:reconstruct_text(control,row.text(5)?)?,status:reconstruct_text(control,row.text(6)?)?,priority:reconstruct_text(control,row.text(7)?)?,labels:read_strings(labels.get(&row.rowid),control)?,creation_date:reconstruct_text(control,row.text(8)?)?,creation_author:reconstruct_text(control,row.text(9)?)?,comments:topic_comments,viewpoints});
  }
  let mut parts=Vec::new();for part in parts_order.get(&1).into_iter().flatten(){checkpoint(control,parts.len(),0)?;parts.push(BcfRawPart{name:reconstruct_text(control,part.text(3)?)?,data:reconstruct_blob(control,part.blob(4)?)?});}let result=Self{schema:reconstruct_text(control,documents[&1].text(1)?)?,version:reconstruct_text(control,documents[&1].text(2)?)?,topics,parts};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(result)
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

