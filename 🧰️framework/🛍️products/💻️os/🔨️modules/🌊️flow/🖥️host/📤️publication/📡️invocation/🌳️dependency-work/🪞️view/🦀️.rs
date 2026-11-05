//! 🪞️ The canonical dependency wire projects original widget, neural tree and dictionary fields.
use super::*;
use semio_framework_pack_json::{JsonWriteNode as Node,JsonWriteSource};
use semio_framework_value::Number;
pub(super) struct FlowDependencyView<'a>{pub source:&'a dyn FlowSnapshotSource,pub widgets:&'a[usize],pub synapses:&'a[usize]}
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original dependency JSON ordinal is absent")}
fn scalar<'a>(text:&'a str,path:&[usize])->Result<Node<'a>,ValueError>{if path.is_empty(){Ok(Node::String(text))}else{Err(absent())}}
fn key(keys:&[&'static str],index:usize)->Result<&'static str,ValueError>{keys.get(index).copied().ok_or_else(absent)}
fn widget_node<'a>(widget:&'a Widget,path:&[usize])->Result<Node<'a>,ValueError>{
 if let Widget::Cluster{tree,..}=widget{return tree_node(tree,path)}
 if path.is_empty(){return Ok(match widget{Widget::Neuron{..}|Widget::Variable{..}=>Node::Object(2),Widget::InputSlider{..}|Widget::InputNote{..}|Widget::InputImage{..}=>Node::Object(1),_=>Node::Null})}
 match(widget,path[0]){
  (Widget::Neuron{neuron_kind,..},0)=>scalar(neuron_kind,&path[1..]),
  (Widget::Neuron{params,..},1)=>params.node_at_path(&path[1..]),
  (Widget::InputSlider{value,..},0)if path.len()==1=>Ok(Node::Number(Number::Float(*value))),
  (Widget::InputNote{text,..},0)=>scalar(text,&path[1..]),
  (Widget::InputImage{src,..},0)=>scalar(src,&path[1..]),
  (Widget::Variable{name,..},0)=>scalar(name,&path[1..]),
  (Widget::Variable{schema,..},1)=>scalar(schema,&path[1..]),
  _=>Err(absent()),
 }
}
fn widget_key<'a>(widget:&'a Widget,path:&[usize],index:usize)->Result<&'a str,ValueError>{
 if let Widget::Cluster{tree,..}=widget{return tree_key(tree,path,index)}
 if let Widget::Neuron{params,..}=widget{if path.first()==Some(&1){return params.object_key_at_path(&path[1..],index)}}
 if !path.is_empty(){return Err(absent())}
 match widget{Widget::Neuron{..}=>key(&["operatorId","params"],index),Widget::InputSlider{..}=>key(&["number"],index),Widget::InputNote{..}=>key(&["text"],index),Widget::InputImage{..}=>key(&["source"],index),Widget::Variable{..}=>key(&["name","schema"],index),_=>Err(absent())}
}
fn neuron_node<'a>(neuron:&'a Neuron,path:&[usize])->Result<Node<'a>,ValueError>{
 if path.is_empty(){return Ok(Node::Object(4))}
 match path[0]{0=>scalar(&neuron.id,&path[1..]),1=>scalar(&neuron.kind,&path[1..]),2=>neuron.params.node_at_path(&path[1..]),3=>match neuron.tree.as_deref(){Some(tree)=>tree_node(tree,&path[1..]),None if path.len()==1=>Ok(Node::Null),None=>Err(absent())},_=>Err(absent())}
}
fn neuron_key<'a>(neuron:&'a Neuron,path:&[usize],index:usize)->Result<&'a str,ValueError>{
 if path.is_empty(){return key(&["id","kind","params","tree"],index)}
 match path[0]{2=>neuron.params.object_key_at_path(&path[1..],index),3=>tree_key(neuron.tree.as_deref().ok_or_else(absent)?,&path[1..],index),_=>Err(absent())}
}
fn synapse_node<'a>(id:&'a str,from:&'a str,to:&'a str,from_port:&'a str,to_port:&'a str,path:&[usize])->Result<Node<'a>,ValueError>{if path.is_empty(){Ok(Node::Object(5))}else if path.len()==1{[id,from,to,from_port,to_port].get(path[0]).copied().map(Node::String).ok_or_else(absent)}else{Err(absent())}}
fn synapse_key(path:&[usize],index:usize)->Result<&'static str,ValueError>{if path.is_empty(){key(&["id","from","to","fromPort","toPort"],index)}else{Err(absent())}}
fn tree_node<'a>(tree:&'a Tree,path:&[usize])->Result<Node<'a>,ValueError>{
 if path.is_empty(){return Ok(Node::Object(2))}
 match path[0]{
  0 if path.len()==1=>Ok(Node::Array(tree.neurons.len())),0=>neuron_node(tree.neurons.get(path[1]).ok_or_else(absent)?,&path[2..]),
  1 if path.len()==1=>Ok(Node::Array(tree.synapses.len())),1=>{let row=tree.synapses.get(path[1]).ok_or_else(absent)?;synapse_node(&row.id,&row.from,&row.to,&row.from_port,&row.to_port,&path[2..])},
  _=>Err(absent()),
 }
}
fn tree_key<'a>(tree:&'a Tree,path:&[usize],index:usize)->Result<&'a str,ValueError>{
 if path.is_empty(){return key(&["neurons","synapses"],index)}
 if path.len()<2{return Err(absent())}
 match path[0]{0=>neuron_key(tree.neurons.get(path[1]).ok_or_else(absent)?,&path[2..],index),1=>synapse_key(&path[2..],index),_=>Err(absent())}
}
impl JsonWriteSource for FlowDependencyView<'_>{
 fn node_at_path(&self,path:&[usize])->Result<Node<'_>,ValueError>{
  if path.is_empty(){return Ok(Node::Object(2))}
  match path[0]{
   0 if path.len()==1=>Ok(Node::Object(self.widgets.len())),0=>widget_node(&self.source.widgets()[*self.widgets.get(path[1]).ok_or_else(absent)?],&path[2..]),
   1 if path.len()==1=>Ok(Node::Object(self.synapses.len())),1=>{let row=&self.source.synapses()[*self.synapses.get(path[1]).ok_or_else(absent)?];synapse_node(&row.id,&row.from,&row.to,&row.from_port,&row.to_port,&path[2..])},
   _=>Err(absent()),
  }
 }
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if path.is_empty(){return key(&["widgets","synapses"],index)}
  match path[0]{
   0 if path.len()==1=>self.widgets.get(index).map(|index|widget_id_for(&self.source.widgets()[*index])).ok_or_else(absent),
   0=>widget_key(&self.source.widgets()[*self.widgets.get(path[1]).ok_or_else(absent)?],&path[2..],index),
   1 if path.len()==1=>self.synapses.get(index).map(|index|self.source.synapses()[*index].id.as_str()).ok_or_else(absent),
   1=>synapse_key(&path[2..],index),
   _=>Err(absent()),
  }
 }
}
