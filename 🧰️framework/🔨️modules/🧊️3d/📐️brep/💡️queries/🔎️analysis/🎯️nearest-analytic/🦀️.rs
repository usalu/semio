//! 🎯️ Deterministic nearest parameters for original lines, conics, and analytic surfaces.

use super::planar_dual_coordinates;
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

fn invalid()->ValueError{
    ValueError::literal(ValueRefusalKind::InvalidValue,"nearest analytic parameters require finite nondegenerate geometry")
}

fn unsupported()->ValueError{
    ValueError::literal(ValueRefusalKind::UnsupportedOwner,"spline projection requires original distance subdivision")
}

fn angle(x:f64,y:f64)->f64{
    if x==0.0&&y==0.0{0.0}else{y.atan2(x).rem_euclid(TAU)}
}

fn check(point:Pnt3)->Result<(),ValueError>{
    if [point.x,point.y,point.z].into_iter().all(f64::is_finite){Ok(())}else{Err(invalid())}
}

fn result<const N:usize>(parameters:[f64;N],projected:Pnt3,target:Pnt3)->Result<([f64;N],Pnt3,f64),ValueError>{
    let d=projected-target;
    let distance=d.x.hypot(d.y).hypot(d.z);
    check(projected)?;
    if !distance.is_finite()||parameters.into_iter().any(|value|!value.is_finite()){return Err(invalid());}
    Ok((parameters,projected,distance))
}

fn line_parameter(origin:Pnt3,direction:Vec3,target:Pnt3,range:Option<(f64,f64)>)->Result<(f64,Pnt3),ValueError>{
    let speed=direction.x.hypot(direction.y).hypot(direction.z);
    if speed==0.0||!speed.is_finite(){return Err(invalid());}
    let unit=Vec3::new(direction.x/speed,direction.y/speed,direction.z/speed);
    let along=(target-origin).dot(unit);
    let parameter=along/speed;
    let parameter=if let Some((a,b))=range{
        if !a.is_finite()||!b.is_finite(){return Err(invalid());}
        parameter.clamp(a.min(b),a.max(b))
    }else{parameter};
    let projected=origin+direction*parameter;
    check(projected)?;
    if !parameter.is_finite(){return Err(invalid());}
    Ok((parameter,projected))
}

fn plane_uv(frame:&Frame3,target:Pnt3)->Result<[f64;2],ValueError>{
    planar_dual_coordinates(*frame,target).map_err(|_|invalid())
}

/// 🧵️ Projects onto the closed trim of a supported analytic edge.
pub fn edge_nearest(curve:&Curve3,target:Pnt3,range:(f64,f64))->Result<Pnt3,ValueError>{
    match curve{
        Curve3::Line{origin,dir}=>Ok(line_parameter(*origin,*dir,target,Some(range))?.1),
        Curve3::Circle{..}=>{
            let (lo,hi)=(range.0.min(range.1),range.0.max(range.1));
            if !range.0.is_finite()||!range.1.is_finite(){return Err(invalid());}
            let ([angle],_,_)=curve_nearest(curve,target)?;
            let on_trim=lo+(angle-lo).rem_euclid(TAU);
            if on_trim<=hi{return Ok(curve.eval(on_trim));}
            let a=curve.eval(lo);
            let b=curve.eval(hi);
            let distance=|point:Pnt3|{let delta=point-target;delta.x.hypot(delta.y).hypot(delta.z)};
            Ok(if distance(a)<=distance(b){a}else{b})
        },
        _=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"closest point on this trim requires original distance subdivision")),
    }
}

/// 💿️ Projects onto a filled circle or ellipse.
pub fn conic_disk_nearest(curve:&Curve3,target:Pnt3)->Result<Pnt3,ValueError>{
    let (frame,a,b)=match curve{
        Curve3::Circle{frame,radius}=>(frame,*radius,*radius),
        Curve3::Ellipse{frame,major_radius,minor_radius}=>(frame,*major_radius,*minor_radius),
        _=>return Err(unsupported()),
    };
    if a<=0.0||b<=0.0||!a.is_finite()||!b.is_finite(){return Err(invalid());}
    let [u,v]=plane_uv(frame,target)?;
    if (u/a).hypot(v/b)<=1.0{return Ok(frame.origin+frame.x*u+frame.y*v);}
    Ok(curve_nearest(curve,target)?.1)
}

/// 🧊️ Projects onto the boundary of an analytic solid primitive.
pub fn primitive_nearest(primitive:SolidPrimitive,target:Pnt3)->Result<Pnt3,ValueError>{
    check(target)?;
    match primitive{
        SolidPrimitive::Box{origin,width,depth,height}=>{
            let delta=target-origin;
            let source=[delta.x,delta.y,delta.z];
            let extents=[width,depth,height];
            let mut point:[f64;3]=std::array::from_fn(|i|source[i].clamp(0.0,extents[i]));
            if point==source{
                let mut nearest=f64::INFINITY;
                let mut face=(0,0.0);
                for i in 0..3{
                    let lo=source[i];
                    let hi=extents[i]-lo;
                    let (distance,bound)=if lo<=hi{(lo,0.0)}else{(hi,extents[i])};
                    if distance<nearest{nearest=distance;face=(i,bound);}
                }
                point[face.0]=face.1;
            }
            Ok(origin+Vec3::new(point[0],point[1],point[2]))
        }
        SolidPrimitive::Sphere{frame,radius}=>Ok(surface_nearest(&Surface::Sphere{frame,radius},target)?.1),
        SolidPrimitive::Torus{frame,major,minor}=>Ok(surface_nearest(&Surface::Torus{
            frame,major_radius:major,minor_radius:minor
        },target)?.1),
        SolidPrimitive::Cylinder{frame,radius,height}|SolidPrimitive::Cone{frame,radius,height}=>{
            let local=frame.to_local(target);
            check(local)?;
            let r=local.x.hypot(local.y);
            let z=local.z;
            let cone=matches!(primitive,SolidPrimitive::Cone{..});
            let side=if cone{
                let origin=Pnt3::new(radius,0.0,0.0);
                let direction=Vec3::new(-radius,0.0,height);
                let point=line_parameter(origin,direction,Pnt3::new(r,0.0,z),Some((0.0,1.0)))?.1;
                [point.x,point.z]
            }else{[radius,z.clamp(0.0,height)]};
            let cap=[r.min(radius),if !cone&&(z-height).abs()<z.abs(){height}else{0.0}];
            let distance=|[radial,axial]:[f64;2]|(r-radial).hypot(z-axial);
            let [radial,axial]=if distance(side)<=distance(cap){side}else{cap};
            let (sine,cosine)=angle(local.x,local.y).sin_cos();
            Ok(frame.origin+frame.x*(radial*cosine)+frame.y*(radial*sine)+frame.z*axial)
        }
    }
}

fn ellipse_parameter(x:f64,y:f64,a:f64,b:f64)->Result<f64,ValueError>{
    if ![x,y,a,b].into_iter().all(f64::is_finite)||a<=0.0||b<=0.0{return Err(invalid());}
    if a==b{return Ok(angle(x,y));}
    let swapped=a<b;
    let (x,y,major,minor)=if swapped{(y,x,b,a)}else{(x,y,a,b)};
    let q=x.abs()/major;
    let r=y.abs()/major;
    let ratio=minor/major;
    let difference=(1.0-ratio)*(1.0+ratio);
    if ratio<=0.0||!q.is_finite()||!r.is_finite(){return Err(invalid());}
    let (u,v)=if q==0.0{(0.0,1.0)}
    else if r==0.0{
        let u=(q/difference).min(1.0);
        (u,(1.0-u*u).max(0.0).sqrt())
    }else{
        let numerator=ratio*r;
        let mut low=0.0;
        let mut high=q.hypot(numerator)+ratio*ratio;
        if !high.is_finite(){return Err(invalid());}
        for _ in 0..80{
            let mid=0.5*low+0.5*high;
            if mid==low||mid==high{break;}
            if (q/(difference+mid)).hypot(numerator/mid)>1.0{low=mid;}
            else{high=mid;}
        }
        (q/(difference+high),numerator/high)
    };
    let (u,v)=(u.copysign(x),v.copysign(y));
    Ok(if swapped{angle(v,u)}else{angle(u,v)})
}

/// 📍️ Returns the nearest parameter, point, and distance on a supported analytic curve.
pub fn curve_nearest(curve:&Curve3,target:Pnt3)->Result<([f64;1],Pnt3,f64),ValueError>{
    check(target)?;
    let parameter=match curve{
        Curve3::Line{origin,dir}=>{
            let (parameter,point)=line_parameter(*origin,*dir,target,None)?;
            return result([parameter],point,target);
        }
        Curve3::Circle{frame,radius} if radius.is_finite()&&*radius>0.0=>{
            let p=frame.to_local(target);
            angle(p.x,p.y)
        }
        Curve3::Ellipse{frame,major_radius,minor_radius}=>{
            let p=frame.to_local(target);
            ellipse_parameter(p.x,p.y,*major_radius,*minor_radius)?
        }
        Curve3::Nurbs{..}=>return Err(unsupported()),
        _=>return Err(invalid()),
    };
    result([parameter],curve.eval(parameter),target)
}

/// 📍️ Returns the nearest parameters, point, and distance on a supported analytic surface.
pub fn surface_nearest(surface:&Surface,target:Pnt3)->Result<([f64;2],Pnt3,f64),ValueError>{
    check(target)?;
    if let Surface::Plane{frame}=surface{
        let [u,v]=plane_uv(frame,target)?;
        return result([u,v],surface.eval(u,v),target);
    }
    let frame=match surface{
        Surface::Plane{frame}|Surface::Cylinder{frame,..}|Surface::Cone{frame,..}|
        Surface::Sphere{frame,..}|Surface::Torus{frame,..}=>frame,
        Surface::Nurbs{..}=>return Err(unsupported()),
    };
    let local=frame.to_local(target);
    check(local)?;
    let radial=local.x.hypot(local.y);
    let azimuth=angle(local.x,local.y);
    let [u,v]=match surface{
        Surface::Cylinder{radius,..} if radius.is_finite()&&*radius>0.0=>[azimuth,local.z],
        Surface::Sphere{radius,..} if radius.is_finite()&&*radius>0.0=>[azimuth,local.z.atan2(radial)],
        Surface::Torus{major_radius,minor_radius,..} if major_radius.is_finite()&&minor_radius.is_finite()&&major_radius>minor_radius&&*minor_radius>0.0=>
            [azimuth,angle(radial-major_radius,local.z)],
        Surface::Cone{half_angle,..} if *half_angle>0.0&&*half_angle<FRAC_PI_2=>{
            let (sine,cosine)=half_angle.sin_cos();
            let z=(sine*radial+cosine*local.z.abs())*cosine;
            if local.z<0.0{[(azimuth+PI).rem_euclid(TAU),-z]}else{[azimuth,z]}
        }
        _=>return Err(invalid()),
    };
    result([u,v],surface.eval(u,v),target)
}
