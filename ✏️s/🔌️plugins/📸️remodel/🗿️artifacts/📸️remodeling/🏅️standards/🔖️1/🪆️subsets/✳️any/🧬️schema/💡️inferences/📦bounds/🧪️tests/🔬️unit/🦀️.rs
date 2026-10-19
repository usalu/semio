use super::*;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::mint_and_stash_mesh;
use semio_s_artifact_remodel_remodeling_io::standards::v1::subsets::any::io::seed_remodeling_mesh;
use semio_framework::MeshData;

#[semio_framework_async_macros::async_test]
async fn empty_mesh_yields_a_zero_bounds() {
    let bounds = compute_remodeling_bounds(&RemodelingSnapshot::default());
    assert_eq!(bounds, RemodelingBounds::default());
}

#[semio_framework_async_macros::async_test]
async fn a_single_triangle_bounds_and_counts_exactly() {
    let mut snapshot = RemodelingSnapshot::default();
    seed_remodeling_mesh(&mut snapshot, &MeshData { positions: vec![-1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0], indices: vec![0, 1, 2], ..MeshData::default() }).expect("a single triangle is inside the bounded envelope");
    let bounds = compute_remodeling_bounds(&snapshot);
    assert_eq!(bounds.bounding_box.min, [-1.0, 0.0, 0.0]);
    assert_eq!(bounds.bounding_box.max, [1.0, 2.0, 0.0]);
    assert_eq!(bounds.vertex_count, 3);
    assert_eq!(bounds.face_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn bounds_is_deterministic() {
    let mut snapshot = RemodelingSnapshot::default();
    snapshot.results.mesh.mesh = mint_and_stash_mesh(MeshData { positions: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], ..MeshData::default() });
    assert_eq!(compute_remodeling_bounds(&snapshot), compute_remodeling_bounds(&snapshot));
}
