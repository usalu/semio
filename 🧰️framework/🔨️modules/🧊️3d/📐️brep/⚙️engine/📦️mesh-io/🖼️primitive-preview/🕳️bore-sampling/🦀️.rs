//! 🕳️ A shared rim parameter grid for bore cap, wall, and edge sampling.

use super::{BorePreviewSource, MAX_SEGMENTS, NativeVec3, Pnt3, TAU, ValueError, Vertex, invalid_preview, segment_count};

const SECTIONS: usize = 5;

#[derive(Clone, Copy)]
pub(super) struct BoreSampling {
    source: BorePreviewSource,
    maximum: [f64; 3],
    corners: [[f64; 2]; 4],
    angles: [f64; SECTIONS + 1],
    offsets: [usize; SECTIONS + 1],
}

semio_framework_value::artifact_retire_leaf!(BoreSampling);

impl BoreSampling {
    pub(super) fn new(source: BorePreviewSource, tolerance: f64) -> Result<Self, ValueError> {
        let [xmin, ymin, zmin] = source.origin;
        let [x, y] = source.center;
        let [xmax, ymax, zmax] = std::array::from_fn(|axis| source.origin[axis] + source.dimensions[axis]);
        let reasonable = |coordinate: f64| coordinate.is_finite() && coordinate.abs() < f64::from(f32::MAX) / 4.0;
        if !tolerance.is_finite() || tolerance <= 0.0
            || !source.radius.is_finite() || source.radius <= 0.0
            || ![xmin, ymin, zmin, xmax, ymax, zmax, x, y].into_iter().all(reasonable)
            || zmin as f32 >= zmax as f32
            || !source.bore_span.into_iter().all(reasonable)
            || source.bore_span[0]<zmin||source.bore_span[1]>zmax
            || source.bore_span[0] as f32>=source.bore_span[1] as f32
        {
            return Err(invalid_preview());
        }
        let radius = source.radius;
        for (low, center, high) in [(xmin, x, xmax), (ymin, y, ymax)] {
            let rim_min = (center - radius) as f32;
            let rim_max = (center + radius) as f32;
            if !((low as f32) < rim_min && rim_min < center as f32 && (center as f32) < rim_max && rim_max < high as f32) {
                return Err(invalid_preview());
            }
        }

        let corners = [[xmax, ymax], [xmin, ymax], [xmin, ymin], [xmax, ymin]];
        let mut angles = [0.0; SECTIONS + 1];
        for (index, [cx, cy]) in corners.into_iter().enumerate() {
            angles[index + 1] = (cy - y).atan2(cx - x).rem_euclid(TAU);
        }
        angles[SECTIONS] = TAU;
        if angles.windows(2).any(|pair| pair[0] >= pair[1]) { return Err(invalid_preview()); }

        let target = segment_count(radius, tolerance, TAU).min(MAX_SEGMENTS - SECTIONS);
        let mut offsets = [0; SECTIONS + 1];
        for section in 0..SECTIONS {
            let sweep = angles[section + 1] - angles[section];
            offsets[section + 1] = offsets[section] + ((target as f64 * sweep / TAU).ceil() as usize).max(1);
        }
        let sampling = Self { source, maximum: [xmax, ymax, zmax], corners, angles, offsets };
        let coincides = |a: Pnt3, b: Pnt3| a.x as f32 == b.x as f32 && a.y as f32 == b.y as f32;
        if (0..sampling.segments()).any(|index| {
            coincides(sampling.rim_point(false, index), sampling.rim_point(false, index + 1))
                || coincides(sampling.stock_point(false, index), sampling.stock_point(false, index + 1))
        }) {
            return Err(invalid_preview());
        }
        Ok(sampling)
    }

    pub(super) fn source(&self) -> BorePreviewSource { self.source }
    pub(super) fn segments(&self) -> usize { self.offsets[SECTIONS] }

    fn section(&self, index: usize) -> usize {
        self.offsets.partition_point(|offset| *offset <= index) - 1
    }

    /// 🧭️ Uses the exact start sample again when the final segment reaches the positive U seam.
    pub(super) fn angle(&self, index: usize) -> f64 {
        debug_assert!(index <= self.segments());
        if index >= self.segments() { return 0.0; }
        let section = self.section(index);
        let slot = index - self.offsets[section];
        let count = self.offsets[section + 1] - self.offsets[section];
        let low = self.angles[section];
        low + (self.angles[section + 1] - low) * slot as f64 / count as f64
    }

    fn z(&self, top: bool) -> f64 {
        if top { self.maximum[2] } else { self.source.origin[2] }
    }

    fn radial(&self, index: usize) -> NativeVec3 {
        let (y, x) = self.angle(index).sin_cos();
        NativeVec3::new(x, y, 0.0)
    }

    pub(super) fn rim_point(&self, top: bool, index: usize) -> Pnt3 {
        let [x, y] = self.source.center;
        Pnt3::new(x,y,self.source.bore_span[usize::from(top)])+self.radial(index)*self.source.radius
    }

    fn stock_point(&self, top: bool, index: usize) -> Pnt3 {
        let z = self.z(top);
        let [x, y] = self.source.center;
        if index == 0 || index == self.segments() {
            return Pnt3::new(self.maximum[0], y, z);
        }
        let section = self.section(index);
        if index == self.offsets[section] {
            let [cx, cy] = self.corners[section - 1];
            return Pnt3::new(cx, cy, z);
        }
        let radial = self.radial(index);
        let [xmin, ymin, _] = self.source.origin;
        let [xmax, ymax, _] = self.maximum;
        let vertical = |edge: f64| Pnt3::new(edge, (y + radial.y * (edge - x) / radial.x).clamp(ymin, ymax), z);
        let horizontal = |edge: f64| Pnt3::new((x + radial.x * (edge - y) / radial.y).clamp(xmin, xmax), edge, z);
        match section {
            0 | 4 => vertical(xmax),
            1 => horizontal(ymax),
            2 => vertical(xmin),
            _ => horizontal(ymin),
        }
    }

    pub(super) fn cap_triangle(&self, top: bool, slot: usize) -> [Vertex; 3] {
        let sector = slot / 2;
        let a = self.rim_point(top, sector);
        let outer = self.stock_point(top, sector + 1);
        let mut points = if slot % 2 == 0 {
            [a, self.stock_point(top, sector), outer]
        } else {
            [a, outer, self.rim_point(top, sector + 1)]
        };
        if !top { points.swap(1, 2); }
        let normal = if top { NativeVec3::Z } else { -NativeVec3::Z };
        points.map(|point| Vertex { point, normal })
    }

    /// 🛟️ Samples the material-facing disk at one interior end of the bore.
    pub(super) fn floor_triangle(&self,top:bool,slot:usize)->[Vertex;3]{
        let [x,y]=self.source.center;
        let center=Pnt3::new(x,y,self.source.bore_span[usize::from(top)]);
        let (a,b)=(self.rim_point(top,slot),self.rim_point(top,slot+1));
        let normal=if top{-NativeVec3::Z}else{NativeVec3::Z};
        (if top{[center,b,a]}else{[center,a,b]}).map(|point|Vertex{point,normal})
    }

    pub(super) fn wall_triangle(&self, slot: usize) -> [Vertex; 3] {
        let sector = slot / 2;
        let corners = if slot % 2 == 0 {
            [(false, sector), (true, sector + 1), (false, sector + 1)]
        } else {
            [(false, sector), (true, sector), (true, sector + 1)]
        };
        corners.map(|(top, index)| Vertex { point: self.rim_point(top, index), normal: -self.radial(index) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bore_rim_is_shared_and_every_stock_corner_is_sampled_exactly() {
        let source = BorePreviewSource {
            origin: [1.0, 2.0, 3.0], dimensions: [4.0, 3.0, 2.0], center: [2.25, 3.75], radius: 0.6,
            bore_span:[3.0,5.0],first_label: 19, volume: 24.0 - 0.72 * std::f64::consts::PI,
        };
        let sampling = BoreSampling::new(source, 0.001).unwrap();
        assert!(sampling.segments() <= MAX_SEGMENTS);
        for top in [false, true] {
            assert_eq!(sampling.rim_point(top, 0), sampling.rim_point(top, sampling.segments()));
            let sampled: Vec<_> = (0..=sampling.segments()).map(|index| sampling.stock_point(top, index)).collect();
            for [x, y] in sampling.corners {
                assert!(sampled.contains(&Pnt3::new(x, y, sampling.z(top))));
            }
        }
        for sector in 0..sampling.segments() {
            let slots = [2 * sector, 2 * sector + 1];
            for top in [false, true] {
                let cap_points = slots.map(|slot| sampling.cap_triangle(top, slot).map(|vertex| vertex.point));
                let wall_points = slots.map(|slot| sampling.wall_triangle(slot).map(|vertex| vertex.point));
                for end in [sector, sector + 1] {
                    let rim = sampling.rim_point(top, end);
                    assert!(cap_points.iter().flatten().any(|point| *point == rim));
                    assert!(wall_points.iter().flatten().any(|point| *point == rim));
                }
            }
            for slot in slots {
                for [a, b, c] in [sampling.cap_triangle(false, slot), sampling.cap_triangle(true, slot), sampling.wall_triangle(slot)] {
                    assert!((b.point - a.point).cross(c.point - a.point).dot(a.normal + b.normal + c.normal) > 0.0);
                }
            }
        }
    }
}
