//! 🧊️ Exact primitive boundary recipes shared by the cold and original constructors.

use super::*;

#[path = "🧭️placement/🦀️.rs"]
mod placement;

#[path = "✂️section/🦀️.rs"]
mod section;
pub use semio_framework_3d_brep_representation::contract::{BoxSectionDraft, SectionPolygon};

/// 🧭️ Fixed numeric parameters and topology of one analytic solid.
#[derive(Clone, Copy)]
pub enum SolidPrimitive {
    Box { origin: Pnt3, width: f64, depth: f64, height: f64 },
    Sphere { frame: Frame3, radius: f64 },
    Cylinder { frame: Frame3, radius: f64, height: f64 },
    Cone { frame: Frame3, radius: f64, height: f64 },
    Torus { frame: Frame3, major: f64, minor: f64 },
}

/// 📏️ Complete analytic definition of one oriented edge.
pub struct PrimitiveEdge {
    pub vertices: [usize; 2],
    pub curve: Curve3,
    pub range: (f64, f64),
}

/// 📎️ A face use of a primitive edge and its exact trim coordinates.
pub struct PrimitiveTrim {
    pub edge: usize,
    pub forward: bool,
    pub curve: Curve2,
    pub range: (f64, f64),
}

const TURN: (f64, f64) = (0.0, TAU);
const UNIT: (f64, f64) = (0.0, 1.0);
const MERIDIAN: (f64, f64) = (-FRAC_PI_2, FRAC_PI_2);

fn origin_at(z: f64) -> Pnt3 { Pnt3::new(0.0, 0.0, z) }

fn line(vertices: [usize; 2], a: Pnt3, b: Pnt3, range: (f64, f64)) -> PrimitiveEdge {
    PrimitiveEdge { vertices, curve: Curve3::Line { origin: a, dir: b - a }, range }
}

fn circle(vertices: [usize; 2], frame: Frame3, radius: f64, range: (f64, f64)) -> PrimitiveEdge {
    PrimitiveEdge { vertices, curve: Curve3::Circle { frame, radius }, range }
}

fn frame_at(frame: Frame3, height: f64) -> Frame3 { Frame3 { origin: frame.origin + frame.z * height, ..frame } }
fn reversed_frame(frame: Frame3) -> Frame3 { Frame3 { y: -frame.y, z: -frame.z, ..frame } }
fn meridian_frame(frame: Frame3, distance: f64) -> Frame3 {
    Frame3 { origin: frame.origin + frame.x * distance, x: frame.x, y: frame.z, z: -frame.y }
}

fn trim_line(edge: usize, forward: bool, origin: (f64, f64), direction: (f64, f64), range: (f64, f64)) -> PrimitiveTrim {
    PrimitiveTrim { edge, forward, curve: Curve2::Line { origin: Pnt2::new(origin.0, origin.1), dir: Vec2::new(direction.0, direction.1) }, range }
}

fn cap(edge: usize, forward: bool, radius: f64) -> PrimitiveTrim {
    PrimitiveTrim {
        edge, forward, curve: Curve2::Circle { center: Pnt2::new(0.0, 0.0), radius },
        range: (0.0, if forward { TAU } else { -TAU }),
    }
}

impl SolidPrimitive {
    pub fn axis_box(width: f64, depth: f64, height: f64) -> Self {
        Self::Box { origin: origin_at(0.0), width, depth, height }
    }

    fn box_corners(origin: Pnt3, width: f64, depth: f64, height: f64) -> [Pnt3; 8] {
        BoxGeometry::corners(width, depth, height).map(|corner| origin + corner.to_vec())
    }

    /// 🌐️ Creates the canonical origin-centered sphere recipe.
    pub fn sphere(radius: f64) -> Self { Self::Sphere { frame: Frame3::WORLD, radius } }

    /// 🔺️ Creates a cone with its base in the world XY plane and its apex along positive Z.
    pub fn cone(radius: f64, height: f64) -> Self { Self::Cone { frame: Frame3::WORLD, radius, height } }

    pub fn cylinder(radius: f64, height: f64) -> Self {
        Self::Cylinder { frame: Frame3::WORLD, radius, height }
    }

    pub fn torus(major: f64, minor: f64) -> Self {
        Self::Torus { frame: Frame3::WORLD, major, minor }
    }

    pub fn validate(self) -> Result<(), &'static str> {
        let positive = |value: f64| value.is_finite() && value > Tol::DEFAULT.value();
        let valid_basis = |frame: Frame3| {
            [frame.x, frame.y, frame.z].into_iter().all(|axis| (axis.dot(axis) - 1.0).abs() <= 1e-9)
                && (frame.x.cross(frame.y) - frame.z).norm() <= 1e-9
        };
        let valid = match self {
            Self::Box { origin, width, depth, height } => {
                positive(width) && positive(depth) && positive(height)
                    && [(origin.x, width), (origin.y, depth), (origin.z, height)].into_iter().all(|(lo, size)| {
                        let hi = lo + size;
                        lo.is_finite() && hi.is_finite() && hi > lo
                    })
            },
            Self::Sphere { frame, radius } => positive(radius) && valid_basis(frame)
                && frame.origin.to_array().into_iter().all(|center| {
                    let lower = center - radius;
                    let upper = center + radius;
                    lower.is_finite() && upper.is_finite() && upper > lower
                }),
            Self::Cylinder { frame, radius, height } => {
                let top = frame.origin + frame.z * height;
                positive(radius) && positive(height) && valid_basis(frame)
                    && [frame.origin.x, frame.origin.y, frame.origin.z, top.x, top.y, top.z].into_iter().all(f64::is_finite)
            },
            Self::Cone { frame, radius, height } => {
                let angle = radius.atan2(height);
                let [min, max] = self.axis_aligned_bounds();
                positive(radius) && positive(height) && valid_basis(frame)
                    && angle > Tol::DEFAULT.value() && angle < FRAC_PI_2
                    && min.to_array().into_iter().zip(max.to_array()).all(|(lo, hi)| lo.is_finite() && hi.is_finite() && hi > lo)
            }
            Self::Torus { frame, major, minor } => {
                let extent = major + minor;
                positive(major) && positive(minor) && major > minor && valid_basis(frame)
                    && [frame.origin.x, frame.origin.y, frame.origin.z].into_iter().all(|coordinate| {
                        coordinate.is_finite() && (coordinate.abs() + extent).is_finite()
                    })
            },
        };
        if valid { Ok(()) } else { Err("invalid or degenerate analytic solid dimensions") }
    }

    /// 🧮️ Computes the exact enclosed volume of constructor-certified geometry.
    pub fn volume(self) -> f64 {
        let pi = std::f64::consts::PI;
        match self {
            Self::Box { width, depth, height, .. } => width * depth * height,
            Self::Sphere { radius, .. } => 4.0 / 3.0 * pi * radius.powi(3),
            Self::Cylinder { radius, height, .. } => pi * radius.powi(2) * height,
            Self::Cone { radius, height, .. } => pi / 3.0 * radius.powi(2) * height,
            Self::Torus { major, minor, .. } => 2.0 * pi.powi(2) * major * minor.powi(2),
        }
    }

    /// 📐️ Gives the exact total surface area of the closed primitive.
    pub fn area(self) -> f64 {
        let pi = std::f64::consts::PI;
        match self {
            Self::Box{width,depth,height,..} => 2.0 * (width*depth + depth*height + width*height),
            Self::Sphere { radius, .. } => 4.0 * pi * radius.powi(2),
            Self::Cylinder{radius,height,..} => 2.0 * pi * radius * (radius + height),
            Self::Cone { radius, height, .. } => pi * radius * (radius + radius.hypot(height)),
            Self::Torus{major,minor,..} => 4.0 * pi.powi(2) * major * minor,
        }
    }

    /// 📦️ Computes the exact world-axis support interval of the analytic solid.
    pub fn axis_aligned_bounds(self) -> [Pnt3; 2] {
        let points = |start: Pnt3, end: Pnt3, extent: Vec3| {
            [
                Pnt3::new(start.x.min(end.x) - extent.x, start.y.min(end.y) - extent.y, start.z.min(end.z) - extent.z),
                Pnt3::new(start.x.max(end.x) + extent.x, start.y.max(end.y) + extent.y, start.z.max(end.z) + extent.z),
            ]
        };
        let radial = |frame: Frame3, radius: f64, padding: f64| Vec3::new(
            radius * frame.x.x.hypot(frame.y.x) + padding,
            radius * frame.x.y.hypot(frame.y.y) + padding,
            radius * frame.x.z.hypot(frame.y.z) + padding,
        );
        match self {
            Self::Box { origin, width, depth, height } => [origin, origin + Vec3::new(width, depth, height)],
            Self::Sphere { frame, radius } => points(frame.origin, frame.origin, Vec3::new(radius, radius, radius)),
            Self::Cylinder { frame, radius, height } => points(frame.origin, frame.origin + frame.z * height, radial(frame, radius, 0.0)),
            Self::Cone { frame, radius, height } => {
                let [lo, hi] = points(frame.origin, frame.origin, radial(frame, radius, 0.0));
                let top = frame.origin + frame.z * height;
                [Pnt3::new(lo.x.min(top.x), lo.y.min(top.y), lo.z.min(top.z)), Pnt3::new(hi.x.max(top.x), hi.y.max(top.y), hi.z.max(top.z))]
            },
            Self::Torus { frame, major, minor } => points(frame.origin, frame.origin, radial(frame, major, minor)),
        }
    }

    /// 📍️ Signed Euclidean distance to the closed primitive boundary.
    pub fn signed_boundary_distance(&self, point: Pnt3) -> f64 {
        match self {
            Self::Box { origin: min, width, depth, height } => {
                let max = *min + Vec3::new(*width, *depth, *height);
                let x = (min.x - point.x).max(point.x - max.x);
                let y = (min.y - point.y).max(point.y - max.y);
                let z = (min.z - point.z).max(point.z - max.z);
                x.max(0.0).hypot(y.max(0.0)).hypot(z.max(0.0)) + x.max(y).max(z).min(0.0)
            }
            Self::Sphere { frame, radius } => {
                let offset = point - frame.origin;
                offset.x.hypot(offset.y).hypot(offset.z) - *radius
            },
            Self::Cylinder { frame, radius, height } => {
                let local = frame.to_local(point);
                let radial = local.x.hypot(local.y) - *radius;
                let axial = (-local.z).max(local.z - *height);
                radial.max(0.0).hypot(axial.max(0.0)) + radial.max(axial).min(0.0)
            }
            Self::Cone { frame, radius, height } => {
                let point = frame.to_local(point);
                let (radius, height) = (*radius, *height);
                let radial = point.x.hypot(point.y);
                let base = (radial - radius).max(0.0).hypot(point.z);
                let slant = radius.hypot(height);
                let along_radial = -radius / slant;
                let along_z = height / slant;
                let along = ((radial - radius) * along_radial + point.z * along_z).clamp(0.0, slant);
                let side = (radial - (radius + along_radial * along)).hypot(point.z - along_z * along);
                let distance = base.min(side);
                if (0.0..=height).contains(&point.z) && radial <= radius * (1.0 - point.z / height) { -distance } else { distance }
            }
            Self::Torus { frame, major, minor } => {
                let local = frame.to_local(point);
                (local.x.hypot(local.y) - *major).hypot(local.z) - *minor
            }
        }
    }

    /// ⚖️ Gives the center of a closed primitive with uniform density.
    pub fn center_of_mass(self) -> Pnt3 {
        match self {
            Self::Box{origin,width,depth,height} => origin + Vec3::new(width*0.5,depth*0.5,height*0.5),
            Self::Cylinder{frame,height,..} => frame.origin + frame.z*(height*0.5),
            Self::Cone { frame, height, .. } => frame.origin + frame.z * (height * 0.25),
            Self::Sphere { frame, .. } => frame.origin,
            Self::Torus{frame,..} => frame.origin,
        }
    }

    pub fn vertex_count(self) -> usize {
        match self { Self::Box { .. } => 8, Self::Torus { .. } => 1, _ => 2 }
    }

    pub fn edge_count(self) -> usize {
        match self { Self::Box { .. } => 12, Self::Sphere { .. } | Self::Cylinder { .. } => 3, _ => 2 }
    }

    pub fn ring_sizes(self) -> &'static [usize] {
        match self {
            Self::Box { .. } => &[4, 4, 4, 4, 4, 4],
            Self::Cylinder { .. } => &[4, 1, 1],
            Self::Cone { .. } => &[3, 1],
            _ => &[4],
        }
    }

    pub fn vertex(self, index: usize) -> Pnt3 {
        match self {
            Self::Box { origin, width, depth, height } => Self::box_corners(origin, width, depth, height)[index],
            Self::Sphere { frame, radius } => frame.origin + frame.z * if index == 0 { -radius } else { radius },
            Self::Cylinder { frame, radius, height } => frame.to_world(Pnt3::new(radius, 0.0, if index == 0 { 0.0 } else { height })),
            Self::Cone { frame, radius, height } => if index == 0 { frame.origin + frame.z * height } else { frame.origin + frame.x * radius },
            Self::Torus { frame, major, minor } => frame.origin + frame.x * (major + minor),
        }
    }

    pub fn edge(self, edge: usize) -> PrimitiveEdge {
        let linear = |a, b, range| line([a, b], self.vertex(a), self.vertex(b), range);
        match self {
            Self::Box { .. } => {
                let [a, b] = BoxGeometry::EDGES[edge];
                linear(a, b, UNIT)
            }
            Self::Sphere { frame, radius } => match edge {
                0 => circle([0, 1], meridian_frame(frame, 0.0), radius, MERIDIAN),
                1 => linear(0, 0, TURN),
                _ => linear(1, 1, TURN),
            },
            Self::Cylinder { frame, radius, height } => match edge {
                0 => circle([0, 0], frame, radius, TURN),
                1 => circle([1, 1], frame_at(frame, height), radius, TURN),
                _ => linear(0, 1, UNIT),
            },
            Self::Cone { frame, radius, .. } => match edge {
                0 => circle([1, 1], frame, radius, TURN),
                _ => linear(1, 0, UNIT),
            },
            Self::Torus { frame, major, minor } => match edge {
                0 => {
                    circle([0, 0], meridian_frame(frame, major), minor, TURN)
                }
                _ => circle([0, 0], frame, major + minor, TURN),
            },
        }
    }

    pub fn surface(self, face: usize) -> Surface {
        match self {
            Self::Box { origin, width, depth, height } => Surface::Plane { frame: BoxGeometry::frame(&Self::box_corners(origin, width, depth, height), face) },
            Self::Sphere { frame, radius } => Surface::Sphere { frame, radius },
            Self::Cylinder { frame, radius, height } => match face {
                0 => Surface::Cylinder { frame, radius },
                1 => Surface::Plane { frame: reversed_frame(frame) },
                _ => Surface::Plane { frame: frame_at(frame, height) },
            },
            Self::Cone { frame, radius, height } if face == 0 => Surface::Cone { frame: reversed_frame(frame_at(frame, height)), half_angle: radius.atan2(height) },
            Self::Cone { frame, .. } => Surface::Plane { frame: reversed_frame(frame) },
            Self::Torus { frame, major, minor } => Surface::Torus { frame, major_radius: major, minor_radius: minor },
        }
    }

    pub fn trim(self, face: usize, member: usize) -> PrimitiveTrim {
        match self {
            Self::Box { .. } => {
                let (edge, forward) = BoxGeometry::FACES[face][member];
                let Surface::Plane { frame } = self.surface(face) else { unreachable!("analytic box face is planar") };
                PrimitiveTrim { edge, forward, curve: planar_pcurve(&self.edge(edge).curve, frame), range: UNIT }
            }
            Self::Sphere { .. } => match member {
                0 => trim_line(1, true, (0.0, -FRAC_PI_2), (1.0, 0.0), TURN),
                1 => trim_line(0, true, (TAU, 0.0), (0.0, 1.0), MERIDIAN),
                2 => trim_line(2, false, (0.0, FRAC_PI_2), (1.0, 0.0), TURN),
                _ => trim_line(0, false, (0.0, 0.0), (0.0, 1.0), MERIDIAN),
            },
            Self::Cylinder { radius, height, .. } => match (face, member) {
                (0, 0) => trim_line(0, true, (0.0, 0.0), (1.0, 0.0), TURN),
                (0, 1) => trim_line(2, true, (TAU, 0.0), (0.0, height), UNIT),
                (0, 2) => trim_line(1, false, (0.0, height), (1.0, 0.0), TURN),
                (0, _) => trim_line(2, false, (0.0, 0.0), (0.0, height), UNIT),
                (1, _) => cap(0, false, radius),
                _ => cap(1, true, radius),
            },
            Self::Cone { radius, height, .. } => match (face, member) {
                (0, 0) => trim_line(0, true, (TAU, height), (-1.0, 0.0), TURN),
                (0, 1) => trim_line(1, true, (0.0, height), (0.0, -height), UNIT),
                (0, _) => trim_line(1, false, (TAU, height), (0.0, -height), UNIT),
                _ => cap(0, false, radius),
            },
            Self::Torus { .. } => match member {
                0 => trim_line(1, true, (0.0, 0.0), (1.0, 0.0), TURN),
                1 => trim_line(0, true, (TAU, 0.0), (0.0, 1.0), TURN),
                2 => trim_line(1, false, (0.0, TAU), (1.0, 0.0), TURN),
                _ => trim_line(0, false, (0.0, 0.0), (0.0, 1.0), TURN),
            },
        }
    }
}

pub(super) fn make_analytic_solid(body: &mut Body, primitive: SolidPrimitive, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    primitive.validate().map_err(|reason| KernelError::InvalidInput(reason.into()))?;
    let tol = Tol::DEFAULT;
    let vertices: Vec<_> = (0..primitive.vertex_count()).map(|index| make_vertex(body, primitive.vertex(index), tol, rec)).collect();
    let edges: Vec<_> = (0..primitive.edge_count()).map(|index| {
        let PrimitiveEdge { vertices: [a, b], curve, range } = primitive.edge(index);
        let curve = body.curves3.insert(curve);
        make_edge(body, curve, range, vertices[a], vertices[b], tol, rec)
    }).collect();
    let faces = primitive.ring_sizes().iter().enumerate().map(|(index, size)| {
        let surface = body.surfaces.insert(primitive.surface(index));
        let members: Vec<_> = (0..*size).map(|member| {
            let trim = primitive.trim(index, member);
            (edges[trim.edge], trim.forward)
        }).collect();
        let face = attach_face(body, surface, &members, false, tol, rec);
        let trims: Vec<_> = (0..*size).map(|member| {
            let trim = primitive.trim(index, member);
            (body.curves2.insert(trim.curve), trim.range)
        }).collect();
        set_outer_pcurves(body, face, &trims);
        face
    }).collect();
    Ok(finish_solid(body, faces, rec))
}
