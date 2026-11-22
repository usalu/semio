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

#[path="🧭️revolution-distance/🦀️.rs"]
mod revolution_distance;
pub use revolution_distance::frustum_signed_distance;

#[path="✂️axial-section/🦀️.rs"]
mod axial_section;
pub use axial_section::AxialPartition;

#[path="✂️clipped-box/🦀️.rs"]
mod clipped_box;
pub use clipped_box::{ClippedBox,ClippedBoxBoundary};

#[path="✂️box-section/🦀️.rs"]
mod box_section;
pub use box_section::{BoxSectionDraft,SectionPolygon};

#[path="📐️pattern-placement/🦀️.rs"]
mod pattern_placement;
pub use pattern_placement::SolidPatternRecipe;

#[path="🪟️perforated-prism/🦀️.rs"]
mod perforated_prism;
pub use perforated_prism::{PerforatedPrism,PERFORATED_RING_SIZES};

#[path="📐️footprint/🦀️.rs"]
mod footprint;
pub use footprint::Footprint;

#[path="🔺️polygon-region/🦀️.rs"]
mod polygon_region;
pub use polygon_region::{PolygonRegion,RectangleOperation};

#[path="📐️box-finish-layout/🦀️.rs"]
pub mod box_finish_layout;
#[path="📦️rounded-box/🦀️.rs"]
mod rounded_box;
pub use rounded_box::{RoundedBox, quadrant as rounded_box_quadrant};

#[path="🍩️torus-sector/🦀️.rs"]
mod torus_sector;
pub use torus_sector::TorusSector;
#[path="🧿️annular-revolution/🦀️.rs"]
mod annular_revolution;
pub use annular_revolution::AnnularRevolution;

#[path="🔵️spherical-shell/🦀️.rs"]
mod spherical_shell;
pub use spherical_shell::{SphericalShell,SphericalShellBoundary};

#[path="🌗️spherical-caps/🦀️.rs"]
mod spherical_caps;
pub use spherical_caps::SphericalCaps;

/// 📐️ Column vector `[x,y,z]`.
pub type Vec3 = [f64; 3];

/// 📦️ Axis-aligned bounds.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// 📏️ Parametric domain `[min, max]`.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct ParamDomain {
    pub min: f64,
    pub max: f64,
}

/// 🧩️ Triangle index range for one B-Rep face.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct FaceGroup {
    pub start: u32,
    pub count: u32,
    pub entity_id: String,
}

/// 🧵️ Line-segment index range for one B-Rep edge.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct EdgeGroup {
    pub start: u32,
    pub count: u32,
    pub entity_id: String,
}

/// 📍️ Original topology vertex range in the preview point buffer.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct VertexGroup {
    pub start: u32,
    pub count: u32,
    pub entity_id: String,
}

/// 🏄️ Analytic surface family behind a tessellated face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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
#[value(crate = "::semio_framework_value")]
#[value(rename_all = "camelCase")]
pub enum CurveKind {
    Line,
    Circle,
    Ellipse,
    Nurbs,
}

/// 🖼️ Per-face metadata alongside a tessellated face group.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct FaceInfo {
    pub entity_id: String,
    pub surface_kind: SurfaceKind,
    pub area: f64,
    pub normal: Vec3,
}

/// 🖇️ Per-edge metadata alongside a tessellated edge group.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct EdgeInfo {
    pub entity_id: String,
    pub curve_kind: CurveKind,
    pub length: f64,
}

/// 🖼️ Tessellated mesh payload for preview upload.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct MeshTransfer {
    pub position: Vec<f32>,
    pub normal: Vec<f32>,
    pub index: Vec<u32>,
    pub edges: Vec<f32>,
    #[value(default)]
    pub points: Vec<f32>,
    #[value(default)]
    pub vertex_groups: Vec<VertexGroup>,
    pub face_groups: Vec<FaceGroup>,
    #[value(default)]
    pub edge_groups: Vec<EdgeGroup>,
    #[value(default)]
    pub face_infos: Vec<FaceInfo>,
    #[value(default)]
    pub edge_infos: Vec<EdgeInfo>,
}

semio_framework_value::artifact_retire_leaf!(SurfaceKind, CurveKind);
semio_framework_value::artifact_retire_struct!(Aabb { min, max });
semio_framework_value::artifact_retire_struct!(FaceGroup { start, count, entity_id });
semio_framework_value::artifact_retire_struct!(EdgeGroup { start, count, entity_id });
semio_framework_value::artifact_retire_struct!(VertexGroup { start, count, entity_id });
semio_framework_value::artifact_retire_struct!(FaceInfo { entity_id, surface_kind, area, normal });
semio_framework_value::artifact_retire_struct!(EdgeInfo { entity_id, curve_kind, length });
semio_framework_value::artifact_retire_struct!(MeshTransfer { position, normal, index, edges, points, vertex_groups, face_groups, edge_groups, face_infos, edge_infos });

/// 🪪️ Immutable native bore dimensions and labels from its original publication.
#[derive(Clone, Copy)]
pub struct BorePreviewSource {
    pub origin: [f64; 3],
    pub dimensions: [f64; 3],
    pub center: [f64; 2],
    pub bore_span: [f64; 2],
    pub radius: f64,
    pub first_label: u64,
    pub volume: f64,
}

impl BorePreviewSource {
    pub const BOTTOM_RING_EDGE: usize = 12;
    pub const TOP_RING_EDGE: usize = 13;
    pub const CYLINDER_SEAM_EDGE: usize = 14;
    pub const BORE_WALL_FACE: usize = 6;


    pub fn is_open(&self, top:bool)->bool {
        self.bore_span[usize::from(top)]==self.origin[2]+if top{self.dimensions[2]}else{0.0}
    }

    pub fn cap_face(&self,top:bool)->usize {
        if self.is_open(top){usize::from(top)}
        else{7+usize::from(top&&!self.is_open(false))}
    }

    pub fn closed_bore(&self)->bool{!self.is_open(false)&&!self.is_open(true)}
    pub fn face_count(&self)->usize{7+usize::from(!self.is_open(false))+usize::from(!self.is_open(true))}

    /// 📍️ Measures the signed distance to exterior planes, bore wall and any interior end disks.
    pub fn signed_boundary_distance(&self,[x,y,z]:[f64;3])->f64{
        let [x0,y0,z0]=self.origin;
        let [width,depth,height]=self.dimensions;
        let [x1,y1,z1]=[x0+width,y0+depth,z0+height];
        let [lo,hi]=self.bore_span;
        let gap=|p:f64,min:f64,max:f64|(min-p).max(p-max).max(0.0);
        let [dx,dy,dz]=[gap(x,x0,x1),gap(y,y0,y1),gap(z,z0,z1)];
        let xy=dx.hypot(dy);
        let radial=(x-self.center[0]).hypot(y-self.center[1]);
        let radial_gap=radial-self.radius;
        let sides=(x-x0).abs().min((x-x1).abs()).hypot(dy).hypot(dz)
            .min((y-y0).abs().min((y-y1).abs()).hypot(dx).hypot(dz));
        let wall=radial_gap.hypot(gap(z,lo,hi));
        let mut distance=sides.min(wall);
        for (top,exterior,interior) in [(false,z0,lo),(true,z1,hi)]{
            let opening=self.is_open(top);
            let cap_xy=if opening{xy.max(-radial_gap)}else{xy};
            distance=distance.min((z-exterior).hypot(cap_xy));
            if !opening{distance=distance.min((z-interior).hypot(radial_gap.max(0.0)));}
        }
        let void=radial_gap<0.0&&(lo..=hi).contains(&z);
        if dx==0.0&&dy==0.0&&dz==0.0&&!void{-distance}else{distance}
    }

    pub fn vertex_label(self, index: usize) -> u64 { self.first_label + index as u64 }
    pub fn edge_label(self, index: usize) -> u64 { self.first_label + 10 + index as u64 }
    pub fn face_label(self, index: usize) -> u64 { self.first_label + 25 + index as u64 }
}

/// 📍️ Point classification relative to a solid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
#[value(rename_all = "camelCase")]
pub enum PointClassification {
    Inside,
    Outside,
    OnBoundary,
}

/// 🎯️ Fidelity of a `BrepKernel` operation's result relative to true analytic B-Rep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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
    "box_prim",
    "sphere_prim",
    "cylinder_prim",
    "cone_prim",
    "torus_prim",
    "convex_hull",
    "line_curve",
    "circle_curve",
    "arc_curve",
    "ellipse_curve",
    "polyline_wire",
    "rectangle_wire",
    "regular_polygon_wire",
    "interpolate_curve",
    "approximate_curve",
    "helix_curve",
    "plane_surface",
    "planar_face_from_points",
    "planar_face_from_wire",
    "nurbs_surface_from_grid",
    "coons_patch",
    "offset_face",
    "thicken_face",
    "extrude_wire",
    "extrude",
    "revolve",
    "loft",
    "sweep",
    "pipe",
    "helical_sweep",
    "fuse",
    "cut",
    "intersect",
    "compound_cut",
    "translate",
    "rotate",
    "rotate_about",
    "scale",
    "scale_axes",
    "mirror",
    "copy_shape",
    "linear_pattern",
    "circular_pattern",
    "grid_pattern",
    "fillet",
    "fillet_variable",
    "fillet_edges",
    "chamfer",
    "chamfer_asymmetric",
    "chamfer_edges",
    "shell",
    "draft",
    "offset_solid",
    "defeature",
    "section",
    "split",
    "curve_curve_intersect",
    "curve_surface_intersect",
    "surface_surface_intersect",
    "curve_point",
    "curve_tangent",
    "curve_domain",
    "curve_curvature",
    "surface_point",
    "surface_normal",
    "curve_closest_parameter",
    "surface_closest_uv",
    "volume",
    "area",
    "length",
    "center_of_mass",
    "bounding_box",
    "distance",
    "closest_point",
    "classify_point",
    "validate",
    "vertex",
    "face_from_wire",
    "sew_faces",
    "heal_solid",
    "convert_to_nurbs",
    "deconstruct",
    "export_stl",
    "export_obj",
    "export_gltf",
    "import_stl",
    "import_obj",
    "export_mesh",
    "import_mesh",
    "kind",
    "tessellate",
    "dispose",
    "retain",
    "registry_len",
    "solid_shells",
    "compound",
    "explode",
    "label",
];

/// 📊️ `(method name, quality)` table reflecting the CURRENT engine implementation (audit
/// `📓️explore-engine-handles-primitives.md` §3, ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME):
/// patterns/split/mesh-format IO still tessellate-and-rebuild from a triangle soup
/// (`MeshDerivedBRep`); primitives/wires/curves/planar-face construction/evaluation/STEP IO/
/// topology bookkeeping stay on the exact analytic supports (`ExactAnalytic`); mass properties,
/// point classification, booleans, sweeps with an arbitrary-curvature path, blends and offsets
/// integrate/traverse/clip numerically to within a tolerance (`ExactNumericalWithinTolerance`);
/// curve/surface fitting is least-squares/control-point-only (`ApproximateBRep`). No operation is
/// `Unsupported` today (DWG round-trips through the same triangle-soup bridge as STL/OBJ/glTF).
const OPERATION_QUALITY: &[(&str, OpQuality)] = &[
    ("box_prim", OpQuality::ExactAnalytic),
    // 🧱 W1-E (26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME) rewrote all four of these onto their
    // own analytic support — one `Surface::Sphere`/`Cylinder`/`Cone`/`Torus` face (plus exact
    // planar caps) with hand-derived exact p-curves, no tessellation anywhere on the path — so the
    // `MeshDerivedBRep` tag they kept until 26/09/09 described a pipeline that no longer exists.
    ("sphere_prim", OpQuality::ExactAnalytic),
    ("cylinder_prim", OpQuality::ExactAnalytic),
    ("cone_prim", OpQuality::ExactAnalytic),
    ("torus_prim", OpQuality::ExactAnalytic),
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
    ("scale_axes", OpQuality::ExactNumericalWithinTolerance),
    ("mirror", OpQuality::ExactAnalytic),
    ("copy_shape", OpQuality::ExactAnalytic),
    ("linear_pattern", OpQuality::MeshDerivedBRep),
    ("circular_pattern", OpQuality::MeshDerivedBRep),
    ("grid_pattern", OpQuality::MeshDerivedBRep),
    // 🎨️ 26/09/09/PROCEDURAL-3D-END-TO-END: `diff::blend` carries every constant-radius patch on
    // its own ANALYTIC support — a `Cylinder` between two planes, a `Torus` at a cylinder's cap,
    // a `Sphere` octant at a fully blended corner, a `Plane` or `Cone` frustum for a chamfer — and
    // every one of their trims is an ISO-LINE whose p-curve is derived in closed form and verified
    // against its own 3D curve. No sampling, no fit, no tolerance anywhere in the construction,
    // and the rounded-box Minkowski closed form comes out to `1e-6` relative
    // (`[[test]] brep_analytic_blend`, cross-checked by `parry3d`). `fillet_variable` is the one
    // exception and stays a rung lower: its cone's oblique cap trim is a conic whose image in the
    // cone's own `(u, v)` is transcendental, so that ONE p-curve is interpolated and certified to
    // `1e-8` rather than derived.
    ("fillet", OpQuality::ExactAnalytic),
    ("fillet_variable", OpQuality::ExactNumericalWithinTolerance),
    ("fillet_edges", OpQuality::ExactAnalytic),
    ("chamfer", OpQuality::ExactAnalytic),
    ("chamfer_asymmetric", OpQuality::ExactAnalytic),
    ("chamfer_edges", OpQuality::ExactAnalytic),
    ("shell", OpQuality::ExactNumericalWithinTolerance),
    ("draft", OpQuality::ExactNumericalWithinTolerance),
    ("offset_solid", OpQuality::ExactNumericalWithinTolerance),
    ("defeature", OpQuality::ExactAnalytic),
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
    ("export_stl", OpQuality::MeshDerivedBRep),
    ("export_obj", OpQuality::MeshDerivedBRep),
    ("export_gltf", OpQuality::MeshDerivedBRep),
    ("import_stl", OpQuality::MeshDerivedBRep),
    ("import_obj", OpQuality::MeshDerivedBRep),
    ("export_mesh", OpQuality::MeshDerivedBRep),
    ("import_mesh", OpQuality::MeshDerivedBRep),
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
    OPERATION_QUALITY.iter().find(|(name, _)| *name == operation).map_or(OpQuality::Unsupported, |(_, quality)| *quality)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
