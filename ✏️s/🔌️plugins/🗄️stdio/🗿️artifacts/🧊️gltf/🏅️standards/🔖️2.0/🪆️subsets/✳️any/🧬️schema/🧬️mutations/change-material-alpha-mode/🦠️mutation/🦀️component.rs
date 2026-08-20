//! 🦠️ change-material-alpha-mode executes one typed alpha-mode mutation.
use crate::artifacts::gltf::schema::mutations::material_animation_private::{index, GltfMaterialAnimationFailure};
use crate::artifacts::gltf::schema::snapshot::GltfAlphaMode;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

pub const ID: &str = "s.stdio.gltf.mutation.change-material-alpha-mode.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials/{material}/alphaMode"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn touched_paths(payload: &GltfChangeMaterialAlphaModePayload) -> Vec<String> {
    vec![format!("document/materials/{}/alphaMode", payload.material)]
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialAlphaModeRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn failure(value: GltfMaterialAnimationFailure) -> GltfChangeMaterialAlphaModeRejection {
    GltfChangeMaterialAlphaModeRejection { code: value.code.into(), path: value.path, detail: value.detail.into() }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialAlphaModePayload {
    pub material: usize,
    pub alpha_mode: GltfAlphaMode,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeMaterialAlphaModePayload, base: &GltfSnapshot) -> Result<(), GltfChangeMaterialAlphaModeRejection> {
    index(&base.document.materials, payload.material, "document/materials").map_err(failure)?;
    (base.document.materials[payload.material].alpha_mode != payload.alpha_mode).then_some(()).ok_or_else(|| GltfChangeMaterialAlphaModeRejection {
        code: "gltf.mutation.no-observable-change".into(),
        path: format!("document/materials/{}/alphaMode", payload.material),
        detail: "alphaMode already has that value".into(),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(snapshot: &mut GltfSnapshot, payload: &GltfChangeMaterialAlphaModePayload) -> Result<(), GltfChangeMaterialAlphaModeRejection> {
    validate(payload, snapshot)?;
    snapshot.document.materials[payload.material].alpha_mode = payload.alpha_mode;
    Ok(())
}
#[cfg(test)]
mod tests {
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
}
