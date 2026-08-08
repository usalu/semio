//! 🔮️ Ground truth used only by tests, kept deliberately independent from the kernel's own
//! algorithms (WFC-crate convention: a brute-force oracle catches bugs a self-referential test
//! never could). This module grows alongside the kernel — [`Sdf`] lands in Phase 0 with the
//! primitives it can already describe; mass-property, watertightness and shape-generator oracles
//! land in the phases that need them.

use crate::brep::mat::Trsf;
use crate::brep::vec::Pnt3;
use crate::brep::topo::Body;

// #region 🔖️Sdf

/// 🔮️ A closed-form signed distance field: negative inside, zero on the boundary, positive
/// outside. Used to probe classification and Boolean results independently of the kernel's own
/// ray-casting/arrangement code.
#[derive(Clone, Debug, PartialEq)]
pub enum Sdf {
    /// 🔮️ Axis-aligned box of the given half-extents, centered at the origin before `placement`.
    Box {
        half_extents: Pnt3,
        placement: Trsf,
    },
    /// 🔮️ Sphere of the given radius, centered at the origin before `placement`.
    Sphere {
        radius: f64,
        placement: Trsf,
    },
    /// 🔮️ Cylinder of the given radius and half-height, axis along local `z`, centered at the
    /// origin before `placement`.
    Cylinder {
        radius: f64,
        half_height: f64,
        placement: Trsf,
    },
    /// 🔮️ Capped cone along local `z`, radius `radius` at `z = -half_height` tapering to apex at
    /// `z = +half_height`, centered at the origin before `placement`.
    Cone {
        radius: f64,
        half_height: f64,
        placement: Trsf,
    },
    /// 🔮️ Torus in the local `xy` plane, major circle radius `major_radius`, tube radius
    /// `minor_radius`, axis along local `z`, centered at the origin before `placement`.
    Torus {
        major_radius: f64,
        minor_radius: f64,
        placement: Trsf,
    },
    /// 🔮️ Boolean combination of two fields.
    Union(Box<Sdf>, Box<Sdf>),
    Intersect(Box<Sdf>, Box<Sdf>),
    Difference(Box<Sdf>, Box<Sdf>),
}

impl Sdf {
    /// 🔮️ Evaluates the field at a world-space point.
    pub fn eval(&self, p: Pnt3) -> f64 {
        match self {
            Sdf::Box { half_extents, placement } => {
                let local = placement.inverse().apply_point(p);
                let dx = local.x.abs() - half_extents.x;
                let dy = local.y.abs() - half_extents.y;
                let dz = local.z.abs() - half_extents.z;
                let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2) + dz.max(0.0).powi(2)).sqrt();
                let inside = dx.max(dy).max(dz).min(0.0);
                outside + inside
            }
            Sdf::Sphere { radius, placement } => {
                let local = placement.inverse().apply_point(p);
                local.to_vec().norm() - radius
            }
            Sdf::Cylinder { radius, half_height, placement } => {
                let local = placement.inverse().apply_point(p);
                let radial = (local.x * local.x + local.y * local.y).sqrt() - radius;
                let axial = local.z.abs() - half_height;
                let outside = (radial.max(0.0).powi(2) + axial.max(0.0).powi(2)).sqrt();
                let inside = radial.max(axial).min(0.0);
                outside + inside
            }
            Sdf::Cone { radius, half_height, placement } => {
                let local = placement.inverse().apply_point(p);
                capped_cone_z(&local, *half_height, *radius, 0.0)
            }
            Sdf::Torus { major_radius, minor_radius, placement } => {
                let local = placement.inverse().apply_point(p);
                let qx = (local.x * local.x + local.y * local.y).sqrt() - major_radius;
                let qz = local.z;
                (qx * qx + qz * qz).sqrt() - minor_radius
            }
            Sdf::Union(a, b) => a.eval(p).min(b.eval(p)),
            Sdf::Intersect(a, b) => a.eval(p).max(b.eval(p)),
            Sdf::Difference(a, b) => a.eval(p).max(-b.eval(p)),
        }
    }
    /// 🔮️ `true` when `p` is inside (or on, within `tol`) the field's boundary.
    pub fn contains(&self, p: Pnt3, tol: f64) -> bool {
        self.eval(p) <= tol
    }
    pub fn union(self, other: Sdf) -> Sdf {
        Sdf::Union(Box::new(self), Box::new(other))
    }
    pub fn intersect(self, other: Sdf) -> Sdf {
        Sdf::Intersect(Box::new(self), Box::new(other))
    }
    pub fn difference(self, other: Sdf) -> Sdf {
        Sdf::Difference(Box::new(self), Box::new(other))
    }
}

/// 🔮️ Capped cone SDF along `z` with base radius `r1` at `z = -h` and `r2` at `z = +h`.
fn capped_cone_z(p: &Pnt3, h: f64, r1: f64, r2: f64) -> f64 {
    let qx = (p.x * p.x + p.y * p.y).sqrt();
    let k1_x = r2;
    let k1_z = h;
    let k2_x = r2 - r1;
    let k2_z = 2.0 * h;
    let cap_r = if p.z < 0.0 { r1 } else { r2 };
    let ca_x = qx - qx.min(cap_r);
    let ca_z = p.z.abs() - h;
    let dot_k1_q = k1_x * (k1_x - qx) + k1_z * (k1_z - p.z);
    let dot_k2_k2 = k2_x * k2_x + k2_z * k2_z;
    let t = (dot_k1_q / dot_k2_k2).clamp(0.0, 1.0);
    let cb_x = qx - k1_x + k2_x * t;
    let cb_z = p.z - k1_z + k2_z * t;
    let sign = if cb_x < 0.0 && ca_z < 0.0 { -1.0 } else { 1.0 };
    let ca_len = ca_x * ca_x + ca_z * ca_z;
    let cb_len = cb_x * cb_x + cb_z * cb_z;
    sign * ca_len.min(cb_len).sqrt()
}

// #endregion 🔖️Sdf

// #region 🔖️ClosedFormMass

/// 🔮️ Closed-form volume and surface area for analytic primitives (test oracle vs [`crate::brep::measure`]).
pub struct ClosedFormMass;

impl ClosedFormMass {
    /// 🔮️ Volume of an axis-aligned box with the given half-extents.
    pub fn box_volume(half_extents: Pnt3) -> f64 {
        8.0 * half_extents.x * half_extents.y * half_extents.z
    }
    /// 🔮️ Total surface area of an axis-aligned box with the given half-extents.
    pub fn box_surface_area(half_extents: Pnt3) -> f64 {
        8.0 * (half_extents.x * half_extents.y + half_extents.y * half_extents.z + half_extents.x * half_extents.z)
    }
    /// 🔮️ Volume of a sphere with the given radius.
    pub fn sphere_volume(radius: f64) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3)
    }
    /// 🔮️ Surface area of a sphere with the given radius.
    pub fn sphere_surface_area(radius: f64) -> f64 {
        4.0 * std::f64::consts::PI * radius.powi(2)
    }
    /// 🔮️ Volume of a right circular cylinder (including caps) with radius and full height `2 * half_height`.
    pub fn cylinder_volume(radius: f64, half_height: f64) -> f64 {
        std::f64::consts::PI * radius.powi(2) * (2.0 * half_height)
    }
    /// 🔮️ Total surface area of a capped right circular cylinder.
    pub fn cylinder_surface_area(radius: f64, half_height: f64) -> f64 {
        2.0 * std::f64::consts::PI * radius * (radius + 2.0 * half_height)
    }
}

// #endregion 🔖️ClosedFormMass

// #region 🔖️Watertightness

/// 🔮️ Watertightness classification returned by the oracle checker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatertightnessVerdict {
    /// 🔮️ Every edge is shared by exactly two faces with consistent orientation.
    Watertight,
    /// 🔮️ At least one boundary edge remains (open shell or non-manifold rim).
    HasBoundaryEdges { count: usize },
    /// 🔮️ Topology not inspected yet (stub until sew/heal lanes wire real counts).
    NotChecked,
}

/// 🔮️ Summary of a watertightness probe for differential tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WatertightnessReport {
    pub verdict: WatertightnessVerdict,
}

/// 🔮️ Stub API: derives a verdict from a pre-counted boundary-edge tally supplied by future topo tests.
pub fn watertightness_from_boundary_edge_count(boundary_edges: usize) -> WatertightnessReport {
    let verdict = if boundary_edges == 0 {
        WatertightnessVerdict::Watertight
    } else {
        WatertightnessVerdict::HasBoundaryEdges { count: boundary_edges }
    };
    WatertightnessReport { verdict }
}

/// 🔮️ Count edges whose coedge valence is not exactly two (boundary or non-manifold).
pub fn count_boundary_edges(body: &Body) -> usize {
    let mut count = 0usize;
    for (edge_id, _) in body.edges.iter() {
        let valence = body.edge_coedges(edge_id).len();
        if valence != 2 {
            count += 1;
        }
    }
    count
}

/// 🔮️ Real watertightness probe from body topology (boundary/non-manifold edge valence).
pub fn watertightness_of_body(body: &Body) -> WatertightnessReport {
    watertightness_from_boundary_edge_count(count_boundary_edges(body))
}

/// 🔮️ Compatibility alias retained for older call sites; prefer [`watertightness_of_body`].
pub fn watertightness_stub_unchecked() -> WatertightnessReport {
    WatertightnessReport {
        verdict: WatertightnessVerdict::NotChecked,
    }
}

// #endregion 🔖️Watertightness

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_sdf_is_negative_inside_and_positive_outside() {
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(5.0, 0.0, 0.0)) > 0.0);
        assert!((b.eval(Pnt3::new(1.0, 0.0, 0.0))).abs() < 1e-9);
    }

    #[test]
    fn sphere_sdf_matches_analytic_distance() {
        let s = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        assert!((s.eval(Pnt3::new(5.0, 0.0, 0.0)) - 3.0).abs() < 1e-9);
        assert!((s.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-2.0)).abs() < 1e-9);
    }

    #[test]
    fn cylinder_sdf_is_correct_on_axis_and_cap() {
        let c = Sdf::Cylinder { radius: 1.0, half_height: 2.0, placement: Trsf::IDENTITY };
        assert!((c.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-1.0)).abs() < 1e-9);
        assert!((c.eval(Pnt3::new(0.0, 0.0, 5.0)) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn torus_sdf_is_negative_on_major_circle_and_positive_outside_tube() {
        let t = Sdf::Torus {
            major_radius: 2.0,
            minor_radius: 0.5,
            placement: Trsf::IDENTITY,
        };
        assert!(t.eval(Pnt3::new(2.0, 0.0, 0.0)) < 0.0);
        assert!((t.eval(Pnt3::new(2.5, 0.0, 0.0))).abs() < 1e-8);
        assert!(t.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }

    #[test]
    fn cone_sdf_is_negative_inside_taper_and_positive_outside() {
        let c = Sdf::Cone { radius: 1.0, half_height: 1.0, placement: Trsf::IDENTITY };
        assert!(c.eval(Pnt3::new(0.0, 0.0, -0.5)) < 0.0);
        assert!((c.eval(Pnt3::new(1.0, 0.0, -1.0))).abs() < 1e-8);
        assert!(c.eval(Pnt3::new(2.0, 0.0, 0.0)) > 0.0);
    }

    #[test]
    fn union_is_the_min_and_matches_containment_of_either_operand() {
        let a = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::brep::vec::Vec3::new(-1.0, 0.0, 0.0)) };
        let b = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::brep::vec::Vec3::new(1.0, 0.0, 0.0)) };
        let u = a.union(b);
        assert!(u.contains(Pnt3::new(-1.0, 0.0, 0.0), 1e-9));
        assert!(u.contains(Pnt3::new(1.0, 0.0, 0.0), 1e-9));
        assert!(!u.contains(Pnt3::new(5.0, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn difference_removes_the_second_operand() {
        let big = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        let small = Sdf::Sphere { radius: 1.0, placement: Trsf::IDENTITY };
        let d = big.difference(small);
        assert!(!d.contains(Pnt3::new(0.0, 0.0, 0.0), 1e-9));
        assert!(d.contains(Pnt3::new(1.5, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn placed_box_sdf_respects_transform() {
        let placement = Trsf::translation(crate::brep::vec::Vec3::new(10.0, 0.0, 0.0));
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement };
        assert!(b.eval(Pnt3::new(10.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }

    #[test]
    fn closed_form_mass_matches_textbook_box_sphere_cylinder() {
        let half = Pnt3::new(1.0, 2.0, 3.0);
        assert!((ClosedFormMass::box_volume(half) - 48.0).abs() < 1e-12);
        assert!((ClosedFormMass::box_surface_area(half) - 88.0).abs() < 1e-12);
        assert!((ClosedFormMass::sphere_volume(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::sphere_surface_area(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::cylinder_volume(2.0, 3.0) - 24.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::cylinder_surface_area(2.0, 3.0) - 32.0 * std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn watertightness_stub_classifies_boundary_edge_count() {
        let tight = watertightness_from_boundary_edge_count(0);
        assert_eq!(tight.verdict, WatertightnessVerdict::Watertight);
        let open = watertightness_from_boundary_edge_count(3);
        assert_eq!(open.verdict, WatertightnessVerdict::HasBoundaryEdges { count: 3 });
        assert_eq!(watertightness_stub_unchecked().verdict, WatertightnessVerdict::NotChecked);
    }
}

    #[test]
    fn watertightness_of_box_is_watertight() {
        use crate::brep::primitives::make_box;
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let _ = solid;
        let report = watertightness_of_body(&body);
        assert_eq!(report.verdict, WatertightnessVerdict::Watertight);
    }

// #endregion 🔖️Tests
