//! 📐️ Exact supporting conics of finite analytic solid sections.

use semio_framework_3d_brep_construction::primitives::SectionPolygon;
use super::{Conic,ConicBoundary,SectionArc,SolidPrimitive,Tol};
use semio_framework_3d_brep_vector::{Pnt2,Pnt3,Vec3,matrix::Frame3};
use semio_framework_value::{ValueError,ValueRefusalKind};

#[path="📈️open-conic/🦀️.rs"]
mod open_conic;

/// 🧭️ Resolves exact analytic planar sections of certified primitive solids.
pub fn conic_section(source:&SolidPrimitive,origin:Pnt3,normal:Vec3)->Result<(Frame3,Conic,ConicBoundary),ValueError>{
    let plane=section_plane(origin,normal)?;
    let (section,conic,boundary)=match *source{
        SolidPrimitive::Sphere{frame,radius}=>{
            let distance=(frame.origin-origin).dot(plane.z);
            let clearance=radius-distance.abs();
            if clearance<=Tol::DEFAULT.value(){return Err(section_miss());}
            let radius=(clearance*(radius+distance.abs())).sqrt();
            (Frame3{origin:frame.origin-plane.z*distance,..plane},Conic::Circle(radius),ConicBoundary::FULL)
        }
        SolidPrimitive::Cylinder{frame,radius,height}=>return axial_conic(frame,[radius;2],height,plane),
        SolidPrimitive::Cone{frame,radius,height}=>return axial_conic(frame,[radius,0.],height,plane),
        _=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Section requires a certified planar or analytic solid")),
    };
    certify_section(section,conic,boundary)
}

/// 🪣️ Resolves a circle or ellipse on the finite radius profile of a certified frustum.
pub fn frustum_section(frame:Frame3,radii:[f64;2],height:f64,origin:Pnt3,normal:Vec3)->Result<(Frame3,Conic,ConicBoundary),ValueError>{
    axial_conic(frame,radii,height,section_plane(origin,normal)?)
}

fn invalid_section()->ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue,"section requires a finite plane with a nonzero normal")
}

fn section_miss()->ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue,"section has no nondegenerate material contour")
}

fn section_plane(origin:Pnt3,normal:Vec3)->Result<Frame3,ValueError>{
    if !origin.to_array().into_iter().chain(normal.to_array()).all(f64::is_finite){return Err(invalid_section());}
    Frame3::from_normal(origin,normal).filter(|frame|frame.z.norm()>0.0&&frame.z.norm().is_finite()).ok_or_else(invalid_section)
}

fn certify_section(frame:Frame3,conic:Conic,boundary:ConicBoundary)->Result<(Frame3,Conic,ConicBoundary),ValueError>{
    let radius=conic.radii().ok_or_else(invalid_section)?.into_iter().fold(0.,f64::max);
    if !radius.is_finite()||radius<=Tol::DEFAULT.value()||!frame.origin.to_array().into_iter().all(|x|(x.abs()+radius).is_finite()){
        return Err(invalid_section());
    }
    Ok((frame,conic,boundary))
}

fn axial_conic(frame:Frame3,radii:[f64;2],height:f64,plane:Frame3)->Result<(Frame3,Conic,ConicBoundary),ValueError>{
    let cosine=frame.z.dot(plane.z).clamp(-1.,1.);
    let cross=plane.z.cross(frame.z);
    let sine=cross.norm();
    let slope=(radii[1]-radii[0])/height;
    let projected_slope=slope*sine;
    let coefficient=cosine*cosine-projected_slope*projected_slope;
    if slope!=0.0&&(coefficient==0.0||coefficient < -1e-12){
        return open_conic::section(frame,radii[0],height,slope,plane,coefficient);
    }
    let analytic_limit=if slope==0.0{1e-24}else{1e-12};
    if !(coefficient>analytic_limit){
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"analytic Section inclination is numerically unresolved"));
    }
    let t=(plane.origin-frame.origin).dot(plane.z)/cosine;
    let radius=slope.mul_add(t,radii[0]);
    let tolerance=Tol::DEFAULT.value();
    if !(radius>tolerance&&radius.is_finite()){return Err(section_miss());}
    let center=frame.origin+frame.z*t;
    if sine<=f64::EPSILON{
        if !(t>tolerance&&t<height-tolerance){return Err(section_miss());}
        return certify_section(Frame3{origin:center,..plane},Conic::Circle(radius),ConicBoundary::FULL);
    }
    let y=cross.normalized().ok_or_else(invalid_section)?;
    let x=y.cross(plane.z);
    let shift=radius*projected_slope/coefficient;
    let major=radius*cosine.abs()/coefficient;
    let minor=radius*cosine.abs()/coefficient.sqrt();
    let mid=t+sine*shift;
    let extent=sine*major;
    if !mid.is_finite()||!extent.is_finite(){return Err(invalid_section());}
    let plane=Frame3{origin:center+x*shift,x,y,z:plane.z};
    clip_ellipse(plane,[major,minor],mid,extent,height)
}

fn clip_ellipse(mut frame:Frame3,radii:[f64;2],mid:f64,extent:f64,height:f64)->Result<(Frame3,Conic,ConicBoundary),ValueError>{
    let [major,minor]=radii;
    let tolerance=Tol::DEFAULT.value();
    if mid+extent<=tolerance||mid-extent>=height-tolerance{return Err(section_miss());}
    let lower=mid-extent<0.0;
    let boundary=match (lower,mid+extent>height){
        (true,true)=>{
            let first=((height-mid)/extent).clamp(-1.0,1.0).acos();
            let last=(-mid/extent).clamp(-1.0,1.0).acos();
            if first.sin()*minor<=tolerance||last.sin()*minor<=tolerance||last<=first{return Err(section_miss());}
            ConicBoundary::clipped((first,last),Some((std::f64::consts::TAU-last,std::f64::consts::TAU-first)))
        }
        (true,false)|(false,true)=>{
            let limit=if lower{
                frame.x=-frame.x;
                frame.y=-frame.y;
                mid/extent
            }else{(height-mid)/extent};
            let first=limit.clamp(-1.0,1.0).acos();
            if first.sin()*minor<=tolerance{return Err(section_miss());}
            ConicBoundary::clipped((first,std::f64::consts::TAU-first),None)
        }
        (false,false)=>ConicBoundary::FULL,
    };
    certify_section(frame,Conic::Ellipse(major,minor),boundary)
}

/// 🔺️ Resolves a finite generator-and-chord contour when the plane contains the native cone apex.
pub fn apex_section(frame:Frame3,radii:[f64;2],height:f64,native_apex:Option<Pnt3>,origin:Pnt3,normal:Vec3)->Result<Option<SectionPolygon>,ValueError>{
    let plane=section_plane(origin,normal)?;
    let slope=(radii[1]-radii[0])/height;
    if slope==0.0{return Ok(None);}
    let level=if radii[1]==0.0{height}else{-radii[0]/slope};
    let contains=|point:Pnt3| (point-origin).dot(normal)==0.0;
    let ideal=frame.origin+frame.z*level;
    let Some(apex)=Some(ideal).filter(|&point|contains(point)).or_else(||native_apex.filter(|&point|contains(point)))else{return Ok(None);};
    let cosine=frame.z.dot(plane.z);
    let tangent=frame.z.cross(plane.z);
    let sine=tangent.norm();
    if !(cosine.abs()<(slope*sine).abs()){return Err(section_miss());}
    let side=tangent.normalized().ok_or_else(invalid_section)?;
    let radial=(plane.z-frame.z*cosine)*sine.recip();
    let chord=|station:f64,radius:f64|->Result<[Pnt3;2],ValueError>{
        if radius==0.0{return Ok([apex;2]);}
        let center=frame.origin+frame.z*station;
        let distance=(center-plane.origin).dot(plane.z)/sine;
        let width=((radius-distance.abs())*(radius+distance.abs())).sqrt();
        if !(width>Tol::DEFAULT.value()&&width.is_finite()){return Err(section_miss());}
        let mid=center-radial*distance;
        let points=[mid-side*width,mid+side*width];
        if points.iter().flat_map(|point|point.to_array()).all(f64::is_finite){Ok(points)}else{Err(invalid_section())}
    };
    let [a,b]=chord(0.,radii[0])?;
    let [d,c]=chord(height,radii[1])?;
    Ok(Some(if radii[1]==0.0{SectionPolygon::ordered_triangle([a,b,c])}
        else if radii[0]==0.0{SectionPolygon::ordered_triangle([a,c,d])}
        else{SectionPolygon::ordered_quadrilateral([a,b,c,d])}))
}

/// 📏️ Resolves a signed rectangular cut when the cylinder axis lies exactly in the plane direction.
pub fn axial_cylinder_section(source: &SolidPrimitive, origin: Pnt3, normal: Vec3) -> Result<Option<[Pnt3; 4]>, ValueError> {
    let SolidPrimitive::Cylinder { frame, radius, height } = *source else { return Ok(None); };
    if normal.dot(frame.z) != 0.0 { return Ok(None); }
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "section requires a finite plane with a nonzero normal");
    if !origin.to_array().into_iter().chain(normal.to_array()).all(f64::is_finite) { return Err(invalid()); }
    let n = normal.normalized().filter(|v|v.norm()>0.0&&v.norm().is_finite()).ok_or_else(invalid)?;
    let distance = (origin - frame.origin).dot(n);
    let clearance = radius - distance.abs();
    if !clearance.is_finite() { return Err(invalid()); }
    if clearance <= Tol::DEFAULT.value() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "section plane misses or is tangent to the cylinder"));
    }
    let half_width = clearance.sqrt() * (radius + distance.abs()).sqrt();
    let side = frame.z.cross(n) * half_width;
    let bottom = frame.origin + n * distance;
    let top = bottom + frame.z * height;
    let points = [bottom - side, bottom + side, top + side, top - side];
    if !points.iter().flat_map(|point| point.to_array()).all(f64::is_finite) { return Err(invalid()); }
    Ok(Some(points))
}
