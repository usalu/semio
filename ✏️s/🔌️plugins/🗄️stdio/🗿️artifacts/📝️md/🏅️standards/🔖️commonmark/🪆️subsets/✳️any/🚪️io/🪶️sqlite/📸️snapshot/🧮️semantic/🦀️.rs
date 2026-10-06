//! 📝️ Borrowed CommonMark roles admit full SQL cells and exclusive ordered ownership.
const TABLES:[(&str,usize);27]=[("md_block",2),("md_block_inline",4),("md_block_quote",1),("md_code_block",3),("md_code_span",2),("md_document",2),("md_document_block",4),("md_emphasis",1),("md_emphasis_inline",4),("md_hard_break",1),("md_heading",2),("md_html_block",2),("md_html_inline",2),("md_image",4),("md_inline",2),("md_link",3),("md_link_text",4),("md_list",4),("md_list_item",3),("md_list_item_block",4),("md_paragraph",1),("md_quote_block",4),("md_soft_break",1),("md_strong",1),("md_strong_inline",4),("md_text",2),("md_thematic_break",1)];
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::NativeDecodeControl;
use store::sqlite_snapshot::SqliteDatabaseLimits;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"CommonMark literal role, variant or exclusive forest differs")}
fn sum(left:usize,right:usize)->Result<usize>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark semantic extent overflow"))}
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{
 let bytes=MdSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).try_fold(0usize,|total,sql|sum(total,sql.len()))?;
 let bytes=TABLES.iter().try_fold(bytes,|total,(name,_)|sum(total,name.len()))?;
 if bytes.max(MdSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark authored schema exceeds caller bytes"))}
 if TABLES.len()>limits.max_tables||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark authored relational extent exceeds caller limits"))}Ok(())
}
fn exact(value:&R,count:usize)->Result<()>{if value.fields.len()!=count||value.fields.keys().any(|id|usize::from(*id)>=count){return Err(invalid())}Ok(())}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(invalid)}
fn record(value:&F,count:usize)->Result<&R>{let F::Record(value)=value else{return Err(invalid())};exact(value,count)?;Ok(value)}
fn list(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid())};Ok(value)}
fn uint(value:&F,maximum:u64)->Result<u64>{match value{F::UInt(value)if *value<=maximum=>Ok(*value),_=>Err(invalid())}}
fn kind(value:&F,names:&[&'static str])->Result<(usize,&'static str)>{let F::Enum(value)=value else{return Err(invalid())};let index=*value as usize;Ok((index,*names.get(index).ok_or_else(invalid)?))}
fn text(value:&F,native:&mut NativeDecodeControl<'_>)->Result<usize>{let F::Text(value)=value else{return Err(invalid())};native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value.len())}
fn optional_text(value:&F,native:&mut NativeDecodeControl<'_>)->Result<Option<usize>>{match value{F::Absent=>Ok(None),value=>text(value,native).map(Some)}}
fn optional_uint(value:&F,maximum:u64)->Result<Option<u64>>{match value{F::Absent=>Ok(None),value=>uint(value,maximum).map(Some)}}
fn optional_bool(value:&F)->Result<Option<bool>>{match value{F::Absent=>Ok(None),F::Bool(value)=>Ok(Some(*value)),_=>Err(invalid())}}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark semantic rows overflow"))?;let total=sum(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark semantic rows exceed caller limit"))}if total>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark semantic cells exceed caller limit"))}native.step()?;self.rows=rows;self.bytes=total;Ok(())}
 fn edges(&mut self,values:&[F],owners:&mut[u8],parent:Option<usize>,native:&mut NativeDecodeControl<'_>)->Result<()>{for value in values{let index=usize::try_from(uint(value,u64::MAX)?).map_err(|_|invalid())?;if parent.is_some_and(|parent|index<=parent){return Err(invalid())}let Some(owner)=owners.get_mut(index)else{return Err(invalid())};if *owner!=0{return Err(invalid())}*owner=1;self.row(32,native)?;}Ok(())}
}
/// 🫳️ Pays only actual borrowed graph frontiers before the owner's existing flat binder.
pub(super)fn admit_record(value:&R,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<()>{
 extent(limits)?;native.scoped_stage(|native|{
  native.begin_stage(0)?;exact(value,4)?;let blocks=list(field(value,2)?)?;let inlines=list(field(value,3)?)?;if blocks.len().checked_add(inlines.len()).is_none_or(|count|count>limits.max_rows){return Err(ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark node count exceeds caller rows"))}
  let mut census=Census{limits,rows:0,bytes:0};census.row(sum(8,text(field(value,0)?,native)?)?,native)?;let mut block_owners=native.allocate_vec::<u8>(blocks.len())?;block_owners.resize(blocks.len(),0);let mut inline_owners=native.allocate_vec::<u8>(inlines.len())?;inline_owners.resize(inlines.len(),0);census.edges(list(field(value,1)?)?,&mut block_owners,None,native)?;
  for(index,value)in blocks.iter().enumerate(){let value=record(value,11)?;let(kind,name)=kind(field(value,0)?,&["heading","paragraph","list","codeBlock","blockQuote","thematicBreak","htmlBlock"])?;let level=optional_uint(field(value,1)?,u8::MAX as u64)?;let inline_ids=list(field(value,2)?)?;let ordered=optional_bool(field(value,3)?)?;let start=optional_uint(field(value,4)?,u32::MAX as u64)?;let tight=optional_bool(field(value,5)?)?;let items=list(field(value,6)?)?;let block_ids=list(field(value,7)?)?;let info=optional_text(field(value,8)?,native)?;let literal=optional_text(field(value,9)?,native)?;let raw=optional_text(field(value,10)?,native)?;
   let mask=(usize::from(level.is_some())<<1)|(usize::from(ordered.is_some())<<3)|(usize::from(start.is_some())<<4)|(usize::from(tight.is_some())<<5)|(usize::from(info.is_some())<<8)|(usize::from(literal.is_some())<<9)|(usize::from(raw.is_some())<<10);
   let allowed=match kind{0=>1<<1,1|4|5=>0,2=>(1<<3)|(1<<4)|(1<<5),3=>(1<<8)|(1<<9),6=>1<<10,_=>return Err(invalid())};
   if mask&!allowed!=0||!matches!(kind,0|1)&&!inline_ids.is_empty()||kind!=2&&!items.is_empty()||kind!=4&&!block_ids.is_empty(){return Err(invalid())}census.row(sum(8,name.len())?,native)?;
   match kind{
    0=>{level.ok_or_else(invalid)?;census.row(16,native)?;census.edges(inline_ids,&mut inline_owners,None,native)?;}
    1=>{census.row(8,native)?;census.edges(inline_ids,&mut inline_owners,None,native)?;}
    2=>{ordered.ok_or_else(invalid)?;tight.ok_or_else(invalid)?;census.row(24+if start.is_some(){8}else{0},native)?;for value in items{let value=record(value,1)?;census.row(24,native)?;census.edges(list(field(value,0)?)?,&mut block_owners,Some(index),native)?;}}
    3=>census.row(sum(sum(8,info.unwrap_or(0))?,literal.ok_or_else(invalid)?)?,native)?,
    4=>{census.row(8,native)?;census.edges(block_ids,&mut block_owners,Some(index),native)?;}
    5=>census.row(8,native)?,
    6=>census.row(sum(8,raw.ok_or_else(invalid)?)?,native)?,
    _=>return Err(invalid())
   }
  }
  for(index,value)in inlines.iter().enumerate(){let value=record(value,8)?;let(kind,name)=kind(field(value,0)?,&["text","emphasis","strong","code","link","image","softBreak","hardBreak","htmlInline"])?;let literal_text=optional_text(field(value,1)?,native)?;let literal=optional_text(field(value,2)?,native)?;let url=optional_text(field(value,3)?,native)?;let title=optional_text(field(value,4)?,native)?;let alt=optional_text(field(value,5)?,native)?;let raw=optional_text(field(value,6)?,native)?;let children=list(field(value,7)?)?;
   let mask=(usize::from(literal_text.is_some())<<1)|(usize::from(literal.is_some())<<2)|(usize::from(url.is_some())<<3)|(usize::from(title.is_some())<<4)|(usize::from(alt.is_some())<<5)|(usize::from(raw.is_some())<<6);
   let allowed=match kind{0=>1<<1,1|2|6|7=>0,3=>1<<2,4=>(1<<3)|(1<<4),5=>(1<<3)|(1<<4)|(1<<5),8=>1<<6,_=>return Err(invalid())};if mask&!allowed!=0||!matches!(kind,1|2|4)&&!children.is_empty(){return Err(invalid())}census.row(sum(8,name.len())?,native)?;
   let bytes=match kind{0=>sum(8,literal_text.ok_or_else(invalid)?)?,1|2|6|7=>8,3=>sum(8,literal.ok_or_else(invalid)?)?,4=>sum(sum(8,url.ok_or_else(invalid)?)?,title.unwrap_or(0))?,5=>sum(sum(sum(8,alt.ok_or_else(invalid)?)?,url.ok_or_else(invalid)?)?,title.unwrap_or(0))?,8=>sum(8,raw.ok_or_else(invalid)?)?,_=>return Err(invalid())};census.row(bytes,native)?;census.edges(children,&mut inline_owners,Some(index),native)?;
  }
  for owner in block_owners.into_iter().chain(inline_owners){if owner!=1{return Err(invalid())}native.step()?;}native.checkpoint()
 })
}
