//! \u{1f4cf}\uFE0F Workflow SQL semantic cells retain every identity, relationship and IEEE companion.
use super::{WorkflowSnapshot,WorkflowParameter,ValueError,native_list,native_record};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_dsl_record::{FieldValue,RecordValue};
fn invalid(message:&str)->ValueError{crate::workflow_invalid(message)}
struct Extent{bytes:usize,maximum:usize}
impl Extent{
 fn add(&mut self,n:usize)->Result<(),ValueError>{self.bytes=self.bytes.checked_add(n).filter(|n|*n<=self.maximum).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"Workflow semantic SQL cells exceed caller limit"))?;Ok(())}
 fn text(&mut self,s:&str)->Result<(),ValueError>{self.add(s.len())}
 fn ieee(&mut self,n:f64)->Result<(),ValueError>{self.add(if n.is_nan(){11}else if n.is_infinite(){32}else{22})}
}
pub(super) fn typed(value:&WorkflowSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{
 let mut e=Extent{bytes:0,maximum:control.limits().max_value_bytes};e.add(24)?;e.text(&value.schema)?;e.text(&value.graph.schema)?;control.checkpoint(phase,0,0)?;
 for(index,node)in value.graph.nodes.iter().enumerate(){e.add(24)?;for text in[&node.id,&node.plugin_id,&node.app_id,&node.label,&node.yields,&node.artifact_ref,&node.config_ref]{e.text(text)?;}for number in[node.x,node.y,node.width,node.height]{e.ieee(number)?;}
  for(bucket,ports)in[("input",&node.inputs),("output",&node.outputs)]{for(index,port)in ports.iter().enumerate(){e.add(64)?;for text in[bucket,&port.id,&port.spec.id,&port.spec.label]{e.text(text)?;}if let Some(kind)=&port.spec.kind_id{e.text(kind)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,ports.len())?;}}}
  if(index+1)%256==0{control.checkpoint(phase,index+1,value.graph.nodes.len())?;}
 }
 for(index,edge)in value.graph.edges.iter().enumerate(){e.add(24)?;for text in[&edge.id,&edge.source_node_id,&edge.source_port_id,&edge.target_node_id,&edge.target_port_id]{e.text(text)?;}let c=&edge.contract;e.add(32)?;e.text(&c.kind_id)?;let(kind,text)=match&c.wire{crate::MediaWireFormat::Binary{format_kind}=>("binary",format_kind),crate::MediaWireFormat::Document{schema}=>("document",schema),crate::MediaWireFormat::Intrinsic{schema}=>("intrinsic",schema)};e.text(kind)?;e.text(text)?;if c.conversion.is_some(){e.add(16)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.graph.edges.len())?;}}
 for(index,parameter)in value.parameters.iter().enumerate(){let(id,name,kind)=match parameter{WorkflowParameter::Numeric{id,name,..}=>(id,name,"numeric"),WorkflowParameter::Categorical{id,name,..}=>(id,name,"categorical"),WorkflowParameter::Toggle{id,name,..}=>(id,name,"toggle"),WorkflowParameter::Text{id,name,..}=>(id,name,"text")};e.add(24)?;for text in[id.as_str(),name,kind]{e.text(text)?;}
  match parameter{WorkflowParameter::Numeric{value,min,max,step,..}=>{e.add(16)?;e.ieee(*value)?;for value in[min,max,step]{if let Some(value)=value{e.ieee(*value)?;}}},WorkflowParameter::Categorical{value,options,..}=>{e.add(16)?;e.text(value)?;for(index,option)in options.iter().enumerate(){e.add(24)?;e.text(option)?;if(index+1)%256==0{control.checkpoint(phase,index+1,options.len())?;}}},WorkflowParameter::Toggle{..}=>e.add(24)?,WorkflowParameter::Text{value,..}=>{e.add(16)?;e.text(value)?;}}
  if(index+1)%256==0{control.checkpoint(phase,index+1,value.parameters.len())?;}
 }
 for(index,row)in value.parameter_bindings.iter().enumerate(){e.add(24)?;for text in[&row.parameter_id,&row.node_id,&row.field_path]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.parameter_bindings.len())?;}}
 for(index,row)in value.inputs.iter().enumerate(){e.add(40)?;for text in[&row.id,&row.kind_id,&row.selector]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.inputs.len())?;}}
 for(index,row)in value.input_bindings.iter().enumerate(){e.add(24)?;for text in[&row.input_id,&row.node_id,&row.port_id]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.input_bindings.len())?;}}
 for(index,row)in value.output_bindings.iter().enumerate(){e.add(24)?;for text in[&row.node_id,&row.port_id,&row.path_template]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.output_bindings.len())?;}}
 control.check_value_bytes(e.bytes)?;control.checkpoint(phase,0,0)
}
fn text(value:Option<&FieldValue>)->Result<&str,ValueError>{match value{Some(FieldValue::Text(s))=>Ok(s),_=>Err(invalid("Workflow semantic cells require literal Text"))}}
fn scalar(value:Option<&FieldValue>)->Result<f64,ValueError>{match value{Some(FieldValue::Float(n))=>Ok(*n),_=>Err(invalid("Workflow semantic IEEE cells require literal Float"))}}
fn optional(value:Option<&FieldValue>)->bool{!matches!(value,None|Some(FieldValue::Absent))}
fn record(value:Option<&FieldValue>)->Result<&RecordValue,ValueError>{native_record(value.ok_or_else(||invalid("Workflow semantic entity is missing"))?)}
pub(super) fn borrowed(value:&RecordValue,native:&mut semio_framework_value::NativeDecodeControl<'_>,maximum:usize)->Result<(),ValueError>{
 let mut e=Extent{bytes:0,maximum};e.add(24)?;e.text(text(value.get(0))?)?;let graph=record(value.get(1))?;e.text(text(graph.get(0))?)?;
 let nodes=native_list(graph.get(1))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(nodes.len())?;for node in nodes{let row=native_record(node)?;e.add(24)?;for field in 0..7{e.text(text(row.get(field))?)?;}for field in 7..11{e.ieee(scalar(row.get(field))?)?;}
  for(field,bucket)in[(11,"input"),(12,"output")]{let ports=native_list(row.get(field))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(ports.len())?;for port in ports{let p=native_record(port)?;e.add(64)?;e.text(bucket)?;for field in 0..3{e.text(text(p.get(field))?)?;}if optional(p.get(6)){e.text(text(p.get(6))?)?;}native.step()?;}Ok(())})?;}native.step()?;}Ok(())})?;
 let edges=native_list(graph.get(2))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(edges.len())?;for edge in edges{let row=native_record(edge)?;e.add(24)?;for field in 0..5{e.text(text(row.get(field))?)?;}let c=record(row.get(5))?;e.add(32)?;e.text(text(c.get(0))?)?;e.text(text(c.get(3))?)?;for field in[4,5]{if optional(c.get(field)){e.text(text(c.get(field))?)?;}}for field in[6,7]{if optional(c.get(field)){e.add(8)?;}}native.step()?;}Ok(())})?;
 let parameters=match value.get(2){Some(FieldValue::Statements(values))=>values.as_slice(),None|Some(FieldValue::Absent)=>&[],_=>return Err(invalid("Workflow semantic parameters require literal statements"))};
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(parameters.len())?;for(kind,p)in parameters{e.add(24)?;e.text(kind)?;for field in 0..2{e.text(text(p.get(field))?)?;}match kind.as_str(){"numeric"=>{e.add(16)?;e.ieee(scalar(p.get(2))?)?;for field in 3..6{if optional(p.get(field)){e.ieee(scalar(p.get(field))?)?;}}},"categorical"=>{e.add(16)?;e.text(text(p.get(2))?)?;let options=native_list(p.get(3))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(options.len())?;for option in options{e.add(24)?;e.text(text(Some(option))?)?;native.step()?;}Ok(())})?;},"toggle"=>e.add(24)?,"text"=>{e.add(16)?;e.text(text(p.get(2))?)?;},_=>return Err(invalid("Workflow semantic parameter branch is undeclared"))}native.step()?;}Ok(())})?;
 for field in 3..7{let rows=native_list(value.get(field))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows{let row=native_record(row)?;e.add(if field==4{40}else{24})?;for id in 0..3{e.text(text(row.get(id))?)?;}native.step()?;}Ok(())})?;}
 native.checkpoint()
}
