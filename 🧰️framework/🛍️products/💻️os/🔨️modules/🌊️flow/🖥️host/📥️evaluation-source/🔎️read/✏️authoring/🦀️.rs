//! 🧵️ Original borrowed Flow authoring retains its exact source, fields and domain outcome.
use super::*;
use semio_framework_artifact_flow_flow::{FlowMutation,WidgetDescriptor};
use semio_os_flow_host_core::authoring_layout::DagLayoutOptions;
#[path="📦️storage/🦀️.rs"]mod storage;
use semio_framework_job_owned_text as text;
#[path="🔎️compare/🦀️.rs"]mod compare;
#[path="🗑️remove/🦀️.rs"]mod remove;
#[path="💎️value/🦀️.rs"]mod value;
#[path="🧩️widget/🦀️.rs"]mod widget;
#[path="🪪️identity/🦀️.rs"]mod identity;
#[path="➕️add/🦀️.rs"]mod add;
/// 🎮️ The original command lends operation fields without constructing owned request mirrors.
#[derive(Clone,Copy)]
pub enum FlowAuthoringRequest<'a>{
 ConnectPorts{from:&'a str,from_port:&'a str,to:&'a str,to_port:&'a str},
 Disconnect{id:&'a str},
 RemoveWidget{id:&'a str},
 AddWidget{descriptor:&'a WidgetDescriptor,x:f64,y:f64},
 InsertPort{id:&'a str,input:bool,index:usize},
 Reorganize{options:&'a DagLayoutOptions},
}
impl FlowAuthoringRequest<'_>{
 fn identity(self)->[u64;20]{
  let mut key=[0;20];
  fn text(key:&mut[u64;20],slot:usize,value:&str){key[slot]=value.as_ptr()as usize as u64;key[slot+1]=value.len()as u64;}
  match self{
   Self::ConnectPorts{from,from_port,to,to_port}=>{key[0]=1;text(&mut key,1,from);text(&mut key,3,from_port);text(&mut key,5,to);text(&mut key,7,to_port)},
   Self::Disconnect{id}=>{key[0]=2;text(&mut key,1,id)},
   Self::RemoveWidget{id}=>{key[0]=3;text(&mut key,1,id)},
   Self::AddWidget{descriptor,x,y}=>{key[0]=4;key[1]=descriptor as*const _ as usize as u64;key[2]=x.to_bits();key[3]=y.to_bits()},
   Self::InsertPort{id,input,index}=>{key[0]=5;text(&mut key,1,id);key[3]=u64::from(input);key[4]=index as u64},
   Self::Reorganize{options}=>{key[0]=6;key[1]=options as*const _ as usize as u64;key[2]=options.layer_spacing.to_bits();key[3]=options.sibling_gap.to_bits();key[4]=match options.orientation{semio_os_flow_host_core::authoring_layout::DagLayoutOrientation::LeftRight=>0,semio_os_flow_host_core::authoring_layout::DagLayoutOrientation::TopBottom=>1};key[5]=u64::from(options.center_x.is_some());key[6]=options.center_x.map_or(0,f64::to_bits);key[7]=u64::from(options.center_y.is_some());key[8]=options.center_y.map_or(0,f64::to_bits)},
  }key
 }
}
/// 📦️ Domain refusal and successful leaves retain distinct, movable original custody.
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowAuthoringOutcome{pub result:Result<Vec<FlowMutation>,FlowCoreError>,pub serials:FlowAuthoringSerials}

#[path="🔗️synapse/🦀️.rs"]mod synapse;
#[path="🔢️unsigned/🦀️.rs"]mod unsigned;
#[path="🔌️insert/🦀️.rs"]mod insert;

#[path="🔌️ports/🦀️.rs"]mod ports;
#[path="🔤️types/🦀️.rs"]mod types;
#[path="🌳️reach/🦀️.rs"]mod reach;
#[path="🔗️connect/🦀️.rs"]mod connect;
#[path="📐️reorganize/🦀️.rs"]mod reorganize;
#[path="🎮️controller/🦀️.rs"]mod controller;
pub use controller::FlowAuthoringWork;

/// 📑️ A genuine borrowed source field becomes one retained owned string.
pub use text::FundedText as FlowTextCopyWork;
/// 🔗️ Five original borrowed endpoint fields become one retained native synapse.
pub use synapse::SynapseWork as FlowSynapseFieldsWork;
