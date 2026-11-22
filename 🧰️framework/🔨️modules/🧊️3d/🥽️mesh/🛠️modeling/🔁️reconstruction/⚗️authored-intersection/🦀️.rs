//! ⚗️ Joins two owned source domains and projects a certified convex intersection.

use super::*;
use semio_framework_3d_mesh_contract::{MeshConvexIntersectionStep,MeshIntersectionFace,MeshIntersectionKernel,MeshIntersectionSolid};
use semio_framework_mesh_engine::PolygonMeshSource;
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;

#[path="🎨️projection/🦀️.rs"]
mod projection;
use projection::Projection;

fn empty()->PolygonMeshSource{
    PolygonMeshSource{vertices:Vec::new(),faces:Vec::new(),surface:Vec::new(),attributes:HistoryFoldIndex::new(),
        materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()}
}
fn refuse(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn kernel(error:MeshKernelError)->ValueError{
    match error{MeshKernelError::Retained(error)=>error,_=>ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())}
}

#[derive(Default,semio_framework_value::RetireOwned)]
struct GeometryCopy {soup:Soup,phase:u8,corner:usize}
impl GeometryCopy {
    fn demand(&self,source:&PolygonMeshSource)->RetirementDemand{
        let mut demand=RetirementDemand{depth:1,..Default::default()};
        match self.phase{
            0=>demand.capacity_bytes=source.vertices.len()*size_of::<[f32;3]>(),
            1=>demand.capacity_bytes=source.faces.len()*size_of::<Vec<u32>>(),
            2 if self.soup.positions.len()<source.vertices.len()=>demand.copy_bytes=size_of::<[f32;3]>(),
            3=>if let Some(ring)=source.faces.get(self.soup.faces.len()){demand.capacity_bytes=ring.len()*size_of::<u32>();},
            4=>demand.copy_bytes=size_of::<u32>(),
            _=>{},
        }
        demand
    }

    fn step(&mut self,source:&PolygonMeshSource,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
        match self.phase{
            0=>{receipt.retained_capacity_bytes=reserve_polygon_backing(&mut self.soup.positions,source.vertices.len(),effect)?;self.phase=1;},
            1=>{receipt.retained_capacity_bytes=reserve_polygon_backing(&mut self.soup.faces,source.faces.len(),effect)?;self.phase=2;},
            2=>{
                if let Some(point)=source.vertices.get(self.soup.positions.len()){self.soup.positions.push(*point);receipt.copied_bytes=size_of::<[f32;3]>();}
                else{self.phase=3;}
            },
            3=>{
                let Some(ring)=source.faces.get(self.soup.faces.len())else{return Ok(true);};
                if ring.len()<3||ring.len()>600_000{return Err(refuse("invalid solid face extent"));}
                let mut copy=Vec::new();
                receipt.retained_capacity_bytes=reserve_polygon_backing(&mut copy,ring.len(),effect)?;
                self.soup.faces.push(copy);self.phase=4;self.corner=0;
            },
            4=>{
                let ring=&source.faces[self.soup.faces.len()-1];
                self.soup.faces.last_mut().unwrap().push(ring[self.corner]);
                receipt.copied_bytes=size_of::<u32>();self.corner+=1;
                if self.corner==ring.len(){self.phase=3;}
            },
            _=>unreachable!(),
        }
        Ok(false)
    }
}

/// 🪜️ Produces a canonical authored polygon source from two certified closed mesh owners.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct MeshAuthoredIntersection<K:MeshIntersectionKernel>{
    inputs:[Option<PolygonMeshSource>;2],copy:[GeometryCopy;2],cut:Option<K>,
    solid:Option<MeshIntersectionSolid>,join:Option<PolygonCombine>,projection:Option<Projection>,
    owners:[bool;2],left_faces:usize,left_vertices:u32,cursor:usize,phase:u8,cancelled:bool,receipt:[usize;4],
}

impl<K:MeshIntersectionKernel> MeshAuthoredIntersection<K>{
    /// 📥️ Stages both source owners without traversing them.
    pub fn new(left:PolygonMeshSource,right:PolygonMeshSource)->Self{
        let left_faces=left.faces.len();
        let left_vertices=left.vertices.len() as u32;
        Self{inputs:[Some(left),Some(right)],copy:Default::default(),cut:None,solid:None,join:None,projection:None,
            owners:[false;2],left_faces,left_vertices,cursor:0,phase:0,cancelled:false,receipt:[0;4]}
    }

    /// 🛑️ Stops before publication and retains pending data for retirement.
    pub fn cancel(&mut self){self.cancelled=true;}

    /// 📈️ Reports progress through geometry capture, clipping and authored projection.
    pub fn progress(&self)->f32{match self.phase{0=>0.0,1=>0.1,2=>0.2,3=>0.55,4=>0.6,5=>0.7+0.29*self.projection.as_ref().unwrap().progress(),_=>1.0}}

    /// 🎟️ Quotes the next event and forwards the shared join's physical capacity demands.
    pub fn demand(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.cancelled{return Ok(Default::default());}
        if self.phase<2 {let i=self.phase as usize;return Ok(self.copy[i].demand(self.inputs[i].as_ref().unwrap()));}
        if let Some(join)=&self.join{return join.demand(copy);}
        if let Some(projection)=&self.projection {if self.phase==5{return projection.demand(copy);}}
        Ok(RetirementDemand{depth:1,..Default::default()})
    }

    /// 🧾️ Transfers the last event's applied receipt.
    pub fn take_progress(&mut self)->RetainedCloneProgress{
        let [copied_items,copied_bytes,retained_capacity_bytes,released_bytes]=std::mem::take(&mut self.receipt);
        RetainedCloneProgress{copied_items,copied_bytes,retained_capacity_bytes,released_bytes}
    }
    fn receipt(&mut self,p:RetainedCloneProgress){self.receipt=[p.copied_items,p.copied_bytes,p.retained_capacity_bytes,p.released_bytes];}

    /// 🪵️ Advances one admitted event, or publishes one complete result.
    pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<PolygonMeshSource>,ValueError>{
        self.receipt=[0;4];
        if self.cancelled{return Err(ValueError::literal(ValueRefusalKind::Canceled,"mesh intersection cancelled"));}
        if self.phase>=6{return Err(refuse("mesh intersection already settled"));}
        let result=self.advance(effect);
        if result.is_err(){self.phase=7;}
        result
    }

    fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<PolygonMeshSource>,ValueError>{
        if let Some(join)=&mut self.join{
            let result=join.step(effect);
            let receipt=join.take_progress();self.receipt(receipt);
            if let Some(source)=result?{
                self.projection=Some(Projection::new(self.solid.take().unwrap(),source,self.left_faces,self.left_vertices));
                self.phase=5;self.join=None;
            }
            return Ok(None);
        }
        let grant=effect.grant();
        if !retained_clone_grant_admits(grant,self.demand(grant.maximum_copy_bytes)?)||!effect.performed(1)?{return Ok(None);}
        let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
        let result=self.event(effect,&mut receipt);
        self.receipt(receipt);
        result
    }

    fn event(&mut self,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<Option<PolygonMeshSource>,ValueError>{
        match self.phase{
            0|1=>{
                let i=self.phase as usize;
                if self.copy[i].step(self.inputs[i].as_ref().unwrap(),effect,receipt)?{self.phase+=1;}
            },
            2=>{
                if self.cut.is_none(){
                    let [left,right]=&mut self.copy;
                    self.cut=Some(K::create(std::mem::take(&mut left.soup),std::mem::take(&mut right.soup)).map_err(kernel)?);
                }else{
                    match self.cut.as_mut().unwrap().advance().map_err(kernel)?{
                        MeshConvexIntersectionStep::Working=>{},
                        MeshConvexIntersectionStep::Cancelled=>return Err(ValueError::literal(ValueRefusalKind::Canceled,"mesh intersection cancelled")),
                        MeshConvexIntersectionStep::Empty=>{self.phase=6;return Ok(Some(empty()));},
                        MeshConvexIntersectionStep::Solid(solid)=>{self.solid=Some(solid);self.phase=3;},
                    }
                }
            },
            3=>{
                let solid=self.solid.as_ref().unwrap();
                if let Some(side)=solid.retained_operand{
                    self.phase=6;return Ok(self.inputs[side as usize].take());
                }else if let Some(face)=solid.faces.get(self.cursor){
                    self.owners[face.source_face().0]=true;
                    self.cursor+=1;
                }else if !self.owners[1]&&solid.points.len()==self.inputs[0].as_ref().unwrap().vertices.len(){
                    self.phase=6;
                    return Ok(self.inputs[0].take());
                }else{self.phase=4;}
            },
            4=>{
                if self.owners==[true,true] {
                    let [left,right]=&mut self.inputs;
                    match PolygonCombine::new(left.take().unwrap(),right.take().unwrap()){
                        Ok(join)=>self.join=Some(join),
                        Err((error,a,b))=>{*left=Some(a);*right=Some(b);return Err(error);}
                    }
                }else{
                    let side=usize::from(self.owners[1]);
                    self.projection=Some(Projection::new(self.solid.take().unwrap(),self.inputs[side].take().unwrap(),0,0));
                    self.phase=5;
                }
            },
            5=>if let Some(output)=self.projection.as_mut().unwrap().step(effect,receipt)?{
                self.phase=6;return Ok(Some(output));
            },
            _=>unreachable!(),
        }
        Ok(None)
    }
}
