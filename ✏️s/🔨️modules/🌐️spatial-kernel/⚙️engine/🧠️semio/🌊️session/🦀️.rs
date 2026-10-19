//! 🌐️ First-party Semio geometry session with explicit instance ownership.
use semio_framework_os_flow::mesh::*;
use neural_engine::{Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, VALUE_TYPE_GEOMETRY, VALUE_TYPE_NUMBER, VALUE_TYPE_POINT, VALUE_TYPE_VECTOR};
use semio_framework_value::{ValueType,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::controlled::{ControlledRetirement,RetainedOwnerGate};
use semio_framework_mesh_engine::HistoryFoldIndex;
use {semio_framework_3d_brep_engine::{Brep, BrepKernel, GeometryHandle, GeometryKind}, semio_framework_3d_brep_representation::contract::{ParamDomain, PointClassification, Vec3}};
use semio_framework_3d_brep_tessellation::{TessellationJob, TessellationStep};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::mem::ManuallyDrop;
use std::sync::{Arc,Weak};

// 🔀️ dedyn-fw-os-misc, O1/R11 case 3: `BrepKernel` has exactly one impl (`Brep`, in `🗄️stdio`) —
// every `dyn BrepKernel` call site was already handing this module a concrete `Brep`, so the trait
// object was a no-op coercion, not a real seam. Deleting it also clears an existing O1 violation:
// every `BrepKernel` method is `async fn`, which is not dyn-compatible (E0038) — `dyn BrepKernel`
// could not have compiled as-is.

#[path = "🧰️operator-registration/🦀️.rs"]
mod operator_registration;
pub use operator_registration::{
    brep_schema, classify_number, domain_span, empty_list_value, geometry_channel, geometry_dict, geometry_schema, kind_label, list_channel, list_indices, map_kernel_error, number_channel, number_dictionary, operator_info_with_outputs, out_area,
    out_box, out_center, out_classification, out_compound, out_curvature, out_curve, out_distance, out_face, out_face_result, out_geometry, out_geometry_result, out_length, out_normal, out_obj, out_point, out_point_result, out_report, out_solid,
    out_span, out_stl, out_surface, out_vertex, out_volume, out_wire, point_channel, point_dictionary, points_to_grid, read_channel_number, read_geometry, read_geometry_list, read_geometry_list_or_empty, read_list, read_nested_point_lists,
    read_optional_geometry, read_point_list, read_text, read_xyz, read_xyz_dict, reg_geo, register_typed, register_untyped, text_dictionary, text_schema, topology_element_schema, topology_list, topology_output, vector_channel, vector_dictionary,
    wire_from_points, BrepDeconstruct,
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
#[path = "📤️media-export/🦀️.rs"]
mod media_export;
pub use media_export::{export_glb_via_tessellation, import_glb_via_tessellation};
#[path = "🌉️generic-invoke/🦀️.rs"]
mod generic_invoke;
use generic_invoke::{arg_bool_or, arg_f64, arg_f64_or, arg_handle, arg_handles, arg_points, arg_usize, arg_vec3, handle_result, invoke_args, mesh_result, number_result, topology_result, unit_result, vec3_result};
#[path = "🧠️session-core/🦀️.rs"]
mod session_core;
use session_core::{controlled, ownership_advance, ownership_busy, ownership_progress, record_shell, shell_bytes, SessionRetirement, SessionState, ShellAllocator, ShellArc};
pub use session_core::{mesh_payload_bytes, GeometryOperations, Session, SessionCapture};

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
mod physical_session_custody {
    use super::*;
    #[global_allocator]
    static SYSTEM:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
    pub(super) fn observe<T>(work:impl FnOnce()->T)->(T,(usize,usize)) {let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
    #[test]
    fn original_session_capture_keeps_same_allocation_until_full_typed_grants() {
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧪️tests/🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
        let row=&fixture["box"];let custody=&fixture["physicalCustody"];
        let identity=semio_framework_os_flow::geometry::GeometryWorkIdentity{operation:61,generation:1};
        let mut caller=super::original_tessellation_authority_tests::original_geometry_caller(identity);
        let stock=super::original_tessellation_authority_tests::original_geometry_source_grant();
        let (mut capture,factory)=observe(|| {
            let source=Session::new();let handle=source.with_kernel(|kernel|kernel.box_prim_sync(row["width"].as_f64().unwrap(),row["depth"].as_f64().unwrap(),row["height"].as_f64().unwrap()).map_err(|error|map_kernel_error(&error))).unwrap();
            let probe=caller.turn(stock,|cx|{let mut result=None;cx.with_original_work_loan(0,cx.deadline_us(),|same|{result=Some(source.tessellate_step(identity,handle.as_str(),0.05,same));Ok(())})?;Ok(result.unwrap())}).unwrap().unwrap();
            assert!(matches!(probe.0,semio_framework_os_flow::geometry::GeometryStep::Working {..}));
            let mut warmed=false;
            for _ in 0..custody["maximumWarmTurns"].as_u64().unwrap() {
                let(step,_)=caller.turn(stock,|cx|Ok(source.tessellate_step(identity,handle.as_str(),0.05,cx))).unwrap().unwrap();
                match step {semio_framework_os_flow::geometry::GeometryStep::Working {..}=>{let owner=source.tessellation_jobs().try_lock().unwrap();let registry=owner.original().unwrap();let faces=registry.jobs.iter().find(|(key,_)|key.0==handle.0&&key.1==0.05f64.to_bits()).map_or(0,|(_,row)|row.job.progress().faces_done);if faces>=custody["warmFaces"].as_u64().unwrap()as usize {warmed=true;break}},other=>panic!("original partial tessellation did not remain owned: {other:?}")}
            }
            assert!(warmed,"original job must retain sampled edges and a partial mesh");
            let capture=source.capture();drop(handle);drop(source);capture
        });
        let retained=factory.0.checked_sub(factory.1).unwrap();let original=Arc::as_ptr(&capture.session.as_ref().unwrap().state);
        let (count,cancel)=observe(||capture.cancel_all_tessellations());assert_eq!(count,1);
        assert_eq!(cancel,(custody["cancelCapacityBytes"].as_u64().unwrap() as usize,custody["cancelReleaseBytes"].as_u64().unwrap() as usize));
        let handoff=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
        let (step,heap)=observe(||capture.close_step(handoff).unwrap());assert!(step.progress().fits(handoff));
        assert_eq!(heap,(custody["handoffCapacityBytes"].as_u64().unwrap() as usize,custody["handoffReleaseBytes"].as_u64().unwrap() as usize));
        assert_eq!(capture.allocation.as_ref().unwrap().as_ptr(),original);
        let (mut born,mut freed,mut refusals,mut turns,mut final_shell)=(0,0,0,0,0);
        while !capture.terminal_is_empty() {
            let copy=capture.next_close_copy_byte_demand().unwrap();let capacity=capture.next_close_capacity_byte_demand(copy).unwrap();let release=capture.next_close_release_byte_demand().unwrap();let depth=capture.next_close_depth_demand().unwrap();
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
            if release>custody["insufficientReleaseBytes"].as_u64().unwrap() as usize {
                for _ in 0..2 {let (step,heap)=observe(||capture.close_step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(capture.next_close_release_byte_demand().unwrap(),release);refusals+=1;}
            }
            let parent=capture.orphan.is_none() && capture.allocation.is_some();
            let (step,heap)=observe(||capture.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
            if parent && capture.terminal_is_empty() {assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(heap.1,release);final_shell+=1;}
        }
        assert_eq!(retained+born,freed);assert_eq!(final_shell,1);assert!(refusals>0);
        caller.retire(stock);
    }
}

#[cfg(test)]
#[path="🧪️tests/⏱️tessellation/🦀️.rs"]
mod original_tessellation_authority_tests;
