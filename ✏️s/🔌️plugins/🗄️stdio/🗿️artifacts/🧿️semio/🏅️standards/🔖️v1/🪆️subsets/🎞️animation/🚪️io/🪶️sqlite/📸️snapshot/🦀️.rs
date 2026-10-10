//! 🎞️ Timelines, channels, discriminated keyframe values and ordered weight entities.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::animation::schema::snapshot::{SemioAnimationSnapshot,AnimTimeline,AnimChannel,AnimTarget,AnimTargetProperty,AnimInterpolation,AnimKeyframe,AnimValue};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3,SemioQuaternion};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="💰️reconstruction/🦀️.rs"]
mod reconstruction;
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
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::animation::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}

 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::animation::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control)}
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
semantic::layout(control.limits())?;crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::project_rows_owned(Self::SQLITE_SCHEMA,control,|out|visit_rows(self,out))
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
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
