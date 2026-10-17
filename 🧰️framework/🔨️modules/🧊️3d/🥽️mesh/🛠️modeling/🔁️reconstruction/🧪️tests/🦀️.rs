//! 🧪️ Reconstruction: 4 tests covering retained weld grid matches shared vectors; direct import and coplanar job consume the same reconstruction owner; remap configuration is closed after actual advance; and more.
use super::*;

#[test]
fn retained_weld_grid_matches_shared_vectors() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in rows.as_array().unwrap(){let point=serde_json::from_value::<[f32;3]>(row["point"].clone()).unwrap();let result=mesh_weld_key(point,row["precision"].as_f64().unwrap()as f32);
        if row["expected"].is_null(){assert!(result.is_err());}else{let(a,b,c)=result.unwrap();assert_eq!(vec![a.to_string(),b.to_string(),c.to_string()],serde_json::from_value::<Vec<String>>(row["expected"].clone()).unwrap());}
    }
}

#[test]
fn direct_import_and_coplanar_job_consume_the_same_reconstruction_owner(){
    let(mut import,_)=MeshImportJob::indexed(vec![0.,0.,0.,1.,0.,0.,1.,1.,0.,0.,1.,0.],vec![0,1,2,0,2,3],Vec::new()).unwrap();
    let mut source=None;for _ in 0..10000{if let Some(mesh)=import.advance().unwrap(){source=Some(mesh);break;}}
    assert!(import.advance().is_err());
    let mut merge=MeshMergeJob::new(source.expect("bounded import"),MeshMergeIntent::Coplanar).unwrap();let mut output=None;assert!(merge.transfer_remaining_assets(&mut HalfedgeMesh::empty()).is_err());
    for _ in 0..10000{
        if let Some(snapshot)=merge.snapshot_frontier_mut(){if !snapshot.complete(){let demand=snapshot.preparation_demands().unwrap();snapshot.advance(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth},&mut RetainedCloneProgress::default()).unwrap();continue;}}
        if let Some(mesh)=merge.advance().unwrap(){output=Some(mesh);break;}
    }let mesh=output.expect("bounded merge");assert_eq!(mesh.vertex_count(),4);assert_eq!(mesh.face_count(),1);assert!(merge.transfer_remaining_assets(&mut HalfedgeMesh::empty()).is_ok());assert_eq!(merge.coplanar_merge_count(),Some(1));assert!(merge.advance().is_err());
}

#[test]
fn remap_configuration_is_closed_after_actual_advance(){
    let mut remap=MeshSourceRemap::new(HalfedgeMesh::empty(),None,None,None,None);assert!(remap.configuration_mut().is_ok());assert!(remap.advance(&mut HalfedgeMesh::empty()).unwrap().is_none());assert!(remap.configuration_mut().is_err());assert!(remap.advance(&mut HalfedgeMesh::empty()).unwrap().is_some());assert!(remap.advance(&mut HalfedgeMesh::empty()).is_err());
}

/// 🪆️ The same original snapshot closes every admitted source, buffer and interruption with actual receipts.
#[test]
fn original_modeling_snapshot_frontier_preserves_same_source_and_actual_receipts(){
    use crate::test_support::observe_tessellation_system as observe;
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/🎟️owners.json")).unwrap();let policy=&law["snapshotPreparation"];
    for flipped in [false,true] {for close_copy in law["copyGrants"].as_array().unwrap().iter().map(|copy|copy.as_u64().unwrap()as usize){for cut in std::iter::once(None).chain(policy["cancelTurns"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize))){
        let(mut snapshot,source)=observe(||{let positions=serde_json::from_value::<Vec<[f32;3]>>(policy["positions"].clone()).unwrap();let faces=serde_json::from_value::<Vec<Vec<u32>>>(policy["faces"].clone()).unwrap();let mut mesh=HalfedgeMesh::from_faces(&positions,&faces).unwrap();mesh.faces[0].flipped=flipped;MeshSourceSnapshot::new(mesh)});let original=(snapshot.source.vertices.as_ptr(),snapshot.source.halfedges.as_ptr(),snapshot.source.faces.as_ptr());let(mut born,mut freed,mut turns,mut copied)=(source.0,source.1,0,0);
        loop {
            if cut==Some(turns){break;}
            let demand=snapshot.preparation_demands().unwrap();if snapshot.state().normalizing&&snapshot.state().reversing.is_none() {let field=if snapshot.state().normal_corner+1==snapshot.scratch_corners().len() {"normalLastCopyBytes"}else if snapshot.state().normal_corner==0 {"normalFirstCopyBytes"}else {"normalMiddleCopyBytes"};assert_eq!(demand.copy_bytes,policy[field].as_u64().unwrap()as usize);}let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let state=((snapshot.state().reservation,snapshot.state().vertex,snapshot.state().face,snapshot.state().halfedge,snapshot.state().normal_corner,snapshot.state().normalizing,snapshot.state().reversing),(snapshot.soup.positions.len(),snapshot.soup.faces.len(),snapshot.soup.normals.len(),snapshot.scratch_corners().len()),(snapshot.soup.positions.as_ptr(),snapshot.soup.faces.as_ptr(),snapshot.soup.normals.as_ptr(),snapshot.scratch_corners().as_ptr()));
            for denied in [Some(RetainedCloneGrant {maximum_items:0,..grant}),Some(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant {maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant})].into_iter().flatten(){
                let mut receipt=RetainedCloneProgress::default();let(result,heap)=observe(||snapshot.advance(denied,&mut receipt).unwrap());assert!(!result);assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));assert_eq!(((snapshot.state().reservation,snapshot.state().vertex,snapshot.state().face,snapshot.state().halfedge,snapshot.state().normal_corner,snapshot.state().normalizing,snapshot.state().reversing),(snapshot.soup.positions.len(),snapshot.soup.faces.len(),snapshot.soup.normals.len(),snapshot.scratch_corners().len()),(snapshot.soup.positions.as_ptr(),snapshot.soup.faces.as_ptr(),snapshot.soup.normals.as_ptr(),snapshot.scratch_corners().as_ptr())),state);assert_eq!(snapshot.preparation_demands().unwrap(),demand);
            }
            let mut receipt=RetainedCloneProgress::default();let(done,heap)=observe(||snapshot.advance(grant,&mut receipt).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(receipt.fits(grant));assert_eq!(receipt.copied_items,1);assert_eq!(receipt.copied_bytes,demand.copy_bytes);born+=heap.0;freed+=heap.1;copied+=receipt.copied_bytes;turns+=1;assert!(turns<1000);if done {assert_eq!(copied,policy[if flipped {"flippedCopyBytes"}else {"forwardCopyBytes"}].as_u64().unwrap()as usize);assert_eq!(snapshot.soup.positions,serde_json::from_value::<Vec<[f32;3]>>(policy["positions"].clone()).unwrap());let mut faces=serde_json::from_value::<Vec<Vec<u32>>>(policy["faces"].clone()).unwrap();if flipped {faces[0].reverse();}assert_eq!(snapshot.soup.faces,faces);assert_eq!(snapshot.soup.normals[0].0,serde_json::from_value::<[f32;3]>(policy[if flipped {"flippedNormal"}else {"normal"}].clone()).unwrap());break;}
        }
        assert_eq!((snapshot.source.vertices.as_ptr(),snapshot.source.halfedges.as_ptr(),snapshot.source.faces.as_ptr()),original);let(mut owner,heap)=observe(||ControlledRetirement::new(snapshot).unwrap_or_else(|_|panic!("original Snapshot needs typed closure")));assert_eq!(heap,(0,0));while !owner.terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:close_copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(close_copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(grant));born+=heap.0;freed+=heap.1;turns+=1;if turns>143&&step.progress()==RetainedCloneProgress::default(){panic!("[DEBUG] Original Snapshot closure stalled flipped={flipped} cut={cut:?} grant={grant:?} nextCopy={}",owner.next_copy_byte_demand().unwrap());}assert!(turns<100000);}assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
    }}}
}

