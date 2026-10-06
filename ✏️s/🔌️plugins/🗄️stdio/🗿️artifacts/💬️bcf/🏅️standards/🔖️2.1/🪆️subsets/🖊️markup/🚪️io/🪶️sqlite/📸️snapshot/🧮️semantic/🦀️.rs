//! 🫳️ Complete declared BCF records admit authored SQL cells before typed construction.
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};
use crate::store::sqlite_snapshot::SqliteDatabaseLimits;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"BCF native field differs from its declared owned role")}
fn sum(left:usize,right:usize)->Result<usize>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"BCF semantic value extent overflow"))}
fn exact(value:&R,count:u16)->Result<()>{if value.fields.len()!=usize::from(count)||value.fields.keys().any(|id|*id>=count){Err(invalid())}else{Ok(())}}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(invalid)}
fn record(value:&F,count:u16)->Result<&R>{let F::Record(value)=value else{return Err(invalid())};exact(value,count)?;Ok(value)}
fn list(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid())};Ok(value)}
fn text(value:&F,native:&mut NativeDecodeControl<'_>)->Result<usize>{let F::Text(value)=value else{return Err(invalid())};native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value.len())}
fn float(value:&F)->Result<usize>{let F::Float(value)=value else{return Err(invalid())};Ok(if value.is_nan(){11}else if value.is_infinite(){32}else{22})}
fn octets(value:&F,native:&mut NativeDecodeControl<'_>)->Result<usize>{let bytes=list(value)?;for byte in bytes{if !matches!(byte,F::UInt(value)if u8::try_from(*value).is_ok()){return Err(invalid())}native.step()?;}Ok(bytes.len())}
fn binary(value:&F,native:&mut NativeDecodeControl<'_>)->Result<usize>{let F::Bytes64(value)=value else{return Err(invalid())};native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value.len())}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"BCF semantic rows overflow"))?;let bytes=sum(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"BCF semantic rows exceed caller limit"))}if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"BCF semantic values exceed caller limit"))}native.step()?;self.rows=rows;self.bytes=bytes;Ok(())}
 fn strings(&mut self,value:&F,native:&mut NativeDecodeControl<'_>)->Result<()>{for value in list(value)?{self.row(sum(24,text(value,native)?)?,native)?;}Ok(())}
}
/// 📷️ Reads the actual block-wrapped single camera choice and all ten raw IEEE roles.
fn camera(value:&F,census:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 let F::Block(value)=value else{return Err(invalid())};let F::Statements(values)=value.as_ref()else{return Err(invalid())};let[(keyword,value)]=values.as_slice()else{return Err(invalid())};if !matches!(keyword.as_str(),"perspective"|"orthogonal"){return Err(invalid())}exact(value,4)?;
 let mut bytes=sum(8,keyword.len())?;for id in 0..3{let point=record(field(value,id)?,3)?;for id in 0..3{bytes=sum(bytes,float(field(point,id)?)?)?;native.step()?;}}census.row(bytes,native)?;census.row(sum(8,float(field(value,3)?)?)?,native)
}
/// 🧩️ Counts every present components body and ordered literal child.
fn components(value:&F,census:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 let value=record(value,3)?;let visibility=record(field(value,1)?,2)?;if !matches!(field(visibility,0)?,F::Bool(_)){return Err(invalid())}census.row(16,native)?;census.strings(field(value,0)?,native)?;census.strings(field(visibility,1)?,native)?;
 for value in list(field(value,2)?)?{let value=record(value,2)?;census.row(sum(24,text(field(value,0)?,native)?)?,native)?;census.strings(field(value,1)?,native)?;}Ok(())
}
/// 🗃️ Visits complete original records with no typed Snapshot, cloned graph or SQL database.
pub(super)fn admit_record(value:&R,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<()>{
 super::admit_layout(limits)?;native.scoped_stage(|native|{native.begin_stage(0)?;exact(value,4)?;let mut census=Census{limits,rows:0,bytes:0};let document=sum(sum(8,text(field(value,0)?,native)?)?,text(field(value,1)?,native)?)?;census.row(document,native)?;
  for value in list(field(value,2)?)?{let topic=record(value,10)?;let mut bytes=24usize;for id in[0,1,2,3,4,6,7]{bytes=sum(bytes,text(field(topic,id)?,native)?)?;}census.row(bytes,native)?;census.strings(field(topic,5)?,native)?;
   for value in list(field(topic,8)?)?{let value=record(value,5)?;let mut bytes=24usize;for id in 0..4{bytes=sum(bytes,text(field(value,id)?,native)?)?;}if !matches!(field(value,4)?,F::Absent){bytes=sum(bytes,text(field(value,4)?,native)?)?;}census.row(bytes,native)?;}
   for value in list(field(topic,9)?)?{let value=record(value,4)?;census.row(sum(24,text(field(value,0)?,native)?)?,native)?;if !matches!(field(value,1)?,F::Absent){camera(field(value,1)?,&mut census,native)?;}if !matches!(field(value,2)?,F::Absent){components(field(value,2)?,&mut census,native)?;}if !matches!(field(value,3)?,F::Absent){census.row(sum(8,octets(field(value,3)?,native)?)?,native)?;}}
  }
  for value in list(field(value,3)?)?{let value=record(value,2)?;let bytes=sum(sum(24,text(field(value,0)?,native)?)?,binary(field(value,1)?,native)?)?;census.row(bytes,native)?;}native.checkpoint()
 })
}
