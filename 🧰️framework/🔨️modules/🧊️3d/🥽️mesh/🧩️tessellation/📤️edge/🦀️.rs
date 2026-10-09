use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[derive(Clone,Copy,Default)]
pub(super) struct MeshEdgeReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(MeshEdgeReceipt);

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
fn room<T>(buffer:&Vec<T>,count:usize)->bool{buffer.capacity().saturating_sub(buffer.len())>=count}

impl MeshTessellationJob{
    fn prepared_edge(&self,local:usize)->Result<(u32,&HalfEdge,&HalfEdge,(u32,u32)),ValueError>{
        if self.cancelled||!self.buffer_reservation_complete()||!self.metadata_capture_complete(){return Err(refusal("original mesh edge requires completed original preparation"));}
        if self.edge_pending.is_some_and(|pending|pending!=local){return Err(refusal("original admitted mesh edge must finish before another edge"));}
        let id=*self.hes.get(local).ok_or_else(||refusal("original mesh edge boundary slot is absent"))?;
        let edge=self.mesh.halfedges.get(id as usize).ok_or_else(||refusal("original mesh edge is absent"))?;
        let next=self.mesh.halfedges.get(edge.next as usize).ok_or_else(||refusal("original mesh edge successor is absent"))?;
        for vertex in[edge.vertex,next.vertex]{if self.mesh.vertices.get(vertex as usize).is_none_or(|vertex|vertex.position.iter().any(|value|!value.is_finite())){return Err(refusal("original mesh edge vertex is invalid"));}}
        Ok((id,edge,next,(edge.vertex.min(next.vertex),edge.vertex.max(next.vertex))))
    }
    fn prepared_edge_uv(&self,id:u32)->Result<[f64;2],ValueError>{
        let edge=self.mesh.halfedges.get(id as usize).ok_or_else(||refusal("original mesh UV halfedge is absent"))?;
        let attribute=self.semantic_attribute(MeshAttributeSemantic::Uv);
        let value=super::tessellation_corner_emission::sample(attribute,id,edge,edge.face.ok_or_else(||refusal("original mesh UV face is absent"))? as usize);
        if attribute.is_some()&&value.is_none(){return Err(refusal("original mesh edge authored UV sample is absent"));}
        let uv=if let Some(value)=value{if value.len()!=2{return Err(refusal("original mesh edge authored UV extent is invalid"));}[value[0].as_f64().ok_or_else(||refusal("original mesh edge authored UV is not numeric"))?,value[1].as_f64().ok_or_else(||refusal("original mesh edge authored UV is not numeric"))?]}else{edge.uv.map(f64::from)};
        if uv.iter().any(|value|!value.is_finite()||value.abs()>f32::MAX as f64){return Err(refusal("original mesh edge UV is invalid"));}Ok(uv)
    }
    /// 🧮️ Prices one complete edge row only after its original natural membership insertion.
    pub(super) fn next_edge_copy_byte_demand(&self,local:usize)->Result<usize,ValueError>{self.prepared_edge(local)?;Ok(if self.edge_pending.is_some(){10*std::mem::size_of::<f32>()+std::mem::size_of::<u32>()+std::mem::size_of::<u8>()}else{0})}
    /// 🎟️ Quotes the original natural edge Set before any new row is transferred to it.
    pub(super) fn next_edge_capacity_byte_demand(&self,local:usize,copy:usize)->Result<usize,ValueError>{
        let(_,_,_,key)=self.prepared_edge(local)?;
        if self.edge_pending.is_none(){return if self.edge_seen.contains(&key){Ok(0)}else{self.edge_seen.next_insert_capacity_byte_demand(&key,copy)};}
        let out=self.output.as_ref().ok_or_else(||refusal("original mesh edge output is absent"))?;
        if !room(&out.edge_positions,6)||!room(&out.edge_uvs,4)||!room(&out.edge_ids,1)||!room(&out.edge_is_seam,1){return Err(refusal("original mesh edge exceeds its admitted output extent"));}Ok(0)
    }
    pub(super) fn next_edge_release_byte_demand(&self,local:usize)->Result<usize,ValueError>{self.prepared_edge(local).map(|_|0)}
    /// 🧗️ Quotes actual original natural membership paths and the borrowed authored UV source path.
    pub(super) fn next_edge_depth_demand(&self,local:usize)->Result<usize,ValueError>{
        let(id,edge,_,key)=self.prepared_edge(local)?;let mut depth=1;
        if self.edge_pending.is_none(){depth=depth.max(self.edge_seen.next_contains_depth_demand(&key)?);if !self.edge_seen.contains(&key){depth=depth.max(self.edge_seen.next_insert_depth_demand(&key)?);}}
        else{depth=depth.max(self.mesh.uv_seams.next_contains_depth_demand(&id)?);if let Some(twin)=edge.twin{depth=depth.max(self.mesh.uv_seams.next_contains_depth_demand(&twin)?);}if let Some(slot)=self.semantic_slots[1]{depth=depth.max(self.mesh.attributes.next_slot_depth_demand(slot)?.checked_add(4).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original mesh edge sample depth overflow"))?);}}
        depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original mesh edge owner depth overflow"))
    }
    /// 🪢️ Admits one original Set page or row, then emits one separately funded atomic edge without reallocating output.
    pub(super) fn emit_edge_step(&mut self,local:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.edge_receipt.0=Default::default();let capacity=self.next_edge_capacity_byte_demand(local,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<self.next_edge_copy_byte_demand(local)?||grant.maximum_capacity_bytes<capacity||grant.maximum_depth<self.next_edge_depth_demand(local)?{return Ok(RetainedCloneStep::Progress(self.edge_receipt.0));}
        let(id,_,_,key)=self.prepared_edge(local)?;
        if self.edge_pending.is_none(){
            if self.edge_seen.contains(&key){self.edge_receipt.0.copied_items=1;return Ok(RetainedCloneStep::Complete(self.edge_receipt.0));}
            if capacity!=0{match self.edge_seen.reserve_insert_step(&key,grant){Ok(receipt)=>self.edge_receipt.0=receipt,Err((error,receipt))=>{self.edge_receipt.0=receipt;return Err(error.with_retained_progress(receipt));}}return Ok(RetainedCloneStep::Progress(self.edge_receipt.0));}
            match self.edge_seen.insert_reserved(key,grant){Ok((true,receipt))=>self.edge_receipt.0=receipt,Ok((false,receipt))=>{self.edge_receipt.0=receipt;return Err(refusal("original mesh edge membership changed during admission").with_retained_progress(receipt));},Err((error,_))=>return Err(error)}
            self.edge_pending=Some(local);return Ok(RetainedCloneStep::Progress(self.edge_receipt.0));
        }
        let(_,edge,next,_)=self.prepared_edge(local)?;let(a,b)=(self.prepared_edge_uv(id)?,self.prepared_edge_uv(edge.next)?);
        let seam=self.mesh.uv_seams.contains(&id)||if let Some(twin)=edge.twin{let twin_edge=self.mesh.halfedges.get(twin as usize).ok_or_else(||refusal("original mesh edge twin is absent"))?;self.mesh.uv_seams.contains(&twin)||a!=self.prepared_edge_uv(twin_edge.next)?||b!=self.prepared_edge_uv(twin)?}else{false};
        let positions=[self.mesh.vertices[edge.vertex as usize].position,self.mesh.vertices[next.vertex as usize].position];let out=self.output.as_mut().unwrap();out.edge_positions.extend_from_slice(&positions[0]);out.edge_positions.extend_from_slice(&positions[1]);out.edge_ids.push(id);out.edge_uvs.extend_from_slice(&a.map(|value|value as f32));out.edge_uvs.extend_from_slice(&b.map(|value|value as f32));out.edge_is_seam.push(u8::from(seam));self.edge_pending=None;
        self.edge_receipt.0=RetainedCloneProgress{copied_items:1,copied_bytes:10*std::mem::size_of::<f32>()+std::mem::size_of::<u32>()+std::mem::size_of::<u8>(),..Default::default()};Ok(RetainedCloneStep::Complete(self.edge_receipt.0))
    }
}
