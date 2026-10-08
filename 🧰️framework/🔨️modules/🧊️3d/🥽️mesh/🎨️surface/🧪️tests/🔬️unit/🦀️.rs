use super::*;

fn drain(mut job: MeshSurfaceJob, budget: usize) -> (HalfedgeMesh, Vec<usize>) {
    let mut done = Vec::new();
    loop {
        match job.step(budget).unwrap() {
            MeshModelingStep::Working(progress) => {
                assert!(progress.units_done <= progress.units_total);
                done.push(progress.units_done);
            }
            MeshModelingStep::Done(mesh) => return (mesh, done),
            MeshModelingStep::Cancelled(_) => panic!("cancelled"),
        }
    }
}

fn grid(cells: usize, size: f32) -> HalfedgeMesh {
    let side = cells + 1;
    let positions: Vec<[f32; 3]> = (0..side * side).map(|index| [(index % side) as f32 * size, (index / side) as f32 * size, 0.0]).collect();
    let faces: Vec<Vec<u32>> = (0..cells * cells)
        .map(|index| {
            let (column, row) = (index % cells, index / cells);
            let origin = (row * side + column) as u32;
            vec![origin, origin + 1, origin + side as u32 + 1, origin + side as u32]
        })
        .collect();
    HalfedgeMesh::from_faces(&positions, &faces).unwrap()
}

fn corner_uvs(mesh: &HalfedgeMesh) -> Vec<Vec<(u32, [f32; 2])>> {
    (0..mesh.face_count())
        .map(|face| mesh.face_halfedge_ids(FaceId(face as u32)).unwrap().into_iter().map(|halfedge| (mesh.halfedges[halfedge as usize].vertex, mesh.corner_uv(EdgeId(halfedge)).unwrap())).collect())
        .collect()
}

#[test]
fn shading_job_marks_faces_and_rebuilds_vertex_normals_in_bounded_units() {
    let icosahedron = HalfedgeMesh::ico_sphere_prim(1.0, 0).unwrap();
    let all: Vec<FaceId> = (0..icosahedron.face_count() as u32).map(FaceId).collect();
    let (smooth, progress) = drain(icosahedron.set_shading_job(&all, true).unwrap(), 1);
    assert!(progress.windows(2).all(|pair| pair[0] < pair[1]));
    for vertex in 0..smooth.vertex_count() {
        let position = smooth.vertex_position(VertexId(vertex as u32)).unwrap().normalize();
        assert!(smooth.vertex_normal(VertexId(vertex as u32)).unwrap().dot(position) > 0.9999);
    }
    let (flat, _) = drain(smooth.set_shading_job(&all, false).unwrap(), 1024);
    for face in 0..flat.face_count() {
        let normal = flat.face_normal(FaceId(face as u32)).unwrap();
        let first = flat.face_vertex_ids(FaceId(face as u32)).unwrap()[0];
        let owner = (0..flat.face_count()).find(|candidate| flat.face_vertex_ids(FaceId(*candidate as u32)).unwrap().contains(&first)).unwrap();
        let expected = flat.face_normal(FaceId(owner as u32)).unwrap();
        assert!(flat.vertex_normal(first).unwrap().dot(expected) > 0.9999);
        assert!(normal.length() > 0.99);
    }
    assert!(matches!(icosahedron.set_shading_job(&[], true), Err(MeshKernelError::EmptySelection)));
    assert_eq!(icosahedron.set_shading_job(&[FaceId(9999)], true).unwrap().finish_with_progress().unwrap_err(), MeshKernelError::InvalidHandle);
}

#[test]
fn normals_job_equals_the_synchronous_rebuild() {
    let mut mesh = HalfedgeMesh::ico_sphere_prim(2.0, 2).unwrap();
    mesh.set_shading(&[FaceId(0), FaceId(3), FaceId(7)], true).unwrap();
    let (job, _) = drain(mesh.recompute_normals_job().unwrap(), 3);
    let mut sync = mesh.clone();
    sync.recompute_normals().unwrap();
    for vertex in 0..mesh.vertex_count() {
        assert_eq!(job.vertex_normal(VertexId(vertex as u32)).unwrap(), sync.vertex_normal(VertexId(vertex as u32)).unwrap());
    }
}

#[test]
fn seam_job_marks_an_edge_and_its_twin_as_one_seam() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let twin = mesh.halfedges[0].twin.unwrap();
    let (marked, _) = drain(mesh.set_uv_seams_job(&[EdgeId(twin)], true).unwrap(), 1);
    assert!(marked.is_uv_seam(EdgeId(0)) && marked.is_uv_seam(EdgeId(twin)));
    let (cleared, _) = drain(marked.set_uv_seams_job(&[EdgeId(0)], false).unwrap(), 5);
    assert!(!cleared.is_uv_seam(EdgeId(0)) && !cleared.is_uv_seam(EdgeId(twin)));
    assert!(matches!(mesh.set_uv_seams_job(&[], true), Err(MeshKernelError::EmptySelection)));
    assert_eq!(mesh.set_uv_seams_job(&[EdgeId(10_000)], true).unwrap().finish_with_progress().unwrap_err(), MeshKernelError::InvalidHandle);
}

#[test]
fn unwrapping_a_flat_grid_is_a_similarity_of_the_surface() {
    let mesh = grid(3, 2.0);
    let (unwrapped, _) = drain(mesh.unwrap_uv_job().unwrap(), 7);
    let mut uv = BTreeMap::new();
    for face in corner_uvs(&unwrapped) {
        for (vertex, coordinate) in face {
            assert_eq!(*uv.entry(vertex).or_insert(coordinate), coordinate);
        }
    }
    assert_eq!(uv.len(), 16);
    let mut ratios = Vec::new();
    for a in 0..16u32 {
        for b in a + 1..16 {
            let world = mesh.vertex_position(VertexId(a)).unwrap().sub(mesh.vertex_position(VertexId(b)).unwrap()).length();
            let texture = ((uv[&a][0] - uv[&b][0]).powi(2) + (uv[&a][1] - uv[&b][1]).powi(2)).sqrt();
            ratios.push(texture / world);
        }
    }
    let mean = ratios.iter().sum::<f32>() / ratios.len() as f32;
    assert!(ratios.iter().all(|ratio| (ratio - mean).abs() < 2e-3 * mean), "{ratios:?}");
    let high = uv.values().fold([0.0f32; 2], |high, uv| [high[0].max(uv[0]), high[1].max(uv[1])]);
    assert!((high[0].max(high[1]) - 1.0).abs() < 1e-5);
    assert!(uv.values().all(|uv| uv[0] >= 0.0 && uv[1] >= 0.0));
}

#[test]
fn unwrapping_a_box_cut_along_every_edge_packs_six_equal_disjoint_squares() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 2.0, 3.0).unwrap();
    let all: Vec<EdgeId> = (0..mesh.halfedge_count() as u32).map(EdgeId).collect();
    mesh.mark_uv_seam(&all, true);
    let (unwrapped, _) = drain(mesh.unwrap_uv_job().unwrap(), 11);
    let faces = corner_uvs(&unwrapped);
    let boxes: Vec<([f32; 2], [f32; 2])> = faces
        .iter()
        .map(|face| face.iter().fold(([f32::MAX; 2], [f32::MIN; 2]), |(low, high), (_, uv)| ([low[0].min(uv[0]), low[1].min(uv[1])], [high[0].max(uv[0]), high[1].max(uv[1])])))
        .collect();
    let mut ratios = Vec::new();
    for (face, (low, high)) in boxes.iter().enumerate() {
        assert!(high[0] > low[0] && high[1] > low[1] && high[0] <= 1.0 + 1e-6 && high[1] <= 1.0 + 1e-6);
        let world: Vec<Vec3> = unwrapped.face_vertex_ids(FaceId(face as u32)).unwrap().iter().map(|vertex| unwrapped.vertex_position(*vertex).unwrap()).collect();
        for corner in 0..4 {
            let next = (corner + 1) % 4;
            let texture = ((faces[face][corner].1[0] - faces[face][next].1[0]).powi(2) + (faces[face][corner].1[1] - faces[face][next].1[1]).powi(2)).sqrt();
            ratios.push(texture / world[corner].sub(world[next]).length());
        }
    }
    let mean = ratios.iter().sum::<f32>() / ratios.len() as f32;
    assert!(ratios.iter().all(|ratio| (ratio - mean).abs() < 2e-3 * mean), "{ratios:?}");
    for a in 0..boxes.len() {
        for b in a + 1..boxes.len() {
            let apart = boxes[a].1[0] <= boxes[b].0[0] || boxes[b].1[0] <= boxes[a].0[0] || boxes[a].1[1] <= boxes[b].0[1] || boxes[b].1[1] <= boxes[a].0[1];
            assert!(apart, "faces {a} and {b} overlap in texture space");
        }
    }
}

#[test]
fn unwrapping_is_deterministic_and_independent_of_the_step_budget() {
    let mut mesh = HalfedgeMesh::cylinder_prim(1.0, 2.0, 12).unwrap();
    mesh.mark_uv_seam(&[EdgeId(0)], true);
    let bits = |mesh: &HalfedgeMesh| corner_uvs(mesh).into_iter().flatten().flat_map(|(_, uv)| [uv[0].to_bits(), uv[1].to_bits()]).collect::<Vec<_>>();
    let (small, progress) = drain(mesh.unwrap_uv_job().unwrap(), 1);
    let (large, _) = drain(mesh.unwrap_uv_job().unwrap(), 1_000_000);
    let (again, _) = drain(mesh.unwrap_uv_job().unwrap(), 5);
    assert_eq!(bits(&small), bits(&large));
    assert_eq!(bits(&small), bits(&again));
    assert!(progress.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(bits(&small).iter().all(|bits| f32::from_bits(*bits).is_finite()));
}

#[test]
fn unwrapping_a_closed_mesh_without_seams_stays_finite_and_bounded() {
    let (sphere, _) = drain(HalfedgeMesh::ico_sphere_prim(1.0, 2).unwrap().unwrap_uv_job().unwrap(), 64);
    for (_, uv) in corner_uvs(&sphere).into_iter().flatten() {
        assert!(uv[0].is_finite() && uv[1].is_finite() && (0.0..=1.0).contains(&uv[0]) && (0.0..=1.0).contains(&uv[1]));
    }
}

#[test]
fn a_cancelled_job_retires_and_a_finished_job_cannot_run_again() {
    let mut job = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap().unwrap_uv_job().unwrap();
    assert!(matches!(job.step(1).unwrap(), MeshModelingStep::Working(_)));
    job.cancel();
    assert!(matches!(job.step(1).unwrap(), MeshModelingStep::Cancelled(_)));
    assert!(job.step(1).is_err());
}

#[test]
fn unwrapping_refuses_an_authored_uv_channel_and_an_empty_mesh() {
    let mut mesh = HalfedgeMesh::plane_prim(1.0, 1.0).unwrap();
    use protocol::value::DslValue;
    let values = vec![DslValue::Array(vec![DslValue::float(0.0), DslValue::float(0.0)]); 4];
    mesh.set_attribute("uv".into(), MeshAttribute { domain: MeshAttributeDomain::Vertex, semantic: MeshAttributeSemantic::Uv, interpolation: MeshAttributeInterpolation::Linear, values, indices: None }).unwrap();
    assert!(matches!(mesh.unwrap_uv_job(), Err(MeshKernelError::InvalidInput(_))));
    assert!(matches!(HalfedgeMesh::empty().unwrap_uv_job(), Err(MeshKernelError::EmptySelection)));
}
