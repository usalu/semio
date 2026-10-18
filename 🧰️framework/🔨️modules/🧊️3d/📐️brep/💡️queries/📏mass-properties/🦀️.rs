//! 📏️ Divergence-theorem mass properties, axis-aligned bounds, and solid distance/classify
//! queries on `SemioBrepSnapshot`'s arena `Body`. `oracle` (below) is a closed-form ground
//! truth used only by tests — deliberately independent of this module's own algorithms.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/{📏️measure,🔮️oracles}/🦀️.rs` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL2.

// 📏 Divergence-theorem mass properties, axis-aligned bounds, and solid distance queries on [`semio_framework_3d_brep_representation::topology::Body`].

#[cfg(test)]
use semio_framework_3d_brep_representation::arena::{CoedgeId, VertexId};
use semio_framework_3d_brep_representation::arena::{EdgeId, FaceId, ShellId, SolidId};
use semio_framework_3d_brep_curve::curve_ops;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::error::KernelError;
use semio_framework_3d_brep_surface::surface_ops;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_vector::predicates::{orient2d, Orient};
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec3};

// #region 🔖️Types

/// 📦 Axis-aligned bounds in model space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisAlignedBox {
    pub min: Pnt3,
    pub max: Pnt3,
}

/// 📏 Volume, area, centroid and inertia tensor of a solid, integrated over its TRIMMED support
/// (ear-clipped UV triangulation, adaptive Gauss-Legendre per triangle — see `solid_mass_properties`),
/// with an error estimate propagated from that adaptive refinement rather than an unverified guess.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MassProperties {
    pub volume: f64,
    pub area: f64,
    pub centroid: Pnt3,
    pub inertia: [[f64; 3]; 3],
    pub error_estimate: f64,
}

// #endregion 🔖️Types

#[path = "📐️solid-mass-properties/🦀️.rs"]
mod solid_measures;
pub use solid_measures::{area_centroid_from_moments, area_from_moments, faces_moments, mass_from_moments, shell_signed_volume, solid_bounding_box, solid_center_of_mass, solid_mass_properties, solid_mass_with_area_centroid, solid_signed_volume, solid_surface_area, solid_volume};

#[path = "📍️face-edge-and-solid-distance/🦀️.rs"]
mod face_edge_and_solid_distance;
use face_edge_and_solid_distance::solid_unique_edges;
pub use face_edge_and_solid_distance::{closest_point_on_solid, distance_solid_solid, edge_length, face_area, solids_overlap};

#[path = "🧮️moment-quadrature-and-triangulation/🦀️.rs"]
mod moment_quadrature_and_triangulation;
use moment_quadrature_and_triangulation::{IDX_AMX, IDX_AMY, IDX_AMZ, IDX_AREA, IDX_JXX2, IDX_JXY, IDX_JXZ, IDX_JYY2, IDX_JYZ, IDX_JZZ2, IDX_MX, IDX_MY, IDX_MZ, IDX_VOL, MOMENT_COMPONENTS, loop_moments};

#[path = "🔁️loop-areas-and-volume-moments/🦀️.rs"]
mod loop_areas_and_volume_moments;
use loop_areas_and_volume_moments::{face_surface, face_volume_moments, loop_area};
pub use loop_areas_and_volume_moments::face_volume_contribution;

#[path = "🗺️uv-polygons/🦀️.rs"]
mod uv_polygons;
use uv_polygons::{loop_uv_polygon, loop_uv_polygon_with, point_in_uv_polygon};
pub use uv_polygons::{loop_uv_signed_area, surface_uv};

#[path = "🧭️face-samples-and-closest-point/🦀️.rs"]
mod face_samples_and_closest_point;
use face_samples_and_closest_point::{expand_bbox_for_surface, try_analytic_sphere_volume};
pub use face_samples_and_closest_point::{closest_point_on_face, face_sample_points};

// #region 🔖️Classify
//
// 🏷️ The ray-parity classifier that used to live here was deleted (ticket
// 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave W1-F): `inferences::classification::point_in_solid`
// is now the ONE authoritative point-in-solid classifier (BVH-culled, p-curve-aware trims, grazing
// retry) — see that module's docstring. `distance_solid_solid` below now calls the real classifier
// for its overlap check instead of keeping a second, coarser implementation in sync with it.
// #endregion 🔖️Classify

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests

// #region 🔖️Oracle
pub mod oracle {
    //! 🔮️ Ground truth used only by tests, kept deliberately independent from the kernel's own
    //! algorithms (WFC-crate convention: a brute-force oracle catches bugs a self-referential test
    //! never could). This module grows alongside the kernel — [`Sdf`] lands in Phase 0 with the
    //! primitives it can already describe; mass-property, watertightness and shape-generator oracles
    //! land in the phases that need them.

    use semio_framework_3d_brep_representation::topology::Body;
    use semio_framework_3d_brep_vector::matrix::Trsf;
    use semio_framework_3d_brep_vector::Pnt3;

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
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn contains(&self, p: Pnt3, tol: f64) -> bool {
            self.eval(p) <= tol
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn union(self, other: Sdf) -> Sdf {
            Sdf::Union(Box::new(self), Box::new(other))
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn intersect(self, other: Sdf) -> Sdf {
            Sdf::Intersect(Box::new(self), Box::new(other))
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn difference(self, other: Sdf) -> Sdf {
            Sdf::Difference(Box::new(self), Box::new(other))
        }
    }

    /// 🔮️ Capped cone SDF along `z` with base radius `r1` at `z = -h` and `r2` at `z = +h`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    /// 🔮️ Closed-form volume and surface area for analytic primitives (test oracle vs the sibling `super` mass-properties module).
    pub struct ClosedFormMass;

    impl ClosedFormMass {
        /// 🔮️ Volume of an axis-aligned box with the given half-extents.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn box_volume(half_extents: Pnt3) -> f64 {
            8.0 * half_extents.x * half_extents.y * half_extents.z
        }
        /// 🔮️ Total surface area of an axis-aligned box with the given half-extents.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn box_surface_area(half_extents: Pnt3) -> f64 {
            8.0 * (half_extents.x * half_extents.y + half_extents.y * half_extents.z + half_extents.x * half_extents.z)
        }
        /// 🔮️ Volume of a sphere with the given radius.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn sphere_volume(radius: f64) -> f64 {
            (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3)
        }
        /// 🔮️ Surface area of a sphere with the given radius.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn sphere_surface_area(radius: f64) -> f64 {
            4.0 * std::f64::consts::PI * radius.powi(2)
        }
        /// 🔮️ Volume of a right circular cylinder (including caps) with radius and full height `2 * half_height`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn cylinder_volume(radius: f64, half_height: f64) -> f64 {
            std::f64::consts::PI * radius.powi(2) * (2.0 * half_height)
        }
        /// 🔮️ Total surface area of a capped right circular cylinder.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    }

    /// 🔮️ Summary of a watertightness probe for differential tests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WatertightnessReport {
        pub verdict: WatertightnessVerdict,
    }

    /// 🔮️ Derives a verdict from a boundary-edge tally.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn watertightness_from_boundary_edge_count(boundary_edges: usize) -> WatertightnessReport {
        let verdict = if boundary_edges == 0 { WatertightnessVerdict::Watertight } else { WatertightnessVerdict::HasBoundaryEdges { count: boundary_edges } };
        WatertightnessReport { verdict }
    }

    /// 🔮️ Count edges whose coedge valence is not exactly two (boundary or non-manifold).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn watertightness_of_body(body: &Body) -> WatertightnessReport {
        watertightness_from_boundary_edge_count(count_boundary_edges(body))
    }

    // #endregion 🔖️Watertightness

    // #region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️oracle-unit/🦀️.rs");

    #[cfg(test)]
    include!("🧪️tests/🔬️oracle-standalone/🦀️.rs");

    // #endregion 🔖️Tests
}
// #endregion 🔖️Oracle
