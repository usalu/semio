//! 🌐️ First-party Semio geometry session with explicit instance ownership.
use semio_os_flow_mesh::*;
use neural_engine::{Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, VALUE_TYPE_GEOMETRY, VALUE_TYPE_NUMBER, VALUE_TYPE_POINT, VALUE_TYPE_VECTOR};
use semio_framework_value::{ValueType,ValueError,ValueRefusalKind,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::controlled::{ControlledRetirement,RetainedOwnerGate};
use semio_framework_mesh_engine::HistoryFoldIndex;
use {semio_framework_3d_brep_engine::{Brep, BrepKernel, GeometryHandle, GeometryKind}, semio_framework_3d_brep_representation::contract::{ParamDomain, PointClassification, Vec3}};
use semio_framework_3d_brep_tessellation::{TessellationJob, TessellationStep};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::mem::ManuallyDrop;
use std::sync::{Arc,Weak};

#[path = "🧰️operator-registration/🦀️.rs"]
mod operator_registration;
pub use operator_registration::{
    brep_schema, classify_number, domain_span, empty_list_value, geometry_channel, geometry_dict, geometry_schema, kind_label, list_channel, list_indices, map_kernel_error, number_channel, number_dictionary, operator_info_with_outputs, out_area,
    out_box, out_center, out_classification, out_compound, out_curvature, out_curve, out_distance, out_face, out_face_result, out_geometry, out_geometry_result, out_length, out_normal, out_obj, out_point, out_point_result, out_report, out_solid,
    out_span, out_stl, out_surface, out_vertex, out_volume, out_wire, point_channel, point_dictionary, points_to_grid, read_channel_number, read_geometry, read_geometry_list, read_geometry_list_or_empty, read_list, read_nested_point_lists,
    read_optional_geometry, read_point_list, read_text, read_xyz, read_xyz_dict, reg_geo, register_typed, register_untyped, text_dictionary, text_schema, topology_element_schema, topology_list, topology_output, vector_channel, vector_dictionary,
    wire_from_points, deconstruct_geometry,
};
#[path = "⚠️errors/🦀️.rs"]
mod errors;
pub use errors::BrepModuleError;
#[path = "🧊️tessellation/🦀️.rs"]
mod tessellation;
use tessellation::{AuthorityClaims, CachedMesh, FamilyClaims, FamilyRetirement, MeshCache, RetainedTessellation, TessellationCancellation, TessellationJobKey, TessellationJobRegistry, TESSELLATION_JOB_CAPACITY};
pub use tessellation::{PreviewDiagnostic, TessellationStepOutcome};
#[path="🏗️construction/🦀️.rs"]
mod construction;
pub use construction::{SessionConstructionCursor,SessionConstructionStep};
#[path="♻️retention/🦀️.rs"]
mod retained_membership;
use retained_membership::SessionRetention;
#[path="🧭️admission/🦀️.rs"]
mod tessellation_admission;
use tessellation_admission::TessellationAdmission;
#[path = "🌉️generic-invoke/🦀️.rs"]
mod generic_invoke;
use generic_invoke::{arg_bool_or, arg_f64, arg_f64_or, arg_handle, arg_handles, arg_points, arg_usize, arg_vec3, handle_result, invoke_args, mesh_result, number_result, topology_result, unit_result, vec3_result};
#[path = "🧠️session-core/🦀️.rs"]
mod session_core;
use session_core::{controlled, ownership_advance, ownership_busy, ownership_progress, record_shell, settled_poll, settled_step, shell_bytes, spend_event, SessionRetirement, SessionState, ShellAllocator, ShellArc};
pub use session_core::{mesh_payload_bytes, GeometryOperations, Session, SessionCapture};
#[path = "📤️mesh-export/🦀️.rs"]
mod mesh_export;
pub use mesh_export::NativeMeshExport;

/// 📤️ Exports tessellated shapes as one GLB mesh.
pub fn export_glb_via_tessellation(kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepModuleError> {
    semio_framework_3d_brep_engine::export_glb_via_tessellation(kernel, shapes, deflection).map_err(BrepModuleError::from)
}

/// 📥️ Imports a GLB mesh and returns its session handle.
pub fn import_glb_via_tessellation(kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<String>, BrepModuleError> {
    semio_framework_3d_brep_engine::import_glb_via_obj(kernel, bytes, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)
}

/// 🧹️ Original analytic metadata keeps its exact typed owner until independently admitted release.
#[cfg(test)]
#[test]
fn retired_analytic_mesh_metadata_obeys_exact_byte_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let grants=&fixture["retirement"];let count=grants["labels"].as_u64().unwrap() as usize;let label_bytes=grants["labelBytes"].as_u64().unwrap() as usize;
    let mesh=semio_framework::MeshData {component_references:semio_framework_mesh_engine::ComponentReferenceTable::from_entries(vec![(String::from("face"),vec!["x".repeat(label_bytes);count])]),..Default::default()};
    let mut owner=controlled(mesh);let mut released=0;let mut turns=0;
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let depth=owner.next_depth_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:release,maximum_depth:depth};
        if release>8 {assert_eq!(owner.step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap().progress(),RetainedCloneProgress::default());}
        let progress=owner.step(grant).unwrap().progress();assert!(progress.fits(grant));released+=progress.released_bytes;turns+=1;assert!(turns<1000000);
    }
    assert!(released>=count*label_bytes,"metadata labels require admitted physical release");
}

#[cfg(test)]
#[path = "🧪️tests/🛂️physical-custody/🦀️.rs"]
mod physical_session_custody;

#[cfg(test)]
#[path="🧪️tests/⏱️tessellation/🦀️.rs"]
mod original_tessellation_authority_tests;

#[cfg(test)]
#[path = "🧪️tests/🧹️capture-close/🦀️.rs"]
mod original_capture_close_laws;

#[path="🩺️validation-report/🦀️.rs"]
mod validation_report;
pub use semio_framework_3d_brep_engine::{NativeValidationReport, SolidHealing};

#[path = "🧾️topology-reach/🦀️.rs"]
mod topology_reach;
pub use semio_framework_3d_brep_engine::{NativeTopologyReach, ReachedTopology};
