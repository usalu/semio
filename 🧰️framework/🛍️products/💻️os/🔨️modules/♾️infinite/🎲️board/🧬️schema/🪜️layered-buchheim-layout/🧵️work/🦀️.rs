//! 🧮️ Allocation-free scalar continuation of the original layered Buchheim geometry.
use super::super::{DagLayoutOptions,DagLayoutOrientation};

const NONE:usize=usize::MAX;
/// 🧷️ An indexed edge borrows the stable order supplied by its graph owner.
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub struct LayeredLayoutEdge{pub source:usize,pub target:usize}
/// 🌲️ One original node retains geometry and constant-time forest links without owned labels.
#[derive(Clone,Copy,Debug,semio_framework_value::RetireOwned)]
pub struct LayeredLayoutNode{incoming:usize,parent:usize,first:usize,last:usize,left:usize,right:usize,number:usize,depth:i32,x:f64,y:f64,modifier:f64,change:f64,shift:f64,inherited:f64,visited:bool}
impl LayeredLayoutNode{
 pub const fn new()->Self{Self{incoming:0,parent:NONE,first:NONE,last:NONE,left:NONE,right:NONE,number:0,depth:-1,x:-1.0,y:0.0,modifier:0.0,change:0.0,shift:0.0,inherited:0.0,visited:false}}
 pub fn position(&self)->(f64,f64){(self.x,self.y)}
}
impl Default for LayeredLayoutNode{fn default()->Self{Self::new()}}
/// 🚫️ Invalid indexed sources return scalar refusals without allocating diagnostic text.
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum LayeredLayoutFault{SourceChanged,MissingSuperNode,InvalidEndpoint,ArithmeticOverflow}
/// 🪜️ Each step mutates at most three fixed-size nodes and the original scalar cursor.
#[derive(semio_framework_value::RetireOwned)]
pub struct LayeredLayoutCursor{binding:Option<[u64;12]>,phase:u8,index:usize,round:usize,changed:bool,roots:bool,current:usize,walk:usize,contour:[usize;4],accum:[f64;2],level:i32,min_x:f64,bounds:[f64;4]}
impl LayeredLayoutCursor{
 pub const fn new()->Self{Self{binding:None,phase:0,index:0,round:0,changed:false,roots:false,current:0,walk:NONE,contour:[0;4],accum:[0.0;2],level:0,min_x:f64::INFINITY,bounds:[f64::INFINITY,f64::NEG_INFINITY,f64::INFINITY,f64::NEG_INFINITY]}}
 pub fn maximum_step_copy_bytes()->usize{std::mem::size_of::<Self>()+3*std::mem::size_of::<LayeredLayoutNode>()}
 pub fn step(&mut self,nodes:&mut[LayeredLayoutNode],edges:&[LayeredLayoutEdge],options:&DagLayoutOptions)->Result<bool,LayeredLayoutFault>{
  if nodes.is_empty(){return Err(LayeredLayoutFault::MissingSuperNode)}
  let binding=[nodes.as_ptr()as usize as u64,nodes.len()as u64,edges.as_ptr()as usize as u64,edges.len()as u64,options as*const _ as usize as u64,options.layer_spacing.to_bits(),options.sibling_gap.to_bits(),u64::from(options.center_x.is_some()),options.center_x.map_or(0,f64::to_bits),u64::from(options.center_y.is_some()),options.center_y.map_or(0,f64::to_bits),u64::from(matches!(options.orientation,DagLayoutOrientation::TopBottom))];
  if self.binding.is_some_and(|original|original!=binding){return Err(LayeredLayoutFault::SourceChanged)}
  self.binding=Some(binding);let count=nodes.len()-1;
  match self.phase{
   0=>{if count==0{self.phase=17;return Ok(true)}if self.index<nodes.len(){nodes[self.index]=LayeredLayoutNode::new();if self.index<count{nodes[self.index].parent=count}self.index+=1}else{self.phase=1;self.index=0}},
   1=>{if let Some(edge)=edges.get(self.index){if edge.source>=count||edge.target>=count{return Err(LayeredLayoutFault::InvalidEndpoint)}nodes[edge.target].incoming=nodes[edge.target].incoming.checked_add(1).ok_or(LayeredLayoutFault::ArithmeticOverflow)?;self.index+=1}else{self.phase=2;self.index=0}},
   2=>{if self.index<count{if nodes[self.index].incoming==0{nodes[self.index].depth=0;self.roots=true}self.index+=1}else{self.phase=if self.roots{4}else{3};self.index=0}},
   3=>{if self.index<count{nodes[self.index].depth=0;self.index+=1}else{self.phase=4;self.index=0}},
   4=>{if let Some(edge)=edges.get(self.index){let depth=nodes[edge.source].depth;if depth>=0{let next=depth.checked_add(1).ok_or(LayeredLayoutFault::ArithmeticOverflow)?;if nodes[edge.target].depth<next{nodes[edge.target].depth=next;self.changed=true}}self.index+=1}else{self.round+=1;if self.changed&&self.round<edges.len().saturating_add(count).saturating_add(4){self.index=0;self.changed=false}else{self.phase=5;self.index=0}}},
   5=>{if let Some(edge)=edges.get(self.index){if self.roots&&nodes[edge.target].incoming>0{let previous=nodes[edge.target].parent;let candidate=(nodes[edge.source].depth.max(0),edge.source);if previous==count||candidate<(nodes[previous].depth.max(0),previous){nodes[edge.target].parent=edge.source}}self.index+=1}else{self.phase=6;self.index=0}},
   6=>{if self.index<count{let child=self.index;let parent=nodes[child].parent;let previous=nodes[parent].last;nodes[child].left=previous;nodes[child].number=if previous==NONE{1}else{nodes[previous].number+1};if previous==NONE{nodes[parent].first=child}else{nodes[previous].right=child}nodes[parent].last=child;self.index+=1}else{self.current=count;self.phase=7}},
   7=>{let node=self.current;if nodes[node].first==NONE{nodes[node].x=if nodes[node].left==NONE{0.0}else{nodes[nodes[node].left].x+1.0};self.phase=8}else{self.current=nodes[node].first}},
   8=>{let node=self.current;if nodes[node].parent==NONE{self.current=count;self.accum[0]=0.0;self.level=0;self.phase=13}else if nodes[node].left==NONE{self.phase=10}else{let left=nodes[node].left;self.contour=[node,left,nodes[nodes[node].parent].first,node];self.accum=[nodes[node].modifier,nodes[left].modifier];self.phase=9}},
   9=>{let[vir,vil,vol,vor]=self.contour;let next_vil=nodes[vil].last;let next_vir=nodes[vir].first;if next_vil==NONE||next_vir==NONE{self.phase=10}else{let next_vol=nodes[vol].first;let next_vor=nodes[vor].last;if next_vol==NONE||next_vor==NONE{self.phase=10}else{let v=self.current;let ancestor=nodes[nodes[v].parent].first;let shift=(nodes[next_vil].x+self.accum[1])-(nodes[next_vir].x+self.accum[0])+1.0;if shift>0.0{let subtrees=(nodes[v].number as i32-nodes[ancestor].number as i32)as f64;if subtrees>0.0{nodes[v].change-=shift/subtrees;nodes[v].shift+=shift;nodes[ancestor].change+=shift/subtrees;nodes[v].x+=shift;nodes[v].modifier+=shift}self.accum[0]+=shift}self.accum[1]+=nodes[next_vil].modifier;self.accum[0]+=nodes[next_vir].modifier;self.contour=[next_vir,next_vil,next_vol,next_vor]}}},
   10=>{let node=self.current;if nodes[node].right!=NONE{self.current=nodes[node].right;self.phase=7}else{self.current=nodes[node].parent;self.walk=nodes[self.current].last;self.accum=[0.0;2];self.phase=11}},
   11=>{if self.walk==NONE{self.phase=12}else{let node=self.walk;nodes[node].x+=self.accum[0];nodes[node].modifier+=self.accum[0];self.accum[1]+=nodes[node].change;self.accum[0]+=nodes[node].shift+self.accum[1];self.walk=nodes[node].left}},
   12=>{let node=self.current;let midpoint=(nodes[nodes[node].first].x+nodes[nodes[node].last].x)*0.5;if nodes[node].left==NONE{nodes[node].x=midpoint}else{nodes[node].x=nodes[nodes[node].left].x+1.0;nodes[node].modifier=nodes[node].x-midpoint}self.phase=8},
   13=>{let node=self.current;nodes[node].x+=self.accum[0];nodes[node].inherited=self.accum[0];nodes[node].y=f64::from(self.level);nodes[node].visited=true;self.min_x=self.min_x.min(nodes[node].x);if nodes[node].first==NONE{self.phase=14}else{self.accum[0]+=nodes[node].modifier;self.level=self.level.checked_add(1).ok_or(LayeredLayoutFault::ArithmeticOverflow)?;self.current=nodes[node].first}},
   14=>{let node=self.current;if nodes[node].right!=NONE{self.current=nodes[node].right;self.phase=13}else if nodes[node].parent!=NONE{let parent=nodes[node].parent;self.current=parent;self.accum[0]=nodes[parent].inherited;self.level=self.level.checked_sub(1).ok_or(LayeredLayoutFault::ArithmeticOverflow)?}else{self.phase=15;self.index=0}},
   15=>{if self.index<nodes.len(){if self.min_x.is_finite()&&self.min_x<0.0&&nodes[self.index].visited{nodes[self.index].x-=self.min_x}self.index+=1}else{self.phase=16;self.index=0}},
   16=>{if self.index<count{let node=&nodes[self.index];self.bounds[0]=self.bounds[0].min(node.x);self.bounds[1]=self.bounds[1].max(node.x);self.bounds[2]=self.bounds[2].min(node.y);self.bounds[3]=self.bounds[3].max(node.y);self.index+=1}else{self.phase=18;self.index=0}},
   17=>return Ok(true),
   18=>{if self.index<count{let cx=(self.bounds[0]+self.bounds[1])*0.5;let cy=(self.bounds[2]+self.bounds[3])*0.5;let gx=options.center_x.unwrap_or(0.0);let gy=options.center_y.unwrap_or(0.0);let node=&mut nodes[self.index];let(x,y)=match options.orientation{DagLayoutOrientation::LeftRight=>(node.y*options.layer_spacing+(gx-cy*options.layer_spacing),node.x*options.sibling_gap+(gy-cx*options.sibling_gap)),DagLayoutOrientation::TopBottom=>(node.x*options.sibling_gap+(gx-cx*options.sibling_gap),node.y*options.layer_spacing+(gy-cy*options.layer_spacing))};node.x=x;node.y=y;self.index+=1}else{self.phase=17;return Ok(true)}},
   _=>return Err(LayeredLayoutFault::SourceChanged),
  }Ok(false)
 }
}
impl Default for LayeredLayoutCursor{fn default()->Self{Self::new()}}
#[cfg(test)]
#[path="🧪️.rs"]
mod laws;
