//! 📏️ IFC4 cells and file bounds share the original iterator with one paid identity index.
use super::*;
use semio_framework_os_kernel::sqlite_snapshot::artifact;
fn sum(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 semantic extent overflow"))}
struct Census{rows:usize,bytes:usize}
impl Census{
 fn rows(&mut self,count:usize,bytes:usize,bound:&artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{
  let rows=self.rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Part21 semantic row count overflow"))?;
  let bytes=sum(self.bytes,count.checked_mul(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 semantic repeated bytes overflow"))?)?;
  bound.check_rows(rows)?;if bytes>bound.limits().max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 semantic cells exceed caller byte limit"))}self.rows=rows;self.bytes=bytes;Ok(())
 }
 fn row(&mut self,bytes:usize,bound:&artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{self.rows(1,bytes,bound)}
}
fn identities(values:impl ExactSizeIterator<Item=u64>,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<Vec<u64>,ValueError>{
 bound.check_rows(values.len())?;let mut identities=bound.allocate_frontier(values.len())?;for value in values{identities.push(value);bound.checkpoint()?;}
 sort_paid(&mut identities,|left,right|{bound.checkpoint()?;Ok(left.cmp(right))})?;for pair in identities.windows(2){if pair[0]==pair[1]{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate IFC instance identifier"))}bound.checkpoint()?;}Ok(identities)
}
fn resolved(value:u64,identities:&[u64],bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<bool,ValueError>{
 let(mut low,mut high)=(0,identities.len());while low<high{bound.checkpoint()?;let middle=low+(high-low)/2;match identities[middle].cmp(&value){std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(true)}}Ok(false)
}
fn text(value:&str,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{bound.repeated(value.len(),16)?;bound.add(96)}
fn rows(count:&mut usize,size:usize,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{*count=count.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 native row census overflow"))?;bound.check_rows(*count)}
fn arguments(roots:&[IfcValue],bound:&mut artifact::NativeEncodingBound<'_,'_>,count:&mut usize,census:&mut Census,identities:&[u64])->Result<(),ValueError>{
 census.rows(roots.len(),32,bound)?;rows(count,roots.len(),bound)?;let mut pending=bound.allocate_frontier::<std::slice::Iter<'_,IfcValue>>(1)?;pending.push(roots.iter());
 while !pending.is_empty(){let Some(value)=pending.last_mut().unwrap().next()else{pending.pop();continue};rows(count,1,bound)?;bound.add(256)?;
 let tag=match value{IfcValue::Unset=>"unset",IfcValue::Derived=>"derived",IfcValue::Integer(_)=>"integer",IfcValue::Real(_)=>"real",IfcValue::String(_)=>"string",IfcValue::Enum(_)=>"enum",IfcValue::Reference(_)=>"reference",IfcValue::Aggregate(_)=>"aggregate",IfcValue::TypedValue(IfcTypedValue {..})=>"typedValue"};
 let payload=match value{IfcValue::Integer(_)=>8,IfcValue::Real(value)=>if value.is_nan(){11}else if value.is_infinite(){32}else{22},IfcValue::String(value)|IfcValue::Enum(value)=>value.len(),IfcValue::Reference(value)=>sum(UnsignedWord::new(*value).text().len(),if resolved(*value,identities,bound)?{8}else{0})?,IfcValue::TypedValue(IfcTypedValue {name,..})=>name.len(),_=>0};census.row(sum(sum(8,tag.len())?,payload)?,bound)?;
 let children=match value{IfcValue::String(value)|IfcValue::Enum(value)=>{text(value,bound)?;&[][..]},IfcValue::Aggregate(values)=>values.as_slice(),IfcValue::TypedValue(IfcTypedValue {name,items})=>{text(name,bound)?;items.as_slice()},_=>&[]};
 census.rows(children.len(),32,bound)?;rows(count,children.len(),bound)?;if !children.is_empty(){bound.push_frontier(&mut pending,children.iter())?;}bound.checkpoint()?;
 }Ok(())
}
pub(super) fn preflight(value:&IfcSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 semantic::extent(control.limits())?;let mut bound=artifact::NativeEncodingBound::file_only(control)?;let identities=identities(value.entities.iter().map(|entity|entity.id),&mut bound)?;let mut census=Census{rows:0,bytes:0};let mut count=0;rows(&mut count,2,&mut bound)?;bound.add(4096)?;text(&value.schema,&mut bound)?;census.row(sum(16,value.schema.len())?,&bound)?;census.row(8,&bound)?;
 for values in [&value.header.file_description,&value.header.file_name,&value.header.file_schema]{arguments(values,&mut bound,&mut count,&mut census,&identities)?;}
 rows(&mut count,value.entities.len(),&mut bound)?;for entity in &value.entities{census.row(sum(sum(24,UnsignedWord::new(entity.id).text().len())?,entity.name.len())?,&bound)?;bound.add(256)?;text(&entity.name,&mut bound)?;arguments(&entity.args,&mut bound,&mut count,&mut census,&identities)?;
 rows(&mut count,entity.complex.len(),&mut bound)?;for part in &entity.complex{census.row(sum(24,part.name.len())?,&bound)?;bound.add(128)?;text(&part.name,&mut bound)?;arguments(&part.args,&mut bound,&mut count,&mut census,&identities)?;}bound.checkpoint()?;}bound.finish()
}
