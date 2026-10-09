use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[derive(Clone,Copy,Default)]
pub(super) struct MeshTriangleReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(MeshTriangleReceipt);

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}

impl MeshTessellationJob{
    fn prepared_triangle(&self,indices:[u32;3])->Result<(),ValueError>{
        if self.cancelled||!self.buffer_reservation_complete()||self.mesh.faces.get(self.face).is_none(){return Err(refusal("original mesh triangle source is not prepared"));}
        let out=self.output.as_ref().ok_or_else(||refusal("original mesh triangle output is absent"))?;
        if indices.iter().any(|index|*index as usize>=out.positions.len()/3){return Err(refusal("original mesh triangle references an absent emitted vertex"));}Ok(())
    }
    /// 🔺️ Writes one actual original triangle row through the shared output producer.
    pub(super) fn append_triangle_indices(&mut self,indices:[u32;3]){let out=self.output.as_mut().unwrap();out.indices.extend_from_slice(&indices);out.face_ids.push(self.face as u32);}
    pub(super) fn next_triangle_copy_byte_demand(&self,indices:[u32;3])->Result<usize,ValueError>{self.prepared_triangle(indices).map(|_|4*std::mem::size_of::<u32>())}
    pub(super) fn next_triangle_capacity_byte_demand(&self,indices:[u32;3],_copy:usize)->Result<usize,ValueError>{self.prepared_triangle(indices)?;let out=self.output.as_ref().unwrap();if out.indices.capacity().saturating_sub(out.indices.len())<3||out.face_ids.capacity()==out.face_ids.len(){return Err(refusal("original mesh triangle exceeds its admitted output extent"));}Ok(0)}
    pub(super) fn next_triangle_release_byte_demand(&self,indices:[u32;3])->Result<usize,ValueError>{self.prepared_triangle(indices).map(|_|0)}
    pub(super) fn next_triangle_depth_demand(&self,indices:[u32;3])->Result<usize,ValueError>{self.prepared_triangle(indices).map(|_|1)}
    /// 🎟️ Purely refuses any under-atomic policy before modifying the original index or face buffer.
    pub(super) fn emit_triangle_step(&mut self,indices:[u32;3],grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.triangle_receipt.0=Default::default();let copy=self.next_triangle_copy_byte_demand(indices)?;self.next_triangle_capacity_byte_demand(indices,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<self.next_triangle_depth_demand(indices)?{return Ok(RetainedCloneStep::Progress(self.triangle_receipt.0));}
        self.append_triangle_indices(indices);self.triangle_receipt.0=RetainedCloneProgress {copied_items:1,copied_bytes:copy,..Default::default()};Ok(RetainedCloneStep::Complete(self.triangle_receipt.0))
    }
}
