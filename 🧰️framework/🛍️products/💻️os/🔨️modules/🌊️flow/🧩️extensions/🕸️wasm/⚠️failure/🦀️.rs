//! 🧯️ Complete SDK failures retain their exact native variant and owned source through domain admission.
use super::*;
use semio_framework_job::{OriginalDomainProjection,OriginalDomainFailure,OriginalDomainView as View};
use semio_framework_pack_json::JsonError;
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original SDK failure projection path is absent")}
fn json_kind(value:&JsonError)->&'static str{match value{JsonError::Native(_)=>"native",JsonError::UnexpectedEof=>"unexpectedEof",JsonError::UnexpectedByte{..}=>"unexpectedByte",JsonError::InvalidNumber(_)=>"invalidNumber",JsonError::InvalidEscape(_)=>"invalidEscape",JsonError::InvalidUnicodeEscape(_)=>"invalidUnicodeEscape",JsonError::UnpairedSurrogate(_)=>"unpairedSurrogate",JsonError::ControlCharacterInString{..}=>"controlCharacterInString",JsonError::InvalidUtf8=>"invalidUtf8",JsonError::TrailingData(_)=>"trailingData",JsonError::MaxDepthExceeded(_)=>"maxDepthExceeded",JsonError::DuplicateMember{..}=>"duplicateMember"}}
fn json_fields(value:&JsonError)->usize{match value{JsonError::Native(_)=>2,JsonError::UnexpectedEof|JsonError::InvalidUtf8=>1,JsonError::UnexpectedByte{..}|JsonError::ControlCharacterInString{..}|JsonError::DuplicateMember{..}=>3,_=>2}}
fn json_view<'a>(value:&'a JsonError,path:&[usize])->Result<View<'a>,ValueError>{
 if path.is_empty(){return Ok(View::IntrinsicObject(json_fields(value)))}
 if path==[0]{return Ok(View::IntrinsicText(json_kind(value)))}
 if let JsonError::Native(original)=value{if path[0]==1{return semio_framework_job::project_original_value_error(original,&path[1..])}}
 if path.len()!=1{return Err(absent())}let uint=|value|Ok(View::IntrinsicNumber(semio_framework_value::Number::UInt(value)));
 match(value,path[0]){(JsonError::UnexpectedByte{found,..},1)|(JsonError::ControlCharacterInString{byte:found,..},1)=>uint(u64::from(*found)),(JsonError::UnexpectedByte{offset,..},2)|(JsonError::ControlCharacterInString{offset,..},2)|(JsonError::DuplicateMember{offset,..},2)=>uint(*offset as u64),(JsonError::DuplicateMember{name,..},1)=>Ok(View::IntrinsicText(name)),(JsonError::InvalidNumber(offset)|JsonError::InvalidEscape(offset)|JsonError::InvalidUnicodeEscape(offset)|JsonError::UnpairedSurrogate(offset)|JsonError::TrailingData(offset),1)=>uint(*offset as u64),(JsonError::MaxDepthExceeded(depth),1)=>uint(u64::from(*depth)),_=>Err(absent())}
}
fn json_key<'a>(value:&'a JsonError,path:&[usize],index:usize)->Result<&'a str,ValueError>{
 if path.is_empty(){if index==0{return Ok("kind")}return match(value,index){(JsonError::Native(_),1)=>Ok("original"),(JsonError::UnexpectedByte{..},1)=>Ok("found"),(JsonError::ControlCharacterInString{..},1)=>Ok("byte"),(JsonError::DuplicateMember{..},1)=>Ok("name"),(JsonError::MaxDepthExceeded(_),1)=>Ok("depth"),(JsonError::UnexpectedByte{..}|JsonError::ControlCharacterInString{..}|JsonError::DuplicateMember{..},2)=>Ok("offset"),(JsonError::InvalidNumber(_)|JsonError::InvalidEscape(_)|JsonError::InvalidUnicodeEscape(_)|JsonError::UnpairedSurrogate(_)|JsonError::TrailingData(_),1)=>Ok("offset"),_=>Err(absent())}}
 if let JsonError::Native(original)=value{if path[0]==1{return semio_framework_job::key_original_value_error(original,&path[1..],index)}}Err(absent())
}
impl EvaluationFailure{
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Pack(value)=>value.reason.retained_progress(),Self::Native(value)|Self::Json(JsonError::Native(value))=>value.retained_progress(),Self::Operator(value)=>value.domain_retained_progress(),Self::Work(value)=>value.retained_progress(),Self::Original(value)=>value.retained_progress(),Self::Json(_)|Self::Literal(_)=>Default::default()}}
}
impl OriginalDomainProjection for EvaluationFailure{
 fn projection_identity(&self)->usize{match self{Self::Pack(value)=>value.reason.message.as_ptr()as usize,Self::Native(value)|Self::Json(JsonError::Native(value))=>value.message.as_ptr()as usize,Self::Operator(value)=>value.projection_identity(),Self::Work(value)=>value.projection_identity(),Self::Original(value)=>value.primary.message().as_ptr()as usize,Self::Json(JsonError::DuplicateMember{name,..})=>name.as_ptr()as usize,Self::Literal(value)=>value.as_ptr()as usize,Self::Json(_)=>self as*const _ as usize}}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
  if path.is_empty(){return Ok(View::IntrinsicObject(2))}if path==[0]{return Ok(View::IntrinsicText(match self{Self::Pack(_)=>"pack",Self::Native(_)=>"native",Self::Json(_)=>"json",Self::Operator(_)=>"operator",Self::Work(_)=>"operatorWork",Self::Original(_)=>"original",Self::Literal(_)=>"literal"}))}if path[0]!=1{return Err(absent())}
  let path=&path[1..];match self{Self::Pack(value)=>{if path.is_empty(){Ok(View::IntrinsicObject(2))}else if path.first()==Some(&0){semio_framework_job::project_original_value_error(&value.reason,&path[1..])}else if path==[1]{Ok(View::IntrinsicNumber(semio_framework_value::Number::UInt(value.written_bytes as u64)))}else{Err(absent())}},Self::Native(value)=>semio_framework_job::project_original_value_error(value,path),Self::Json(value)=>json_view(value,path),Self::Operator(value)=>value.projection_view(path),Self::Work(value)=>value.projection_view(path),Self::Original(value)=>semio_framework_job::project_original_turn_failure(value,path),Self::Literal(value)if path.is_empty()=>Ok(View::IntrinsicText(value)),_=>Err(absent())}
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty(){return ["source","original"].get(index).copied().ok_or_else(absent)}if path[0]!=1{return Err(absent())}let path=&path[1..];match self{Self::Pack(value)=>{if path.is_empty(){["refusal","initializedPrefixBytes"].get(index).copied().ok_or_else(absent)}else if path.first()==Some(&0){semio_framework_job::key_original_value_error(&value.reason,&path[1..],index)}else{Err(absent())}},Self::Native(value)=>semio_framework_job::key_original_value_error(value,path,index),Self::Json(value)=>json_key(value,path,index),Self::Operator(value)=>value.projection_key(path,index),Self::Work(value)=>value.projection_key(path,index),Self::Original(value)=>semio_framework_job::key_original_turn_failure(value,path,index),Self::Literal(_)=>Err(absent())}}
}
impl OriginalDomainFailure for EvaluationFailure{
 fn domain_code(&self)->&str{match self{Self::Pack(_)=>"flowEvaluation.pack",Self::Native(_)=>"flowEvaluation.native",Self::Json(_)=>"flowEvaluation.json",Self::Operator(value)=>value.domain_code(),Self::Work(value)=>value.domain_code(),Self::Original(value)=>match &value.primary{semio_framework_job::OriginalTurnPrimary::Fault(value)=>&value.code.0,semio_framework_job::OriginalTurnPrimary::Domain(value)=>value.code(),_=>"flowEvaluation.original"},Self::Literal(_)=>"flowEvaluation.literal"}}
 fn domain_message(&self)->&str{match self{Self::Pack(value)=>&value.reason.message,Self::Native(value)|Self::Json(JsonError::Native(value))=>&value.message,Self::Operator(value)=>value.domain_message(),Self::Work(value)=>value.domain_message(),Self::Original(value)=>value.primary.message(),Self::Json(JsonError::DuplicateMember{name,..})=>name,Self::Json(value)=>json_kind(value),Self::Literal(value)=>value}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.retained_progress()}
}

/// 🪢️ The original failed producer and its complete close/receipt causes stay in one paid owner.
#[derive(Debug,semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct EvaluationCloseFailure{pub initial:Option<EvaluationFailure>,pub closure:EvaluationFailure,pub receipt_refusal:Option<ValueError>}
impl OriginalDomainFailure for EvaluationCloseFailure{
 fn domain_code(&self)->&str{"flowEvaluation.sourceClose"}
 fn domain_message(&self)->&str{self.initial.as_ref().unwrap_or(&self.closure).domain_message()}
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
impl OriginalDomainProjection for EvaluationCloseFailure{
 fn projection_identity(&self)->usize{self.initial.as_ref().unwrap_or(&self.closure).projection_identity()}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
  if path.is_empty(){return Ok(View::IntrinsicObject(1+usize::from(self.initial.is_some())+usize::from(self.receipt_refusal.is_some())))}
  let index=path[0];let rest=&path[1..];if let Some(initial)=&self.initial{if index==0{return initial.projection_view(rest)}}
  let closure=usize::from(self.initial.is_some());if index==closure{return self.closure.projection_view(rest)}
  if index==closure+1{if let Some(refusal)=&self.receipt_refusal{return semio_framework_job::project_original_value_error(refusal,rest)}}Err(absent())
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if path.is_empty(){if self.initial.is_some()&&index==0{return Ok("initial")}let closure=usize::from(self.initial.is_some());if index==closure{return Ok("closure")}if self.receipt_refusal.is_some()&&index==closure+1{return Ok("receiptRefusal")}return Err(absent())}
  let slot=path[0];let rest=&path[1..];if let Some(initial)=&self.initial{if slot==0{return initial.projection_key(rest,index)}}let closure=usize::from(self.initial.is_some());if slot==closure{return self.closure.projection_key(rest,index)}if slot==closure+1{if let Some(refusal)=&self.receipt_refusal{return semio_framework_job::key_original_value_error(refusal,rest,index)}}Err(absent())
 }
}
