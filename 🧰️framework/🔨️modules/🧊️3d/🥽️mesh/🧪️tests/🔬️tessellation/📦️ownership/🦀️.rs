//! 📦️ Ownership: 2 tests covering original mesh tessellation cancellation retains exact output until funded closure; original mesh tessellation source collection requires original buffer reservation.
use super::*;

/// 🧹️ Keeps the original output and source through a zero-effect cancellation request and funded physical closure.
#[test]
fn original_mesh_tessellation_cancellation_retains_exact_output_until_funded_closure(){
    use crate::test_support::observe_tessellation_system as observe;
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=crate::fixture_json("🧪️tests/🧫️fixtures/🧩️tessellation/🎟️owners.json");
    for copy in law["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize){
        let(mut job,source)=observe(||{let positions=serde_json::from_value::<Vec<[f32;3]>>(law["positions"].clone()).unwrap();let faces=serde_json::from_value::<Vec<Vec<u32>>>(law["faces"].clone()).unwrap();let mut job=MeshTessellationJob::new(HalfedgeMesh::from_faces(&positions,&faces).unwrap());while job.output.as_ref().unwrap().positions.is_empty(){assert!(matches!(job.step(super::tessellation_metadata_tests::funded_normal_grant(&job)).unwrap().0,MeshTessellationStep::Working(_)));}job});
        let output=job.output.as_ref().unwrap().positions.as_ptr();let original=job.mesh.vertices.as_ptr();let progress=job.progress();
        let(_,signal)=observe(||job.cancel());assert_eq!(signal,(law["cancellationCapacityBytes"].as_u64().unwrap() as usize,law["cancellationReleaseBytes"].as_u64().unwrap() as usize));assert_eq!(job.output.as_ref().unwrap().positions.as_ptr(),output);assert_eq!(job.mesh.vertices.as_ptr(),original);
        let(step,heap)=observe(||job.step(super::tessellation_metadata_tests::funded_normal_grant(&job)).unwrap().0);assert!(matches!(step,MeshTessellationStep::Cancelled(value) if value==progress));assert_eq!(heap,(0,0));
        let(mut owner,handoff)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original mesh tessellation owner unsupported")));assert_eq!(handoff,(0,0));let(mut born,mut freed,mut turns)=(0,0,0);
        while !owner.terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:law["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert_eq!(step.progress(),owner.step_progress());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);}
        assert_eq!(source.0-source.1+born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,law["terminalDropBytes"].as_u64().unwrap() as usize));
    }
}

/// 🧺️ Original source collection uses existing reserved scratch rather than growing an unadmitted Vec.
#[test]
fn original_mesh_tessellation_source_collection_requires_original_buffer_reservation(){
    use crate::test_support::observe_tessellation_system as observe;
    let law:serde_json::Value=crate::fixture_json("🧪️tests/🧫️fixtures/🧩️tessellation/🎟️owners.json");
    let(mut job,source)=observe(||{let positions=serde_json::from_value::<Vec<[f32;3]>>(law["positions"].clone()).unwrap();let faces=serde_json::from_value::<Vec<Vec<u32>>>(law["faces"].clone()).unwrap();MeshTessellationJob::new(HalfedgeMesh::from_faces(&positions,&faces).unwrap())});
    let original=job.mesh.vertices.as_ptr();
    let(mut reservation_born,mut reservation_freed)=(0,0);
    while !job.buffer_reservation_complete(){let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:job.next_buffer_capacity_byte_demand(0).unwrap(),maximum_release_bytes:0,maximum_depth:job.next_buffer_depth_demand().unwrap()};let(step,heap)=observe(||job.reserve_buffer_step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));reservation_born+=heap.0;reservation_freed+=heap.1;}
    let(_,heap)=observe(||{job.step(super::tessellation_metadata_tests::funded_normal_grant(&job)).unwrap();job.step(super::tessellation_metadata_tests::funded_normal_grant(&job)).unwrap();});
    assert_eq!(heap,(0,0),"original source collection requires previously admitted backing");
    assert_eq!(job.mesh.vertices.as_ptr(),original);
    let(mut owner,handoff)=observe(||semio_framework_value::retirement::controlled::ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original tessellation closure unsupported")));assert_eq!(handoff,(0,0));let(mut born,mut freed)=(source.0+reservation_born,source.1+reservation_freed);
    while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
    assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
}
