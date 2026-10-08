
//! 📏️ Every Remodeling native role is counted before constructing its typed snapshot.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::{NativeDecodeControl as N,ValueRefusalKind};
use store::sqlite_snapshot::SqliteDatabaseLimits;
type Result<T>=std::result::Result<T,ValueError>;
const WIDTHS:&[(&str,usize)]=&[("remodel_asset",9),("remodel_byte_buffer",4),("remodel_calibration",2),("remodel_camera",25),("remodel_camera_distortion",6),("remodel_camera_pose",25),("remodel_cloud",3),("remodel_dense_parameters",9),("remodel_document",3),("remodel_durable_artifact",8),("remodel_durable_byte",5),("remodel_durable_chunk",8),("remodel_durable_float",6),("remodel_durable_integer",4),("remodel_durable_text",3),("remodel_feature_parameters",8),("remodel_float_buffer",7),("remodel_float_sample",6),("remodel_frame",8),("remodel_geo_parameters",22),("remodel_geo_products",5),("remodel_ground_control_observation",11),("remodel_ground_control_point",14),("remodel_ingest_parameters",8),("remodel_match_parameters",10),("remodel_mesh_parameters",15),("remodel_mesh_result",9),("remodel_motion_parameters",9),("remodel_motion_track",9),("remodel_parameters",2),("remodel_qc_report",17),("remodel_qc_warning",4),("remodel_results",2),("remodel_rig_extrinsic",25),("remodel_sfm_parameters",12),("remodel_stream",13),("remodel_trajectory",2),("remodel_video_source",11),("remodel_watertight_report",21)];
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{if SCHEMA.len()>limits.max_schema_bytes||limits.max_tables<WIDTHS.len()||WIDTHS.iter().any(|(_,columns)|*columns>limits.max_columns)||limits.max_rows<13{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Remodeling authored schema extent exceeds caller limits"))}Ok(())}
pub(super)fn shape(value:&R,count:u16)->Result<()>{if value.fields.keys().copied().eq(0..count){Ok(())}else{Err(invalid("Remodeling native record field map differs"))}}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(||invalid("Remodeling required native role is absent"))}
fn record(value:&F,count:u16)->Result<&R>{let F::Record(value)=value else{return Err(invalid("Remodeling native role requires record"))};shape(value,count)?;Ok(value)}
fn block(value:&F,count:u16)->Result<&R>{let F::Block(value)=value else{return Err(invalid("Remodeling native role requires authored block"))};record(value,count)}
fn optional(value:&F,count:u16)->Result<Option<&R>>{if matches!(value,F::Absent){Ok(None)}else{block(value,count).map(Some)}}
fn items(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid("Remodeling native role requires ordered list"))};Ok(value)}
fn map(value:&F)->Result<&[(String,F)]>{let F::Map(value)=value else{return Err(invalid("Remodeling native role requires literal map"))};Ok(value)}
fn text(value:&F,n:&mut N<'_>)->Result<usize>{String::native(value,n)}
fn bytes(value:&F,n:&mut N<'_>)->Result<usize>{n.step()?;let F::Bytes64(value)=value else{return Err(invalid("Remodeling native role requires octets"))};Ok(value.len())}
fn child(value:&R,n:&mut N<'_>)->Result<usize>{shape(value,2)?;let target=record(field(value,1)?,4)?;let mut bytes=text(field(value,0)?,n)?;for id in 0..4{bytes=semantic_add(bytes,text(field(target,id)?,n)?)?;}Ok(bytes)}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,table:&str,bytes:usize,n:&mut N<'_>)->Result<()>{n.step()?;if !WIDTHS.iter().any(|(name,_)|*name==table){return Err(invalid("Remodeling semantic table is not authored"))}let rows=semantic_add(self.rows,1)?;let bytes=semantic_add(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Remodeling semantic row limit exceeded"))}if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Remodeling semantic value limit exceeded"))}self.rows=rows;self.bytes=bytes;Ok(())}
 fn entity<T:Entity>(&mut self,source:&R,table:&str,base:usize,n:&mut N<'_>)->Result<()>{let bytes=T::native(source,n)?;self.row(table,semantic_add(base,bytes)?,n)}
 fn list<T:Entity>(&mut self,source:&F,table:&str,n:&mut N<'_>)->Result<()>{for value in items(source)?{self.entity::<T>(record(value,T::FIELDS)?,table,24,n)?;}Ok(())}
}
fn byte_buffer(value:&F,slot:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{if matches!(value,F::Absent){n.step()?;return Ok(())}let cost=semantic_add(semantic_add(16,slot.len())?,bytes(value,n)?)?;c.row("remodel_byte_buffer",cost,n)}
fn float_buffer(value:&F,slot:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{
 let F::Block(value)=value else{return Err(invalid("Remodeling float buffer requires block"))};let F::Statements(value)=value.as_ref()else{return Err(invalid("Remodeling float buffer requires one declared variant"))};let[(keyword,source)]=value.as_slice()else{return Err(invalid("Remodeling float buffer variant count differs"))};let base=semantic_add(16,slot.len())?;
 match keyword.as_str(){
  "inline"=>{shape(source,1)?;c.row("remodel_float_buffer",semantic_add(base,6)?,n)?;for value in items(field(source,0)?)?{let cost=f32::native(value,n)?;c.row("remodel_float_sample",semantic_add(24,cost)?,n)?;}},
  "content"=>{shape(source,2)?;let content=text(field(source,0)?,n)?;let count=u64::native(field(source,1)?,n)?;c.row("remodel_float_buffer",semantic_add(semantic_add(semantic_add(base,7)?,content)?,count)?,n)?;},
  _=>return Err(invalid("Remodeling native float buffer variant is undeclared"))
 }Ok(())
}
fn durable_chunk(kind:&str,value:&F,c:&mut Census,n:&mut N<'_>)->Result<()>{
 let F::Bytes64(bytes)=value else{return Err(invalid("Remodeling durable role requires octets"))};let shape=durable_shape(kind,bytes,|_,_|n.step())?;
 let mut cost=semantic_add(32,shape.kind.len())?;if let Some(field)=shape.field{cost=semantic_add(cost,field.len())?}if shape.tag.is_some(){cost=semantic_add(cost,8)?}if shape.kind=="raw"{cost=semantic_add(cost,bytes.len())?}c.row("remodel_durable_chunk",cost,n)?;
 match shape.kind{
  "raw"=>{},"text"=>c.row("remodel_durable_text",semantic_add(16,shape.count)?,n)?,
  "f32"|"u32"=>{for word in bytes[shape.offset..shape.offset+shape.count*4].chunks_exact(4){let bits=durable_word(word)?;if shape.kind=="f32"{let value=f64::from(f32::from_bits(bits));c.row("remodel_durable_float",semantic_add(if value.is_finite(){40}else{32},class(value).len())?,n)?;}else{c.row("remodel_durable_integer",32,n)?;}}for _ in 0..shape.tail{c.row("remodel_durable_byte",45,n)?;}},
  _=>{for _ in 0..shape.count{c.row("remodel_durable_byte",39,n)?;}}
 }Ok(())
}
pub(super)fn admit_record(source:&R,limits:SqliteDatabaseLimits,n:&mut N<'_>)->Result<()>{
 extent(limits)?;n.scoped_stage(|n|{
  n.begin_stage(0)?;shape(source,9)?;let mut c=Census{limits,rows:0,bytes:0};
  let schema=text(field(source,0)?,n)?;let id=text(field(source,1)?,n)?;c.row("remodel_document",semantic_add(semantic_add(8,schema)?,id)?,n)?;
  for table in["remodel_calibration","remodel_parameters","remodel_results"]{c.row(table,16,n)?;}
  for(key,value)in map(field(source,3)?)?{n.step()?;let cost=child(record(value,2)?,n)?;c.row("remodel_asset",semantic_add(semantic_add(24,key.len())?,cost)?,n)?;}
  for(key,value)in map(field(source,4)?)?{n.step()?;let value=record(value,RemodelingDurableArtifact::FIELDS)?;let cost=RemodelingDurableArtifact::native(value,n)?;c.row("remodel_durable_artifact",semantic_add(semantic_add(24,key.len())?,cost)?,n)?;let F::Text(kind)=field(value,0)?else{return Err(invalid("Remodeling durable kind requires text"))};for chunk in items(field(value,4)?)?{durable_chunk(kind,chunk,&mut c,n)?;}}
  for value in items(field(source,2)?)?{let value=record(value,MediaStream::FIELDS)?;c.entity::<MediaStream>(value,"remodel_stream",24,n)?;c.list::<FrameRef>(field(value,6)?,"remodel_frame",n)?;if let Some(value)=optional(field(value,7)?,VideoSource::FIELDS)?{c.entity::<VideoSource>(value,"remodel_video_source",16,n)?;}}
  let calibration=block(field(source,5)?,2)?;
  for value in items(field(calibration,0)?)?{let value=record(value,CameraCalibration::FIELDS)?;c.entity::<CameraCalibration>(value,"remodel_camera",24,n)?;let F::Tuple(coefficients)=field(value,8)?else{return Err(invalid("Remodeling camera distortion requires tuple"))};if coefficients.len()!=5{return Err(invalid("Remodeling camera distortion requires five coefficients"))}for coefficient in coefficients{let cost=f32::native(coefficient,n)?;c.row("remodel_camera_distortion",semantic_add(24,cost)?,n)?;}}
  c.list::<RigExtrinsic>(field(calibration,1)?,"remodel_rig_extrinsic",n)?;
  for value in items(field(source,7)?)?{let value=record(value,GroundControlPoint::FIELDS)?;c.entity::<GroundControlPoint>(value,"remodel_ground_control_point",24,n)?;c.list::<GcpObservation>(field(value,3)?,"remodel_ground_control_observation",n)?;}
  let parameters=block(field(source,6)?,8)?;
  c.entity::<IngestParams>(block(field(parameters,0)?,IngestParams::FIELDS)?,"remodel_ingest_parameters",16,n)?;
  c.entity::<FeatureParams>(block(field(parameters,1)?,FeatureParams::FIELDS)?,"remodel_feature_parameters",16,n)?;
  c.entity::<MatchParams>(block(field(parameters,2)?,MatchParams::FIELDS)?,"remodel_match_parameters",16,n)?;
  c.entity::<SfmParams>(block(field(parameters,3)?,SfmParams::FIELDS)?,"remodel_sfm_parameters",16,n)?;
  c.entity::<DenseParams>(block(field(parameters,4)?,DenseParams::FIELDS)?,"remodel_dense_parameters",16,n)?;
  c.entity::<MeshParams>(block(field(parameters,5)?,MeshParams::FIELDS)?,"remodel_mesh_parameters",16,n)?;
  c.entity::<MotionParams>(block(field(parameters,6)?,MotionParams::FIELDS)?,"remodel_motion_parameters",16,n)?;
  c.entity::<GeoParams>(block(field(parameters,7)?,GeoParams::FIELDS)?,"remodel_geo_parameters",16,n)?;
  let results=block(field(source,8)?,7)?;let mesh=block(field(results,2)?,4)?;let handle=child(block(field(mesh,0)?,2)?,n)?;let origin=MeshSource::native(field(mesh,1)?,n)?;let texture=<Option<String>as Scalar>::native(field(mesh,2)?,n)?;c.row("remodel_mesh_result",semantic_add(semantic_add(semantic_add(16,handle)?,origin)?,texture)?,n)?;
  if let Some(value)=optional(field(mesh,3)?,WatertightReportSnapshot::FIELDS)?{c.entity::<WatertightReportSnapshot>(value,"remodel_watertight_report",16,n)?;}
  if let Some(value)=optional(field(results,0)?,2)?{c.row("remodel_cloud",22,n)?;float_buffer(field(value,0)?,"points",&mut c,n)?;byte_buffer(field(value,1)?,"colors",&mut c,n)?;}
  if let Some(value)=optional(field(results,1)?,4)?{c.row("remodel_cloud",21,n)?;float_buffer(field(value,0)?,"positions",&mut c,n)?;byte_buffer(field(value,1)?,"colors",&mut c,n)?;if !matches!(field(value,2)?,F::Absent){float_buffer(field(value,2)?,"confidence",&mut c,n)?;}byte_buffer(field(value,3)?,"classification",&mut c,n)?;}
  if let Some(value)=optional(field(results,3)?,1)?{c.row("remodel_trajectory",16,n)?;c.list::<CameraPosePreview>(field(value,0)?,"remodel_camera_pose",n)?;}
  c.list::<MotionTrackSummary>(field(results,4)?,"remodel_motion_track",n)?;
  if let Some(value)=optional(field(results,5)?,GeoProducts::FIELDS)?{c.entity::<GeoProducts>(value,"remodel_geo_products",16,n)?;}
  if let Some(value)=optional(field(results,6)?,QcReportSnapshot::FIELDS)?{c.entity::<QcReportSnapshot>(value,"remodel_qc_report",16,n)?;for warning in items(field(value,6)?)?{let cost=text(warning,n)?;c.row("remodel_qc_warning",semantic_add(24,cost)?,n)?;}if let Some(value)=optional(field(value,2)?,WatertightReportSnapshot::FIELDS)?{c.entity::<WatertightReportSnapshot>(value,"remodel_watertight_report",16,n)?;}}
  n.checkpoint()
 })
}
