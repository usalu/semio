//! 🎞️ Timelines, channels, discriminated keyframe values and ordered weight entities.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::animation::schema::snapshot::{SemioAnimationSnapshot,AnimTimeline,AnimChannel,AnimTarget,AnimTargetProperty,AnimInterpolation,AnimKeyframe,AnimValue};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3,SemioQuaternion};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🫳️ Visits every timeline, channel, keyframe and exact scalar detail through one row writer.
pub(crate)fn visit_rows(snapshot:&SemioAnimationSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("semio_animation_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,timeline)in snapshot.timelines.iter().enumerate(){
  let timeline_id=out.insert("semio_animation_timeline",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),timeline.name.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
  for(ordinal,channel)in timeline.channels.iter().enumerate(){
   let(property,custom)=match &channel.target.property{AnimTargetProperty::Translation=>("translation",Cell::Null),AnimTargetProperty::Rotation=>("rotation",Cell::Null),AnimTargetProperty::Scale=>("scale",Cell::Null),AnimTargetProperty::Weights=>("weights",Cell::Null),AnimTargetProperty::Custom{name}=>("custom",Cell::Text(name))};
   let channel_id=out.insert("semio_animation_channel",&[Cell::Integer(timeline_id),Cell::Integer(number(ordinal)?),Cell::Text(&channel.target.node),Cell::Text(property),custom,Cell::Text(match channel.interpolation{AnimInterpolation::Linear=>"linear",AnimInterpolation::Step=>"step",AnimInterpolation::CubicSpline=>"cubic_spline"})])?;
   for(ordinal,keyframe)in channel.keyframes.iter().enumerate(){
    let kind=match keyframe.value{AnimValue::Scalar{..}=>"scalar",AnimValue::Vec3{..}=>"vector3",AnimValue::Quat{..}=>"quaternion",AnimValue::Weights{..}=>"weights"};
    let id=out.insert_float("semio_animation_keyframe",&[Cell::Integer(channel_id),Cell::Integer(number(ordinal)?),Cell::Real(keyframe.t),Cell::Text(kind)],float_columns("semio_animation_keyframe"))?;
    match &keyframe.value{
     AnimValue::Scalar{value}=>out.insert_key_float("semio_animation_scalar",id,&[Cell::Real(*value)],float_columns("semio_animation_scalar"))?,
     AnimValue::Vec3{value}=>out.insert_key_float("semio_animation_vector3",id,&[Cell::Real(value.x),Cell::Real(value.y),Cell::Real(value.z)],float_columns("semio_animation_vector3"))?,
     AnimValue::Quat{value}=>out.insert_key_float("semio_animation_quaternion",id,&[Cell::Real(value.x),Cell::Real(value.y),Cell::Real(value.z),Cell::Real(value.w)],float_columns("semio_animation_quaternion"))?,
     AnimValue::Weights{values}=>for(ordinal,value)in values.iter().enumerate(){out.insert_float("semio_animation_weight",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(*value)],float_columns("semio_animation_weight"))?;}
    }
   }
  }
 }Ok(())
}
/// 🎟️ Admits complete typed cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioAnimationSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before allocating typed fields.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts actual document primitive cells before allocating typed fields.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation row identity or columns"))}else{Ok(())}}



impl ArtifactSqliteSnapshot for SemioAnimationSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::animation::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}

 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::animation::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="animation"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_animation_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioAnimationSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::RowIndex,transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let document=single_float_row(database,"semio_animation_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation document identifier"))}
 let timelines=RowIndex::new(database,"semio_animation_timeline",4,float_columns("semio_animation_timeline"),control,"invalid Semio animation timeline owner or identity")?;
 let channels=RowIndex::new(database,"semio_animation_channel",7,float_columns("semio_animation_channel"),control,"invalid Semio animation relationship owner or identity")?;
 let keyframes=RowIndex::new(database,"semio_animation_keyframe",5,float_columns("semio_animation_keyframe"),control,"invalid Semio animation relationship owner or identity")?;
 let mut weights=RowIndex::new(database,"semio_animation_weight",4,float_columns("semio_animation_weight"),control,"invalid Semio animation relationship owner or identity")?;
 let mut scalars=RowIndex::new(database,"semio_animation_scalar",2,float_columns("semio_animation_scalar"),control,"duplicate Semio animation variant detail")?;
 let mut vectors=RowIndex::new(database,"semio_animation_vector3",4,float_columns("semio_animation_vector3"),control,"duplicate Semio animation variant detail")?;
 let mut quaternions=RowIndex::new(database,"semio_animation_quaternion",5,float_columns("semio_animation_quaternion"),control,"duplicate Semio animation variant detail")?;
 let order=timelines.ordered(2,control,"Semio animation timeline ordinals must be contiguous")?;
 for(count,&index)in timelines.indices().iter().enumerate(){if timelines.row(index)?.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation timeline owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,timelines.len())?;}
 for(count,&index)in channels.indices().iter().enumerate(){if timelines.get(channels.row(index)?.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation relationship owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,channels.len())?;}
 for(count,&index)in keyframes.indices().iter().enumerate(){if channels.get(keyframes.row(index)?.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation relationship owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,keyframes.len())?;}
 for(count,&index)in weights.indices().iter().enumerate(){if keyframes.get(weights.row(index)?.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation relationship owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,weights.len())?;}
 let channel_order=channels.grouped_by(2,control,"Semio animation relationship ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let keyframe_order=keyframes.grouped_by(2,control,"Semio animation relationship ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let weight_order=weights.grouped_by(2,control,"Semio animation relationship ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),timelines:Vec::new()});snapshot.get_mut().timelines=reserve(order.len(),control)?;let mut completed=0;
 for index in order{
  let row=timelines.row(index)?;let range=channels.range_by(&channel_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let mut timeline=Owned::new(AnimTimeline{name:None,channels:Vec::new()});timeline.get_mut().channels=reserve(range.len(),control)?;timeline.get_mut().name=row.optional_text(3)?.map(|word|reconstruct_text(control,word)).transpose()?;
  for position in range{
   let row=channels.row(channel_order[position])?;let custom=row.optional_text(5)?;let property=match row.text(4)?{"translation"=>AnimTargetProperty::Translation,"rotation"=>AnimTargetProperty::Rotation,"scale"=>AnimTargetProperty::Scale,"weights"=>AnimTargetProperty::Weights,"custom"=>{if custom.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"missing custom animation property name"))}AnimTargetProperty::Custom{name:String::new()}},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio animation target property"))};
   if row.text(4)?!="custom"&&custom.is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unexpected custom animation property name"))}
   let interpolation=match row.text(6)?{"linear"=>AnimInterpolation::Linear,"step"=>AnimInterpolation::Step,"cubic_spline"=>AnimInterpolation::CubicSpline,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio animation interpolation"))};
   let range=keyframes.range_by(&keyframe_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
   let mut channel=Owned::new(AnimChannel{target:AnimTarget{node:String::new(),property},interpolation,keyframes:Vec::new()});channel.get_mut().keyframes=reserve(range.len(),control)?;channel.get_mut().target.node=reconstruct_text(control,row.text(3)?)?;
   if let AnimTargetProperty::Custom{name}=&mut channel.get_mut().target.property{*name=reconstruct_text(control,custom.unwrap())?;}
   for position in range{
    let row=keyframes.row(keyframe_order[position])?;let mut keyframe=Owned::new(AnimKeyframe{t:row.real(3)?,value:AnimValue::Scalar{value:0.0}});
    match row.text(4)?{
     "scalar"=>{let detail=scalars.take(row.rowid,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing scalar animation value"))?;keyframe.get_mut().value=AnimValue::Scalar{value:detail.real(1)?};},
     "vector3"=>{let detail=vectors.take(row.rowid,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing vector animation value"))?;keyframe.get_mut().value=AnimValue::Vec3{value:SemioPoint3{x:detail.real(1)?,y:detail.real(2)?,z:detail.real(3)?}};},
     "quaternion"=>{let detail=quaternions.take(row.rowid,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing quaternion animation value"))?;keyframe.get_mut().value=AnimValue::Quat{value:SemioQuaternion{x:detail.real(1)?,y:detail.real(2)?,z:detail.real(3)?,w:detail.real(4)?}};},
     "weights"=>{
      keyframe.get_mut().value=AnimValue::Weights{values:Vec::new()};let range=weights.range_by(&weight_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
      if let AnimValue::Weights{values}=&mut keyframe.get_mut().value{*values=reserve(range.len(),control)?;for position in range{let detail=weights.take_index(weight_order[position],control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"multiply owned Semio animation weight"))?;values.push(detail.real(3)?);completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
     },
     _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio animation keyframe value kind")),
    }
    channel.get_mut().keyframes.push(keyframe.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
   }
   timeline.get_mut().channels.push(channel.take());control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().timelines.push(timeline.take());control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 if scalars.remaining()!=0||vectors.remaining()!=0||quaternions.remaining()!=0||weights.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"orphan or contradictory Semio animation variant detail"))}
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_animation_keyframe"=>&[FloatColumn::Binary64(3)],"semio_animation_scalar"=>&[FloatColumn::Binary64(1)],"semio_animation_vector3"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3)],"semio_animation_quaternion"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_animation_weight"=>&[FloatColumn::Binary64(3)],_=>&[]}}


fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioAnimationSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.timelines.len())?;for timeline in &self.timelines{b.optional_text(timeline.name.as_deref())?;b.entities(timeline.channels.len())?;for channel in &timeline.channels{b.text(&channel.target.node)?;b.scalars(2)?;if let AnimTargetProperty::Custom{name}=&channel.target.property{b.text(name)?;}b.entities(channel.keyframes.len())?;for key in &channel.keyframes{b.scalars(1)?;match &key.value{AnimValue::Scalar{..}=>b.scalars(1)?,AnimValue::Vec3{..}=>b.scalars(3)?,AnimValue::Quat{..}=>b.scalars(4)?,AnimValue::Weights{values}=>{b.entities(values.len())?;b.scalars(values.len())?;}}}}}Ok(())}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
