//! 🧾️ Reader of the committed IFCJ resource: one raw DEFLATE stream (RFC 1951) around interned strings, a tagged value tree and LEB128 integers. Written by `🛠️extract/📦️json-pack.ts`, which is the specification. See <https://www.rfc-editor.org/rfc/rfc1951>.
use semio_framework_deflate::{inflate,DeflateError};
use std::collections::HashMap;
const MAGIC:[u8;5]=*b"IFCJ\x01";
const MAXIMUM_PAYLOAD_BYTES:usize=1<<25;
const MAXIMUM_DEPTH:usize=512;
const NULL:u8=0;
const FALSE:u8=1;
const TRUE:u8=2;
const UNSIGNED:u8=3;
const NEGATIVE:u8=4;
const STRING:u8=5;
const ARRAY:u8=6;
const OBJECT:u8=7;
/// 🚨️ Closed causes of a refused resource or of a decoded value that does not have the expected shape.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum JsonPackError{Inflate(DeflateError),Magic,Truncated,Overflow,Utf8,Index,Tag,Depth,Trailing,Shape(&'static str)}
/// 🌳️ A decoded JSON value whose strings borrow the inflated payload; floating point numbers do not exist in the container.
#[derive(Clone,Debug,PartialEq)]
pub enum Json<'a>{Null,Bool(bool),Int(i64),Str(&'a str),Arr(Vec<Json<'a>>),Obj(Vec<(&'a str,Json<'a>)>)}
/// 🔑️ Members of an object whose keys were checked against a closed list.
#[derive(Clone,Copy,Debug)]
pub struct Fields<'r,'a>(&'r[(&'a str,Json<'a>)]);
impl<'r,'a> Fields<'r,'a>{
/// 🔎️ Borrow an optional member.
pub fn get(&self,key:&str)->Option<&'r Json<'a>>{self.0.iter().find(|(name,_)|*name==key).map(|(_,value)|value)}
/// 🔎️ Borrow a mandatory member.
pub fn require(&self,key:&'static str)->Result<&'r Json<'a>,JsonPackError>{self.get(key).ok_or(JsonPackError::Shape(key))}
}
impl<'a> Json<'a>{
/// 🔤️ Read a string.
pub fn string(&self)->Result<&'a str,JsonPackError>{match self{Json::Str(text)=>Ok(text),_=>Err(JsonPackError::Shape("string"))}}
/// 🔢️ Read an integer.
pub fn integer(&self)->Result<i64,JsonPackError>{match self{Json::Int(value)=>Ok(*value),_=>Err(JsonPackError::Shape("integer"))}}
/// 🔢️ Read a non-negative integer as an index.
pub fn index(&self)->Result<usize,JsonPackError>{usize::try_from(self.integer()?).map_err(|_|JsonPackError::Shape("index"))}
/// ✅️ Read a boolean.
pub fn boolean(&self)->Result<bool,JsonPackError>{match self{Json::Bool(value)=>Ok(*value),_=>Err(JsonPackError::Shape("boolean"))}}
/// 📚️ Read an array.
pub fn array(&self)->Result<&[Json<'a>],JsonPackError>{match self{Json::Arr(items)=>Ok(items),_=>Err(JsonPackError::Shape("array"))}}
/// 🔑️ Read an object whose keys are unique and all listed in `allowed`.
pub fn object(&self,allowed:&[&str])->Result<Fields<'_,'a>,JsonPackError>{
let Json::Obj(members)=self else{return Err(JsonPackError::Shape("object"));};
for(position,(key,_))in members.iter().enumerate(){if !allowed.contains(key)||members[..position].iter().any(|(earlier,_)|earlier==key){return Err(JsonPackError::Shape("member"));}}
Ok(Fields(members))
}
}
struct Cursor<'a>{bytes:&'a[u8],at:usize}
impl<'a> Cursor<'a>{
fn take(&mut self,length:usize)->Result<&'a[u8],JsonPackError>{let end=self.at.checked_add(length).filter(|end|*end<=self.bytes.len()).ok_or(JsonPackError::Truncated)?;let slice=&self.bytes[self.at..end];self.at=end;Ok(slice)}
fn byte(&mut self)->Result<u8,JsonPackError>{Ok(self.take(1)?[0])}
fn wide(&mut self)->Result<u64,JsonPackError>{let mut value=0u64;let mut shift=0u32;loop{let byte=self.byte()?;if shift>=64||shift==63&&byte>1{return Err(JsonPackError::Overflow);}value|=u64::from(byte&127)<<shift;if byte<128{return Ok(value);}shift+=7;}}
fn index(&mut self)->Result<usize,JsonPackError>{usize::try_from(self.wide()?).map_err(|_|JsonPackError::Overflow)}
fn count(&mut self)->Result<usize,JsonPackError>{let count=self.index()?;if count>self.bytes.len()-self.at{return Err(JsonPackError::Truncated);}Ok(count)}
fn value(&mut self,strings:&[&'a str],depth:usize)->Result<Json<'a>,JsonPackError>{
if depth>MAXIMUM_DEPTH{return Err(JsonPackError::Depth);}
let string=|cursor:&mut Self|-> Result<&'a str,JsonPackError>{strings.get(cursor.index()?).copied().ok_or(JsonPackError::Index)};
match self.byte()?{
NULL=>Ok(Json::Null),
FALSE=>Ok(Json::Bool(false)),
TRUE=>Ok(Json::Bool(true)),
UNSIGNED=>Ok(Json::Int(i64::try_from(self.wide()?).map_err(|_|JsonPackError::Overflow)?)),
NEGATIVE=>Ok(Json::Int(-i64::try_from(self.wide()?).map_err(|_|JsonPackError::Overflow)?-1)),
STRING=>Ok(Json::Str(string(self)?)),
ARRAY=>{let count=self.count()?;let mut items=Vec::with_capacity(count);for _ in 0..count{items.push(self.value(strings,depth+1)?);}Ok(Json::Arr(items))}
OBJECT=>{let count=self.count()?;let mut members=Vec::with_capacity(count);for _ in 0..count{let key=string(self)?;members.push((key,self.value(strings,depth+1)?));}Ok(Json::Obj(members))}
_=>Err(JsonPackError::Tag),
}
}
}
/// 📥️ Inflate a committed resource and hand the decoded value tree to `read`, bounded against hostile input; the tree borrows the payload for the duration of the call.
pub fn with_json_pack<R>(pack:&[u8],read:impl for<'a> FnOnce(&Json<'a>)->Result<R,JsonPackError>)->Result<R,JsonPackError>{
let payload=inflate(pack,MAXIMUM_PAYLOAD_BYTES).map_err(JsonPackError::Inflate)?;
let mut cursor=Cursor{bytes:&payload,at:0};
if cursor.take(MAGIC.len())?!=MAGIC{return Err(JsonPackError::Magic);}
let mut strings=Vec::new();
for _ in 0..cursor.count()?{let length=cursor.count()?;strings.push(std::str::from_utf8(cursor.take(length)?).map_err(|_|JsonPackError::Utf8)?);}
let root=cursor.value(&strings,0)?;
if cursor.at!=payload.len(){return Err(JsonPackError::Trailing);}
read(&root)
}
/// ♾️ Interns decoded text and slices as process-lifetime statics; the decoded tables are immutable and shared, so leaking them once is the allocation.
#[derive(Default)]
pub struct Leaker{strings:HashMap<&'static str,&'static str>}
impl Leaker{
/// 🔤️ Leak one text once; equal texts share the leaked allocation.
pub fn text(&mut self,text:&str)->&'static str{
if text.is_empty(){return "";}
if let Some(found)=self.strings.get(text){return *found;}
let leaked:&'static str=Box::leak(text.to_owned().into_boxed_str());
self.strings.insert(leaked,leaked);
leaked
}
/// 🔤️ Leak an optional text.
pub fn optional(&mut self,value:Option<&Json<'_>>)->Result<Option<&'static str>,JsonPackError>{match value{None|Some(Json::Null)=>Ok(None),Some(json)=>Ok(Some(self.text(json.string()?)))}}
/// 🔤️ Leak an array of texts.
pub fn texts(&mut self,value:Option<&Json<'_>>)->Result<&'static[&'static str],JsonPackError>{match value{None=>Ok(&[]),Some(json)=>{let items=json.array()?.iter().map(|item|Ok(self.text(item.string()?))).collect::<Result<Vec<_>,JsonPackError>>()?;Ok(Self::slice(items))}}}
/// 📚️ Leak a vector as an immutable slice.
pub fn slice<T>(items:Vec<T>)->&'static[T]{if items.is_empty(){&[]}else{Box::leak(items.into_boxed_slice())}}
}
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod laws;
