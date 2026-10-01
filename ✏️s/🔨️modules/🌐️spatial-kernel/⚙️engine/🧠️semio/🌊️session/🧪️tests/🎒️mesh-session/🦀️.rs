//! 🎒️ Concrete Semio tessellation cost and cache laws.
use semio_framework_os_flow::mesh::*;
use semio_s_spatial_kernel_semio_session::{Session, TessellationStepOutcome};
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
        let json_bytes = semio_framework_os_flow::os_pack::json::to_json_string(&mesh).len();
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
