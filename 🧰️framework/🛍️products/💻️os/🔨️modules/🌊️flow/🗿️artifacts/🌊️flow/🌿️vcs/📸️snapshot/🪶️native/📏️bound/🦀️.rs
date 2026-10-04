//! 📏️ Borrowed Flow native output bounds with separately admitted traversal backing.
use super::*;
use crate::store::sqlite_snapshot::{artifact::NativeEncodingBound,SqliteSnapshotControl,ValueError,ValueRefusalKind};
enum Item<'a>{Widget(&'a Widget),Dictionary(&'a Dictionary),Tree(&'a Tree),Gui(&'a FlowGui)}
fn text(value:&str,bound:&mut NativeEncodingBound<'_,'_>)->Result<(),ValueError>{let bytes=value.len().checked_mul(16).and_then(|bytes|bytes.checked_add(64)).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Flow native literal bound overflow"))?;bound.add(bytes)}
fn strings(values:&[String],bound:&mut NativeEncodingBound<'_,'_>)->Result<(),ValueError>{for value in values{text(value,bound)?;}Ok(())}
fn synapse(id:&str,from:&str,to:&str,from_port:&str,to_port:&str,bound:&mut NativeEncodingBound<'_,'_>)->Result<(),ValueError>{for value in [id,from,to,from_port,to_port]{text(value,bound)?;}bound.add(256)}
/// 🌊️ Bounds the real persisted literals and record syntax without constructing a metadata mirror.
pub(super) fn preflight(value:&FlowHostSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let mut bound=NativeEncodingBound::new(control)?;bound.add(4096)?;text(&value.schema,&mut bound)?;bound.add(3*128)?;
 for(key,_)in &value.layout{text(key,&mut bound)?;bound.add(2*128)?;}
 for value in &value.synapses{synapse(&value.id,&value.from,&value.to,&value.from_port,&value.to_port,&mut bound)?;}
 let mut frontier=bound.allocate_frontier(value.widgets.len())?;for value in &value.widgets{frontier.push(Item::Widget(value));}
 let mut work=0usize;
 while let Some(value)=frontier.pop(){work=work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow native borrowed work overflow"))?;bound.check_rows(work)?;bound.checkpoint()?;
  match value{
   Item::Dictionary(value)=>{bound.add(256)?;for(key,value)in value.iter(){text(key,&mut bound)?;bound.add(256)?;match value{NeuralValue::Atom(Atom::String(value))=>text(value,&mut bound)?,NeuralValue::Dictionary(value)=>bound.push_frontier(&mut frontier,Item::Dictionary(value))?,_=>bound.add(128)?,}}}
   Item::Tree(value)=>{bound.add(256)?;for value in &value.synapses{synapse(&value.id,&value.from,&value.to,&value.from_port,&value.to_port,&mut bound)?;}for neuron in &value.neurons{text(&neuron.id,&mut bound)?;text(&neuron.kind,&mut bound)?;bound.add(256)?;bound.push_frontier(&mut frontier,Item::Dictionary(&neuron.params))?;if let Some(value)=&neuron.tree{bound.push_frontier(&mut frontier,Item::Tree(value))?;}}}
   Item::Gui(value)=>{bound.add(3*128+256)?;for(key,node)in &value.nodes{text(key,&mut bound)?;bound.add(2*128+256)?;match &node.chrome{NodeChrome::Plain{..}=>bound.add(128)?,NodeChrome::Slider{label,..}=>{text(label,&mut bound)?;bound.add(4*128)?;},NodeChrome::Note{text:value}=>text(value,&mut bound)?,NodeChrome::Image{src}=>text(src,&mut bound)?,NodeChrome::Variable{name,schema}=>{text(name,&mut bound)?;text(schema,&mut bound)?;}}}for preview in &value.previews{text(&preview.id,&mut bound)?;text(&preview.mode,&mut bound)?;bound.add(256)?;if let Some(source)=&preview.source{text(&source.neuron,&mut bound)?;text(&source.channel,&mut bound)?;}for path in &preview.expanded{text(path,&mut bound)?;}if preview.layout.is_some(){bound.add(2*128)?;}bound.push_frontier(&mut frontier,Item::Dictionary(&preview.preview))?;}}
   Item::Widget(value)=>{bound.add(512)?;match value{
    Widget::Neuron{id,neuron_kind,params,input_ports,output_ports,..}=>{text(id,&mut bound)?;text(neuron_kind,&mut bound)?;strings(input_ports,&mut bound)?;strings(output_ports,&mut bound)?;bound.push_frontier(&mut frontier,Item::Dictionary(params))?;},
    Widget::InputSlider{id,label,..}=>{text(id,&mut bound)?;text(label,&mut bound)?;bound.add(4*128)?;},
    Widget::InputNote{id,text:value}=>{text(id,&mut bound)?;text(value,&mut bound)?;},
    Widget::InputImage{id,src}=>{text(id,&mut bound)?;text(src,&mut bound)?;},
    Widget::Variable{id,name,schema}=>{text(id,&mut bound)?;text(name,&mut bound)?;text(schema,&mut bound)?;},
    Widget::OutputPreview{id,preview,expanded}=>{text(id,&mut bound)?;for path in expanded{text(path,&mut bound)?;}bound.push_frontier(&mut frontier,Item::Dictionary(preview))?;},
    Widget::OutputAction{id,action}=>{text(id,&mut bound)?;text(action,&mut bound)?;},
    Widget::OutputExport{id,format}=>{text(id,&mut bound)?;text(format,&mut bound)?;},
    Widget::Cluster{id,name,tree,flow}=>{text(id,&mut bound)?;text(name,&mut bound)?;bound.push_frontier(&mut frontier,Item::Tree(tree))?;bound.push_frontier(&mut frontier,Item::Gui(flow))?;}
   }}
  }
 }
 bound.finish()
}
