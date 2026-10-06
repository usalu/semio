//! 🛂️ Borrowed literal PDF fields admitted against their handwritten SQL cells.
use crate::standards::v1_7::subsets::base::io::sqlite::snapshot::*;
use pack::value::DslValue as D;
use semio_framework_value::NativeDecodeControl;
#[derive(Clone,Copy)]
enum Cell<'a>{Null,Int,Real(f64),Text(&'a str),Bytes(usize)}
use Cell::{Null,Int,Real,Text,Bytes};
struct Cells<'a>{values:[Cell<'a>;64],length:usize}
impl<'a> Cells<'a>{
 fn new()->Self{Self{values:[Null;64],length:0}}
 fn from(values:&[Cell<'a>])->Self{let mut result=Self::new();result.extend(values.iter().copied());result}
 fn push(&mut self,value:Cell<'a>){self.values[self.length]=value;self.length+=1;}
 fn extend(&mut self,values:impl IntoIterator<Item=Cell<'a>>){for value in values{self.push(value);}}
}
impl<'a> std::ops::Deref for Cells<'a>{type Target=[Cell<'a>];fn deref(&self)->&Self::Target{&self.values[..self.length]}}
impl<'a> IntoIterator for Cells<'a>{type Item=Cell<'a>;type IntoIter=std::iter::Take<std::array::IntoIter<Cell<'a>,64>>;fn into_iter(self)->Self::IntoIter{self.values.into_iter().take(self.length)}}

struct Census<'a,'p>{native:&'a mut NativeDecodeControl<'p>,limits:sqlite_snapshot::SqliteDatabaseLimits,rows:usize,bytes:usize,depth:usize}
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"PDF borrowed SQL semantic field differs")}
fn limit()->ValueError{ValueError::new(ValueRefusalKind::OwnershipLimit,"PDF complete SQL semantic value bytes exceeded")}
fn field<'a>(value:&'a D,key:&str)->Result<&'a D,ValueError>{match value{D::Object(fields)=>Ok(fields.iter().find(|(name,_)|name==key).map_or(&D::Null,|(_,value)|value)),D::Null=>Ok(&D::Null),_=>Err(invalid())}}
fn list(value:&D)->Result<&[D],ValueError>{match value{D::Array(values)=>Ok(values),D::Null=>Ok(&[]),_=>Err(invalid())}}
fn text(value:&D)->Result<&str,ValueError>{value.as_str().ok_or_else(invalid)}
fn optional_text(value:&D)->Result<Cell<'_>,ValueError>{if matches!(value,D::Null){Ok(Null)}else{text(value).map(Text)}}
fn integer(value:&D)->Result<Cell<'_>,ValueError>{if matches!(value,D::Null){Ok(Null)}else if matches!(value,D::Number(_)|D::Bool(_)){Ok(Int)}else{Err(invalid())}}
fn real(value:&D)->Result<Cell<'_>,ValueError>{if matches!(value,D::Null){Ok(Null)}else{value.as_f64().map(Real).ok_or_else(invalid)}}
fn kind(value:&D)->Result<&str,ValueError>{text(field(value,"kind")?)}
impl Census<'_,'_>{
 fn row(&mut self,table:&str,cells:&[Cell<'_>])->Result<(),ValueError>{
  let columns=number::columns(table);let width=cells.len().checked_add(columns.len()*2).and_then(|n|n.checked_add(1)).ok_or_else(limit)?;
  if width>self.limits.max_columns{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"PDF authored SQL row column limit exceeded"))}
  let mut bytes=8usize;
  for cell in cells{let size=match cell{Null=>0,Int|Real(_)=>8,Text(text)=>text.len(),Bytes(bytes)=>*bytes};bytes=bytes.checked_add(size).ok_or_else(limit)?;}
  for column in columns{let index=match column{sqlite_snapshot::artifact::FloatColumn::Binary64(i)|sqlite_snapshot::artifact::FloatColumn::Binary32(i)=>*i};match cells.get(index-1).ok_or_else(invalid)?{
   Null=>{},Real(value)=>{let class=if value.is_nan(){bytes-=8;"nan"}else if *value==f64::INFINITY{"positiveInfinity"}else if *value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};bytes=bytes.checked_add(8+class.len()).ok_or_else(limit)?;},_=>return Err(invalid())
  }}
  self.bytes=self.bytes.checked_add(bytes).filter(|n|*n<=self.limits.max_value_bytes).ok_or_else(limit)?;
  self.rows=self.rows.checked_add(1).filter(|n|*n<=self.limits.max_rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"PDF complete SQL semantic row limit exceeded"))?;
  self.native.step()
 }
 fn blob<'a>(&mut self,value:&'a D)->Result<Cell<'a>,ValueError>{match value{
  D::Null=>Ok(Null),D::Bytes(bytes)=>Ok(Bytes(bytes.len())),D::Array(values)=>{for value in values{if !value.as_u64().is_some_and(|value|value<=255){return Err(invalid())}self.native.step()?;}Ok(Bytes(values.len()))},_=>Err(invalid())
 }}
 fn relation(&mut self,table:&str)->Result<(),ValueError>{self.row(table,&[Int,Int,Int])}
}

fn real_or(value:&D,default:f64)->Result<Cell<'_>,ValueError>{if matches!(value,D::Null){Ok(Real(default))}else{real(value)}}
#[path="🧩️cos/🦀️.rs"] mod objects;
#[path="🌈️color/🦀️.rs"] mod colors;
#[path="🔤️font/🦀️.rs"] mod glyphs;
#[path="🖋️content/🦀️.rs"] mod render;
#[path="🖼️resource/🦀️.rs"] mod resources;
#[path="📇️metadata/🦀️.rs"] mod meta;
#[path="🎯️navigation/🦀️.rs"] mod nav;
#[path="📌️annotation/🦀️.rs"] mod annotations;
#[path="📝️form/🦀️.rs"] mod forms;
fn page(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 render::ops(c,field(v,"content")?)?;let group=optional(c,field(v,"group")?,resources::group)?;let transition=optional(c,field(v,"transition")?,objects::dictionary)?;objects::dictionary(c,field(v,"additionalActions")?)?;objects::dictionary(c,field(v,"extra")?)?;
 let mut f=Cells::new();for key in ["mediaBox","cropBox","bleedBox","trimBox","artBox"]{f.extend(resources::rect(field(v,key)?,4)?);}
 f.extend([Int,real(field(v,"userUnit")?)?,Int,group,optional_text(field(v,"thumbnail")?)?,integer(field(v,"structParents")?)?,transition,real(field(v,"duration")?)?,optional_text(field(v,"metadata")?)?,Int,Int]);c.row("pdf_page",&f)?;
 for a in list(field(v,"annotations")?)?{annotations::annotation(c,a)?;c.relation("pdf_page_annotation")?;}Ok(())
}
fn named_color(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{colors::color(c,field(v,"colorSpace")?)?;c.row("pdf_named_color",&[Text(text(field(v,"name")?)?),Int])}
fn properties(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{objects::dictionary(c,field(v,"entries")?)?;c.row("pdf_named_properties",&[Text(text(field(v,"name")?)?),Int])}
fn indirect(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{objects::object(c,field(v,"value")?)?;c.row("pdf_indirect_object",&[Int;3])}
fn optional(c:&mut Census<'_,'_>,v:&D,visit:fn(&mut Census<'_,'_>,&D)->Result<(),ValueError>)->Result<Cell<'static>,ValueError>{if matches!(v,D::Null){Ok(Null)}else{visit(c,v)?;Ok(Int)}}
pub(super) fn borrowed(record:&semio_framework_dsl_record::RecordValue,native:&mut NativeDecodeControl<'_>,limits:sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{
 use semio_framework_dsl_record::FieldValue as F;
 native.scoped_stage(|native|{
 native.begin_stage(0)?;
 let value=|id|->Result<&D,ValueError>{match record.get(id){Some(F::Value(v))=>Ok(v),None|Some(F::Absent)if(17..=27).contains(&id)=>Ok(&D::Null),_=>Err(invalid())}};
 let root_text=|id|->Result<&str,ValueError>{match record.get(id){Some(F::Text(v))=>Ok(v),_=>Err(invalid())}};
 let mut c=Census{native,limits,rows:0,bytes:0,depth:0};
 let acro=optional(&mut c,value(17)?,forms::acro).map_err(|error|error.under("acroForm"))?;let oc=optional(&mut c,value(18)?,forms::optional).map_err(|error|error.under("optionalContent"))?;let prefs=optional(&mut c,value(21)?,meta::preferences).map_err(|error|error.under("viewerPreferences"))?;let open=optional(&mut c,value(22)?,nav::open).map_err(|error|error.under("openAction"))?;let mark=optional(&mut c,value(24)?,meta::mark).map_err(|error|error.under("markInfo"))?;let enc=optional(&mut c,value(27)?,meta::encryption).map_err(|error|error.under("encryption"))?;meta::info(&mut c,value(28)?)?;objects::dictionary(&mut c,value(29)?)?;objects::dictionary(&mut c,value(31)?)?;
 let ids=list(value(26)?)?;let (first,second)=if ids.is_empty(){(Null,Null)}else{if ids.len()!=2{return Err(invalid())}(c.blob(&ids[0])?,c.blob(&ids[1])?)};
 c.row("pdf_document",&[Text(root_text(1)?),Text(root_text(2)?),acro,oc,optional_text(value(19)?)?,optional_text(value(20)?)?,prefs,open,optional_text(value(23)?)?,mark,optional_text(value(25)?)?,first,second,enc,Int,Int,Int])?;
 let collections:[(u16,&str,fn(&mut Census<'_,'_>,&D)->Result<(),ValueError>);15]=[
 (3,"pdf_document_page",page),(4,"pdf_document_font",glyphs::font),(5,"pdf_document_image",resources::image),(6,"pdf_document_form",resources::form),(7,"pdf_document_state",resources::state),(8,"pdf_document_shading",resources::shading),(9,"pdf_document_pattern",resources::pattern),(10,"pdf_document_color",named_color),(11,"pdf_document_properties",properties),(12,"pdf_document_outline",nav::outline),(13,"pdf_document_destination",nav::named),(14,"pdf_document_label",nav::label),(15,"pdf_document_file",meta::file),(16,"pdf_document_intent",meta::intent),(30,"pdf_document_object",indirect)];
 for(id,table,visit)in collections{for child in list(value(id)?)?{visit(&mut c,child).map_err(|error|error.under(table))?;c.relation(table)?;}}
 Ok(())
 })
}
