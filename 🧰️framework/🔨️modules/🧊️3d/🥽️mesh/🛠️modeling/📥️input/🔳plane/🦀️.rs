//! 🔳️ Constructs the actual plane topology with independently admitted original backing and entity writes.
use super::super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🧵️ Retains every original plane vertex, halfedge and face through refusal or cancellation.
#[derive(semio_framework_value::RetireOwned)]
pub struct MeshPlaneProduction{width:f32,depth:f32,vertices:Vec<MeshVertex>,halfedges:Vec<HalfEdge>,faces:Vec<MeshFace>,phase:u8,ordinal:usize}
impl MeshPlaneProduction{
    /// 🌱️ Admits dimensions inline before retaining any geometry backing.
    pub fn new(width:f32,depth:f32)->Result<Self,ValueError>{if !width.is_finite()||!depth.is_finite()||width<=0.0||depth<=0.0||width/2.0==0.0||depth/2.0==0.0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original plane dimensions must be finite and positive"))}Ok(Self{width,depth,vertices:Vec::new(),halfedges:Vec::new(),faces:Vec::new(),phase:0,ordinal:0})}
    /// 📊️ Reads retained original plane progress without traversing topology.
    pub fn progress(&self)->f32{(self.phase as f32+self.ordinal as f32/4.0)/7.0}
    pub fn terminal_is_empty(&self)->bool{self.phase==7&&self.vertices.capacity()==0&&self.halfedges.capacity()==0&&self.faces.capacity()==0}
    /// 🎟️ Quotes the selected original backing, entity or whole mesh donation.
    pub fn demand(&self)->semio_framework_value::RetirementDemand{let(copy,capacity)=match self.phase{0=>(0,4*size_of::<MeshVertex>()),1=>(0,4*size_of::<HalfEdge>()),2=>(0,size_of::<MeshFace>()),3=>(size_of::<MeshVertex>(),0),4=>(size_of::<HalfEdge>(),0),5=>(size_of::<MeshFace>(),0),_=>(0,0)};semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()}}
    /// 🪜️ Advances one backing reserve, one actual entity, or one mesh header donation.
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<(Option<HalfedgeMesh>,RetainedCloneProgress),ValueError>{
        if self.phase==7||grant.maximum_items==0{return Ok((None,Default::default()))}let demand=self.demand();if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok((None,Default::default()))}
        let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match self.phase{
            0=>{self.vertices.try_reserve_exact(4).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.vertices.capacity()*size_of::<MeshVertex>();self.phase=1;}
            1=>{self.halfedges.try_reserve_exact(4).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.halfedges.capacity()*size_of::<HalfEdge>();self.phase=2;}
            2=>{self.faces.try_reserve_exact(1).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.faces.capacity()*size_of::<MeshFace>();self.phase=3;}
            3=>{let(a,b)=(self.width/2.0,self.depth/2.0);let position=[[-a,0.0,-b],[a,0.0,-b],[a,0.0,b],[-a,0.0,b]][self.ordinal];self.vertices.push(MeshVertex{position,normal:Some([0.0,-1.0,0.0]),halfedge:(self.ordinal==0).then_some(0)});self.ordinal+=1;if self.ordinal==4{self.ordinal=0;self.phase=4;}}
            4=>{let i=self.ordinal as u32;self.halfedges.push(HalfEdge{vertex:i,twin:None,next:(i+1)%4,face:Some(0),uv:[0.0,0.0]});self.ordinal+=1;if self.ordinal==4{self.ordinal=0;self.phase=5;}}
            5=>{self.faces.push(MeshFace{halfedge:0,smooth:false,flipped:false});self.phase=6;}
            6=>{self.phase=7;return Ok((Some(HalfedgeMesh::from_owned_parts(std::mem::take(&mut self.vertices),std::mem::take(&mut self.halfedges),std::mem::take(&mut self.faces),Default::default(),Default::default(),Default::default(),Default::default())),receipt))}
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original plane production phase is invalid")),
        }
        if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original plane allocator exceeded admitted backing").with_retained_progress(receipt))}Ok((None,receipt))
    }
}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"original plane topology backing reserve failed")}
