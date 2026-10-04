//! 👉️ A declared JSON input is decoded into the typed rewrite program and its actual parameter defaults.
use crate::standards::v1::subsets::any::schema::{Rhs,snapshot::json,mutations::{edit_rhs,text::RewriteRuleMutation}};
use crate::RewritingSnapshot;
use semio_framework_plugin::{Emit,Fault,FaultOrigin,FaultCode,NoConfigMutation};
pub(crate) fn set_rhs(state:&RewritingSnapshot,value:&str)->Result<Emit<RewriteRuleMutation,NoConfigMutation>,Fault>{
 let invalid=|message:String|Fault::new(FaultOrigin::App,FaultCode::new("app.command.invalid-args"),message);
 let value=semio_framework_pack_json::parse(value,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|invalid(error.to_string()))?;
 let rhs=<Rhs as semio_framework_value::FromValue>::from_value(json::rhs(semio_framework_pack_json::to_dsl_value(&value),true).map_err(|error|invalid(error.into_message()))?).map_err(|error|invalid(error.into_message()))?;
 if state.rhs==rhs{return Ok(Emit::default())}
 let defaults=crate::editor::rewriting::default_parameter_bindings(&rhs);
 Ok(Emit::mutations(std::iter::once(edit_rhs(rhs)).chain(crate::editor::rewriting::parameter_binding_mutations(&state.parameter_bindings,&defaults)).collect()))
}
