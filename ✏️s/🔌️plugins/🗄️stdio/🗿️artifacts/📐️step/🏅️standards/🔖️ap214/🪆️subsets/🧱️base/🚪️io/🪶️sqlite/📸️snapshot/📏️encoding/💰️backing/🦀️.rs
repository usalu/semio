//! 📏️ Borrowed STEP cells and file bounds share the original paid iterator frontier.
use crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::*;
fn sum(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"STEP semantic extent overflow"))}
struct Census{rows:usize,bytes:usize}
impl Census{
 fn rows(&mut self,count:usize,bytes:usize,bound:&artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{
  let rows=self.rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STEP semantic row count overflow"))?;
  let bytes=sum(self.bytes,count.checked_mul(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"STEP semantic repeated bytes overflow"))?)?;
  bound.check_rows(rows)?;if bytes>bound.limits().max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"STEP semantic cells exceed caller byte limit"))}self.rows=rows;self.bytes=bytes;Ok(())
 }
 fn row(&mut self,bytes:usize,bound:&artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{self.rows(1,bytes,bound)}
}
fn text(value:&str,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{bound.repeated(value.len(),16)?;bound.add(96)}
fn arguments(roots:&[StepValue],bound:&mut artifact::NativeEncodingBound<'_,'_>,census:&mut Census)->Result<(),ValueError>{
 census.rows(roots.len(),32,bound)?;let mut pending=bound.allocate_frontier::<std::slice::Iter<'_,StepValue>>(1)?;pending.push(roots.iter());let mut nodes=0usize;
 while !pending.is_empty(){let Some(value)=pending.last_mut().unwrap().next()else{pending.pop();continue};nodes=nodes.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STEP bound node count overflow"))?;bound.check_rows(nodes)?;bound.add(256)?;
 let tag=match value{StepValue::Unset=>"unset",StepValue::Derived=>"derived",StepValue::Integer(_)=>"integer",StepValue::Real(_)=>"real",StepValue::String(_)=>"string",StepValue::Enum(_)=>"enum",StepValue::Reference(_)=>"reference",StepValue::Aggregate(_)=>"aggregate",StepValue::TypedValue(StepTypedValue {..})=>"typedValue"};
 let payload=match value{StepValue::Integer(_)=>8,StepValue::Real(value)=>if value.is_nan(){11}else if value.is_infinite(){32}else{22},StepValue::String(value)|StepValue::Enum(value)=>value.len(),StepValue::Reference(value)=>sum(8,UnsignedWord::new(*value).text().len())?,_=>0};census.row(sum(sum(8,tag.len())?,payload)?,bound)?;
 let children=match value{StepValue::String(value)|StepValue::Enum(value)=>{text(value,bound)?;&[][..]},StepValue::Aggregate(values)=>{census.rows(values.len(),32,bound)?;values.as_slice()},StepValue::TypedValue(StepTypedValue {type_name,value})=>{text(type_name,bound)?;census.row(sum(24,type_name.len())?,bound)?;std::slice::from_ref(value.as_ref())},_=>&[]};if !children.is_empty(){bound.push_frontier(&mut pending,children.iter())?;}bound.checkpoint()?;
 }Ok(())
}
pub(super) fn preflight(value:&StepSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 semantic::extent(control.limits())?;let mut bound=artifact::NativeEncodingBound::file_only(control)?;let mut census=Census{rows:0,bytes:0};bound.add(4096)?;text(&value.schema,&mut bound)?;census.row(sum(16,value.schema.len())?,&bound)?;let header=&value.header;let name=&header.file_name;
 let mut header_bytes=8usize;for value in [&header.file_description.implementation_level,&name.name,&name.timestamp,&name.preprocessor_version,&name.originating_system,&name.authorization]{header_bytes=sum(header_bytes,value.len())?;text(value,&mut bound)?;}census.row(header_bytes,&bound)?;
 for list in [&header.file_description.description,&name.author,&name.organization,&header.file_schema.schemas]{bound.check_rows(list.len())?;for value in list{census.row(sum(24,value.len())?,&bound)?;text(value,&mut bound)?;bound.checkpoint()?;}}
 bound.check_rows(value.entities.len())?;for entity in &value.entities{census.row(sum(sum(24,UnsignedWord::new(entity.id).text().len())?,entity.name.len())?,&bound)?;bound.add(256)?;text(&entity.name,&mut bound)?;arguments(&entity.args,&mut bound,&mut census)?;bound.check_rows(entity.complex.len())?;for part in &entity.complex{census.row(sum(24,part.name.len())?,&bound)?;bound.add(128)?;text(&part.name,&mut bound)?;arguments(&part.args,&mut bound,&mut census)?;}bound.checkpoint()?;}bound.finish()
}
