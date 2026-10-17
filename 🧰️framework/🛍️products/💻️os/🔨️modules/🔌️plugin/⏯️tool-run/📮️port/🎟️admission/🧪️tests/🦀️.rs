//! 📮️ Native original effect queue births precede same-pointer handoffs and literal0 closure.
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPortState;
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_ui_scene::SceneDoc;
use semio_framework_plugin_decl_views::tool_run_contract::{ToolRunJobPort, ToolRunRetargetableJob};
use crate::{ToolRunMemberEmissionOwner, ToolRunMemberOwners};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;
#[test]
fn original_tool_run_port_dispatch_preserves_same_effect_and_native_queue_receipts(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:8192,maximum_release_bytes:65536,maximum_depth:9};let port=ToolRunJobPort::default();let source=String::from("original action source 🧵");let pointer=source.as_ptr();let mut source=Some(Effect::DispatchAction{req:semio_framework::kernel::RequestId(17),action:source,args:None,delay_ms:0});let mut born=0;let mut released=0;
 for turn in 0..2{for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||port.admit_original_dispatch(&mut source,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(source.is_some());}if turn==0{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||port.admit_original_dispatch(&mut source,RetainedCloneGrant{maximum_capacity_bytes:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||port.admit_original_dispatch(&mut source,grant).unwrap());assert_eq!(step.progress().copied_bytes,0);assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert_eq!(source.is_some(),turn==0);}
 let original=port.pop_effect().unwrap();assert!(matches!(&original,Effect::DispatchAction{action,..}if action.as_ptr()==pointer));drop(original);let mut original=Some(port);let(mut close,receipt)=ToolRunJobPort::admit_original_close(&mut original,grant).unwrap().unwrap();assert_eq!(receipt.copied_bytes,0);while !close.terminal_is_empty(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close.close_step(grant).unwrap());assert_eq!(step.progress().copied_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));released+=heap.released_bytes;}assert_eq!(released-born,semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<ToolRunJobPortState>());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(close));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
