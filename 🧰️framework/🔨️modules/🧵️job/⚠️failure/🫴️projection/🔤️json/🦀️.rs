//! 🔤️ Complete borrowed JSON syntax failures preserve original names, offsets and nested native causes.
use super::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,ValueError};
use semio_framework_pack_json::JsonError;
use semio_framework_value_retained_clone::{Number,ValueRefusalKind,RetainedCloneProgress};
#[derive(Clone,Copy)]enum Node<'a>{Json(&'a JsonError),Value(&'a ValueError),Text(&'a str),UInt(u64)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original JSON failure projection ordinal changed")}
fn json_kind(value:&JsonError)->&'static str{match value{JsonError::Native(_)=>"Native",JsonError::UnexpectedEof=>"UnexpectedEof",JsonError::UnexpectedByte{..}=>"UnexpectedByte",JsonError::InvalidNumber(_)=>"InvalidNumber",JsonError::InvalidEscape(_)=>"InvalidEscape",JsonError::InvalidUnicodeEscape(_)=>"InvalidUnicodeEscape",JsonError::UnpairedSurrogate(_)=>"UnpairedSurrogate",JsonError::ControlCharacterInString{..}=>"ControlCharacterInString",JsonError::InvalidUtf8=>"InvalidUtf8",JsonError::TrailingData(_)=>"TrailingData",JsonError::MaxDepthExceeded(_)=>"MaxDepthExceeded",JsonError::DuplicateMember{..}=>"DuplicateMember"}}
impl<'a> Node<'a>{
 fn keys(self)->Result<&'static[&'static str],ValueError>{let Self::Json(j)=self else{return Err(invalid())};Ok(match j{JsonError::Native(_)=>&["kind","original"],JsonError::UnexpectedEof|JsonError::InvalidUtf8=>&["kind"],JsonError::UnexpectedByte{..}=>&["kind","found","offset"],JsonError::ControlCharacterInString{..}=>&["kind","byte","offset"],JsonError::DuplicateMember{..}=>&["kind","name","offset"],JsonError::MaxDepthExceeded(_)=>&["kind","depth"],_=>&["kind","offset"]})}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
 Self::Json(v)=>if index==0{Self::Text(json_kind(v))}else{match(index,v){(1,JsonError::Native(v))=>Self::Value(v),(1,JsonError::UnexpectedByte{found,..})|(1,JsonError::ControlCharacterInString{byte:found,..})=>Self::UInt(u64::from(*found)),(2,JsonError::UnexpectedByte{offset,..})|(2,JsonError::ControlCharacterInString{offset,..})|(2,JsonError::DuplicateMember{offset,..})=>Self::UInt(*offset as u64),(1,JsonError::InvalidNumber(v))|(1,JsonError::InvalidEscape(v))|(1,JsonError::InvalidUnicodeEscape(v))|(1,JsonError::UnpairedSurrogate(v))|(1,JsonError::TrailingData(v))=>Self::UInt(*v as u64),(1,JsonError::MaxDepthExceeded(v))=>Self::UInt(u64::from(*v)),(1,JsonError::DuplicateMember{name,..})=>Self::Text(name),_=>return Err(invalid())}},
 _=>return Err(invalid())})}
 fn view(self)->Result<V<'a>,ValueError>{Ok(match self{Self::Json(_)=>V::IntrinsicObject(self.keys()?.len()),Self::Value(v)=>return super::project_original_value_error(v,&[]),Self::Text(v)=>V::IntrinsicText(v),Self::UInt(v)=>V::IntrinsicNumber(Number::UInt(v))})}
}
fn locate<'a>(mut node:Node<'a>,path:&[usize])->Result<(Node<'a>,usize),ValueError>{for(position,&index)in path.iter().enumerate(){if matches!(node,Node::Value(_)){return Ok((node,position))}node=node.child(index)?}Ok((node,path.len()))}
/// 🧾️ Lends every original JSON syntax field without rendering or copying its owned name.
pub fn project_original_json_failure<'a>(error:&'a JsonError,path:&[usize])->Result<V<'a>,ValueError>{let(node,position)=locate(Node::Json(error),path)?;match node{Node::Value(v)=>super::project_original_value_error(v,&path[position..]),node=>node.view()}}
/// 🔑️ Lends each complete original syntax field key at its selected original path.
pub fn key_original_json_failure<'a>(error:&'a JsonError,path:&[usize],index:usize)->Result<&'a str,ValueError>{let(node,position)=locate(Node::Json(error),path)?;match node{Node::Value(v)=>super::key_original_value_error(v,&path[position..],index),node=>node.keys()?.get(index).copied().ok_or_else(invalid)}}
/// 🫴️ Owns the unchanged syntax failure through the common Source-qualified domain receiving port.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub struct OriginalJsonFailure{pub original:JsonError}
impl From<JsonError> for OriginalJsonFailure{fn from(original:JsonError)->Self{Self{original}}}
impl OriginalDomainFailure for OriginalJsonFailure{
 fn domain_code(&self)->&str{json_kind(&self.original)}
 fn domain_message(&self)->&str{match &self.original{JsonError::Native(v)=>&v.message,_=>json_kind(&self.original)}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match &self.original{JsonError::Native(v)=>v.retained_progress(),_=>Default::default()}}
}
impl OriginalDomainProjection for OriginalJsonFailure{
 fn projection_identity(&self)->usize{&self.original as*const JsonError as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{project_original_json_failure(&self.original,path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{key_original_json_failure(&self.original,path,index)}
}
