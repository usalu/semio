//! 🔍️ Independent serde_json projection of genuine Host mutation records for the native golden.
use super::*;
pub(super) fn value(value:&neural_engine::Value)->serde_json::Value{match value{neural_engine::Value::Atom(atom)=>match atom{neural_engine::Atom::Null=>serde_json::Value::Null,neural_engine::Atom::Boolean(v)=>serde_json::json!(v),neural_engine::Atom::Integer(v)=>serde_json::json!(v),neural_engine::Atom::Decimal(v)=>serde_json::json!(v),neural_engine::Atom::String(v)=>serde_json::json!(v)},neural_engine::Value::Dictionary(dictionary)=>dictionary_json(dictionary)}}
fn dictionary_json(dictionary:&neural_engine::Dictionary)->serde_json::Value{let mut result=serde_json::Map::new();for(key,entry)in dictionary.pairs().iter(){result.insert(key.clone(),value(entry));}serde_json::Value::Object(result)}
fn widget(widget:&Widget)->serde_json::Value{match widget{
 Widget::Neuron{id,neuron_kind,params,input_ports,output_ports,preview}=>serde_json::json!({"kind":"neuron","id":id,"neuronKind":neuron_kind,"params":dictionary_json(params),"inputPorts":input_ports,"outputPorts":output_ports,"preview":preview}),
 Widget::InputSlider{id,label,value,min,max,step}=>serde_json::json!({"kind":"inputSlider","id":id,"label":label,"value":value,"min":min,"max":max,"step":step}),
 Widget::InputNote{id,text}=>serde_json::json!({"kind":"inputNote","id":id,"text":text}),
 Widget::InputImage{id,src}=>serde_json::json!({"kind":"inputImage","id":id,"src":src}),
 Widget::Variable{id,name,schema}=>serde_json::json!({"kind":"variable","id":id,"name":name,"schema":schema}),
 Widget::OutputAction{id,action}=>serde_json::json!({"kind":"outputAction","id":id,"action":action}),
 Widget::OutputExport{id,format}=>serde_json::json!({"kind":"outputExport","id":id,"format":format}),
 Widget::OutputPreview{id,preview,expanded}=>serde_json::json!({"kind":"outputPreview","id":id,"preview":dictionary_json(preview),"expanded":expanded.iter().collect::<Vec<_>>()}),
 Widget::Cluster{..}=>panic!("cluster authoring is outside the declared six-operation fixture"),
}}
fn wire(wire:&SynapseSpec)->serde_json::Value{serde_json::json!({"id":wire.id,"from":wire.from,"to":wire.to,"fromPort":wire.from_port,"toPort":wire.to_port})}
pub(super) fn leaves(leaves:&[semio_framework_artifact_flow_flow::FlowMutation])->serde_json::Value{
 use semio_framework_artifact_flow_flow::FlowMutation as M;
 serde_json::Value::Array(leaves.iter().map(|leaf|match leaf{
  M::AddWidget(row)=>serde_json::json!({"operation":"addWidget","index":row.index,"widget":widget(&row.widget)}),
  M::RemoveWidget(row)=>serde_json::json!({"operation":"removeWidget","id":row.id}),
  M::ChangeWidget(row)=>serde_json::json!({"operation":"changeWidget","id":row.id,"widget":widget(&row.widget)}),
  M::AddSynapse(row)=>serde_json::json!({"operation":"addSynapse","index":row.index,"synapse":wire(&row.synapse)}),
  M::RemoveSynapse(row)=>serde_json::json!({"operation":"removeSynapse","id":row.id}),
  M::ChangeSynapse(row)=>serde_json::json!({"operation":"changeSynapse","id":row.id,"synapse":wire(&row.synapse)}),
  M::ChangeLayout(row)=>serde_json::json!({"operation":"changeLayout","entries":row.entries.iter().map(|entry|serde_json::json!({"id":entry.id,"layout":entry.layout.as_ref().map(|layout|serde_json::json!({"x":layout.x,"y":layout.y}))})).collect::<Vec<_>>()}),
  _=>panic!("unexpected operation in the declared authoring fixture"),
 }).collect())
}
