//! 📇️ Retains canonical polygon topology and authored data through funded native capture.

use super::*;
use semio_framework_mesh_engine::{PolygonMeshSource,PolygonSurfaceKind,PolygonSurfaceSample};
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneProgress,retained_clone_grant_admits};
use std::mem::size_of;

#[path="🎨️metadata/🦀️.rs"]
mod metadata;
use metadata::Metadata;

/// 🏁️ Status of one funded polygon source capture turn.
pub enum MeshPolygonSourceStep {
    Working,
    Done(PolygonMeshSource),
    Cancelled,
}

/// 📸️ Captures a polygon source in funded turns.
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct MeshPolygonSourceCapture {
    output:Option<PolygonMeshSource>,
    metadata:Metadata,
    raw:Vec<u32>,
    corners:Vec<u32>,
    edges:Vec<u32>,
    remapped:[bool;2],
    face:Vec<u32>,
    face_start:usize,
    next:Option<u32>,
    cursor:usize,
    samples:usize,
    surface_kind:usize,
    units:usize,
    source:Option<usize>,
    phase:u8,
    cancelled:bool,
}

fn absent(reason:&'static str)->ValueError {ValueError::literal(ValueRefusalKind::InvalidValue,reason)}

fn bytes<T>(count:usize)->Result<usize,ValueError> {
    count.checked_mul(size_of::<T>()).ok_or_else(||absent("polygon source backing extent overflow"))
}

/// 📦️ Reserves an exact native vector backing under an admitted polygon source effect.
pub fn reserve_polygon_backing<T>(target:&mut Vec<T>,count:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<usize,ValueError> {
    if count==0 {return Ok(0);}
    if target.capacity()!=0 || effect.actual().is_some(){return Err(ValueError::invariant("polygon allocation frontier changed"));}
    let result=target.try_reserve_exact(count);
    let bytes=target.capacity()*size_of::<T>();
    *effect.actual()=Some(bytes);
    result.map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"polygon backing allocation refused"))?;
    if bytes!=count*size_of::<T>() {return Err(ValueError::invariant("polygon allocation differed from its quote"));}
    Ok(bytes)
}

fn reserved<T>(vec:&mut Vec<T>,len:usize,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<(),ValueError>{
    receipt.retained_capacity_bytes=reserve_polygon_backing(vec,len,effect)?;
    Ok(())
}

fn canonical_rows(raw:&[u32],flipped:bool,index:usize)->[u32;2]{
    if !flipped {return [raw[index];2];}
    let reversed=raw.len()-1-index;
    [raw[reversed],raw[(reversed+raw.len()-1)%raw.len()]]
}

impl Default for MeshPolygonSourceCapture {fn default()->Self {Self::new()}}

impl MeshPolygonSourceCapture {
    /// 🌱️ Creates an empty capture cursor without copying the source mesh.
    pub fn new()->Self {
        let output=PolygonMeshSource{vertices:Vec::new(),faces:Vec::new(),surface:Vec::new(),attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()};
        Self {output:Some(output),metadata:Metadata::default(),raw:Vec::new(),corners:Vec::new(),edges:Vec::new(),remapped:[false;2],
            face:Vec::new(),face_start:0,next:None,cursor:0,samples:0,surface_kind:0,units:0,source:None,phase:0,cancelled:false}
    }

    /// 📊️ Reports completed turns and estimated total.
    pub fn progress(&self,mesh:&HalfedgeMesh)->(usize,usize) {
        if self.output.is_none(){return (self.units,self.units);}
        let estimate=32+4*mesh.vertex_count()+6*mesh.face_count()+8*mesh.halfedge_count()+8*(mesh.attributes.len()+mesh.materials.len()+mesh.textures.len());
        (self.units,estimate.max(self.units.saturating_add(1)))
    }

    /// 🛑️ Stops publication.
    pub fn cancel(&mut self){self.cancelled=true;}

    fn source_identity(&self,mesh:&HalfedgeMesh)->Result<usize,ValueError>{
        let address=std::ptr::from_ref(mesh) as usize;
        if self.source.is_some_and(|source|source!=address){return Err(absent("polygon capture source changed"));}
        Ok(address)
    }

    fn sample(&self,mesh:&HalfedgeMesh)->Option<PolygonSurfaceSample>{
        let kind=[PolygonSurfaceKind::VertexNormal,PolygonSurfaceKind::SmoothFace,PolygonSurfaceKind::CornerUv,PolygonSurfaceKind::SeamEdge][self.surface_kind];
        let index=self.cursor as u32;
        let source=match self.surface_kind {
            0|1=>index,
            2=>*self.corners.get(self.cursor)?,
            _=>*self.edges.get(self.cursor)?,
        };
        mesh.polygon_surface_sample(kind,index,source)
    }

    fn sample_limit(&self,mesh:&HalfedgeMesh)->usize {
        [mesh.vertex_count(),mesh.face_count(),self.corners.len(),self.edges.len()][self.surface_kind]
    }

    fn topology(&self)->metadata::Remap<'_>{metadata::Remap{corners:&self.corners,edges:&self.edges,changed:self.remapped}}

    /// 🎟️ Quotes the next turn.
    pub fn demand(&self,mesh:&HalfedgeMesh,copy:usize)->Result<RetirementDemand,ValueError>{
        self.source_identity(mesh)?;
        let mut demand=RetirementDemand{depth:1,..Default::default()};
        let Some(output)=self.output.as_ref().filter(|_|!self.cancelled)else{return Ok(demand);};
        match self.phase {
            0=>demand.capacity_bytes=bytes::<[f32;3]>(mesh.vertex_count())?,
            1=>demand.capacity_bytes=bytes::<Vec<u32>>(mesh.face_count())?,
            2..=4=>demand.capacity_bytes=bytes::<u32>(mesh.halfedge_count())?,
            5 if self.cursor<mesh.vertex_count()=>demand.copy_bytes=size_of::<[f32;3]>(),
            6 if self.next.is_some()=>demand.copy_bytes=size_of::<u32>(),
            7=>demand.capacity_bytes=bytes::<u32>(self.raw.len()-self.face_start)?,
            8 if self.face.len()<self.raw.len()-self.face_start=>demand.copy_bytes=bytes::<u32>(3)?,
            8=>demand.copy_bytes=size_of::<Vec<u32>>(),
            10=>demand.capacity_bytes=bytes::<PolygonSurfaceSample>(self.samples)?,
            11 if self.cursor<self.sample_limit(mesh)&&self.sample(mesh).is_some()=>demand.copy_bytes=size_of::<PolygonSurfaceSample>(),
            12=>return self.metadata.demand(mesh,output,self.topology(),copy),
            _=>{},
        }
        Ok(demand)
    }

    /// 🪜️ Advances one funded event.
    pub fn step(&mut self,mesh:&HalfedgeMesh,effect:&mut RetainedAllocationEffect<'_>)->Result<(MeshPolygonSourceStep,RetainedCloneProgress),ValueError>{
        use MeshPolygonSourceStep::*;
        let identity=self.source_identity(mesh)?;
        if self.cancelled {return Ok((Cancelled,Default::default()));}
        if self.output.is_none(){return Err(absent("polygon capture output already transferred"));}
        let grant=effect.grant();
        let demand=self.demand(mesh,grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant,demand)||!effect.performed(1)? {return Ok((Working,Default::default()));}
        self.source=Some(identity);
        let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
        if self.phase==12 {
            let remap=metadata::Remap{corners:&self.corners,edges:&self.edges,changed:self.remapped};
            if self.metadata.step(mesh,self.output.as_mut().unwrap(),remap,effect,&mut receipt)? {self.phase=13;}
        }else if self.phase==13 {
            self.units+=1;
            return Ok((Done(self.output.take().unwrap()),receipt));
        }else if self.phase==9||self.phase==11 {
            self.surface_step(mesh,&mut receipt);
        }else {
            let out=self.output.as_mut().unwrap();
            match self.phase {
                0=>{reserved(&mut out.vertices,mesh.vertex_count(),effect,&mut receipt)?;self.phase=1;},
                1=>{reserved(&mut out.faces,mesh.face_count(),effect,&mut receipt)?;self.phase=2;},
                2=>{reserved(&mut self.raw,mesh.halfedge_count(),effect,&mut receipt)?;self.phase=3;},
                3=>{reserved(&mut self.corners,mesh.halfedge_count(),effect,&mut receipt)?;self.phase=4;},
                4=>{reserved(&mut self.edges,mesh.halfedge_count(),effect,&mut receipt)?;self.phase=5;},
                5 if self.cursor<mesh.vertex_count()=>{out.vertices.push(mesh.vertices[self.cursor].position);self.cursor+=1;receipt.copied_bytes=demand.copy_bytes;},
                5=>{self.cursor=0;self.phase=6;},
                6=>{
                    let rank=out.faces.len();
                    if let Some(face)=mesh.faces.get(rank){
                        if let Some(next)=self.next {
                            let edge=mesh.halfedges.get(next as usize).filter(|edge|edge.face==Some(rank as u32)&&(edge.vertex as usize)<mesh.vertex_count()).ok_or_else(||absent("invalid polygon boundary corner"))?;
                            if self.raw.len()==mesh.halfedge_count(){return Err(absent("polygon boundary does not close"));}
                            self.raw.push(next);receipt.copied_bytes=demand.copy_bytes;
                            if edge.next==face.halfedge {
                                if self.raw.len()-self.face_start<3{return Err(absent("polygon boundary needs three corners"));}
                                self.next=None;self.phase=7;
                            }else{self.next=Some(edge.next);}
                        }else{self.face_start=self.raw.len();self.next=Some(face.halfedge);}
                    }else{
                        if self.raw.len()!=mesh.halfedge_count(){return Err(absent("polygon boundary does not cover the source corners"));}
                        self.phase=9;
                    }
                },
                7=>{reserved(&mut self.face,self.raw.len()-self.face_start,effect,&mut receipt)?;self.phase=8;},
                8=>{
                    let row=&self.raw[self.face_start..];
                    if self.face.len()==row.len(){out.faces.push(std::mem::take(&mut self.face));self.phase=6;}
                    else{
                        let [corner,edge]=canonical_rows(row,mesh.faces[out.faces.len()].flipped,self.face.len());
                        self.remapped[0]|=corner as usize!=self.corners.len();self.remapped[1]|=edge as usize!=self.edges.len();
                        self.corners.push(corner);self.edges.push(edge);self.face.push(mesh.halfedges[corner as usize].vertex);
                    }
                    receipt.copied_bytes=demand.copy_bytes;
                },
                10=>{reserved(&mut out.surface,self.samples,effect,&mut receipt)?;self.phase=11;},
                _=>unreachable!(),
            }
        }
        self.units+=1;
        Ok((Working,receipt))
    }

    fn surface_step(&mut self,mesh:&HalfedgeMesh,receipt:&mut RetainedCloneProgress){
        if self.cursor==self.sample_limit(mesh){
            self.cursor=0;self.surface_kind+=1;
            if self.surface_kind==4{self.surface_kind=0;self.phase+=1;}
        }else{
            if let Some(sample)=self.sample(mesh){
                if self.phase==9 {self.samples+=1;}
                else {self.output.as_mut().unwrap().surface.push(sample);receipt.copied_bytes=size_of::<PolygonSurfaceSample>();}
            }
            self.cursor+=1;
        }
    }
}
