//! 📐️ Exact prism boundary for drafting one planar side of a native box.

use semio_framework_3d_brep_construction::{primitives::SolidPrimitive, prism_boundary::{MAX_PRISM_CORNERS,PrismBoundary}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3};

/// 📐️ Rotates one box support about its intersection with the pull-normal neutral plane.
pub(super) fn box_face_draft(primitive: SolidPrimitive, face: usize, pull: [f64;3], neutral: [f64;3], angle: f64) -> Option<PrismBoundary> {
    let SolidPrimitive::Box { origin,width,depth,height } = primitive else { return None; };
    if face >= primitive.ring_sizes().len() || !pull.into_iter().chain(neutral).all(f64::is_finite)
        || !(1e-15 < angle.abs() && angle.abs() < std::f64::consts::FRAC_PI_2)
    { return None; }
    let Surface::Plane { frame } = primitive.surface(face) else { return None; };
    let normal = frame.z.to_array();
    let side = normal.iter().position(|&x| x != 0.)?;
    let direction = pull.iter().position(|&value| value != 0.)?;
    if side == direction || pull.iter().enumerate().any(|(i,&value)| i != direction && value != 0.) { return None; }
    let axis = 3-side-direction;
    let (u,v) = ((axis+1)%3,(axis+2)%3);
    let lo = origin.to_array();
    let size = [width,depth,height];
    let hi: [f64;3] = std::array::from_fn(|i| lo[i]+size[i]);
    let outward = normal[side];
    let wall = if outward > 0. { hi[side] } else { lo[side] };
    let opposite = if outward > 0. { lo[side] } else { hi[side] };
    let signed_slope = -outward*pull[direction].signum()*angle.tan();
    let drafted = |h:f64| wall+signed_slope*(h-neutral[direction]);
    if [lo[direction],hi[direction]].into_iter().any(|h| {
        let x = drafted(h);
        !x.is_finite() || (x-opposite)*outward <= Tol::DEFAULT.value()
    }) { return None; }
    let mut points = [origin;MAX_PRISM_CORNERS];
    for (index,(a,b)) in [(lo[u],lo[v]),(hi[u],lo[v]),(hi[u],hi[v]),(lo[u],hi[v])].into_iter().enumerate() {
        let mut point=lo;
        point[u]=a;point[v]=b;
        if point[side] == wall { point[side]=drafted(point[direction]); }
        points[index]=Pnt3::from_array(point);
    }
    let mut normal=[0.;3];normal[axis]=1.;
    let normal=Vec3::from_array(normal);
    PrismBoundary::from_planar_ring(points,4,normal*size[axis],normal)
}
