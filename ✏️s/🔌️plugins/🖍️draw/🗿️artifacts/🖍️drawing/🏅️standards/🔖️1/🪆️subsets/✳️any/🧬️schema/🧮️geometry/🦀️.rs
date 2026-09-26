//! 📐️ Affine composition and exact cubic extrema, independent of rendering.
#[path = "✏️editing/🦀️.rs"]
pub mod editing;
#[path = "🎯️selection/🦀️.rs"]
pub mod selection;
pub fn multiply(a: [f64; 6], b: [f64; 6]) -> [f64; 6] {
    [a[0]*b[0]+a[2]*b[1], a[1]*b[0]+a[3]*b[1], a[0]*b[2]+a[2]*b[3], a[1]*b[2]+a[3]*b[3], a[0]*b[4]+a[2]*b[5]+a[4], a[1]*b[4]+a[3]*b[5]+a[5]]
}

pub fn cubic_bounds(points: [[f64; 2]; 4]) -> [f64; 4] {
    let mut result = [0.0; 4];
    for axis in 0..2 {
        let [p0, p1, p2, p3] = points.map(|point| point[axis]);
        let a = -p0 + 3.0*p1 - 3.0*p2 + p3;
        let b = 2.0*(p0 - 2.0*p1 + p2);
        let c = p1 - p0;
        let mut min = p0.min(p3);
        let mut max = p0.max(p3);
        let mut roots = [f64::NAN; 2];
        if a.abs() < 1e-12 {
            if b.abs() >= 1e-12 { roots[0] = -c / b; }
        } else {
            let discriminant = b*b - 4.0*a*c;
            if discriminant >= 0.0 {
                roots = [(-b + discriminant.sqrt())/(2.0*a), (-b - discriminant.sqrt())/(2.0*a)];
            }
        }
        for t in roots {
            if t > 0.0 && t < 1.0 {
                let u = 1.0-t;
                let value = u*u*u*p0 + 3.0*u*u*t*p1 + 3.0*u*t*t*p2 + t*t*t*p3;
                min = min.min(value);
                max = max.max(value);
            }
        }
        result[axis] = min;
        result[axis + 2] = max - min;
    }
    result
}

pub fn inverse(matrix: [f64; 6]) -> Option<[f64; 6]> {
    if !matrix.iter().all(|value| value.is_finite()) { return None; }
    let [a,b,c,d,e,f] = matrix;
    let determinant = a*d-b*c;
    if determinant == 0.0 || !determinant.is_finite() { return None; }
    let output = [d/determinant,-b/determinant,-c/determinant,a/determinant,(c*f-d*e)/determinant,(b*e-a*f)/determinant];
    output.iter().all(|value| value.is_finite()).then_some(output)
}

pub fn split_cubic(points: [[f64; 2]; 4], t: f64) -> Option<[[[f64; 2]; 4]; 2]> {
    if !(0.0..=1.0).contains(&t) || !points.iter().flatten().all(|value| value.is_finite()) { return None; }
    let mix = |a: [f64; 2], b: [f64; 2]| [a[0]*(1.0-t)+b[0]*t,a[1]*(1.0-t)+b[1]*t];
    let [a,b,c,d] = points;
    let ab = mix(a,b);
    let bc = mix(b,c);
    let cd = mix(c,d);
    let abc = mix(ab,bc);
    let bcd = mix(bc,cd);
    let center = mix(abc,bcd);
    let output = [[a,ab,abc,center],[center,bcd,cd,d]];
    output.iter().flatten().flatten().all(|value| value.is_finite()).then_some(output)
}

/// 🥚 Center-parameterized SVG ellipse, with radii corrected to reach both endpoints.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcGeometry {
    pub center: [f64; 2],
    pub radii: [f64; 2],
    pub rotation: f64,
    pub start: f64,
    pub sweep: f64,
}

impl ArcGeometry {
    pub fn point(&self, t: f64) -> [f64; 2] {
        let angle = self.start + self.sweep * t;
        let (sin, cos) = angle.sin_cos();
        let (sr, cr) = self.rotation.sin_cos();
        [self.center[0] + self.radii[0]*cos*cr - self.radii[1]*sin*sr, self.center[1] + self.radii[0]*cos*sr + self.radii[1]*sin*cr]
    }
}

pub fn arc_geometry(from: [f64; 2], radii: [f64; 2], rotation_degrees: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Option<ArcGeometry> {
    if !from.iter().chain(to.iter()).chain(radii.iter()).all(|value| value.is_finite()) || !rotation_degrees.is_finite() || radii.iter().any(|value| *value <= 0.0) || from == to { return None; }
    let rotation = rotation_degrees.to_radians();
    let (sin, cos) = rotation.sin_cos();
    let dx = (from[0] - to[0])/2.0;
    let dy = (from[1] - to[1])/2.0;
    let x = cos*dx + sin*dy;
    let y = -sin*dx + cos*dy;
    let factor = (x/radii[0]).hypot(y/radii[1]).max(1.0);
    let [rx, ry] = radii.map(|radius| radius*factor);
    let nx = x/rx; let ny = y/ry;
    let length = nx*nx + ny*ny;
    if length <= 0.0 || !length.is_finite() { return None; }
    let coefficient = ((1.0-length).max(0.0)/length).sqrt() * if large_arc == sweep { -1.0 } else { 1.0 };
    let cx = coefficient*rx*ny;
    let cy = -coefficient*ry*nx;
    let center = [cos*cx - sin*cy + (from[0]+to[0])/2.0, sin*cx + cos*cy + (from[1]+to[1])/2.0];
    let a = [(x-cx)/rx, (y-cy)/ry];
    let b = [(-x-cx)/rx, (-y-cy)/ry];
    let start = a[1].atan2(a[0]);
    let mut delta = (a[0]*b[1]-a[1]*b[0]).atan2(a[0]*b[0]+a[1]*b[1]);
    if sweep && delta < 0.0 { delta += std::f64::consts::TAU; }
    if !sweep && delta > 0.0 { delta -= std::f64::consts::TAU; }
    [center[0],center[1],rx,ry,rotation,start,delta].iter().all(|value| value.is_finite()).then_some(ArcGeometry { center, radii: [rx,ry], rotation, start, sweep: delta })
}

/// 🎯 Exact affine bounds for one path segment, usable by incremental selection queries.
pub fn segment_bounds(segment: &crate::PathSegment, from: [f64;2], contour_start: [f64;2], matrix: [f64;6]) -> [f64;4] {
    use crate::PathSegment;
    let map = |point: [f64;2]| [matrix[0]*point[0]+matrix[2]*point[1]+matrix[4],matrix[1]*point[0]+matrix[3]*point[1]+matrix[5]];
    let line = |a: [f64;2], b: [f64;2]| { let a=map(a); let b=map(b); [a[0].min(b[0]),a[1].min(b[1]),(a[0]-b[0]).abs(),(a[1]-b[1]).abs()] };
    match *segment {
        PathSegment::Move { to } => { let to=map(to); [to[0],to[1],0.0,0.0] }
        PathSegment::Line { to } => line(from,to),
        PathSegment::Close => line(from,contour_start),
        PathSegment::Cubic { ctrl1,ctrl2,to } => cubic_bounds([map(from),map(ctrl1),map(ctrl2),map(to)]),
        PathSegment::Quad { ctrl,to } => {
            let a=[from[0]+(ctrl[0]-from[0])*2.0/3.0,from[1]+(ctrl[1]-from[1])*2.0/3.0];
            let b=[to[0]+(ctrl[0]-to[0])*2.0/3.0,to[1]+(ctrl[1]-to[1])*2.0/3.0];
            cubic_bounds([map(from),map(a),map(b),map(to)])
        }
        PathSegment::Arc { rx,ry,rotation,large_arc,sweep,to } => {
            let Some(arc)=arc_geometry(from,[rx,ry],rotation,large_arc,sweep,to) else { return line(from,to) };
            let (sin,cos)=arc.rotation.sin_cos();
            let [rx,ry]=arc.radii;
            let u=[matrix[0]*rx*cos+matrix[2]*rx*sin,matrix[1]*rx*cos+matrix[3]*rx*sin];
            let v=[-matrix[0]*ry*sin+matrix[2]*ry*cos,-matrix[1]*ry*sin+matrix[3]*ry*cos];
            let mut result=line(from,to);
            for axis in 0..2 {
                let angle=v[axis].atan2(u[axis]);
                for candidate in [angle,angle+std::f64::consts::PI] {
                    let distance=if arc.sweep>=0.0 { (candidate-arc.start).rem_euclid(std::f64::consts::TAU) } else { (arc.start-candidate).rem_euclid(std::f64::consts::TAU) };
                    if distance<=arc.sweep.abs() {
                        let point=map(arc.point(distance/arc.sweep.abs()));
                        let max=(result[axis]+result[axis+2]).max(point[axis]);
                        result[axis]=result[axis].min(point[axis]);
                        result[axis+2]=max-result[axis];
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/📐️bounds/🦀️.rs"]
mod tests;

#[path = "↔️translation/🦀️.rs"]
pub mod translation;
