//! 🦠️ change-material-double-sided executes one typed render-state mutation.
use crate::artifacts::gltf::schema::mutations::material_animation_private::{index, GltfMaterialAnimationFailure};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.change-material-double-sided.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials/{material}/doubleSided"];
pub async fn touched_paths(payload: &GltfChangeMaterialDoubleSidedPayload) -> Vec<String> {
    vec![format!("document/materials/{}/doubleSided", payload.material)]
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}
async fn failure(value: GltfMaterialAnimationFailure) -> GltfChangeMaterialDoubleSidedRejection {
    GltfChangeMaterialDoubleSidedRejection { code: value.code.into(), path: value.path, detail: value.detail.into() }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedPayload {
    pub material: usize,
    pub double_sided: bool,
}
pub async fn validate(payload: &GltfChangeMaterialDoubleSidedPayload, base: &GltfSnapshot) -> Result<(), GltfChangeMaterialDoubleSidedRejection> {
    index(&base.document.materials, payload.material, "document/materials").map_err(failure)?;
    (base.document.materials[payload.material].double_sided != payload.double_sided).then_some(()).ok_or_else(|| GltfChangeMaterialDoubleSidedRejection {
        code: "gltf.mutation.no-observable-change".into(),
        path: format!("document/materials/{}/doubleSided", payload.material),
        detail: "doubleSided already has that value".into(),
    })
}
pub async fn apply(snapshot: &mut GltfSnapshot, payload: &GltfChangeMaterialDoubleSidedPayload) -> Result<(), GltfChangeMaterialDoubleSidedRejection> {
    validate(payload, snapshot)?;
    snapshot.document.materials[payload.material].double_sided = payload.double_sided;
    Ok(())
}
#[cfg(test)]
mod tests {
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
}
