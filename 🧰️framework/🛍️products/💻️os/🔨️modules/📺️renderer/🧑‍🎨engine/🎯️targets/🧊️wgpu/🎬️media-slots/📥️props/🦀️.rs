//! 🎟️ Original accepted params feed the existing paid grammar without a second wire or semantic clone.
use semio_framework_value::{DslValue, RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_pack_json::{JsonGrammarCursor,JsonReadLimits,JsonMemberPolicy,JsonError};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V};
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct PresentedParamsFailure{pub cause:JsonError,pub receipt_refusal:Option<ValueError>,pub progress:RetainedCloneProgress}
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum PresentedParamsStepFailure{Grammar(PresentedParamsFailure),Original(OriginalTurnFailure)}
impl From<ValueError> for PresentedParamsStepFailure{fn from(source:ValueError)->Self{Self::Original(source.into())}}

pub struct OriginalPresentedParams{source:usize,length:usize,identity:(semio_framework_job::OperationId,semio_framework_job::Generation),clock:fn()->Option<u64>,deadline:u64,ledger:Option<u64>,parser:ControlledRetirement<JsonGrammarCursor<DslValue>>,closing:bool}
impl OriginalPresentedParams{
 pub fn admit(source:&str,limits:JsonReadLimits,cx:&mut StepContext<'_>)->Result<Option<Self>,PresentedParamsStepFailure>{
  let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  let original=semio_framework_pack_json::JsonSourceCursor::<_,DslValue>::new(source,JsonMemberPolicy::Reject,limits)?;let(_,parser)=original.into_grammar();
  let parser=ControlledRetirement::new(parser).unwrap_or_else(|_|unreachable!("first-party JSON grammar supports controlled retirement"));
  cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(Some(Self{source:source.as_ptr()as usize,length:source.len(),identity:(cx.operation(),cx.generation()),clock:cx.clock_source(),deadline:cx.deadline_us(),ledger:None,parser,closing:false}))
 }
 pub fn advance(&mut self,source:&str,output:&mut Option<DslValue>,native:&mut NativeDecodeControl<'_>,cx:&mut StepContext<'_>)->Result<bool,PresentedParamsStepFailure>{
  self.check_caller(cx)?;
  if self.closing||(self.source,self.length)!=(source.as_ptr()as usize,source.len())||self.ledger.is_some_and(|original|original!=native.ledger_identity()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"accepted media params source differs from original caller loan").into())}
  if output.is_some(){return Ok(true)}
  let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0{return Ok(false)}
  let parser=self.parser.original_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"accepted media params grammar has entered retirement"))?;
  let demand=parser.normal_step_demands(source).map_err(|cause|PresentedParamsStepFailure::Grammar(PresentedParamsFailure{cause,receipt_refusal:None,progress:Default::default()}))?;
  if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(false)}
  self.ledger=Some(native.ledger_identity());let result=parser.step_source(source,1,native,grant,true);let complete=match result{Ok(value)=>{*output=value;Ok(output.is_some())},Err(cause)=>Err(cause)};let progress=parser.normal_step_progress();let receipt_refusal=cx.consume_retained(progress).err();cx.consume_fuel(1);
  match(complete,receipt_refusal){(Ok(ready),None)=>Ok(ready),(Err(cause),receipt_refusal)=>Err(PresentedParamsStepFailure::Grammar(PresentedParamsFailure{cause,receipt_refusal,progress})),(Ok(_),Some(refusal))=>Err(PresentedParamsStepFailure::Original(refusal.into()))}
 }
 fn check_caller(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if self.identity!=(cx.operation(),cx.generation())||!std::ptr::fn_addr_eq(self.clock,cx.clock_source())||self.deadline!=cx.deadline_us(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"accepted media params require their initiating caller clock and deadline"))}Ok(())}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.check_caller(cx)?;self.closing=true;if self.parser.terminal_is_empty(){return Ok(true)}let demand=self.parser.selected_demand(cx.retained_grant().maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.parser.step(grant))?;Ok(self.parser.terminal_is_empty())}
 pub fn terminal_is_empty(&self)->bool{self.parser.terminal_is_empty()}
}

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original accepted params failure projection changed")}
fn number(value:usize)->V<'static>{V::IntrinsicNumber(semio_framework_value::Number::UInt(value as u64))}
fn json_kind(cause:&JsonError)->&'static str{match cause{JsonError::Native(_)=>"native",JsonError::UnexpectedEof=>"unexpectedEof",JsonError::UnexpectedByte{..}=>"unexpectedByte",JsonError::InvalidNumber(_)=>"invalidNumber",JsonError::InvalidEscape(_)=>"invalidEscape",JsonError::InvalidUnicodeEscape(_)=>"invalidUnicodeEscape",JsonError::UnpairedSurrogate(_)=>"unpairedSurrogate",JsonError::ControlCharacterInString{..}=>"controlCharacterInString",JsonError::InvalidUtf8=>"invalidUtf8",JsonError::TrailingData(_)=>"trailingData",JsonError::MaxDepthExceeded(_)=>"maxDepthExceeded",JsonError::DuplicateMember{..}=>"duplicateMember"}}
impl OriginalDomainFailure for PresentedParamsFailure{
 fn domain_code(&self)->&str{"media.params-json"}
 fn domain_message(&self)->&str{match &self.cause{JsonError::Native(source)=>source.message.as_ref(),JsonError::DuplicateMember{name,..}=>name,_=>json_kind(&self.cause)}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.progress}
}
impl OriginalDomainProjection for PresentedParamsFailure{
 fn projection_identity(&self)->usize{&self.cause as*const JsonError as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{Ok(match path{
  []=>V::IntrinsicObject(3),[0]=>V::IntrinsicObject(6),[0,0]=>V::IntrinsicText(json_kind(&self.cause)),
  [0,1]=>match &self.cause{JsonError::UnexpectedByte{offset,..}|JsonError::ControlCharacterInString{offset,..}|JsonError::DuplicateMember{offset,..}|JsonError::InvalidNumber(offset)|JsonError::InvalidEscape(offset)|JsonError::InvalidUnicodeEscape(offset)|JsonError::UnpairedSurrogate(offset)|JsonError::TrailingData(offset)=>number(*offset),_=>V::IntrinsicNull},
  [0,2]=>match &self.cause{JsonError::UnexpectedByte{found,..}|JsonError::ControlCharacterInString{byte:found,..}=>number(*found as usize),_=>V::IntrinsicNull},
  [0,3]=>match &self.cause{JsonError::MaxDepthExceeded(depth)=>number(*depth as usize),_=>V::IntrinsicNull},
  [0,4]=>match &self.cause{JsonError::DuplicateMember{name,..}=>V::IntrinsicText(name),_=>V::IntrinsicNull},
  [0,5,rest @ ..]=>match &self.cause{JsonError::Native(source)=>return semio_framework_job::project_original_value_error(source,rest),_ if rest.is_empty()=>V::IntrinsicNull,_=>return Err(invalid())},
  [1,rest @ ..]=>match &self.receipt_refusal{Some(source)=>return semio_framework_job::project_original_value_error(source,rest),None if rest.is_empty()=>V::IntrinsicNull,_=>return Err(invalid())},
  [2]=>V::IntrinsicObject(4),[2,index]=>number(match index{0=>self.progress.copied_items,1=>self.progress.copied_bytes,2=>self.progress.retained_capacity_bytes,3=>self.progress.released_bytes,_=>return Err(invalid())}),_=>return Err(invalid())})}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{[]=>["cause","receiptRefusal","retainedProgress"].get(index).copied().ok_or_else(invalid),[0]=>["kind","offset","byte","depth","name","native"].get(index).copied().ok_or_else(invalid),[0,5,rest @ ..]=>match &self.cause{JsonError::Native(source)=>semio_framework_job::key_original_value_error(source,rest,index),_=>Err(invalid())},[1,rest @ ..]=>self.receipt_refusal.as_ref().ok_or_else(invalid).and_then(|source|semio_framework_job::key_original_value_error(source,rest,index)),[2]=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(invalid),_=>Err(invalid())}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
