//! 👈️ A declared JSON input is decoded once into the actual typed match program.
use crate::standards::v1::subsets::any::schema::{mutations::{edit_lhs,RewriteRuleMutation}};
use crate::standards::v1::subsets::any::schema::rule::Lhs;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use semio_framework_plugin::{Emit,Fault,FaultOrigin,FaultCode,NoConfigMutation};
pub fn set_lhs(state:&RewritingSnapshot,value:&str)->Result<Emit<RewriteRuleMutation,NoConfigMutation>,Fault>{
 let lhs:Lhs=semio_framework_pack_json::from_json_str(value,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|Fault::new(FaultOrigin::App,FaultCode::new("app.command.invalid-args"),error.to_string()))?;
 Ok(if state.lhs==lhs{Emit::default()}else{Emit::mutations(vec![edit_lhs(lhs)])})
}
