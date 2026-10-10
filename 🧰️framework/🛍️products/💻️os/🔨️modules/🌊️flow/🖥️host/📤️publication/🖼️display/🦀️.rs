//! 🖼️ Original channel views serialize through admitted ranked metadata and the canonical writer.
use super::*;
use semio_framework_pack_json::{JsonWriteSource,JsonWriteNode,JsonBorrowedWriteCursor};
#[derive(semio_framework_value::RetireOwned)]
struct Field{kind:u8,rank:usize}
#[derive(semio_framework_value::RetireOwned)]
struct Row{widget:usize,fields:Vec<Field>}
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FlowDisplayCursor{control:Option<semio_framework_value::NativeEncodeContinuation>,rows:Vec<Row>,writer:JsonBorrowedWriteCursor,text:Option<String>,widget:usize,scan:usize,rank:usize,stage:u8}
struct OriginalDisplay<'a>{snapshot:&'a FlowHostSnapshot,channels:&'a EvalChannels,infos:&'a HistoryFoldIndex<String,OperatorInfo>,rows:&'a[Row]}
impl<'a> OriginalDisplay<'a>{
 fn input(&self,widget:usize)->Option<&'a Dictionary>{self.channels.inputs.get(widget_id_for(&self.snapshot.widgets[widget]))}
 fn params(&self,widget:usize)->Option<&'a Dictionary>{match &self.snapshot.widgets[widget]{Widget::Neuron{params,..}=>Some(params),_=>None}}
 fn info(&self,widget:usize)->Option<&'a OperatorInfo>{match &self.snapshot.widgets[widget]{Widget::Neuron{neuron_kind,..}=>self.infos.get(neuron_kind),_=>None}}
 fn merged(&self,widget:usize,key:&str)->Option<&'a NeuralValue>{self.params(widget).and_then(|dict|dict.get(key)).or_else(||self.input(widget).and_then(|dict|dict.get(key)))}
 fn slots(&self,widget:usize)->Option<&'a Dictionary>{self.info(widget)?.variadic_input.as_ref().and_then(|port|self.merged(widget,&port.slot_key)).and_then(NeuralValue::as_dictionary)}
 fn field(&self,row:&Row,index:usize)->Result<(&'a str,&'a NeuralValue),ValueError>{
  let field=row.fields.get(index).ok_or_else(absent)?;
  match field.kind{0=>self.input(row.widget).and_then(|dict|dict.entry_at_rank(field.rank)).map(|(key,value)|(key.as_str(),value)).ok_or_else(absent),1=>self.params(row.widget).and_then(|dict|dict.entry_at_rank(field.rank)).map(|(key,value)|(key.as_str(),value)).ok_or_else(absent),2=>self.slots(row.widget).and_then(|dict|dict.entry_at_rank(field.rank)).map(|(key,value)|(key.as_str(),value)).ok_or_else(absent),3=>{let key=&self.info(row.widget).and_then(|info|info.inputs.get(field.rank)).ok_or_else(absent)?.name;Ok((key,self.merged(row.widget,key).ok_or_else(absent)?))},_=>Err(absent())}
 }
 fn output(&self,row:&Row)->Option<&'a Dictionary>{self.channels.outputs.get(widget_id_for(&self.snapshot.widgets[row.widget]))}
 fn error(&self,row:&Row)->Option<&'a str>{self.output(row)?.get("error")?.as_atom()?.as_str()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original display source rank is absent")}
fn value_node<'a>(value:&'a NeuralValue,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{
 if let Some(dictionary)=value.as_dictionary(){return dictionary.node_at_path(path)}
 if !path.is_empty(){return Err(absent())}
 Ok(match value{NeuralValue::Atom(Atom::Null)=>JsonWriteNode::Null,NeuralValue::Atom(Atom::Boolean(value))=>JsonWriteNode::Bool(*value),NeuralValue::Atom(Atom::Integer(value))=>JsonWriteNode::Number(semio_framework_value::Number::Int(*value)),NeuralValue::Atom(Atom::Decimal(value))=>JsonWriteNode::Number(semio_framework_value::Number::Float(*value)),NeuralValue::Atom(Atom::String(value))=>JsonWriteNode::String(value),_=>return Err(absent())})
}
impl JsonWriteSource for OriginalDisplay<'_>{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{
  if path.is_empty(){return Ok(JsonWriteNode::Object(self.rows.len()))}
  let row=self.rows.get(path[0]).ok_or_else(absent)?;
  if path.len()==1{return Ok(JsonWriteNode::Object(2+usize::from(self.error(row).is_some())))}
  match path[1]{0=>{if path.len()==2{return Ok(JsonWriteNode::Object(row.fields.len()))}value_node(self.field(row,path[2])?.1,&path[3..])},1=>{match self.output(row){Some(dict)=>dict.node_at_path(&path[2..]),None if path.len()==2=>Ok(JsonWriteNode::Object(0)),None=>Err(absent())}},2 if path.len()==2=>self.error(row).map(JsonWriteNode::String).ok_or_else(absent),_=>Err(absent())}
 }
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if path.is_empty(){return self.rows.get(index).map(|row|widget_id_for(&self.snapshot.widgets[row.widget])).ok_or_else(absent)}
  let row=self.rows.get(path[0]).ok_or_else(absent)?;
  if path.len()==1{return ["in","out","error"].get(index).copied().ok_or_else(absent)}
  match path[1]{0 if path.len()==2=>Ok(self.field(row,index)?.0),0=>self.field(row,path[2])?.1.as_dictionary().ok_or_else(absent)?.object_key_at_path(&path[3..],index),1=>self.output(row).ok_or_else(absent)?.object_key_at_path(&path[2..],index),_=>Err(absent())}
 }
}
impl FlowDisplayCursor{
 pub(super) fn new()->Self{Self{control:None,rows:Vec::new(),writer:JsonBorrowedWriteCursor::new(),text:None,widget:0,scan:0,rank:0,stage:0}}
 pub(super) fn complete(&self)->bool{self.stage==255}
 pub(super) fn take_text(&mut self)->Option<String>{self.text.take()}
 fn source<'a>(&'a self,snapshot:&'a FlowHostSnapshot,channels:&'a EvalChannels,infos:&'a HistoryFoldIndex<String,OperatorInfo>)->OriginalDisplay<'a>{OriginalDisplay{snapshot,channels,infos,rows:&self.rows}}
 pub(super) fn demands(&self,snapshot:&FlowHostSnapshot,channels:&EvalChannels,infos:&HistoryFoldIndex<String,OperatorInfo>)->Result<RetirementDemand,ValueError>{
  let mut d=RetirementDemand{depth:1,..Default::default()};
  if self.stage==0{d.capacity_bytes=snapshot.widgets.len().checked_mul(std::mem::size_of::<Row>()).ok_or_else(absent)?;}
  if self.stage==2{let source=self.source(snapshot,channels,infos);let maximum=source.input(self.widget).map_or(0,Dictionary::len)+source.params(self.widget).map_or(0,Dictionary::len)+source.slots(self.widget).map_or(0,Dictionary::len)+source.info(self.widget).map_or(0,|info|info.inputs.len());d.capacity_bytes=maximum.checked_mul(std::mem::size_of::<Field>()).ok_or_else(absent)?;}
  if self.stage==7{return self.writer.normal_step_demands(&self.source(snapshot,channels,infos))}
  Ok(d)
 }
 pub(super) fn step(&mut self,snapshot:&FlowHostSnapshot,channels:&EvalChannels,infos:&HistoryFoldIndex<String,OperatorInfo>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let d=self.demands(snapshot,channels,infos)?;if grant.maximum_items==0{return Ok(Default::default())}if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original display requires source depth"))}if grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes{return Ok(Default::default())}
  let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};
  match self.stage{
   0=>{self.rows.try_reserve_exact(snapshot.widgets.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original display roster birth refused"))?;p.retained_capacity_bytes=self.rows.capacity()*std::mem::size_of::<Row>();self.stage=1;self.scan=1;},
   1=>{if self.widget==snapshot.widgets.len(){self.stage=7;}else if self.scan==snapshot.widgets.len(){self.stage=2;}else if widget_id_for(&snapshot.widgets[self.widget])==widget_id_for(&snapshot.widgets[self.scan]){self.widget+=1;self.scan=self.widget+1;}else{self.scan+=1;}},
   2=>{let count=d.capacity_bytes/std::mem::size_of::<Field>();let mut fields=Vec::new();fields.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original display port roster birth refused"))?;p.retained_capacity_bytes=fields.capacity()*std::mem::size_of::<Field>();self.rows.push(Row{widget:self.widget,fields});self.rank=0;self.stage=if self.source(snapshot,channels,infos).info(self.widget).is_some(){5}else{3};},
   3=>{let source=self.source(snapshot,channels,infos);if let Some((key,_))=source.input(self.widget).and_then(|dict|dict.entry_at_rank(self.rank)){let keep=source.params(self.widget).is_none_or(|params|params.get(key).is_none());if keep{self.rows.last_mut().unwrap().fields.push(Field{kind:0,rank:self.rank});}self.rank+=1;}else{self.rank=0;self.stage=4;}},
   4=>{if self.source(snapshot,channels,infos).params(self.widget).and_then(|dict|dict.entry_at_rank(self.rank)).is_some(){self.rows.last_mut().unwrap().fields.push(Field{kind:1,rank:self.rank});self.rank+=1;}else{self.next_widget();}},
   5=>{if self.source(snapshot,channels,infos).slots(self.widget).and_then(|dict|dict.entry_at_rank(self.rank)).is_some(){self.stage=8;self.scan=0;}else{self.stage=6;self.rank=0;}},
   6=>{let source=self.source(snapshot,channels,infos);if let Some(port)=source.info(self.widget).and_then(|info|info.inputs.get(self.rank)){let keep=port.name!="*"&&source.merged(self.widget,&port.name).is_some();if keep{self.rows.last_mut().unwrap().fields.push(Field{kind:3,rank:self.rank});}self.rank+=1;}else{self.next_widget();}},
   8=>{let source=self.source(snapshot,channels,infos);let key=source.slots(self.widget).and_then(|dict|dict.entry_at_rank(self.rank)).map(|(key,_)|key.as_str()).ok_or_else(absent)?;if let Some(port)=source.info(self.widget).and_then(|info|info.inputs.get(self.scan)){let replaced=port.name==key&&source.merged(self.widget,key).is_some();if replaced{self.rank+=1;self.stage=5;}else{self.scan+=1;}}else{self.rows.last_mut().unwrap().fields.push(Field{kind:2,rank:self.rank});self.rank+=1;self.stage=5;}},
   7=>{let source=OriginalDisplay{snapshot,channels,infos,rows:&self.rows};let mut receive=|_|true;let mut control=match self.control.take(){Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,&mut receive)?,None=>semio_framework_value::NativeEncodeControl::new_retained(&mut receive)};let result=self.writer.step(&source,1,&mut control,grant);p=self.writer.normal_step_progress();self.control=Some(control.pause()?);if let Some(text)=result.map_err(|error|error.with_retained_progress(p))?{self.text=Some(text);self.stage=255;}},
   255=>return Ok(Default::default()),_=>return Err(absent())
  }Ok(p)
 }
 fn next_widget(&mut self){self.widget+=1;self.scan=self.widget+1;self.stage=1;self.rank=0;}
}
