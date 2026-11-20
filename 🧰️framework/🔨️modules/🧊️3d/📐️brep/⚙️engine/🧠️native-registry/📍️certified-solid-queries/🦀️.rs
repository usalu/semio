//! 📍️ Original point classification and rectangular solid separation.

use super::*;
use semio_framework_3d_brep_representation::contract::PointClassification;

const CONTACT_TOLERANCE: f64 = 1e-6;

fn unsupported(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, message)
}

fn bore_boundary_distance(source: &BorePreviewSource, point: EVec3) -> f64 {
    let [x0, y0, z0] = source.origin;
    let [width, depth, height] = source.dimensions;
    let [x1, y1, z1] = [x0 + width, y0 + depth, z0 + height];
    let [x, y, z] = point;
    let overflow = |p: f64, low: f64, high: f64| (low - p).max(p - high).max(0.0);
    let x_gap = overflow(x, x0, x1);
    let y_gap = overflow(y, y0, y1);
    let z_gap = overflow(z, z0, z1);
    let x_side = (x - x0).abs().min((x - x1).abs());
    let y_side = (y - y0).abs().min((y - y1).abs());
    let z_cap = (z - z0).abs().min((z - z1).abs());
    let radial = (x - source.center[0]).hypot(y - source.center[1]);
    let x_faces = x_side.hypot(y_gap).hypot(z_gap);
    let y_faces = y_side.hypot(x_gap).hypot(z_gap);
    let end_faces = z_cap.hypot(x_gap.hypot(y_gap).max(source.radius - radial));
    let wall = (radial - source.radius).hypot(z_gap);
    let distance = x_faces.min(y_faces).min(end_faces).min(wall);
    if x_gap == 0.0 && y_gap == 0.0 && z_gap == 0.0 && radial >= source.radius { -distance } else { distance }
}

impl BrepRegistry {
    /// 🎟️ Quotes the live index and native solid arena used by a certified query.
    pub fn certified_solid_depth(&self, handle: &str) -> Result<usize, ValueError> {
        let depth = self.tessellation_source_depth_demand(handle)?.max(1);
        let Entity::Solid(id, Some(_)) = self.original_query_entity(handle)? else { return Ok(depth); };
        Ok(depth.max(self.body.solids.next_remove_depth_demand(*id)?))
    }

    /// 📌️ Classifies one point against the exact boundary of certified solid geometry.
    pub fn original_solid_classification(&self, handle: &str, point: EVec3) -> Result<PointClassification, ValueError> {
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "point classification requires finite coordinates and distance");
        if !point.into_iter().all(f64::is_finite) { return Err(invalid()); }
        let unsupported_shape = || unsupported("original point classification requires a certified primitive or bore");
        let Entity::Solid(id, Some(source)) = self.original_query_entity(handle)? else { return Err(unsupported_shape()); };
        if !self.body.solids.contains(*id) { return Err(ValueError::invariant("live point classification solid is missing")); }
        let distance = match source {
            SolidProvenance::Primitive(source) => source.primitive.signed_boundary_distance(pnt(point)),
            SolidProvenance::Bore(source) => bore_boundary_distance(source, point),
            SolidProvenance::PlanarBoundary => return Err(unsupported_shape()),
        };
        if !distance.is_finite() { return Err(invalid()); }
        Ok(if distance.abs() <= CONTACT_TOLERANCE { PointClassification::OnBoundary }
            else if distance < 0.0 { PointClassification::Inside }
            else { PointClassification::Outside })
    }

    /// 📏️ Computes the separation of two closed boxes, returning zero at contact or overlap.
    pub fn original_box_distance(&self, a: &str, b: &str) -> Result<f64, ValueError> {
        let bounds = |handle| {
            let unsupported = || unsupported("original solid distance requires two certified boxes");
            let Entity::Solid(id, Some(SolidProvenance::Primitive(source))) = self.original_query_entity(handle)? else { return Err(unsupported()); };
            if !self.body.solids.contains(*id) { return Err(ValueError::invariant("live distance solid is missing")); }
            let SolidPrimitive::Box { origin, width, depth, height } = &source.primitive else { return Err(unsupported()); };
            Ok([evec(*origin), evec(*origin + NativeVec3::new(*width, *depth, *height))])
        };
        let [a_min, a_max] = bounds(a)?;
        let [b_min, b_max] = bounds(b)?;
        let gap = |axis: usize| (a_min[axis] - b_max[axis]).max(b_min[axis] - a_max[axis]).max(0.0);
        let distance = gap(0).hypot(gap(1)).hypot(gap(2));
        if !distance.is_finite() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "box distance exceeds finite numeric range"));
        }
        Ok(distance)
    }
}
