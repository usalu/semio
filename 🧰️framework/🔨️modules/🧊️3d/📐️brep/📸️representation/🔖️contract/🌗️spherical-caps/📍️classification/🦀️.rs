//! 📍️ Signed distance to the exact finite faces of a spherical Boolean boundary.

use super::*;

#[derive(Clone, Copy)]
struct Meridian {
    axial: f64,
    radial: f64,
}

impl Meridian {
    fn new(point: Pnt3, frame: Frame3) -> Self {
        let local = frame.to_local(point);
        Self { axial: local.z, radial: local.x.hypot(local.y) }
    }

    fn disk(self, [axial, radius]: [f64; 2]) -> f64 {
        (self.axial - axial).hypot((self.radial - radius).max(0.0))
    }

    fn sphere(self, radius: f64, [low, high]: [[f64; 2]; 2]) -> (f64, bool) {
        let length = self.radial.hypot(self.axial);
        if !length.is_finite() { return (f64::INFINITY, false); }
        if length == 0.0 { return (radius, true); }
        let projected = radius * (self.axial / length);
        let distance = if projected < low[0] {
            (self.axial - low[0]).hypot(self.radial - low[1])
        } else if projected > high[0] {
            (self.axial - high[0]).hypot(self.radial - high[1])
        } else {
            (length - radius).abs()
        };
        (distance, length <= radius)
    }
}

fn cap_limits(radius: f64, rim: [f64; 2], positive: bool) -> [[f64; 2]; 2] {
    if positive { [rim, [radius, 0.0]] } else { [[-radius, 0.0], rim] }
}

impl SphericalCaps {
    /// 🧲️ Measures the nearest retained face, signing the distance by material membership.
    pub fn signed_boundary_distance(self, point: Pnt3) -> f64 {
        let a = Meridian::new(point, self.frame);
        let [radius_a, radius_b] = self.radii;
        let lower = [self.offsets[0], self.rim];
        let (distance, inside) = match self.closure {
            Closure::Disk => {
                let (distance, within) = a.sphere(radius_a, cap_limits(radius_a, lower, true));
                (distance.min(a.disk(lower)), within && a.axial >= lower[0])
            }
            Closure::Band { offset, radius } => {
                let upper = [offset, radius];
                let (distance, within) = a.sphere(radius_a, [lower, upper]);
                (distance.min(a.disk(lower)).min(a.disk(upper)), within && (lower[0]..=upper[0]).contains(&a.axial))
            }
            Closure::Sphere => {
                let [positive_a, positive_b] = self.positive;
                let (a_distance, within_a) = a.sphere(radius_a, cap_limits(radius_a, lower, positive_a));
                let b = Meridian::new(point, self.frame(1));
                let (b_distance, within_b) = b.sphere(radius_b, cap_limits(radius_b, [self.offsets[1], self.rim], positive_b));
                let within = if self.inward { within_a && !within_b }
                    else if positive_a { within_a && within_b }
                    else { within_a || within_b };
                (a_distance.min(b_distance), within)
            }
        };
        if inside { -distance } else { distance }
    }
}
