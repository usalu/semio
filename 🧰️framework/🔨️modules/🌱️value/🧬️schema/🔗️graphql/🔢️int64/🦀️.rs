//! 🧮️ Canonical GraphQL Int64 coercion uses only owned firstparty value and literal types.
use crate::DslValue;
/// ⛔️ Refuses a scalar category, literal category, decimal spelling or signed range.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Int64Error { Type, Literal, Decimal, Range }
fn decimal(text:&str)->Result<i64,Int64Error>{
 let bytes=text.as_bytes();
 if bytes==b"0"{return Ok(0);}
 let digits=if bytes.first()==Some(&b'-'){&bytes[1..]}else{bytes};
 if digits.is_empty()||digits.len()>19||!(b'1'..=b'9').contains(&digits[0])||!digits.iter().all(u8::is_ascii_digit){return Err(Int64Error::Decimal);}
 text.parse().map_err(|_|Int64Error::Range)
}
/// ➖️ Admits exactly the canonical decimal variable transport owned by the Value schema.
pub fn parse_int64_variable(value:&DslValue)->Result<i64,Int64Error>{
 match value{DslValue::String(text)=>decimal(text),_=>Err(Int64Error::Type)}
}
/// 🧾️ Preserves complete signed words in integral and quoted literal descriptors.
pub fn parse_int64_literal(kind:&str,value:Option<&str>)->Result<i64,Int64Error>{
 if kind!="IntValue"&&kind!="StringValue"{return Err(Int64Error::Literal);}
 decimal(value.ok_or(Int64Error::Type)?)
}
/// 📤️ Emits a complete intrinsic signed word through canonical decimal text.
pub fn serialize_int64(value:i64)->String{value.to_string()}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
