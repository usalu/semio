
use super::*;
#[semio_framework_async_macros::async_test]
async fn changes_only_alpha_mode_and_rejects_identity() {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.materials.push(Default::default());
    let payload = GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Mask };
    apply(&mut snapshot, &payload).unwrap();
    assert_eq!(snapshot.document.materials[0].alpha_mode, GltfAlphaMode::Mask);
    assert!(apply(&mut snapshot, &payload).is_err());
}
