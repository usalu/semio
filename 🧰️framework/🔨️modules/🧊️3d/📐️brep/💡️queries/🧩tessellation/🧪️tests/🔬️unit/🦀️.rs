//! 🔬️ Native tessellation tests through the public crate boundary.

use semio_framework_3d_brep_tessellation::*;
use semio_framework_3d_brep_representation::{arena::{EdgeId,FaceId,LoopId,SolidId},contract::{MeshTransfer,SurfaceKind},error::KernelError,topology::{Edge,Wire}};
use semio_framework_3d_brep_vector::{Pnt3,Vec3};

#[path="../🧰️support/🦀️.rs"]
mod support;
use support::{build_unit_box,exact_retirement_grant,retire_once,small_retirement_grant,triangle_area_sum,with_retirement_effect};
use semio_framework_mesh_engine::HistoryFoldIndex;
use semio_framework_3d_testing_support::observe_tessellation_system;
use {semio_framework_3d_brep_construction::euler::{add_face, add_shell, add_solid, make_edge, make_vertex}, semio_framework_3d_brep_representation::topology::make_loop};
use semio_framework_3d_brep_construction::primitives::{make_cylinder, make_rectangle_wire, make_sphere};
use semio_framework_3d_brep_representation::arena::ArenaId;
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt2, Vec2};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_plane_face_with_hole(body: &mut Body, rec: &mut OpRecorder) -> FaceId {
    let tol = Tol::DEFAULT;
    let corners = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(4.0, 0.0, 0.0), Pnt3::new(4.0, 3.0, 0.0), Pnt3::new(0.0, 3.0, 0.0)];
    let v: Vec<_> = corners.iter().map(|&p| make_vertex(body, p, tol, rec)).collect();
    let mut edges = Vec::new();
    for i in 0..4 {
        let a = i;
        let b = (i + 1) % 4;
        let curve = body.curves3.insert(Curve3::Line { origin: corners[a], dir: corners[b] - corners[a] });
        edges.push(make_edge(body, curve, (0.0, 1.0), v[a], v[b], tol, rec));
    }
    let outer_members: Vec<(EdgeId, bool)> = (0..4).map(|i| (edges[i], true)).collect();
    let outer = make_loop(body, FaceId::from_raw(0, 0), &outer_members);
    let hole_center = Pnt3::new(2.0, 1.5, 0.0);
    let hole_frame = Frame3::from_normal(hole_center, Vec3::Z).unwrap();
    let hole_v = make_vertex(body, hole_frame.to_world(Pnt3::new(0.5, 0.0, 0.0)), tol, rec);
    let circle_curve = body.curves3.insert(Curve3::Circle { frame: hole_frame, radius: 0.5 });
    let hole_edge = make_edge(body, circle_curve, (0.0, std::f64::consts::TAU), hole_v, hole_v, tol, rec);
    let inner = make_loop(body, FaceId::from_raw(0, 0), &[(hole_edge, false)]);
    let surface = body.surfaces.insert(Surface::Plane { frame: Frame3::from_normal(corners[0], Vec3::Z).unwrap() });
    let face = add_face(body, surface, Some(outer), vec![inner], false, tol, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    body.loops.get_mut(inner).unwrap().face = face;
    face
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9

#[semio_framework_async_macros::async_test]
async fn unit_box_has_six_face_groups_and_unit_normals() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_unit_box(&mut body, &mut rec);
    let (mesh, _report) = tessellate_solid_with_report(&body, solid, 0.1).expect("tessellate unit box");
    assert_eq!(mesh.face_groups.len(), 6, "unit box must yield 6 face groups");
    assert_eq!(mesh.index.len() / 3, 12, "unit box must yield 12 triangles (2 per planar quad)");
    assert_eq!(mesh.edge_groups.len(), 12, "unit box must yield 12 edge groups");
    assert!(!mesh.position.is_empty(), "positions must be nonempty");
    assert!(!mesh.index.is_empty(), "indices must be nonempty");
    assert!(!mesh.normal.is_empty(), "normals must be nonempty");
    assert_eq!(mesh.position.len(), mesh.normal.len());
    assert_eq!(mesh.position.len() % 3, 0);
    assert_eq!(mesh.index.len() % 3, 0);
    for n in mesh.normal.chunks_exact(3) {
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-3, "normal length {len} should be ~1");
    }
    let total_group = mesh.face_groups.iter().map(|g| g.count as usize).sum::<usize>();
    assert_eq!(total_group, mesh.index.len());
    for label in mesh.face_groups.iter().map(|g| &g.entity_id) {
        assert!(label.parse::<u64>().is_ok(), "face entity_id {label} must be a decimal PersistentLabel");
    }
    assert_eq!(mesh.face_infos.len(), 6);
    for info in &mesh.face_infos {
        assert_eq!(info.surface_kind, SurfaceKind::Plane);
        assert!((info.area - 1.0).abs() < 1e-6, "unit box face area should be 1, got {}", info.area);
    }
}

#[semio_framework_async_macros::async_test]
async fn tessellate_face_matches_one_box_face() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_unit_box(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let mesh = tessellate_face(&body, face, 0.1).expect("tessellate face");
    assert_eq!(mesh.face_groups.len(), 1);
    assert_eq!(mesh.index.len(), 6);
    assert_eq!(mesh.position.len() / 3, 4);
}

#[semio_framework_async_macros::async_test]
async fn sample_edge_polyline_returns_line_endpoints() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let v1 = make_vertex(&mut body, Pnt3::new(2.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X * 2.0 });
    let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, Tol::DEFAULT, &mut rec);
    let poly = sample_edge_polyline(&body, edge, 0.1);
    assert_eq!(poly.len(), 6);
    assert!((poly[0] - 0.0).abs() < 1e-6);
    assert!((poly[3] - 2.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn shared_edge_samples_are_identical_across_adjacent_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_unit_box(&mut body, &mut rec);
    let faces = body.solid_faces(solid);
    let edge = body.face_coedges(faces[0]).into_iter().map(|c| body.coedges.get(c).unwrap().edge).next().unwrap();
    let a = sample_edge_polyline(&body, edge, 0.05);
    let b = sample_edge_polyline(&body, edge, 0.05);
    assert_eq!(a, b);
}

#[semio_framework_async_macros::async_test]
async fn circle_edge_samples_respect_deflection() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let frame = Frame3::WORLD;
    let radius = 1.0;
    let curve = body.curves3.insert(Curve3::Circle { frame, radius });
    let v = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let edge = make_edge(&mut body, curve, (0.0, std::f64::consts::TAU), v, v, Tol::DEFAULT, &mut rec);
    let coarse = sample_edge_polyline(&body, edge, 0.05);
    let fine = sample_edge_polyline(&body, edge, 0.005);
    assert!(fine.len() > coarse.len(), "tighter deflection must densify circle samples ({} vs {})", fine.len(), coarse.len());
    assert!(coarse.len() >= 6);
}

#[semio_framework_async_macros::async_test]
async fn missing_solid_returns_missing_entity() {
    let body = Body::new();
    let err = tessellate_solid(&body, SolidId::from_raw(9, 0), 0.1).unwrap_err();
    assert!(matches!(err, KernelError::MissingEntity(_)));
}

#[semio_framework_async_macros::async_test]
async fn tessellate_rectangle_wire_emits_edge_segments() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let wire = make_rectangle_wire(&mut body, 2.0, 1.5, &mut rec).expect("wire");
    let mesh = tessellate_wire(&body, &wire, 0.1).expect("tessellate wire");
    assert!(mesh.edges.len() >= 24, "expected closed rectangle edge polylines, got {}", mesh.edges.len());
    assert!(mesh.position.is_empty());
    assert!(mesh.index.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn cylinder_shared_edge_vertices_are_reused_exactly() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).expect("cylinder");
    let deflection = 0.1;
    let mesh = tessellate_solid(&body, solid, deflection).expect("tessellate cylinder");
    let mut circle_edge = None;
    'search: for face in body.solid_faces(solid) {
        for coedge_id in body.face_coedges(face) {
            let edge_id = body.coedges.get(coedge_id).unwrap().edge;
            if let Some(Curve3::Circle { .. }) = body.curves3.get(body.edges.get(edge_id).unwrap().curve) {
                circle_edge = Some(edge_id);
                break 'search;
            }
        }
    }
    let edge_id = circle_edge.expect("cylinder must have a circle edge");
    let poly = sample_edge_polyline(&body, edge_id, deflection);
    for chunk in poly.chunks_exact(3) {
        let p = Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64);
        let found = mesh.position.chunks_exact(3).any(|q| Pnt3::new(q[0] as f64, q[1] as f64, q[2] as f64).distance(p) < 1e-6);
        assert!(found, "edge sample point {p:?} must be reused verbatim as a mesh vertex (crack-free)");
    }
}

#[semio_framework_async_macros::async_test]
async fn cylinder_lateral_face_area_matches_analytic_across_the_seam() {
    let (radius, height) = (1.0, 2.0);
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, radius, height, &mut rec).expect("cylinder");
    let lateral = body.solid_faces(solid)[0];
    let (mesh, _report) = tessellate_face_with_report(&body, lateral, 0.02).expect("tessellate lateral face");
    let area = triangle_area_sum(&mesh);
    let analytic = 2.0 * std::f64::consts::PI * radius * height;
    assert!((area - analytic).abs() / analytic < 0.02, "lateral area {area} should match analytic {analytic} across the seam");
    assert_eq!(mesh.face_infos.len(), 1);
    assert_eq!(mesh.face_infos[0].surface_kind, SurfaceKind::Cylinder);
    for n in mesh.normal.chunks_exact(3) {
        let len = ((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]) as f64).sqrt();
        assert!((len - 1.0).abs() < 1e-3, "normal length {len} should be ~1 across the seam");
    }
}

#[semio_framework_async_macros::async_test]
async fn tighter_deflection_yields_more_triangles() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).expect("cylinder");
    let coarse = tessellate_solid(&body, solid, 0.2).expect("coarse");
    let fine = tessellate_solid(&body, solid, 0.02).expect("fine");
    assert!(fine.index.len() > coarse.index.len(), "finer deflection must yield more triangles ({} vs {})", fine.index.len(), coarse.index.len());
}

#[semio_framework_async_macros::async_test]
async fn report_max_chordal_respects_deflection() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).expect("cylinder");
    let deflection = 0.05;
    let (_mesh, report) = tessellate_solid_with_report(&body, solid, deflection).expect("tessellate with report");
    assert!(report.max_chordal <= deflection * 1.05, "max_chordal {} should respect deflection {}", report.max_chordal, deflection);
}

#[semio_framework_async_macros::async_test]
async fn face_with_circular_hole_triangulates_inside_the_trim() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = build_plane_face_with_hole(&mut body, &mut rec);
    let mesh = tessellate_face(&body, face, 0.1).expect("tessellate hole face");
    assert!(!mesh.index.is_empty());
    let hole_center = (2.0_f64, 1.5_f64);
    let hole_radius = 0.5_f64;
    for tri in mesh.index.chunks_exact(3) {
        let p = |i: u32| -> Pnt3 {
            let b = (i as usize) * 3;
            Pnt3::new(mesh.position[b] as f64, mesh.position[b + 1] as f64, mesh.position[b + 2] as f64)
        };
        let (a, b, c) = (p(tri[0]), p(tri[1]), p(tri[2]));
        let cx = (a.x + b.x + c.x) / 3.0;
        let cy = (a.y + b.y + c.y) / 3.0;
        assert!((-1e-6..=4.0 + 1e-6).contains(&cx) && (-1e-6..=3.0 + 1e-6).contains(&cy), "triangle centroid ({cx}, {cy}) outside outer rectangle");
        let dist = ((cx - hole_center.0).powi(2) + (cy - hole_center.1).powi(2)).sqrt();
        assert!(dist >= hole_radius - 1e-3, "triangle centroid ({cx}, {cy}) falls inside the hole (dist {dist})");
    }
}

#[semio_framework_async_macros::async_test]
async fn sphere_caps_collapse_pole_to_single_vertex() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let radius = 1.0;
    let solid = make_sphere(&mut body, radius, &mut rec).expect("sphere");
    for face in body.solid_faces(solid) {
        let mesh = tessellate_face(&body, face, 0.1).expect("tessellate cap");
        let mut extreme = Pnt3::new(0.0, 0.0, 0.0);
        let mut extreme_abs = 0.0_f64;
        for chunk in mesh.position.chunks_exact(3) {
            let p = Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64);
            if p.z.abs() > extreme_abs {
                extreme_abs = p.z.abs();
                extreme = p;
            }
        }
        assert!((extreme_abs - radius).abs() < 0.05, "cap should approach the pole radius, got {extreme_abs}");
        let count = mesh.position.chunks_exact(3).filter(|chunk| Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64).distance(extreme) < 1e-4).count();
        assert_eq!(count, 1, "pole must collapse to exactly one vertex, found {count}");
    }
}

#[test]
fn original_native_family_handoff_retains_all_system_owners_until_grants() {
    use semio_framework_3d_brep_engine::Brep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let mut family=Brep::new();
    family.box_prim_sync(1.0,1.0,1.0).unwrap();
    let (mut retirement,handoff)=observe_tessellation_system(||family.detach_retirement());
    let small=fixture["grants"]["bytes"].as_u64().unwrap() as usize;
    let mut maximum_release=0;
    let mut maximum_birth=0;
    let mut turns=0;
    while !retirement.terminal_is_empty() {
        let grant=small_retirement_grant(&retirement,1,small);
        let (step,heap)=observe_tessellation_system(||retire_once(&mut retirement,grant));
        maximum_birth=maximum_birth.max(heap.0);maximum_release=maximum_release.max(heap.1);
        if step.progress()==Default::default() {let grant=exact_retirement_grant(&retirement,1);let (receipt,heap)=observe_tessellation_system(||retire_once(&mut retirement,grant));assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes));}
        turns+=1;assert!(turns<100000);
    }
    assert_eq!(handoff,(fixture["nativeFamily"]["handoffCapacityBytes"].as_u64().unwrap() as usize,fixture["nativeFamily"]["handoffReleaseBytes"].as_u64().unwrap() as usize));
    assert!(maximum_release<=small,"original native family physical release exceeded the actual eight-byte grant");
    assert_eq!(maximum_birth,0);
}

#[test]
fn original_native_family_typed_owner_covers_nurbs_wire_aliases_and_compound() {
    use semio_framework_3d_brep_engine::Brep;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let cases=&fixture["nativeFamily"];
    for converted in cases["convertToNurbs"].as_array().unwrap() {
        let converted=converted.as_bool().unwrap();
        let (family,source)=observe_tessellation_system(|| {
            let mut family=Brep::new();let first=family.box_prim_sync(1.0,1.0,1.0).unwrap();let second=family.box_prim_sync(1.0,1.0,1.0).unwrap();
            if converted {family.convert_to_nurbs_sync(&first).unwrap();}
            family.deconstruct_sync(&first).unwrap();
            let compound=family.compound_sync(&[first,second]).unwrap();assert!((family.volume_sync(&compound).unwrap()-cases["compoundVolume"].as_f64().unwrap()).abs()<1e-8);
            let wire=family.rectangle_wire_sync(cases["wire"]["width"].as_f64().unwrap(),cases["wire"]["height"].as_f64().unwrap()).unwrap();assert!((family.length_sync(&wire).unwrap()-cases["wire"]["length"].as_f64().unwrap()).abs()<1e-8);
            family.arc_curve_sync([0.0;3],[0.0,0.0,1.0],1.0,0.0,1.0).unwrap();family.plane_surface_sync([0.0;3],[0.0,0.0,1.0]).unwrap();
            family
        });
        let vertex=family.representation().vertices.ids().next().unwrap();let pointer=family.representation().vertices.get(vertex).unwrap() as *const _;
        let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new(family).unwrap_or_else(|_|panic!("original Brep controlled family")));assert_eq!(handoff,(0,0));assert_eq!(owner.original().unwrap().representation().vertices.get(vertex).unwrap() as *const _,pointer);
        let (mut born,mut released,mut refusals,mut turns)=(0,0,0,0);
        while !owner.terminal_is_empty() {
            let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            if grant.maximum_release_bytes>8 {for _ in 0..2 {let (step,heap)=observe_tessellation_system(||retire_once(&mut owner,RetainedCloneGrant{maximum_release_bytes:8,..grant}));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refusals+=1;}}
            let (step,heap)=observe_tessellation_system(||retire_once(&mut owner,grant));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;turns+=1;assert!(turns<100000);
        }
        assert!(refusals>0);assert_eq!(released,source.0-source.1+born);
    }
}

#[test]
fn original_live_index_extraction_preserves_row_pointers_without_heap_work() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let rows=&fixture["liveRows"];
    let ((mut index,mut extracted),source)=observe_tessellation_system(|| {
        let index:HistoryFoldIndex<String,Vec<u8>>=rows["keys"].as_array().unwrap().iter().map(|key|(key.as_str().unwrap().to_owned(),vec![0;rows["payloadBytes"].as_u64().unwrap() as usize])).collect();
        (index,Vec::with_capacity(rows["removedKeys"].as_array().unwrap().len()))
    });
    let pointers=index.iter().map(|(key,value)|(key.clone(),(key.as_ptr(),value.as_ptr()))).collect::<std::collections::BTreeMap<_,_>>();
    let removed=rows["removedKeys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap()).collect::<Vec<_>>();
    for slot in 0..index.slot_count() {
        let mut predicates=0;
        let (_,heap)=observe_tessellation_system(||if let Some(row)=index.extract_slot_if(slot,|key,_|{predicates+=1;!removed.contains(&key.as_str())}) {assert_eq!((row.0.as_ptr(),row.1.as_ptr()),pointers[&row.0]);extracted.push(row);});
        assert!(predicates<=rows["maximumPredicatesPerTurn"].as_u64().unwrap() as usize);assert_eq!(heap,(rows["extractionCapacityBytes"].as_u64().unwrap() as usize,rows["extractionReleaseBytes"].as_u64().unwrap() as usize));
    }
    assert_eq!(extracted.iter().map(|(key,_)|key.as_str()).collect::<Vec<_>>(),removed);
    let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new((index,extracted)).unwrap_or_else(|_|panic!("original live index row ownership")));assert_eq!(handoff,(0,0));
    let (mut born,mut released)=(0,0);
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let (step,heap)=observe_tessellation_system(||retire_once(&mut owner,grant));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
    }
    assert_eq!(released,source.0-source.1+born);
}

#[test]
fn original_body_arenas_retire_by_exact_physical_full_grants() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();
    let ((body,vertex),source)=observe_tessellation_system(|| {
        let mut body=Body::new();build_unit_box(&mut body,&mut OpRecorder::new());
        let vertex=body.vertices.ids().next().unwrap();
        let spare=body.vertices.get(vertex).unwrap().clone();let hole=body.vertices.insert(spare);drop(body.vertices.remove(hole));
        (body,vertex)
    });
    let pointer=body.vertices.get(vertex).unwrap() as *const _;
    let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new(body).unwrap_or_else(|_|panic!("original Body controlled authority")));
    assert_eq!(handoff,(0,0));assert_eq!(owner.original().unwrap().vertices.get(vertex).unwrap() as *const _,pointer);
    let (mut born,mut freed,mut refusals,mut turns)=(0,0,0,0);
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:fixture["grants"]["items"].as_u64().unwrap() as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let small=fixture["grants"]["bytes"].as_u64().unwrap() as usize;
        if grant.maximum_release_bytes>small {
            for _ in 0..2 {
                let (step,heap)=observe_tessellation_system(||retire_once(&mut owner,RetainedCloneGrant{maximum_release_bytes:small,..grant}));
                assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refusals+=1;
            }
        }
        let (step,heap)=observe_tessellation_system(||retire_once(&mut owner,grant));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);
    }
    assert!(refusals>0);assert_eq!(freed,source.0-source.1+born);
}

#[test]
fn original_validation_scratch_retains_all_physical_buffers_until_full_grants() {
    use semio_framework_3d_brep_validation::BodyValidationJob;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let laws=&fixture["validationScratch"];
    let mut body=Body::new();build_unit_box(&mut body,&mut OpRecorder::new());
    let (mut job,constructor)=observe_tessellation_system(||BodyValidationJob::new(&body));assert_eq!(constructor,(laws["constructorCapacityBytes"].as_u64().unwrap() as usize,laws["constructorReleaseBytes"].as_u64().unwrap() as usize));
    let (_,source)=observe_tessellation_system(||job.step(&body,laws["warmTurns"].as_u64().unwrap() as usize));assert!(source.0-source.1>=laws["minimumRetainedBytes"].as_u64().unwrap() as usize);
    let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original validation scratch requires typed retirement")));assert_eq!(handoff,(laws["handoffCapacityBytes"].as_u64().unwrap() as usize,laws["handoffReleaseBytes"].as_u64().unwrap() as usize));
    let (mut born,mut freed,mut refusals,mut turns)=(0,0,0,0);
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        if grant.maximum_release_bytes>8 {for _ in 0..2 {let (step,heap)=observe_tessellation_system(||retire_once(&mut owner,RetainedCloneGrant {maximum_release_bytes:8,..grant}));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refusals+=1;}}
        let (step,heap)=observe_tessellation_system(||retire_once(&mut owner,grant));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
    }
    assert!(refusals>0);assert_eq!(source.0-source.1+born,freed);
}

#[test]
fn original_mesh_import_cancel_retains_original_buffers_before_full_grants() {
    use semio_framework_3d_brep_engine::MeshImportCursor;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let laws=&fixture["meshImport"];
    let ((mut body,mut cursor),source)=observe_tessellation_system(||{let positions=laws["positions"].as_array().unwrap().iter().map(|n|n.as_f64().unwrap() as f32).collect();let indices=laws["indices"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u32).collect();let mut body=Body::new();let mut cursor=MeshImportCursor::new(positions,Vec::new(),indices,0.1).unwrap();cursor.step(&mut body,laws["warmTurns"].as_u64().unwrap() as usize).unwrap();(body,cursor)});
    let (_,cancel)=observe_tessellation_system(||cursor.cancel());assert_eq!(cancel,(0,0));
    let transition=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:8,maximum_depth:1};
    let (receipt,handoff)=observe_tessellation_system(||with_retirement_effect(transition,|effect|cursor.close_step(&mut body,effect)).unwrap());assert_eq!(handoff,(laws["handoffCapacityBytes"].as_u64().unwrap() as usize,0));assert!(receipt.progress().fits(transition));
    let (mut born,mut freed,mut refusals,mut turns)=(0,0,0,0);
    while !cursor.retirement_complete() {
        let copy=cursor.next_close_copy_byte_demand(&body).unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(&body,copy).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand(&body).unwrap(),maximum_depth:cursor.next_close_depth_demand(&body).unwrap()};
        if grant.maximum_release_bytes>8 {for _ in 0..2 {let (receipt,heap)=observe_tessellation_system(||with_retirement_effect(RetainedCloneGrant{maximum_release_bytes:8,..grant},|effect|cursor.close_step(&mut body,effect)).unwrap());assert_eq!(receipt.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.next_close_release_byte_demand(&body).unwrap(),grant.maximum_release_bytes);refusals+=1;}}
        let (receipt,heap)=observe_tessellation_system(||with_retirement_effect(grant,|effect|cursor.close_step(&mut body,effect)).unwrap());assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
    }
    assert!(body.vertices.is_empty() && body.edges.is_empty() && body.faces.is_empty() && body.curves3.is_empty() && body.curves2.is_empty() && body.surfaces.is_empty());
    let mut owner=ControlledRetirement::new(body).unwrap_or_else(|_|panic!("same original import arena backing requires typed ownership"));
    while !owner.terminal_is_empty() {let grant=exact_retirement_grant(&owner,1);let (receipt,heap)=observe_tessellation_system(||retire_once(&mut owner,grant));assert!(receipt.progress().fits(grant));assert_eq!(heap,(receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes));born+=heap.0;freed+=heap.1;}
    let (_,shell)=observe_tessellation_system(||drop(cursor));assert_eq!(shell,(0,0));assert!(refusals>0);assert_eq!(source.0-source.1+born,freed);
}

#[test]
fn analytic_curve_preview_closes_the_seam_and_yields_between_segments() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/curve-preview.json")).unwrap();
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    for row in fixture.as_array().unwrap() {
        let major = row["major"].as_f64().unwrap();
        let minor = row["minor"].as_f64().unwrap();
        let curve = if major == minor { Curve3::Circle { frame, radius: major } }
            else { Curve3::Ellipse { frame, major_radius: major, minor_radius: minor } };
        let body = Body::new();
        let mut job = TessellationJob::new(row["deflection"].as_f64().unwrap());
        let input = TessellationInput::Curve(&curve);
        assert!(matches!(job.step(&body, input, 0).unwrap(), TessellationStep::Working(_)));
        let mut steps = 0;
        while !job.is_terminal() {
            job.step(&body, input, 1).unwrap();
            steps += 1;
            assert!(steps < 1000);
        }
        let (mesh, _) = job.into_mesh().unwrap();
        let segments = mesh.edges.len() / 6;
        assert_eq!(steps, segments + 1);
        assert!(segments >= row["minimumSegments"].as_u64().unwrap() as usize);
        assert_eq!(&mesh.edges[..3], &mesh.edges[mesh.edges.len()-3..]);
        let mut funded = TessellationJob::for_analytic_curve(&curve, row["deflection"].as_f64().unwrap()).unwrap();
        let mut copied = 0;
        let mut reserved = 0;
        loop {
            let demand = funded.analytic_demand().unwrap();
            let grant = semio_framework_value::RetainedCloneGrant {
                maximum_items: 1, maximum_copy_bytes: demand.copy_bytes,
                maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 0, maximum_depth: 1,
            };
            let (result, receipt) = funded.step_analytic_funded(&curve, grant).unwrap();
            assert_eq!(receipt.copied_items, 1);
            copied += receipt.copied_bytes;
            reserved += receipt.retained_capacity_bytes;
            if let Some(edges) = result {
                assert_eq!(edges, mesh.edges);
                assert_eq!(reserved, edges.capacity() * size_of::<f32>());
                assert_eq!(copied, edges.len() * size_of::<f32>());
                break;
            }
        }
        for pair in mesh.edges.chunks_exact(6).collect::<Vec<_>>().windows(2) {
            assert_eq!(&pair[0][3..], &pair[1][..3]);
        }
    }
}

#[test]
fn funded_conic_face_preview_winds_each_fan_and_closes_the_periodic_edge() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/curve-preview.json")).unwrap();
    for row in fixture.as_array().unwrap() {
        let curve = Curve3::Ellipse { frame:Frame3::WORLD, major_radius:row["major"].as_f64().unwrap(),minor_radius:row["minor"].as_f64().unwrap() };
        for flipped in [false,true] {
            let mut job = TessellationJob::for_conic_face(&curve,row["deflection"].as_f64().unwrap()).unwrap();
            let z = if flipped {-1.0} else {1.0};
            let mut steps=0;
            let mesh=loop {
                let d=job.analytic_demand().unwrap();
                let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,maximum_release_bytes:0};
                let (mesh, receipt)=job.step_conic_face_funded(&curve,[0.0,0.0,z],flipped,grant).unwrap();
                assert_eq!(receipt.copied_items,1);
                steps+=1;
                assert!(steps<1000);
                if let Some(mesh)=mesh{break mesh}
            };
            assert_eq!(steps,5+mesh.index.len()/3);
            assert_eq!(&mesh.edges[..3],&mesh.edges[mesh.edges.len()-3..]);
            let area=mesh.index.chunks_exact(3).map(|tri|{
                let coordinate=|v:u32,i:usize|mesh.position[v as usize*3+i] as f64;
                let twice=coordinate(tri[1],0)*coordinate(tri[2],1)-coordinate(tri[1],1)*coordinate(tri[2],0);
                assert!(twice*z>0.0);
                twice*z/2.0
            }).sum::<f64>();
            assert!((area/row["diskArea"].as_f64().unwrap()-1.0).abs()<0.01);
        }
    }
}

#[test]
fn original_spline_preview_reserves_once_and_meets_local_chord_bounds() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../📸️representation/➰️curve/🧪️tests/🧫️fixtures/rational-jets.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let points:Vec<[f64;3]>=serde_json::from_value(row["controls"].clone()).unwrap();
        let knots:Vec<f64>=serde_json::from_value(row["knots"].clone()).unwrap();
        let weights:Vec<f64>=serde_json::from_value(row["weights"].clone()).unwrap();
        let degree=row["degree"].as_u64().unwrap() as usize;
        let curve=Curve3::Nurbs{
            knots:semio_framework_3d_brep_curve::bspline::KnotVector::new(knots,degree,points.len()).unwrap(),
            controls:points.into_iter().map(|[x,y,z]|Pnt3::new(x,y,z)).collect(),weights,
        };
        let tolerance=row["previewDeflection"].as_f64().unwrap();
        let mut job=TessellationJob::for_analytic_curve(&curve,tolerance).unwrap();
        let mut reserves=0;
        let mut events=0;
        let edges=loop {
            let d=job.analytic_demand().unwrap();
            let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,maximum_release_bytes:0};
            assert_eq!(job.step_analytic_funded(&curve,semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant}).unwrap(),(None,Default::default()));
            let (edges,receipt)=job.step_analytic_funded(&curve,grant).unwrap();
            assert_eq!(receipt.copied_items,1);
            assert!(receipt.fits(grant));
            reserves+=usize::from(receipt.retained_capacity_bytes!=0);
            events+=1;
            assert!(events<100_000);
            if let Some(edges)=edges{break edges;}
        };
        assert_eq!(reserves,1);
        if let Some(endpoints)=row.get("previewEndpoints") {
            let expected:Vec<[f32;3]>=serde_json::from_value(endpoints.clone()).unwrap();
            assert_eq!(edges,expected.into_iter().flatten().collect::<Vec<_>>());
        }
        assert_eq!(job.progress().units_total,events);
        let (first,end)=curve.original_spline_span_range().unwrap();
        let mut segments=edges.chunks_exact(6);
        for span in first..end {
            let (lo,hi,count)=curve.original_spline_subdivision(span,tolerance).unwrap();
            for i in 0..count {
                let edge=segments.next().unwrap();
                let t=|fraction| lo+(hi-lo)*(i as f64+fraction)/count as f64;
                for fraction in [0.125,0.25,0.5,0.75,0.875] {
                    let p=curve.eval(t(fraction));
                    let lerp=|k|edge[k] as f64*(1.0-fraction)+edge[k+3] as f64*fraction;
                    assert!(p.distance(Pnt3::new(lerp(0),lerp(1),lerp(2)))<=tolerance+1e-6);
                }
            }
        }
        assert!(segments.next().is_none());
    }
}

fn preview_ring(body: &mut Body, rec: &mut OpRecorder, ring: &[[f64; 2]], frame: Frame3) -> LoopId {
    let points: Vec<_> = ring.iter().map(|&[u,v]| frame.origin + frame.x*u + frame.y*v).collect();
    let vertices: Vec<_> = points.iter().map(|&p| make_vertex(body,p,Tol::DEFAULT,rec)).collect();
    let members: Vec<_> = points.iter().enumerate().map(|(i,&origin)| {
        let j=(i+1)%points.len();
        let curve=body.curves3.insert(Curve3::Line {origin,dir:points[j]-origin});
        (make_edge(body,curve,(0.0,1.0),vertices[i],vertices[j],Tol::DEFAULT,rec),true)
    }).collect();
    make_loop(body,FaceId::from_raw(0,0),&members)
}

fn preview_polygon(body: &mut Body, rec: &mut OpRecorder, ring: &[[f64; 2]], frame: Frame3, flipped: bool) -> FaceId {
    let outer=preview_ring(body,rec,ring,frame);
    let surface=body.surfaces.insert(Surface::Plane{frame});
    let face=add_face(body,surface,Some(outer),vec![],flipped,Tol::DEFAULT,rec);
    body.loops.get_mut(outer).unwrap().face=face;
    face
}

fn run_planar_preview(body:&Body,root:PlanarPreviewRoot)->Result<semio_framework_mesh_engine::MeshData,semio_framework_value::ValueError>{
    run_planar_preview_roots(body,root,None)
}

fn run_planar_preview_roots(body:&Body,root:PlanarPreviewRoot,members:Option<&[SolidId]>)->Result<semio_framework_mesh_engine::MeshData,semio_framework_value::ValueError>{
    use semio_framework_value::RetainedCloneGrant;
    let mut job=TessellationJob::for_planar_shape(root,7,0.01);
    for paid in 0..1000 {
        let d=job.planar_preview_demand(body,members).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let refused=RetainedCloneGrant{maximum_items:0,..grant};
        assert_eq!(job.step_planar_funded(body,root,members,7,refused).unwrap(),(None,Default::default()));
        assert_eq!(job.progress().units_done,paid);
        let (mesh,receipt)=job.step_planar_funded(body,root,members,7,grant)?;
        assert_eq!(receipt.copied_items,1);
        assert!(receipt.fits(grant));
        if let Some(mesh)=mesh {
            assert_eq!(job.progress().units_total,paid+1);
            mesh.validate_component_references().unwrap();
            return Ok(mesh);
        }
    }
    panic!("bounded planar preview stalled");
}

#[test]
fn retained_planar_spline_trims_keep_authored_edges_and_reject_interior_jumps() {
    use semio_framework_3d_brep_curve::bspline::KnotVector;
    use semio_framework_value::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    let row = &fixture["planarSplineTrim"];
    let tolerance = row["deflection"].as_f64().unwrap();
    let frame = Frame3::from_normal(Pnt3::new(3., -4., 2.), Vec3::new(0., 0.6, 0.8)).unwrap();
    for (name, reverse, jump, bend, accept) in [
        ("continuous", false, false, false, true), ("continuous", true, false, false, true),
        ("continuous", false, true, false, false), ("continuous", false, false, true, false),
        ("clippedBeforeJump", false, true, false, true),
    ] {
        let [lo, hi]: [f64; 2] = serde_json::from_value(row[name]["trim"].clone()).unwrap();
        let mut controls: Vec<[f64; 3]> = serde_json::from_value(row["controls"].clone()).unwrap();
        if jump { controls[row["jumpControl"].as_u64().unwrap() as usize][0] += row["jumpDelta"][0].as_f64().unwrap(); }
        if bend { controls[1][2] += 0.05; }
        let curve = Curve3::Nurbs {
            knots: KnotVector::new(serde_json::from_value(row["knots"].clone()).unwrap(), row["degree"].as_u64().unwrap() as usize, controls.len()).unwrap(),
            controls: controls.into_iter().map(|[u,v,w]| frame.to_world(Pnt3::new(u,v,w))).collect(),
            weights: serde_json::from_value(row["weights"].clone()).unwrap(),
        };
        let mut ends = [lo, hi].map(|time| { let point = frame.to_local(curve.eval(time)); [point.x,point.y] });
        if reverse { ends.reverse(); }
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let face = preview_polygon(&mut body, &mut rec, &[ends[0],ends[1],[0.,0.]], frame, false);
        let face_row = body.faces.get(face).unwrap();
        let first = body.loops.get(face_row.outer.unwrap()).unwrap().first;
        let edge_id = body.coedges.get(first).unwrap().edge;
        let edge = body.edges.get_mut(edge_id).unwrap();
        let label = edge.label.0.to_string();
        edge.range = (lo,hi);
        if reverse {
            std::mem::swap(&mut edge.v0, &mut edge.v1);
            body.coedges.get_mut(first).unwrap().forward = false;
        }
        *body.curves3.get_mut(edge.curve).unwrap() = curve;
        let root = PlanarPreviewRoot::Face(face);
        let mut job = TessellationJob::for_planar_shape(root, 14, tolerance);
        let mut mesh = None;
        let mut retained = 0;
        let mut invalid_jump = false;
        for _ in 0..5_000 {
            let d = job.planar_preview_demand(&body,None).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: d.copy_bytes,
                maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth };
            assert!(grant.maximum_copy_bytes <= 128);
            let zero = RetainedCloneGrant { maximum_items: 0, ..grant };
            assert_eq!(job.step_planar_funded(&body,root,None,14,zero).unwrap(),(None,Default::default()));
            let (output,receipt) = match job.step_planar_funded(&body,root,None,14,grant) {
                Ok(outcome) => outcome,
                Err(error) => {
                    assert!(jump && !accept && error.to_string().contains("jumps at an interior knot"));
                    invalid_jump = true;
                    break;
                }
            };
            assert_eq!(receipt.copied_items,1);
            assert!(receipt.fits(grant));
            retained += receipt.retained_capacity_bytes;
            if output.is_some() || !job.is_planar_preview() { mesh=output; break; }
        }
        if !accept {
            assert!(mesh.is_none());
            assert_eq!(invalid_jump,jump);
            if !jump { assert!(!job.is_planar_preview()); }
            assert_eq!(retained,0);
            continue;
        }
        let mesh = mesh.unwrap();
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["edge"].len(),3);
        assert_eq!(mesh.component_references["vertex"].len(),3);
        let pick = mesh.component_references["edge"].iter().position(|key|key==&label).unwrap() as u32;
        assert!(mesh.edge_ids.iter().filter(|&&id|id==pick).count()>2);
        let point = |id:u32| Pnt3::from_array(std::array::from_fn(|i|mesh.positions[id as usize*3+i].into()));
        let area = mesh.indices.chunks_exact(3).map(|tri| {
            let [a,b,c] = [point(tri[0]),point(tri[1]),point(tri[2])];
            let normal = (b-a).cross(c-a);
            assert!(normal.dot(frame.z)>0.0);
            normal.norm()*0.5
        }).sum::<f64>();
        assert!((area-row[name]["area"].as_f64().unwrap()).abs()<0.02);
    }
}

#[test]
fn planar_preview_keeps_winding_persistent_picks_and_shared_edges() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/planar-preview.json")).unwrap();
    let read_ring=|row:&serde_json::Value|serde_json::from_value::<Vec<[f64;2]>>(row["ring"].clone()).unwrap();
    let frame=Frame3::from_normal(Pnt3::new(3.0,-4.0,2.0),Vec3::new(0.0,0.6,0.8)).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let ring=read_ring(row);
        for flipped in [false,true] {
            let mut body=Body::new();
            let face=preview_polygon(&mut body,&mut OpRecorder::new(),&ring,frame,flipped);
            let mesh=run_planar_preview(&body,PlanarPreviewRoot::Face(face)).unwrap();
            let expected_normal=if flipped{-frame.x.cross(frame.y)}else{frame.x.cross(frame.y)};
            let point=|i:u32|{let v=&mesh.positions[i as usize*3..];Pnt3::new(v[0] as f64,v[1] as f64,v[2] as f64)};
            let area=mesh.indices.chunks_exact(3).map(|tri|{
                let [a,b,c]=[point(tri[0]),point(tri[1]),point(tri[2])];
                let normal=(b-a).cross(c-a);
                assert!(normal.dot(expected_normal)>0.0);
                normal.norm()*0.5
            }).sum::<f64>();
            assert!((area-row["area"].as_f64().unwrap()).abs()<1e-5);
            assert_eq!(mesh.indices.len()/3,row["triangles"].as_u64().unwrap() as usize);
            let perimeter=mesh.edge_positions.chunks_exact(6).map(|edge|
                Vec3::new((edge[3]-edge[0]) as f64,(edge[4]-edge[1]) as f64,(edge[5]-edge[2]) as f64).norm()).sum::<f64>();
            assert!((perimeter-row["perimeter"].as_f64().unwrap()).abs()<1e-5);
            assert_eq!(mesh.component_references["edge"].len(),ring.len());
            assert_eq!(mesh.component_references["vertex"].len(),ring.len());
            let label=body.faces.get(face).unwrap().label;
            assert_eq!(mesh.component_references["face"],vec![label.decimal(&mut [0;20]).to_owned()]);
        }
    }
    let mut body=Body::new();
    let id=build_unit_box(&mut body,&mut OpRecorder::new());
    let mesh=run_planar_preview(&body,PlanarPreviewRoot::Solid(id)).unwrap();
    let expected=&fixture["unitBox"];
    assert_eq!(mesh.indices.len()/3,expected["triangles"].as_u64().unwrap() as usize);
    for (domain,key) in [("face","faces"),("edge","edges"),("vertex","vertices")] {
        assert_eq!(mesh.component_references[domain].len(),expected[key].as_u64().unwrap() as usize);
    }
    assert_eq!(mesh.edge_positions.len()/6,mesh.component_references["edge"].len());
    for row in fixture["invalid"].as_array().unwrap(){
        let mut body=Body::new();
        let face=preview_polygon(&mut body,&mut OpRecorder::new(),&read_ring(row),Frame3::WORLD,false);
        let error=run_planar_preview(&body,PlanarPreviewRoot::Face(face)).unwrap_err();
        assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::InvalidValue);
    }
}

#[test]
fn original_topology_preview_samples_trimmed_and_reversed_circular_edges() {
    use semio_framework_3d_brep_representation::topology::Vertex;
    use semio_framework_3d_brep_tolerance::Tol;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json")).unwrap();
    let row = &corpus["edgePreview"];
    let axes = |key: &str| std::array::from_fn(|axis| row[key][axis].as_f64().unwrap() as f32);
    let to_point = |[x, y, z]: [f32; 3]| Pnt3::new(x.into(), y.into(), z.into());
    let start = axes("start");
    let end = axes("end");
    let deflection = row["deflection"].as_f64().unwrap();
    let mut body = Body::new();
    let start_label = body.new_label();
    let end_label = body.new_label();
    let edge_label = body.new_label();
    let v0 = body.vertices.insert(Vertex { position: to_point(start), label: start_label, tol: Tol::DEFAULT });
    let v1 = body.vertices.insert(Vertex { position: to_point(end), label: end_label, tol: Tol::DEFAULT });
    let curve = body.curves3.insert(Curve3::Circle { frame: Frame3::WORLD, radius: row["radius"].as_f64().unwrap() });
    let range = (row["startAngle"].as_f64().unwrap(), row["endAngle"].as_f64().unwrap());
    let edge = body.edges.insert(Edge { curve, range, v0, v1, tol: Tol::DEFAULT, label: edge_label });
    let reverse = [(edge, false)];
    for revision in [7, 8] {
        if revision == 8 {
            let spline = body.curves3.get(curve).unwrap().to_nurbs(range);
            *body.curves3.get_mut(curve).unwrap() = Curve3::Nurbs {
                knots: spline.knots, controls: spline.controls, weights: spline.weights,
            };
        }
        for (input, a, b) in [
            (WirePreviewSource::Edge(edge), start, end),
            (WirePreviewSource::Wire { label: 999, members: &reverse }, end, start),
        ] {
            let mesh = run_funded_wire_preview(&body, input, revision, deflection);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.component_references["edge"], [edge_label.0.to_string()]);
            assert_eq!(mesh.component_references["vertex"].len(), 2);
            assert!((to_point(mesh.edge_positions[..3].try_into().unwrap()) - to_point(a)).norm() < 1e-6);
            assert!((to_point(mesh.edge_positions[mesh.edge_positions.len() - 3..].try_into().unwrap()) - to_point(b)).norm() < 1e-6);
            let mut length = 0.0;
            for segment in mesh.edge_positions.chunks_exact(6) {
                let a = to_point(segment[..3].try_into().unwrap());
                let b = to_point(segment[3..].try_into().unwrap());
                length += (b - a).norm();
                let center = a + (b - a) * 0.5;
                assert!((row["radius"].as_f64().unwrap() - (center - Frame3::WORLD.origin).norm()).abs() <= deflection);
            }
            assert!((row["length"].as_f64().unwrap() - length).abs() <= deflection);
        }
    }
}

#[test]
fn funded_axial_face_reads_original_cylinder_and_cone_trims() {
    use semio_framework_3d_brep_construction::primitives::make_cone;
    use semio_framework_value::RetainedCloneGrant;
    let values: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    for row in values["axialFacePreview"].as_array().unwrap() {
        let radius = row["radius"].as_f64().unwrap();
        let height = row["height"].as_f64().unwrap();
        let tolerance = row["deflection"].as_f64().unwrap();
        let mut body = Body::new();
        let mut record = OpRecorder::new();
        let solid = if row["kind"] == "cone" { make_cone(&mut body, radius, height, &mut record) }
            else { make_cylinder(&mut body, radius, height, &mut record) }.unwrap();
        let face = body.solid_faces(solid).into_iter().find(|&face| {
            let row = body.faces.get(face).unwrap();
            matches!(body.surfaces.get(row.surface), Some(Surface::Cylinder { .. } | Surface::Cone { .. }))
        }).unwrap();
        let face_row = body.faces.get(face).unwrap();
        let ring = face_row.outer.unwrap();
        let uses = body.loop_coedges(ring);
        let native_edges = uses.iter().map(|&id| body.coedges.get(id).unwrap().edge).collect::<Vec<_>>();
        let seam = native_edges.iter().find(|id| native_edges.iter().filter(|other| *other == *id).count() == 2).unwrap();
        let seam_label = body.edges.get(*seam).unwrap().label.0.to_string();
        let mut job = TessellationJob::for_axial_face_preview(face, 806, tolerance);
        let mut mesh = None;
        for paid in 0..30_000 {
            let d = job.axial_face_preview_demand(&body, face, 806).unwrap();
            let grant = RetainedCloneGrant {
                maximum_items: 1, maximum_copy_bytes: d.copy_bytes, maximum_capacity_bytes: d.capacity_bytes,
                maximum_release_bytes: d.release_bytes, maximum_depth: d.depth,
            };
            let zero = RetainedCloneGrant { maximum_items: 0, ..grant };
            assert_eq!(job.step_axial_face_preview(&body, face, 806, zero).unwrap(), (None, Default::default()));
            assert_eq!(job.progress().units_done, paid);
            let (output, receipt) = job.step_axial_face_preview(&body, face, 806, grant).unwrap();
            assert_eq!(receipt.copied_items, 1);
            assert!(receipt.fits(grant));
            assert!(job.is_axial_face_preview(), "native lateral trim was unsupported");
            if output.is_some() { mesh = output; break; }
        }
        let mesh = mesh.expect("funded axial preview completes");
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["face"], [face_row.label.0.to_string()]);
        assert_eq!(mesh.component_references["edge"].len(), row["edges"].as_u64().unwrap() as usize);
        assert_eq!(mesh.component_references["vertex"].len(), row["vertices"].as_u64().unwrap() as usize);
        let seam_id = mesh.component_references["edge"].iter().position(|x| *x == seam_label).unwrap() as u32;
        assert_eq!(mesh.edge_ids.iter().filter(|&&id| id == seam_id).count(), 1);
        let (min, max) = mesh.aabb();
        assert!((f64::from(min[2])).abs() < 1e-6 && (f64::from(max[2]) - height).abs() < 1e-6);
        let point = |index: u32| Pnt3::from_array(std::array::from_fn(|axis| mesh.positions[index as usize * 3 + axis].into()));
        let mut area = 0.;
        for indices in mesh.indices.chunks_exact(3) {
            let [a,b,c] = [point(indices[0]), point(indices[1]), point(indices[2])];
            let cross = (b-a).cross(c-a);
            area += cross.norm() * 0.5;
            let p = a + (b-a) * (1./3.) + (c-a) * (1./3.);
            let radial = Vec3::new(p.x, p.y, 0.);
            let expected_radius = if row["kind"] == "cone" { radius * (height - p.z) / height } else { radius };
            assert!((radial.norm() - expected_radius).abs() < tolerance);
            assert!(cross.dot(radial) > 0.);
        }
        assert!((area - row["area"].as_f64().unwrap()).abs() < tolerance);
        let mut cancelled = TessellationJob::for_axial_face_preview(face, 807, tolerance);
        cancelled.cancel();
        assert_eq!(cancelled.step_axial_face_preview(&body, face, 807, RetainedCloneGrant::default()).unwrap(), (None, Default::default()));
        let mut unsupported = TessellationJob::for_axial_face_preview(face, 808, tolerance);
        body.faces.get_mut(face).unwrap().inners.push(ring);
        let d = unsupported.axial_face_preview_demand(&body, face, 808).unwrap();
        let (mesh, receipt) = unsupported.step_axial_face_preview(&body, face, 808, RetainedCloneGrant {
            maximum_items: 1, maximum_copy_bytes: d.copy_bytes, maximum_depth: d.depth, ..Default::default()
        }).unwrap();
        assert!(mesh.is_none() && !unsupported.is_axial_face_preview());
        assert_eq!(receipt.retained_capacity_bytes, 0);
    }
}

fn run_funded_wire_preview(body: &Body, input: WirePreviewSource<'_>, revision: u64, deflection: f64) -> semio_framework_mesh_engine::MeshData {
    use semio_framework_value::RetainedCloneGrant;
    let mut job = TessellationJob::for_wire_preview(input, revision, deflection);
    for paid in 0..500 {
        let d = job.wire_preview_demand(body, input, revision).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 1, maximum_copy_bytes: d.copy_bytes,
            maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth,
        };
        let zero = RetainedCloneGrant { maximum_items: 0, ..grant };
        assert_eq!(job.step_wire_preview(body, input, revision, zero).unwrap(), (None, Default::default()));
        assert_eq!(job.progress().units_done, paid);
        let (mesh, work) = job.step_wire_preview(body, input, revision, grant).unwrap();
        assert_eq!(work.copied_items, 1);
        assert!(work.fits(grant));
        if let Some(mesh) = mesh { return mesh; }
        assert!(job.is_wire_preview());
    }
    panic!("topology preview completes with one paid event per turn");
}

#[test]
fn original_topology_preview_clips_spline_spans_and_keeps_jumps_open() {
    use semio_framework_3d_brep_curve::bspline::KnotVector;
    use semio_framework_3d_brep_representation::topology::Vertex;
    use semio_framework_value::RetainedCloneGrant;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json")).unwrap();
    let row = &corpus["splineEdgePreview"];
    let deflection = row["deflection"].as_f64().unwrap();
    let controls: Vec<[f64; 3]> = serde_json::from_value(row["controls"].clone()).unwrap();
    let knots = KnotVector::new(serde_json::from_value(row["knots"].clone()).unwrap(), row["degree"].as_u64().unwrap() as usize, controls.len()).unwrap();
    let weights = serde_json::from_value(row["weights"].clone()).unwrap();
    let segments: Vec<[[f32; 3]; 2]> = serde_json::from_value(row["segments"].clone()).unwrap();
    let [start, end]: [f64; 2] = serde_json::from_value(row["trim"].clone()).unwrap();
    let mut body = Body::new();
    let a_label = body.new_label();
    let b_label = body.new_label();
    let edge_label = body.new_label();
    let point = |[x,y,z]: [f32; 3]| Pnt3::new(x.into(), y.into(), z.into());
    let a = body.vertices.insert(Vertex { position: point(segments[0][0]), label: a_label, tol: Tol::DEFAULT });
    let b = body.vertices.insert(Vertex { position: point(segments.last().unwrap()[1]), label: b_label, tol: Tol::DEFAULT });
    let curve = body.curves3.insert(Curve3::Nurbs {
        knots, controls: controls.into_iter().map(|[x,y,z]| Pnt3::new(x,y,z)).collect(), weights,
    });
    let edge = body.edges.insert(Edge { curve, range: (start,end), v0: a, v1: b, tol: Tol::DEFAULT, label: edge_label });
    let descending_label = body.new_label();
    let descending = body.edges.insert(Edge { curve, range: (end,start), v0: b, v1: a, tol: Tol::DEFAULT, label: descending_label });
    for (id, edge_label, reversed) in [(edge, edge_label, false), (descending, descending_label, true)] {
        let members = [(id,false)];
        for (input, reverse) in [
            (WirePreviewSource::Edge(id), reversed),
            (WirePreviewSource::Wire { label: 314, members: &members }, !reversed),
        ] {
            let mesh = run_funded_wire_preview(&body, input, 9, deflection);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.component_references["edge"], [edge_label.0.to_string()]);
            let vertices = if reverse { [b_label, a_label] } else { [a_label, b_label] };
            let points = segments.iter().flatten();
            let expected: Vec<_> = if reverse {
                points.rev().flatten().copied().collect()
            } else {
                points.flatten().copied().collect()
            };
            assert_eq!(mesh.component_references["vertex"], vertices.map(|label| label.0.to_string()));
            assert_eq!(mesh.edge_positions, expected);
            assert_eq!(mesh.edge_ids, vec![0; segments.len()]);
        }
    }

    let Curve3::Nurbs { weights, .. } = body.curves3.get_mut(curve).unwrap() else { unreachable!() };
    weights[0] = -1.0;
    let input = WirePreviewSource::Edge(edge);
    let mut job = TessellationJob::for_wire_preview(input, 10, deflection);
    for _ in 0..2 {
        let demand = job.wire_preview_demand(&body, input, 10).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_depth: demand.depth, ..Default::default() };
        let (mesh, receipt) = job.step_wire_preview(&body, input, 10, grant).unwrap();
        assert!(mesh.is_none());
        assert_eq!(receipt.copied_items, 1);
        assert_eq!(receipt.retained_capacity_bytes, 0);
    }
    assert!(!job.is_wire_preview());
}

#[test]
fn planar_preview_preserves_inner_rings_and_component_counts() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/planar-preview.json")).unwrap();
    let expected=&fixture["holes"];
    let coords=|value:&serde_json::Value|serde_json::from_value::<Vec<[f64;2]>>(value.clone()).unwrap();
    let frame=Frame3::from_normal(Pnt3::new(2.0,-1.0,3.0),Vec3::new(0.0,0.6,0.8)).unwrap();
    for flipped in [false,true]{
        let mut body=Body::new();
        let mut rec=OpRecorder::new();
        let outer=preview_ring(&mut body,&mut rec,&coords(&expected["outer"]),frame);
        let inners:Vec<_>=expected["rings"].as_array().unwrap().iter()
            .map(|ring|preview_ring(&mut body,&mut rec,&coords(ring),frame)).collect();
        let surface=body.surfaces.insert(Surface::Plane{frame});
        let face=add_face(&mut body,surface,Some(outer),inners.clone(),flipped,Tol::DEFAULT,&mut rec);
        for id in std::iter::once(outer).chain(inners){body.loops.get_mut(id).unwrap().face=face;}
        let mesh=run_planar_preview(&body,PlanarPreviewRoot::Face(face)).unwrap();
        let edges=expected["edges"].as_u64().unwrap() as usize;
        assert_eq!(mesh.edge_ids.len(),edges);
        assert_eq!(mesh.component_references["vertex"].len(),edges);
        assert_eq!(mesh.indices.len()/3,expected["triangles"].as_u64().unwrap() as usize);
        let point=|i:u32|{let p=&mesh.positions[i as usize*3..];Pnt3::new(p[0].into(),p[1].into(),p[2].into())};
        let expected_normal=if flipped{-frame.x.cross(frame.y)}else{frame.x.cross(frame.y)};
        let area=mesh.indices.chunks_exact(3).map(|tri|{
            let [a,b,c]=[point(tri[0]),point(tri[1]),point(tri[2])];
            let normal=(b-a).cross(c-a);
            assert!(normal.dot(expected_normal)>0.0);
            normal.norm()*0.5
        }).sum::<f64>();
        assert!((area-expected["area"].as_f64().unwrap()).abs()<1e-5);
    }
}

fn run_surface_patch_preview(surface:&Surface,tolerance:f64)->semio_framework_mesh_engine::MeshData {
    use semio_framework_value::RetainedCloneGrant;
    let mut job=TessellationJob::for_surface_patch(surface,701,tolerance).unwrap();
    let mut turns=0;
    let mesh=loop {
        let d=job.analytic_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let refused=RetainedCloneGrant{maximum_items:0,..grant};
        assert_eq!(job.step_surface_patch_funded(surface,701,refused).unwrap(),(None,Default::default()));
        let (mesh,receipt)=job.step_surface_patch_funded(surface,701,grant).unwrap();
        assert_eq!(receipt.copied_items,1);
        assert!(receipt.fits(grant));
        turns+=1;
        assert!(turns<200_010);
        if let Some(mesh)=mesh{break mesh;}
    };
    assert_eq!(job.progress().units_done,turns);
    mesh
}

#[test]
fn funded_spline_surface_preview_meets_its_chord_bound() {
    use semio_framework_3d_brep_curve::bspline::KnotVector;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../📸️representation/🏄️surface/🧪️tests/🧫️fixtures/rational-patch.json")).unwrap();
    let net:Vec<Vec<[f64;3]>>=serde_json::from_value(fixture["controls"].clone()).unwrap();
    let degree=|key:&str|fixture[key].as_u64().unwrap() as usize;
    let knots=|key:&str|serde_json::from_value(fixture[key].clone()).unwrap();
    let surface=Surface::Nurbs{
        u_knots:KnotVector::new(knots("uKnots"),degree("uDegree"),net.len()).unwrap(),
        v_knots:KnotVector::new(knots("vKnots"),degree("vDegree"),net[0].len()).unwrap(),
        controls:net.into_iter().map(|row|row.into_iter().map(|[x,y,z]|Pnt3::new(x,y,z)).collect()).collect(),
        weights:serde_json::from_value(fixture["weights"].clone()).unwrap(),
    };
    let tolerance=fixture["previewDeflection"].as_f64().unwrap();
    let mesh=run_surface_patch_preview(&surface,tolerance);
    let n=((mesh.indices.len()/6)as f64).sqrt() as usize;
    assert!(n>1);
    assert_eq!(mesh.indices.len(),n*n*6);
    let point=|i:u32|{let c=&mesh.positions[i as usize*3..];Pnt3::new(c[0].into(),c[1].into(),c[2].into())};
    for (i,tri) in mesh.indices.chunks_exact(3).enumerate(){
        let [a,b,c]=[point(tri[0]),point(tri[1]),point(tri[2])];
        let rendered=a+((b-a)+(c-a))*(1.0/3.0);
        let (du,dv)=if i%2==0{(2.0/3.0,1.0/3.0)}else{(1.0/3.0,2.0/3.0)};
        let u=((i/2/n)as f64+du)/n as f64;
        let v=((i/2%n)as f64+dv)/n as f64;
        let expected=surface.point_original(u,v).unwrap();
        let normal=surface.normal_original(u,v).unwrap();
        assert!(rendered.distance(expected)<=tolerance);
        assert!((b-a).cross(c-a).dot(normal)>0.0);
    }
}

#[test]
fn funded_spline_surface_preview_does_not_bridge_discontinuous_knots() {
    use semio_framework_3d_brep_curve::bspline::KnotVector;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../📸️representation/🏄️surface/🧪️tests/🧫️fixtures/rational-patch.json")).unwrap();
    let case=&fixture["discontinuous"];
    let sections:Vec<[f64;2]>=serde_json::from_value(case["sections"].clone()).unwrap();
    let controls:Vec<Vec<Pnt3>>=sections.iter().flatten().map(|&x|vec![Pnt3::new(x,0.0,0.0),Pnt3::new(x,1.0,0.0)]).collect();
    let surface=Surface::Nurbs {
        u_knots:KnotVector::new(serde_json::from_value(case["uKnots"].clone()).unwrap(),1,controls.len()).unwrap(),
        v_knots:KnotVector::new(serde_json::from_value(fixture["vKnots"].clone()).unwrap(),1,2).unwrap(),
        weights:vec![vec![1.0;2];controls.len()],
        controls,
    };
    let mesh=run_surface_patch_preview(&surface,fixture["previewDeflection"].as_f64().unwrap());
    assert_eq!(mesh.indices.len()/3,case["triangles"].as_u64().unwrap() as usize);
    assert!(mesh.indices.chunks_exact(3).all(|face| {
        let points=[face[0],face[1],face[2]].map(|id|mesh.positions[id as usize*3] as f64);
        sections.iter().any(|&[a,b]|points.iter().all(|v|(a..=b).contains(v)))
    }));
}

#[test]
fn original_vertex_preview_transfers_its_position_and_persistent_label() {
    use semio_framework_value::RetainedCloneGrant;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/planar-preview.json")).unwrap();
    let point: [f64;3]=serde_json::from_value(fixture["vertex"]["point"].clone()).unwrap();
    let [x,y,z]=point;
    let mut body=Body::new();
    let id=make_vertex(&mut body,Pnt3::new(x,y,z),Tol::DEFAULT,&mut OpRecorder::new());
    let source=WirePreviewSource::Vertex(id);
    let mut job=TessellationJob::for_wire_preview(source,11,0.02);
    for _ in 0..60{
        let d=job.wire_preview_demand(&body,source,11).unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let (result,receipt)=job.step_wire_preview(&body,source,11,grant).unwrap();
        assert!(receipt.fits(grant));
        assert_eq!(receipt.copied_items,1);
        if let Some(mesh)=result {
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.positions,point.map(|axis|axis as f32));
            assert!(mesh.indices.is_empty()&&mesh.edge_positions.is_empty());
            assert_eq!(mesh.vertex_ids,[0]);
            let label=body.vertices.get(id).unwrap().label.decimal(&mut [0;20]).to_owned();
            assert_eq!(mesh.component_references["vertex"],[label]);
            return;
        }
    }
    panic!("funded vertex preview did not complete");
}

#[test]
fn original_planar_preview_samples_a_closed_circular_inner_edge() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/planar-preview.json")).unwrap();
    let expected=&fixture["circularHole"];
    let mut body=Body::new();
    let face=build_plane_face_with_hole(&mut body,&mut OpRecorder::new());
    let mesh=run_planar_preview(&body,PlanarPreviewRoot::Face(face)).unwrap();
    let expected_edges=expected["edges"].as_u64().unwrap() as usize;
    assert_eq!(mesh.component_references["edge"].len(),expected_edges);
    assert_eq!(mesh.component_references["vertex"].len(),expected_edges);
    assert!(mesh.edge_ids.iter().filter(|&&pick|pick==4).count()>4);
    let point=|i:u32|{let p=&mesh.positions[i as usize*3..];Pnt3::new(p[0].into(),p[1].into(),p[2].into())};
    let area=mesh.indices.chunks_exact(3).map(|tri|{
        let [a,b,c]=[point(tri[0]),point(tri[1]),point(tri[2])];
        (b-a).cross(c-a).norm()*0.5
    }).sum::<f64>();
    assert!((area-expected["area"].as_f64().unwrap()).abs()<expected["tolerance"].as_f64().unwrap());
}

#[test]
fn original_cylinder_surface_preview_is_finite_and_has_outward_normals() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../📸️representation/🏄️surface/🧪️tests/🧫️fixtures/rational-patch.json")).unwrap();
    let law=&fixture["analyticCylinder"];
    let radius=law["radius"].as_f64().unwrap();
    let tolerance=law["tolerance"].as_f64().unwrap();
    let surface=Surface::Cylinder {frame:Frame3::WORLD,radius};
    let mesh=run_surface_patch_preview(&surface,tolerance);
    let p=|i:u32|{let p=&mesh.positions[i as usize*3..];Pnt3::new(p[0].into(),p[1].into(),p[2].into())};
    let area=mesh.indices.chunks_exact(3).map(|tri|{
        let [a,b,c]=[p(tri[0]),p(tri[1]),p(tri[2])];
        let normal=(b-a).cross(c-a);
        assert!(normal.dot(Vec3::new(a.x,a.y,0.0))>0.0);
        normal.norm()*0.5
    }).sum::<f64>();
    assert!((area-law["area"].as_f64().unwrap()).abs()<tolerance);
    let half=law["height"].as_f64().unwrap()*0.5;
    assert!(mesh.positions.chunks_exact(3).all(|p|(f64::from(p[2]).abs()-half).abs()<1e-6));
}

#[test]
fn original_analytic_preview_picks_preserve_exact_shape_labels(){
    use semio_framework_mesh_engine::MeshData;
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError};
    let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/analytic-picks.json")).unwrap();
    let complete=|job:&mut TessellationJob,step:&dyn Fn(&mut TessellationJob,RetainedCloneGrant)->Result<(Option<MeshData>,RetainedCloneProgress),ValueError>|{
        for turn in 1..2000{
            let d=job.analytic_demand().unwrap();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
                maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
            let before=job.progress();
            assert_eq!(step(job,RetainedCloneGrant{maximum_items:0,..grant}).unwrap(),(None,Default::default()));
            assert_eq!(job.progress(),before);
            let (mesh,receipt)=step(job,grant).unwrap();
            assert_eq!(receipt.copied_items,1);
            assert!(receipt.fits(grant));
            assert_eq!(job.progress().units_done,turn);
            if let Some(mesh)=mesh{mesh.validate_component_references().unwrap();return mesh;}
        }
        panic!("original analytic preview did not complete");
    };

    let curve_label=f["curve"]["label"].as_str().unwrap();
    let raw=curve_label.parse().unwrap();
    let curve=Curve3::Circle{frame:Frame3::WORLD,radius:f["curve"]["radius"].as_f64().unwrap()};
    let mut job=TessellationJob::for_curve_component(&curve,raw,f["curve"]["deflection"].as_f64().unwrap()).unwrap();
    let mesh=complete(&mut job,&|job,grant|job.step_curve_component(&curve,raw,grant));
    assert!(mesh.edge_ids.len()>4);
    assert_eq!(mesh.edge_ids.len(),mesh.edge_positions.len()/6);
    assert!(mesh.edge_ids.iter().all(|&id|id==0));
    assert_eq!(mesh.component_references["edge"],[curve_label]);

    let plane_label=f["plane"]["label"].as_str().unwrap();
    let raw=plane_label.parse().unwrap();
    let mut job=TessellationJob::for_plane_component(0.01,raw);
    let mesh=complete(&mut job,&|job,grant|job.step_plane_component(&Frame3::WORLD,raw,grant));
    assert_eq!(mesh.face_ids.len(),f["plane"]["triangles"].as_u64().unwrap()as usize);
    assert!(mesh.face_ids.iter().all(|&id|id==0));
    assert_eq!(mesh.component_references["face"],[plane_label]);
}

#[test]
fn reflected_analytic_sphere_pick_has_a_consistent_pole_normal(){
    let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/analytic-picks.json")).unwrap();
    let row=&f["reflectedSphere"];
    let radius=row["radius"].as_f64().unwrap();
    let pole:[f32;3]=serde_json::from_value(row["pole"].clone()).unwrap();
    let expected:[f32;3]=serde_json::from_value(row["normal"].clone()).unwrap();
    let surface=Surface::Sphere{frame:Frame3{x:-Vec3::X,..Frame3::WORLD},radius};
    let mesh=run_surface_patch_preview(&surface,0.15);
    mesh.validate_component_references().unwrap();
    assert_eq!(mesh.face_ids.len(),mesh.indices.len()/3);
    assert_eq!(mesh.component_references["face"],["701"]);
    let normals:Vec<_>=mesh.positions.chunks_exact(3).zip(mesh.normals.chunks_exact(3))
        .filter_map(|(point,normal)|(point==pole).then_some(normal)).collect();
    assert!(!normals.is_empty());
    assert!(normals.into_iter().all(|normal|normal==expected));
}
