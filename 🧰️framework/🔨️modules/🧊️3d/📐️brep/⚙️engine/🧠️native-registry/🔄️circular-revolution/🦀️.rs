//! 🔄️ Original full and partial revolutions of certified meridional profiles.

use super::*;
use semio_framework_3d_brep_representation::contract::TorusSector;
use semio_framework_3d_brep_solid_transaction::{SolidBoundary,plan_annular_revolution};
use std::f64::consts::TAU;

fn unsupported(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, message)
}

impl BrepRegistry {
    /// 🎟️ Quotes the live profile and original arena paths needed to certify its planar trim.
    pub fn planar_face_boundary_depth(&self, handle: &str) -> Result<usize, ValueError> {
        let mut depth = self.live.next_contains_depth_demand(handle)?.max(1);
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Ok(depth); };
        depth = depth.max(self.body.faces.next_remove_depth_demand(*id)?);
        let Some(face) = self.body.faces.get(*id) else { return Ok(depth); };
        depth = depth.max(self.body.surfaces.next_remove_depth_demand(face.surface)?)
            .max(semio_framework_3d_brep_solid_transaction::rectangular_face_depth(&self.body,*id)?);
        let Some(ring) = face.outer else { return Ok(depth); };
        depth = depth.max(self.body.loops.next_remove_depth_demand(ring)?);
        let Some(ring) = self.body.loops.get(ring) else { return Ok(depth); };
        depth = depth.max(self.body.coedges.next_remove_depth_demand(ring.first)?);
        let Some(coedge) = self.body.coedges.get(ring.first) else { return Ok(depth); };
        if let Some(trim) = coedge.pcurve {
            depth = depth.max(self.body.curves2.next_remove_depth_demand(trim)?);
        }
        depth = depth.max(self.body.edges.next_remove_depth_demand(coedge.edge)?);
        if let Some(edge) = self.body.edges.get(coedge.edge) {
            depth = depth.max(self.body.curves3.next_remove_depth_demand(edge.curve)?);
        }
        Ok(depth)
    }

    /// 🌀️ Plans the one closed circular sweep representable by an exact analytic torus.
    pub fn plan_closed_helical_sweep(&mut self, handle:&str, origin:EVec3, direction:EVec3, parameters:[f64;3]) -> Result<SolidPublication,ValueError> {
        let Entity::Face(profile)=self.original_query_entity(handle)? else {
            return Err(unsupported("original closed Helical Sweep requires a live circular profile face"));
        };
        let Some((Curve3::Circle{radius,..},_,_))=self.conic_face_boundary(*profile) else {
            return Err(unsupported("original closed Helical Sweep requires one centered circular profile"));
        };
        self.plan_solid_primitive(closed_helical_torus(origin,direction,parameters,*radius)?)
    }

    /// 🧭️ Plans the exact sweep of a meridional circular or rectangular face.
    pub fn plan_circular_revolution(&mut self, handle: &str, origin: EVec3, direction: EVec3, angle: f64)
        -> Result<SolidPublication, ValueError> {
        let invalid_axis = || ValueError::literal(ValueRefusalKind::InvalidValue, "revolution requires a finite axis with nonzero direction");
        let axis_length = direction[0].hypot(direction[1]).hypot(direction[2]);
        if !axis_length.is_finite() || axis_length <= Tol::DEFAULT.value() || origin.iter().any(|coordinate| !coordinate.is_finite()) {
            return Err(invalid_axis());
        }
        let sweep=angle.abs();
        if !sweep.is_finite() || sweep<=1e-12 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"revolution angle must be finite and nonzero"));
        }
        let full=sweep>=TAU-1e-9;
        let Entity::Face(id) = self.original_query_entity(handle)? else {
            return Err(unsupported("original circular revolution requires a live circular face"));
        };
        let sign=if full{1.}else{angle.signum()};
        let axis=vec3(direction.map(|value|sign*value/axis_length));
        let origin=pnt(origin);
        let Some((Curve3::Circle { frame, radius }, _, _)) = self.conic_face_boundary(*id) else {
            let sweep=if full{TAU}else{sweep};
            let shape=plan_annular_revolution(&self.body,*id,origin,axis,sweep)?;
            return self.plan_solid_boundary(SolidBoundary::Annular(shape));
        };
        let normal=frame.z;
        let center=frame.origin;
        let minor=*radius;
        let along_axis = (center - origin).dot(axis);
        let axis_center = origin + axis * along_axis;
        let radial = center - axis_center;
        let major = radial.x.hypot(radial.y).hypot(radial.z);
        if !major.is_finite() || major - minor <= Tol::DEFAULT.value() {
            return Err(unsupported("original circular revolution requires clearance between profile and axis"));
        }
        let radial = radial * major.recip();
        let plane_tol = 1e-10;
        if normal.dot(axis).abs() > plane_tol || normal.dot(radial).abs() > plane_tol {
            return Err(unsupported("original circular revolution requires the axis in the profile plane"));
        }
        let y = axis.cross(radial).normalized().ok_or_else(invalid_axis)?;
        let frame = Frame3 { origin: axis_center, x: y.cross(axis), y, z: axis };
        let primitive=SolidPrimitive::Torus{frame,major,minor};
        if full{return self.plan_solid_primitive(primitive);}
        primitive.validate().map_err(|reason|ValueError::literal(ValueRefusalKind::InvalidValue,reason))?;
        let sector=TorusSector::new(frame,major,minor,sweep,Tol::DEFAULT.value()).ok_or_else(||
            ValueError::literal(ValueRefusalKind::InvalidValue,"revolution angle does not separate its two cap rims at kernel tolerance"))?;
        self.plan_solid_boundary(SolidBoundary::TorusSector(sector))
    }
}

fn closed_helical_torus(origin:EVec3,direction:EVec3,parameters:[f64;3],minor:f64)->Result<SolidPrimitive,ValueError>{
    let [major,pitch,turns]=parameters;
    let length=direction[0].hypot(direction[1]).hypot(direction[2]);
    if !origin.into_iter().all(f64::is_finite) || !length.is_finite() || length<=Tol::DEFAULT.value()
        || !parameters.into_iter().all(f64::is_finite)
    {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Helical Sweep requires finite parameters and a nonzero axis"));
    }
    if pitch!=0. || turns.abs()!=1. {
        return Err(unsupported("exact original Helical Sweep requires zero pitch and one signed full turn"));
    }
    if major-minor<=Tol::DEFAULT.value() {
        return Err(unsupported("original closed Helical Sweep requires path radius greater than the circular profile radius"));
    }
    let axis=vec3(direction.map(|component|component/length));
    let x=axis.any_orthogonal();
    let frame=Frame3{origin:pnt(origin),x,y:axis.cross(x),z:axis};
    Ok(SolidPrimitive::Torus{frame,major,minor})
}
