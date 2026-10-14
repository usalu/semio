//! 🧠️ Pack borrows original ordered neural keys and atoms without a value or text mirror.
use super::{Dictionary,Value,Atom,Number,ValueError};
use semio_framework_job::{OriginalDomainProjection,OriginalDomainView as View};
fn absent()->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Dictionary projection path is absent")}
fn dictionary_at<'a>(mut dictionary:&'a Dictionary,path:&[usize])->Result<&'a Dictionary,ValueError>{for index in path{dictionary=dictionary.entry_at_rank(*index).and_then(|(_,value)|value.as_dictionary()).ok_or_else(absent)?;}Ok(dictionary)}
impl OriginalDomainProjection for Dictionary{
 fn projection_identity(&self)->usize{self.pairs.original_root_identity().unwrap_or(self as*const _ as usize)}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
  let Some((index,parent))=path.split_last()else{return Ok(View::IntrinsicObject(self.len()))};let dictionary=dictionary_at(self,parent)?;let value=dictionary.entry_at_rank(*index).map(|(_,value)|value).ok_or_else(absent)?;
  Ok(match value{Value::Dictionary(value)=>View::IntrinsicObject(value.len()),Value::Atom(Atom::Null)=>View::IntrinsicNull,Value::Atom(Atom::Boolean(value))=>View::IntrinsicBool(*value),Value::Atom(Atom::Integer(value))=>View::IntrinsicNumber(Number::Int(*value)),Value::Atom(Atom::Decimal(value))=>View::IntrinsicNumber(Number::Float(*value)),Value::Atom(Atom::String(value))=>View::IntrinsicText(value)})
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{dictionary_at(self,path)?.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(absent)}
}
