//! 🔃️ Updates face winding in the original indexed mesh.

use super::*;

impl Orient {
    pub(crate) fn flip_surface_demand(&self)->Option<semio_framework_value::RetirementDemand> {
        let source=&self.snapshot.source;
        let mut demand=semio_framework_value::RetirementDemand {depth:3,..Default::default()};
        match self.phase {
            8 if self.cursor<self.input.as_ref().unwrap().len()=>demand.copy_bytes=size_of::<bool>(),
            9 if self.flipped.capacity()==0=>demand.capacity_bytes=source.face_count()*size_of::<bool>(),
            9 if self.flipped.len()<source.face_count()=>demand.copy_bytes=size_of::<bool>(),
            10 if self.flip_incidence.capacity()==0=>demand.capacity_bytes=source.vertex_count(),
            10 if self.flip_incidence.len()<source.vertex_count()=>demand.copy_bytes=1,
            11 if self.cursor<source.halfedge_count()=>demand.copy_bytes=1,
            12 if source.vertices.get(self.cursor).is_some_and(|vertex|vertex.normal.is_some())&&self.flip_incidence[self.cursor]==2=>demand.copy_bytes=size_of::<[f32;3]>(),
            13 if self.flipped.get(self.cursor)==Some(&true)=>demand.copy_bytes=size_of::<bool>(),
            8..=14=>{},
            _=>return None,
        }
        Some(demand)
    }

    pub(super) fn advance_flip(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<Option<HalfedgeMesh>> {
        let demand=self.flip_surface_demand().unwrap();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand) {return Ok(None);}
        *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        let source=&mut self.snapshot.source;
        let allocation=|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"face flip backing reservation failed"));
        match self.phase {
            8=>{
                let input=self.input.as_ref().unwrap();
                if let Some(&face)=input.get(self.cursor) {
                    let _=source.face_boundary(face)?;
                    self.flipped[face.0 as usize]=!self.flipped[face.0 as usize];
                    self.cursor+=1;
                }else {self.cursor=0;self.phase=10;}
            },
            9 if demand.capacity_bytes!=0=>{
                self.flipped.try_reserve_exact(source.face_count()).map_err(allocation)?;
                receipt.retained_capacity_bytes=self.flipped.capacity()*size_of::<bool>();
            },
            9 if self.flipped.len()<source.face_count()=>self.flipped.push(false),
            9=>self.phase=8,
            10 if demand.capacity_bytes!=0=>{
                self.flip_incidence.try_reserve_exact(source.vertex_count()).map_err(allocation)?;
                receipt.retained_capacity_bytes=self.flip_incidence.capacity();
            },
            10 if self.flip_incidence.len()<source.vertex_count()=>self.flip_incidence.push(0),
            10=>self.phase=11,
            11=>{
                if let Some(edge)=source.halfedges.get(self.cursor) {
                    let face=edge.face.ok_or(MeshKernelError::InvalidHandle)? as usize;
                    self.flip_incidence[edge.vertex as usize]|=if self.flipped[face]{2}else{1};
                    self.cursor+=1;
                }else {self.cursor=0;self.phase=12;}
            },
            12=>{
                if let Some(vertex)=source.vertices.get_mut(self.cursor) {
                    if self.flip_incidence[self.cursor]==2 {vertex.normal=vertex.normal.map(|normal|normal.map(|axis|-axis));}
                    self.cursor+=1;
                }else {self.cursor=0;self.phase=if self.input.is_some(){13}else{6};}
            },
            13=>{
                if let Some(face)=source.faces.get_mut(self.cursor) {
                    if self.flipped[self.cursor] {face.flipped=!face.flipped;self.flip_count+=1;}
                    self.cursor+=1;
                }else {self.phase=14;}
            },
            14=>return Ok(Some(std::mem::replace(source,HalfedgeMesh::empty()))),
            _=>unreachable!(),
        }
        if !receipt.fits(grant) {
            return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"face flip allocation exceeded its quote").with_retained_progress(*receipt)));
        }
        Ok(None)
    }

}
