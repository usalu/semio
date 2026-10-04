//! 🧠️ Controlled construction of the actual neural tree owner family.
use super::*;
use semio_framework_value::{DecodedValue,NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
use std::mem::{replace,size_of,take};
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn object(value:&DslValue)->Result<&DslValue>{if matches!(value,DslValue::Object(_)){Ok(value)}else{Err(invalid("neural native object required"))}}
fn array(value:Option<&DslValue>)->Result<&[DslValue]>{match value{None=>Ok(&[]),Some(DslValue::Array(values))=>Ok(values),_=>Err(invalid("neural native array required"))}}
fn string(value:&DslValue,key:&str,control:&mut NativeDecodeControl<'_>)->Result<String>{String::from_value_controlled(value.get(key).ok_or_else(||invalid("required neural native string missing"))?,control)}
fn optional_string(value:&DslValue,key:&str,control:&mut NativeDecodeControl<'_>)->Result<String>{match value.get(key){Some(value)=>String::from_value_controlled(value,control),None=>{control.checkpoint()?;Ok(String::new())}}}
fn push(output:&mut Vec<(String,DslValue)>,key:&str,value:DslValue,control:&mut NativeEncodeControl<'_>)->Result<()>{let value=value.guard_decoded();let key=control.copy_text(key)?;output.push((key,value.take()));control.step()}
fn encoded_object(count:usize,control:&mut NativeEncodeControl<'_>)->Result<DecodedValue<Vec<(String,DslValue)>>>{Ok(DecodedValue::new(control.allocate_vec(count)?,|items|{for(_,value)in items{DslValue::retire_decoded(value)}}))}
/// ♻️ Traverses actual child boxes using their existing storage for parent linkage.
pub(super) fn retire_tree(mut value:Tree){let mut parents:Option<Box<Tree>>=None;loop{
 if let Some(mut neuron)=value.neurons.pop(){take(&mut neuron.params).retire_cold();if let Some(mut child)=neuron.tree.take(){let next=replace(&mut *child,value);neuron.tree=parents.take();child.neurons.push(neuron);parents=Some(child);value=next;}}
 else{drop(value);let Some(parent)=parents.take()else{break};value=*parent;let mut marker=value.neurons.pop().expect("neural cold parent marker remains owned");parents=marker.tree.take();marker.params.retire_cold();}
}}
/// ♻️ Retires one actual neuron and its optional child tree without a recursive destructor.
pub(super) fn retire_neuron(value:Neuron){value.params.retire_cold();if let Some(tree)=value.tree{retire_tree(*tree);}}
fn retire_neurons(values:Vec<Neuron>){for value in values{retire_neuron(value)}}
/// 🛫️ Copies all five actual wire literals through cumulative ownership admission.
pub(super) fn encode_synapse(value:&Synapse,control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(5)?;let mut output=encoded_object(5,control)?;for(key,value)in [("id",&value.id),("from",&value.from),("to",&value.to),("fromPort",&value.from_port),("toPort",&value.to_port)]{let value=value.to_value_controlled(control)?;push(output.get_mut(),key,value,control)?;}Ok(DslValue::Object(output.take()))}))}
/// 🛬️ Copies borrowed synapse fields directly into their actual first-party owner.
pub(super) fn decode_synapse(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Synapse>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let value=object(value)?;Ok(Synapse{id:string(value,"id",control)?,from:string(value,"from",control)?,to:string(value,"to",control)?,from_port:optional_string(value,"fromPort",control)?,to_port:optional_string(value,"toPort",control)?})}))}
/// 🛫️ Encodes the actual neuron, exact dictionary and optional child tree.
pub(super) fn encode_neuron(value:&Neuron,control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(4)?;let mut output=encoded_object(4,control)?;
 let id=value.id.to_value_controlled(control)?;push(output.get_mut(),"id",id,control)?;
 let kind=value.kind.to_value_controlled(control)?;push(output.get_mut(),"kind",kind,control)?;
 let params=value.params.to_value_controlled(control)?;push(output.get_mut(),"params",params,control)?;
 let tree=match &value.tree{Some(tree)=>encode_tree(tree,control)?,None=>().to_value_controlled(control)?};push(output.get_mut(),"tree",tree,control)?;
 Ok(DslValue::Object(output.take()))
}))}
/// 🛬️ Keeps every owned dictionary and child box guarded until neuron publication.
pub(super) fn decode_neuron(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Neuron>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let value=object(value)?;let id=string(value,"id",control)?;let kind=string(value,"kind",control)?;let params=ColdOwner::new(match value.get("params"){Some(value)=>Dictionary::from_value_controlled(value,control)?,None=>Dictionary::new()});let tree=match value.get("tree"){None|Some(DslValue::Null)=>None,Some(value)=>{let tree=DecodedValue::new(decode_tree(value,control)?,retire_tree);control.charge(size_of::<Tree>())?;Some(Box::new(tree.take()))}};Ok(Neuron{id,kind,params:params.into_inner(),tree})}))}
fn encode_neurons(values:&[Neuron],control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=Vec::<DslValue>::guard_decoded(control.allocate_vec(values.len())?);for value in values{output.get_mut().push(encode_neuron(value,control)?);control.step()?;}Ok(DslValue::Array(output.take()))})}
fn encode_synapses(values:&[Synapse],control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=Vec::<DslValue>::guard_decoded(control.allocate_vec(values.len())?);for value in values{output.get_mut().push(encode_synapse(value,control)?);control.step()?;}Ok(DslValue::Array(output.take()))})}
/// 🛫️ Encodes both actual graph collections without creating an ordinary mirror.
pub(super) fn encode_tree(value:&Tree,control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(2)?;let mut output=encoded_object(2,control)?;let neurons=encode_neurons(&value.neurons,control)?;push(output.get_mut(),"neurons",neurons,control)?;let synapses=encode_synapses(&value.synapses,control)?;push(output.get_mut(),"synapses",synapses,control)?;Ok(DslValue::Object(output.take()))}))}
/// 🛬️ Allocates exact actual graph vectors before copying any owned child fields.
pub(super) fn decode_tree(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Tree>{control.scoped_depth(64,|control|control.scoped_stage(|control|{let value=object(value)?;let neurons=array(value.get("neurons"))?;let synapses=array(value.get("synapses"))?;let work=neurons.len().checked_add(synapses.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"neural native graph work overflow"))?;control.begin_stage(work)?;let mut owned_neurons=DecodedValue::new(control.allocate_vec(neurons.len())?,retire_neurons);for(index,value)in neurons.iter().enumerate(){owned_neurons.get_mut().push(decode_neuron(value,control).map_err(|error|error.under(index))?);control.step()?;}let mut owned_synapses=control.allocate_vec(synapses.len())?;for(index,value)in synapses.iter().enumerate(){owned_synapses.push(decode_synapse(value,control).map_err(|error|error.under(index))?);control.step()?;}Ok(Tree{neurons:owned_neurons.take(),synapses:owned_synapses})}))}
