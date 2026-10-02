//! 📸️ Authored Remodeling entities, literal scalar words and owned relationships.
use crate::*;
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,Projection}};
use std::collections::{BTreeMap,BTreeSet};
pub const SCHEMA:&str=include_str!("🗄️.sql");
#[cfg(test)]
#[path="../🧪️tests/🪶️sqlite/🧬️owned/🦀️.rs"]
mod typed_tests;
enum ScalarCell<'a>{Borrowed(Cell<'a>),Decimal(String)}
trait Scalar:Sized{
 const WIDTH:usize;
 fn append<'a>(&'a self,cells:&mut Vec<ScalarCell<'a>>);
 fn read(row:&SqliteRow,index:&mut usize,native:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>;
}
fn next<'a>(row:&'a SqliteRow,index:&mut usize)->Result<&'a SqliteValue,String>{let value=row.values.get(*index).ok_or("Remodeling scalar is absent")?;*index+=1;Ok(value)}
impl Scalar for String{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){c.push(ScalarCell::Borrowed(Cell::Text(self)))}
 fn read(r:&SqliteRow,i:&mut usize,n:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{match next(r,i)?{SqliteValue::Text(v)=>n.copy_text(v),_=>Err("Remodeling requires literal text".into())}}
}
macro_rules! integer_scalar{($t:ty)=>{impl Scalar for $t{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){c.push(ScalarCell::Borrowed(Cell::Integer(*self as i64)))}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{match next(r,i)?{SqliteValue::Integer(v)=><$t>::try_from(*v).map_err(|e|e.to_string()),_=>Err("Remodeling requires a native integer".into())}}
}}}
integer_scalar!(u32);integer_scalar!(i64);
impl Scalar for bool{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){c.push(ScalarCell::Borrowed(Cell::Integer(i64::from(*self))))}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{match next(r,i)?{SqliteValue::Integer(0)=>Ok(false),SqliteValue::Integer(1)=>Ok(true),_=>Err("Remodeling requires a canonical Boolean".into())}}
}
impl Scalar for u64{
 const WIDTH:usize=2;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){c.push(ScalarCell::Decimal(self.to_string()));c.push(ScalarCell::Borrowed(Cell::Integer(*self as i64)))}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let text=match next(r,i)?{SqliteValue::Text(v)=>v,_=>return Err("Remodeling unsigned64 decimal is absent".into())};let bits=match next(r,i)?{SqliteValue::Integer(v)=>*v as u64,_=>return Err("Remodeling unsigned64 exact word is absent".into())};if text!=&bits.to_string(){return Err("Remodeling unsigned64 decimal disagrees with its word".into())}Ok(bits)}
}
fn class(v:f64)->&'static str{if v.is_nan(){"nan"}else if v==f64::INFINITY{"positiveInfinity"}else if v==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"}}
fn numeric_agrees(query:&SqliteValue,value:f64)->bool{match query{SqliteValue::Null=>value.is_nan(),SqliteValue::Real(v)=>!value.is_nan()&&*v==value,SqliteValue::Integer(v)=>value.is_finite()&&value>=-9223372036854775808.0&&value<9223372036854775808.0&&value.fract()==0.0&&value as i64==*v&&*v as f64==value,_=>false}}
impl Scalar for f64{
 const WIDTH:usize=3;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){c.extend([ScalarCell::Borrowed(if self.is_nan(){Cell::Null}else{Cell::Real(*self)}),ScalarCell::Borrowed(Cell::Integer(self.to_bits()as i64)),ScalarCell::Borrowed(Cell::Text(class(*self)))]);}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let query=next(r,i)?;let value=match next(r,i)?{SqliteValue::Integer(v)=>f64::from_bits(*v as u64),_=>return Err("Remodeling binary64 word is absent".into())};if !matches!(next(r,i)?,SqliteValue::Text(v)if v==class(value))||!numeric_agrees(query,value){return Err("Remodeling binary64 query or class disagrees with its word".into())}Ok(value)}
}
impl Scalar for f32{
 const WIDTH:usize=3;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){let query=f64::from(*self);c.extend([ScalarCell::Borrowed(if self.is_nan(){Cell::Null}else{Cell::Real(query)}),ScalarCell::Borrowed(Cell::Integer(i64::from(self.to_bits()))),ScalarCell::Borrowed(Cell::Text(class(query)))]);}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let query=next(r,i)?;let word=match next(r,i)?{SqliteValue::Integer(v)=>u32::try_from(*v).map_err(|e|e.to_string())?,_=>return Err("Remodeling binary32 word is absent".into())};let value=f32::from_bits(word);let query_value=f64::from(value);if !matches!(next(r,i)?,SqliteValue::Text(v)if v==class(query_value))||!numeric_agrees(query,query_value){return Err("Remodeling binary32 query or class disagrees with its word".into())}Ok(value)}
}
impl<T:Scalar> Scalar for Option<T>{
 const WIDTH:usize=T::WIDTH;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){match self{Some(v)=>v.append(c),None=>c.extend((0..Self::WIDTH).map(|_|ScalarCell::Borrowed(Cell::Null)))}}
 fn read(r:&SqliteRow,i:&mut usize,n:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let end=i.checked_add(Self::WIDTH).ok_or("Remodeling optional width overflow")?;let fields=r.values.get(*i..end).ok_or("Remodeling optional scalar is absent")?;if fields.iter().all(|v|matches!(v,SqliteValue::Null)){*i=end;Ok(None)}else{T::read(r,i,n).map(Some)}}
}
impl<T:Scalar,const N:usize> Scalar for [T;N]{
 const WIDTH:usize=N*T::WIDTH;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){for v in self{v.append(c)}}
 fn read(r:&SqliteRow,i:&mut usize,n:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{let mut values=n.allocate_vec(N)?;for _ in 0..N{n.step()?;values.push(T::read(r,i,n)?)}values.try_into().map_err(|_|"Remodeling fixed tuple arity differs".into())}
}
macro_rules! enumeration{($t:ty;$($variant:ident=>$name:literal),+)=>{impl Scalar for $t{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){c.push(ScalarCell::Borrowed(Cell::Text(match self{$(Self::$variant=>$name),+})))}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{match next(r,i)?{SqliteValue::Text(v)=>match v.as_str(){$($name=>Ok(Self::$variant)),+,_=>Err("Remodeling enum symbol is undeclared".into())},_=>Err("Remodeling enum requires literal text".into())}}
}}}
enumeration!(MediaKind;ImageSequence=>"image-sequence",Video=>"video");
enumeration!(VideoCodec;Avc=>"avc",Hevc=>"hevc",Vp9=>"vp9",Av1=>"av1",Mjpeg=>"mjpeg",Unknown=>"unknown");
enumeration!(FeatureDetector;Orb=>"orb",Akaze=>"akaze",Harris=>"harris");
enumeration!(MatcherKind;BruteForce=>"brute-force",KdTree=>"kd-tree");
enumeration!(RobustLossKind;L2=>"l2",Huber=>"huber",Cauchy=>"cauchy");
enumeration!(DenseResolution;Low=>"low",Medium=>"medium",High=>"high");
enumeration!(MeshSource;Placeholder=>"placeholder",Reconstructed=>"reconstructed",Imported=>"imported");
enumeration!(TrackClass;Static=>"static",Moving=>"moving");
trait Entity:Sized{
 const WIDTH:usize;
 fn fields<'a>(&'a self,cells:&mut Vec<ScalarCell<'a>>);
 fn read(row:&SqliteRow,start:usize,native:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>;
}
macro_rules! entity{($t:ty;$($field:ident:$kind:ty),+$(;$($nested:ident),+)?)=>{impl Entity for $t{
 const WIDTH:usize=0$(+<$kind as Scalar>::WIDTH)+;
 fn fields<'a>(&'a self,c:&mut Vec<ScalarCell<'a>>){$(self.$field.append(c);)+}
 fn read(row:&SqliteRow,start:usize,n:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{n.charge(std::mem::size_of::<Self>())?;let mut index=start;let result=Self{$($field:<$kind as Scalar>::read(row,&mut index,n)?,)+$($($nested:Default::default(),)+)?};if index!=row.values.len(){return Err("Remodeling entity has extra scalar columns".into())}Ok(result)}
}}}
entity!(FrameRef;index:u32,timestamp_ms:f64,asset_id:String);
entity!(VideoSource;name:String,container:String,codec:VideoCodec,duration_ms:f64,frame_count:u32,width:u32,height:u32);
entity!(RigExtrinsic;camera_id:String,rotation_wxyz:[f32;4],translation_m:[f32;3]);
entity!(GcpObservation;stream_id:String,frame_index:u32,pixel:[f32;2]);
entity!(IngestParams;frame_sample_stride:u32,max_frames:u32,downscale_long_edge_px:u32,min_sharpness:f32);
entity!(FeatureParams;detector:FeatureDetector,target_count:u32,octaves:u32,edge_threshold:f32);
entity!(MatchParams;matcher:MatcherKind,ratio_test:f32,cross_check:bool,sequential_window:u32,max_pairs_per_frame:u32,loop_closure:bool);
entity!(SfmParams;ransac_iterations:u32,ransac_threshold_px:f32,min_track_length:u32,ba_max_iterations:u32,robust_loss:RobustLossKind,huber_delta_px:f32);
entity!(DenseParams;resolution:DenseResolution,window_radius_px:u32,min_view_consistency:u32,confidence_threshold:f32,max_points:u32);
entity!(MeshParams;tsdf_voxel_size_mm:f32,tsdf_truncation_mm:f32,decimate_target_triangles:u32,smoothing_iterations:u32,texture_enabled:bool,texture_size:u32,guarantee_watertight:bool,hole_fill_max_boundary_verts:u32,self_intersection_check:bool);
entity!(MotionParams;enabled:bool,max_tracks:u32,track_window_px:u32,min_track_quality:f32,min_track_length_frames:u32);
entity!(GeoParams;enabled:bool,origin_lon:Option<f64>,origin_lat:Option<f64>,origin_alt:Option<f64>,gsd_m:f32,dsm_cell_m:f32,dtm_filter_radius_m:f32,ortho_max_px:u32);
entity!(CameraPosePreview;camera_id:String,rotation_wxyz:[f32;4],translation:[f32;3]);
entity!(MotionTrackSummary;id:String,length:u32,class:TrackClass,mean_speed_m_s:f32);
entity!(GeoProducts;dsm_asset_id:Option<String>,dtm_asset_id:Option<String>,ortho_asset_id:Option<String>);
entity!(WatertightReportSnapshot;vertex_count:u32,triangle_count:u32,boundary_edge_count:u32,boundary_loop_count:u32,non_manifold_edge_count:u32,non_manifold_vertex_count:u32,connected_components:u32,consistently_oriented:bool,euler_characteristic:i64,genus:Option<i64>,signed_volume:f64,self_intersection_pairs:Option<u32>,closed_fallback_used:bool,is_closed:bool,is_two_manifold:bool,is_watertight:bool);
entity!(MediaStream;id:String,name:String,kind:MediaKind,camera_id:Option<String>,sync_offset_ms:f64,fps_hint:f64;frames,source);
entity!(CameraCalibration;id:String,label:String,model:String,fx:f64,fy:f64,cx:f64,cy:f64,skew:f64,rms_reprojection_px:Option<f32>,locked:bool;distortion);
entity!(GroundControlPoint;id:String,name:String,world_position:[f64;3];observations);
entity!(RemodelingDurableArtifact;kind:String,mime:Option<String>,width:u32,height:u32;chunks);
entity!(QcReportSnapshot;reprojection_rms_px:f64,gcp_checkpoint_rmse:Option<f64>,mean_track_length:f32,registered_frame_ratio:f32,dense_coverage_ratio:f32;warnings,watertight);
fn emit(p:&mut Projection<'_,'_>,table:&str,cells:&[ScalarCell<'_>])->Result<i64,String>{let cells:Vec<_>=cells.iter().map(|v|match v{ScalarCell::Borrowed(c)=>*c,ScalarCell::Decimal(s)=>Cell::Text(s)}).collect();p.insert(table,&cells)}
fn scalar_cell(c:Cell<'_>)->ScalarCell<'_>{ScalarCell::Borrowed(c)}
fn entity_row<T:Entity>(p:&mut Projection<'_,'_>,table:&str,parent:i64,ordinal:Option<usize>,value:&T)->Result<i64,String>{let mut c=vec![scalar_cell(Cell::Integer(parent))];if let Some(index)=ordinal{c.push(scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?)))}value.fields(&mut c);emit(p,table,&c)}
fn entity_list<T:Entity>(p:&mut Projection<'_,'_>,table:&str,parent:i64,values:&[T])->Result<(),String>{for(index,value)in values.iter().enumerate(){entity_row(p,table,parent,Some(index),value)?;}Ok(())}
fn child_fields<'a,S>(child:&'a store::ArtifactChild<S>,c:&mut Vec<ScalarCell<'a>>){child.child_id.append(c);child.target.artifact_id.append(c);child.target.dialect.artifact_kind.append(c);child.target.dialect.standard.append(c);child.target.dialect.subset.append(c)}

fn project(snapshot:&super::RemodelingSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
 let count=forecast(snapshot,c,SqliteSnapshotPhase::ProjectSnapshot)?;
 let mut p=Projection::new(SCHEMA,c)?;
 let root=p.insert("remodel_document",&[Cell::Text(&snapshot.schema),Cell::Text(&snapshot.id)])?;
 let calibration=p.insert("remodel_calibration",&[Cell::Integer(root)])?;
 let parameters=p.insert("remodel_parameters",&[Cell::Integer(root)])?;
 let results=p.insert("remodel_results",&[Cell::Integer(root)])?;
 for(index,(key,child))in snapshot.assets.iter().enumerate(){let mut fields=vec![scalar_cell(Cell::Integer(root)),scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?)),scalar_cell(Cell::Text(key))];child_fields(child,&mut fields);emit(&mut p,"remodel_asset",&fields)?;}
 for(index,(key,value))in snapshot.durable_artifacts.iter().enumerate(){let mut fields=vec![scalar_cell(Cell::Integer(root)),scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?)),scalar_cell(Cell::Text(key))];value.fields(&mut fields);let id=emit(&mut p,"remodel_durable_artifact",&fields)?;for(ordinal,chunk)in value.chunks.iter().enumerate(){p.insert("remodel_durable_chunk",&[Cell::Integer(id),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Blob(&chunk.0)])?;}}
 for(index,value)in snapshot.streams.iter().enumerate(){let id=entity_row(&mut p,"remodel_stream",root,Some(index),value)?;entity_list(&mut p,"remodel_frame",id,&value.frames)?;if let Some(source)=&value.source{entity_row(&mut p,"remodel_video_source",id,None,source)?;}}
 for(index,value)in snapshot.calibration.cameras.iter().enumerate(){let id=entity_row(&mut p,"remodel_camera",calibration,Some(index),value)?;for(ordinal,coefficient)in value.distortion.iter().enumerate(){let mut fields=vec![scalar_cell(Cell::Integer(id)),scalar_cell(Cell::Integer(ordinal as i64))];coefficient.append(&mut fields);emit(&mut p,"remodel_camera_distortion",&fields)?;}}
 entity_list(&mut p,"remodel_rig_extrinsic",calibration,&snapshot.calibration.rig)?;
 for(index,value)in snapshot.gcps.iter().enumerate(){let id=entity_row(&mut p,"remodel_ground_control_point",root,Some(index),value)?;entity_list(&mut p,"remodel_ground_control_observation",id,&value.observations)?;}
 entity_row(&mut p,"remodel_ingest_parameters",parameters,None,&snapshot.params.ingest)?;
 entity_row(&mut p,"remodel_feature_parameters",parameters,None,&snapshot.params.feature)?;
 entity_row(&mut p,"remodel_match_parameters",parameters,None,&snapshot.params.matching)?;
 entity_row(&mut p,"remodel_sfm_parameters",parameters,None,&snapshot.params.sfm)?;
 entity_row(&mut p,"remodel_dense_parameters",parameters,None,&snapshot.params.dense)?;
 entity_row(&mut p,"remodel_mesh_parameters",parameters,None,&snapshot.params.mesh)?;
 entity_row(&mut p,"remodel_motion_parameters",parameters,None,&snapshot.params.motion)?;
 entity_row(&mut p,"remodel_geo_parameters",parameters,None,&snapshot.params.geo)?;
 let mesh=&snapshot.results.mesh;
 let mut fields=vec![scalar_cell(Cell::Integer(results))];child_fields(&mesh.mesh,&mut fields);mesh.source.append(&mut fields);mesh.texture_asset_id.append(&mut fields);let mesh_id=emit(&mut p,"remodel_mesh_result",&fields)?;
 if let Some(report)=&mesh.watertight{project_watertight(&mut p,Some(mesh_id),None,report)?;}
 if let Some(sparse)=&snapshot.results.sparse{let id=p.insert("remodel_cloud",&[Cell::Integer(results),Cell::Text("sparse")])?;project_float_buffer(&mut p,id,"points",&sparse.points)?;project_byte_buffer(&mut p,id,"colors",sparse.colors.as_ref())?;}
 if let Some(dense)=&snapshot.results.dense{let id=p.insert("remodel_cloud",&[Cell::Integer(results),Cell::Text("dense")])?;project_float_buffer(&mut p,id,"positions",&dense.positions)?;if let Some(value)=&dense.confidence{project_float_buffer(&mut p,id,"confidence",value)?;}project_byte_buffer(&mut p,id,"colors",dense.colors.as_ref())?;project_byte_buffer(&mut p,id,"classification",dense.classification.as_ref())?;}
 if let Some(trajectory)=&snapshot.results.trajectory{let id=p.insert("remodel_trajectory",&[Cell::Integer(results)])?;entity_list(&mut p,"remodel_camera_pose",id,&trajectory.poses)?;}
 entity_list(&mut p,"remodel_motion_track",results,&snapshot.results.tracks)?;
 if let Some(geo)=&snapshot.results.geo{entity_row(&mut p,"remodel_geo_products",results,None,geo)?;}
 if let Some(qc)=&snapshot.results.qc{let id=entity_row(&mut p,"remodel_qc_report",results,None,qc)?;for(index,warning)in qc.warnings.iter().enumerate(){p.insert("remodel_qc_warning",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?),Cell::Text(warning)])?;}if let Some(report)=&qc.watertight{project_watertight(&mut p,None,Some(id),report)?;}}
 p.checkpoint_total(count)?;
 p.finish()
}
fn project_watertight(p:&mut Projection<'_,'_>,mesh:Option<i64>,qc:Option<i64>,value:&WatertightReportSnapshot)->Result<(),String>{let mut fields=Vec::new();mesh.append(&mut fields);qc.append(&mut fields);value.fields(&mut fields);emit(p,"remodel_watertight_report",&fields)?;Ok(())}
fn project_float_buffer(p:&mut Projection<'_,'_>,cloud:i64,slot:&str,value:&Float32Buffer)->Result<(),String>{
 let mut fields=vec![scalar_cell(Cell::Integer(cloud)),scalar_cell(Cell::Text(slot))];
 match value{Float32Buffer::Inline{..}=>fields.extend([scalar_cell(Cell::Text("inline")),scalar_cell(Cell::Null),scalar_cell(Cell::Null),scalar_cell(Cell::Null)]),Float32Buffer::Content{content_id,chunk_count}=>{fields.push(scalar_cell(Cell::Text("content")));content_id.append(&mut fields);chunk_count.append(&mut fields);}}
 let id=emit(p,"remodel_float_buffer",&fields)?;
 if let Float32Buffer::Inline{values}=value{for(index,value)in values.iter().enumerate(){let mut fields=vec![scalar_cell(Cell::Integer(id)),scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?))];value.append(&mut fields);emit(p,"remodel_float_sample",&fields)?;}}
 Ok(())
}
fn project_byte_buffer(p:&mut Projection<'_,'_>,cloud:i64,slot:&str,value:Option<&ByteBuffer>)->Result<(),String>{if let Some(value)=value{p.insert("remodel_byte_buffer",&[Cell::Integer(cloud),Cell::Text(slot),Cell::Blob(&value.0)])?;}Ok(())}

struct Reader<'d,'n,'p>{database:&'d SqliteDatabase,native:&'n mut dsl::NativeDecodeControl<'p>,groups:BTreeMap<(&'static str,usize),BTreeMap<i64,Vec<&'d SqliteRow>>>,used:BTreeSet<(&'static str,i64)>}
impl<'d,'n,'p>Reader<'d,'n,'p>{
 fn use_row(&mut self,table:&'static str,row:&SqliteRow)->Result<(),String>{self.native.charge(96)?;if !self.used.insert((table,row.rowid)){return Err("Remodeling row is multiply owned".into())}Ok(())}
 fn take(&mut self,table:&'static str,parent:i64,column:usize,width:usize,ordinal:bool)->Result<Vec<&'d SqliteRow>,String>{
  let key=(table,column);
  if !self.groups.contains_key(&key){let rows=&self.database.table(table)?.rows;self.native.charge(rows.len().checked_mul(256).ok_or("Remodeling ownership workspace overflow")?)?;let mut groups:BTreeMap<i64,Vec<&SqliteRow>>=BTreeMap::new();let mut ids=BTreeSet::new();for row in rows{self.native.step()?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid||!ids.insert(row.rowid){return Err("Remodeling relational row shape differs".into())}if let Some(SqliteValue::Integer(parent))=row.values.get(column){groups.entry(*parent).or_default().push(row)}else if row.values.get(column)!=Some(&SqliteValue::Null){return Err("Remodeling ownership key is invalid".into())}}self.groups.insert(key,groups);}
  let rows=self.groups.get_mut(&key).and_then(|m|m.remove(&parent)).unwrap_or_default();
  if ordinal{let mut ordered=BTreeMap::new();for row in rows{self.native.step()?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|e.to_string())?;if ordered.insert(ordinal,row).is_some(){return Err("Remodeling relationship ordinal is repeated".into())}}let mut result=self.native.allocate_vec(ordered.len())?;for(index,(ordinal,row))in ordered.into_iter().enumerate(){self.native.step()?;if ordinal!=index{return Err("Remodeling relationship ordinals are not dense".into())}self.use_row(table,row)?;result.push(row)}Ok(result)}else{for row in &rows{self.native.step()?;self.use_row(table,row)?;}Ok(rows)}
 }
 fn optional(&mut self,table:&'static str,parent:i64,width:usize)->Result<Option<&'d SqliteRow>,String>{let rows=self.take(table,parent,1,width,false)?;if rows.len()>1{return Err("Remodeling optional owned singleton is repeated".into())}Ok(rows.into_iter().next())}
 fn required(&mut self,table:&'static str,parent:i64,width:usize)->Result<&'d SqliteRow,String>{self.optional(table,parent,width)?.ok_or_else(||"Remodeling required owned singleton is absent".into())}
 fn singleton(&mut self,table:&'static str,width:usize)->Result<&'d SqliteRow,String>{let rows=&self.database.table(table)?.rows;if rows.len()!=1||rows[0].rowid<=0||rows[0].integer(0)?!=rows[0].rowid||rows[0].values.len()!=width{return Err("Remodeling document singleton shape differs".into())}self.use_row(table,&rows[0])?;Ok(&rows[0])}
 fn entity<T:Entity>(&mut self,table:&'static str,parent:i64)->Result<T,String>{let row=self.required(table,parent,T::WIDTH+2)?;T::read(row,2,self.native)}
 fn optional_entity<T:Entity>(&mut self,table:&'static str,parent:i64)->Result<Option<T>,String>{let row=self.optional(table,parent,T::WIDTH+2)?;row.map(|row|T::read(row,2,self.native)).transpose()}
 fn list<T:Entity>(&mut self,table:&'static str,parent:i64)->Result<Vec<T>,String>{let rows=self.take(table,parent,1,T::WIDTH+3,true)?;let mut values=self.native.allocate_vec(rows.len())?;for row in rows{self.native.step()?;values.push(T::read(row,3,self.native)?)}Ok(values)}
 fn finish(self)->Result<(),String>{let mut total=0usize;for table in &self.database.tables{self.native.step()?;total=total.checked_add(table.rows.len()).ok_or("Remodeling database row count overflow")?;}if total!=self.used.len(){return Err("Remodeling contains unowned rows".into())}self.native.checkpoint()}
}
fn read_child<S>(row:&SqliteRow,index:&mut usize,n:&mut dsl::NativeDecodeControl<'_>)->Result<store::ArtifactChild<S>,String>{let child_id=String::read(row,index,n)?;let artifact_id=String::read(row,index,n)?;let artifact_kind=String::read(row,index,n)?;let standard=String::read(row,index,n)?;let subset=String::read(row,index,n)?;Ok(store::ArtifactChild::new(child_id,store::io_schema::ArtifactRef{artifact_id,dialect:store::io_schema::ArtifactDialect{artifact_kind,standard,subset}}))}
fn read_float_buffer(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<Float32Buffer,String>{
 let samples=r.take("remodel_float_sample",row.rowid,1,6,true)?;
 match row.text(3)?{
  "inline"=>{if row.values[4..7].iter().any(|v|!matches!(v,SqliteValue::Null)){return Err("Remodeling inline buffer contains a content reference".into())}let mut values=r.native.allocate_vec(samples.len())?;for sample in samples{r.native.step()?;let mut index=3;values.push(f32::read(sample,&mut index,r.native)?)}Ok(Float32Buffer::Inline{values})},
  "content"=>{if !samples.is_empty(){return Err("Remodeling content reference contains inline samples".into())}let mut index=4;let content_id=String::read(row,&mut index,r.native)?;let chunk_count=u64::read(row,&mut index,r.native)?;Ok(Float32Buffer::Content{content_id,chunk_count})},
  _=>Err("Remodeling float buffer storage is undeclared".into())
 }
}
fn read_cloud(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<(Float32Buffer,Option<ByteBuffer>,Option<Float32Buffer>,Option<ByteBuffer>),String>{
 let slot=row.text(2)?;let required=match slot{"sparse"=>"points","dense"=>"positions",_=>return Err("Remodeling cloud slot is undeclared".into())};
 let mut points=None;let mut confidence=None;for buffer in r.take("remodel_float_buffer",row.rowid,1,7,false)?{let key=buffer.text(2)?;if key==required{if points.is_some(){return Err("Remodeling required float buffer is repeated".into())}points=Some(read_float_buffer(r,buffer)?);}else if slot=="dense"&&key=="confidence"{if confidence.is_some(){return Err("Remodeling confidence buffer is repeated".into())}confidence=Some(read_float_buffer(r,buffer)?);}else{return Err("Remodeling float buffer owner or slot differs".into())}}
 let mut colors=None;let mut classification=None;for buffer in r.take("remodel_byte_buffer",row.rowid,1,4,false)?{let key=buffer.text(2)?;let target=if key=="colors"{&mut colors}else if slot=="dense"&&key=="classification"{&mut classification}else{return Err("Remodeling byte buffer owner or slot differs".into())};if target.is_some(){return Err("Remodeling byte buffer slot is repeated".into())}*target=Some(ByteBuffer(r.native.copy_bytes(buffer.blob(3)?)?));}
 Ok((points.ok_or("Remodeling required float buffer is absent")?,colors,confidence,classification))
}
fn read_watertight(r:&mut Reader<'_,'_,'_>,parent:i64,column:usize)->Result<Option<WatertightReportSnapshot>,String>{let rows=r.take("remodel_watertight_report",parent,column,WatertightReportSnapshot::WIDTH+3,false)?;if rows.len()>1{return Err("Remodeling watertight report is repeated".into())}let Some(row)=rows.into_iter().next()else{return Ok(None)};if row.values.get(3-column)!=Some(&SqliteValue::Null){return Err("Remodeling watertight report has two owners".into())}WatertightReportSnapshot::read(row,3,r.native).map(Some)}

fn restore(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<super::RemodelingSnapshot,String>{
 c.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 store::sqlite_snapshot::validate_sqlite_database_schema(database,SCHEMA,c.limits()).map_err(|e|e.to_string())?;
 let maximum=c.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=dsl::NativeDecodeControl::new(maximum,&mut progress);
 native.charge(std::mem::size_of::<super::RemodelingSnapshot>())?;
 let mut r=Reader{database,native:&mut native,groups:BTreeMap::new(),used:BTreeSet::new()};
 let document=r.singleton("remodel_document",3)?;let root=document.rowid;let schema=r.native.copy_text(document.text(1)?)?;let id=r.native.copy_text(document.text(2)?)?;
 let calibration_id=r.required("remodel_calibration",root,2)?.rowid;
 let parameters_id=r.required("remodel_parameters",root,2)?.rowid;
 let results_id=r.required("remodel_results",root,2)?.rowid;
 let mut assets=BTreeMap::new();for row in r.take("remodel_asset",root,1,9,true)?{r.native.step()?;r.native.charge(128)?;let key=r.native.copy_text(row.text(3)?)?;let mut index=4;let child=read_child(row,&mut index,r.native)?;if assets.insert(key,child).is_some(){return Err("Remodeling asset map key is repeated".into())}}
 let mut durable_artifacts=BTreeMap::new();for row in r.take("remodel_durable_artifact",root,1,8,true)?{r.native.step()?;r.native.charge(128)?;let key=r.native.copy_text(row.text(3)?)?;let mut value=RemodelingDurableArtifact::read(row,4,r.native)?;let chunks=r.take("remodel_durable_chunk",row.rowid,1,4,true)?;value.chunks=r.native.allocate_vec(chunks.len())?;for chunk in chunks{r.native.step()?;value.chunks.push(ByteBuffer(r.native.copy_bytes(chunk.blob(3)?)?));}if durable_artifacts.insert(key,value).is_some(){return Err("Remodeling content map key is repeated".into())}}
 let stream_rows=r.take("remodel_stream",root,1,MediaStream::WIDTH+3,true)?;let mut streams=r.native.allocate_vec(stream_rows.len())?;for row in stream_rows{r.native.step()?;let mut value=MediaStream::read(row,3,r.native)?;value.frames=r.list("remodel_frame",row.rowid)?;value.source=r.optional_entity("remodel_video_source",row.rowid)?;streams.push(value);}
 let camera_rows=r.take("remodel_camera",calibration_id,1,CameraCalibration::WIDTH+3,true)?;let mut cameras=r.native.allocate_vec(camera_rows.len())?;for row in camera_rows{r.native.step()?;let mut value=CameraCalibration::read(row,3,r.native)?;let coefficients=r.take("remodel_camera_distortion",row.rowid,1,6,true)?;if coefficients.len()!=5{return Err("Remodeling camera distortion requires five literal coefficients".into())}for(index,coefficient)in coefficients.iter().enumerate(){r.native.step()?;let mut column=3;value.distortion[index]=f32::read(coefficient,&mut column,r.native)?;}cameras.push(value);}
 let calibration=CalibrationState{cameras,rig:r.list("remodel_rig_extrinsic",calibration_id)?};
 let params=ReconstructionParams{
  ingest:r.entity("remodel_ingest_parameters",parameters_id)?,feature:r.entity("remodel_feature_parameters",parameters_id)?,matching:r.entity("remodel_match_parameters",parameters_id)?,sfm:r.entity("remodel_sfm_parameters",parameters_id)?,
  dense:r.entity("remodel_dense_parameters",parameters_id)?,mesh:r.entity("remodel_mesh_parameters",parameters_id)?,motion:r.entity("remodel_motion_parameters",parameters_id)?,geo:r.entity("remodel_geo_parameters",parameters_id)?
 };
 let point_rows=r.take("remodel_ground_control_point",root,1,GroundControlPoint::WIDTH+3,true)?;let mut gcps=r.native.allocate_vec(point_rows.len())?;for row in point_rows{r.native.step()?;let mut value=GroundControlPoint::read(row,3,r.native)?;value.observations=r.list("remodel_ground_control_observation",row.rowid)?;gcps.push(value);}
 let mesh_row=r.required("remodel_mesh_result",results_id,9)?;let mut index=2;let mesh=read_child(mesh_row,&mut index,r.native)?;let source=MeshSource::read(mesh_row,&mut index,r.native)?;let texture_asset_id=Option::<String>::read(mesh_row,&mut index,r.native)?;let watertight=read_watertight(&mut r,mesh_row.rowid,1)?;
 let mesh=RemodelingMesh{mesh,source,texture_asset_id,watertight};let mut sparse=None;let mut dense=None;
 for row in r.take("remodel_cloud",results_id,1,3,false)?{r.native.step()?;match row.text(2)?{"sparse"=>{if sparse.is_some(){return Err("Remodeling sparse cloud is repeated".into())}let(points,colors,_,_)=read_cloud(&mut r,row)?;sparse=Some(SparseCloud{points,colors});},"dense"=>{if dense.is_some(){return Err("Remodeling dense cloud is repeated".into())}let(positions,colors,confidence,classification)=read_cloud(&mut r,row)?;dense=Some(DenseCloud{positions,colors,confidence,classification});},_=>return Err("Remodeling cloud slot is undeclared".into())}}
 let trajectory=if let Some(row)=r.optional("remodel_trajectory",results_id,2)?{Some(CameraTrajectory{poses:r.list("remodel_camera_pose",row.rowid)?})}else{None};
 let tracks=r.list("remodel_motion_track",results_id)?;
 let geo=r.optional_entity("remodel_geo_products",results_id)?;
 let qc=if let Some(row)=r.optional("remodel_qc_report",results_id,QcReportSnapshot::WIDTH+2)?{let mut value=QcReportSnapshot::read(row,2,r.native)?;let warnings=r.take("remodel_qc_warning",row.rowid,1,4,true)?;value.warnings=r.native.allocate_vec(warnings.len())?;for warning in warnings{r.native.step()?;value.warnings.push(r.native.copy_text(warning.text(3)?)?);}value.watertight=read_watertight(&mut r,row.rowid,2)?;Some(value)}else{None};
 let results=ReconstructionResults{sparse,dense,mesh,trajectory,tracks,geo,qc};
 let snapshot=super::RemodelingSnapshot{schema,id,streams,assets,durable_artifacts,calibration,params,gcps,results};r.finish()?;Ok(snapshot)
}


fn add_rows(count:&mut usize,amount:usize,maximum:usize)->Result<(),String>{*count=count.checked_add(amount).filter(|n|*n<=maximum).ok_or("Remodeling row budget exceeded")?;Ok(())}
fn buffer_rows(value:&Float32Buffer)->Result<usize,String>{match value{Float32Buffer::Inline{values}=>values.len().checked_add(1).ok_or_else(||"Remodeling buffer row count overflow".into()),Float32Buffer::Content{..}=>Ok(1)}}

fn native_record(value:&dsl::FieldValue)->Result<&dsl::RecordValue,String>{match value{dsl::FieldValue::Record(record)=>Ok(record),dsl::FieldValue::Block(value)=>native_record(value),_=>Err("Remodeling native field requires an authored record".into())}}
fn native_list(value:Option<&dsl::FieldValue>)->Result<&[dsl::FieldValue],String>{match value{None|Some(dsl::FieldValue::Absent)=>Ok(&[]),Some(dsl::FieldValue::List(values))=>Ok(values),_=>Err("Remodeling native field requires an ordered list".into())}}
fn native_map(value:Option<&dsl::FieldValue>)->Result<&[(String,dsl::FieldValue)],String>{match value{None|Some(dsl::FieldValue::Absent)=>Ok(&[]),Some(dsl::FieldValue::Map(values))=>Ok(values),_=>Err("Remodeling native field requires a literal map".into())}}
fn optional_native_record(value:Option<&dsl::FieldValue>)->Result<Option<&dsl::RecordValue>,String>{match value{None|Some(dsl::FieldValue::Absent)=>Ok(None),Some(value)=>native_record(value).map(Some)}}
fn native_buffer_rows(value:&dsl::FieldValue)->Result<usize,String>{
 let dsl::FieldValue::Block(value)=value else{return Err("Remodeling native buffer requires a block".into())};let dsl::FieldValue::Statements(variants)=value.as_ref()else{return Err("Remodeling native buffer requires a tagged variant".into())};let[(keyword,record)]=variants.as_slice()else{return Err("Remodeling native buffer requires one variant".into())};match keyword.as_str(){"inline"=>native_list(record.get(0))?.len().checked_add(1).ok_or_else(||"Remodeling native sample count overflow".into()),"content"=>Ok(1),_=>Err("Remodeling native buffer variant is undeclared".into())}
}
fn admit_native(record:&dsl::RecordValue,n:&mut dsl::NativeDecodeControl<'_>,maximum:usize)->Result<(),String>{
 let mut count=13;add_rows(&mut count,0,maximum)?;add_rows(&mut count,native_map(record.get(3))?.len(),maximum)?;
 for(_,value)in native_map(record.get(4))?{n.step()?;let value=native_record(value)?;add_rows(&mut count,1,maximum)?;add_rows(&mut count,native_list(value.get(4))?.len(),maximum)?;}
 for value in native_list(record.get(2))?{n.step()?;let value=native_record(value)?;add_rows(&mut count,1,maximum)?;add_rows(&mut count,native_list(value.get(6))?.len(),maximum)?;add_rows(&mut count,usize::from(optional_native_record(value.get(7))?.is_some()),maximum)?;}
 if let Some(value)=optional_native_record(record.get(5))?{add_rows(&mut count,native_list(value.get(0))?.len().checked_mul(6).ok_or("Remodeling native camera count overflow")?,maximum)?;add_rows(&mut count,native_list(value.get(1))?.len(),maximum)?;}
 for value in native_list(record.get(7))?{n.step()?;let value=native_record(value)?;add_rows(&mut count,1,maximum)?;add_rows(&mut count,native_list(value.get(3))?.len(),maximum)?;}
 if let Some(results)=optional_native_record(record.get(8))?{
  if let Some(mesh)=optional_native_record(results.get(2))?{add_rows(&mut count,usize::from(optional_native_record(mesh.get(3))?.is_some()),maximum)?;}
  for(id,required)in[(0,0),(1,0)]{if let Some(cloud)=optional_native_record(results.get(id))?{n.step()?;add_rows(&mut count,1,maximum)?;let buffer=cloud.get(required).ok_or("Remodeling required native float buffer is absent")?;add_rows(&mut count,native_buffer_rows(buffer)?,maximum)?;add_rows(&mut count,usize::from(cloud.get(1).is_some_and(|v|!matches!(v,dsl::FieldValue::Absent))),maximum)?;if id==1{if let Some(value)=cloud.get(2).filter(|v|!matches!(v,dsl::FieldValue::Absent)){add_rows(&mut count,native_buffer_rows(value)?,maximum)?;}add_rows(&mut count,usize::from(cloud.get(3).is_some_and(|v|!matches!(v,dsl::FieldValue::Absent))),maximum)?;}}}
  if let Some(value)=optional_native_record(results.get(3))?{add_rows(&mut count,1,maximum)?;add_rows(&mut count,native_list(value.get(0))?.len(),maximum)?;}
  add_rows(&mut count,native_list(results.get(4))?.len(),maximum)?;add_rows(&mut count,usize::from(optional_native_record(results.get(5))?.is_some()),maximum)?;
  if let Some(value)=optional_native_record(results.get(6))?{add_rows(&mut count,1,maximum)?;add_rows(&mut count,native_list(value.get(6))?.len(),maximum)?;add_rows(&mut count,usize::from(optional_native_record(value.get(2))?.is_some()),maximum)?;}
 }
 n.checkpoint()
}
impl store::ArtifactSqliteSnapshot for super::RemodelingSnapshot{
 const SQLITE_SCHEMA:&'static str=SCHEMA;
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project(self,c)}
 fn from_sqlite_database(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{restore(database,c)}
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{forecast(self,c,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{c.check_rows(13)?;if SCHEMA.len()>c.limits().max_schema_bytes{return Err("Remodeling authored schema exceeds caller limit".into())}let maximum=c.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{admit_native(record,native,maximum).map_err(dsl::__rt::field_error)?;Self::__dsl_from_record_controlled(record,native)},c)}
}

fn forecast(snapshot:&super::RemodelingSnapshot,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,String>{
 c.checkpoint(phase,0,0)?;if SCHEMA.len()>c.limits().max_schema_bytes{return Err("Remodeling authored schema exceeds caller limit".into())}let maximum=c.limits().max_rows;let mut count=13;add_rows(&mut count,snapshot.assets.len(),maximum)?;add_rows(&mut count,snapshot.durable_artifacts.len(),maximum)?;
 for(index,value)in snapshot.durable_artifacts.values().enumerate(){add_rows(&mut count,value.chunks.len(),maximum)?;if index%256==0{c.checkpoint(phase,index,0)?;}}
 for(index,value)in snapshot.streams.iter().enumerate(){add_rows(&mut count,1,maximum)?;add_rows(&mut count,value.frames.len(),maximum)?;add_rows(&mut count,usize::from(value.source.is_some()),maximum)?;if index%256==0{c.checkpoint(phase,index,0)?;}}
 add_rows(&mut count,snapshot.calibration.cameras.len().checked_mul(6).ok_or("Remodeling camera row count overflow")?,maximum)?;add_rows(&mut count,snapshot.calibration.rig.len(),maximum)?;
 for(index,value)in snapshot.gcps.iter().enumerate(){add_rows(&mut count,1,maximum)?;add_rows(&mut count,value.observations.len(),maximum)?;if index%256==0{c.checkpoint(phase,index,0)?;}}
 let results=&snapshot.results;add_rows(&mut count,usize::from(results.mesh.watertight.is_some()),maximum)?;
 if let Some(value)=&results.sparse{add_rows(&mut count,1,maximum)?;add_rows(&mut count,buffer_rows(&value.points)?,maximum)?;add_rows(&mut count,usize::from(value.colors.is_some()),maximum)?;}
 if let Some(value)=&results.dense{add_rows(&mut count,1,maximum)?;add_rows(&mut count,buffer_rows(&value.positions)?,maximum)?;if let Some(confidence)=&value.confidence{add_rows(&mut count,buffer_rows(confidence)?,maximum)?;}add_rows(&mut count,usize::from(value.colors.is_some())+usize::from(value.classification.is_some()),maximum)?;}
 if let Some(value)=&results.trajectory{add_rows(&mut count,1,maximum)?;add_rows(&mut count,value.poses.len(),maximum)?;}
 add_rows(&mut count,results.tracks.len()+usize::from(results.geo.is_some()),maximum)?;
 if let Some(value)=&results.qc{add_rows(&mut count,1,maximum)?;add_rows(&mut count,value.warnings.len(),maximum)?;add_rows(&mut count,usize::from(value.watertight.is_some()),maximum)?;}
 c.check_rows(count)?;c.checkpoint(phase,count,count)?;Ok(count)
}
