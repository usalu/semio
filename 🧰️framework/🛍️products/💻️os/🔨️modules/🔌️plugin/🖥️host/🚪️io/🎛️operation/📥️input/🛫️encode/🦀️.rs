//! 🛫️ Borrowed host JSON uses scalar refusals and the original native writer without error allocations.
use super::{JsonInputWriter,ValueError,ValueRefusalKind};
use serde::ser::{self,Serialize};
use std::fmt::Write;

#[derive(Debug)]
pub(super) struct Refusal(ValueError);
impl std::fmt::Display for Refusal{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Display::fmt(&self.0,f)}}
impl std::error::Error for Refusal{}
impl ser::Error for Refusal{fn custom<T:std::fmt::Display>(_:T)->Self{Self(ValueError::literal(ValueRefusalKind::InvalidValue,"host JSON source refused serialization"))}}
impl From<ValueError> for Refusal{fn from(error:ValueError)->Self{Self(error)}}
fn invalid()->Refusal{Refusal(ValueError::literal(ValueRefusalKind::InvalidValue,"host JSON source has an invalid key or scalar"))}
struct Scalar{bytes:[u8;384],length:usize}
impl Scalar{fn new()->Self{Self{bytes:[0;384],length:0}}fn text(&self)->&str{std::str::from_utf8(&self.bytes[..self.length]).expect("formatted scalar is UTF8")}}
impl Write for Scalar{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}}

pub(super) fn write<I:Serialize>(writer:&mut JsonInputWriter<'_,'_,'_>,input:&I)->Result<(),ValueError>{input.serialize(&mut Encoder{writer,key:false}).map_err(|error|error.0)}
struct Encoder<'a,'control,'observer,'output>{writer:&'a mut JsonInputWriter<'control,'observer,'output>,key:bool}
impl Encoder<'_,'_,'_,'_>{
 fn raw(&mut self,bytes:&[u8])->Result<(),Refusal>{self.writer.native_write(bytes).map_err(Refusal)}
 fn string(&mut self,text:&str)->Result<(),Refusal>{self.raw(b"\"")?;self.contents(text)?;self.raw(b"\"")}
 fn contents(&mut self,text:&str)->Result<(),Refusal>{let bytes=text.as_bytes();let mut start=0;for(index,byte)in bytes.iter().copied().enumerate(){let escape=match byte{b'"'=>Some(&b"\\\""[..]),b'\\'=>Some(&b"\\\\"[..]),8=>Some(&b"\\b"[..]),9=>Some(&b"\\t"[..]),10=>Some(&b"\\n"[..]),12=>Some(&b"\\f"[..]),13=>Some(&b"\\r"[..]),0..=31=>Some(&b""[..]),_=>None};if let Some(escape)=escape{if start<index{self.raw(&bytes[start..index])?;}if escape.is_empty(){const HEX:&[u8;16]=b"0123456789abcdef";self.raw(&[b'\\',b'u',b'0',b'0',HEX[(byte>>4)as usize],HEX[(byte&15)as usize]])?;}else{self.raw(escape)?;}start=index+1;}}if start<bytes.len(){self.raw(&bytes[start..])?;}Ok(())}
 fn number(&mut self,value:impl std::fmt::Display)->Result<(),Refusal>{let mut scalar=Scalar::new();write!(scalar,"{value}").map_err(|_|invalid())?;if self.key{self.string(scalar.text())}else{self.raw(scalar.text().as_bytes())}}
 fn float(&mut self,value:f64)->Result<(),Refusal>{if self.key{return Err(invalid());}let mut scalar=Scalar::new();semio_framework_pack_json::write_float_to(value,&mut scalar).map_err(|_|invalid())?;self.raw(scalar.text().as_bytes())}
}
pub(super) struct Compound<'a,'b,'control,'observer,'output>{encoder:&'a mut Encoder<'b,'control,'observer,'output>,first:bool,end:&'static[u8]}
impl Compound<'_,'_,'_,'_,'_>{fn separator(&mut self)->Result<(),Refusal>{if self.first{self.first=false;Ok(())}else{self.encoder.raw(b",")}}fn item<T:Serialize+?Sized>(&mut self,value:&T)->Result<(),Refusal>{self.separator()?;value.serialize(&mut*self.encoder)}fn finish(self)->Result<(),Refusal>{self.encoder.raw(self.end)}}
macro_rules! integers{($($name:ident:$ty:ty),*)=>{$(fn $name(self,value:$ty)->Result<(),Refusal>{self.number(value)})*};}
impl<'a,'b,'control,'observer,'output> ser::Serializer for &'a mut Encoder<'b,'control,'observer,'output>{
 type Ok=();type Error=Refusal;type SerializeSeq=Compound<'a,'b,'control,'observer,'output>;type SerializeTuple=Self::SerializeSeq;type SerializeTupleStruct=Self::SerializeSeq;type SerializeTupleVariant=Self::SerializeSeq;type SerializeMap=Self::SerializeSeq;type SerializeStruct=Self::SerializeSeq;type SerializeStructVariant=Self::SerializeSeq;
 integers!(serialize_i8:i8,serialize_i16:i16,serialize_i32:i32,serialize_i64:i64,serialize_i128:i128,serialize_u8:u8,serialize_u16:u16,serialize_u32:u32,serialize_u64:u64,serialize_u128:u128);
 fn serialize_bool(self,value:bool)->Result<(),Refusal>{if self.key{self.string(if value{"true"}else{"false"})}else{self.raw(if value{b"true"}else{b"false"})}}
 fn serialize_f32(self,value:f32)->Result<(),Refusal>{if self.key{return Err(invalid());}let mut scalar=Scalar::new();semio_framework_pack_json::write_float32_to(value,&mut scalar).map_err(|_|invalid())?;self.raw(scalar.text().as_bytes())}
 fn serialize_f64(self,value:f64)->Result<(),Refusal>{self.float(value)}
 fn serialize_char(self,value:char)->Result<(),Refusal>{self.string(value.encode_utf8(&mut[0;4]))}
 fn serialize_str(self,value:&str)->Result<(),Refusal>{self.string(value)}
 fn serialize_bytes(self,value:&[u8])->Result<(),Refusal>{if self.key{return Err(invalid());}self.raw(b"[")?;for(index,byte)in value.iter().enumerate(){if index!=0{self.raw(b",")?;}self.number(byte)?;}self.raw(b"]")}
 fn serialize_none(self)->Result<(),Refusal>{self.serialize_unit()}
 fn serialize_some<T:Serialize+?Sized>(self,value:&T)->Result<(),Refusal>{value.serialize(self)}
 fn serialize_unit(self)->Result<(),Refusal>{if self.key{return Err(invalid());}self.raw(b"null")}
 fn serialize_unit_struct(self,_:&'static str)->Result<(),Refusal>{self.serialize_unit()}
 fn serialize_unit_variant(self,_:&'static str,_:u32,name:&'static str)->Result<(),Refusal>{self.string(name)}
 fn serialize_newtype_struct<T:Serialize+?Sized>(self,_:&'static str,value:&T)->Result<(),Refusal>{value.serialize(self)}
 fn serialize_newtype_variant<T:Serialize+?Sized>(self,_:&'static str,_:u32,name:&'static str,value:&T)->Result<(),Refusal>{if self.key{return Err(invalid());}self.raw(b"{")?;self.string(name)?;self.raw(b":")?;value.serialize(&mut*self)?;self.raw(b"}")}
 fn serialize_seq(self,_:Option<usize>)->Result<Self::SerializeSeq,Refusal>{if self.key{return Err(invalid());}self.raw(b"[")?;Ok(Compound{encoder:self,first:true,end:b"]"})}
 fn serialize_tuple(self,length:usize)->Result<Self::SerializeTuple,Refusal>{self.serialize_seq(Some(length))}
 fn serialize_tuple_struct(self,_:&'static str,length:usize)->Result<Self::SerializeTupleStruct,Refusal>{self.serialize_seq(Some(length))}
 fn serialize_tuple_variant(self,_:&'static str,_:u32,name:&'static str,_:usize)->Result<Self::SerializeTupleVariant,Refusal>{if self.key{return Err(invalid());}self.raw(b"{")?;self.string(name)?;self.raw(b":[")?;Ok(Compound{encoder:self,first:true,end:b"]}"})}
 fn serialize_map(self,_:Option<usize>)->Result<Self::SerializeMap,Refusal>{if self.key{return Err(invalid());}self.raw(b"{")?;Ok(Compound{encoder:self,first:true,end:b"}"})}
 fn serialize_struct(self,_:&'static str,length:usize)->Result<Self::SerializeStruct,Refusal>{self.serialize_map(Some(length))}
 fn serialize_struct_variant(self,_:&'static str,_:u32,name:&'static str,_:usize)->Result<Self::SerializeStructVariant,Refusal>{if self.key{return Err(invalid());}self.raw(b"{")?;self.string(name)?;self.raw(b":{")?;Ok(Compound{encoder:self,first:true,end:b"}}"})}
 fn collect_str<T:std::fmt::Display+?Sized>(self,value:&T)->Result<(),Refusal>{self.raw(b"\"")?;let mut formatter=TextFormatter{encoder:self,error:None};let result=write!(formatter,"{value}");if let Some(error)=formatter.error{return Err(error);}result.map_err(|_|invalid())?;self.raw(b"\"")}
}
macro_rules! sequence{($trait:ident,$method:ident)=>{impl ser::$trait for Compound<'_,'_,'_,'_,'_>{type Ok=();type Error=Refusal;fn $method<T:Serialize+?Sized>(&mut self,value:&T)->Result<(),Refusal>{self.item(value)}fn end(self)->Result<(),Refusal>{self.finish()}}};}
sequence!(SerializeSeq,serialize_element);sequence!(SerializeTuple,serialize_element);sequence!(SerializeTupleStruct,serialize_field);sequence!(SerializeTupleVariant,serialize_field);
impl ser::SerializeMap for Compound<'_,'_,'_,'_,'_>{type Ok=();type Error=Refusal;fn serialize_key<T:Serialize+?Sized>(&mut self,key:&T)->Result<(),Refusal>{self.separator()?;self.encoder.key=true;let result=key.serialize(&mut*self.encoder);self.encoder.key=false;result?;self.encoder.raw(b":")}fn serialize_value<T:Serialize+?Sized>(&mut self,value:&T)->Result<(),Refusal>{value.serialize(&mut*self.encoder)}fn end(self)->Result<(),Refusal>{self.finish()}}
macro_rules! structure{($trait:ident)=>{impl ser::$trait for Compound<'_,'_,'_,'_,'_>{type Ok=();type Error=Refusal;fn serialize_field<T:Serialize+?Sized>(&mut self,key:&'static str,value:&T)->Result<(),Refusal>{self.separator()?;self.encoder.string(key)?;self.encoder.raw(b":")?;value.serialize(&mut*self.encoder)}fn end(self)->Result<(),Refusal>{self.finish()}}};}
structure!(SerializeStruct);structure!(SerializeStructVariant);

struct TextFormatter<'a,'b,'control,'observer,'output>{encoder:&'a mut Encoder<'b,'control,'observer,'output>,error:Option<Refusal>}
impl Write for TextFormatter<'_ ,'_ ,'_ ,'_ ,'_>{fn write_str(&mut self,text:&str)->std::fmt::Result{if self.error.is_some(){return Err(std::fmt::Error);}self.encoder.contents(text).map_err(|error|{self.error=Some(error);std::fmt::Error})}}
