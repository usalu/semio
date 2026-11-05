//! 🧪️ Original authoring matches genuine Host leaves within finite caller opportunities and cancellation custody.
use super::*;
use semio_framework_artifact_flow_flow::FlowMutation;
#[path="🔍️serde/🦀️.rs"]mod serde_oracle;
use semio_os_flow_operators::registry::{flow_operator_registry,install_flow_extension_manifest,uninstall_flow_extension,lock_flow_extension_registry_for_test,drain_flow_extension_registry_retirements};
use neural_engine::ColdRetire;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalTurnFailure,StepContext,OperationId,Generation,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value::retained_clone::RetainedCloneClose;
fn clock()->Option<u64>{Some(1000)}
fn native<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
fn source(infos:&HistoryFoldIndex<String,OperatorInfo>,operation:&str)->FlowHostSnapshot{
 let mut source=FlowHostSnapshot::default();
 source.widgets=vec![Widget::InputSlider{id:"a".into(),label:"A".into(),value:1.0,min:0.0,max:10.0,step:1.0},Widget::InputNote{id:"b".into(),text:"B".into()},Widget::OutputAction{id:"sink".into(),action:String::new()}];
 source.synapses=vec![SynapseSpec{id:"s101".into(),from:"a".into(),from_port:"number".into(),to:"sink".into(),to_port:String::new()}];
 source.layout=Default::default();source.layout.insert("a".into(),WidgetLayout{x:20.0,y:30.0});
 if operation.starts_with("insert"){let input=operation=="insertInput";let(kind,_)=infos.iter().find(|(_,info)|if input{info.variadic_input.is_some()}else{info.variadic_output.is_some()}).expect("real variadic catalogue operation");source.widgets.push(Widget::Neuron{id:"variadic".into(),neuron_kind:kind.clone(),params:Default::default(),input_ports:Default::default(),output_ports:Default::default(),preview:true});source.synapses.push(if input{SynapseSpec{id:"shift".into(),from:"a".into(),from_port:"number".into(),to:"variadic".into(),to_port:"0".into()}}else{SynapseSpec{id:"shift".into(),from:"variadic".into(),from_port:"0".into(),to:"sink".into(),to_port:String::new()}})}
 if operation=="addOccupied"{source.widgets.push(Widget::InputNote{id:"note_02".into(),text:String::new()});source.widgets.push(Widget::InputNote{id:"note_+3".into(),text:String::new()});}
 source
}
#[test]
fn original_authoring_matches_actual_host_all_six_operations_and_cancel_custody(){
 let _registry=lock_flow_extension_registry_for_test();
 let mut installed=Vec::new();
 for relative in ["✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🧩️component/🔣️.json","✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🧩️component/🔣️.json"]{
  let descriptor:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!(relative)).unwrap();
  let contribution=descriptor["manifest"]["topicContributions"].as_array().unwrap().iter().find(|value|value["topic"]=="flow.extension").unwrap();
  let manifest=contribution["payload"]["manifestJson"].as_str().unwrap();let metadata:serde_json::Value=serde_json::from_str(manifest).unwrap();
  install_flow_extension_manifest(descriptor["manifest"]["pluginId"].as_str().unwrap(),manifest).expect("actual authored extension manifest");installed.push(metadata["id"].as_str().unwrap().to_owned());
 }
 let law:serde_json::Value=serde_json::from_str(include_str!("📁️profiles.json")).unwrap();
 let infos:Arc<HistoryFoldIndex<String,OperatorInfo>>=Arc::new(flow_operator_registry().operator_infos().map(|info|(info.id.clone(),info.clone())).collect());
 for operation in law["operations"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()){
  let source=source(&infos,operation);let pointer=source.widgets.as_ptr();let descriptor=if let Some(descriptor)=law["descriptors"].get(operation){serde_json::from_value(descriptor.clone()).unwrap()}else{WidgetDescriptor::InputNote{id:match operation{"addTrimmed"=>Some("\u{2003} original \n\t".into()),"addBlank"=>Some(" ".repeat(5000)),_=>None},text:Some("new".into())}};let options=semio_os_flow_host_core::authoring_layout::DagLayoutOptions::default();
  let request=match operation{
   "connect"=>FlowAuthoringRequest::ConnectPorts{from:"b",from_port:"text",to:"sink",to_port:""},
   "selfConnect"=>FlowAuthoringRequest::ConnectPorts{from:"a",from_port:"number",to:"a",to_port:"number"},
   "disconnect"=>FlowAuthoringRequest::Disconnect{id:"s101"},
   "unknownDisconnect"=>FlowAuthoringRequest::Disconnect{id:"absent"},
   "remove"=>FlowAuthoringRequest::RemoveWidget{id:"a"},
   "add"|"addOccupied"|"addTrimmed"|"addBlank"|"addSlider"|"addSliderRange"|"addSliderDefaults"=>FlowAuthoringRequest::AddWidget{descriptor:&descriptor,x:42.0,y:13.0},
   "insertInput"=>FlowAuthoringRequest::InsertPort{id:"variadic",input:true,index:0},
   "insertOutput"=>FlowAuthoringRequest::InsertPort{id:"variadic",input:false,index:0},
   "reorganize"=>FlowAuthoringRequest::Reorganize{options:&options},
   _=>unreachable!(),
  };
  let mut host=FlowHost::from_host_snapshot(source.clone()).with_history_store_factory(semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY);host.set_neuron_kind_info_map(infos.clone());host.begin_gesture();
  let mut progress=|_|true;let mut control=semio_os_flow_host_core::authoring_layout::LayoutControl::new(u64::MAX,&mut progress);
  let expected=match request{
   FlowAuthoringRequest::ConnectPorts{from,from_port,to,to_port}=>host.connect_ports(from,from_port,to,to_port).map(|_|()),
   FlowAuthoringRequest::Disconnect{id}=>host.disconnect(id),FlowAuthoringRequest::RemoveWidget{id}=>host.remove_widget(id),
   FlowAuthoringRequest::AddWidget{descriptor,x,y}=>host.add_widget_descriptor(descriptor,x,y).map(|_|()),
   FlowAuthoringRequest::InsertPort{id,input:true,index}=>host.add_input_port(id,index),FlowAuthoringRequest::InsertPort{id,input:false,index}=>host.add_output_port(id,index),
   FlowAuthoringRequest::Reorganize{options}=>host.reorganize(options,&mut control),
  };
  for profile in law["profiles"].as_array().unwrap(){
   for cancel in law["cancelAfter"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain([None]){
    let calls=profile["calls"].as_u64().unwrap()as usize;let g=RetainedCloneGrant{maximum_items:profile["items"].as_u64().unwrap()as usize,maximum_copy_bytes:profile["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:4096};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128*1024*1024,maximum_capacity_bytes:256*1024*1024,maximum_release_bytes:512*1024*1024,maximum_depth:4096},clock,1500,32768);
    let mut decoder=OriginalJobDecodeOwner::empty();native(&mut caller,&mut decoder,g,|_|Ok(()));let mut work=FlowAuthoringWork::new(FlowAuthoringSerials::default());let mut output=None;
    for turn in 0..calls{
     if cancel==Some(turn){caller.cancel().cancel_now();break}
     let before=work.next_demands(&source,&infos,request,g.maximum_copy_bytes).unwrap();
     native(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..g},|cx|work.step(&source,&infos,request,cx));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(work.next_demands(&source,&infos,request,g.maximum_copy_bytes).unwrap(),before);
     output=native(&mut caller,&mut decoder,g,|cx|work.step(&source,&infos,request,cx));assert_eq!(source.widgets.as_ptr(),pointer);if output.is_some(){break}assert!(turn+1<calls,"{operation} did not finish in original {calls} calls: demand={before:?}, progress={:?}, work={}",work.step_progress(),work.last_turn_work());
    }
    if let Some(outcome)=output.as_ref(){match(&outcome.result,&expected){(Ok(leaves),Ok(()))=>{assert_eq!(leaves,&host.pending_leaves,"{operation}");let ours:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(leaves)).unwrap();let oracle=serde_oracle::leaves(&host.pending_leaves);assert_eq!(ours,oracle);if let Some(expected)=law["sliderFields"].get(operation){let FlowMutation::AddWidget(semio_framework_artifact_flow_flow::AddWidget{widget:Widget::InputSlider{value,min,max,step,..},..})=&leaves[0]else{panic!("authored slider is the first domain leaf")};for(actual,expected)in [*value,*min,*max,*step].into_iter().zip(expected.as_array().unwrap()){assert_eq!(actual,expected.as_f64().unwrap(),"{operation}")}}},(Err(actual),Err(expected))=>assert_eq!(actual.to_string(),expected.to_string(),"{operation}"),_=>panic!("genuine authoring result mismatch for {operation}")}}
    if cancel.is_none(){assert!(output.is_some(),"{operation} original caller opportunity exhausted")}
    work.begin_close();for turn in 0..10000{if work.terminal_is_empty(){break}native(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{let d=work.next_close_demands(g.maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|work.close_step(grant))?;Ok(())}));assert!(turn+1<10000)}
    assert!(work.terminal_is_empty());drop(work);let mut close=RetainedCloneClose::default();
    for turn in 0..10000{if output.is_none()&&close.is_empty(){break}native(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{let d=close.next_owner_demands::<FlowAuthoringOutcome>(output.is_some(),&None,g.maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|if close.is_empty(){let turn=close.begin_granted(&mut output,grant);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}else{close.step_granted(grant)})?;Ok(())}));assert!(turn+1<10000)}
    while !decoder.terminal_is_empty(){caller.turn(g,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(g.maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}caller.retire(g);
   }
  }
  assert!(host.retire_cold().is_ok(),"genuine authoring oracle closes");source.retire_cold();
 }
 let Ok(infos)=Arc::try_unwrap(infos)else{panic!("actual host catalogue snapshot retains a live alias")};infos.retire_cold();for extension in installed{uninstall_flow_extension(&extension).expect("actual extension fixture closes");}assert_eq!(drain_flow_extension_registry_retirements(),0);
}
