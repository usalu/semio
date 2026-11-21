//! 🕳️ Funds one translated circular opening on an exact planar face.

use super::{remove_topology_slot, store_demand, store_insert, topology_slot_depth};
use semio_framework_3d_brep_construction::primitives::planar_pcurve;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::{CoedgeId,Curve3Id,FaceId,LoopId},topology::{Body,Coedge,Edge,EntityRef,Loop,Vertex,dummy_coedge,history::PersistentLabel}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use semio_framework_value::{RetirementDemand,RetainedAllocationEffect,RetainedCloneProgress as Progress,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::{size_of,take};

const RANGE:(f64,f64)=(0.,std::f64::consts::TAU);

/// 🕳️ Stages the original hole and independently removes each unpublished native record.
pub struct CircularOpeningOffset {
    source: FaceId,
    ring: LoopId,
    labels:[PersistentLabel;2],
    curve:Option<Curve3Id>,
    circle:Option<(Frame3,f64)>,
    forward:bool,
    side:Option<CoedgeId>,
    entries:[Option<EntityRef>;6],
    used:usize,
    stage:u8,
}

macro_rules! entity {
    ($opening:expr,$index:literal,$kind:ident) => {
        match $opening.entries[$index] {
            Some(EntityRef::$kind(id)) => id,
            _ => unreachable!("opening references its reserved native record"),
        }
    };
}

fn invalid()->ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained planar offset requires one complete circular hole strictly clear of its line boundary")
}

impl CircularOpeningOffset {
    pub fn new(source:FaceId, ring:LoopId, labels:[PersistentLabel;2])->Self {
        Self { source,ring,labels,curve:None,circle:None,forward:false,side:None,entries:[None;6],used:0,stage:0 }
    }
    pub fn ready(&self)->bool { self.stage==13 }

    pub(super) fn orientation(frame:&Frame3)->bool {
        let [x,y,z]=[frame.x,frame.y,frame.z];
        let e=128.*f64::EPSILON;
        [x,y,z].into_iter().all(|axis|(axis.norm()-1.).abs()<=e)
            && x.dot(y).abs()<=e && (x.cross(y)-z).norm()<=e
    }

    fn source_member<'a>(&self,body:&'a Body)->Result<(CoedgeId,&'a Coedge,&'a Edge),ValueError> {
        opening_member(body,self.source,self.ring).ok_or_else(invalid)
    }

    fn outer_member<'a>(&self,body:&'a Body,target:FaceId)->Result<(LoopId,CoedgeId,CoedgeId,&'a Coedge,&'a Edge),ValueError> {
        let ring_id=body.faces.get(target).and_then(|face|face.outer).ok_or_else(invalid)?;
        let first=body.loops.get(ring_id).ok_or_else(invalid)?.first;
        let id=self.side.unwrap_or(first);
        let use_=body.coedges.get(id).filter(|row|row.loop_id==ring_id).ok_or_else(invalid)?;
        let edge=body.edges.get(use_.edge).ok_or_else(invalid)?;
        Ok((ring_id,first,id,use_,edge))
    }

    pub fn demand(&self,body:&Body,target:FaceId)->Result<RetirementDemand,ValueError> {
        let target_depth=||body.faces.next_remove_depth_demand(target);
        let base=|depth,copy_bytes|Ok(RetirementDemand{depth,copy_bytes,..Default::default()});
        match self.stage {
            0 => {
                let (id,use_,_)=self.source_member(body)?;
                base(body.loops.next_remove_depth_demand(self.ring)?.max(body.coedges.next_remove_depth_demand(id)?)
                    .max(body.edges.next_remove_depth_demand(use_.edge)?),size_of::<Curve3Id>()+size_of::<bool>())
            }
            1 => {
                let source=self.curve.ok_or_else(invalid)?;
                base(target_depth()?.max(body.curves3.next_remove_depth_demand(source)?),13*size_of::<f64>())
            }
            2 => {
                let (ring,_,id,use_,edge)=self.outer_member(body,target)?;
                let depth=target_depth()?.max(body.loops.next_remove_depth_demand(ring)?)
                    .max(body.coedges.next_remove_depth_demand(id)?).max(body.edges.next_remove_depth_demand(use_.edge)?)
                    .max(body.vertices.next_remove_depth_demand(edge.v0)?).max(body.vertices.next_remove_depth_demand(edge.v1)?);
                base(depth,6*size_of::<f64>())
            }
            3 => store_demand(&body.vertices),
            4 => store_demand(&body.curves3),
            5 => store_demand(&body.edges),
            6 => store_demand(&body.loops),
            7 => store_demand(&body.curves2),
            8 => store_demand(&body.coedges),
            9 => base(body.coedges.next_remove_depth_demand(entity!(self,5,Coedge))?,0),
            10 => base(body.loops.next_remove_depth_demand(entity!(self,3,Loop))?,0),
            11 => Ok(RetirementDemand{depth:target_depth()?,capacity_bytes:size_of::<LoopId>(),..Default::default()}),
            12 => base(target_depth()?,size_of::<LoopId>()),
            _ => Err(ValueError::invariant("circular opening has already completed")),
        }
    }

    pub fn step(&mut self,body:&mut Body,target:FaceId,plane:Frame3,offset:Vec3,tolerance:semio_framework_3d_brep_tolerance::Tol,effect:&mut RetainedAllocationEffect<'_>)->Result<Progress,ValueError> {
        let grant=effect.grant();
        let demand=self.demand(body,target)?;
        if grant.maximum_items==0 || !retained_clone_grant_admits(grant,demand) || !effect.performed(1)? { return Ok(Default::default()); }
        let mut receipt=Progress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match self.stage {
            0 => {
                let (_,use_,edge)=self.source_member(body)?;
                self.curve=Some(edge.curve);self.forward=use_.forward;
            }
            1 => {
                let Some(Curve3::Circle{frame,radius})=body.curves3.get(self.curve.ok_or_else(invalid)?) else {return Err(invalid());};
                let mut frame=*frame;frame.origin=frame.origin+offset;
                let alignment=frame.z.dot(plane.z);
                if !Self::orientation(&plane) || !Self::orientation(&frame)
                    || (alignment.abs()-1.).abs()>128.*f64::EPSILON || (alignment>0.)==self.forward
                    || !((frame.origin-plane.origin).dot(plane.z).abs()<=tolerance.value())
                    || !radius.is_finite() || *radius<=tolerance.value()
                { return Err(invalid()); }
                self.circle=Some((frame,*radius));
            }
            2 => {
                let (_,first,_,use_,edge)=self.outer_member(body,target)?;
                let (a,b)=if use_.forward{(edge.v0,edge.v1)}else{(edge.v1,edge.v0)};
                let a=body.vertices.get(a).ok_or_else(invalid)?.position;
                let b=body.vertices.get(b).ok_or_else(invalid)?.position;
                let inside=plane.z.cross(b-a).normalized().ok_or_else(invalid)?;
                let (frame,radius)=self.circle.ok_or_else(invariant_circle)?;
                if !((frame.origin-a).dot(inside)>radius+tolerance.value()) { return Err(invalid()); }
                if use_.next!=first { self.side=Some(use_.next);return Ok(receipt); }
            }
            3..=8 => return self.insert(body,target,plane,tolerance,grant),
            9 => {
                let id=entity!(self,5,Coedge);
                let use_=body.coedges.get_mut(id).ok_or_else(invalid)?;
                use_.prev=id;use_.next=id;
            }
            10 => body.loops.get_mut(entity!(self,3,Loop)).ok_or_else(invalid)?.first=entity!(self,5,Coedge),
            11 => {
                let inners=&mut body.faces.get_mut(target).ok_or_else(invalid)?.inners;
                if inners.capacity()!=0 { return Err(ValueError::invariant("original inner ring storage already exists")); }
                inners.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"planar inner ring allocation failed"))?;
                receipt.retained_capacity_bytes=inners.capacity()*size_of::<LoopId>();
                if !receipt.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original inner ring capacity exceeded grant").with_retained_progress(receipt)); }
            }
            12 => body.faces.get_mut(target).ok_or_else(invalid)?.inners.push(entity!(self,3,Loop)),
            _ => return Err(ValueError::invariant("circular opening has already completed")),
        }
        self.stage+=1;
        Ok(receipt)
    }

    fn insert(&mut self,body:&mut Body,target:FaceId,plane:Frame3,tol:semio_framework_3d_brep_tolerance::Tol,grant:semio_framework_value::RetainedCloneGrant)->Result<Progress,ValueError> {
        macro_rules! put {
            ($store:ident,$kind:ident,$value:expr) => {{
                let (id,receipt)=store_insert(&mut body.$store,grant,||$value)?;
                (id.map(EntityRef::$kind),receipt)
            }};
        }
        let (frame,radius)=self.circle.ok_or_else(invariant_circle)?;
        let curve=Curve3::Circle{frame,radius};
        let (entity,receipt)=match self.stage {
            3=>put!(vertices,Vertex,Vertex{position:curve.eval(0.),tol,label:self.labels[0]}),
            4=>put!(curves3,Curve3,curve),
            5=>put!(edges,Edge,Edge{curve:entity!(self,1,Curve3),range:RANGE,v0:entity!(self,0,Vertex),v1:entity!(self,0,Vertex),tol,label:self.labels[1]}),
            6=>put!(loops,Loop,Loop{face:target,first:dummy_coedge()}),
            7=>put!(curves2,Curve2,planar_pcurve(&curve,plane)),
            8=>put!(coedges,Coedge,Coedge{edge:entity!(self,2,Edge),forward:self.forward,prange:RANGE,pcurve:Some(entity!(self,4,Curve2)),loop_id:entity!(self,3,Loop),next:dummy_coedge(),prev:dummy_coedge()}),
            _=>return Err(ValueError::invariant("invalid circular opening insert")),
        };
        if let Some(entity)=entity { self.entries[self.used]=Some(entity);self.used+=1;self.stage+=1; }
        Ok(receipt)
    }

    pub fn close_demand(&self,body:&Body,target:FaceId)->Result<Option<RetirementDemand>,ValueError> {
        let capacity=body.faces.get(target).ok_or_else(invalid)?.inners.capacity();
        if capacity!=0 { return Ok(Some(RetirementDemand{depth:body.faces.next_remove_depth_demand(target)?,release_bytes:capacity*size_of::<LoopId>(),..Default::default()})); }
        self.used.checked_sub(1).and_then(|index|self.entries[index]).map(|entity|
            Ok(RetirementDemand{depth:topology_slot_depth(body,entity)?,..Default::default()})).transpose()
    }

    pub fn close(&mut self,body:&mut Body,target:FaceId,demand:RetirementDemand)->Result<Progress,ValueError> {
        if demand.release_bytes!=0 { drop(take(&mut body.faces.get_mut(target).ok_or_else(invalid)?.inners)); }
        else { self.used-=1;remove_topology_slot(body,self.entries[self.used].take().ok_or_else(invalid)?); }
        Ok(Progress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})
    }
}

fn invariant_circle()->ValueError { ValueError::invariant("circular opening lacks its exact source recipe") }

pub(super) fn opening_member(body:&Body,source:FaceId,ring_id:LoopId)->Option<(CoedgeId,&Coedge,&Edge)> {
    let ring=body.loops.get(ring_id).filter(|row|row.face==source)?;
    let use_=body.coedges.get(ring.first).filter(|row|row.loop_id==ring_id && row.next==ring.first && row.prev==ring.first)?;
    let edge=body.edges.get(use_.edge).filter(|row|row.v0==row.v1 && row.range==RANGE && use_.prange==RANGE)?;
    Some((ring.first,use_,edge))
}
