//! 📏️ IFC2x3 cells and file bounds share the original iterator with one paid identity index.
use super::*;
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
fn arguments(roots:&[Part21Value],bound:&mut artifact::NativeEncodingBound<'_,'_>,census:&mut Census,identities:&[u64])->Result<(),ValueError>{
 census.rows(roots.len(),32,bound)?;let mut pending=bound.allocate_frontier::<std::slice::Iter<'_,Part21Value>>(1)?;pending.push(roots.iter());let mut nodes=0usize;
 while !pending.is_empty(){let Some(value)=pending.last_mut().unwrap().next()else{pending.pop();continue};nodes=nodes.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 bound node count overflow"))?;bound.check_rows(nodes)?;bound.add(384)?;
 let tag=match value{Part21Value::Unset=>"unset",Part21Value::Derived=>"derived",Part21Value::Int(_)=>"int",Part21Value::Real(_)=>"real",Part21Value::Str(_)=>"str",Part21Value::Enum(_)=>"enum",Part21Value::Ref(_)=>"ref",Part21Value::List(_)=>"list",Part21Value::Typed{..}=>"typed"};
 let payload=match value{Part21Value::Int(_)=>8,Part21Value::Real(value)=>sum(sum(16,value.coefficient.len())?,if value.exponent.is_some(){8}else{0})?,Part21Value::Str(value)|Part21Value::Enum(value)=>value.len(),Part21Value::Ref(value)=>sum(UnsignedWord::new(*value).text().len(),if resolved(*value,identities,bound)?{8}else{0})?,Part21Value::Typed{name,..}=>name.len(),_=>0};census.row(sum(sum(8,tag.len())?,payload)?,bound)?;
 let children=match value{Part21Value::Str(value)|Part21Value::Enum(value)=>{text(value,bound)?;&[][..]},Part21Value::Real(decimal)=>{text(&decimal.coefficient,bound)?;&[][..]},Part21Value::List(values)=>values.as_slice(),Part21Value::Typed{name,items}=>{text(name,bound)?;items.as_slice()},_=>&[]};census.rows(children.len(),32,bound)?;if !children.is_empty(){bound.push_frontier(&mut pending,children.iter())?;}bound.checkpoint()?;
 }Ok(())
}
pub(super) fn preflight(value:&Ifc2x3Snapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 semantic::extent(control.limits())?;let mut bound=artifact::NativeEncodingBound::file_only(control)?;let identities=identities(value.document.instances.iter().map(|instance|instance.id),&mut bound)?;let mut census=Census{rows:0,bytes:0};bound.add(4096)?;text(&value.schema,&mut bound)?;census.row(sum(sum(16,value.schema.len())?,if value.edm_preamble.is_some(){8}else{0})?,&bound)?;census.row(8,&bound)?;
 if let Some(edm)=&value.edm_preamble{let mut bytes=8usize;for value in [&edm.producer,&edm.module,&edm.creation_date,&edm.host,&edm.database,&edm.database_version,&edm.database_creation_date,&edm.schema,&edm.model,&edm.model_creation_date,&edm.header_model,&edm.header_model_creation_date,&edm.user,&edm.group,&edm.license,&edm.options]{bytes=sum(bytes,value.len())?;text(value,&mut bound)?;}census.row(bytes,&bound)?;}
 for values in [&value.document.header.file_description,&value.document.header.file_name,&value.document.header.file_schema]{arguments(values,&mut bound,&mut census,&identities)?;}
 bound.check_rows(value.document.instances.len())?;for instance in &value.document.instances{census.row(sum(24,UnsignedWord::new(instance.id).text().len())?,&bound)?;bound.add(192)?;bound.check_rows(instance.entities.len())?;for(name,values)in &instance.entities{census.row(sum(24,name.len())?,&bound)?;bound.add(128)?;text(name,&mut bound)?;arguments(values,&mut bound,&mut census,&identities)?;}bound.checkpoint()?;}bound.finish()
}
