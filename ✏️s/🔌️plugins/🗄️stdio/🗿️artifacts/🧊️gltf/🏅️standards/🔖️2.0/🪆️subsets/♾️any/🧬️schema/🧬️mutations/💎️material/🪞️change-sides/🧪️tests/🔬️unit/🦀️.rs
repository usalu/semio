use super::*;
#[semio_framework_async_macros::async_test]
async fn applies_and_rejects_identity() {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.materials.push(Default::default());
    let payload = GltfChangeMaterialDoubleSidedPayload { material: 0, double_sided: true };
    apply(&mut snapshot, &payload).unwrap();
    assert!(snapshot.document.materials[0].double_sided);
    assert!(apply(&mut snapshot, &payload).is_err());
}
