//! 🧮️ Domain-owned Part21 cells are admitted before flat typed ownership.
const TABLES:[(&str,usize);12]=[("ifc2x3_document",4),("ifc2x3_edm_preamble",17),("ifc2x3_entity_argument",4),("ifc2x3_entity_type",4),("ifc2x3_file_description_argument",4),("ifc2x3_file_name_argument",4),("ifc2x3_file_schema_argument",4),("ifc2x3_header",1),("ifc2x3_instance",4),("ifc2x3_list_element",4),("ifc2x3_typed_argument",4),("ifc2x3_value",12)];
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::NativeDecodeControl;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Part21 native role or exclusive forest differs from its authored fields")}
fn sum(left:usize,right:usize)->Result<usize>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 semantic extent overflow"))}
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{
 let bytes=Ifc2x3Snapshot::SQLITE_SCHEMA.split(';').map(str::trim).try_fold(0usize,|total,sql|sum(total,sql.len()))?;
 let bytes=TABLES.iter().try_fold(bytes,|total,(name,_)|sum(total,name.len()))?;
 if bytes.max(Ifc2x3Snapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 authored schema exceeds caller bytes"))}
 if TABLES.len()>limits.max_tables||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<2{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Part21 authored relational extent exceeds caller limits"))}Ok(())
}
fn closed(value:&R,last:u16)->Result<()>{if value.fields.keys().any(|id|*id>last){return Err(invalid())}Ok(())}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(invalid)}
fn record(value:&F,last:u16)->Result<&R>{let F::Record(value)=value else{return Err(invalid())};closed(value,last)?;Ok(value)}
fn list(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid())};Ok(value)}
fn uint(value:&F)->Result<u64>{let F::UInt(value)=value else{return Err(invalid())};Ok(*value)}
fn optional(value:&R,id:u16)->Option<&F>{match value.get(id){None|Some(F::Absent)=>None,value=>value}}
fn text<'a>(value:&'a F,native:&mut NativeDecodeControl<'_>)->Result<&'a str>{let F::Text(value)=value else{return Err(invalid())};native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value)}
fn optional_text<'a>(value:&'a R,id:u16,native:&mut NativeDecodeControl<'_>)->Result<Option<&'a str>>{optional(value,id).map(|value|text(value,native)).transpose()}
fn optional_int(value:&R,id:u16)->Result<Option<i64>>{optional(value,id).map(|value|{let F::Int(value)=value else{return Err(invalid())};Ok(*value)}).transpose()}
fn optional_uint(value:&R,id:u16)->Result<Option<u64>>{optional(value,id).map(uint).transpose()}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Part21 semantic row extent overflow"))?;let total=sum(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Part21 semantic rows exceed caller limit"))}if total>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 semantic values exceed caller limit"))}native.step()?;self.rows=rows;self.bytes=total;Ok(())}
 fn edges(&mut self,values:&[F],owners:&mut[u8],parent:Option<usize>,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{for value in values{let index=usize::try_from(uint(value)?).map_err(|_|invalid())?;if parent.is_some_and(|parent|index<=parent){return Err(invalid())}let Some(owner)=owners.get_mut(index)else{return Err(invalid())};if *owner!=0{return Err(invalid())}*owner=1;self.row(bytes,native)?;}Ok(())}
}
fn identities(values:&[F],last:u16,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<Vec<u64>>{
 if values.len()>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Part21 identity count exceeds caller rows"))}
 let mut identities=native.allocate_vec(values.len())?;for value in values{identities.push(uint(field(record(value,last)?,0)?)?);native.step()?;}
 super::sort_paid(&mut identities,|left,right|{native.step()?;Ok(left.cmp(right))})?;for pair in identities.windows(2){if pair[0]==pair[1]{return Err(invalid())}native.step()?;}Ok(identities)
}
fn nodes(values:&[F],identities:&[u64],owners:&mut[u8],census:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 for(index,value)in values.iter().enumerate(){let value=record(value,9)?;let kind=text(field(value,0)?,native)?;let integer=optional_int(value,1)?;let reference=optional_uint(value,2)?;let literal=optional_text(value,3,native)?;let name=optional_text(value,4,native)?;let negative=optional(value,5).map(|value|{let F::Bool(value)=value else{return Err(invalid())};Ok(*value)}).transpose()?;let coefficient=optional_text(value,6,native)?;let scale=optional_uint(value,7)?;if scale.is_some_and(|value|value>u32::MAX as u64){return Err(invalid())}let exponent=optional_int(value,8)?;if exponent.is_some_and(|value|i32::try_from(value).is_err()){return Err(invalid())}let children=list(field(value,9)?)?;
  let payloads=usize::from(integer.is_some())+usize::from(reference.is_some())+usize::from(literal.is_some())+usize::from(name.is_some())+usize::from(negative.is_some())+usize::from(coefficient.is_some())+usize::from(scale.is_some())+usize::from(exponent.is_some());
  let valid=match kind{"unset"|"derived"=>payloads==0&&children.is_empty(),"integer"=>integer.is_some()&&payloads==1&&children.is_empty(),"reference"=>reference.is_some()&&payloads==1&&children.is_empty(),"string"|"enum"=>literal.is_some()&&payloads==1&&children.is_empty(),"decimal"=>negative.is_some()&&coefficient.is_some()&&scale.is_some()&&payloads==3+usize::from(exponent.is_some())&&children.is_empty(),"list"=>payloads==0,"typed"=>name.is_some()&&payloads==1,_=>false};if !valid{return Err(invalid())}
  let sql_kind=match kind{"integer"=>"int","reference"=>"ref","string"=>"str","decimal"=>"real",value=>value};let mut bytes=sum(8,sql_kind.len())?;
  if integer.is_some(){bytes=sum(bytes,8)?;}if let Some(value)=literal{bytes=sum(bytes,value.len())?;}if let Some(value)=name{bytes=sum(bytes,value.len())?;}
  if let Some(value)=reference{bytes=sum(bytes,UnsignedWord::new(value).text().len())?;if identities.binary_search(&value).is_ok(){bytes=sum(bytes,8)?;}}
  if kind=="decimal"{bytes=sum(bytes,sum(16,coefficient.unwrap().len())?)?;if exponent.is_some(){bytes=sum(bytes,8)?;}}
  census.row(bytes,native)?;census.edges(children,owners,Some(index),32,native)?;
 }Ok(())
}
/// 🫳️ Admits exact decimal and EDM literals with paid identity and forest frontiers.
pub(super)fn admit_record(value:&R,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<()>{
 extent(limits)?;native.scoped_stage(|native|{
  native.begin_stage(0)?;closed(value,6)?;let instances=list(field(value,4)?)?;let values=list(field(value,5)?)?;if values.len()>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 node count exceeds caller rows"))}
  let edm=optional(value,6).map(|value|record(value,15)).transpose()?;let mut census=Census{limits,rows:0,bytes:0};census.row(sum(sum(16,text(field(value,0)?,native)?.len())?,if edm.is_some(){8}else{0})?,native)?;census.row(8,native)?;if let Some(edm)=edm{let mut bytes=8usize;for id in 0..16{bytes=sum(bytes,text(field(edm,id)?,native)?.len())?;}census.row(bytes,native)?;}
  let identities=identities(instances,1,limits,native)?;let mut owners=native.allocate_vec::<u8>(values.len())?;owners.resize(values.len(),0);for id in[1,2,3]{census.edges(list(field(value,id)?)?,&mut owners,None,32,native)?;}
  for value in instances{let value=record(value,1)?;let word=UnsignedWord::new(uint(field(value,0)?)?);census.row(sum(24,word.text().len())?,native)?;for value in list(field(value,1)?)?{let value=record(value,1)?;census.row(sum(24,text(field(value,0)?,native)?.len())?,native)?;census.edges(list(field(value,1)?)?,&mut owners,None,32,native)?;}}
  nodes(values,&identities,&mut owners,&mut census,native)?;for owner in owners{if owner!=1{return Err(invalid())}native.step()?;}native.checkpoint()
 })
}
