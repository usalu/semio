//! 🛢️ Exact analytic recognition of an authored periodic cylinder with two full circular caps.

use super::*;

#[derive(Clone, Copy)]
pub(super) struct CircleTrim {
    edge: u64,
    vertex: u64,
    point: [f64; 3],
    center: [f64; 3],
    axis: [f64; 3],
    radius: f64,
    forward: bool,
}

#[derive(Clone, Copy)]
enum Member {
    Circle(CircleTrim),
    Seam(Edge),
}

impl Member {
    fn endpoints(self) -> [u64; 2] {
        match self {
            Self::Circle(circle) => [circle.vertex; 2],
            Self::Seam(edge) => [edge.from, edge.to],
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum CylinderFace {
    Empty,
    Cap { plane: Placement, circle: CircleTrim },
    Wall { cylinder: Placement, radius: f64, circles: [CircleTrim; 2] },
}

impl StepSolidDecode {
    pub(super) fn cylinder_face(&self, source: &str, id: u64) -> Result<CylinderFace, ValueError> {
        let bound = self.bound(source,id)?;
        if let Ok(attributes) = self.row(source,bound.surface,"CYLINDRICAL_SURFACE") {
            let [_, axis, radius] = fields(attributes)?;
            let cylinder = self.placement(source,reference(axis)?)?;
            let radius = positive_radius(radius)?;
            if bound.count != 4 || !bound.same { return Err(cylinder_required()); }
            let first = self.cylinder_member(source, &bound, 0)?;
            let second = self.cylinder_member(source, &bound, 1)?;
            let third = self.cylinder_member(source, &bound, 2)?;
            let fourth = self.cylinder_member(source, &bound, 3)?;
            let order = [first, second, third, fourth];
            if (0..4).any(|i| order[i].endpoints()[1] != order[(i+1)%4].endpoints()[0]) {
                return Err(invalid("STEP cylinder wall boundary is discontinuous"));
            }
            let [a,b,c,d] = if matches!(first,Member::Circle(_)) { order } else { [second,third,fourth,first] };
            let (Member::Circle(first),Member::Seam(seam),Member::Circle(second),Member::Seam(reverse)) = (a,b,c,d) else {
                return Err(cylinder_required());
            };
            if first.edge == second.edge || first.vertex == second.vertex || seam.id != reverse.id
                || seam.from != reverse.to || seam.to != reverse.from {
                return Err(invalid("STEP cylinder wall requires paired uses of one authored seam"));
            }
            Ok(CylinderFace::Wall { cylinder, radius, circles: [first,second] })
        } else {
            let [_, axis] = fields(self.row(source,bound.surface,"PLANE")?)?;
            let mut plane = self.placement(source,reference(axis)?)?;
            if !bound.same { plane.axis = plane.axis.map(|value| -value); }
            if bound.count != 1 { return Err(cylinder_required()); }
            let Member::Circle(circle) = self.cylinder_member(source, &bound, 0)? else { return Err(cylinder_required()); };
            Ok(CylinderFace::Cap { plane, circle })
        }
    }

    fn cylinder_member(&self, source: &str, bound: &Bound, index: usize) -> Result<Member, ValueError> {
        let index = if bound.forward { index } else { bound.count-1-index };
        let mut edge = self.coedge(source,bound.members[index])?;
        edge.forward = edge.forward == bound.forward;
        if let Ok(attributes) = self.row(source,edge.curve,"SEAM_CURVE") {
            let [_, line, parameter_curves, _] = fields(attributes)?;
            let curves: [u64;2] = refs(parameter_curves)?;
            if curves[0] == curves[1] { return Err(cylinder_required()); }
            let mut parameter = [0.0;2];
            for (parameter, curve) in parameter.iter_mut().zip(curves) {
                let [_, surface, reference_curve] = fields(self.row(source,curve,"PCURVE")?)?;
                if reference(surface)? != bound.surface { return Err(cylinder_required()); }
                *parameter = self.seam_parameter(source,reference(reference_curve)?)?;
            }
            let span = (parameter[1]-parameter[0]).abs();
            if (span - std::f64::consts::TAU).abs() > 1e-10 { return Err(cylinder_required()); }
            return self.line_edge(source,edge,reference(line)?).map(Member::Seam);
        }
        let curve = self.wrapped_curve(source,edge.curve,"SURFACE_CURVE")?.unwrap_or(edge.curve);
        let [_, axis, radius] = fields(self.row(source,curve,"CIRCLE")?)?;
        let placement = self.placement(source,reference(axis)?)?;
        if edge.from != edge.to { return Err(unsupported("STEP cylinder caps require authored full circular trims")); }
        Ok(Member::Circle(CircleTrim {
            edge: edge.edge,
            vertex: edge.from,
            point: self.vertex(source,edge.from)?,
            center: placement.origin,
            axis: placement.axis,
            radius: positive_radius(radius)?,
            forward: edge.forward == edge.same,
        }))
    }
    fn seam_parameter(&self, source: &str, definition: u64) -> Result<f64, ValueError> {
        let [_, items, _] = fields(self.row(source,definition,"DEFINITIONAL_REPRESENTATION")?)?;
        let [line] = refs(items)?;
        let [_, origin, vector] = fields(self.row(source,line,"LINE")?)?;
        let [_, point] = fields(self.row(source,reference(origin)?,"CARTESIAN_POINT")?)?;
        let [u,v] = fields(list(point)?)?;
        number(v)?;
        let [_, direction, length] = fields(self.row(source,reference(vector)?,"VECTOR")?)?;
        let [_, vector] = fields(self.row(source,reference(direction)?,"DIRECTION")?)?;
        let [du,dv] = fields(list(vector)?)?;
        if number(du)? != 0.0 || number(dv)? == 0.0 || number(length)? <= 0.0 { return Err(cylinder_required()); }
        number(u)
    }
}

pub(super) fn certify_cylinder(faces: &[CylinderFace;3]) -> Result<StepSolid, ValueError> {
    let wall = faces.iter().find_map(|face| match face {
        CylinderFace::Wall { cylinder, radius, circles } => Some((*cylinder,*radius,*circles)),
        _ => None,
    }).ok_or_else(cylinder_required)?;
    let (cylinder,radius,circles) = wall;
    let axis = only_axis(cylinder.axis).ok_or_else(cylinder_required)?;
    let (u,v) = ((axis+1)%3,(axis+2)%3);
    let mut found = [false;2];
    let mut cap_senses = [false;2];
    for face in faces {
        let CylinderFace::Cap { plane, circle } = *face else { continue; };
        let index = circles.iter().position(|wall| wall.edge == circle.edge
            && wall.vertex == circle.vertex && wall.forward != circle.forward).ok_or_else(cylinder_required)?;
        let positive = (circle.axis[axis] > 0.0) == circle.forward;
        if found[index] || only_axis(plane.axis) != Some(axis) || only_axis(circle.axis) != Some(axis)
            || circle.center[u] != cylinder.origin[u] || circle.center[v] != cylinder.origin[v]
            || plane.origin[axis] != circle.center[axis] || circle.point[axis] != circle.center[axis]
            || circle.radius != radius || (plane.axis[axis] > 0.0) != positive {
            return Err(cylinder_required());
        }
        let authored_radius = (circle.point[u]-circle.center[u]).hypot(circle.point[v]-circle.center[v]);
        if authored_radius != radius { return Err(cylinder_required()); }
        found[index] = true;
        cap_senses[index] = plane.axis[axis] > 0.0;
    }
    if found != [true;2] || circles[0].center[axis] == circles[1].center[axis] { return Err(cylinder_required()); }
    let lower = usize::from(circles[1].center[axis] < circles[0].center[axis]);
    if cap_senses[lower] || !cap_senses[1-lower] { return Err(cylinder_required()); }
    let positive = cylinder.axis[axis] > 0.0;
    let base = circles[if positive { lower } else { 1-lower }].center;
    let top = circles[if positive { 1-lower } else { lower }].center;
    let height = (top[axis]-base[axis]).abs();
    let along = if positive { height } else { -height };
    if !height.is_finite() || base[axis]+along != top[axis] { return Err(cylinder_required()); }
    let radial = cylinder.radial.unwrap_or_else(|| Vec3::from_array(cylinder.axis).any_orthogonal().to_array());
    let proof = StepSolid::Cylinder { base, axis: cylinder.axis, radial, radius, height };
    if proof.analytic_primitive().is_none() { return Err(cylinder_required()); }
    Ok(proof)
}

fn positive_radius(text: &str) -> Result<f64, ValueError> {
    let radius = number(text)?;
    if radius <= 0.0 { return Err(invalid("STEP cylinder radius is degenerate")); }
    Ok(radius)
}

fn cylinder_required() -> ValueError { unsupported("STEP faces do not certify one complete authored axial cylinder") }
