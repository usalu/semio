use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[derive(Clone,Copy,Default)]
pub(super) struct MeshFaceReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(MeshFaceReceipt);

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
fn room<T>(buffer:&Vec<T>)->bool{buffer.len()<buffer.capacity()}

impl MeshTessellationJob{
    /// 🧭️ Face preparation ends before any original corner, triangle, or edge output is emitted.
    pub(super) fn face_preparation_complete(&self)->bool{self.cancelled||self.face>=self.mesh.face_count()||self.phase>=7}
    pub(super) fn next_face_copy_byte_demand(&self)->Result<usize,ValueError>{
        if self.face_preparation_complete(){return Ok(0);}
        if !self.buffer_reservation_complete(){return Err(refusal("original face preparation requires admitted buffers"));}
        Ok(match self.phase{0=>0,1=>std::mem::size_of::<u32>(),2=>std::mem::size_of::<Vec3>(),3=>std::mem::size_of::<[f64;3]>()+if self.cursor+1==self.hes.len(){std::mem::size_of::<Vec3>()+std::mem::size_of::<(Vec3f64,Vec3f64)>()}else{0},4=>3*std::mem::size_of::<f64>()+2*std::mem::size_of::<usize>(),5=>if self.scale==0.0{0}else{2*std::mem::size_of::<f64>()},6=>std::mem::size_of::<f64>()+if self.cursor+1==self.hes.len(){std::mem::size_of::<u32>()}else{0},_=>return Err(refusal("original face preparation phase is invalid"))})
    }
    pub(super) fn next_face_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError>{
        self.next_face_copy_byte_demand()?;if self.face_preparation_complete(){return Ok(0);}
        let admitted=match self.phase{1=>room(&self.hes),2=>room(&self.points),4=>room(&self.projected)&&room(&self.links),_=>true};if !admitted{return Err(refusal("original face preparation exceeds its admitted backing"));}Ok(0)
    }
    pub(super) fn next_face_release_byte_demand(&self)->Result<usize,ValueError>{self.next_face_copy_byte_demand().map(|_|0)}
    /// 🧗️ Includes the actual optional original selected-face membership path and the direct original source row.
    pub(super) fn next_face_depth_demand(&self)->Result<usize,ValueError>{
        if self.face_preparation_complete(){return Ok(0);}
        let depth=if self.phase==0{self.selected_faces.as_ref().map_or(Ok(1),|faces|faces.next_contains_depth_demand(&(self.face as u32)))}else{Ok(2)}?;
        depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original face preparation depth overflow"))
    }
    /// 🎟️ Drives the same original face producer only after admitting its exact one-event scalar write extent.
    pub(super) fn prepare_face_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.face_receipt.0=Default::default();if self.face_preparation_complete(){return Ok(RetainedCloneStep::Complete(self.face_receipt.0));}
        let copy=self.next_face_copy_byte_demand()?;self.next_face_capacity_byte_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<self.next_face_depth_demand()?{return Ok(RetainedCloneStep::Progress(self.face_receipt.0));}
        if self.phase==1&&(self.hes.len()>=self.mesh.halfedges.len()||self.mesh.halfedges.get(self.next_he as usize).is_none()){return Err(refusal("original face boundary does not close on its actual source"));}
        let next=self.done.checked_add(1).ok_or_else(||refusal("original mesh preparation progress overflow"))?;
        self.face_receipt.0=RetainedCloneProgress {copied_items:1,copied_bytes:copy,..Default::default()};self.advance().map_err(|_|refusal("original mesh face preparation source is invalid").with_retained_progress(self.face_receipt.0))?;self.done=next;
        Ok(if self.face_preparation_complete(){RetainedCloneStep::Complete(self.face_receipt.0)}else{RetainedCloneStep::Progress(self.face_receipt.0)})
    }
}
