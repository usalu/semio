//! 🪷️ Funds exact closed conic topology under the original session.

use super::{remove_topology_slot,store_demand,store_insert,topology_slot_depth};
use semio_framework_3d_brep_curve::{Curve2,Curve3};
use semio_framework_3d_brep_representation::{arena::FaceId,topology::{Body,Coedge,Edge,EntityRef,Face,Loop,Vertex,dummy_coedge,history::PersistentLabel}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt2,Pnt3,Vec2,matrix::Frame3};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,ValueError};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

pub use semio_framework_3d_brep_spline_construction::quadratic_arc::SectionArc;
use semio_framework_3d_brep_spline_construction::quadratic_arc::{self, ArcBuffers};

const FULL: (f64, f64) = (0.0, std::f64::consts::TAU);
const ENTRIES: usize = 23;

#[derive(Clone, Copy)]
pub enum Conic {
    Circle(f64),
    Ellipse(f64, f64),
    Quadratic([SectionArc;2]),
}

impl Conic {
    fn curve(self, frame: Frame3) -> Curve3 {
        match self {
            Self::Circle(radius) => Curve3::Circle { frame, radius },
            Self::Ellipse(major_radius, minor_radius) => Curve3::Ellipse { frame, major_radius, minor_radius },
            Self::Quadratic(_) => unreachable!("quadratic curve requires funded arrays"),
        }
    }

    fn trim(self) -> Curve2 {
        let center = Pnt2::new(0.0, 0.0);
        match self {
            Self::Circle(radius) => Curve2::Circle { center, radius },
            Self::Ellipse(major_radius, minor_radius) => Curve2::Ellipse { center, x_axis: Vec2::new(1.0, 0.0), major_radius, minor_radius },
            Self::Quadratic(_) => unreachable!("quadratic pcurve requires funded arrays"),
        }
    }

    pub fn radii(self) -> Option<[f64;2]> {
        match self {
            Self::Circle(radius) => Some([radius;2]),
            Self::Ellipse(major, minor) => Some([major,minor]),
            Self::Quadratic(_) => None,
        }
    }
}

/// 🎞️ Describes a full conic or the surviving increasing arcs alternating with cap chords.
#[derive(Clone, Copy)]
pub struct ConicBoundary { arcs: [(f64, f64); 2], clipped_arcs: usize }

impl ConicBoundary {
    pub const FULL: Self = Self { arcs: [FULL; 2], clipped_arcs: 0 };

    /// 🔗️ Joins certified increasing conic arcs with the chords between their ends.
    pub fn clipped(first: (f64, f64), second: Option<(f64, f64)>) -> Self {
        Self { arcs: [first, second.unwrap_or(FULL)], clipped_arcs: 1 + usize::from(second.is_some()) }
    }

    pub fn members(self) -> usize { (2 * self.clipped_arcs).max(1) }

    fn arc(self, member: usize) -> Option<(f64, f64)> {
        (member % 2 == 0).then(|| self.arcs[member / 2])
    }

    fn parameter(self, vertex: usize) -> f64 {
        let arc = self.arcs[vertex / 2];
        if vertex % 2 == 0 { arc.0 } else { arc.1 }
    }
}

/// 🎟️ Owns the unpublished topology slots of an exact conic face.
pub struct ConicFacePublication {
    owner: usize,
    frame: Frame3,
    conic: Conic,
    boundary: ConicBoundary,
    buffers: ArcBuffers,
    flipped: bool,
    first_label: u64,
    handle: [u8; 64],
    inserted: [Option<EntityRef>; ENTRIES],
    count: usize,
    stage: usize,
    open: bool,
}

macro_rules! entity {
    ($owner:expr, $index:expr, $kind:ident) => {
        match $owner.inserted[$index] {
            Some(EntityRef::$kind(id)) => id,
            _ => unreachable!("conic topology references an admitted arena record"),
        }
    };
}

impl ConicFacePublication {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("the digest uses lowercase hexadecimal") }
    pub fn topology_ready(&self) -> bool { self.stage == self.link_start() + self.boundary.members() + 2 }
    pub fn terminal_is_empty(&self) -> bool { !self.open }

    /// 🎟️ Binds the exact conic patch to the already reserved original session.
    pub fn new(owner:usize,frame:Frame3,conic:Conic,boundary:ConicBoundary,flipped:bool,first_label:u64,handle:[u8;64])->Self {
        Self { owner,frame,conic,boundary,buffers:ArcBuffers::default(),flipped,first_label,handle,inserted:[None;ENTRIES],count:0,stage:0,open:true }
    }
    pub fn check_owner(&self, owner:usize) -> Result<(), ValueError> {
        if self.open && self.owner == owner { Ok(()) }
        else { Err(ValueError::invariant("conic face requires its original unpublished session")) }
    }
    pub fn face_id(&self)->FaceId { entity!(self,1,Face) }
    pub fn finish(&mut self){self.open=false;}

    fn label(&self, offset: usize) -> PersistentLabel { PersistentLabel(self.first_label + offset as u64) }
    fn member_start(&self, member: usize) -> usize { 3 + self.boundary.members() + 4 * member }
    fn link_start(&self) -> usize { self.member_start(self.boundary.members()) }
    fn rest(&self, vertex: usize) -> usize { (vertex+1) % self.boundary.members() }

    fn rational_stage(&self)->Option<(SectionArc,bool)>{
        let Conic::Quadratic(arcs)=self.conic else{return None;};
        if !(self.member_start(0)..self.link_start()).contains(&self.stage){return None;}
        let offset=self.stage-self.member_start(0);
        let (member,record)=(offset/4,offset%4);
        (member%2==0&&matches!(record,0|2)).then(||(arcs[member/2],record==0))
    }

    fn uv(&self, vertex: usize) -> Pnt2 {
        if let Conic::Quadratic(arcs)=self.conic{return arcs[vertex/2].controls[2*(vertex%2)];}
        let t=self.boundary.parameter(vertex);
        self.conic.trim().eval(t)
    }

    fn world(&self, vertex: usize) -> Pnt3 {
        if matches!(self.conic,Conic::Quadratic(_)){
            let p=self.uv(vertex);
            return self.frame.origin+self.frame.x*p.x+self.frame.y*p.y;
        }
        self.conic.curve(self.frame).eval(self.boundary.parameter(vertex))
    }

    pub fn topology_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        if let Some((_,world))=self.rational_stage(){
            let bytes=self.buffers.next_bytes(world);
            if bytes!=0{return Ok(RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()});}
        }
        let stage=self.stage;
        if stage==0 { return store_demand(&body.surfaces); }
        if stage==1 { return store_demand(&body.faces); }
        if stage==2 { return store_demand(&body.loops); }
        if stage<self.member_start(0) { return store_demand(&body.vertices); }
        if stage<self.link_start() {
            return match (stage-self.member_start(0))%4 {
                0=>store_demand(&body.curves3),
                1=>store_demand(&body.edges),
                2=>store_demand(&body.curves2),
                _=>store_demand(&body.coedges),
            };
        }
        if self.topology_ready() { return Err(ValueError::invariant("conic topology is already complete")); }
        let link=stage-self.link_start();
        let n=self.boundary.members();
        let slot=if link<n { self.member_start(link)+3 } else if link==n { 2 } else { 1 };
        let entity=self.inserted[slot].ok_or_else(|| ValueError::invariant("conic topology requires its original arena slot"))?;
        Ok(RetirementDemand { depth: topology_slot_depth(body,entity)?, ..Default::default() })
    }

    fn insert(&mut self, body: &mut Body, grant: Grant) -> Result<Progress, ValueError> {
        macro_rules! put {
            ($store:ident, $kind:ident, $record:expr) => {{
                let (id, receipt) = store_insert(&mut body.$store, grant, || $record)?;
                (id.map(EntityRef::$kind), receipt)
            }};
        }
        macro_rules! put_quadratic {
            ($store:ident,$kind:ident,$take:ident,$restore:ident) => {{
                let store=&mut body.$store;
                if store.next_insert_capacity_byte_demand(0)?!=0{
                    let receipt=store.reserve_insert_step(grant).map_err(|(error,receipt)|error.with_retained_progress(receipt))?;
                    (None,receipt)
                }else{
                    match store.insert_reserved(self.buffers.$take(),grant){
                        Ok((id,receipt))=>(Some(EntityRef::$kind(id)),receipt),
                        Err((error,value))=>{self.buffers.$restore(value);return Err(error);}
                    }
                }
            }};
        }
        let stage=self.stage;
        let n=self.boundary.members();
        let (entry,receipt)=match stage {
            0=>put!(surfaces, Surface, Surface::Plane { frame:self.frame }),
            1=>put!(faces, Face, Face { surface:entity!(self,0,Surface), outer:None, inners:Vec::new(), flipped:self.flipped, tol:Tol::DEFAULT, label:self.label(2*n) }),
            2=>put!(loops, Loop, Loop { first:dummy_coedge(), face:self.face_id() }),
            _ if stage<self.member_start(0)=>{
                let vertex=stage-3;
                put!(vertices, Vertex, Vertex { position:self.world(vertex), label:self.label(vertex), tol:Tol::DEFAULT })
            }
            _=>{
                let offset=stage-self.member_start(0);
                let member=offset/4;
                let start=self.member_start(member);
                let next=self.rest(member);
                let arc=self.boundary.arc(member);
                let range=arc.unwrap_or((0.0,1.0));
                match offset%4 {
                    0 if self.rational_stage().is_some()=>put_quadratic!(curves3,Curve3,curve,restore_curve),
                    0=>put!(curves3, Curve3, if arc.is_some(){ self.conic.curve(self.frame) }else{
                        let origin=self.world(member);
                        Curve3::Line { origin, dir:self.world(next)-origin }
                    }),
                    1=>put!(edges, Edge, Edge { curve:entity!(self,start,Curve3), range, v0:entity!(self,3+member,Vertex), v1:entity!(self,3+next,Vertex), tol:Tol::DEFAULT, label:self.label(n+member) }),
                    2 if self.rational_stage().is_some()=>put_quadratic!(curves2,Curve2,pcurve,restore_pcurve),
                    2=>put!(curves2, Curve2, if arc.is_some(){ self.conic.trim() }else{
                        let origin=self.uv(member);
                        Curve2::Line { origin, dir:self.uv(next)-origin }
                    }),
                    _=>put!(coedges, Coedge, Coedge { edge:entity!(self,start+1,Edge), forward:true, pcurve:Some(entity!(self,start+2,Curve2)), prange:range, loop_id:entity!(self,2,Loop), next:dummy_coedge(), prev:dummy_coedge() }),
                }
            }
        };
        if let Some(entry)=entry {
            self.inserted[stage]=Some(entry);
            self.stage+=1;
            self.count+=1;
        }
        Ok(receipt)
    }

    fn link(&mut self, body: &mut Body) -> Result<Progress, ValueError> {
        let member=self.stage-self.link_start();
        let n=self.boundary.members();
        let ring=entity!(self,2,Loop);
        if member<n {
            let next=entity!(self,self.member_start(self.rest(member))+3,Coedge);
            let prev=entity!(self,self.member_start((member+n-1)%n)+3,Coedge);
            let id=entity!(self,self.member_start(member)+3,Coedge);
            let use_=body.coedges.get_mut(id).expect("original conic coedge");
            use_.next=next;
            use_.prev=prev;
        }else if member==n {
            body.loops.get_mut(ring).expect("original conic loop").first=entity!(self,self.member_start(0)+3,Coedge);
        }else{
            body.faces.get_mut(self.face_id()).expect("original conic face").outer=Some(ring);
        }
        self.stage+=1;
        Ok(Progress { copied_items:1, ..Default::default() })
    }
}

impl ConicFacePublication {
    /// 🧱️ Advances one funded native topology action.
    pub fn step(&mut self,body:&mut Body,effect:&mut RetainedAllocationEffect<'_>)->Result<Progress,ValueError>{
        let grant=effect.grant();
        if grant.maximum_items==0 || !retained_clone_grant_admits(grant,self.topology_demand(body)?) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        if let Some((arc,world))=self.rational_stage(){
            if self.buffers.next_bytes(world)!=0{return self.buffers.prepare(arc,self.frame,world);}
        }
        if self.stage<self.link_start() { self.insert(body,grant) } else { self.link(body) }
    }

    /// 🧹️ Quotes one native arena removal of an unpublished conic face.
    pub fn close_demand(&self,body:&Body)->Result<RetirementDemand,ValueError>{
        let release_bytes=self.buffers.release_bytes();
        if release_bytes!=0{return Ok(RetirementDemand{depth:1,release_bytes,..Default::default()});}
        let entity=self.count.checked_sub(1).and_then(|i|self.inserted[i]);
        let release_bytes=match entity{
            Some(EntityRef::Curve3(id))=>body.curves3.get(id).map_or(0,quadratic_arc::curve_capacity),
            Some(EntityRef::Curve2(id))=>body.curves2.get(id).map_or(0,quadratic_arc::trim_capacity),
            _=>0,
        };
        let depth=entity.map(|entity|topology_slot_depth(body,entity)).transpose()?.unwrap_or(1);
        Ok(RetirementDemand{depth,release_bytes,..Default::default()})
    }

    /// ♻️ Removes one original unpublished conic record.
    pub fn close(&mut self,body:&mut Body,effect:&mut RetainedAllocationEffect<'_>)->Result<Step,ValueError>{
        let demand=self.close_demand(body)?;
        if effect.grant().maximum_items==0 || !retained_clone_grant_admits(effect.grant(),demand) || !effect.performed(1)? {
            return Ok(Step::Progress(Progress::default()));
        }
        let progress=Progress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()};
        if self.buffers.release_bytes()!=0{self.buffers.release();return Ok(Step::Progress(progress));}
        if self.count==0 {self.finish();return Ok(Step::Complete(progress));}
        self.count-=1;
        remove_topology_slot(body,self.inserted[self.count].take().expect("original conic arena entry"));
        Ok(Step::Progress(progress))
    }
}
