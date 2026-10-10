//! 🎞️ Bounded Pack tick emission from the immutable original writer, including virtual native columns.
use crate::{ToolRunTickWriter,ToolRunIdentity,ToolRunProgress,ToolRunStep,ToolRunStepArg,ToolRunTraceOp as Op,ToolRunTraceSubject as Subject,TOOL_RUN_TICK_BYTES_MAX,TOOL_RUN_TRACE_PAGE_BYTES_MAX};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress}};

#[derive(Clone,Copy)]
enum Node{Tick,Identity(u8,usize),Progress,Step(bool,usize),Steps(bool),Ops,Traces,Page(usize),PageBody(usize),Column(usize,usize),Bytes(usize),Entities,Counters,Args(bool,usize)}
#[derive(Clone,Copy)]
struct Frame{node:Node,phase:u8,index:usize,offset:usize}
impl Frame{const fn new(node:Node)->Self{Self{node,phase:0,index:0,offset:0}}}

/// 📬️ Keeps each wire chunk in its original inline slot until the caller acknowledges its paid append.
pub struct ToolRunTickWireCursor{original_address:usize,frames:[Frame;8],depth:usize,bytes:[u8;64],length:usize,total:usize,columns:[usize;9],done:bool,closed:bool}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn size(mut value:u64)->usize{let mut count=1;while value>=128{value>>=7;count+=1;}count}
fn identity_size(identity:ToolRunIdentity)->usize{1+4+3+size(u64::from(identity.id.app_instance_id))+size(identity.id.run)+size(u64::from(identity.generation))+1+size(32)+32}
fn column_size(op:Op,column:usize)->usize{match(op,column){(_,0)=>1,(Op::Upsert{..}|Op::Retire{..},1)=>8,(Op::Upsert{..},2|4)=>1,(Op::Upsert{..},3)=>2,(Op::Upsert{subject:Subject::Instance3d{..},..},5)=>4,(Op::Upsert{subject:Subject::Placement2d{..},..},6)=>4,(Op::Upsert{subject:Subject::Entity{..},..},7)=>8,(Op::Upsert{subject:Subject::Instance3d{..},..},8)=>32,(Op::Upsert{subject:Subject::Placement2d{..},..},8)=>12,_=>0}}
fn op_column_byte(op:Op,column:usize,offset:usize)->u8{match(op,column){
 (Op::Upsert{..},0)=>0,(Op::Retire{..},0)=>1,(Op::Clear,0)=>2,
 (Op::Upsert{key,..}|Op::Retire{key},1)=>key.to_le_bytes()[offset],
 (Op::Upsert{verdict,..},2)=>verdict.ordinal(),(Op::Upsert{reason,..},3)=>reason.to_le_bytes()[offset],
 (Op::Upsert{subject,..},4)=>subject.ordinal(),
 (Op::Upsert{subject:Subject::Instance3d{mesh,..},..},5)=>mesh.to_le_bytes()[offset],
 (Op::Upsert{subject:Subject::Placement2d{shape,..},..},6)=>shape.to_le_bytes()[offset],
 (Op::Upsert{subject:Subject::Entity{entity},..},7)=>entity.to_le_bytes()[offset],
 (Op::Upsert{subject:Subject::Instance3d{position,rotation,scale,..},..},8)=>{let index=offset/4;let value=if index<3{position[index]}else if index<7{rotation[index-3]}else{scale};value.to_le_bytes()[offset%4]},
 (Op::Upsert{subject:Subject::Placement2d{position,rotation,..},..},8)=>{let index=offset/4;let value=if index<2{position[index]}else{rotation};value.to_le_bytes()[offset%4]},_=>unreachable!()
}}
fn page_ops(writer:&ToolRunTickWriter,index:usize)->Result<&[Op],ValueError>{if index<writer.pages.len(){Ok(&writer.pages[index].ops)}else if index==writer.pages.len()&&!writer.page_ops.is_empty(){Ok(&writer.page_ops)}else{Err(invalid("original tick page disappeared"))}}
fn page_identity(writer:&ToolRunTickWriter,index:usize)->ToolRunIdentity{writer.pages.get(index).map_or(writer.identity,|page|page.identity)}
fn page_number(writer:&ToolRunTickWriter,index:usize)->u32{writer.pages.get(index).map_or(writer.next_page,|page|page.page)}
fn progress(writer:&ToolRunTickWriter)->Result<&ToolRunProgress,ValueError>{writer.progress.as_ref().ok_or_else(||invalid("original tick progress disappeared"))}
fn step(writer:&ToolRunTickWriter,ring:bool,index:usize)->Result<&ToolRunStep,ValueError>{if ring{progress(writer)?.steps.steps.get(index)}else{writer.steps.get(index)}.ok_or_else(||invalid("original tick step disappeared"))}

impl ToolRunTickWireCursor{
 /// 🎟️ Captures only the original immutable address; no wire mirror or heap allocation is born.
 pub fn admit(writer:&ToolRunTickWriter,grant:Grant)->Option<(Self,Progress)>{if grant.maximum_items==0||grant.maximum_depth<8{return None;}Some((Self{original_address:writer as*const _ as usize,frames:[Frame::new(Node::Tick);8],depth:0,bytes:[0;64],length:0,total:0,columns:[0;9],done:false,closed:false},Progress{copied_items:1,..Default::default()}))}
 pub fn chunk(&self)->&[u8]{&self.bytes[..self.length]}
 pub fn is_complete(&self)->bool{self.done&&self.length==0}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.length==0}
 pub fn written_bytes(&self)->usize{self.total}
 /// 🤝️ Acknowledges the retained chunk only after its consumer has preserved its own original receipt.
 pub fn acknowledge_chunk(&mut self,grant:Grant)->Progress{if self.length==0||grant.maximum_items==0||grant.maximum_depth<8{return Default::default();}self.length=0;Progress{copied_items:1,..Default::default()}}
 fn queue(&mut self,bytes:&[u8]){self.bytes[..bytes.len()].copy_from_slice(bytes);self.length=bytes.len();}
 fn number(&mut self,mut value:u64){loop{let byte=(value&127)as u8;value>>=7;self.bytes[self.length]=byte|if value>0{128}else{0};self.length+=1;if value==0{break;}}}
 fn scalar(&mut self,id:usize,value:u64){self.queue(&[id as u8,4]);self.number(value);}
 fn prefix(&mut self,id:usize,node:Node,record:bool)->Result<(),ValueError>{if record{self.queue(&[id as u8,13]);}else{self.queue(&[id as u8]);}self.push(node)}
 fn push(&mut self,node:Node)->Result<(),ValueError>{if self.depth+1==self.frames.len(){return Err(invalid("original tick exceeds declared inline frames"));}self.depth+=1;self.frames[self.depth]=Frame::new(node);Ok(())}
 fn pop(&mut self){if self.depth==0{self.done=true;}else{self.depth-=1;}}
 fn bytes_prefix(&mut self,length:usize){self.queue(&[8]);self.number(length as u64);}
 fn measure_page(&mut self,writer:&ToolRunTickWriter,index:usize)->Result<bool,ValueError>{let ops=page_ops(writer,index)?;if ops.len()>crate::TOOL_RUN_TRACE_PAGE_OPS_MAX{return Err(invalid("original tick page exceeds operation limit"));}let position=self.frames[self.depth].index;if position<ops.len(){for column in 0..9{self.columns[column]+=column_size(ops[position],column);}self.frames[self.depth].index+=1;Ok(false)}else{Ok(true)}}
 fn page_size(&self,writer:&ToolRunTickWriter,index:usize)->usize{let mut length=1+1+1+1+identity_size(page_identity(writer,index))+1+1+size(u64::from(page_number(writer,index)));for column in self.columns{if column>0{length+=1+1+size(column as u64)+column;}}length}
 fn present(writer:&ToolRunTickWriter,id:usize)->bool{match id{1|2=>true,3=>writer.progress.is_some(),4=>!writer.steps.is_empty(),5=>!writer.pages.is_empty()||!writer.page_ops.is_empty(),6=>!writer.append_ops.is_empty(),7=>!writer.append_entities.is_empty(),8=>writer.retract_to.is_some(),9=>writer.payload.is_some(),_=>false}}
 /// 📏️ Quotes only the next actual output range; cursor and frame metadata copy no payload bytes.
 pub fn next_copy_byte_demand(&self,writer:&ToolRunTickWriter,maximum_bytes:usize)->Result<usize,ValueError>{
  if self.closed||self.done||self.length>0{return Ok(0)}let f=self.frames[self.depth];let scalar=|value:u64|2+size(value);let range=|remaining:usize|remaining.min(64).min(maximum_bytes.max(1));
  Ok(match f.node{
   Node::Tick=>if f.phase<2{1}else if f.index>9||!Self::present(writer,f.index){0}else{match f.index{1|3=>2,2=>scalar(writer.next_sequence),8=>scalar(u64::from(writer.retract_to.unwrap())),_=>1}},
   Node::Identity(which,index)=>{let value=match which{0=>writer.identity,1=>progress(writer)?.identity,_=>page_identity(writer,index)};if f.phase==0{1}else{match f.index{1=>scalar(u64::from(value.id.app_instance_id)),2=>scalar(value.id.run),3=>scalar(u64::from(value.generation)),4=>3,5=>range(32-f.offset),_=>0}}},
   Node::Progress=>{let v=progress(writer)?;if f.phase==0{1}else{match f.index{1=>2,2=>scalar(v.sequence),3=>scalar(u64::from(v.state.ordinal())),4=>scalar(u64::from(v.stage)),5=>scalar(v.completed),6=>v.total.map_or(0,scalar),7=>usize::from(!v.counters.is_empty()),8=>10,9=>scalar(u64::from(v.conflicts)),10=>usize::from(!v.steps.is_empty()),_=>0}}},
   Node::Step(ring,index)=>{let v=step(writer,ring,index)?;if f.phase==0{1}else{match f.index{1=>scalar(v.sequence),2=>scalar(u64::from(v.kind.ordinal())),3=>scalar(u64::from(v.stage)),4=>scalar(u64::from(v.reason)),5=>v.subject.map_or(0,scalar),6=>scalar(u64::from(v.repeat)),7=>usize::from(!v.args.is_empty()),_=>0}}},
   Node::Steps(ring)=>{let count=if ring{progress(writer)?.steps.len()}else{writer.steps.len()};if f.phase==0{1+size(count as u64)}else{usize::from(f.index<count)}},
   Node::Ops|Node::Traces=>{let count=if matches!(f.node,Node::Traces){writer.pages.len()+usize::from(!writer.page_ops.is_empty())}else{writer.append_ops.len()};if f.phase==0{1+size(count as u64)}else{0}},
   Node::Bytes(index)=>{let bytes=if index==usize::MAX{writer.payload.as_deref().ok_or_else(||invalid("original tick payload disappeared"))?}else{writer.append_ops.get(index).ok_or_else(||invalid("original tick operation disappeared"))?};if f.phase==0{1+size(bytes.len()as u64)}else{range(bytes.len()-f.offset)}},
   Node::Entities|Node::Counters|Node::Args(..)=>{let(width,count)=match f.node{Node::Entities=>(8,writer.append_entities.len()),Node::Counters=>(10,progress(writer)?.counters.len()),Node::Args(ring,index)=>(9,step(writer,ring,index)?.args.len()),_=>unreachable!()};if f.phase==0{1+size(count.checked_mul(width).ok_or_else(||invalid("original tick column overflow"))?as u64)}else if f.index<count{width}else{0}},
   Node::Page(index)=>if f.phase==0&&f.index>=page_ops(writer,index)?.len(){1+size(self.page_size(writer,index)as u64)}else{0},
   Node::PageBody(index)=>if f.phase<2{1}else{match f.index{1=>2,2=>scalar(u64::from(page_number(writer,index))),3..=11=>usize::from(self.columns[f.index-3]>0),_=>0}},
   Node::Column(index,column)=>{let ops=page_ops(writer,index)?;if f.phase==0{1+size(self.columns[column]as u64)}else if f.index<ops.len(){range(column_size(ops[f.index],column)-f.offset)}else{0}}
  })
 }
 /// ⏱️ Emits one structural event or an original payload range under the unchanged caller grant.
 pub fn advance(&mut self,writer:&ToolRunTickWriter,grant:Grant)->Result<Progress,ValueError>{
  if self.closed{return Err(invalid("original tick serializer is closed"));}if self.original_address!=writer as*const _ as usize{return Err(invalid("original tick writer address changed"));}if self.done||self.length>0{return Ok(Default::default());}let demand=self.next_copy_byte_demand(writer,grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand||grant.maximum_depth<8{return Ok(Default::default());}
  if self.total.checked_add(demand).is_none_or(|total|total>TOOL_RUN_TICK_BYTES_MAX){return Err(invalid("original tick requires its next bounded output unit"));}let frame=self.frames[self.depth];
  match frame.node{
   Node::Tick=>{if frame.phase==0{self.queue(&[0]);self.frames[self.depth].phase=1;}else if frame.phase==1{let count=(1..=9).filter(|id|Self::present(writer,*id)).count();self.queue(&[count as u8]);self.frames[self.depth].phase=2;self.frames[self.depth].index=1;}else if frame.index>9{self.pop();}else{self.frames[self.depth].index+=1;let id=frame.index;if Self::present(writer,id){match id{1=>self.prefix(id,Node::Identity(0,0),true)?,2=>self.scalar(id,writer.next_sequence),3=>self.prefix(id,Node::Progress,true)?,4=>self.prefix(id,Node::Steps(false),false)?,5=>self.prefix(id,Node::Traces,false)?,6=>self.prefix(id,Node::Ops,false)?,7=>self.prefix(id,Node::Entities,false)?,8=>self.scalar(id,u64::from(writer.retract_to.unwrap())),9=>self.prefix(id,Node::Bytes(usize::MAX),false)?,_=>unreachable!()}}}},
   Node::Identity(which,index)=>{let identity=match which{0=>writer.identity,1=>progress(writer)?.identity,_=>page_identity(writer,index)};if frame.phase==0{self.queue(&[4]);self.frames[self.depth].phase=1;self.frames[self.depth].index=1;}else{self.frames[self.depth].index+=1;match frame.index{1=>self.scalar(1,u64::from(identity.id.app_instance_id)),2=>self.scalar(2,identity.id.run),3=>self.scalar(3,u64::from(identity.generation)),4=>{self.queue(&[4,8,32]);},5=>{let count=demand;self.queue(&identity.base_revision[frame.offset..frame.offset+count]);self.frames[self.depth].offset+=count;if self.frames[self.depth].offset<32{self.frames[self.depth].index=5;}},_=>self.pop()}}},
   Node::Progress=>{let value=progress(writer)?;if value.counters.len()>crate::TOOL_RUN_COUNTERS_MAX{return Err(invalid("original tick counters exceed schema"));}if frame.phase==0{self.queue(&[(7+usize::from(value.total.is_some())+usize::from(!value.counters.is_empty())+usize::from(!value.steps.is_empty()))as u8]);self.frames[self.depth].phase=1;self.frames[self.depth].index=1;}else if frame.index>10{self.pop();}else{self.frames[self.depth].index+=1;match frame.index{1=>self.prefix(1,Node::Identity(1,0),true)?,2=>self.scalar(2,value.sequence),3=>self.scalar(3,u64::from(value.state.ordinal())),4=>self.scalar(4,u64::from(value.stage)),5=>self.scalar(5,value.completed),6=>{if let Some(total)=value.total{self.scalar(6,total);}},7=>{if !value.counters.is_empty(){self.prefix(7,Node::Counters,false)?;}},8=>{self.queue(&[8,5]);let bytes=f64::from(value.units_per_second).to_le_bytes();self.bytes[2..10].copy_from_slice(&bytes);self.length=10;},9=>self.scalar(9,u64::from(value.conflicts)),10=>{if !value.steps.is_empty(){self.prefix(10,Node::Steps(true),false)?;}},_=>unreachable!()}}},
   Node::Step(ring,index)=>{let value=step(writer,ring,index)?;if value.repeat==0||value.args.len()>crate::TOOL_RUN_STEP_ARGS_MAX{return Err(invalid("original tick step violates schema"));}if frame.phase==0{self.queue(&[(5+usize::from(value.subject.is_some())+usize::from(!value.args.is_empty()))as u8]);self.frames[self.depth].phase=1;self.frames[self.depth].index=1;}else if frame.index>7{self.pop();}else{self.frames[self.depth].index+=1;match frame.index{1=>self.scalar(1,value.sequence),2=>self.scalar(2,u64::from(value.kind.ordinal())),3=>self.scalar(3,u64::from(value.stage)),4=>self.scalar(4,u64::from(value.reason)),5=>{if let Some(subject)=value.subject{self.scalar(5,subject);}},6=>self.scalar(6,u64::from(value.repeat)),7=>{if !value.args.is_empty(){self.prefix(7,Node::Args(ring,index),false)?;}},_=>unreachable!()}}},
   Node::Steps(ring)=>{let count=if ring{progress(writer)?.steps.len()}else{writer.steps.len()};if frame.phase==0{self.queue(&[12]);self.number(count as u64);self.frames[self.depth].phase=1;}else if frame.index<count{self.frames[self.depth].index+=1;self.queue(&[13]);self.push(Node::Step(ring,frame.index))?;}else{self.pop();}},
   Node::Ops|Node::Traces=>{let traces=matches!(frame.node,Node::Traces);let count=if traces{writer.pages.len()+usize::from(!writer.page_ops.is_empty())}else{writer.append_ops.len()};if frame.phase==0{self.queue(&[12]);self.number(count as u64);self.frames[self.depth].phase=1;}else if frame.index<count{self.frames[self.depth].index+=1;let node=if traces{self.columns=[0;9];Node::Page(frame.index)}else{Node::Bytes(frame.index)};self.push(node)?;}else{self.pop();}},
   Node::Bytes(index)=>{let bytes=if index==usize::MAX{writer.payload.as_deref().ok_or_else(||invalid("original tick payload disappeared"))?}else{writer.append_ops.get(index).ok_or_else(||invalid("original tick operation disappeared"))?};if frame.phase==0{self.bytes_prefix(bytes.len());self.frames[self.depth].phase=1;}else if frame.offset<bytes.len(){let count=demand;self.queue(&bytes[frame.offset..frame.offset+count]);self.frames[self.depth].offset+=count;}else{self.pop();}},
   Node::Entities|Node::Counters|Node::Args(..)=>{let width=match frame.node{Node::Entities=>8,Node::Counters=>10,_=>9};let count=match frame.node{Node::Entities=>writer.append_entities.len(),Node::Counters=>progress(writer)?.counters.len(),Node::Args(ring,index)=>step(writer,ring,index)?.args.len(),_=>unreachable!()};if frame.phase==0{self.bytes_prefix(count.checked_mul(width).ok_or_else(||invalid("original tick native column overflow"))?);self.frames[self.depth].phase=1;}else if frame.index<count{match frame.node{Node::Entities=>self.queue(&writer.append_entities[frame.index].to_le_bytes()),Node::Counters=>{let value=progress(writer)?.counters[frame.index];self.queue(&value.counter.to_le_bytes());self.bytes[2..10].copy_from_slice(&value.value.to_le_bytes());self.length=10;},Node::Args(ring,index)=>{let value=step(writer,ring,index)?.args[frame.index];let(kind,bytes)=match value{ToolRunStepArg::Unsigned(value)=>(0,value.to_le_bytes()),ToolRunStepArg::Float(value)=>(1,value.to_le_bytes())};self.queue(&[kind]);self.bytes[1..9].copy_from_slice(&bytes);self.length=9;},_=>unreachable!()}self.frames[self.depth].index+=1;}else{self.pop();}},
   Node::Page(index)=>{if frame.phase==0{if self.measure_page(writer,index)?{let length=self.page_size(writer,index);if length>TOOL_RUN_TRACE_PAGE_BYTES_MAX{return Err(invalid("original tick page exceeds wire limit"));}self.bytes_prefix(length);self.frames[self.depth].phase=1;}}else if frame.phase==1{self.frames[self.depth].phase=2;self.push(Node::PageBody(index))?;}else{self.pop();}},
   Node::PageBody(index)=>{if frame.phase==0{self.queue(&[0]);self.frames[self.depth].phase=1;}else if frame.phase==1{self.queue(&[(2+self.columns.iter().filter(|bytes|**bytes>0).count())as u8]);self.frames[self.depth].phase=2;self.frames[self.depth].index=1;}else if frame.index>11{self.pop();}else{self.frames[self.depth].index+=1;match frame.index{1=>self.prefix(1,Node::Identity(2,index),true)?,2=>self.scalar(2,u64::from(page_number(writer,index))),column=>{if self.columns[column-3]>0{self.prefix(column,Node::Column(index,column-3),false)?;}}}}},
   Node::Column(index,column)=>{let ops=page_ops(writer,index)?;if frame.phase==0{self.bytes_prefix(self.columns[column]);self.frames[self.depth].phase=1;}else if frame.index<ops.len(){let length=column_size(ops[frame.index],column);for offset in 0..demand{self.bytes[offset]=op_column_byte(ops[frame.index],column,frame.offset+offset);}self.length=demand;self.frames[self.depth].offset+=demand;if self.frames[self.depth].offset==length{self.frames[self.depth].index+=1;self.frames[self.depth].offset=0;}}else{self.pop();}}
  }
  self.total+=self.length;Ok(Progress{copied_items:1,copied_bytes:self.length,..Default::default()})
 }
 /// ♻️ Cancellation clears only the paid inline chunk; the original writer remains borrowed and unchanged.
 pub fn close_step(&mut self,grant:Grant)->Progress{if self.closed||grant.maximum_items==0||grant.maximum_depth<8{return Default::default();}self.length=0;self.closed=true;Progress{copied_items:1,..Default::default()}}
}
impl Drop for ToolRunTickWireCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original tick wire cursor abandoned before explicit close");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
