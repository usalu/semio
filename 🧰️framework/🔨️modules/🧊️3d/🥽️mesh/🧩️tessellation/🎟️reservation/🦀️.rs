use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[derive(Clone,Copy,Default)]
pub(super) struct MeshBufferReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(MeshBufferReceipt);

fn refusal(kind:ValueRefusalKind,reason:&'static str)->ValueError{ValueError::literal(kind,reason)}
fn extent(count:usize,factor:usize)->Result<usize,ValueError>{count.checked_mul(factor).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"original mesh buffer extent overflow"))}
fn demand<T>(buffer:&Vec<T>,count:usize)->Result<usize,ValueError>{
    if !buffer.is_empty(){return Err(refusal(ValueRefusalKind::InvariantViolated,"original mesh buffer reservation must precede payload emission"));}
    if buffer.capacity()>=count{return Ok(0);}
    if buffer.capacity()!=0{return Err(refusal(ValueRefusalKind::InvariantViolated,"original empty mesh buffer has partial unadmitted backing"));}
    extent(count,std::mem::size_of::<T>())
}
fn reserve<T>(buffer:&mut Vec<T>,count:usize,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<(),ValueError>{
    if demand(buffer,count)?!=0{
        buffer.try_reserve_exact(count).map_err(|_|refusal(ValueRefusalKind::AllocationFailed,"original mesh buffer reservation failed"))?;
        receipt.retained_capacity_bytes=extent(buffer.capacity(),std::mem::size_of::<T>())?;
        if receipt.retained_capacity_bytes>grant.maximum_capacity_bytes{return Err(refusal(ValueRefusalKind::OwnershipLimit,"original mesh allocator exceeded admitted backing").with_retained_progress(*receipt));}
    }
    receipt.copied_items=1;Ok(())
}

impl MeshTessellationJob{
    /// 🧺️ Reports completion of this original buffer preparation frontier, including cancellation.
    pub fn buffer_reservation_complete(&self)->bool{self.cancelled||self.output.is_none()||self.buffer_reservation==16}
    /// 🧾️ Preserves actual accepted backing on the successful or failed original reservation turn.
    pub fn buffer_reservation_progress(&self)->RetainedCloneProgress{self.buffer_receipt.0}
    /// 🪶️ Empty-buffer reservation moves no original payload bytes.
    pub fn next_buffer_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_buffer_capacity_byte_demand(0).map(|_|0)}
    /// 📐️ Quotes one actual empty buffer using the original source halfedge extent without traversing a directory.
    pub fn next_buffer_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{
        if self.buffer_reservation_complete(){return Ok(0);}
        let count=self.mesh.halfedges.len();let out=self.output.as_ref().unwrap();
        macro_rules! quote{($buffer:expr,$factor:expr)=>{demand($buffer,extent(count,$factor)?)};}
        match self.buffer_reservation{
            0=>quote!(&self.hes,1),1=>quote!(&self.points,1),2=>quote!(&self.projected,1),3=>quote!(&self.links,1),
            4=>quote!(&out.positions,9),5=>quote!(&out.normals,9),6=>quote!(&out.colors,12),7=>quote!(&out.indices,3),
            8=>quote!(&out.uvs,6),9=>quote!(&out.face_ids,1),10=>quote!(&out.vertex_ids,3),11=>quote!(&out.edge_positions,6),
            12=>quote!(&out.edge_ids,1),13=>quote!(&out.edge_uvs,4),14=>quote!(&out.edge_is_seam,1),15=>quote!(&self.corner_ids,3),
            _=>Err(refusal(ValueRefusalKind::InvariantViolated,"original mesh buffer reservation cursor is invalid")),
        }
    }
    /// 🍂️ Every accepted original backing remains retained until typed closure.
    pub fn next_buffer_release_byte_demand(&self)->Result<usize,ValueError>{self.next_buffer_capacity_byte_demand(0).map(|_|0)}
    /// 📏️ Reservation accesses one original Vec field and its allocator backing.
    pub fn next_buffer_depth_demand(&self)->Result<usize,ValueError>{self.next_buffer_capacity_byte_demand(0).map(|_|usize::from(!self.buffer_reservation_complete()))}
    /// 🎟️ Reserves one original empty buffer under independent currencies without changing any source or prior buffer address.
    pub fn reserve_buffer_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.buffer_receipt.0=Default::default();
        if self.buffer_reservation_complete(){return Ok(RetainedCloneStep::Complete(self.buffer_receipt.0));}
        let capacity=self.next_buffer_capacity_byte_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(self.buffer_receipt.0));}
        let count=self.mesh.halfedges.len();let out=self.output.as_mut().unwrap();
        macro_rules! admit{($buffer:expr,$factor:expr)=>{reserve($buffer,extent(count,$factor)?,grant,&mut self.buffer_receipt.0)};}
        match self.buffer_reservation{
            0=>admit!(&mut self.hes,1),1=>admit!(&mut self.points,1),2=>admit!(&mut self.projected,1),3=>admit!(&mut self.links,1),
            4=>admit!(&mut out.positions,9),5=>admit!(&mut out.normals,9),6=>admit!(&mut out.colors,12),7=>admit!(&mut out.indices,3),
            8=>admit!(&mut out.uvs,6),9=>admit!(&mut out.face_ids,1),10=>admit!(&mut out.vertex_ids,3),11=>admit!(&mut out.edge_positions,6),
            12=>admit!(&mut out.edge_ids,1),13=>admit!(&mut out.edge_uvs,4),14=>admit!(&mut out.edge_is_seam,1),15=>admit!(&mut self.corner_ids,3),
            _=>Err(refusal(ValueRefusalKind::InvariantViolated,"original mesh buffer reservation cursor is invalid")),
        }?;
        self.buffer_reservation+=1;
        Ok(if self.buffer_reservation_complete(){RetainedCloneStep::Complete(self.buffer_receipt.0)}else{RetainedCloneStep::Progress(self.buffer_receipt.0)})
    }
}
