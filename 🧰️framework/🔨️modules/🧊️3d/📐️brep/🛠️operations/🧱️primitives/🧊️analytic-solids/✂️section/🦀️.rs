//! ✂️ Exact bounded plane cuts through the edges of a certified box.

use super::*;
use std::mem::take;

const MAX_CORNERS: usize = 6;

/// 📐️ Holds one unambiguous convex intersection in its requested plane orientation.
pub struct BoxSectionPolygon {
    points: [Pnt3; MAX_CORNERS],
    order: [u8; MAX_CORNERS],
    count: usize,
}

impl BoxSectionPolygon {
    pub fn count(&self) -> usize { self.count }
    pub fn point(&self, index: usize) -> Option<[f64; 3]> {
        (index < self.count).then(|| self.points[usize::from(self.order[index])].to_array())
    }
}

/// 🎟️ Clips one original box edge per turn without heap scratch.
pub struct BoxSectionDraft {
    solid: SolidPrimitive,
    plane: Frame3,
    next_edge: usize,
    points: [Pnt3; MAX_CORNERS],
    count: usize,
}

impl BoxSectionDraft {
    /// 🔻 Admits one finite plane and a certified rectangular stock shape.
    pub fn new(solid: SolidPrimitive, origin: [f64; 3], normal: [f64; 3]) -> Result<Self, &'static str> {
        if !matches!(solid, SolidPrimitive::Box { .. }) {
            return Err("original Section requires a certified box");
        }
        if origin.into_iter().chain(normal).any(|coordinate| !coordinate.is_finite()) {
            return Err("box section plane must be finite");
        }
        let plane = Frame3::from_normal(Pnt3::from_array(origin), Vec3::from_array(normal))
            .ok_or("box section plane requires a nonzero normal")?;
        if !plane.z.norm().is_finite() || plane.z.norm() == 0.0 {
            return Err("box section normal lies outside the representable range");
        }
        Ok(Self { solid, plane, next_edge: 0, points: [Pnt3::default(); MAX_CORNERS], count: 0 })
    }

    pub fn has_edge(&self) -> bool { self.next_edge < BoxGeometry::EDGES.len() }

    /// ✂️ Intersects one line segment with both endpoints outside the ambiguity band.
    pub fn advance_edge(&mut self) -> Result<(), &'static str> {
        let &[first, last] = BoxGeometry::EDGES.get(self.next_edge).ok_or("box section edge cursor is complete")?;
        let [first, last] = [self.solid.vertex(first), self.solid.vertex(last)];
        let signed = |point: Pnt3| (point - self.plane.origin).dot(self.plane.z);
        let [a, b] = [signed(first), signed(last)];
        if !a.is_finite() || !b.is_finite() { return Err("box section exceeds the finite coordinate range"); }
        if a.abs() <= Tol::DEFAULT.value() || b.abs() <= Tol::DEFAULT.value() {
            return Err("original Section requires a plane clear of all box vertices");
        }
        if a.is_sign_negative() != b.is_sign_negative() {
            let scale = a.abs().max(b.abs());
            let [a, b] = [a.abs() / scale, b.abs() / scale];
            let point = first.lerp(last, a / (a + b));
            if !point.to_array().into_iter().all(f64::is_finite) {
                return Err("box section crossing exceeds the finite coordinate range");
            }
            *self.points.get_mut(self.count).ok_or("box section exceeded six distinct crossings")? = point;
            self.count += 1;
        }
        self.next_edge += 1;
        Ok(())
    }

    /// 🧭️ Orders the strictly convex section boundary by the requested plane normal.
    pub fn finish(&mut self) -> Result<BoxSectionPolygon, &'static str> {
        if self.has_edge() || self.count < 3 {
            return Err("section produced no closed box cross-section");
        }
        let reference = self.points[0];
        let mut projected = [[0.0; 2]; MAX_CORNERS];
        let mut center = [0.0; 2];
        let mut order = [0u8; MAX_CORNERS];
        for (index, &point) in self.points[..self.count].iter().enumerate() {
            let delta = point - reference;
            let point = [delta.dot(self.plane.x), delta.dot(self.plane.y)];
            if !point.into_iter().all(f64::is_finite) { return Err("box section projection is nonfinite"); }
            projected[index] = point;
            for axis in 0..2 { center[axis] += point[axis] / self.count as f64; }
            order[index] = index as u8;
        }
        let angle = |index: u8| {
            let [x, y] = projected[usize::from(index)];
            (y - center[1]).atan2(x - center[0])
        };
        order[..self.count].sort_unstable_by(|&left, &right| angle(left).total_cmp(&angle(right)));
        Ok(BoxSectionPolygon { points: take(&mut self.points), order, count: take(&mut self.count) })
    }
}

impl SolidPrimitive {
    /// 🪓️ Returns the positive and negative box halves in geometric plane order.
    pub fn split_by_axis_plane(self, origin: [f64; 3], normal: [f64; 3]) -> Result<[Self; 2], &'static str> {
        let Self::Box { origin: base, width, depth, height } = self else { return Err("original Split requires a certified box"); };
        if origin.into_iter().chain(normal).any(|coordinate| !coordinate.is_finite()) {
            return Err("box split plane must be finite");
        }
        let axis = normal.iter().position(|&value| value != 0.0).ok_or("box split plane requires a nonzero normal")?;
        if normal[axis + 1..].iter().any(|&value| value != 0.0) { return Err("original Split requires an axis-aligned plane"); }
        let mut upper_base = base.to_array();
        let mut lower_size = [width, depth, height];
        let mut upper_size = lower_size;
        lower_size[axis] = origin[axis] - upper_base[axis];
        upper_size[axis] = (upper_base[axis] + upper_size[axis]) - origin[axis];
        if lower_size[axis].min(upper_size[axis]) <= Tol::DEFAULT.value() {
            return Err("original Split requires a plane strictly inside the box");
        }
        upper_base[axis] = origin[axis];
        let shape = |origin, [width, depth, height]: [f64; 3]| Self::Box { origin, width, depth, height };
        let lower = shape(base, lower_size);
        let upper = shape(Pnt3::from_array(upper_base), upper_size);
        lower.validate()?;
        upper.validate()?;
        Ok(if normal[axis].is_sign_positive() { [upper, lower] } else { [lower, upper] })
    }
}
