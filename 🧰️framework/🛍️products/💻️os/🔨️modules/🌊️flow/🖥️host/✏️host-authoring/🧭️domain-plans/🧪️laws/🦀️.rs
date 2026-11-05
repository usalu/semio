//! 🧪️ Exact authoring leaves against the genuine host implementation.
use super::*;

#[test]
fn borrowed_authoring_plans_match_genuine_host_operations(){
 let cases=semio_framework_pack_json::parse(include_str!("📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let cases=cases.as_array().unwrap();
 assert_eq!(cases.len(),9);
 for case in cases{
  let operation=case.get("operation").and_then(|value|value.as_str()).unwrap();
  let infos:Arc<HistoryFoldIndex<String,OperatorInfo>>=Arc::new(flow_operator_registry().operator_infos().map(|info|(info.id.clone(),info.clone())).collect());
  let mut source=FlowHostSnapshot::default();
  source.widgets=vec![Widget::InputSlider{id:"a".into(),label:"A".into(),value:1.0,min:0.0,max:10.0,step:1.0},Widget::InputNote{id:"b".into(),text:"B".into()},Widget::OutputAction{id:"sink".into(),action:String::new()}];
  if operation.starts_with("insert"){
   let input=operation=="insertInput";
   let (kind,_)=infos.iter().find(|(_,info)|if input{info.variadic_input.is_some()}else{info.variadic_output.is_some()}).expect("real catalogue contains variadic operation");
   source.widgets.push(Widget::Neuron{id:"variadic".into(),neuron_kind:kind.clone(),params:Default::default(),input_ports:Default::default(),output_ports:Default::default(),preview:true});
  }
  let from_port=first_output_port("a",&source.widgets,&source.synapses,&infos);
  let to_port=first_input_port("sink",&source.widgets,&source.synapses,&infos);
  source.synapses.push(SynapseSpec{id:"s101".into(),from:"a".into(),from_port:from_port.clone(),to:"sink".into(),to_port:to_port.clone()});
  source.layout.insert("a".into(),WidgetLayout{x:20.0,y:30.0});
  let mut host=FlowHost::from_host_snapshot(source.clone()).with_history_store_factory(semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY);
  host.set_neuron_kind_info_map(infos.clone());
  host.begin_gesture();
  let mut serials=FlowAuthoringSerials::default();
  let descriptor=WidgetDescriptor::InputNote{id:None,text:Some("new".into())};
  let options=DagLayoutOptions::default();
  let mut progress=|_|true;
  let mut control=semio_os_infinite::board::schema::layout::LayoutControl::new(u64::MAX,&mut progress);
  let plan=match operation{
   "connect"=>FlowAuthoringPlan::connect_ports(&source,&infos,&mut serials,"b",&first_output_port("b",&source.widgets,&source.synapses,&infos),"sink",&to_port),
   "selfConnect"=>FlowAuthoringPlan::connect_ports(&source,&infos,&mut serials,"a",&from_port,"a",&from_port),
   "disconnect"=>FlowAuthoringPlan::disconnect(&source,"s101"),
   "unknownDisconnect"=>FlowAuthoringPlan::disconnect(&source,"absent"),
   "remove"=>FlowAuthoringPlan::remove_widget(&source,"a"),
   "add"=>FlowAuthoringPlan::add_widget(&source,&infos,&mut serials,&descriptor,42.0,13.0),
   "insertInput"=>FlowAuthoringPlan::insert_port(&source,&infos,"variadic",true,0),
   "insertOutput"=>FlowAuthoringPlan::insert_port(&source,&infos,"variadic",false,0),
   "reorganize"=>FlowAuthoringPlan::reorganize(&source,&infos,&options,&mut control),
   other=>panic!("unknown authoring case {other}"),
  };
  let mut original_progress=|_|true;
  let mut original_control=semio_os_infinite::board::schema::layout::LayoutControl::new(u64::MAX,&mut original_progress);
  let actual=match operation{
   "connect"=>host.connect_ports("b",&first_output_port("b",&source.widgets,&source.synapses,&infos),"sink",&to_port).map(|_|()),
   "selfConnect"=>host.connect_ports("a",&from_port,"a",&from_port).map(|_|()),
   "disconnect"=>host.disconnect("s101"),
   "unknownDisconnect"=>host.disconnect("absent"),
   "remove"=>host.remove_widget("a"),
   "add"=>host.add_widget_descriptor(&descriptor,42.0,13.0).map(|_|()),
   "insertInput"=>host.add_input_port("variadic",0),
   "insertOutput"=>host.add_output_port("variadic",0),
   "reorganize"=>host.reorganize(&options,&mut original_control),
   _=>unreachable!(),
  };
  match(plan,actual){
   (Ok(leaves),Ok(()))=>{assert_eq!(leaves,host.pending_leaves,"{}",operation);FlowAuthoringPlan::apply(&mut source,&leaves);assert_eq!(source,host.host_snapshot,"{}",operation);for leaf in leaves{retire_flow_mutation(leaf);}},
   (Err(expected),Err(actual))=>assert_eq!(expected.to_string(),actual.to_string(),"{}",operation),
   (expected,actual)=>panic!("authoring result mismatch for {}: plan success {}, host success {}",operation,expected.is_ok(),actual.is_ok()),
  }
  source.retire_cold();
  assert!(host.retire_cold().is_ok(),"genuine oracle host closes");
 }
}
