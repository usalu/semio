//! 🔒️ Native tessellation tests that inspect owned backing directly.

use super::*;
use semio_os_store_test_support::{observe_tessellation_system,watch_released_pointers,MESH_RELEASES,OBSERVED_MESH,OBSERVED_RELEASE_BYTES,OBSERVED_SAMPLE,SAMPLE_RELEASES};
use semio_framework_3d_brep_construction::euler::{add_face,make_edge,make_vertex};
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_representation::{arena::ArenaId,topology::{make_loop,history::OpRecorder}};
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt2,Vec2};

#[path="../🧰️support/🦀️.rs"]
mod support;
use support::{build_unit_box,exact_retirement_grant,retire_once,small_retirement_grant,triangle_area_sum};

#[semio_framework_async_macros::async_test]
async fn coedge_uv_prefers_stored_pcurve_when_present() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let p0 = Pnt3::new(0.0, 0.0, 0.0);
    let p1 = Pnt3::new(1.0, 0.0, 0.0);
    let v0 = make_vertex(&mut body, p0, tol, &mut rec);
    let v1 = make_vertex(&mut body, p1, tol, &mut rec);
    let curve = body.curves3.insert(Curve3::Line { origin: p0, dir: p1 - p0 });
    let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, tol, &mut rec);
    let pcurve = body.curves2.insert(Curve2::Line { origin: Pnt2::new(5.0, 5.0), dir: Vec2::new(1.0, 0.0) });
    let surface = body.surfaces.insert(Surface::Plane { frame: Frame3::WORLD });
    let outer = make_loop(&mut body, FaceId::from_raw(0, 0), &[(edge, true)]);
    let face = add_face(&mut body, surface, Some(outer), vec![], false, tol, &mut rec);
    body.loops.get_mut(outer).unwrap().face = face;
    let coedge_id = body.loop_coedges(outer)[0];
    {
        let coedge = body.coedges.get_mut(coedge_id).unwrap();
        coedge.pcurve = Some(pcurve);
        coedge.prange = (0.0, 1.0);
    }
    let mut cache = EdgeSampleCache::new();
    cache.insert(edge, sample_edge_points(&body, edge, 0.1).unwrap());
    let (_positions, uvs, _poles) = collect_loop_uv(&body, outer, body.surfaces.get(surface).unwrap(), &cache).unwrap();
    assert!((uvs[0].0 - 5.0).abs() < 1e-9 && (uvs[0].1 - 5.0).abs() < 1e-9, "first sample should come from the stored pcurve, got {:?}", uvs[0]);
}

#[test]
fn tessellation_cancel_handoff_and_small_grants_cover_every_actual_system_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body = Body::new();
    let solid = build_unit_box(&mut body, &mut OpRecorder::new());
    let (mut job, constructor) = observe_tessellation_system(|| TessellationJob::new(0.1));
    while job.transfer.position.is_empty() { assert!(matches!(job.step(&body, TessellationInput::Solid(solid), 1).unwrap(), TessellationStep::Working(_))); }
    let (_, signal) = observe_tessellation_system(|| job.cancel());
    let (mut retirement, handoff) = observe_tessellation_system(|| job.detach_retirement());
    let items = fixture["grants"]["items"].as_u64().unwrap() as usize;
    let bytes = fixture["grants"]["bytes"].as_u64().unwrap() as usize;
    let mut maximum_small_release = 0;
    let mut maximum_unfunded_capacity = 0;
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        let grant=small_retirement_grant(&retirement,items,bytes);
        let (step, heap) = observe_tessellation_system(|| retire_once(&mut retirement,grant));
        maximum_small_release = maximum_small_release.max(heap.1);
        maximum_unfunded_capacity = maximum_unfunded_capacity.max(heap.0);
        assert!(step.progress().fits(grant));
        if step.progress()==Default::default() { let grant=exact_retirement_grant(&retirement,items);let (receipt,heap)=observe_tessellation_system(||retire_once(&mut retirement,grant));assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes)); }
        turns += 1;
        assert!(turns < 10000);
    }
    assert_eq!(constructor, (fixture["system"]["constructorCapacityBytes"].as_u64().unwrap() as usize, 0));
    assert_eq!(signal, (fixture["system"]["cancelCapacityBytes"].as_u64().unwrap() as usize, fixture["system"]["cancelReleaseBytes"].as_u64().unwrap() as usize));
    assert_eq!(handoff, (fixture["system"]["handoffCapacityBytes"].as_u64().unwrap() as usize, fixture["system"]["handoffReleaseBytes"].as_u64().unwrap() as usize), "existing owner handoff requires no unfunded allocation or release");
    assert!(maximum_small_release <= bytes, "actual whole System deallocation exceeded the eight-byte grant: {maximum_small_release}");
    assert_eq!(maximum_unfunded_capacity, fixture["system"]["turnCapacityBytes"].as_u64().unwrap() as usize);
}

#[test]
fn original_whole_tessellation_typed_owner_matches_each_physical_full_grant() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant, retirement::controlled::ControlledRetirement};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body = Body::new();
    let solid = build_unit_box(&mut body, &mut OpRecorder::new());
    let (mut job, source) = observe_tessellation_system(|| {
        let mut job = TessellationJob::new(0.1);
        while job.transfer.position.is_empty() { job.step(&body, TessellationInput::Solid(solid), 1).unwrap(); }
        job
    });
    let sample_pointer = job.edge_cache.values().next().unwrap().as_ptr();
    let mesh_pointer = job.transfer.position.as_ptr();
    let (_, cancel) = observe_tessellation_system(|| job.cancel());
    let (mut owner, handoff) = observe_tessellation_system(|| ControlledRetirement::new(job).unwrap_or_else(|_| panic!("original whole Tessellation authority")));
    assert_eq!(owner.original().unwrap().edge_cache.values().next().unwrap().as_ptr(), sample_pointer);
    assert_eq!(owner.original().unwrap().transfer.position.as_ptr(), mesh_pointer);
    assert_eq!(cancel, (0, 0));
    assert_eq!(handoff, (0, 0));
    let mut born = 0;
    let mut freed = 0;
    let mut refusals = 0;
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: fixture["grants"]["items"].as_u64().unwrap() as usize, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() };
        let small_release = fixture["grants"]["bytes"].as_u64().unwrap() as usize;
        if grant.maximum_release_bytes > small_release {
            for _ in 0..2 {
                let small = RetainedCloneGrant { maximum_release_bytes: small_release, ..grant };
                let (step, heap) = observe_tessellation_system(|| retire_once(&mut owner,small));
                assert!(step.progress().fits(small));
                assert_eq!(heap, (0, 0), "actual whole allocation must stay owned under the eight-byte grant");
                assert_eq!(step.progress(), Default::default());
                assert_eq!(owner.next_release_byte_demand().unwrap(), grant.maximum_release_bytes);
                refusals += 1;
            }
        }
        let (step, heap) = observe_tessellation_system(|| retire_once(&mut owner,grant));
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes), "each whole original typed receipt equals System allocation/deallocation");
        born += heap.0;
        freed += heap.1;
        turns += 1;
        assert!(turns < 100000);
    }
    assert!(refusals > 0);
    assert_eq!(freed, source.0 - source.1 + born);
}

#[test]
fn tessellation_cancel_preserves_original_allocations_until_retirement_grants() {
    use std::sync::atomic::Ordering::SeqCst;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body=Body::new();let mut rec=OpRecorder::new();let solid=build_unit_box(&mut body,&mut rec);
    let mut job=TessellationJob::new(0.1);
    while job.transfer.position.is_empty() { assert!(matches!(job.step(&body,TessellationInput::Solid(solid),1).unwrap(),TessellationStep::Working(_))); }
    let sample=job.edge_cache.values().next().unwrap();
    watch_released_pointers();OBSERVED_SAMPLE.store(sample.as_ptr() as usize,SeqCst);OBSERVED_MESH.store(job.transfer.position.as_ptr() as usize,SeqCst);
    SAMPLE_RELEASES.store(0,SeqCst);MESH_RELEASES.store(0,SeqCst);OBSERVED_RELEASE_BYTES.store(0,SeqCst);
    job.cancel();
    assert_eq!(SAMPLE_RELEASES.load(SeqCst),fixture["cancel"]["releasesAtSignal"].as_u64().unwrap() as usize,"cancel must retain the original sample allocation");
    assert_eq!(MESH_RELEASES.load(SeqCst),0,"cancel must retain the original transfer allocation");
    assert!(matches!(job.step(&body,TessellationInput::Solid(solid),1).unwrap(),TessellationStep::Cancelled(_)));
    let mut retirement=job.detach_retirement();
    for (items,release) in [(0,8),(1,0)] {
        let grant=small_retirement_grant(&retirement,items,release);
        assert_eq!(retire_once(&mut retirement,grant).progress(),Default::default());
    }
    assert_eq!(SAMPLE_RELEASES.load(SeqCst),0);assert_eq!(MESH_RELEASES.load(SeqCst),0);
    let mut turns=0;let mut refused=0;let mut physical=0;
    while !retirement.terminal_is_empty() {
        OBSERVED_RELEASE_BYTES.store(0,SeqCst);let grant=small_retirement_grant(&retirement,1,8);let small=retire_once(&mut retirement,grant);
        let released=OBSERVED_RELEASE_BYTES.load(SeqCst);assert!(released<=8,"physical sample/mesh release exceeded the actual eight-byte grant");physical+=released;assert!(small.progress().fits(grant));
        if small.progress()==Default::default() {
            refused+=1;let sample_before=SAMPLE_RELEASES.load(SeqCst);let mesh_before=MESH_RELEASES.load(SeqCst);
            assert_eq!(retire_once(&mut retirement,grant).progress(),Default::default());assert_eq!(SAMPLE_RELEASES.load(SeqCst),sample_before);assert_eq!(MESH_RELEASES.load(SeqCst),mesh_before);
            let demand=exact_retirement_grant(&retirement,1);OBSERVED_RELEASE_BYTES.store(0,SeqCst);let receipt=retire_once(&mut retirement,demand);let released=OBSERVED_RELEASE_BYTES.load(SeqCst);assert!(released<=demand.maximum_release_bytes,"physical sample/mesh release exceeded admitted original demand");physical+=released;assert!(receipt.progress().fits(demand));
        }
        turns+=1;assert!(turns<10000);
    }
    assert!(refused>0);assert!(physical>8);
    assert_eq!(SAMPLE_RELEASES.load(SeqCst),fixture["cancel"]["sampleReleases"].as_u64().unwrap() as usize);assert_eq!(MESH_RELEASES.load(SeqCst),fixture["cancel"]["meshReleases"].as_u64().unwrap() as usize);
    OBSERVED_SAMPLE.store(0,SeqCst);OBSERVED_MESH.store(0,SeqCst);
}

#[test]
fn tessellation_cold_admission_does_not_collect_original_topology_before_a_grant() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut body=Body::new();let mut rec=OpRecorder::new();let solid=build_unit_box(&mut body,&mut rec);
    let mut job=TessellationJob::new(0.1);
    assert_eq!(job.edge_order.len(),fixture["cold"]["initialEdges"].as_u64().unwrap() as usize);
    assert_eq!(job.faces.len(),fixture["cold"]["initialFaces"].as_u64().unwrap() as usize);
    assert_eq!(job.progress().units_done,fixture["cold"]["initialUnits"].as_u64().unwrap() as usize);
    assert!(matches!(job.step(&body,TessellationInput::Solid(solid),0).unwrap(),TessellationStep::Working(_)));
    assert!(job.edge_order.is_empty() && job.faces.is_empty());
    let before=job.progress().units_done;assert!(matches!(job.step(&body,TessellationInput::Solid(solid),1).unwrap(),TessellationStep::Working(_)));assert_eq!(job.progress().units_done,before+1);
    let (mesh,_)=job.run_to_completion(&body,TessellationInput::Solid(solid)).unwrap();assert_eq!(mesh.face_groups.len(),fixture["box"]["faces"].as_u64().unwrap() as usize);assert_eq!(mesh.edge_groups.len(),12);assert_eq!(mesh.index.len()/3,12);assert!((triangle_area_sum(&mesh)-6.0).abs()<1e-6);
    let volume=mesh.index.chunks_exact(3).map(|tri| {let p=|id:u32|Vec3::new(mesh.position[id as usize*3] as f64,mesh.position[id as usize*3+1] as f64,mesh.position[id as usize*3+2] as f64);p(tri[0]).dot(p(tri[1]).cross(p(tri[2])))/6.0}).sum::<f64>().abs();assert!((volume-fixture["box"]["volume"].as_f64().unwrap()).abs()<1e-6);
    use semio_framework_3d_brep_construction::staged::{StageStep, StagedOperation};
    let mut caller=semio_framework_3d_brep_boolean::SplitJob::new(&body,solid,Pnt3::new(0.5,0.0,0.0),Vec3::new(1.0,0.0,0.0),0.1).unwrap();
    for turn in 0..fixture["cold"]["callerProbeTurns"].as_u64().unwrap() as usize {
        assert_eq!(caller.advance(&mut body,&mut rec).unwrap(),StageStep::Working);
        assert_eq!(caller.progress().phase,"tessellate","original Split Plan advanced before its cold child produced a mesh");assert_eq!(caller.progress().done,turn+1);assert!(caller.progress().total>=caller.progress().done);
    }
    assert!((semio_framework_3d_brep_queries::mass_properties::solid_volume(&body,solid,0.1).unwrap()-fixture["box"]["volume"].as_f64().unwrap()).abs()<1e-6);
}
