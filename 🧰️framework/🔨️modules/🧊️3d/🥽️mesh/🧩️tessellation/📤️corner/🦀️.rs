use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[derive(Clone,Copy,Default)]
pub(super) struct MeshCornerReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(MeshCornerReceipt);

pub(super) struct CornerData{corner:u32,vertex:u32,position:[f32;3],normal:[f32;3],uv:[f32;2],color:Option<[f32;4]>}
fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
pub(super) fn sample<'a>(attribute:Option<&'a MeshAttribute>,edge_id:u32,edge:&HalfEdge,face:usize)->Option<&'a [protocol::value::DslValue]>{
    let attribute=attribute?;attribute.value_at(match attribute.domain{MeshAttributeDomain::Corner|MeshAttributeDomain::Edge=>edge_id as usize,MeshAttributeDomain::Face=>face,MeshAttributeDomain::Vertex=>edge.vertex as usize}).and_then(protocol::value::DslValue::as_array)
}
fn room<T>(buffer:&Vec<T>,count:usize)->bool{buffer.capacity().saturating_sub(buffer.len())>=count}
fn valid_sample(value:Option<&[protocol::value::DslValue]>,count:usize)->bool{value.is_none_or(|value|value.len()==count&&value.iter().all(|number|number.as_f64().is_some_and(|number|number.is_finite()&&number.abs()<=f32::MAX as f64)))}

impl MeshTessellationJob{
    fn prepared_corner(&self,local:usize)->Result<(u32,&HalfEdge,&MeshVertex),ValueError>{
        if self.cancelled||!self.buffer_reservation_complete()||!self.metadata_capture_complete(){return Err(refusal("original mesh corner requires completed original preparation"));}
        let face=self.mesh.faces.get(self.face).ok_or_else(||refusal("original mesh corner face is absent"))?;
        let slot=if face.flipped{self.hes.len().checked_sub(local.checked_add(1).ok_or_else(||refusal("original mesh corner index overflow"))?).ok_or_else(||refusal("original mesh corner index is absent"))?}else{local};
        let edge_id=*self.hes.get(slot).ok_or_else(||refusal("original mesh boundary corner is absent"))?;
        let edge=self.mesh.halfedges.get(edge_id as usize).ok_or_else(||refusal("original mesh halfedge is absent"))?;
        let vertex=self.mesh.vertices.get(edge.vertex as usize).ok_or_else(||refusal("original mesh vertex is absent"))?;
        if vertex.position.iter().any(|value|!value.is_finite()){return Err(refusal("original mesh position is not finite"));}
        for(semantic,count)in[(MeshAttributeSemantic::Normal,3),(MeshAttributeSemantic::Uv,2),(MeshAttributeSemantic::Color,4)]{
            let attribute=self.semantic_attribute(semantic);let value=sample(attribute,edge_id,edge,self.face);
            if(attribute.is_some()&&value.is_none())||!valid_sample(value,count){return Err(refusal("original authored mesh corner sample is invalid"));}
        }
        Ok((edge_id,edge,vertex))
    }
    /// 🧮️ Computes one corner from borrowed source channels; both callers append through the same producer below.
    pub(super) fn corner_data(&self,local:usize,attributes:[Option<&MeshAttribute>;3])->CornerData{
        let edge_id=self.halfedge(local);let edge=&self.mesh.halfedges[edge_id as usize];let vertex=&self.mesh.vertices[edge.vertex as usize];
        let normal=sample(attributes[0],edge_id,edge,self.face).map(|value|[0,1,2].map(|axis|value[axis].as_f64().unwrap()as f32)).unwrap_or_else(||if self.mesh.faces[self.face].smooth{vertex.normal.unwrap_or(self.normal.0)}else{self.normal.0});
        let uv=sample(attributes[1],edge_id,edge,self.face).map(|value|[0,1].map(|axis|value[axis].as_f64().unwrap()as f32)).unwrap_or(edge.uv);
        let color=sample(attributes[2],edge_id,edge,self.face).map(|value|[0,1,2,3].map(|axis|value[axis].as_f64().unwrap()as f32));
        CornerData{corner:edge_id,vertex:edge.vertex,position:vertex.position,normal,uv,color}
    }
    pub(super) fn append_corner_data(&mut self,data:CornerData){
        self.corner_ids.push(data.corner);let out=self.output.as_mut().unwrap();out.positions.extend_from_slice(&data.position);out.normals.extend_from_slice(&data.normal);out.vertex_ids.push(data.vertex);out.uvs.extend_from_slice(&data.uv);if let Some(color)=data.color{out.colors.extend_from_slice(&color);}
    }
    /// 📏️ Prices every output scalar plus original vertex/corner identities in this atomic corner.
    pub(super) fn next_corner_copy_byte_demand(&self,local:usize)->Result<usize,ValueError>{let(edge_id,edge,_)=self.prepared_corner(local)?;Ok((8+4*usize::from(sample(self.semantic_attribute(MeshAttributeSemantic::Color),edge_id,edge,self.face).is_some()))*std::mem::size_of::<f32>()+2*std::mem::size_of::<u32>())}
    /// 🧺️ Requires the already admitted original backing; emission cannot trigger another allocation.
    pub(super) fn next_corner_capacity_byte_demand(&self,local:usize,_maximum_copy_bytes:usize)->Result<usize,ValueError>{
        let color=self.next_corner_copy_byte_demand(local)?>40;let out=self.output.as_ref().ok_or_else(||refusal("original mesh corner output is absent"))?;
        if !room(&out.positions,3)||!room(&out.normals,3)||!room(&out.vertex_ids,1)||!room(&out.uvs,2)||!room(&self.corner_ids,1)||(color&&!room(&out.colors,4)){return Err(refusal("original mesh corner exceeds its admitted output extent"));}Ok(0)
    }
    pub(super) fn next_corner_release_byte_demand(&self,local:usize)->Result<usize,ValueError>{self.next_corner_copy_byte_demand(local).map(|_|0)}
    /// 🧗️ Covers the actual borrowed original slot and the fixed authored sample/array/number path.
    pub(super) fn next_corner_depth_demand(&self,local:usize)->Result<usize,ValueError>{
        self.prepared_corner(local)?;let mut depth=1;
        for slot in self.semantic_slots.into_iter().flatten(){depth=depth.max(self.mesh.attributes.next_slot_depth_demand(slot)?.checked_add(4).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original authored mesh corner depth overflow"))?);}
        Ok(depth)
    }
    /// 📤️ Writes one quoted atomic corner, leaving source and output unchanged under an independently deficient grant.
    pub(super) fn emit_corner_step(&mut self,local:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.corner_receipt.0=Default::default();let copy=self.next_corner_copy_byte_demand(local)?;self.next_corner_capacity_byte_demand(local,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<self.next_corner_depth_demand(local)?{return Ok(RetainedCloneStep::Progress(self.corner_receipt.0));}
        let data=self.corner_data(local,[self.semantic_attribute(MeshAttributeSemantic::Normal),self.semantic_attribute(MeshAttributeSemantic::Uv),self.semantic_attribute(MeshAttributeSemantic::Color)]);
        if data.normal.iter().chain(data.uv.iter()).any(|value|!value.is_finite()){return Err(refusal("original computed mesh corner sample is not finite"));}
        self.append_corner_data(data);self.corner_receipt.0=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};Ok(RetainedCloneStep::Complete(self.corner_receipt.0))
    }
}
