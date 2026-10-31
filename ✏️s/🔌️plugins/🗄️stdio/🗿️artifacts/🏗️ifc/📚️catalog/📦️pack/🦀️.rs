//! 📦️ Reader of the committed IFCP catalog resource: one raw DEFLATE stream (RFC 1951) around interned strings, deduplicated type expressions and attributes, and LEB128 integers. Written by `🛠️extract/📦️pack.ts`, which is the specification. See <https://www.rfc-editor.org/rfc/rfc1951>.
use super::{IfcAttributeDeclaration,IfcCatalog,IfcDerivedDeclaration,IfcEntityDeclaration,IfcTypeDeclaration,IfcTypeExpression};
use semio_framework_deflate::{inflate,DeflateError};
use std::collections::BTreeMap;
const MAGIC:[u8;5]=*b"IFCP\x01";
const MAXIMUM_PAYLOAD_BYTES:usize=1<<24;
/// 🚨️ Closed causes of a refused catalog resource.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CatalogPackError{Inflate(DeflateError),Magic,Truncated,Overflow,Utf8,Index,Trailing}
struct Cursor<'a>{bytes:&'a[u8],at:usize}
impl<'a> Cursor<'a>{
fn take(&mut self,length:usize)->Result<&'a[u8],CatalogPackError>{let end=self.at.checked_add(length).filter(|end|*end<=self.bytes.len()).ok_or(CatalogPackError::Truncated)?;let slice=&self.bytes[self.at..end];self.at=end;Ok(slice)}
fn byte(&mut self)->Result<u8,CatalogPackError>{Ok(self.take(1)?[0])}
fn wide(&mut self)->Result<u64,CatalogPackError>{let mut value=0u64;let mut shift=0u32;loop{let byte=self.byte()?;if shift>=64||shift==63&&byte>1{return Err(CatalogPackError::Overflow);}value|=u64::from(byte&127)<<shift;if byte<128{return Ok(value);}shift+=7;}}
fn index(&mut self)->Result<usize,CatalogPackError>{usize::try_from(self.wide()?).map_err(|_|CatalogPackError::Overflow)}
fn count(&mut self)->Result<usize,CatalogPackError>{let count=self.index()?;if count>self.bytes.len()-self.at{return Err(CatalogPackError::Truncated);}Ok(count)}
fn flag(&mut self)->Result<bool,CatalogPackError>{Ok(self.index()?==1)}
fn optional_integer(&mut self)->Result<Option<i64>,CatalogPackError>{let raw=self.wide()?;if raw==0{return Ok(None);}let zigzag=raw-1;let magnitude=i64::try_from(zigzag>>1).map_err(|_|CatalogPackError::Overflow)?;Ok(Some(if zigzag&1==1{-magnitude-1}else{magnitude}))}
}
struct Tables{strings:Vec<String>,expressions:Vec<IfcTypeExpression>,attributes:Vec<IfcAttributeDeclaration>}
impl Tables{
fn string(&self,cursor:&mut Cursor)->Result<String,CatalogPackError>{self.strings.get(cursor.index()?).cloned().ok_or(CatalogPackError::Index)}
fn optional_string(&self,cursor:&mut Cursor)->Result<Option<String>,CatalogPackError>{match cursor.index()?{0=>Ok(None),id=>self.strings.get(id-1).cloned().map(Some).ok_or(CatalogPackError::Index)}}
fn expression(&self,cursor:&mut Cursor)->Result<IfcTypeExpression,CatalogPackError>{self.expressions.get(cursor.index()?).cloned().ok_or(CatalogPackError::Index)}
fn strings(&self,cursor:&mut Cursor)->Result<Vec<String>,CatalogPackError>{(0..cursor.count()?).map(|_|self.string(cursor)).collect()}
fn attributes(&self,cursor:&mut Cursor)->Result<Vec<IfcAttributeDeclaration>,CatalogPackError>{(0..cursor.count()?).map(|_|self.attributes.get(cursor.index()?).cloned().ok_or(CatalogPackError::Index)).collect()}
}
/// 📥️ Decode a committed catalog resource into the retained declarations, bounded against hostile input.
pub fn decode_catalog_pack(pack:&[u8])->Result<IfcCatalog,CatalogPackError>{
let payload=inflate(pack,MAXIMUM_PAYLOAD_BYTES).map_err(CatalogPackError::Inflate)?;
let mut cursor=Cursor{bytes:&payload,at:0};
if cursor.take(MAGIC.len())?!=MAGIC{return Err(CatalogPackError::Magic);}
let mut tables=Tables{strings:Vec::new(),expressions:Vec::new(),attributes:Vec::new()};
for _ in 0..cursor.count()?{let length=cursor.count()?;tables.strings.push(String::from_utf8(cursor.take(length)?.to_vec()).map_err(|_|CatalogPackError::Utf8)?);}
for _ in 0..cursor.count()?{
let expression=match cursor.byte()?{
0=>IfcTypeExpression{kind:"named".into(),name:tables.optional_string(&mut cursor)?,width:cursor.optional_integer()?,fixed:cursor.flag()?,..Default::default()},
1=>{let aggregate=tables.optional_string(&mut cursor)?;let lower=cursor.optional_integer()?;let upper=cursor.optional_integer()?;let lower_expression=tables.optional_string(&mut cursor)?;let upper_expression=tables.optional_string(&mut cursor)?;let flags=cursor.index()?;let item=tables.expression(&mut cursor)?;IfcTypeExpression{kind:"aggregate".into(),aggregate,lower,upper,lower_expression,upper_expression,unique:flags&1==1,optional:flags&2==2,item:Some(Box::new(item)),..Default::default()}}
_=>return Err(CatalogPackError::Index),
};
tables.expressions.push(expression);
}
for _ in 0..cursor.count()?{let name=tables.string(&mut cursor)?;let type_expr=tables.expression(&mut cursor)?;let flags=cursor.index()?;let declared_by=tables.string(&mut cursor)?;tables.attributes.push(IfcAttributeDeclaration{name,type_expr,optional:flags&1==1,derived:flags&2==2,declared_by});}
let schema=tables.string(&mut cursor)?;let version=tables.string(&mut cursor)?;let source_url=tables.string(&mut cursor)?;let source_mirror=tables.string(&mut cursor)?;let source_sha256=tables.string(&mut cursor)?;
let mut entities=BTreeMap::new();
for _ in 0..cursor.count()?{
let key=tables.string(&mut cursor)?;let name=tables.string(&mut cursor)?;let abstract_entity=cursor.byte()?==1;let supertype=tables.optional_string(&mut cursor)?;let supertype_expression=tables.string(&mut cursor)?;let attributes=tables.attributes(&mut cursor)?;let all_attributes=tables.attributes(&mut cursor)?;
let derived=(0..cursor.count()?).map(|_|Ok(IfcDerivedDeclaration{name:tables.string(&mut cursor)?,type_expr:tables.expression(&mut cursor)?,redeclares:tables.optional_string(&mut cursor)?,expression:tables.string(&mut cursor)?})).collect::<Result<Vec<_>,CatalogPackError>>()?;
let inverse=tables.strings(&mut cursor)?;let where_rules=tables.strings(&mut cursor)?;let unique_rules=tables.strings(&mut cursor)?;
entities.insert(key,IfcEntityDeclaration{name,abstract_entity,supertype,supertype_expression,attributes,all_attributes,derived,inverse,where_rules,unique_rules});
}
let mut types=BTreeMap::new();
for _ in 0..cursor.count()?{
let key=tables.string(&mut cursor)?;let name=tables.string(&mut cursor)?;let kind=tables.string(&mut cursor)?;let members=tables.strings(&mut cursor)?;
let type_expr=match cursor.index()?{0=>None,id=>Some(tables.expressions.get(id-1).cloned().ok_or(CatalogPackError::Index)?)};
let where_rules=tables.strings(&mut cursor)?;
types.insert(key,IfcTypeDeclaration{name,kind,members,type_expr,where_rules});
}
let mut texts=[BTreeMap::new(),BTreeMap::new()];
for table in &mut texts{for _ in 0..cursor.count()?{let key=tables.string(&mut cursor)?;let text=tables.string(&mut cursor)?;table.insert(key,text);}}
if cursor.at!=payload.len(){return Err(CatalogPackError::Trailing);}
let[functions,rules]=texts;
Ok(IfcCatalog{schema,version,source_url,source_mirror,source_sha256,entities,types,functions,rules})
}
/// 🧬️ Decode the catalog resource a standard embeds with `include_bytes!`; the generator and the per-standard laws prove it decodes, so a refusal here is a corrupted build artifact.
pub fn embedded_catalog(pack:&[u8])->IfcCatalog{match decode_catalog_pack(pack){Ok(catalog)=>catalog,Err(error)=>panic!("embedded IFC catalog resource refused: {error:?}")}}
