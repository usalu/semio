//! 🧺️ Funds the original retained modeling source projection before any original backing or scalar output changes.

use super::*;
use protocol::value::{RetirementDemand,ValueError,ValueRefusalKind};

fn refusal(kind:ValueRefusalKind,reason:&'static str)->ValueError {ValueError::literal(kind,reason)}
fn extent<T>(count:usize)->Result<usize,ValueError> {count.checked_mul(std::mem::size_of::<T>()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"original snapshot buffer extent overflow"))}
fn backing<T>(buffer:&Vec<T>,count:usize)->Result<usize,ValueError> {
    if !buffer.is_empty()||buffer.capacity()!=0 {return Err(refusal(ValueRefusalKind::InvariantViolated,"original snapshot reservation requires its exact empty buffer"));}
    extent::<T>(count)
}
fn reserve<T>(buffer:&mut Vec<T>,count:usize,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<(),ValueError> {
    buffer.try_reserve_exact(count).map_err(|_|refusal(ValueRefusalKind::AllocationFailed,"original snapshot buffer reservation failed"))?;
    receipt.retained_capacity_bytes=extent::<T>(buffer.capacity())?;receipt.copied_items=1;
    if receipt.retained_capacity_bytes>grant.maximum_capacity_bytes {return Err(refusal(ValueRefusalKind::OwnershipLimit,"original snapshot allocator exceeded admitted backing").with_retained_progress(*receipt));}Ok(())
}

impl Snapshot {
    pub(super) fn complete(&self)->bool {self.reservation>=3&&self.vertex==self.source.vertices.len()&&self.face==self.source.faces.len()}
    fn next_reservation(&self)->Option<u8> {
        if self.reservation<3 {Some(self.reservation)}else if self.vertex==self.source.vertices.len()&&self.face<self.source.faces.len()&&self.corners.capacity()==0 {Some(3)}else {None}
    }
    /// 🪆️ Quotes the same original source, output and retained scalar frontier.
    pub(super) fn preparation_demands(&self)->Result<RetirementDemand,ValueError> {
        if let Some(buffer)=self.next_reservation() {
            let capacity=match buffer {0=>backing(&self.soup.positions,self.source.vertices.len())?,1=>backing(&self.soup.faces,self.source.faces.len())?,2=>backing(&self.soup.normals,self.source.faces.len())?,3=>backing(&self.corners,self.source.halfedges.len())?,_=>unreachable!()};
            return Ok(RetirementDemand {copy_bytes:0,capacity_bytes:capacity,release_bytes:0,depth:3});
        }
        let copy=if self.vertex<self.source.vertices.len() {12}else if self.face==self.source.faces.len() {0}else if let Some(index)=self.reversing {if index<self.corners.len()/2 {8}else {0}}else if self.normalizing {
            if self.corners.len()<3||self.normal_corner>=self.corners.len() {return Err(refusal(ValueRefusalKind::InvalidValue,"original snapshot normal boundary is invalid"));}
            24+usize::from(self.normal_corner==0)*12+usize::from(self.normal_corner+1==self.corners.len())*48
        }else {4};
        Ok(RetirementDemand {copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:3})
    }
    /// 🎟️ Advances one same original producer event with independently admitted physical custody.
    pub(super) fn advance(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError> {
        *receipt=Default::default();let demand=self.preparation_demands()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(false);}
        if let Some(buffer)=self.next_reservation() {
            match buffer {0=>reserve(&mut self.soup.positions,self.source.vertices.len(),grant,receipt)?,1=>reserve(&mut self.soup.faces,self.source.faces.len(),grant,receipt)?,2=>reserve(&mut self.soup.normals,self.source.faces.len(),grant,receipt)?,3=>reserve(&mut self.corners,self.source.halfedges.len(),grant,receipt)?,_=>unreachable!()};
            if self.reservation<3 {self.reservation+=1;}return Ok(false);
        }
        if self.vertex<self.source.vertices.len() {
            if self.soup.positions.len()==self.soup.positions.capacity() {return Err(refusal(ValueRefusalKind::InvariantViolated,"original snapshot position exceeds its admitted buffer"));}
        }else if self.face<self.source.faces.len() {
            if self.normalizing {
                if [self.corners[self.normal_corner],self.corners[(self.normal_corner+1)%self.corners.len()]].into_iter().any(|id|id as usize>=self.soup.positions.len()) {return Err(refusal(ValueRefusalKind::InvalidValue,"original snapshot normal source is invalid"));}
                if self.normal_corner+1==self.corners.len()&&(self.soup.normals.len()==self.soup.normals.capacity()||self.soup.faces.len()==self.soup.faces.capacity()) {return Err(refusal(ValueRefusalKind::InvariantViolated,"original snapshot face exceeds its admitted buffers"));}
            }else if self.reversing.is_none() {
                let id=self.halfedge.unwrap_or(self.source.faces[self.face].halfedge);
                if self.source.halfedges.get(id as usize).is_none()||self.corners.len()>=self.source.halfedges.len() {return Err(refusal(ValueRefusalKind::InvalidValue,"original snapshot boundary does not close on its source"));}
                if self.corners.len()==self.corners.capacity() {return Err(refusal(ValueRefusalKind::InvariantViolated,"original snapshot corner exceeds its admitted buffer"));}
            }
        }
        *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};Ok(self.advance_original())
    }
}
