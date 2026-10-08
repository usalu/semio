//! 📸️ Authored Remodeling entities, literal scalar words and owned relationships.
use crate::*;
#[path="📏️cells/🦀️.rs"]mod semantic_cells;
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,RowWriter}};
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

pub const SCHEMA:&str=include_str!("🗄️.sql");
#[cfg(test)]
#[path="./🧪️tests/🧬️owned/🦀️.rs"]
mod typed_tests;

#[derive(Clone,Copy)]
struct Decimal{bytes:[u8;20],start:usize}
impl Decimal{
 fn new(mut value:u64)->Self{let mut output=Self{bytes:[0;20],start:20};loop{output.start-=1;output.bytes[output.start]=b'0'+(value%10)as u8;value/=10;if value==0{return output}}}
 fn text(&self)->Result<&str,ValueError>{std::str::from_utf8(&self.bytes[self.start..]).map_err(|e|invalid(e.to_string()))}
 fn len(&self)->usize{20-self.start}
}
#[derive(Clone,Copy)]
enum ScalarCell<'a>{Borrowed(Cell<'a>),Decimal(Decimal)}
struct ScalarRow<'a>{cells:[ScalarCell<'a>;25],len:usize}
impl<'a>ScalarRow<'a>{
 fn new()->Self{Self{cells:[ScalarCell::Borrowed(Cell::Null);25],len:0}}
 fn push(&mut self,value:ScalarCell<'a>)->Result<(),ValueError>{if self.len==self.cells.len(){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Remodeling scalar row exceeds authored maximum width"))}self.cells[self.len]=value;self.len+=1;Ok(())}
 fn extend(&mut self,values:impl IntoIterator<Item=ScalarCell<'a>>)->Result<(),ValueError>{for value in values{self.push(value)?;}Ok(())}
}
trait Scalar:Sized{
 const WIDTH:usize;
 fn append<'a>(&'a self,cells:&mut ScalarRow<'a>)->Result<(),ValueError>;
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>;
 fn read(row:&SqliteRow,index:&mut usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>;
}
fn native_scalar<T:semio_framework_dsl_record::DslField>(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T,ValueError>{n.scoped_stage(|n|{n.begin_stage(0)?;T::from_value_controlled(value,n)})}
fn semantic_add(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Remodeling semantic extent overflow"))}
fn float_bytes(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn next<'a>(row:&'a SqliteRow,index:&mut usize)->Result<&'a SqliteValue,ValueError>{let value=row.values.get(*index).ok_or_else(||invalid("Remodeling scalar is absent"))?;*index+=1;Ok(value)}
impl Scalar for String{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{c.push(ScalarCell::Borrowed(Cell::Text(self)))}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{n.step()?;match value{semio_framework_dsl_record::FieldValue::Text(value)=>Ok(value.len()),_=>Err(invalid("Remodeling native scalar requires text"))}}
 fn read(r:&SqliteRow,i:&mut usize,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{match next(r,i)?{SqliteValue::Text(v)=>n.copy_text(v),_=>Err(invalid("Remodeling requires literal text"))}}
}
macro_rules! integer_scalar{($t:ty)=>{impl Scalar for $t{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{c.push(ScalarCell::Borrowed(Cell::Integer(*self as i64)))}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{let _=native_scalar::<Self>(value,n)?;Ok(8)}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{match next(r,i)?{SqliteValue::Integer(v)=><$t>::try_from(*v).map_err(|e|invalid(e.to_string())),_=>Err(invalid("Remodeling requires a native integer"))}}
}}}
integer_scalar!(u32);integer_scalar!(i64);
impl Scalar for bool{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{c.push(ScalarCell::Borrowed(Cell::Integer(i64::from(*self))))}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{let _=native_scalar::<Self>(value,n)?;Ok(8)}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{match next(r,i)?{SqliteValue::Integer(0)=>Ok(false),SqliteValue::Integer(1)=>Ok(true),_=>Err(invalid("Remodeling requires a canonical Boolean"))}}
}
impl Scalar for u64{
 const WIDTH:usize=2;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{c.push(ScalarCell::Decimal(Decimal::new(*self)))?;c.push(ScalarCell::Borrowed(Cell::Integer(*self as i64)))}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{Ok(semantic_add(Decimal::new(native_scalar::<Self>(value,n)?).len(),8)?)}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let text=match next(r,i)?{SqliteValue::Text(v)=>v,_=>return Err(invalid("Remodeling unsigned64 decimal is absent"))};let bits=match next(r,i)?{SqliteValue::Integer(v)=>*v as u64,_=>return Err(invalid("Remodeling unsigned64 exact word is absent"))};if text!=Decimal::new(bits).text()?{return Err(invalid("Remodeling unsigned64 decimal disagrees with its word"))}Ok(bits)}
}
fn class(v:f64)->&'static str{if v.is_nan(){"nan"}else if v==f64::INFINITY{"positiveInfinity"}else if v==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"}}
fn numeric_agrees(query:&SqliteValue,value:f64)->bool{match query{SqliteValue::Null=>value.is_nan(),SqliteValue::Real(v)=>!value.is_nan()&&*v==value,SqliteValue::Integer(v)=>value.is_finite()&&value>=-9223372036854775808.0&&value<9223372036854775808.0&&value.fract()==0.0&&value as i64==*v&&*v as f64==value,_=>false}}
impl Scalar for f64{
 const WIDTH:usize=3;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{c.extend([ScalarCell::Borrowed(if self.is_nan(){Cell::Null}else{Cell::Real(*self)}),ScalarCell::Borrowed(Cell::Integer(self.to_bits()as i64)),ScalarCell::Borrowed(Cell::Text(class(*self)))])}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{Ok(float_bytes(native_scalar::<Self>(value,n)?))}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let query=next(r,i)?;let value=match next(r,i)?{SqliteValue::Integer(v)=>f64::from_bits(*v as u64),_=>return Err(invalid("Remodeling binary64 word is absent"))};if !matches!(next(r,i)?,SqliteValue::Text(v)if v==class(value))||!numeric_agrees(query,value){return Err(invalid("Remodeling binary64 query or class disagrees with its word"))}Ok(value)}
}
impl Scalar for f32{
 const WIDTH:usize=3;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{let query=f64::from(*self);c.extend([ScalarCell::Borrowed(if self.is_nan(){Cell::Null}else{Cell::Real(query)}),ScalarCell::Borrowed(Cell::Integer(i64::from(self.to_bits()))),ScalarCell::Borrowed(Cell::Text(class(query)))])}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{Ok(float_bytes(f64::from(native_scalar::<Self>(value,n)?)))}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let query=next(r,i)?;let word=match next(r,i)?{SqliteValue::Integer(v)=>u32::try_from(*v).map_err(|e|invalid(e.to_string()))?,_=>return Err(invalid("Remodeling binary32 word is absent"))};let value=f32::from_bits(word);let query_value=f64::from(value);if !matches!(next(r,i)?,SqliteValue::Text(v)if v==class(query_value))||!numeric_agrees(query,query_value){return Err(invalid("Remodeling binary32 query or class disagrees with its word"))}Ok(value)}
}
impl<T:Scalar> Scalar for Option<T>{
 const WIDTH:usize=T::WIDTH;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{match self{Some(v)=>v.append(c),None=>c.extend((0..Self::WIDTH).map(|_|ScalarCell::Borrowed(Cell::Null)))}}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{if matches!(value,semio_framework_dsl_record::FieldValue::Absent){n.step()?;Ok(0)}else{T::native(value,n)}}
 fn read(r:&SqliteRow,i:&mut usize,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let end=i.checked_add(Self::WIDTH).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Remodeling optional width overflow"))?;let fields=r.values.get(*i..end).ok_or_else(||invalid("Remodeling optional scalar is absent"))?;if fields.iter().all(|v|matches!(v,SqliteValue::Null)){*i=end;Ok(None)}else{T::read(r,i,n).map(Some)}}
}
impl<T:Scalar,const N:usize> Scalar for [T;N]{
 const WIDTH:usize=N*T::WIDTH;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{for v in self{v.append(c)?;}Ok(())}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{let semio_framework_dsl_record::FieldValue::Tuple(items)=value else{return Err(invalid("Remodeling scalar requires fixed tuple"))};if items.len()!=N{return Err(invalid("Remodeling scalar tuple arity differs"))}let mut bytes=0;for item in items{bytes=semantic_add(bytes,T::native(item,n)?)?;}Ok(bytes)}
 fn read(r:&SqliteRow,i:&mut usize,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let mut values=n.allocate_vec(N)?;for _ in 0..N{n.step()?;values.push(T::read(r,i,n)?)}values.try_into().map_err(|_|invalid("Remodeling fixed tuple arity differs"))}
}
macro_rules! enumeration{($t:ty;$($variant:ident=>$name:literal),+)=>{impl Scalar for $t{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{c.push(ScalarCell::Borrowed(Cell::Text(match self{$(Self::$variant=>$name),+})))}
 fn native(value:&semio_framework_dsl_record::FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{Ok(match native_scalar::<Self>(value,n)?{$(Self::$variant=>$name.len()),+})}
 fn read(r:&SqliteRow,i:&mut usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{match next(r,i)?{SqliteValue::Text(v)=>match v.as_str(){$($name=>Ok(Self::$variant)),+,_=>Err(invalid("Remodeling enum symbol is undeclared"))},_=>Err(invalid("Remodeling enum requires literal text"))}}
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
 const FIELDS:u16;
 fn fields<'a>(&'a self,cells:&mut ScalarRow<'a>)->Result<(),ValueError>;
 fn native(record:&semio_framework_dsl_record::RecordValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>;
 fn read(row:&SqliteRow,start:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>;
}
macro_rules! entity{($t:ty,$count:expr;$($id:literal=>$field:ident:$kind:ty),+$(;$($nested:ident),+)?)=>{impl Entity for $t{
 const WIDTH:usize=0$(+<$kind as Scalar>::WIDTH)+;
 const FIELDS:u16=$count;
 fn fields<'a>(&'a self,c:&mut ScalarRow<'a>)->Result<(),ValueError>{$(self.$field.append(c)?;)+Ok(())}
 fn native(record:&semio_framework_dsl_record::RecordValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{semantic_cells::shape(record,Self::FIELDS)?;let mut bytes=0;$(bytes=semantic_add(bytes,<$kind as Scalar>::native(record.get($id).ok_or_else(||invalid("Remodeling native scalar role is absent"))?,n)?)?;)+Ok(bytes)}
 fn read(row:&SqliteRow,start:usize,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{n.charge(std::mem::size_of::<Self>())?;let mut index=start;let result=Self{$($field:<$kind as Scalar>::read(row,&mut index,n)?,)+$($($nested:Default::default(),)+)?};if index!=row.values.len(){return Err(invalid("Remodeling entity has extra scalar columns"))}Ok(result)}
}}}
entity!(FrameRef,3;0=>index:u32,1=>timestamp_ms:f64,2=>asset_id:String);
entity!(VideoSource,7;0=>name:String,1=>container:String,2=>codec:VideoCodec,3=>duration_ms:f64,4=>frame_count:u32,5=>width:u32,6=>height:u32);
entity!(RigExtrinsic,3;0=>camera_id:String,1=>rotation_wxyz:[f32;4],2=>translation_m:[f32;3]);
entity!(GcpObservation,3;0=>stream_id:String,1=>frame_index:u32,2=>pixel:[f32;2]);
entity!(IngestParams,4;0=>frame_sample_stride:u32,1=>max_frames:u32,2=>downscale_long_edge_px:u32,3=>min_sharpness:f32);
entity!(FeatureParams,4;0=>detector:FeatureDetector,1=>target_count:u32,2=>octaves:u32,3=>edge_threshold:f32);
entity!(MatchParams,6;0=>matcher:MatcherKind,1=>ratio_test:f32,2=>cross_check:bool,3=>sequential_window:u32,4=>max_pairs_per_frame:u32,5=>loop_closure:bool);
entity!(SfmParams,6;0=>ransac_iterations:u32,1=>ransac_threshold_px:f32,2=>min_track_length:u32,3=>ba_max_iterations:u32,4=>robust_loss:RobustLossKind,5=>huber_delta_px:f32);
entity!(DenseParams,5;0=>resolution:DenseResolution,1=>window_radius_px:u32,2=>min_view_consistency:u32,3=>confidence_threshold:f32,4=>max_points:u32);
entity!(MeshParams,9;0=>tsdf_voxel_size_mm:f32,1=>tsdf_truncation_mm:f32,2=>decimate_target_triangles:u32,3=>smoothing_iterations:u32,4=>texture_enabled:bool,5=>texture_size:u32,6=>guarantee_watertight:bool,7=>hole_fill_max_boundary_verts:u32,8=>self_intersection_check:bool);
entity!(MotionParams,5;0=>enabled:bool,1=>max_tracks:u32,2=>track_window_px:u32,3=>min_track_quality:f32,4=>min_track_length_frames:u32);
entity!(GeoParams,8;0=>enabled:bool,1=>origin_lon:Option<f64>,2=>origin_lat:Option<f64>,3=>origin_alt:Option<f64>,4=>gsd_m:f32,5=>dsm_cell_m:f32,6=>dtm_filter_radius_m:f32,7=>ortho_max_px:u32);
entity!(CameraPosePreview,3;0=>camera_id:String,1=>rotation_wxyz:[f32;4],2=>translation:[f32;3]);
entity!(MotionTrackSummary,4;0=>id:String,1=>length:u32,2=>class:TrackClass,3=>mean_speed_m_s:f32);
entity!(GeoProducts,3;0=>dsm_asset_id:Option<String>,1=>dtm_asset_id:Option<String>,2=>ortho_asset_id:Option<String>);
entity!(WatertightReportSnapshot,16;0=>vertex_count:u32,1=>triangle_count:u32,2=>boundary_edge_count:u32,3=>boundary_loop_count:u32,4=>non_manifold_edge_count:u32,5=>non_manifold_vertex_count:u32,6=>connected_components:u32,7=>consistently_oriented:bool,8=>euler_characteristic:i64,9=>genus:Option<i64>,10=>signed_volume:f64,11=>self_intersection_pairs:Option<u32>,12=>closed_fallback_used:bool,13=>is_closed:bool,14=>is_two_manifold:bool,15=>is_watertight:bool);
entity!(MediaStream,8;0=>id:String,1=>name:String,2=>kind:MediaKind,3=>camera_id:Option<String>,4=>sync_offset_ms:f64,5=>fps_hint:f64;frames,source);
entity!(CameraCalibration,11;0=>id:String,1=>label:String,2=>model:String,3=>fx:f64,4=>fy:f64,5=>cx:f64,6=>cy:f64,7=>skew:f64,9=>rms_reprojection_px:Option<f32>,10=>locked:bool;distortion);
entity!(GroundControlPoint,4;0=>id:String,1=>name:String,2=>world_position:[f64;3];observations);
entity!(RemodelingDurableArtifact,5;0=>kind:String,1=>mime:Option<String>,2=>width:u32,3=>height:u32;chunks);
entity!(QcReportSnapshot,7;0=>reprojection_rms_px:f64,1=>gcp_checkpoint_rmse:Option<f64>,3=>mean_track_length:f32,4=>registered_frame_ratio:f32,5=>dense_coverage_ratio:f32;warnings,watertight);

fn emit(p:&mut RowWriter<'_,'_>,table:&str,row:&ScalarRow<'_>)->Result<i64,ValueError>{let mut cells=[Cell::Null;25];for(index,value)in row.cells[..row.len].iter().enumerate(){cells[index]=match value{ScalarCell::Borrowed(cell)=>*cell,ScalarCell::Decimal(decimal)=>Cell::Text(decimal.text()?)};}p.insert(table,&cells[..row.len])}
fn scalar_cell(c:Cell<'_>)->ScalarCell<'_>{ScalarCell::Borrowed(c)}
fn entity_row<T:Entity>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,ordinal:Option<usize>,value:&T)->Result<i64,ValueError>{let mut c=ScalarRow::new();c.push(scalar_cell(Cell::Integer(parent)))?;if let Some(index)=ordinal{c.push(scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|invalid(e.to_string()))?)))?;}value.fields(&mut c)?;emit(p,table,&c)}
fn entity_list<T:Entity>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&[T])->Result<(),ValueError>{for(index,value)in values.iter().enumerate(){entity_row(p,table,parent,Some(index),value)?;}Ok(())}
fn child_fields<'a,S>(child:&'a store::ArtifactChild<S>,c:&mut ScalarRow<'a>)->Result<(),ValueError>{child.child_id.append(c)?;child.target.artifact_id.append(c)?;child.target.dialect.artifact_kind.append(c)?;child.target.dialect.standard.append(c)?;child.target.dialect.subset.append(c)}

const MESH_FIELDS:[&str;12]=["positions","normals","colors","indices","uvs","face_ids","vertex_ids","edge_positions","edge_ids","edge_uvs","edge_is_seam","paint_texture_base64"];
const MESH_TYPES:[&str;12]=["f32","f32","f32","u32","f32","u32","u32","f32","u32","f32","u8","text"];
struct DurableShape<'a>{field:Option<&'static str>,tag:Option<u8>,kind:&'static str,count:usize,offset:usize,tail:usize,text:Option<&'a str>}
/// 🔡️ Validates original UTF8 octets with bounded cancellation before borrowing their text.
fn durable_text(bytes:&[u8],mut checkpoint:impl FnMut(usize,usize)->Result<(),ValueError>)->Result<Option<&str>,ValueError>{
 let mut i=0;let mut frontier=0;while i<bytes.len(){if i>=frontier{checkpoint(i,bytes.len())?;frontier=i.saturating_add(4096)}let first=bytes[i];i+=1;if first<128{continue}let width=match first{194..=223=>1,224..=239=>2,240..=244=>3,_=>return Ok(None)};if bytes.len()-i<width{return Ok(None)}let second=bytes[i];if !(128..=191).contains(&second)||first==224&&second<160||first==237&&second>159||first==240&&second<144||first==244&&second>143{return Ok(None)}for byte in &bytes[i+1..i+width]{if !(128..=191).contains(byte){return Ok(None)}}i+=width;}checkpoint(bytes.len(),bytes.len())?;Ok(Some(unsafe{std::str::from_utf8_unchecked(bytes)}))
}
fn durable_shape<'a>(kind:&str,bytes:&'a[u8],checkpoint:impl FnMut(usize,usize)->Result<(),ValueError>)->Result<DurableShape<'a>,ValueError>{
 if kind!="sparse"&&kind!="mesh"{return Ok(DurableShape{field:None,tag:None,kind:"raw",count:bytes.len(),offset:0,tail:0,text:None})}
 if kind=="mesh"&&bytes.is_empty(){return Ok(DurableShape{field:Some("absent"),tag:None,kind:"u8",count:0,offset:0,tail:0,text:None})}
 let tag=if kind=="mesh"{Some(bytes[0])}else{None};let offset=usize::from(tag.is_some());let (field,mut kind)=match tag{None=>("samples","f32"),Some(tag)if tag<12=>(MESH_FIELDS[usize::from(tag)],MESH_TYPES[usize::from(tag)]),_=>("unknown","u8")};let length=bytes.len()-offset;let mut text=None;if kind=="text"{text=durable_text(&bytes[offset..],checkpoint)?;if text.is_none(){kind="invalid-text"}}let word=kind=="f32"||kind=="u32";Ok(DurableShape{field:Some(field),tag,kind,count:if word{length/4}else{length},offset,tail:if word{length%4}else{0},text})
}
fn durable_word(bytes:&[u8])->Result<u32,ValueError>{Ok(u32::from_le_bytes(bytes.try_into().map_err(|_|invalid("Remodeling durable word width differs"))?))}
fn project_durable(p:&mut RowWriter<'_,'_>,parent:i64,ordinal:usize,kind:&str,bytes:&[u8])->Result<(),ValueError>{
 let shape=durable_shape(kind,bytes,|done,total|p.checkpoint_work(done,total))?;
 let id=p.insert("remodel_durable_chunk",&[Cell::Integer(parent),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),shape.field.map(Cell::Text).unwrap_or(Cell::Null),shape.tag.map(|tag|Cell::Integer(i64::from(tag))).unwrap_or(Cell::Null),Cell::Text(shape.kind),Cell::Integer(i64::try_from(shape.count).map_err(|e|invalid(e.to_string()))?),if shape.kind=="raw"{Cell::Blob(bytes)}else{Cell::Null}])?;
 match shape.kind{
  "raw"=>return Ok(()),"text"=>{p.insert("remodel_durable_text",&[Cell::Integer(id),Cell::Text(shape.text.ok_or_else(||invalid("Remodeling durable text is absent"))?)])?;},
  "f32"|"u32"=>{for(index,word)in bytes[shape.offset..shape.offset+shape.count*4].chunks_exact(4).enumerate(){let bits=durable_word(word)?;if shape.kind=="f32"{let value=f64::from(f32::from_bits(bits));p.insert("remodel_durable_float",&[Cell::Integer(id),Cell::Integer(index as i64),if value.is_finite(){Cell::Real(value)}else{Cell::Null},Cell::Integer(i64::from(bits)),Cell::Text(class(value))])?;}else{p.insert("remodel_durable_integer",&[Cell::Integer(id),Cell::Integer(index as i64),Cell::Integer(i64::from(bits))])?;}}for(index,byte)in bytes[shape.offset+shape.count*4..].iter().enumerate(){p.insert("remodel_durable_byte",&[Cell::Integer(id),Cell::Integer(index as i64),Cell::Text("trailing-word"),Cell::Integer(i64::from(*byte))])?;}},
  _=>{for(index,byte)in bytes[shape.offset..].iter().enumerate(){p.insert("remodel_durable_byte",&[Cell::Integer(id),Cell::Integer(index as i64),Cell::Text("payload"),Cell::Integer(i64::from(*byte))])?;}}
 }Ok(())
}

fn visit_rows(snapshot:&crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let root=p.insert("remodel_document",&[Cell::Text(&snapshot.schema),Cell::Text(&snapshot.id)])?;
 let calibration=p.insert("remodel_calibration",&[Cell::Integer(root)])?;
 let parameters=p.insert("remodel_parameters",&[Cell::Integer(root)])?;
 let results=p.insert("remodel_results",&[Cell::Integer(root)])?;
 for(index,(key,child))in snapshot.assets.iter().enumerate(){let mut fields=ScalarRow::new();fields.extend([scalar_cell(Cell::Integer(root)),scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|invalid(e.to_string()))?)),scalar_cell(Cell::Text(key))])?;child_fields(child,&mut fields)?;emit(p,"remodel_asset",&fields)?;}
 for(index,(key,value))in snapshot.durable_artifacts.iter().enumerate(){let mut fields=ScalarRow::new();fields.extend([scalar_cell(Cell::Integer(root)),scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|invalid(e.to_string()))?)),scalar_cell(Cell::Text(key))])?;value.fields(&mut fields)?;let id=emit(p,"remodel_durable_artifact",&fields)?;for(ordinal,chunk)in value.chunks.iter().enumerate(){project_durable(p,id,ordinal,&value.kind,&chunk.0)?;}}
 for(index,value)in snapshot.streams.iter().enumerate(){let id=entity_row(p,"remodel_stream",root,Some(index),value)?;entity_list(p,"remodel_frame",id,&value.frames)?;if let Some(source)=&value.source{entity_row(p,"remodel_video_source",id,None,source)?;}}
 for(index,value)in snapshot.calibration.cameras.iter().enumerate(){let id=entity_row(p,"remodel_camera",calibration,Some(index),value)?;for(ordinal,coefficient)in value.distortion.iter().enumerate(){let mut fields=ScalarRow::new();fields.extend([scalar_cell(Cell::Integer(id)),scalar_cell(Cell::Integer(ordinal as i64))])?;coefficient.append(&mut fields)?;emit(p,"remodel_camera_distortion",&fields)?;}}
 entity_list(p,"remodel_rig_extrinsic",calibration,&snapshot.calibration.rig)?;
 for(index,value)in snapshot.gcps.iter().enumerate(){let id=entity_row(p,"remodel_ground_control_point",root,Some(index),value)?;entity_list(p,"remodel_ground_control_observation",id,&value.observations)?;}
 entity_row(p,"remodel_ingest_parameters",parameters,None,&snapshot.params.ingest)?;
 entity_row(p,"remodel_feature_parameters",parameters,None,&snapshot.params.feature)?;
 entity_row(p,"remodel_match_parameters",parameters,None,&snapshot.params.matching)?;
 entity_row(p,"remodel_sfm_parameters",parameters,None,&snapshot.params.sfm)?;
 entity_row(p,"remodel_dense_parameters",parameters,None,&snapshot.params.dense)?;
 entity_row(p,"remodel_mesh_parameters",parameters,None,&snapshot.params.mesh)?;
 entity_row(p,"remodel_motion_parameters",parameters,None,&snapshot.params.motion)?;
 entity_row(p,"remodel_geo_parameters",parameters,None,&snapshot.params.geo)?;
 let mesh=&snapshot.results.mesh;
 let mut fields=ScalarRow::new();fields.extend([scalar_cell(Cell::Integer(results))])?;child_fields(&mesh.mesh,&mut fields)?;mesh.source.append(&mut fields)?;mesh.texture_asset_id.append(&mut fields)?;let mesh_id=emit(p,"remodel_mesh_result",&fields)?;
 if let Some(report)=&mesh.watertight{project_watertight(p,Some(mesh_id),None,report)?;}
 if let Some(sparse)=&snapshot.results.sparse{let id=p.insert("remodel_cloud",&[Cell::Integer(results),Cell::Text("sparse")])?;project_float_buffer(p,id,"points",&sparse.points)?;project_byte_buffer(p,id,"colors",sparse.colors.as_ref())?;}
 if let Some(dense)=&snapshot.results.dense{let id=p.insert("remodel_cloud",&[Cell::Integer(results),Cell::Text("dense")])?;project_float_buffer(p,id,"positions",&dense.positions)?;if let Some(value)=&dense.confidence{project_float_buffer(p,id,"confidence",value)?;}project_byte_buffer(p,id,"colors",dense.colors.as_ref())?;project_byte_buffer(p,id,"classification",dense.classification.as_ref())?;}
 if let Some(trajectory)=&snapshot.results.trajectory{let id=p.insert("remodel_trajectory",&[Cell::Integer(results)])?;entity_list(p,"remodel_camera_pose",id,&trajectory.poses)?;}
 entity_list(p,"remodel_motion_track",results,&snapshot.results.tracks)?;
 if let Some(geo)=&snapshot.results.geo{entity_row(p,"remodel_geo_products",results,None,geo)?;}
 if let Some(qc)=&snapshot.results.qc{let id=entity_row(p,"remodel_qc_report",results,None,qc)?;for(index,warning)in qc.warnings.iter().enumerate(){p.insert("remodel_qc_warning",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|e|invalid(e.to_string()))?),Cell::Text(warning)])?;}if let Some(report)=&qc.watertight{project_watertight(p,None,Some(id),report)?;}}
 Ok(())
}
fn project(snapshot:&crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic_cells::extent(c.limits())?;let mut p=RowWriter::new(SCHEMA,c)?;visit_rows(snapshot,&mut p)?;p.finish()}
fn admit(snapshot:&crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic_cells::extent(c.limits())?;let mut p=RowWriter::borrowed(c,SqliteSnapshotPhase::ProjectSnapshot)?;visit_rows(snapshot,&mut p)?;p.finish_borrowed()}

fn project_watertight(p:&mut RowWriter<'_,'_>,mesh:Option<i64>,qc:Option<i64>,value:&WatertightReportSnapshot)->Result<(),ValueError>{let mut fields=ScalarRow::new();mesh.append(&mut fields)?;qc.append(&mut fields)?;value.fields(&mut fields)?;emit(p,"remodel_watertight_report",&fields)?;Ok(())}
fn project_float_buffer(p:&mut RowWriter<'_,'_>,cloud:i64,slot:&str,value:&Float32Buffer)->Result<(),ValueError>{
 let mut fields=ScalarRow::new();fields.extend([scalar_cell(Cell::Integer(cloud)),scalar_cell(Cell::Text(slot))])?;
 match value{Float32Buffer::Inline{..}=>fields.extend([scalar_cell(Cell::Text("inline")),scalar_cell(Cell::Null),scalar_cell(Cell::Null),scalar_cell(Cell::Null)])?,Float32Buffer::Content{content_id,chunk_count}=>{fields.push(scalar_cell(Cell::Text("content")))?;content_id.append(&mut fields)?;chunk_count.append(&mut fields)?;}}
 let id=emit(p,"remodel_float_buffer",&fields)?;
 if let Float32Buffer::Inline{values}=value{for(index,value)in values.iter().enumerate(){let mut fields=ScalarRow::new();fields.extend([scalar_cell(Cell::Integer(id)),scalar_cell(Cell::Integer(i64::try_from(index).map_err(|e|invalid(e.to_string()))?))])?;value.append(&mut fields)?;emit(p,"remodel_float_sample",&fields)?;}}
 Ok(())
}
fn project_byte_buffer(p:&mut RowWriter<'_,'_>,cloud:i64,slot:&str,value:Option<&ByteBuffer>)->Result<(),ValueError>{if let Some(value)=value{p.insert("remodel_byte_buffer",&[Cell::Integer(cloud),Cell::Text(slot),Cell::Blob(&value.0)])?;}Ok(())}

struct Reader<'d,'n,'p>{database:&'d SqliteDatabase,native:&'n mut semio_framework_value::NativeDecodeControl<'p>,groups:BTreeMap<(&'static str,usize),BTreeMap<i64,Vec<&'d SqliteRow>>>,used:BTreeSet<(&'static str,i64)>}
impl<'d,'n,'p>Reader<'d,'n,'p>{
 fn use_row(&mut self,table:&'static str,row:&SqliteRow)->Result<(),ValueError>{self.native.charge(96)?;if !self.used.insert((table,row.rowid)){return Err(invalid("Remodeling row is multiply owned"))}Ok(())}
 fn take(&mut self,table:&'static str,parent:i64,column:usize,width:usize,ordinal:bool)->Result<Vec<&'d SqliteRow>,ValueError>{
  let key=(table,column);
  if !self.groups.contains_key(&key){let rows=&self.database.table(table)?.rows;self.native.charge(rows.len().checked_mul(256).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Remodeling ownership workspace overflow"))?)?;let mut groups:BTreeMap<i64,Vec<&SqliteRow>>=BTreeMap::new();let mut ids=BTreeSet::new();for row in rows{self.native.step()?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid||!ids.insert(row.rowid){return Err(invalid("Remodeling relational row shape differs"))}if let Some(SqliteValue::Integer(parent))=row.values.get(column){groups.entry(*parent).or_default().push(row)}else if row.values.get(column)!=Some(&SqliteValue::Null){return Err(invalid("Remodeling ownership key is invalid"))}}self.groups.insert(key,groups);}
  let rows=self.groups.get_mut(&key).and_then(|m|m.remove(&parent)).unwrap_or_default();
  if ordinal{let mut ordered=BTreeMap::new();for row in rows{self.native.step()?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|invalid(e.to_string()))?;if ordered.insert(ordinal,row).is_some(){return Err(invalid("Remodeling relationship ordinal is repeated"))}}let mut result=self.native.allocate_vec(ordered.len())?;for(index,(ordinal,row))in ordered.into_iter().enumerate(){self.native.step()?;if ordinal!=index{return Err(invalid("Remodeling relationship ordinals are not dense"))}self.use_row(table,row)?;result.push(row)}Ok(result)}else{for row in &rows{self.native.step()?;self.use_row(table,row)?;}Ok(rows)}
 }
 fn optional(&mut self,table:&'static str,parent:i64,width:usize)->Result<Option<&'d SqliteRow>,ValueError>{let rows=self.take(table,parent,1,width,false)?;if rows.len()>1{return Err(invalid("Remodeling optional owned singleton is repeated"))}Ok(rows.into_iter().next())}
 fn required(&mut self,table:&'static str,parent:i64,width:usize)->Result<&'d SqliteRow,ValueError>{self.optional(table,parent,width)?.ok_or_else(||invalid("Remodeling required owned singleton is absent"))}
 fn singleton(&mut self,table:&'static str,width:usize)->Result<&'d SqliteRow,ValueError>{let rows=&self.database.table(table)?.rows;if rows.len()!=1||rows[0].rowid<=0||rows[0].integer(0)?!=rows[0].rowid||rows[0].values.len()!=width{return Err(invalid("Remodeling document singleton shape differs"))}self.use_row(table,&rows[0])?;Ok(&rows[0])}
 fn entity<T:Entity>(&mut self,table:&'static str,parent:i64)->Result<T,ValueError>{let row=self.required(table,parent,T::WIDTH+2)?;T::read(row,2,self.native)}
 fn optional_entity<T:Entity>(&mut self,table:&'static str,parent:i64)->Result<Option<T>,ValueError>{let row=self.optional(table,parent,T::WIDTH+2)?;row.map(|row|T::read(row,2,self.native)).transpose()}
 fn list<T:Entity>(&mut self,table:&'static str,parent:i64)->Result<Vec<T>,ValueError>{let rows=self.take(table,parent,1,T::WIDTH+3,true)?;let mut values=self.native.allocate_vec(rows.len())?;for row in rows{self.native.step()?;values.push(T::read(row,3,self.native)?)}Ok(values)}
 fn finish(self)->Result<(),ValueError>{let mut total=0usize;for table in &self.database.tables{self.native.step()?;total=total.checked_add(table.rows.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Remodeling database row count overflow"))?;}if total!=self.used.len(){return Err(invalid("Remodeling contains unowned rows"))}self.native.checkpoint()}
}
fn read_durable(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,kind:&str)->Result<ByteBuffer,ValueError>{
 let field=match &row.values[3]{SqliteValue::Null=>None,SqliteValue::Text(value)=>Some(value.as_str()),_=>return Err(invalid("Remodeling durable field requires text"))};let tag=match &row.values[4]{SqliteValue::Null=>None,SqliteValue::Integer(value)=>Some(u8::try_from(*value).map_err(|e|invalid(e.to_string()))?),_=>return Err(invalid("Remodeling durable tag requires byte"))};let storage=row.text(5)?;let count=usize::try_from(row.integer(6)?).map_err(|e|invalid(e.to_string()))?;
 if storage=="raw"{if kind=="sparse"||kind=="mesh"||field.is_some()||tag.is_some()||row.blob(7)?.len()!=count{return Err(invalid("Remodeling raw durable boundary differs"))}return Ok(ByteBuffer(r.native.copy_bytes(row.blob(7)?)?))}
 if !matches!(row.values[7],SqliteValue::Null)||kind!="sparse"&&kind!="mesh"{return Err(invalid("Remodeling structured durable owner differs"))}
 let offset=usize::from(tag.is_some());let word=storage=="f32"||storage=="u32";let mut bytes;
 if storage=="text"{let text=r.required("remodel_durable_text",row.rowid,3)?.text(2)?;if text.len()!=count{return Err(invalid("Remodeling durable UTF8 count differs"))}bytes=r.native.allocate_vec(semantic_add(count,offset)?)?;if let Some(tag)=tag{bytes.push(tag)}for part in text.as_bytes().chunks(4096){r.native.step()?;bytes.extend_from_slice(part)}}else{
  let (table,width)=match storage{"f32"=>("remodel_durable_float",6),"u32"=>("remodel_durable_integer",4),"u8"|"invalid-text"=>("remodel_durable_byte",5),_=>return Err(invalid("Remodeling durable type is undeclared"))};let elements=r.take(table,row.rowid,1,width,true)?;if elements.len()!=count{return Err(invalid("Remodeling durable element count differs"))}let tail=if word{r.take("remodel_durable_byte",row.rowid,1,5,true)?}else{Vec::new()};if tail.len()>3{return Err(invalid("Remodeling partial word exceeds three octets"))}let length=count.checked_mul(if word{4}else{1}).and_then(|v|v.checked_add(offset)).and_then(|v|v.checked_add(tail.len())).ok_or_else(||invalid("Remodeling durable byte extent overflow"))?;bytes=r.native.allocate_vec(length)?;if let Some(tag)=tag{bytes.push(tag)}
  for element in elements{r.native.step()?;if storage=="f32"{let bits=u32::try_from(element.integer(4)?).map_err(|e|invalid(e.to_string()))?;let value=f64::from(f32::from_bits(bits));if element.text(5)?!=class(value)||if value.is_finite(){!numeric_agrees(&element.values[3],value)}else{!matches!(element.values[3],SqliteValue::Null)}{return Err(invalid("Remodeling durable IEEE query differs"))}bytes.extend_from_slice(&bits.to_le_bytes())}else if storage=="u32"{let value=u32::try_from(element.integer(3)?).map_err(|e|invalid(e.to_string()))?;bytes.extend_from_slice(&value.to_le_bytes())}else{if element.text(3)?!="payload"{return Err(invalid("Remodeling durable byte role differs"))}bytes.push(u8::try_from(element.integer(4)?).map_err(|e|invalid(e.to_string()))?)} }
  for element in tail{r.native.step()?;if element.text(3)?!="trailing-word"{return Err(invalid("Remodeling partial word role differs"))}bytes.push(u8::try_from(element.integer(4)?).map_err(|e|invalid(e.to_string()))?)}
 }
 let shape=durable_shape(kind,&bytes,|_,_|r.native.step())?;if shape.field!=field||shape.tag!=tag||shape.kind!=storage||shape.count!=count{return Err(invalid("Remodeling structured durable boundary differs"))}Ok(ByteBuffer(bytes))
}
fn read_child<S>(row:&SqliteRow,index:&mut usize,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<store::ArtifactChild<S>,ValueError>{let child_id=String::read(row,index,n)?;let artifact_id=String::read(row,index,n)?;let artifact_kind=String::read(row,index,n)?;let standard=String::read(row,index,n)?;let subset=String::read(row,index,n)?;Ok(store::ArtifactChild::new(child_id,semio_framework_artifact_reference::ArtifactRef{artifact_id,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind,standard,subset}}))}
fn read_float_buffer(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<Float32Buffer,ValueError>{
 let samples=r.take("remodel_float_sample",row.rowid,1,6,true)?;
 match row.text(3)?{
  "inline"=>{if row.values[4..7].iter().any(|v|!matches!(v,SqliteValue::Null)){return Err(invalid("Remodeling inline buffer contains a content reference"))}let mut values=r.native.allocate_vec(samples.len())?;for sample in samples{r.native.step()?;let mut index=3;values.push(f32::read(sample,&mut index,r.native)?)}Ok(Float32Buffer::Inline{values})},
  "content"=>{if !samples.is_empty(){return Err(invalid("Remodeling content reference contains inline samples"))}let mut index=4;let content_id=String::read(row,&mut index,r.native)?;let chunk_count=u64::read(row,&mut index,r.native)?;Ok(Float32Buffer::Content{content_id,chunk_count})},
  _=>Err(invalid("Remodeling float buffer storage is undeclared"))
 }
}
fn read_cloud(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<(Float32Buffer,Option<ByteBuffer>,Option<Float32Buffer>,Option<ByteBuffer>),ValueError>{
 let slot=row.text(2)?;let required=match slot{"sparse"=>"points","dense"=>"positions",_=>return Err(invalid("Remodeling cloud slot is undeclared"))};
 let mut points=None;let mut confidence=None;for buffer in r.take("remodel_float_buffer",row.rowid,1,7,false)?{let key=buffer.text(2)?;if key==required{if points.is_some(){return Err(invalid("Remodeling required float buffer is repeated"))}points=Some(read_float_buffer(r,buffer)?);}else if slot=="dense"&&key=="confidence"{if confidence.is_some(){return Err(invalid("Remodeling confidence buffer is repeated"))}confidence=Some(read_float_buffer(r,buffer)?);}else{return Err(invalid("Remodeling float buffer owner or slot differs"))}}
 let mut colors=None;let mut classification=None;for buffer in r.take("remodel_byte_buffer",row.rowid,1,4,false)?{let key=buffer.text(2)?;let target=if key=="colors"{&mut colors}else if slot=="dense"&&key=="classification"{&mut classification}else{return Err(invalid("Remodeling byte buffer owner or slot differs"))};if target.is_some(){return Err(invalid("Remodeling byte buffer slot is repeated"))}*target=Some(ByteBuffer(r.native.copy_bytes(buffer.blob(3)?)?));}
 Ok((points.ok_or_else(||invalid("Remodeling required float buffer is absent"))?,colors,confidence,classification))
}
fn read_watertight(r:&mut Reader<'_,'_,'_>,parent:i64,column:usize)->Result<Option<WatertightReportSnapshot>,ValueError>{let rows=r.take("remodel_watertight_report",parent,column,WatertightReportSnapshot::WIDTH+3,false)?;if rows.len()>1{return Err(invalid("Remodeling watertight report is repeated"))}let Some(row)=rows.into_iter().next()else{return Ok(None)};if row.values.get(3-column)!=Some(&SqliteValue::Null){return Err(invalid("Remodeling watertight report has two owners"))}WatertightReportSnapshot::read(row,3,r.native).map(Some)}

fn restore(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot,ValueError>{
 c.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 store::sqlite_snapshot::validate_sqlite_database_schema(database,SCHEMA,c.limits())?;
 let maximum=c.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=semio_framework_value::NativeDecodeControl::new(maximum,&mut progress);
 native.charge(std::mem::size_of::<crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot>())?;
 let mut r=Reader{database,native:&mut native,groups:BTreeMap::new(),used:BTreeSet::new()};
 let document=r.singleton("remodel_document",3)?;let root=document.rowid;let schema=r.native.copy_text(document.text(1)?)?;let id=r.native.copy_text(document.text(2)?)?;
 let calibration_id=r.required("remodel_calibration",root,2)?.rowid;
 let parameters_id=r.required("remodel_parameters",root,2)?.rowid;
 let results_id=r.required("remodel_results",root,2)?.rowid;
 let mut assets=BTreeMap::new();for row in r.take("remodel_asset",root,1,9,true)?{r.native.step()?;r.native.charge(128)?;let key=r.native.copy_text(row.text(3)?)?;let mut index=4;let child=read_child(row,&mut index,r.native)?;if assets.insert(key,child).is_some(){return Err(invalid("Remodeling asset map key is repeated"))}}
 let mut durable_artifacts=BTreeMap::new();for row in r.take("remodel_durable_artifact",root,1,8,true)?{r.native.step()?;r.native.charge(128)?;let key=r.native.copy_text(row.text(3)?)?;let mut value=RemodelingDurableArtifact::read(row,4,r.native)?;let chunks=r.take("remodel_durable_chunk",row.rowid,1,8,true)?;value.chunks=r.native.allocate_vec(chunks.len())?;for chunk in chunks{r.native.step()?;value.chunks.push(read_durable(&mut r,chunk,&value.kind)?);}if durable_artifacts.insert(key,value).is_some(){return Err(invalid("Remodeling content map key is repeated"))}}
 let stream_rows=r.take("remodel_stream",root,1,MediaStream::WIDTH+3,true)?;let mut streams=r.native.allocate_vec(stream_rows.len())?;for row in stream_rows{r.native.step()?;let mut value=MediaStream::read(row,3,r.native)?;value.frames=r.list("remodel_frame",row.rowid)?;value.source=r.optional_entity("remodel_video_source",row.rowid)?;streams.push(value);}
 let camera_rows=r.take("remodel_camera",calibration_id,1,CameraCalibration::WIDTH+3,true)?;let mut cameras=r.native.allocate_vec(camera_rows.len())?;for row in camera_rows{r.native.step()?;let mut value=CameraCalibration::read(row,3,r.native)?;let coefficients=r.take("remodel_camera_distortion",row.rowid,1,6,true)?;if coefficients.len()!=5{return Err(invalid("Remodeling camera distortion requires five literal coefficients"))}for(index,coefficient)in coefficients.iter().enumerate(){r.native.step()?;let mut column=3;value.distortion[index]=f32::read(coefficient,&mut column,r.native)?;}cameras.push(value);}
 let calibration=CalibrationState{cameras,rig:r.list("remodel_rig_extrinsic",calibration_id)?};
 let params=ReconstructionParams{
  ingest:r.entity("remodel_ingest_parameters",parameters_id)?,feature:r.entity("remodel_feature_parameters",parameters_id)?,matching:r.entity("remodel_match_parameters",parameters_id)?,sfm:r.entity("remodel_sfm_parameters",parameters_id)?,
  dense:r.entity("remodel_dense_parameters",parameters_id)?,mesh:r.entity("remodel_mesh_parameters",parameters_id)?,motion:r.entity("remodel_motion_parameters",parameters_id)?,geo:r.entity("remodel_geo_parameters",parameters_id)?
 };
 let point_rows=r.take("remodel_ground_control_point",root,1,GroundControlPoint::WIDTH+3,true)?;let mut gcps=r.native.allocate_vec(point_rows.len())?;for row in point_rows{r.native.step()?;let mut value=GroundControlPoint::read(row,3,r.native)?;value.observations=r.list("remodel_ground_control_observation",row.rowid)?;gcps.push(value);}
 let mesh_row=r.required("remodel_mesh_result",results_id,9)?;let mut index=2;let mesh=read_child(mesh_row,&mut index,r.native)?;let source=MeshSource::read(mesh_row,&mut index,r.native)?;let texture_asset_id=Option::<String>::read(mesh_row,&mut index,r.native)?;let watertight=read_watertight(&mut r,mesh_row.rowid,1)?;
 let mesh=RemodelingMesh{mesh,source,texture_asset_id,watertight};let mut sparse=None;let mut dense=None;
 for row in r.take("remodel_cloud",results_id,1,3,false)?{r.native.step()?;match row.text(2)?{"sparse"=>{if sparse.is_some(){return Err(invalid("Remodeling sparse cloud is repeated"))}let(points,colors,_,_)=read_cloud(&mut r,row)?;sparse=Some(SparseCloud{points,colors});},"dense"=>{if dense.is_some(){return Err(invalid("Remodeling dense cloud is repeated"))}let(positions,colors,confidence,classification)=read_cloud(&mut r,row)?;dense=Some(DenseCloud{positions,colors,confidence,classification});},_=>return Err(invalid("Remodeling cloud slot is undeclared"))}}
 let trajectory=if let Some(row)=r.optional("remodel_trajectory",results_id,2)?{Some(CameraTrajectory{poses:r.list("remodel_camera_pose",row.rowid)?})}else{None};
 let tracks=r.list("remodel_motion_track",results_id)?;
 let geo=r.optional_entity("remodel_geo_products",results_id)?;
 let qc=if let Some(row)=r.optional("remodel_qc_report",results_id,QcReportSnapshot::WIDTH+2)?{let mut value=QcReportSnapshot::read(row,2,r.native)?;let warnings=r.take("remodel_qc_warning",row.rowid,1,4,true)?;value.warnings=r.native.allocate_vec(warnings.len())?;for warning in warnings{r.native.step()?;value.warnings.push(r.native.copy_text(warning.text(3)?)?);}value.watertight=read_watertight(&mut r,row.rowid,2)?;Some(value)}else{None};
 let results=ReconstructionResults{sparse,dense,mesh,trajectory,tracks,geo,qc};
 let snapshot=crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot{schema,id,streams,assets,durable_artifacts,calibration,params,gcps,results};r.finish()?;Ok(snapshot)
}


/// 🪶️ The concrete owner shares one complete relational admission visitor across public and direct IO.
impl store::ArtifactSqliteSnapshot for crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot{
 const SQLITE_SCHEMA:&'static str=SCHEMA;
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{admit(self,c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{project(self,c)}
 fn from_sqlite_database(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{restore(database,c)}
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{admit(self,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let limits=c.limits();semantic_cells::extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{semantic_cells::admit_record(record,limits,native)?;Self::__dsl_from_record_controlled(record,native)},c)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
