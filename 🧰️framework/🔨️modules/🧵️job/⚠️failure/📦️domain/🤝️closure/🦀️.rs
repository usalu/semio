//! 🤝️ Source retirement preserves both complete original failures under the existing paid domain frontier.
use crate::{OriginalTurnFailure,OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,project_original_turn_failure,key_original_turn_failure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
/// 🪢️ The initial failure remains independent from its complete source-closure failure.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalClosureFailure{pub initial:OriginalTurnFailure,pub closure:OriginalTurnFailure}
impl OriginalDomainFailure for OriginalClosureFailure{
 fn domain_code(&self)->&str{"original.source-closure"}
 fn domain_message(&self)->&str{self.initial.primary.message()}
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original closure failure projection path changed")}
impl OriginalDomainProjection for OriginalClosureFailure{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::IntrinsicObject(2)),[0,rest @ ..]=>project_original_turn_failure(&self.initial,rest),[1,rest @ ..]=>project_original_turn_failure(&self.closure,rest),_=>Err(invalid())}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{[]=>["initial","closure"].get(index).copied().ok_or_else(invalid),[0,rest @ ..]=>key_original_turn_failure(&self.initial,rest,index),[1,rest @ ..]=>key_original_turn_failure(&self.closure,rest,index),_=>Err(invalid())}}
}
