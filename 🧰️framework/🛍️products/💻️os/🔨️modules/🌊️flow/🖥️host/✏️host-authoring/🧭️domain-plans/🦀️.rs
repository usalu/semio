//! 🧭️ Domain authoring plans over the original borrowed Flow snapshot.
use super::*;

#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub struct FlowAuthoringSerials { pub widget:u64, pub synapse:u64 }
impl Default for FlowAuthoringSerials { fn default()->Self { Self{widget:1,synapse:100} } }

pub struct FlowAuthoringPlan;
impl FlowAuthoringPlan {
 pub fn connect_ports(source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,serials:&mut FlowAuthoringSerials,from:&str,from_port:&str,to:&str,to_port:&str)->Result<Vec<FlowMutation>,FlowCoreError>{
  if from==to{return Err(FlowCoreError::SelfConnection)}
  if !widget_has_output(from,&source.widgets(),&source.synapses(),infos){return Err(FlowCoreError::NoOutputPort(from.into()))}
  if !widget_has_input(to,&source.widgets(),&source.synapses(),infos){return Err(FlowCoreError::NoInputPort(to.into()))}
  if !widget_has_declared_port(from,from_port,PortSide::Output,&source.widgets(),&source.synapses(),infos){return Err(FlowCoreError::UnknownOutputPort(from_port.into()))}
  if !widget_has_declared_port(to,to_port,PortSide::Input,&source.widgets(),&source.synapses(),infos){return Err(FlowCoreError::UnknownInputPort(to_port.into()))}
  let edges:Vec<_>=source.synapses().iter().map(|wire|(wire.from.clone(),wire.to.clone())).collect();
  if would_create_cycle(&edges,from,to){return Err(FlowCoreError::CycleWouldBeCreated)}
  if source.synapses().iter().any(|wire|wire.from==from&&wire.from_port==from_port&&wire.to==to&&wire.to_port==to_port){return Err(FlowCoreError::ConnectionAlreadyExists)}
  let source_types=widget_port_value_types(from,from_port,PortSide::Output,&source.widgets(),&source.synapses(),infos);
  let target_types=widget_port_value_types(to,to_port,PortSide::Input,&source.widgets(),&source.synapses(),infos);
  if !port_value_types_compatible(&source_types,&target_types){return Err(FlowCoreError::IncompatiblePortTypes{source:format!("{from}@{from_port}"),source_type:source_types.join(","),target:format!("{to}@{to_port}"),target_type:target_types.join(",")})}
  let mut leaves:Vec<_>=source.synapses().iter().filter(|wire|wire.to==to&&wire.to_port==to_port).map(|wire|FlowMutation::RemoveSynapse(RemoveSynapse{id:wire.id.clone()})).collect();
  let id=loop{serials.synapse=serials.synapse.wrapping_add(1);let id=format!("s{}",serials.synapse);if source.synapses().iter().filter(|wire|!(wire.to==to&&wire.to_port==to_port)).all(|wire|wire.id!=id){break id}};
  let index=source.synapses().len()-leaves.len();
  leaves.push(FlowMutation::AddSynapse(AddSynapse{index:u32::try_from(index).unwrap_or(u32::MAX),synapse:SynapseSpec{id,from:from.into(),to:to.into(),from_port:from_port.into(),to_port:to_port.into()}}));
  Ok(leaves)
 }
 pub fn disconnect(source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,id:&str)->Result<Vec<FlowMutation>,FlowCoreError>{
  if !source.synapses().iter().any(|wire|wire.id==id){return Err(FlowCoreError::UnknownSynapse(id.into()))}
  Ok(vec![FlowMutation::RemoveSynapse(RemoveSynapse{id:id.into()})])
 }
 pub fn remove_widget(source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,id:&str)->Result<Vec<FlowMutation>,FlowCoreError>{
  if !source.widgets().iter().any(|widget|widget_id_for(widget)==id){return Err(FlowCoreError::UnknownWidget(id.into()))}
  let mut leaves:Vec<_>=source.synapses().iter().filter(|wire|wire.from==id||wire.to==id).map(|wire|FlowMutation::RemoveSynapse(RemoveSynapse{id:wire.id.clone()})).collect();
  if source.layout().contains_key(id){leaves.push(FlowMutation::ChangeLayout(ChangeLayout{entries:vec![FlowLayoutEntry{id:id.into(),layout:None}]}))}
  leaves.push(FlowMutation::RemoveWidget(RemoveWidget{id:id.into()}));Ok(leaves)
 }
 pub fn add_widget(source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,serials:&mut FlowAuthoringSerials,descriptor:&WidgetDescriptor,x:f64,y:f64)->Result<Vec<FlowMutation>,FlowCoreError>{
  let id=descriptor_explicit_id(descriptor).unwrap_or_else(||{let(id,serial)=generated_widget_id(descriptor,source.widgets().iter().map(widget_id_for));serials.widget=serial;id});
  if source.widgets().iter().any(|widget|widget_id_for(widget)==id){return Err(FlowCoreError::WidgetIdExists(id))}
  let widget=widget_from_descriptor(descriptor,id.clone(),infos);
  Ok(vec![FlowMutation::AddWidget(AddWidget{index:u32::try_from(source.widgets().len()).unwrap_or(u32::MAX),widget}),FlowMutation::ChangeLayout(ChangeLayout{entries:vec![FlowLayoutEntry{id,layout:Some(WidgetLayout{x,y})}]})])
 }
 pub fn insert_port(source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,id:&str,input:bool,index:usize)->Result<Vec<FlowMutation>,FlowCoreError>{
  let original=source.widgets().iter().find(|widget|widget_id_for(widget)==id).ok_or_else(||FlowCoreError::UnknownNeuronWidget(id.into()))?;
  let Widget::Neuron{neuron_kind,input_ports,output_ports,..}=original else{return Err(FlowCoreError::UnknownNeuronWidget(id.into()))};
  let info=infos.get(neuron_kind);
  let spec=if input{info.and_then(|info|info.variadic_input.as_ref()).ok_or_else(||FlowCoreError::NotVariadicInput(id.into()))?}else{info.and_then(|info|info.variadic_output.as_ref()).ok_or_else(||FlowCoreError::NotVariadicOutput(id.into()))?};
  let ports=if input{default_neuron_input_ports(neuron_kind,input_ports,infos)}else{default_neuron_output_ports(neuron_kind,output_ports,infos)};
  if spec.max.is_some_and(|max|ports.len()>=max){return Err(if input{FlowCoreError::MaxInputPortsReached(id.into())}else{FlowCoreError::MaxOutputPortsReached(id.into())})}
  let at=index.min(ports.len());let mut widget=original.clone();
  let Widget::Neuron{input_ports,output_ports,..}=&mut widget else{unreachable!()};
  *if input{input_ports}else{output_ports}=(0..ports.len()+1).map(|slot|slot.to_string()).collect();
  let mut leaves=Vec::new();
  for wire in source.synapses(){let endpoint=if input{&wire.to}else{&wire.from};let port=if input{&wire.to_port}else{&wire.from_port};if endpoint==id{if let Ok(old)=port.parse::<usize>(){if old>=at{let mut wire=wire.clone();if input{wire.to_port=(old+1).to_string()}else{wire.from_port=(old+1).to_string()}leaves.push(FlowMutation::ChangeSynapse(ChangeSynapse{id:wire.id.clone(),synapse:wire}));}}}}
  leaves.push(FlowMutation::ChangeWidget(ChangeWidget{id:id.into(),widget}));Ok(leaves)
 }
}

impl FlowAuthoringPlan {
 pub fn reorganize(source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,options:&DagLayoutOptions,control:&mut semio_os_infinite::board::schema::layout::LayoutControl<'_>)->Result<Vec<FlowMutation>,FlowCoreError>{
  let mut dag=DagHost::from_host_snapshot_without_layout(FlowHost::raw_dag_snapshot(source,infos));
  dag.graph.reorganize(options,control)?;
  let mut layouts:BTreeMap<_,_>=source.layout().iter().map(|(id,layout)|(id.clone(),layout.clone())).collect();
  layouts.extend(dag.graph.host_snapshot.nodes.iter().map(|node|(node.id.clone(),WidgetLayout{x:node.x,y:node.y})));
  let entries:Vec<_>=layouts.into_iter().map(|(id,layout)|FlowLayoutEntry{id,layout:Some(layout)}).collect();
  Ok(if entries.is_empty(){Vec::new()}else{vec![FlowMutation::ChangeLayout(ChangeLayout{entries})]})
 }
 pub(super) fn apply(source:&mut FlowHostSnapshot,leaves:&[FlowMutation]){
  for leaf in leaves{match leaf{
   FlowMutation::AddWidget(row)=>source.widgets.insert(usize::try_from(row.index).unwrap_or(usize::MAX).min(source.widgets.len()),row.widget.clone()),
   FlowMutation::RemoveWidget(row)=>source.widgets.retain(|widget|widget_id_for(widget)!=row.id),
   FlowMutation::ChangeWidget(row)=>{if let Some(widget)=source.widgets.iter_mut().find(|widget|widget_id_for(widget)==row.id){std::mem::replace(widget,row.widget.clone()).retire_cold();}},
   FlowMutation::AddSynapse(row)=>source.synapses.insert(usize::try_from(row.index).unwrap_or(usize::MAX).min(source.synapses.len()),row.synapse.clone()),
   FlowMutation::RemoveSynapse(row)=>source.synapses.retain(|wire|wire.id!=row.id),
   FlowMutation::ChangeSynapse(row)=>{if let Some(wire)=source.synapses.iter_mut().find(|wire|wire.id==row.id){*wire=row.synapse.clone();}},
   FlowMutation::ChangeLayout(row)=>{for entry in &row.entries{if let Some(layout)=&entry.layout{source.layout.insert(entry.id.clone(),layout.clone());}else{source.layout.remove(&entry.id);}}},
   _=>unreachable!("domain authoring emits only widget, synapse and layout leaves"),
  }}
 }
}

#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod laws;

