//! 📐️ Inverts the rational bilinear image of a simple planar quadrilateral.

use super::{Pnt3,Vec3};

type Uv = [f64;2];
const EPSILON:f64=256.0*f64::EPSILON;
const PARAMETER_TOLERANCE:f64=1e-10;

fn delta(a:Uv,b:Uv)->Uv{[a[0]-b[0],a[1]-b[1]]}
fn dot(a:Uv,b:Uv)->f64{a[0]*b[0]+a[1]*b[1]}
fn wedge(a:Uv,b:Uv)->f64{a[0]*b[1]-a[1]*b[0]}
fn length(a:Uv)->f64{a[0].hypot(a[1])}

fn orient(a:Uv,b:Uv,c:Uv)->f64{wedge(delta(b,a),delta(c,a))}

fn scaled(a:Vec3,scale:f64)->Vec3{Vec3::new(a.x/scale,a.y/scale,a.z/scale)}

fn projected_chart(points:[Pnt3;4],target:Pnt3)->Option<([Uv;4],Uv)>{
    let origin=points[0];
    let offsets=points.map(|p|p-origin);
    let scale=offsets.iter().flat_map(|p|[p.x,p.y,p.z]).map(f64::abs).fold(0.,f64::max);
    if !scale.is_finite()||scale==0. {return None;}
    let local=offsets.map(|p|scaled(p,scale));
    let x=local[1].normalized()?;
    let normal=x.cross(local[3]).normalized()?;
    if local.iter().any(|&p| !((p.dot(normal)).abs()<=EPSILON)){return None;}
    let y=normal.cross(x);
    let project=|p:Vec3|[p.dot(x),p.dot(y)];
    let ring=local.map(project);
    if (0..4).any(|i| !(orient(ring[i],ring[(i+1)%4],ring[(i+2)%4])>EPSILON)){return None;}
    let target=project(scaled(target-origin,scale));
    target.into_iter().all(f64::is_finite).then_some((ring,target))
}

fn closest_on_ring(ring:[Uv;4],target:Uv)->Uv{
    if (0..4).all(|i|orient(ring[i],ring[(i+1)%4],target)>=0.){return target;}
    let mut best=(ring[0],f64::INFINITY);
    for i in 0..4{
        let a=ring[i];
        let segment=delta(ring[(i+1)%4],a);
        let fraction=(dot(delta(target,a),segment)/dot(segment,segment)).clamp(0.,1.);
        let point=std::array::from_fn(|axis|segment[axis].mul_add(fraction,a[axis]));
        let distance=length(delta(point,target));
        if distance<best.1{best=(point,distance);}
    }
    best.0
}

fn coefficients([p00,p10,p11,p01]:[f64;4])->[f64;4]{
    [p00,p10-p00,p01-p00,p11-p10-p01+p00]
}

fn roots([c,b,a]:[f64;3])->[Option<f64>;2]{
    let scale=c.abs().max(b.abs()).max(a.abs());
    if scale==0.||!scale.is_finite(){return [None;2];}
    let [c,b,a]=[c,b,a].map(|n|n/scale);
    if a==0.{return [(b!=0.).then(||-c/b),None];}
    let discriminant=b.mul_add(b,-4.*a*c);
    if discriminant< -EPSILON{return [None;2];}
    let q= -0.5*(b+discriminant.max(0.).sqrt().copysign(b));
    if q==0.{[Some(-b/(2.*a)),None]}else{[Some(q/a),Some(c/q)]}
}

fn parameter(value:f64)->Option<f64>{
    (-PARAMETER_TOLERANCE..=1.+PARAMETER_TOLERANCE).contains(&value).then(||value.clamp(0.,1.))
}

fn invert(ring:[Uv;4],weights:[f64;4],foot:Uv)->Option<Uv>{
    let [a,b,c,d]=coefficients(std::array::from_fn(|i|weights[i]*(ring[i][0]-foot[0])));
    let [e,f,g,h]=coefficients(std::array::from_fn(|i|weights[i]*(ring[i][1]-foot[1])));
    let polynomial=[a*g-e*c,a*h+b*g-e*d-f*c,b*h-f*d];
    roots(polynomial).into_iter().flatten().find_map(|s|{
        let u=parameter(s)?;
        let left=d.mul_add(u,c);
        let right=h.mul_add(u,g);
        let t=if left.abs()>=right.abs(){-b.mul_add(u,a)/left}else{-f.mul_add(u,e)/right};
        let v=parameter(t)?;
        let basis=[(1.-u)*(1.-v),u*(1.-v),u*v,(1.-u)*v];
        let coeff: [f64;4]=std::array::from_fn(|i|basis[i]*weights[i]);
        let denominator=coeff.iter().sum::<f64>();
        if denominator<=0.||!denominator.is_finite(){return None;}
        let point=std::array::from_fn(|axis|(0..4).map(|i|ring[i][axis]*coeff[i]).sum::<f64>()/denominator);
        (length(delta(point,foot))<=PARAMETER_TOLERANCE).then_some([u,v])
    })
}

/// 📍️ Returns normalized parameters for the exact nearest position on the authored quadrilateral.
pub(super) fn nearest_uv(points:[Pnt3;4],weights:[f64;4],target:Pnt3)->Option<Uv>{
    if weights.iter().any(|&w|!w.is_finite()||w<=0.){return None;}
    let max=weights.into_iter().fold(0.,f64::max);
    let weights=weights.map(|w|w/max);
    if weights.contains(&0.){return None;}
    let (ring,target)=projected_chart(points,target)?;
    invert(ring,weights,closest_on_ring(ring,target))
}
