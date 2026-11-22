//! 📍️ Original point classification and certified solid separation.

use super::*;
use semio_framework_3d_brep_representation::contract::PointClassification;

use semio_framework_3d_brep_solid_transaction::OrientedBox;

const CONTACT_TOLERANCE: f64 = 1e-6;

enum CertifiedDistanceSolid<'a> {
    Box(OrientedBox),
    Sphere(Pnt3, f64),
    Primitive(SolidPrimitive),
    Material(&'a SolidProvenance),
}

impl BrepRegistry {
    /// 🎟️ Quotes the live index and native solid arena used by a certified query.
    pub fn certified_solid_depth(&self, handle: &str) -> Result<usize, ValueError> {
        let depth = self.tessellation_source_depth_demand(handle)?.max(1);
        let source=self.original_query_entity(handle)?;
        if let Entity::Compound(solids,_)=source {
            return semio_framework_3d_brep_analysis::planar_compound_classification_depth(&self.body,solids).transpose().map(|quoted|depth.max(quoted.unwrap_or(1)));
        }
        let Entity::Solid(id, proof) = source else { return Ok(depth); };
        let solid_depth = if matches!(proof, None | Some(SolidProvenance::PlanarBoundary(None))) {
            semio_framework_3d_brep_analysis::planar_classification_depth(&self.body, *id)?
        } else { self.body.solids.next_remove_depth_demand(*id)? };
        Ok(depth.max(solid_depth))
    }

    /// 📌️ Classifies one point against the exact boundary of certified solid geometry.
    pub fn original_solid_classification(&self, handle: &str, point: EVec3) -> Result<PointClassification, ValueError> {
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "point classification requires finite coordinates and distance");
        if !point.into_iter().all(f64::is_finite) { return Err(invalid()); }
        let unsupported_shape = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original point classification requires a supported certified boundary");
        let entity=self.original_query_entity(handle)?;
        if let Entity::Compound(solids,_)=entity {
            return semio_framework_3d_brep_analysis::classify_planar_compound(&self.body,solids,pnt(point),CONTACT_TOLERANCE).ok_or_else(unsupported_shape);
        }
        let Entity::Solid(id, Some(source)) = entity else { return Err(unsupported_shape()); };
        if !self.body.solids.contains(*id) { return Err(ValueError::invariant("live point classification solid is missing")); }
        if matches!(source, SolidProvenance::PlanarBoundary(None)) {
            return semio_framework_3d_brep_analysis::classify_planar_solid(&self.body, *id, pnt(point), CONTACT_TOLERANCE).ok_or_else(unsupported_shape);
        }
        let distance=source.certified_signed_distance(pnt(point))?;
        if !distance.is_finite() { return Err(invalid()); }
        Ok(if distance.abs() <= CONTACT_TOLERANCE { PointClassification::OnBoundary }
            else if distance < 0.0 { PointClassification::Inside }
            else { PointClassification::Outside })
    }

    /// 📌️ Projects onto exposed rectangles of an original orthogonal solid boundary.
    pub fn original_orthogonal_nearest(&self, source: &SolidProvenance, target: Pnt3) -> Result<Pnt3, ValueError> {
        let (cells, frame) = match source {
            SolidProvenance::Orthogonal(cells) => (cells, Frame3::WORLD),
            SolidProvenance::PlacedOrthogonal(cells, frame) => (cells, *frame),
            _ => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "geometry requires original closest-point traversal")),
        };
        let (nearest, distance) = cells.nearest_boundary(frame.to_local(target).to_array());
        distance.is_finite().then(|| frame.to_world(Pnt3::from_array(nearest)))
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "closest-point distance exceeds finite numeric range"))
    }

    /// 📏️ Computes minimum separation of supported certified solid pairs.
    pub fn original_solid_distance(&self, a: &str, b: &str) -> Result<f64, ValueError> {
        use semio_framework_3d_brep_analysis::{axial_cylinder_box_distance, parallel_cylinder_distance, separated_box_distance, sphere_boundary_gap, sphere_box_distance, sphere_pair_distance};
        use semio_framework_3d_brep_construction::skew_box_clip::boxes_separated;
        use CertifiedDistanceSolid::{Box, Material, Primitive, Sphere};
        use SolidPrimitive::Cylinder;
        let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original solid distance requires a supported pair of certified solids");
        let certified_shape = |handle| {
            let Entity::Solid(id, Some(source)) = self.original_query_entity(handle)? else { return Err(unsupported()); };
            if !self.body.solids.contains(*id) { return Err(ValueError::invariant("live distance solid is missing")); }
            let rectangular = match source {
                SolidProvenance::Primitive(source) => match source.primitive {
                    SolidPrimitive::Sphere { frame, radius } => return Ok(Sphere(frame.origin, radius)),
                    primitive => return Ok(OrientedBox::from_primitive(primitive).map_or(Primitive(primitive), Box)),
                },
                SolidProvenance::PlanarBoundary(Some(source)) => source.rectangular(),
                SolidProvenance::SphericalShell(..) | SolidProvenance::Orthogonal(_) | SolidProvenance::PlacedOrthogonal(..) => return Ok(Material(source)),
                _ => None,
            };
            rectangular.map(Box).ok_or_else(unsupported)
        };
        match (certified_shape(a)?, certified_shape(b)?) {
            (Primitive(Cylinder { frame: a, radius: ra, height: ha }),
             Primitive(Cylinder { frame: b, radius: rb, height: hb })) =>
                parallel_cylinder_distance((a, ra, ha), (b, rb, hb))?.ok_or_else(unsupported),
            (Primitive(Cylinder { frame, radius, height }), Box(box_shape)) |
            (Box(box_shape), Primitive(Cylinder { frame, radius, height })) =>
                axial_cylinder_box_distance((frame, radius, height), box_shape.placement())?.ok_or_else(unsupported),
            (Sphere(a, ra), Sphere(b, rb)) => sphere_pair_distance((a, ra), (b, rb)),
            (Sphere(center, radius), Box(box_shape)) | (Box(box_shape), Sphere(center, radius)) =>
                sphere_box_distance((center, radius), box_shape.placement()),
            (Sphere(center, radius), Primitive(shape)) | (Primitive(shape), Sphere(center, radius)) =>
                sphere_boundary_gap(shape.signed_boundary_distance(center), radius),
            (Sphere(center, radius), Material(shape)) | (Material(shape), Sphere(center, radius)) =>
                sphere_boundary_gap(shape.certified_signed_distance(center)?, radius),
            (Box(a), Box(b)) => {
                let (a, b) = (a.placement(), b.placement());
                match boxes_separated(a, b, 0.0) {
                    Some(true) => separated_box_distance(a, b),
                    Some(false) => Ok(0.0),
                    None => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "box distance exceeds finite numeric range")),
                }
            }
            _ => Err(unsupported()),
        }
    }
}
