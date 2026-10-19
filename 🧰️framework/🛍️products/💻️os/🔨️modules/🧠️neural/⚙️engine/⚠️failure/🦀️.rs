//! 🧳️ Original neural failures lend complete native variants without formatted diagnostic mirrors.
use super::{EvalError,OperatorWorkFailure,RetainedCloneProgress,ValueError};
use semio_framework_job::{OriginalDomainProjection,OriginalDomainView as View,OriginalDomainFailure};
fn absent()->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original neural failure projection path is absent")}
impl EvalError{
 fn original_variant(&self)->&'static str{match self{Self::Retained(_)=>"retained",Self::UnknownKind(_)=>"unknownKind",Self::MissingInput(_)=>"missingInput",Self::InvalidInput(_)=>"invalidInput",Self::CardinalityViolation(_)=>"cardinalityViolation",Self::HeterogeneousList(_)=>"heterogeneousList",Self::CycleDetected=>"cycleDetected",Self::PendingExtension{..}=>"pendingExtension"}}
 fn original_text(&self)->Option<&str>{match self{Self::UnknownKind(value)|Self::MissingInput(value)|Self::InvalidInput(value)|Self::CardinalityViolation(value)|Self::HeterogeneousList(value)=>Some(value),_=>None}}
}
impl OriginalDomainProjection for EvalError{
 fn projection_identity(&self)->usize{match self{Self::Retained(value)=>value.message.as_ptr()as usize,Self::PendingExtension{extension_id,..}=>extension_id.as_ptr()as usize,_=>self.original_text().map_or(self as*const _ as usize,|value|value.as_ptr()as usize)}}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
  if path.is_empty(){return Ok(View::IntrinsicObject(match self{Self::CycleDetected=>1,Self::PendingExtension{..}=>4,_=>2}))}if path==[0]{return Ok(View::IntrinsicText(self.original_variant()))}
  if let Self::Retained(value)=self{if path.first()==Some(&1){return semio_framework_job::project_original_value_error(value,&path[1..])}}
  if path.len()!=1{return Err(absent())}if let Some(value)=self.original_text(){return if path[0]==1{Ok(View::IntrinsicText(value))}else{Err(absent())}}
  if let Self::PendingExtension{extension_id,operator_id,node_hash}=self{return match path[0]{1=>Ok(View::IntrinsicText(extension_id)),2=>Ok(View::IntrinsicText(operator_id)),3=>Ok(View::IntrinsicNumber(semio_framework_value::Number::UInt(*node_hash))),_=>Err(absent())}}Err(absent())
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if path.is_empty(){return match index{0=>Ok("variant"),1=>Ok(match self{Self::Retained(_)=>"retained",Self::PendingExtension{..}=>"extensionId",_ if self.original_text().is_some()=>"value",_=>return Err(absent())}),2 if matches!(self,Self::PendingExtension{..})=>Ok("operatorId"),3 if matches!(self,Self::PendingExtension{..})=>Ok("nodeHash"),_=>Err(absent())}}
  if let Self::Retained(value)=self{if path.first()==Some(&1){return semio_framework_job::key_original_value_error(value,&path[1..],index)}}Err(absent())
 }
}
impl OriginalDomainFailure for EvalError{
 fn domain_code(&self)->&str{match self{Self::Retained(_)=>"neural.retained",Self::UnknownKind(_)=>"neural.unknown-kind",Self::MissingInput(_)=>"neural.missing-input",Self::InvalidInput(_)=>"neural.invalid-input",Self::CardinalityViolation(_)=>"neural.cardinality-violation",Self::HeterogeneousList(_)=>"neural.heterogeneous-list",Self::CycleDetected=>"neural.cycle-detected",Self::PendingExtension{..}=>"neural.pending-extension"}}
 fn domain_message(&self)->&str{match self{Self::Retained(value)=>&value.message,Self::CycleDetected=>"cycle detected",Self::PendingExtension{extension_id,..}=>extension_id,_=>self.original_text().unwrap()}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match self{Self::Retained(value)=>value.retained_progress(),_=>Default::default()}}
}
fn progress_view(progress:RetainedCloneProgress,path:&[usize])->Result<View<'static>,ValueError>{if path.is_empty(){return Ok(View::IntrinsicObject(4))}if path.len()!=1{return Err(absent())}Ok(View::IntrinsicNumber(semio_framework_value::Number::UInt(match path[0]{0=>progress.copied_items,1=>progress.copied_bytes,2=>progress.retained_capacity_bytes,3=>progress.released_bytes,_=>return Err(absent())}as u64)))}
impl OriginalDomainProjection for OperatorWorkFailure{
 fn projection_identity(&self)->usize{match self{Self::Evaluation{primary,..}=>primary.projection_identity(),Self::Original(value)=>value.primary.message().as_ptr()as usize}}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
  match self{
   Self::Original(value)=>semio_framework_job::project_original_turn_failure(value,path),
   Self::Evaluation{primary,retained,receipt_refusal}=>{if path.is_empty(){return Ok(View::IntrinsicObject(4))}match path[0]{0 if path.len()==1=>Ok(View::IntrinsicText("evaluation")),1=>primary.projection_view(&path[1..]),2=>progress_view(*retained,&path[1..]),3=>match receipt_refusal{Some(value)=>semio_framework_job::project_original_value_error(value,&path[1..]),None if path.len()==1=>Ok(View::IntrinsicNull),_=>Err(absent())},_=>Err(absent())}},
  }
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match self{Self::Original(value)=>semio_framework_job::key_original_turn_failure(value,path,index),Self::Evaluation{primary,receipt_refusal,..}=>{if path.is_empty(){return ["variant","primary","retained","receiptRefusal"].get(index).copied().ok_or_else(absent)}match path[0]{1=>primary.projection_key(&path[1..],index),2 if path.len()==1=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(absent),3=>receipt_refusal.as_ref().map_or_else(||Err(absent()),|value|semio_framework_job::key_original_value_error(value,&path[1..],index)),_=>Err(absent())}}}}
}
impl OriginalDomainFailure for OperatorWorkFailure{
 fn domain_code(&self)->&str{match self{Self::Evaluation{primary,..}=>primary.domain_code(),Self::Original(value)=>match &value.primary{semio_framework_job::OriginalTurnPrimary::Domain(value)=>value.code(),semio_framework_job::OriginalTurnPrimary::Value(_)=>"neural.original-value",semio_framework_job::OriginalTurnPrimary::Receiving(_)=>"neural.original-receiving",semio_framework_job::OriginalTurnPrimary::Fault(value)=>&value.code.0}}}
 fn domain_message(&self)->&str{match self{Self::Evaluation{primary,..}=>primary.domain_message(),Self::Original(value)=>value.primary.message()}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.retained_progress()}
}
