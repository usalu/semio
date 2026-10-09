use super::*;
use crate::brep::operations::euler::make_vertex;
use crate::brep::operations::primitives::make_box;

/// ♻️ A closed box plus one orphan vertex: `compact` must free exactly the orphan and nothing
/// the box's solid transitively reaches.
#[semio_framework_async_macros::async_test]
async fn compact_frees_exactly_the_unreachable_orphan() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let orphan = make_vertex(&mut body, Pnt3::new(9.0, 9.0, 9.0), Tol::DEFAULT, &mut rec);
    let before = body.entity_counts();
    assert_eq!(before.vertices, 9, "8 box corners + 1 orphan");

    let keep = body.reachable_from(&[EntityRef::Solid(solid)]);
    assert!(!keep.vertices.contains(&orphan), "the orphan is not reachable from the solid");
    let freed = body.compact(keep);
    assert_eq!(freed.freed_vertices, 1);
    assert_eq!(freed.freed_edges, 0);
    assert_eq!(freed.freed_faces, 0);

    let after = body.entity_counts();
    assert_eq!(after.vertices, 8);
    assert!(!body.vertices.is_live(orphan));
    assert!(body.solids.is_live(solid), "the kept solid's id must stay valid — no index remap");
}

/// ♻️ A stale id from before a `compact` must be rejected by the generation check, never alias
/// whatever entity ends up reusing the freed slot.
#[semio_framework_async_macros::async_test]
async fn compact_leaves_stale_ids_rejected_by_generation() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let orphan = make_vertex(&mut body, Pnt3::new(9.0, 9.0, 9.0), Tol::DEFAULT, &mut rec);

    let keep = body.reachable_from(&[EntityRef::Solid(solid)]);
    body.compact(keep);
    assert_eq!(body.vertices.get(orphan), None, "stale id must not resolve after compaction");

    let reused = make_vertex(&mut body, Pnt3::new(1.0, 2.0, 3.0), Tol::DEFAULT, &mut rec);
    assert_eq!(reused.raw_index(), orphan.raw_index(), "LIFO free list reuses the freed slot");
    assert_ne!(reused.raw_generation(), orphan.raw_generation());
    assert_eq!(body.vertices.get(orphan), None, "the old id still must not alias the new vertex");
}

/// ♻️ `merge` must leave `self`'s own ids/labels untouched (existing handles stay resolvable)
/// while grafting `other`'s entities in with non-colliding, offset labels.
#[semio_framework_async_macros::async_test]
async fn merge_preserves_self_and_offsets_others_labels() {
    let mut a = Body::new();
    let mut rec = history::OpRecorder::new();
    let a_solid = make_box(&mut a, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let a_label_before = a.solids.get(a_solid).unwrap().label;
    let a_next_before = a.labels.next();

    let mut b = Body::new();
    let mut rec_b = history::OpRecorder::new();
    let b_solid = make_box(&mut b, 2.0, 2.0, 2.0, &mut rec_b).unwrap();
    let b_label = b.solids.get(b_solid).unwrap().label;

    let map = a.merge(&b);

    assert!(a.solids.is_live(a_solid), "self's own solid id must stay valid after merge");
    assert_eq!(a.solids.get(a_solid).unwrap().label, a_label_before, "self's own labels must not shift");

    let merged_solid = map.solids[&b_solid];
    assert!(a.solids.is_live(merged_solid));
    let merged_label = a.solids.get(merged_solid).unwrap().label;
    assert_eq!(merged_label.0, b_label.0 + a_next_before, "other's labels are offset above self's high-water mark");
    assert!(a.labels.next() > merged_label.0, "self's label source now carries the merged high-water mark forward");

    let keep = a.reachable_from(&[EntityRef::Solid(merged_solid)]);
    let mesh_faces = a.solid_faces(merged_solid);
    assert_eq!(mesh_faces.len(), 6, "the merged box keeps all 6 faces");
    assert_eq!(keep.faces.len(), 6);
}

/// 🎟️ Original reachable membership allocations remain owned through independently granted physical closure.
#[test]
fn original_reach_set_owns_exact_physical_membership() {
    use crate::brep::queries::tessellation::tests::observe_tessellation_system as observe;
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️reachability/🔣️.json")).unwrap();
    let mut body=Body::new();let mut rec=history::OpRecorder::new();let solid=make_box(&mut body,1.,1.,1.,&mut rec).unwrap();let orphan=make_vertex(&mut body,Pnt3::new(9.,9.,9.),Tol::DEFAULT,&mut rec);
    for copy in law["copyGrants"].as_array().unwrap() {
        let (keep,source)=observe(||body.reachable_from(&[EntityRef::Solid(solid)]));
        let counts=[keep.vertices.len(),keep.edges.len(),keep.coedges.len(),keep.loops.len(),keep.faces.len(),keep.shells.len(),keep.solids.len(),keep.curves3.len(),keep.curves2.len(),keep.surfaces.len()];
        for (count,key) in counts.into_iter().zip(["vertices","edges","coedges","loops","faces","shells","solids","curves3","curves2","surfaces"]) {assert_eq!(count,law["box"][key].as_u64().unwrap() as usize,"{key}");}
        assert!(!keep.vertices.contains(&orphan));
        let (owner,heap)=observe(||ControlledRetirement::new(keep).unwrap_or_else(|(error,_)|panic!("original reachable owner: {error}")));assert_eq!(heap,(law["handoffBirthBytes"].as_u64().unwrap() as usize,law["handoffReleaseBytes"].as_u64().unwrap() as usize));let mut owner=owner;
        let(mut born,mut freed,mut denied,mut turns)=(0,0,0,0);
        while !owner.terminal_is_empty() {
            let copy=copy.as_u64().unwrap() as usize;let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            if grant.maximum_capacity_bytes!=0 {let(step,heap)=observe(||owner.step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
            if grant.maximum_release_bytes>law["deniedReleaseBytes"].as_u64().unwrap() as usize {for _ in 0..2 {let (step,heap)=observe(||owner.step(RetainedCloneGrant {maximum_release_bytes:law["deniedReleaseBytes"].as_u64().unwrap() as usize,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));denied+=1;}}
            let (step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress()!=Default::default()||owner.terminal_is_empty(),"stalled original ReachSet turn={turns} copyDemand={} grant={grant:?} step={step:?} actual={heap:?} source={source:?} born={born} freed={freed}",owner.next_copy_byte_demand().unwrap());born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);
        }
        assert!(denied>0);assert_eq!(source.0-source.1+born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,law["terminalDropBytes"].as_u64().unwrap() as usize));eprintln!("[DEBUG] Original ReachSet copy={copy} source={} admitted={born} physical={freed} refusals={denied} turns={turns} terminalDrop=0",source.0-source.1);
    }
}

/// 🧭️ The original body walk borrows each topology member under exact independent grants and retains all cancellation scratch.
#[test]
fn original_reachability_turns_preserve_source_and_actual_physical_receipts() {
    use crate::brep::queries::tessellation::tests::observe_tessellation_system as observe;
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️reachability/🔣️.json")).unwrap();
    let mut body=Body::new();let mut rec=history::OpRecorder::new();let solid=make_box(&mut body,1.,1.,1.,&mut rec).unwrap();let original=body.entity_counts();
    for cancel in std::iter::once(None).chain(law["incremental"]["cancelTurns"].as_array().unwrap().iter().map(|turn|Some(turn.as_u64().unwrap() as usize))) {
        let (mut job,heap)=observe(||ReachabilityJob::new());assert_eq!(heap,(0,0));let (accepted,heap)=observe(||job.begin_root(EntityRef::Solid(solid)));accepted.unwrap();assert_eq!(heap,(0,0));
        let(mut admitted,mut physical,mut turns)=(0,0,0);
        while !job.walk_is_complete() {
            if cancel==Some(turns) {let(_,heap)=observe(||job.cancel());assert_eq!(heap,(0,0));let(step,heap)=observe(||job.step(&body,RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:64}).unwrap());assert!(matches!(step,ReachabilityStep::Cancelled(_)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));break;}
            let copy=job.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:job.next_release_byte_demand().unwrap(),maximum_depth:job.next_depth_demand().unwrap()};
            for denied in [RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant}].into_iter().chain(grant.maximum_capacity_bytes.checked_sub(1).map(|bytes|RetainedCloneGrant {maximum_capacity_bytes:bytes,..grant})) {let(step,heap)=observe(||job.step(&body,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
            let(step,heap)=observe(||job.step(&body,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress().copied_items,1);assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));admitted+=heap.0;physical+=heap.1;turns+=1;assert!(turns<10000);assert_eq!(body.entity_counts(),original);
        }
        if cancel.is_none(){let keep=job.keep();assert_eq!(keep.vertices.len(),law["box"]["vertices"].as_u64().unwrap() as usize);assert_eq!(keep.faces.len(),law["box"]["faces"].as_u64().unwrap() as usize);assert_eq!(keep.curves2.len(),law["box"]["curves2"].as_u64().unwrap() as usize);}
        let(mut owner,heap)=observe(||ControlledRetirement::new(job).unwrap_or_else(|(error,_)|panic!("original walk closure: {error}")));assert_eq!(heap,(0,0));while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));admitted+=heap.0;physical+=heap.1;}
        assert_eq!(admitted,physical);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));eprintln!("[DEBUG] Original reachability cancel={cancel:?} sameBody=true oneItem=true physical={physical} turns={turns} terminalDrop=0");
    }
}
