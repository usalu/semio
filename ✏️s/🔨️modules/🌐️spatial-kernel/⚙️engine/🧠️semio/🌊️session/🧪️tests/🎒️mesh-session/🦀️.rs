//! 🎒️ Concrete Semio tessellation cost and cache laws.
use semio_framework_os_flow::mesh::*;
use semio_s_spatial_kernel_semio_session::{Session, TessellationStepOutcome};

#[test]
fn original_session_cold_validation_remains_inside_each_granted_turn() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧊️validation/🔣️.json")).unwrap();
    let session = Session::new();
    let created: serde_json::Value = serde_json::from_str(&session.brep_invoke_json("box", &fixture["box"].to_string())).unwrap();
    let handle = created["handle"].as_str().unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    let mut traces = Vec::new();
    traces.push(session.tessellate_step(handle, tolerance, fixture["probeUnits"].as_u64().unwrap() as usize));
    for _ in 0..fixture["initialTurns"].as_u64().unwrap() { traces.push(session.tessellate_step(handle, tolerance, fixture["turnUnits"].as_u64().unwrap() as usize)); }
    let cancelled = session.cancel_tessellation(handle, tolerance);
    let cancelled_progress = session.tessellation_progress(handle, tolerance);
    let mesh = session.tessellate_geometry(handle, tolerance).unwrap();
    let point = |id: u32| { let i = id as usize * 3; [mesh.positions[i] as f64, mesh.positions[i + 1] as f64, mesh.positions[i + 2] as f64] };
    let mut area = 0.0;
    let mut volume = 0.0;
    for ids in mesh.indices.chunks_exact(3) {
        let [a, b, c] = [point(ids[0]), point(ids[1]), point(ids[2])];
        let u = std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]);
        let v = std::array::from_fn::<_, 3, _>(|i| c[i] - a[i]);
        let cross = [u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0]];
        area += cross.iter().map(|value| value*value).sum::<f64>().sqrt()/2.0;
        volume += (a[0]*(b[1]*c[2]-b[2]*c[1]) + a[1]*(b[2]*c[0]-b[0]*c[2]) + a[2]*(b[0]*c[1]-b[1]*c[0]))/6.0;
    }
    let triangles = mesh.indices.len()/3;
    session.evict_mesh_cache_for_handle(handle);
    session.close();
    assert!(cancelled);
    assert!(cancelled_progress.is_none());
    let mut done = 0;
    for trace in traces {
        match trace {
            TessellationStepOutcome::Working { units_done, units_total, faces_done, faces_total, phase } => {
                assert_eq!(phase, fixture["phase"].as_str().unwrap());
                assert!(units_done >= done && units_done <= done + 1 && units_done <= units_total);
                assert_eq!((faces_done, faces_total), (0, 0));
                done = units_done;
            }
            other => panic!("cold validation left its original funded phase: {other:?}"),
        }
    }
    assert_eq!(triangles, fixture["expected"]["triangles"].as_u64().unwrap() as usize);
    assert!((area - fixture["expected"]["area"].as_f64().unwrap()).abs() < 1e-8);
    assert!((volume - fixture["expected"]["volume"].as_f64().unwrap()).abs() < 1e-8);
    eprintln!("[DEBUG] original Session cold validation turns={done} cancelled={cancelled} triangles={triangles} area={area} volume={volume}");
}
// #region 📏️RealMeshCost
/// ⚖️ LAW: on a REAL tessellated solid (not a toy triangle) the binary body is at least twice as
/// small as the JSON number-array string. Prints the measured bytes per LOD under `--nocapture` —
/// where this ticket's transport numbers come from.
#[test]
fn a_real_tessellated_sphere_halves_the_wire_at_every_lod() {
    let session = Session::new();
    for (lod, tolerance) in [("coarse", 0.15_f64), ("default", 0.05), ("fine", 0.02)] {
        let handle = session.with_kernel(|kernel| {
            use semio_framework_3d::brep::engine::BrepKernel;
            kernel.sphere_prim(1.0).map_err(|error| neural_engine::EvalError::InvalidInput(error.to_string()))
        })
        .expect("sphere");
        let mut steps = 0;
        let mesh = loop {
            match session.tessellate_step(handle.as_str(), tolerance, 8) {
                TessellationStepOutcome::Ready { mesh, .. } => break mesh,
                TessellationStepOutcome::Working { .. } => {
                    steps += 1;
                    assert!(steps < 10_000, "a budgeted tessellation must converge");
                }
                other => panic!("unexpected tessellation outcome: {other:?}"),
            }
        };
        let json_bytes = semio_framework_pack_json::to_json_string(&mesh).len();
        let body = encode_mesh_pack(&mesh).expect("encode");
        let base64 = encode_base64(&body);
        let chunks = chunk_mesh_base64(&base64);
        eprintln!(
            "sphere/{lod} (deflection {tolerance}): {} triangles, {steps} budgeted round trips, json={json_bytes} B, pack={} B, base64={} B, chunks={}",
            mesh.indices.len() / 3,
            body.len(),
            base64.len(),
            chunks.len()
        );
        assert!(body.len() * 2 <= json_bytes, "pack body {} B must at least halve JSON {json_bytes} B", body.len());
        assert_eq!(decode_mesh_pack(&body).expect("decode"), mesh, "a real mesh must round-trip too");
    }
    session.close();
}
// #endregion 📏️RealMeshCost

// #region 🎚️LodCache
/// ⚖️ LAW: a cached mesh at a FINER tolerance answers a coarser request, so switching the LOD mode
/// down never re-runs the tessellator at a tolerance nobody asked for — and a COARSER cached mesh
/// never satisfies a finer request, so quality is never silently downgraded.
#[test]
fn a_finer_cached_mesh_answers_a_coarser_lod_request() {
    let session = Session::new();
    let fixture:serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    let created:serde_json::Value = serde_json::from_str(&session.brep_invoke_json("box", &fixture["box"].to_string())).unwrap();
    let handle = created["handle"].as_str().unwrap();
    let mesh = session.tessellate_geometry(handle,0.02).unwrap();
    assert_eq!(session.cached_mesh_at_or_finer(handle, 0.15), Some(mesh.clone()), "a 0.02 mesh must satisfy a 0.15 request");
    assert_eq!(session.cached_mesh_at_or_finer(handle, 0.05), Some(mesh.clone()), "and a 0.05 one");
    assert_eq!(session.cached_mesh_at_or_finer(handle, 0.02), Some(mesh), "the exact tolerance must hit too");
    assert_eq!(session.cached_mesh_at_or_finer(handle, 0.001), None, "a coarser cached mesh must NOT answer a finer request");
    session.evict_mesh_cache_for_handle(handle);
    session.close();
}
// #endregion 🎚️LodCache
