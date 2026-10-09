//! 📤️ Canonical JSON views borrow admitted semantic graph records under a retained operation lease.
use crate::infinite::board::schema::dag_input::{DagChannelDirection,output::{DagChannelView,DagOutputSource,DagOutputCandidates,DagRefusalView}};
use semio_framework_pack_json::{JsonWriteNode,JsonWriteSource};
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(Clone,Copy)]
pub enum DagOutputKind{Nodes,Edges,Selection,Channels,Hover}
pub struct DagOutputPreparation{kind:DagOutputKind,nodes:Vec<usize>,edges:Vec<usize>,channels:Vec<usize>,field:usize,index:usize,phase:u8,complete:bool,progress:semio_framework_value::retained_clone::RetainedCloneProgress}
impl DagOutputPreparation{
    pub fn new(kind:DagOutputKind)->Self{Self{kind,nodes:Vec::new(),edges:Vec::new(),channels:Vec::new(),field:match kind{DagOutputKind::Edges=>1,DagOutputKind::Channels=>2,_=>0},index:0,phase:0,complete:matches!(kind,DagOutputKind::Hover),progress:Default::default()}}
    pub fn normal_step_progress(&self)->semio_framework_value::retained_clone::RetainedCloneProgress{self.progress}
    pub fn step<S:DagOutputCandidates+?Sized>(&mut self,source:&S,units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<bool,ValueError>{
        self.progress=Default::default();if units==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}for _ in 0..1{control.checkpoint()?;if self.complete{return Ok(true)}let count=match self.field{0=>source.node_candidate_count(),1=>source.edge_candidate_count(),_=>source.channel_candidate_count()};let output=match self.field{0=>&mut self.nodes,1=>&mut self.edges,_=>&mut self.channels};
            let bytes=if self.phase==0{count.checked_mul(std::mem::size_of::<usize>()).ok_or_else(absent)?}else{0};if bytes>grant.maximum_capacity_bytes{return Ok(false)}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;let before=control.owned_bytes();
            if self.phase==0{*output=control.allocate_vec(count)?;self.phase=1;}
            else if self.index==count{self.field+=1;self.index=0;self.phase=0;self.complete=self.field==match self.kind{DagOutputKind::Nodes=>1,DagOutputKind::Edges=>2,_=>3};}
            else{let admitted=match self.field{0=>source.node_candidate_id(self.index).is_some(),1=>source.edge_candidate_id(self.index).is_some(),_=>source.channel_candidate(self.index).is_some()&&source.handle_candidate_identity(self.index).is_some()};if admitted{output.push(self.index)}self.index+=1;}
            self.progress.copied_items=1;self.progress.retained_capacity_bytes=control.owned_bytes()-before;control.step()?;
        }Ok(self.complete)
    }
    pub fn view<'a,S:DagOutputCandidates+?Sized>(&'a self,source:&'a S)->DagPreparedOutput<'a,S>{assert!(self.complete);DagPreparedOutput{source,prepared:self}}
}
pub struct DagPreparedOutput<'a,S:DagOutputCandidates+?Sized>{source:&'a S,prepared:&'a DagOutputPreparation}
impl<S:DagOutputCandidates+?Sized> DagOutputSource for DagPreparedOutput<'_,S>{
    fn selected_node_count(&self)->usize{self.prepared.nodes.len()}
    fn selected_node_id(&self,index:usize)->Option<&str>{self.source.node_candidate_id(*self.prepared.nodes.get(index)?)}
    fn selected_edge_count(&self)->usize{self.prepared.edges.len()}
    fn selected_edge_id(&self,index:usize)->Option<&str>{self.source.edge_candidate_id(*self.prepared.edges.get(index)?)}
    fn selected_channel_count(&self)->usize{self.prepared.channels.len()}
    fn selected_channel(&self,index:usize)->Option<DagChannelView<'_>>{self.source.channel_candidate(*self.prepared.channels.get(index)?)}
    fn selected_handle_identity(&self,index:usize)->Option<&str>{self.source.handle_candidate_identity(*self.prepared.channels.get(index)?)}
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>{self.source.hovered_channel_view()}
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>{self.source.wire_refusal_view()}
}
semio_framework_value::artifact_retire_leaf!(DagOutputKind);
semio_framework_value::artifact_retire_struct!(DagOutputPreparation{kind,nodes,edges,channels,field,index,phase,complete,progress});
pub struct DagJsonSource<'a,S:DagOutputSource+?Sized>{pub source:&'a S,pub kind:DagOutputKind}
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"DAG publication source changed")}
fn channel<'a>(value:DagChannelView<'a>,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{match path{[]=>Ok(JsonWriteNode::Object(3)),[0]=>Ok(JsonWriteNode::String(value.widget_id)),[1]=>Ok(JsonWriteNode::String(value.port)),[2]=>Ok(JsonWriteNode::String(match value.direction{DagChannelDirection::In=>"in",DagChannelDirection::Out=>"out"})),_=>Err(absent())}}
fn refusal<'a>(value:DagRefusalView<'a>,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{match path{[]=>Ok(JsonWriteNode::Object(4)),[0]=>Ok(JsonWriteNode::String(value.source)),[1]=>Ok(JsonWriteNode::Array(value.source_types.len())),[2]=>Ok(JsonWriteNode::String(value.target)),[3]=>Ok(JsonWriteNode::Array(value.target_types.len())),[1,index]=>value.source_types.get(*index).map(|text|JsonWriteNode::String(text)).ok_or_else(absent),[3,index]=>value.target_types.get(*index).map(|text|JsonWriteNode::String(text)).ok_or_else(absent),_=>Err(absent())}}
impl<S:DagOutputSource+?Sized> JsonWriteSource for DagJsonSource<'_,S>{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{match self.kind{
        DagOutputKind::Nodes=>match path{[]=>Ok(JsonWriteNode::Array(self.source.selected_node_count())),[index]=>self.source.selected_node_id(*index).map(JsonWriteNode::String).ok_or_else(absent),_=>Err(absent())},
        DagOutputKind::Edges=>match path{[]=>Ok(JsonWriteNode::Array(self.source.selected_edge_count())),[index]=>self.source.selected_edge_id(*index).map(JsonWriteNode::String).ok_or_else(absent),_=>Err(absent())},
        DagOutputKind::Selection=>match path{[]=>Ok(JsonWriteNode::Object(3)),[0]=>Ok(JsonWriteNode::Array(self.source.selected_node_count())),[1]=>Ok(JsonWriteNode::Array(self.source.selected_edge_count())),[2]=>Ok(JsonWriteNode::Array(self.source.selected_channel_count())),[0,index]=>self.source.selected_node_id(*index).map(JsonWriteNode::String).ok_or_else(absent),[1,index]=>self.source.selected_edge_id(*index).map(JsonWriteNode::String).ok_or_else(absent),[2,index]=>self.source.selected_handle_identity(*index).map(JsonWriteNode::String).ok_or_else(absent),_=>Err(absent())},
        DagOutputKind::Channels=>match path{[]=>Ok(JsonWriteNode::Array(self.source.selected_channel_count())),[index,rest @ ..]=>channel(self.source.selected_channel(*index).ok_or_else(absent)?,rest)},
        DagOutputKind::Hover=>match path{[]=>Ok(JsonWriteNode::Object(2)),[0,rest @ ..]=>match self.source.hovered_channel_view(){Some(value)=>channel(value,rest),None if rest.is_empty()=>Ok(JsonWriteNode::Null),_=>Err(absent())},[1,rest @ ..]=>match self.source.wire_refusal_view(){Some(value)=>refusal(value,rest),None if rest.is_empty()=>Ok(JsonWriteNode::Null),_=>Err(absent())},_=>Err(absent())},
    }}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
        let keys:&[&str]=match(self.kind,path){(DagOutputKind::Selection,[])=>&["nodes","edges","handles"],(DagOutputKind::Channels,[_])|(DagOutputKind::Hover,[0])=>&["widgetId","port","direction"],(DagOutputKind::Hover,[])=>&["channel","refusal"],(DagOutputKind::Hover,[1])=>&["source","sourceTypes","target","targetTypes"],_=>return Err(absent())};keys.get(index).copied().ok_or_else(absent)
    }
}
