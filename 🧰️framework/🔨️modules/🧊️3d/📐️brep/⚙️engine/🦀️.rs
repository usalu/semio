//! ⚙️ First-party boundary-representation engine of the 3D kernel: the arena-backed `Brep` API over the representation,
//! query and operation crates, with bounded operation jobs, shape values, mesh import, retention and retirement.
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-13-wave-e4-3d-architect-bim.md`). The handle and error vocabulary with the
//! `BrepKernel` trait, the mesh exchange and the standalone shape value live in `semio-framework-3d-brep-kernel-contract`,
//! `-mesh-io` and `-shape-value`; this crate re-exports exactly the items its clients name.

use std::collections::{HashMap};
#[cfg(test)] use std::collections::{BTreeMap};

pub use semio_framework_3d_brep_mesh_io::{mesh_data_from_mesh_transfer, MeshImportCursor, MeshImportStep};

use semio_framework_3d_brep_representation::contract::*;

#[path = "🚚️shape-exchange/🦀️.rs"]
mod shape_exchange;
pub use semio_framework_3d_brep_shape_value::{ImportedShape, ShapeComponent, ShapeRoot, ShapeTessellationJob, ShapeValue, ShapeWire};
pub use semio_framework_3d_brep_kernel_contract::{BrepError, BrepKernel, BrepTopology, ClosestPoint, GeometryHandle, GeometryKind, SolidFaceLoops};
use semio_framework_3d_brep_kernel_contract::map_err;

use semio_framework_3d_brep_boolean::{boolean_job, boolean_solid, BooleanAdmission, BooleanJob, BooleanOp, BooleanProgress, BooleanStep};
use semio_framework_3d_brep_construction::euler::make_vertex;
use semio_framework_3d_brep_intersect::intersect_curve_curve;
use semio_framework_3d_brep_intersect::intersect_curve_surface;
use semio_framework_3d_brep_intersect::intersect_surface_surface;
use {semio_framework_3d_brep_construction::primitives::{make_box, make_cone, make_cylinder, make_planar_face_from_points, make_planar_face_from_wire, make_polyline_wire, make_rectangle_wire, make_regular_polygon_wire, make_sphere, make_torus}, semio_framework_3d_brep_representation::topology::Wire};
use semio_framework_3d_brep_construction::sew::{convert_to_nurbs, heal_solid, sew_faces};
use semio_framework_3d_brep_construction::transform::{copy_solid, transform_face, transform_solid, transform_wire};
use semio_framework_3d_brep_queries::classification::point_in_solid;
use semio_framework_3d_brep_queries::mass_properties::{closest_point_on_face, closest_point_on_solid, distance_solid_solid, edge_length, face_area, solid_bounding_box, solid_center_of_mass, solid_surface_area, solid_volume};
use semio_framework_3d_brep_tessellation::TessellationJob;
#[cfg(test)] use semio_framework_3d_brep_tessellation::tessellate_solid;
use semio_framework_3d_brep_validation::{issue_reaches, validate_body};
use semio_framework_3d_brep_representation::arena::{ArenaId, EdgeId, FaceId, SolidId, VertexId};
use {semio_framework_3d_brep_curve::curve_ops::{approximate_curve_with_count, closest_parameter as curve_closest_parameter_fn, interpolate_curve, ParamMethod}, semio_framework_3d_brep_surface::surface_ops::{coons_patch_nurbs, interpolate_surface_grid}};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::error::{KernelError, ValidationIssue};
use semio_framework_3d_brep_surface::surface_ops::closest_uv as surface_closest_uv_fn;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_vector::matrix::{Affine3, Frame3};
use semio_framework_3d_brep_vector::{Pnt3, Vec3 as NativeVec3};
use semio_framework_3d_brep_representation::contract::Vec3 as EVec3;
use semio_framework_3d_brep_mesh_io::{export_solid_glb, export_solid_mesh, export_solid_obj, export_solid_stl, import_glb_to_body, import_mesh_to_body, import_obj_to_body, import_stl_to_body};

#[path = "🧹️retirement/🦀️.rs"]
pub mod retirement;

// #region 🔖️Types

use semio_framework_3d_brep_representation::arena::ShellId;
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_3d_brep_representation::topology::EntityRef;

#[path = "🧠️brep-session-state/🦀️.rs"]
mod brep_session_state;
use brep_session_state::Entity;
pub use brep_session_state::{Brep, BrepBooleanAdmission, BrepBooleanJob, BrepBooleanStep};

#[path = "🧾️conversions-and-registry/🦀️.rs"]
mod conversions_and_registry;
use conversions_and_registry::{entity_roots, entity_tag, evec, label_of_entity, pnt, require_direction, require_finite_scalar, require_finite_vector, require_positive_size, require_scale_factor, vec3};

#[path = "🧱️primitive-and-curve-constructors/🦀️.rs"]
mod primitive_and_curve_constructors;

#[path = "🔀️boolean-transform-and-copy/🦀️.rs"]
mod boolean_transform_and_copy;

#[path = "🔍️intersection-evaluation-and-measure/🦀️.rs"]
mod intersection_evaluation_and_measure;

#[path = "🩺️healing-exchange-and-tessellation/🦀️.rs"]
mod healing_exchange_and_tessellation;

#[path = "🏷️validation-gate-and-labels/🦀️.rs"]
mod validation_gate_and_labels;
use validation_gate_and_labels::all_edges;

#[path = "🔌️brep-kernel-implementation/🦀️.rs"]
mod brep_kernel_implementation;

#[path = "💾️solid-exchange-codecs/🦀️.rs"]
mod solid_exchange_codecs;
pub use solid_exchange_codecs::{GlbSolidExporter, GlbSolidImporter, ObjSolidExporter, ObjSolidImporter, SolidExporter, SolidImporter, StlSolidExporter, StlSolidImporter};

#[path="♻️retention/🦀️.rs"]
mod retention;
pub use retention::BrepRetentionJob;
#[cfg(test)]
#[path="🧪️tests/♻️retention/🦀️.rs"]
mod retention_tests;

// #endregion 🔖️Types

// #region 🧮Convert

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

// #endregion 🧮Convert

// #region 🔖️OperationJobs
#[path = "⏱️operation-jobs/🦀️.rs"]
mod operation_jobs;
pub use operation_jobs::{BrepOperation, BrepOperationAdmission, BrepOperationJob, BrepOperationProgress, BrepOperationStep};
// #endregion 🔖️OperationJobs

// #region 🔖️AnalysisSession
#[path = "🔎️analysis-session/🦀️.rs"]
mod analysis_session;
// #endregion 🔖️AnalysisSession

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧪️TestSupport
#[cfg(test)]
pub(crate) use semio_framework_3d_testing_support as test_support;
//#endregion 🧪️TestSupport
