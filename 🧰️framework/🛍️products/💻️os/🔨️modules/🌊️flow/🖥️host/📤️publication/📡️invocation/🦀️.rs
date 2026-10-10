//! 📡️ The original invocation source remains leased until every nested response has been admitted.
use super::*;
#[path="🪪️identity/🦀️.rs"]
mod identity;
pub use identity::FlowInvocationIdentity;
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowInvocationOrigin{
 pub window_id:String,
 pub window_kind_id:String,
 pub neuron_id:String,
 pub extension_id:String,
 pub operator_id:String,
 pub node_hash:u64,
 pub input_json:String,
 pub dependency_json:String,
 pub operator_version:String,
 pub outer_node_hash:Option<u64>,
}
pub type FlowInvocationOriginLease=protocol::value::ordered::SharedOwner<FlowInvocationOrigin>;
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FlowInvocationEntry{pub origin:FlowInvocationOriginLease,pub reply:Option<String>}
impl FlowInvocationOrigin{
 /// 🧊️ The diagnostic wire borrows the original source on initial dispatch and every retry.
 pub fn request_json_cold(&self,resume:bool)->String{use semio_framework_value::DslValue as V;semio_framework_pack_json::to_json_string(&V::object([
  ("neuronId".into(),V::String(self.neuron_id.clone())),("operatorId".into(),V::String(self.operator_id.clone())),("inputJson".into(),V::String(self.input_json.clone())),("dependencyJson".into(),V::String(self.dependency_json.clone())),("operatorVersion".into(),V::String(self.operator_version.clone())),("nodeHash".into(),V::uint(self.node_hash)),("resume".into(),V::Bool(resume)),("windowId".into(),V::String(self.window_id.clone())),("windowKindId".into(),V::String(self.window_kind_id.clone())),
 ]))}
 /// 🧊️ The genuine pending row supplies its identity and input; its caller supplies the original context.
 pub fn pending_origin_cold(&self,pending:neural::PendingExtensionEval)->Self{Self{window_id:self.window_id.clone(),window_kind_id:self.window_kind_id.clone(),neuron_id:pending.neuron_id,extension_id:pending.extension_id,operator_id:pending.operator_id,node_hash:pending.node_hash,input_json:pending.input_json,dependency_json:self.dependency_json.clone(),operator_version:self.operator_version.clone(),outer_node_hash:Some(self.node_hash)}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
