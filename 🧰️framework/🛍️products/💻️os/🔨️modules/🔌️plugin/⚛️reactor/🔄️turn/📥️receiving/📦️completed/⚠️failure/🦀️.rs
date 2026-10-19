//! 🧳️ Original parser refusals keep their complete native fields through the existing paid domain frontier.
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_framework_job::{OriginalDomainFailure, OriginalDomainProjection, OriginalDomainView as V};
use semio_framework_pack_json::JsonError;
use pack::PackRefusal;
use pack::PackRetryDisposition;
use pack::PackTransportCategory;
#[derive(Debug,semio_framework_value::RetireOwned)]
pub(super) enum DecodeFailure{Json(JsonError,RetainedCloneProgress),Pack(PackRefusal,RetainedCloneProgress)}
impl OriginalDomainFailure for DecodeFailure{
 fn domain_code(&self)->&str{"plugin.completed.decode"}
 fn domain_message(&self)->&str{"original Completed input failed bounded decoding"}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match self{Self::Json(_,progress)|Self::Pack(_,progress)=>*progress}}
}
#[doc(hidden)]
#[derive(Clone,Copy)]
pub enum Node<'a>{Failure(&'a DecodeFailure),Json(&'a JsonError),Pack(&'a PackRefusal),Value(&'a ValueError),TextError(&'a semio_framework_diagnostic::TextError),Progress(RetainedCloneProgress),Span(semio_framework_diagnostic::TextSpan),Text(&'a str),UInt(u64)}
#[doc(hidden)]
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original decoder failure projection ordinal changed")}
impl<'a> Node<'a>{
 pub fn fields(self)->[Option<(&'a str,Self)>;8]{
  let mut fields=[None;8];
  let mut add=|key:&'a str,value:Self|{let slot=fields.iter_mut().find(|slot|slot.is_none()).unwrap();*slot=Some((key,value))};
  match self{
   Self::Failure(error)=>{add("format",Self::Text(match error{DecodeFailure::Json(..)=>"json",DecodeFailure::Pack(..)=>"pack"}));add("original",match error{DecodeFailure::Json(error,_)=>Self::Json(error),DecodeFailure::Pack(error,_)=>Self::Pack(error)});add("retainedProgress",Self::Progress(error.domain_retained_progress()))},
   Self::Json(error)=>{add("kind",Self::Text(match error{JsonError::Native(_)=>"native",JsonError::UnexpectedEof=>"unexpectedEof",JsonError::UnexpectedByte{..}=>"unexpectedByte",JsonError::InvalidNumber(_)=>"invalidNumber",JsonError::InvalidEscape(_)=>"invalidEscape",JsonError::InvalidUnicodeEscape(_)=>"invalidUnicodeEscape",JsonError::UnpairedSurrogate(_)=>"unpairedSurrogate",JsonError::ControlCharacterInString{..}=>"controlCharacterInString",JsonError::InvalidUtf8=>"invalidUtf8",JsonError::TrailingData(_)=>"trailingData",JsonError::MaxDepthExceeded(_)=>"maxDepthExceeded",JsonError::DuplicateMember{..}=>"duplicateMember"}));match error{JsonError::Native(error)=>add("original",Self::Value(error)),JsonError::UnexpectedByte{found,offset}|JsonError::ControlCharacterInString{byte:found,offset}=>{add("byte",Self::UInt(u64::from(*found)));add("offset",Self::UInt(*offset as u64))},JsonError::InvalidNumber(offset)|JsonError::InvalidEscape(offset)|JsonError::InvalidUnicodeEscape(offset)|JsonError::UnpairedSurrogate(offset)|JsonError::TrailingData(offset)=>add("offset",Self::UInt(*offset as u64)),JsonError::MaxDepthExceeded(depth)=>add("depth",Self::UInt(u64::from(*depth))),JsonError::DuplicateMember{name,offset}=>{add("name",Self::Text(name));add("offset",Self::UInt(*offset as u64))},_=>{}}},
   Self::Pack(error)=>{add("kind",Self::Text(error.kind().as_str()));match error{
    PackRefusal::BadMagic=>add("variant",Self::Text("badMagic")),PackRefusal::ContentHashMismatch=>add("variant",Self::Text("contentHashMismatch")),
    PackRefusal::UnsupportedVersion{major,minor}=>{add("variant",Self::Text("unsupportedVersion"));add("major",Self::UInt(u64::from(*major)));add("minor",Self::UInt(u64::from(*minor)))},
    PackRefusal::UnknownRequiredFlags(flags)=>{add("variant",Self::Text("unknownRequiredFlags"));add("flags",Self::UInt(u64::from(*flags)))},
    PackRefusal::Truncated(offset)=>{add("variant",Self::Text("truncated"));add("offset",Self::UInt(*offset))},
    PackRefusal::ChecksumMismatch{segment,offset}=>{add("variant",Self::Text("checksumMismatch"));add("segment",Self::Text(segment));add("offset",Self::UInt(*offset))},
    PackRefusal::LimitExceeded{limit,..}=>{add("variant",Self::Text("limitExceeded"));add("limit",Self::Text(limit))},
    PackRefusal::RetainedMalformed{what,offset,..}|PackRefusal::Malformed{what,offset,..}=>{add("variant",Self::Text(if matches!(error,PackRefusal::RetainedMalformed{..}){"retainedMalformed"}else{"malformed"}));add("what",Self::Text(what));add("offset",Self::UInt(*offset));let detail=match error{PackRefusal::RetainedMalformed{detail,..}=>*detail,PackRefusal::Malformed{detail,..}=>detail.as_str(),_=>unreachable!()};add("detail",Self::Text(detail))},
    PackRefusal::RetainedAllocation{allocated_bytes,what,offset,detail,..}=>{add("variant",Self::Text("retainedAllocation"));add("allocatedBytes",Self::UInt(*allocated_bytes as u64));add("what",Self::Text(what));add("offset",Self::UInt(*offset));add("detail",Self::Text(detail))},
    PackRefusal::NonCanonical(reason)=>{add("variant",Self::Text("nonCanonical"));add("reason",Self::Text(reason))},PackRefusal::UnsupportedCodec(codec)=>{add("variant",Self::Text("unsupportedCodec"));add("codec",Self::UInt(u64::from(*codec)))},
    PackRefusal::ValueRefusal(error)=>{add("variant",Self::Text("valueRefusal"));add("original",Self::Value(error))},PackRefusal::TextRefusal(error)=>{add("variant",Self::Text("textRefusal"));add("original",Self::TextError(error))},
    PackRefusal::Io{error,retry}=>{add("variant",Self::Text("io"));add("original",Self::Value(error));add("retry",Self::Text(match retry{PackRetryDisposition::Never=>"never",PackRetryDisposition::Transient=>"transient"}))},
    PackRefusal::TransportAdmission{category,refusal}=>{add("variant",Self::Text("transportAdmission"));add("category",Self::Text(match category{PackTransportCategory::NativeIo=>"nativeIo",PackTransportCategory::HttpRequest=>"httpRequest",PackTransportCategory::HttpBody=>"httpBody"}));add("refusalKind",Self::Text(refusal.kind.as_str()));add("reason",Self::Text(refusal.reason));add("allocatedBytes",Self::UInt(refusal.allocated_bytes as u64));add("physicalBytes",Self::UInt(refusal.physical_bytes as u64))},
   }},
   Self::Progress(p)=>for(key,value)in[("copiedItems",p.copied_items),("copiedBytes",p.copied_bytes),("retainedCapacityBytes",p.retained_capacity_bytes),("releasedBytes",p.released_bytes)]{add(key,Self::UInt(value as u64))},
   Self::Span(s)=>for(key,value)in[("line",s.line),("column",s.column),("length",s.length)]{add(key,Self::UInt(u64::from(value)))},
   Self::TextError(error)=>{add("kind",Self::Text(error.kind.as_str()));add("message",Self::Text(&error.message));add("span",Self::Span(error.span));if let Some(expected)=error.expected.as_ref(){add("expected",Self::Text(expected))}},
   _=>{},
  }fields
 }
 pub fn locate(mut self,path:&[usize])->Result<(Self,usize),ValueError>{for(position,index)in path.iter().enumerate(){if matches!(self,Self::Value(_)){return Ok((self,position))}self=self.fields().into_iter().flatten().nth(*index).map(|(_,value)|value).ok_or_else(invalid)?}Ok((self,path.len()))}
}
impl OriginalDomainProjection for DecodeFailure{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{let(node,position)=Node::Failure(self).locate(path)?;Ok(match node{Node::Value(error)=>return semio_framework_job::project_original_value_error(error,&path[position..]),Node::Text(text)=>V::IntrinsicText(text),Node::UInt(value)=>V::IntrinsicNumber(semio_framework_value::Number::UInt(value)),node=>V::IntrinsicObject(node.fields().into_iter().flatten().count())})}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let(node,position)=Node::Failure(self).locate(path)?;if let Node::Value(error)=node{return semio_framework_job::key_original_value_error(error,&path[position..],index)}node.fields().into_iter().flatten().nth(index).map(|(key,_)|key).ok_or_else(invalid)}
}
