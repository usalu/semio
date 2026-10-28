//! 🔢️ Borrow exact Part21 decimal values without a graph or binary64 conversion.
use super::domain::{DomainValue,DomainView,TypedView,NumberView};
use semio_s_artifact_stdio_contract::part21::Part21Value;
use std::borrow::Cow;
struct DecimalView<'a>{digits:Cow<'a,str>,start:usize,end:usize,power:i128,negative:bool}
fn decimal<'a>(value:&'a Part21Value,tick:&mut dyn FnMut()->Result<(),String>)->Result<Option<DecimalView<'a>>,String>{
 let(digits,mut power,negative)=match value{Part21Value::Int(integer)=>{let text=integer.unsigned_abs().to_string();(Cow::Owned(text),0,*integer<0)},Part21Value::Real(number)=>(Cow::Borrowed(number.coefficient.as_str()),number.exponent.unwrap_or(0)as i128-number.scale as i128,number.negative),_=>return Ok(None)};
 if digits.is_empty(){return Ok(None);}for digit in digits.bytes(){tick()?;if !digit.is_ascii_digit(){return Ok(None);}}let mut start=0;let mut end=digits.len();while start<end&&digits.as_bytes()[start]==b'0'{tick()?;start+=1;}while end>start&&digits.as_bytes()[end-1]==b'0'{tick()?;end-=1;power+=1;}Ok(Some(DecimalView{digits,start,end,power,negative:start!=end&&negative}))
}
impl DomainValue for Part21Value{
 fn number(&self)->Option<NumberView<'_>>{match self{Self::Int(value)=>Some(NumberView::Integer(*value)),Self::Real(value)=>Some(NumberView::Decimal{coefficient:&value.coefficient,negative:value.negative,scale:value.scale,exponent:value.exponent}),_=>None}}
 fn view(&self)->DomainView<'_,Self>{match self{Self::Unset=>DomainView::Unset,Self::Derived=>DomainView::Derived,Self::Int(_)=>DomainView::Integer,Self::Real(_)=>DomainView::Real,Self::Ref(id)=>DomainView::Reference(*id),Self::Str(text)=>DomainView::String(text),Self::Enum(text)=>DomainView::Enum(text),Self::Binary(text)=>DomainView::Binary(text),Self::List(items)=>DomainView::Aggregate(items),Self::Typed{name,items}=>DomainView::TypedValue(TypedView{name,items})}}
 fn numeric_valid(&self,tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>{Ok(decimal(self,tick)?.is_some())}
 fn numbers_equal(&self,other:&Self,tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>{let(Some(a),Some(b))=(decimal(self,tick)?,decimal(other,tick)?)else{return Ok(false)};if a.start==a.end&&b.start==b.end{return Ok(true);}if a.negative!=b.negative||a.power!=b.power||a.end-a.start!=b.end-b.start{return Ok(false);}for index in 0..a.end-a.start{tick()?;if a.digits.as_bytes()[a.start+index]!=b.digits.as_bytes()[b.start+index]{return Ok(false);}}Ok(true)}
 fn numbers_identical(&self,other:&Self,tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>{Ok(match(self,other){(Self::Int(a),Self::Int(b))=>a==b,(Self::Real(a),Self::Real(b))=>{if a.negative!=b.negative||a.scale!=b.scale||a.exponent!=b.exponent||a.coefficient.len()!=b.coefficient.len(){return Ok(false);}for(a,b)in a.coefficient.bytes().zip(b.coefficient.bytes()){tick()?;if a!=b{return Ok(false);}}true},_=>false})}
}
