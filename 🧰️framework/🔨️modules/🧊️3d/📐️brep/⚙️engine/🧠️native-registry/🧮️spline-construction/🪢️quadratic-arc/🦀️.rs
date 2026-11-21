//! 🪢️ Owns the exact bounded quadratic parameters and per-array work grants.

use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, matrix::Frame3};
use semio_framework_value::{RetainedCloneProgress as Progress, ValueError};
use semio_framework_3d_brep_curve::bspline::KnotVector;
use semio_framework_value::ValueRefusalKind;
use std::mem::{size_of,take};

/// 🧮️ One exact rational quadratic Bézier span in the signed section plane.
#[derive(Clone,Copy)]
pub struct SectionArc {
    pub controls:[Pnt2;3],
    pub middle_weight:f64,
}

/// 📥️ The independently funded curve and trim storage for one section span.
#[derive(Default)]
pub struct ArcBuffers {
    world:Vec<Pnt3>,
    uv:Vec<Pnt2>,
    weights:Vec<f64>,
    knots:Vec<f64>,
}

impl ArcBuffers {
    pub fn next_bytes(&self,world:bool)->usize{
        if self.knots.is_empty(){6*size_of::<f64>()}
        else if self.weights.is_empty(){3*size_of::<f64>()}
        else if world&&self.world.is_empty(){3*size_of::<Pnt3>()}
        else if !world&&self.uv.is_empty(){3*size_of::<Pnt2>()}
        else{0}
    }

    pub fn prepare(&mut self,arc:SectionArc,frame:Frame3,world:bool)->Result<Progress,ValueError>{
        let bytes=self.next_bytes(world);
        let capacity=if self.knots.is_empty(){
            self.knots=exact_vec([0.,0.,0.,1.,1.,1.])?;
            capacity(&self.knots)
        }else if self.weights.is_empty(){
            self.weights=exact_vec([1.,arc.middle_weight,1.])?;
            capacity(&self.weights)
        }else if world{
            self.world=exact_vec(arc.controls.map(|p|frame.origin+frame.x*p.x+frame.y*p.y))?;
            capacity(&self.world)
        }else{
            self.uv=exact_vec(arc.controls)?;
            capacity(&self.uv)
        };
        Ok(Progress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()})
    }

    fn take_knots(&mut self)->KnotVector{KnotVector{degree:2,knots:take(&mut self.knots)}}

    pub fn curve(&mut self)->Curve3{
        Curve3::Nurbs{knots:self.take_knots(),controls:take(&mut self.world),weights:take(&mut self.weights)}
    }

    pub fn pcurve(&mut self)->Curve2{
        Curve2::Nurbs{knots:self.take_knots(),controls:take(&mut self.uv),weights:take(&mut self.weights)}
    }

    pub fn restore_curve(&mut self,curve:Curve3){
        let Curve3::Nurbs{knots,controls,weights}=curve else{unreachable!("funded section curve")};
        self.knots=knots.knots;self.world=controls;self.weights=weights;
    }

    pub fn restore_pcurve(&mut self,curve:Curve2){
        let Curve2::Nurbs{knots,controls,weights}=curve else{unreachable!("funded section trim")};
        self.knots=knots.knots;self.uv=controls;self.weights=weights;
    }

    pub fn release_bytes(&self)->usize{
        [capacity(&self.world),capacity(&self.uv),capacity(&self.weights),capacity(&self.knots)]
            .into_iter().find(|&bytes|bytes!=0).unwrap_or(0)
    }

    pub fn release(&mut self){
        if !self.world.is_empty(){drop(take(&mut self.world));}
        else if !self.uv.is_empty(){drop(take(&mut self.uv));}
        else if !self.weights.is_empty(){drop(take(&mut self.weights));}
        else{drop(take(&mut self.knots));}
    }
}

pub fn curve_capacity(curve:&Curve3)->usize{
    if let Curve3::Nurbs{knots,controls,weights}=curve{
        capacity(&knots.knots)+capacity(controls)+capacity(weights)
    }else{0}
}

pub fn trim_capacity(curve:&Curve2)->usize{
    if let Curve2::Nurbs{knots,controls,weights}=curve{
        capacity(&knots.knots)+capacity(controls)+capacity(weights)
    }else{0}
}

fn capacity<T>(value:&Vec<T>)->usize{value.capacity()*size_of::<T>()}

fn exact_vec<T,const N:usize>(items:[T;N])->Result<Vec<T>,ValueError>{
    let mut out=Vec::new();
    out.try_reserve_exact(N).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"section quadratic allocation failed"))?;
    out.extend(items);
    Ok(out)
}
