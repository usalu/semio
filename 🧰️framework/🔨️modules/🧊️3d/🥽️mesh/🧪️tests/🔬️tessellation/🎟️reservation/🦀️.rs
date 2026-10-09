use super::*;

/// 🧺️ Quotes and reserves only actual original empty buffers, retaining source and accepted backing through every refusal.
#[test]
fn original_mesh_tessellation_reservation_receipts_preserve_all_original_buffers(){
    use crate::brep::queries::tessellation::tests::observe_tessellation_system as observe;
    use protocol::value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧩️tessellation/🎟️owners.json")).unwrap();
    for copy in law["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let(mut job,source)=observe(||{let positions=serde_json::from_value::<Vec<[f32;3]>>(law["positions"].clone()).unwrap();let faces=serde_json::from_value::<Vec<Vec<u32>>>(law["faces"].clone()).unwrap();MeshTessellationJob::new(HalfedgeMesh::from_faces(&positions,&faces).unwrap())});
        let original=job.mesh.vertices.as_ptr();let(mut born,mut freed,mut turns)=(source.0,source.1,0);let mut pointers=[0usize;16];
        while !job.buffer_reservation_complete(){
            let grant=RetainedCloneGrant {maximum_items:law["normalReservation"]["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_buffer_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:job.next_buffer_release_byte_demand().unwrap(),maximum_depth:job.next_buffer_depth_demand().unwrap()};
            assert_eq!(job.next_buffer_copy_byte_demand().unwrap(),law["normalReservation"]["copyBytes"].as_u64().unwrap()as usize);assert_eq!(grant.maximum_release_bytes,law["normalReservation"]["releaseBytes"].as_u64().unwrap()as usize);
            for denied in [RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant},RetainedCloneGrant {maximum_depth:0,..grant}]{let(step,heap)=observe(||job.reserve_buffer_step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(job.buffer_reservation_progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(job.mesh.vertices.as_ptr(),original);}
            let(step,heap)=observe(||job.reserve_buffer_step(grant).unwrap());assert_eq!(step.progress(),job.buffer_reservation_progress());assert!(step.progress().fits(grant));assert_eq!(step.progress().copied_items,1);assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_eq!(heap.0,grant.maximum_capacity_bytes);assert_eq!(heap.1,0);born+=heap.0;freed+=heap.1;
            let out=job.output.as_ref().unwrap();let current=[job.hes.as_ptr()as usize,job.points.as_ptr()as usize,job.projected.as_ptr()as usize,job.links.as_ptr()as usize,out.positions.as_ptr()as usize,out.normals.as_ptr()as usize,out.colors.as_ptr()as usize,out.indices.as_ptr()as usize,out.uvs.as_ptr()as usize,out.face_ids.as_ptr()as usize,out.vertex_ids.as_ptr()as usize,out.edge_positions.as_ptr()as usize,out.edge_ids.as_ptr()as usize,out.edge_uvs.as_ptr()as usize,out.edge_is_seam.as_ptr()as usize,job.corner_ids.as_ptr()as usize];for index in 0..turns{assert_eq!(current[index],pointers[index]);}pointers[turns]=current[turns];turns+=1;assert_eq!(job.mesh.vertices.as_ptr(),original);
        }
        assert_eq!(turns,law["normalReservation"]["buffers"].as_array().unwrap().len());let(_,heap)=observe(||{job.step(super::tessellation_metadata_tests::funded_normal_grant(&job)).unwrap();job.step(super::tessellation_metadata_tests::funded_normal_grant(&job)).unwrap();});assert_eq!(heap,(0,0));
        let(mut owner,handoff)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original reserved tessellation closure unsupported")));assert_eq!(handoff,(0,0));let mut close_turns=0;
        while !owner.terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(grant));born+=heap.0;freed+=heap.1;close_turns+=1;assert!(close_turns<100000);}
        assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));eprintln!("[DEBUG] Original Mesh buffer reservation copy={copy} buffers={turns} sameSource=true sameBacking=true physical={freed} terminalDrop=0");
    }
}
