//! 🧰️ Shared native fixture and retirement grant construction for tessellation tests.

use semio_framework_3d_brep_construction::euler::{add_face,add_shell,add_solid,make_edge,make_vertex};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::{ArenaId,EdgeId,FaceId,SolidId},contract::MeshTransfer};
use semio_framework_3d_brep_representation::topology::{Body,make_loop,history::OpRecorder};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use std::collections::HashMap;
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkPoll,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,controlled::ControlledRetirement}};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn build_unit_box(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
    let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(0.0, 1.0, 1.0)];
    let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
    let edge_pairs = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    let mut edges = HashMap::new();
    for &(a, b) in &edge_pairs {
        let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
        let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
        edges.insert((a, b), edge);
        edges.insert((b, a), edge);
    }
    let face_defs: [([usize; 4], Vec3); 6] = [([0, 3, 2, 1], -Vec3::Z), ([4, 5, 6, 7], Vec3::Z), ([0, 1, 5, 4], -Vec3::Y), ([3, 7, 6, 2], Vec3::Y), ([0, 4, 7, 3], -Vec3::X), ([1, 2, 6, 5], Vec3::X)];
    let mut faces = Vec::new();
    for (corners, normal) in face_defs {
        let frame = Frame3::from_normal(positions[corners[0]], normal).unwrap();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let members: Vec<(EdgeId, bool)> = (0..4)
            .map(|i| {
                let a = corners[i];
                let b = corners[(i + 1) % 4];
                let edge = edges[&(a, b)];
                let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                (edge, forward)
            })
            .collect();
        let outer = make_loop(body, FaceId::from_raw(0, 0), &members);
        let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
        body.loops.get_mut(outer).unwrap().face = face;
        faces.push(face);
    }
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

pub(super) fn exact_retirement_grant<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,items:usize)->semio_framework_value::retained_clone::RetainedCloneGrant {
    let copy=owner.next_copy_byte_demand().unwrap();semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}
}
pub(super) fn small_retirement_grant<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,items:usize,release:usize)->semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:items,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()}
}

pub(super) fn with_retirement_effect<R>(grant: RetainedCloneGrant, step: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    OriginalRetirementPump::new()
        .turn(1 << 16, 1 << 20, || Some(1), grant)
        .and_then(|mut turn| turn.with_effect(step))
        .expect("original retirement authoring")
        .expect("retirement work admitted")
        .0
}

pub(super) fn retire_once<T: RetireOwned>(owner: &mut ControlledRetirement<T>, grant: RetainedCloneGrant) -> RetainedCloneStep {
    match with_retirement_effect(grant, |effect| owner.step(grant.maximum_copy_bytes, effect)) {
        RetainedWorkPoll::Complete(turn) => turn.result.expect("original retirement step"),
        RetainedWorkPoll::Pending { work } => {
            assert_eq!(work, 0, "pending original retirement follows an unfunded quote");
            RetainedCloneStep::Progress(Default::default())
        }
    }
}

pub(super) fn triangle_area_sum(mesh: &MeshTransfer) -> f64 {
    let mut area = 0.0;
    for tri in mesh.index.chunks_exact(3) {
        let p = |i: u32| -> Pnt3 {
            let b = (i as usize) * 3;
            Pnt3::new(mesh.position[b] as f64, mesh.position[b + 1] as f64, mesh.position[b + 2] as f64)
        };
        let (a, b, c) = (p(tri[0]), p(tri[1]), p(tri[2]));
        area += (b - a).cross(c - a).norm() * 0.5;
    }
    area
}

