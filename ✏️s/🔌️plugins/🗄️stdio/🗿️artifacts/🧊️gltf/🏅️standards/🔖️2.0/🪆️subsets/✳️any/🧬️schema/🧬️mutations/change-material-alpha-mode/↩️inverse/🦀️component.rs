//! ↩️ change-material-alpha-mode reconstructs the prior alpha-mode value.
use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::mutation::{validate, GltfChangeMaterialAlphaModePayload, GltfChangeMaterialAlphaModeRejection};
use crate::artifacts::gltf::schema::snapshot::GltfAlphaMode;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialAlphaModeInverse {
    pub material: usize,
    pub expected_alpha_mode: GltfAlphaMode,
    pub alpha_mode: GltfAlphaMode,
    pub touched_paths: Vec<String>,
}
impl GltfChangeMaterialAlphaModeInverse {
    pub fn expected_touched_paths(&self) -> Vec<String> {
        vec![format!("document/materials/{}/alphaMode", self.material)]
    }
    pub fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfChangeMaterialAlphaModeRejection> {
        if self.touched_paths != self.expected_touched_paths() {
            return Err(GltfChangeMaterialAlphaModeRejection { code: "gltf.mutation.invalid-touched-paths".into(), path: "inverse/touchedPaths".into(), detail: "touched paths must equal the concrete material alpha-mode path".into() });
        }
        let material = snapshot.document.materials.get_mut(self.material).ok_or_else(|| GltfChangeMaterialAlphaModeRejection {
            code: "gltf.mutation.index-out-of-range".into(),
            path: "document/materials".into(),
            detail: "the addressed index must exist".into(),
        })?;
        if material.alpha_mode != self.expected_alpha_mode {
            return Err(GltfChangeMaterialAlphaModeRejection {
                code: "gltf.mutation.stale-inverse".into(),
                path: format!("document/materials/{}/alphaMode", self.material),
                detail: "current alpha mode does not equal the planned forward result".into(),
            });
        }
        material.alpha_mode = self.alpha_mode;
        Ok(())
    }
}
pub fn reconstruct(payload: &GltfChangeMaterialAlphaModePayload, base: &GltfSnapshot) -> Result<GltfChangeMaterialAlphaModeInverse, GltfChangeMaterialAlphaModeRejection> {
    validate(payload, base)?;
    let touched_paths = vec![format!("document/materials/{}/alphaMode", payload.material)];
    Ok(GltfChangeMaterialAlphaModeInverse { material: payload.material, expected_alpha_mode: payload.alpha_mode, alpha_mode: base.document.materials[payload.material].alpha_mode, touched_paths })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inverse_reconstructs_the_prior_value() {
        let mut base = GltfSnapshot::default();
        base.document.materials.push(Default::default());
        let payload = GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Blend };
        let inverse = reconstruct(&payload, &base).unwrap();
        crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::mutation::apply(&mut base, &payload).unwrap();
        inverse.apply(&mut base).unwrap();
        assert_eq!(base.document.materials[0].alpha_mode, GltfAlphaMode::Opaque);
        assert_eq!(inverse.apply(&mut base).unwrap_err().code, "gltf.mutation.stale-inverse");
    }
}
