//! 📐️ Neutral kernel contract types owned by `🧊️brep` itself: `Vec3`/`Aabb`/`ParamDomain`/
//! `FaceGroup`/`EdgeGroup`/`FaceInfo`/`EdgeInfo`/`MeshTransfer`/`PointClassification` moved
//! verbatim from `semio_framework_3d::engine` (ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME
//! wave 1, W1-A) — the kernel no longer reaches across the `stdio → semio-framework-3d` forward
//! edge for its own wire contract. `EdgeGroup`/`FaceInfo`/`EdgeInfo` are new: they carry the
//! per-entity metadata the CAD renderer's `MeshTransfer` bridge expects (see
//! `🧰️framework/🔨️modules/🧊️3d/🟦️.ts` `MeshTransfer`/`FaceInfo`/`EdgeInfo`, and
//! `📓️explore-js-legacy-and-wasm-bridge.md` §7's gap list).
//!
//! `OpQuality`/`operation_quality` are new: every `BrepKernel` method now carries explicit
//! capability metadata (audit Phase 0/1) instead of silently mixing exact analytic results with
//! tessellate-then-rebuild mesh approximations under one undifferentiated `Result`.

/// 📐️ Column vector `[x,y,z]`.
pub type Vec3 = [f64; 3];

/// 📦️ Axis-aligned bounds.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// 📏️ Parametric domain `[min, max]`.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ParamDomain {
    pub min: f64,
    pub max: f64,
}

/// 🧩️ Triangle index range for one B-Rep face.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FaceGroup {
    pub start: u32,
    pub count: u32,
    pub entity_id: String,
}

/// 🧵️ Line-segment index range for one B-Rep edge.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EdgeGroup {
    pub start: u32,
    pub count: u32,
    pub entity_id: String,
}

/// 🏄️ Analytic surface family behind a tessellated face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum SurfaceKind {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    Nurbs,
}

/// ➰️ Analytic curve family behind a tessellated edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum CurveKind {
    Line,
    Circle,
    Ellipse,
    Nurbs,
}

/// 🖼️ Per-face metadata alongside a tessellated face group.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FaceInfo {
    pub entity_id: String,
    pub surface_kind: SurfaceKind,
    pub area: f64,
    pub normal: Vec3,
}

/// 🖇️ Per-edge metadata alongside a tessellated edge group.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EdgeInfo {
    pub entity_id: String,
    pub curve_kind: CurveKind,
    pub length: f64,
}

/// 🖼️ Tessellated mesh payload for preview upload.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct MeshTransfer {
    pub position: Vec<f32>,
    pub normal: Vec<f32>,
    pub index: Vec<u32>,
    pub edges: Vec<f32>,
    #[value(default)]
    pub points: Vec<f32>,
    pub face_groups: Vec<FaceGroup>,
    #[value(default)]
    pub edge_groups: Vec<EdgeGroup>,
    #[value(default)]
    pub face_infos: Vec<FaceInfo>,
    #[value(default)]
    pub edge_infos: Vec<EdgeInfo>,
}

/// 📍️ Point classification relative to a solid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PointClassification {
    Inside,
    Outside,
    OnBoundary,
}

/// 🎯️ Fidelity of a `BrepKernel` operation's result relative to true analytic B-Rep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum OpQuality {
    ExactAnalytic,
    ExactNumericalWithinTolerance,
    ApproximateBRep,
    MeshDerivedBRep,
    PreviewOnly,
    Unsupported,
}

/// 📇️ Every `BrepKernel` trait method name, in trait declaration order — the unit test below
/// asserts [`OPERATION_QUALITY`] covers exactly this set with no duplicates.
pub const BREP_KERNEL_OPERATIONS: &[&str] = &[
    "box_prim", "sphere_prim", "cylinder_prim", "cone_prim", "torus_prim", "convex_hull",
    "line_curve", "circle_curve", "arc_curve", "ellipse_curve", "polyline_wire", "rectangle_wire", "regular_polygon_wire", "interpolate_curve", "approximate_curve", "helix_curve",
    "plane_surface", "planar_face_from_points", "planar_face_from_wire", "nurbs_surface_from_grid", "coons_patch", "offset_face", "thicken_face",
    "extrude_wire", "extrude", "revolve", "loft", "sweep", "pipe", "helical_sweep",
    "fuse", "cut", "intersect", "compound_cut",
    "translate", "rotate", "rotate_about", "scale", "mirror", "copy_shape", "linear_pattern", "circular_pattern", "grid_pattern",
    "fillet", "fillet_variable", "fillet_edges", "chamfer", "chamfer_asymmetric", "chamfer_edges", "shell", "draft", "offset_solid", "defeature",
    "section", "split", "curve_curve_intersect", "curve_surface_intersect", "surface_surface_intersect",
    "curve_point", "curve_tangent", "curve_domain", "curve_curvature", "surface_point", "surface_normal", "curve_closest_parameter", "surface_closest_uv",
    "volume", "area", "length", "center_of_mass", "bounding_box", "distance", "closest_point", "classify_point", "validate",
    "vertex", "face_from_wire", "sew_faces", "heal_solid", "convert_to_nurbs", "deconstruct",
    "export_step", "export_stl", "export_obj", "export_gltf", "import_step", "import_stl", "import_obj", "export_dwg", "import_dwg",
    "kind", "tessellate", "dispose", "retain", "registry_len", "solid_shells", "compound", "explode", "label",
];

/// 📊️ `(method name, quality)` table reflecting the CURRENT engine implementation (audit
/// `📓️explore-engine-handles-primitives.md` §3, ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME):
/// primitives/transforms/booleans/sweeps (excluding the single-profile `extrude` prism case)/
/// blends/offsets/draft/mesh-format IO still tessellate-and-rebuild from a triangle soup
/// (`MeshDerivedBRep`); box/wires/curves/planar-face construction/evaluation/STEP IO/topology
/// bookkeeping stay on the exact analytic supports (`ExactAnalytic`); mass properties and point
/// classification integrate/traverse numerically to within a tolerance
/// (`ExactNumericalWithinTolerance`); curve/surface fitting is least-squares/control-point-only
/// (`ApproximateBRep`). No operation is `Unsupported` today (DWG round-trips through the same
/// triangle-soup bridge as STL/OBJ/glTF).
const OPERATION_QUALITY: &[(&str, OpQuality)] = &[
    ("box_prim", OpQuality::ExactAnalytic),
    ("sphere_prim", OpQuality::MeshDerivedBRep),
    ("cylinder_prim", OpQuality::MeshDerivedBRep),
    ("cone_prim", OpQuality::MeshDerivedBRep),
    ("torus_prim", OpQuality::MeshDerivedBRep),
    ("convex_hull", OpQuality::ExactAnalytic),
    ("line_curve", OpQuality::ExactAnalytic),
    ("circle_curve", OpQuality::ExactAnalytic),
    ("arc_curve", OpQuality::ExactAnalytic),
    ("ellipse_curve", OpQuality::ExactAnalytic),
    ("polyline_wire", OpQuality::ExactAnalytic),
    ("rectangle_wire", OpQuality::ExactAnalytic),
    ("regular_polygon_wire", OpQuality::ExactAnalytic),
    ("interpolate_curve", OpQuality::ApproximateBRep),
    ("approximate_curve", OpQuality::ApproximateBRep),
    ("helix_curve", OpQuality::ExactAnalytic),
    ("plane_surface", OpQuality::ExactAnalytic),
    ("planar_face_from_points", OpQuality::ExactAnalytic),
    ("planar_face_from_wire", OpQuality::ExactAnalytic),
    ("nurbs_surface_from_grid", OpQuality::ApproximateBRep),
    ("coons_patch", OpQuality::ApproximateBRep),
    ("offset_face", OpQuality::ExactNumericalWithinTolerance),
    ("thicken_face", OpQuality::ExactNumericalWithinTolerance),
    // W2-C (26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME): exact per-edge analytic/NURBS sweeps,
    // no triangle soup anywhere — see `➡️sweep/📓️w2c-sweeps.md`. `extrude`/`extrude_wire`/`revolve`
    // build every lateral face from the profile edge's own analytic kind (Plane/Cylinder/Cone/
    // Torus/Sphere) or its exact `to_nurbs()` control net; `loft` skins harmonized NURBS control
    // columns exactly. `sweep`/`pipe`/`helical_sweep` fast-path a straight/circular path onto
    // extrude/revolve exactly, but their general (arbitrary-curvature) path chains adaptively
    // sampled rotation-minimizing-frame stations — bounded-error, not exact.
    ("extrude_wire", OpQuality::ExactAnalytic),
    ("extrude", OpQuality::ExactAnalytic),
    ("revolve", OpQuality::ExactAnalytic),
    ("loft", OpQuality::ExactAnalytic),
    ("sweep", OpQuality::ExactNumericalWithinTolerance),
    ("pipe", OpQuality::ExactNumericalWithinTolerance),
    ("helical_sweep", OpQuality::ExactNumericalWithinTolerance),
    // 🔀️ W2-B (26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME): the exact imprint→classify→select→
    // stitch boolean engine (`diff::boolean::exact_imprint_boolean`, plus the still-exact
    // trivial-topology/box-analytic fast paths ahead of it) is now the default path — the old
    // tessellate→classify→triangle-soup pipeline survives only as the explicit opt-in
    // `boolean_solid_mesh_preview`, which `boolean_solid`/`compound_cut` never call. Not
    // `ExactAnalytic`: the imprint curve's domain is clipped via fixed-resolution sampling +
    // bisection refinement (matching `intersect::surface_surface`'s own `curve_surface_intersect`/
    // `surface_surface_intersect` rating below), not a certified closed-form clip.
    ("fuse", OpQuality::ExactNumericalWithinTolerance),
    ("cut", OpQuality::ExactNumericalWithinTolerance),
    ("intersect", OpQuality::ExactNumericalWithinTolerance),
    ("compound_cut", OpQuality::ExactNumericalWithinTolerance),
    ("translate", OpQuality::ExactAnalytic),
    ("rotate", OpQuality::ExactAnalytic),
    ("rotate_about", OpQuality::ExactAnalytic),
    ("scale", OpQuality::ExactAnalytic),
    ("mirror", OpQuality::ExactAnalytic),
    ("copy_shape", OpQuality::ExactAnalytic),
    ("linear_pattern", OpQuality::MeshDerivedBRep),
    ("circular_pattern", OpQuality::MeshDerivedBRep),
    ("grid_pattern", OpQuality::MeshDerivedBRep),
    ("fillet", OpQuality::ExactNumericalWithinTolerance),
    ("fillet_variable", OpQuality::ExactNumericalWithinTolerance),
    ("fillet_edges", OpQuality::ExactNumericalWithinTolerance),
    ("chamfer", OpQuality::ExactNumericalWithinTolerance),
    ("chamfer_asymmetric", OpQuality::ExactNumericalWithinTolerance),
    ("chamfer_edges", OpQuality::ExactNumericalWithinTolerance),
    ("shell", OpQuality::ExactNumericalWithinTolerance),
    ("draft", OpQuality::ExactNumericalWithinTolerance),
    ("offset_solid", OpQuality::ExactNumericalWithinTolerance),
    ("defeature", OpQuality::MeshDerivedBRep),
    ("section", OpQuality::ExactAnalytic),
    ("split", OpQuality::MeshDerivedBRep),
    ("curve_curve_intersect", OpQuality::ExactNumericalWithinTolerance),
    ("curve_surface_intersect", OpQuality::ExactNumericalWithinTolerance),
    ("surface_surface_intersect", OpQuality::ExactNumericalWithinTolerance),
    ("curve_point", OpQuality::ExactAnalytic),
    ("curve_tangent", OpQuality::ExactAnalytic),
    ("curve_domain", OpQuality::ExactAnalytic),
    ("curve_curvature", OpQuality::ExactAnalytic),
    ("surface_point", OpQuality::ExactAnalytic),
    ("surface_normal", OpQuality::ExactAnalytic),
    ("curve_closest_parameter", OpQuality::ExactNumericalWithinTolerance),
    ("surface_closest_uv", OpQuality::ExactNumericalWithinTolerance),
    ("volume", OpQuality::ExactNumericalWithinTolerance),
    ("area", OpQuality::ExactNumericalWithinTolerance),
    ("length", OpQuality::ExactNumericalWithinTolerance),
    ("center_of_mass", OpQuality::ExactNumericalWithinTolerance),
    ("bounding_box", OpQuality::ExactNumericalWithinTolerance),
    ("distance", OpQuality::ExactNumericalWithinTolerance),
    ("closest_point", OpQuality::ExactNumericalWithinTolerance),
    ("classify_point", OpQuality::ExactNumericalWithinTolerance),
    ("validate", OpQuality::ExactAnalytic),
    ("vertex", OpQuality::ExactAnalytic),
    ("face_from_wire", OpQuality::ExactAnalytic),
    ("sew_faces", OpQuality::ExactAnalytic),
    ("heal_solid", OpQuality::ExactNumericalWithinTolerance),
    ("convert_to_nurbs", OpQuality::ExactAnalytic),
    ("deconstruct", OpQuality::ExactAnalytic),
    ("export_step", OpQuality::ExactAnalytic),
    ("export_stl", OpQuality::MeshDerivedBRep),
    ("export_obj", OpQuality::MeshDerivedBRep),
    ("export_gltf", OpQuality::MeshDerivedBRep),
    ("import_step", OpQuality::ExactAnalytic),
    ("import_stl", OpQuality::MeshDerivedBRep),
    ("import_obj", OpQuality::MeshDerivedBRep),
    ("export_dwg", OpQuality::MeshDerivedBRep),
    ("import_dwg", OpQuality::MeshDerivedBRep),
    ("kind", OpQuality::ExactAnalytic),
    ("tessellate", OpQuality::ExactAnalytic),
    ("dispose", OpQuality::ExactAnalytic),
    ("retain", OpQuality::ExactAnalytic),
    ("registry_len", OpQuality::ExactAnalytic),
    ("solid_shells", OpQuality::ExactAnalytic),
    ("compound", OpQuality::ExactAnalytic),
    ("explode", OpQuality::ExactAnalytic),
    ("label", OpQuality::ExactAnalytic),
];

/// 🔎️ Looks up a `BrepKernel` method's current result fidelity by name; unknown names report
/// [`OpQuality::Unsupported`].
pub fn operation_quality(operation: &str) -> OpQuality {
    OPERATION_QUALITY.iter().find(|(name, _)| *name == operation).map(|(_, quality)| *quality).unwrap_or(OpQuality::Unsupported)
}

#[cfg(test)]
mod tests {
    use super::{operation_quality, OpQuality, BREP_KERNEL_OPERATIONS, OPERATION_QUALITY};
    use std::collections::HashSet;

    #[test]
    fn operation_quality_table_covers_every_brep_kernel_method_with_no_duplicates() {
        let table_names: Vec<&str> = OPERATION_QUALITY.iter().map(|(name, _)| *name).collect();
        let unique_table_names: HashSet<&str> = table_names.iter().copied().collect();
        assert_eq!(table_names.len(), unique_table_names.len(), "OPERATION_QUALITY has duplicate method names");
        for method in BREP_KERNEL_OPERATIONS {
            assert!(unique_table_names.contains(method), "OPERATION_QUALITY is missing BrepKernel method {method:?}");
            assert_ne!(operation_quality(method), OpQuality::Unsupported, "BrepKernel method {method:?} resolved to Unsupported — add it to OPERATION_QUALITY");
        }
        let known_methods: HashSet<&str> = BREP_KERNEL_OPERATIONS.iter().copied().collect();
        for name in &table_names {
            assert!(known_methods.contains(name), "OPERATION_QUALITY names {name:?}, which is not a BrepKernel trait method");
        }
    }

    #[test]
    fn unknown_operation_reports_unsupported() {
        assert_eq!(operation_quality("not_a_real_method"), OpQuality::Unsupported);
    }
}
