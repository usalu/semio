//! 🌊️ Authored Flow operation fields emit their original text grammar directly into paid backing.
use crate::standards::v1::subsets::flow::schema::mutations::*;
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowNode, FlowEdge, PortRef, SemioFlowSnapshot};
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::flow::io::binary::mutations::wire_tag;
use semio_framework_os_kernel::os_spr::operation_bytes::{OperationByteOutput,OperationByteLimitedOutput};
use protocol::{PackRefusal,codec::PackEncodeOptions};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
use std::fmt::Write;

struct Scalar{bytes:[u8;384],length:usize}
impl std::fmt::Write for Scalar{
    fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}
}
fn raw(output:&mut dyn OperationByteOutput,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{output.write_bytes(text.as_bytes(),control)}
fn number(output:&mut dyn OperationByteOutput,value:f64,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{control.checkpoint()?;let mut text=Scalar{bytes:[0;384],length:0};write!(text,"{value}").map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"Flow float exceeds fixed native Display cell"))?;output.write_bytes(&text.bytes[..text.length],control)}
fn hex(output:&mut dyn OperationByteOutput,value:&str,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
    control.scoped_stage(|control|{control.begin_stage(value.len())?;for byte in value.bytes(){let digits=b"0123456789abcdef";output.write_bytes(&[digits[usize::from(byte>>4)],digits[usize::from(byte&15)]],control)?;control.advance(1)?;}Ok(())})
}
fn tuple(options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>,length:usize,mut item:impl FnMut(usize,&mut dyn OperationByteOutput,&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>)->Result<(),PackRefusal>{
    control.scoped_depth(usize::from(options.limits.max_depth),|control|{
        if length as u64>options.limits.max_items{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"Flow source collection extent"});}
        raw(output,"[",control)?;
        control.scoped_stage(|control|{control.begin_stage(length)?;for index in 0..length{control.checkpoint()?;if index!=0{raw(output,",",control)?;}item(index,output,control)?;control.step()?;}Ok::<_,PackRefusal>(())})?;
        raw(output,"]",control)
    })
}
fn point(options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>,value:&SemioPoint2)->Result<(),PackRefusal>{tuple(options,output,control,2,|index,output,control|number(output,if index==0{value.x}else{value.y},control))}
fn port(options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>,value:&PortRef)->Result<(),PackRefusal>{tuple(options,output,control,2,|index,output,control|hex(output,if index==0{&value.node}else{&value.port},control))}
fn node(options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>,value:&FlowNode)->Result<(),PackRefusal>{
    tuple(options,output,control,5,|index,output,control|match index{
        0=>hex(output,&value.id,control),1=>hex(output,&value.kind,control),2=>hex(output,&value.label,control),
        3=>tuple(options,output,control,value.params.len(),|index,output,control|{let param=&value.params[index];tuple(options,output,control,2,|index,output,control|hex(output,if index==0{&param.key}else{&param.value},control))}),
        _=>point(options,output,control,&value.position),
    })
}
fn edge(options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>,value:&FlowEdge)->Result<(),PackRefusal>{tuple(options,output,control,4,|index,output,control|match index{0=>hex(output,&value.id,control),1=>port(options,output,control,&value.from),2=>port(options,output,control,&value.to),_=>hex(output,&value.kind,control)})}
fn snapshot(options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>,value:&SemioFlowSnapshot)->Result<(),PackRefusal>{tuple(options,output,control,3,|index,output,control|match index{0=>hex(output,&value.schema,control),1=>tuple(options,output,control,value.nodes.len(),|index,output,control|node(options,output,control,&value.nodes[index])),_=>tuple(options,output,control,value.edges.len(),|index,output,control|edge(options,output,control,&value.edges[index]))})}

impl SemioFlowMutation{
    /// ✍️ Streams every original Flow frame under the caller's exact source and full-frame policy.
    pub fn encode_op_into(&self,options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),protocol::ProtocolError>{
        semio_framework_os_kernel::os_spr::operation_bytes::with_operation_encode_policy(options,control,|control|{
        let mut limited=OperationByteLimitedOutput::new(output,options.limits.max_file_len);
        let output:&mut dyn OperationByteOutput=&mut limited;
        output.write_bytes(&[1,wire_tag(self)],control)?;
        match self{
            Self::InsertNode(insert_node::InsertNode{node:value})=>{raw(output,"node=",control)?;node(options,output,control,value)?;},
            Self::RemoveNode(remove_node::RemoveNode{id})|Self::RemoveEdge(remove_edge::RemoveEdge{id})=>{raw(output,"id=",control)?;hex(output,id,control)?;},
            Self::SetNodeKind(set_node_kind::SetNodeKind{id,kind})|Self::SetEdgeKind(set_edge_kind::SetEdgeKind{id,kind})=>{raw(output,"id=",control)?;hex(output,id,control)?;raw(output," kind=",control)?;hex(output,kind,control)?;},
            Self::SetNodeLabel(set_node_label::SetNodeLabel{id,label})=>{raw(output,"id=",control)?;hex(output,id,control)?;raw(output," label=",control)?;hex(output,label,control)?;},
            Self::SetNodePosition(set_node_position::SetNodePosition{id,position})=>{raw(output,"id=",control)?;hex(output,id,control)?;raw(output," position=",control)?;point(options,output,control,position)?;},
            Self::SetNodeParam(set_node_param::SetNodeParam{id,key,value})=>{raw(output,"id=",control)?;hex(output,id,control)?;raw(output," key=",control)?;hex(output,key,control)?;raw(output," value=",control)?;hex(output,value,control)?;},
            Self::RemoveNodeParam(remove_node_param::RemoveNodeParam{id,key})=>{raw(output,"id=",control)?;hex(output,id,control)?;raw(output," key=",control)?;hex(output,key,control)?;},
            Self::InsertEdge(insert_edge::InsertEdge{edge:value})=>{raw(output,"edge=",control)?;edge(options,output,control,value)?;},
            Self::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints{id,from,to})=>{raw(output,"id=",control)?;hex(output,id,control)?;raw(output," from=",control)?;port(options,output,control,from)?;raw(output," to=",control)?;port(options,output,control,to)?;},
            Self::DragNodes(drag_nodes::DragNodes{targets,dx,dy})=>{raw(output,"targets=",control)?;tuple(options,output,control,targets.len(),|index,output,control|hex(output,&targets[index],control))?;raw(output," dx=",control)?;number(output,*dx,control)?;raw(output," dy=",control)?;number(output,*dy,control)?;},
        }
        Ok(())
        })
    }
}
