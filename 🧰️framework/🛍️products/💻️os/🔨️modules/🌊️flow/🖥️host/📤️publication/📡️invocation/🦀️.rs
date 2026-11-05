//! 📡️ The original invocation source remains leased until every nested response has been admitted.
use super::*;
#[path="🪪️identity/🦀️.rs"]
pub mod identity;
pub use identity::FlowInvocationIdentity;
pub use semio_os_flow_host_source::invocation_request as request_work;
pub use request_work::{FlowInvocationRequestSource,FlowInvocationRequestWork,FlowInvocationExternalResumeSource};
pub use semio_os_flow_host_source::dependency_publication as dependency_work;
pub use dependency_work::FlowInferenceDependencyWork;
#[path="📥️origin-work/🦀️.rs"]
pub mod origin_work;
pub use origin_work::FlowInvocationOriginWork;
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
 pub preview_source:Option<FlowPreviewSource>,
}
pub type FlowInvocationOriginLease=semio_framework_value_ordered::ordered::SharedOwner<FlowInvocationOrigin>;
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct FlowInvocationEntry{pub origin:FlowInvocationOriginLease,pub reply:Option<String>}
impl FlowInvocationOrigin{
 /// 🧊️ The diagnostic wire borrows the original source on initial dispatch and every retry.
 pub fn request_json_cold(&self,resume:bool)->String{use semio_framework_value::DslValue as V;semio_framework_pack_json::to_json_string(&V::object([
  ("neuronId".into(),V::String(self.neuron_id.clone())),("operatorId".into(),V::String(self.operator_id.clone())),("inputJson".into(),V::String(if resume{String::new()}else{self.input_json.clone()})),("dependencyJson".into(),V::String(if resume{String::new()}else{self.dependency_json.clone()})),("operatorVersion".into(),V::String(self.operator_version.clone())),("nodeHash".into(),V::uint(self.node_hash)),("resume".into(),V::Bool(resume)),("windowId".into(),V::String(self.window_id.clone())),("windowKindId".into(),V::String(self.window_kind_id.clone())),("extensionId".into(),V::String(self.extension_id.clone())),
 ]))}
 /// 🧊️ The genuine pending row supplies its identity and input; its caller supplies the original context.
 pub fn pending_origin_cold(&self,pending:neural::PendingExtensionEval)->Self{Self{window_id:self.window_id.clone(),window_kind_id:self.window_kind_id.clone(),neuron_id:pending.neuron_id,extension_id:pending.extension_id,operator_id:pending.operator_id,node_hash:pending.node_hash,input_json:pending.input_json,dependency_json:self.dependency_json.clone(),operator_version:self.operator_version.clone(),outer_node_hash:Some(self.node_hash),preview_source:None}}
}

pub use semio_os_flow_host_source::invocation_dispatch as dispatch_work;
pub use dispatch_work::{FlowInvocationDispatch,FlowInvocationDispatchWork};

impl FlowInvocationRequestSource for FlowInvocationOrigin{
 fn invocation_source_identity(&self)->usize{self as *const _ as usize}
 fn extension_id(&self)->&str{&self.extension_id}
 fn neuron_id(&self)->&str{&self.neuron_id}
 fn operator_id(&self)->&str{&self.operator_id}
 fn input_json(&self)->&str{&self.input_json}
 fn dependency_json(&self)->&str{&self.dependency_json}
 fn operator_version(&self)->&str{&self.operator_version}
 fn window_id(&self)->&str{&self.window_id}
 fn window_kind_id(&self)->&str{&self.window_kind_id}
 fn node_hash(&self)->u64{self.node_hash}
}
