use super::super::{
    area_volume::contact_area,
    clearance::{interference_volume, overlap_volume},
};
use super::*;

fn box_part(lo: V3, hi: V3) -> RawPart {
    RawPart {
        address: GltfEntityAddress { scope: GltfEntityScope::Primitive, scene: Some(0), node_path: vec![0], mesh: Some(0), primitive: Some(0), component: None, surface_region: None, content_fingerprint: "pair-geometry-test".into() },
        name: None,
        points: [lo[0], hi[0]].into_iter().flat_map(|x| [lo[1], hi[1]].into_iter().flat_map(move |y| [lo[2], hi[2]].into_iter().map(move |z| [x, y, z]))).collect(),
        triangles: Vec::new(),
        diagnostic_ids: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn pair_geometry_preserves_contact_distance_and_box_overlap_leaf_semantics() {
    let first = box_part([0.0, 0.0, 0.0], [4.0, 3.0, 3.0]);
    let contact = box_part([4.0, 0.0, 0.0], [5.0, 3.0, 3.0]);
    let overlapping = box_part([2.0, 1.0, -1.0], [5.0, 5.0, 4.0]);
    let separated = box_part([6.0, 0.0, 0.0], [8.0, 3.0, 3.0]);

    let contact_pair = pair_geometry(&first, &contact, &policy()).expect("finite contact boxes");
    let overlapping_pair = pair_geometry(&first, &overlapping, &policy()).expect("finite overlapping boxes");
    let separated_pair = pair_geometry(&first, &separated, &policy()).expect("finite separated boxes");

    assert_eq!(contact_pair.minimum_distance, 0.0);
    assert_eq!(contact_pair.contact_area, Some(9.0));
    assert_eq!(contact_area::infer_pair(&contact_pair).quality.method, GltfComputationMethod::DeterministicEstimate);
    assert_eq!(overlap_volume::infer_pair(&overlapping_pair).value, Some(12.0));
    assert_eq!(interference_volume::infer_pair(&overlapping_pair).value, Some(12.0));
    assert_eq!(overlap_volume::infer_pair(&separated_pair).value, Some(0.0));
    assert_eq!(interference_volume::infer_pair(&separated_pair).value, Some(0.0));
    assert_eq!(overlap_volume::infer_pair(&overlapping_pair).quality.method, GltfComputationMethod::DeterministicEstimate);
    assert_eq!(separated_pair.minimum_distance, 2.0);
    assert_eq!(contact_area::infer_pair(&separated_pair).quality.method, GltfComputationMethod::Exact);
}
