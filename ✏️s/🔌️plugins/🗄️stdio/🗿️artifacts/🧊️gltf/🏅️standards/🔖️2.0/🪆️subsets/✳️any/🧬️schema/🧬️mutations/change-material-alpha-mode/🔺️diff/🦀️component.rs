//! 🔺️ change-material-alpha-mode owns a sparse alpha-mode diff.
use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::mutation::{validate, GltfChangeMaterialAlphaModePayload, GltfChangeMaterialAlphaModeRejection};
use crate::artifacts::gltf::schema::snapshot::GltfAlphaMode;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialAlphaModeDiff {
    pub material: usize,
    pub expected_alpha_mode: GltfAlphaMode,
    pub alpha_mode: GltfAlphaMode,
    pub touched_paths: Vec<String>,
}
impl GltfChangeMaterialAlphaModeDiff {
    pub fn expected_touched_paths(&self) -> Vec<String> {
        vec![format!("document/materials/{}/alphaMode", self.material)]
    }
    pub fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfChangeMaterialAlphaModeRejection> {
        if self.touched_paths != self.expected_touched_paths() {
            return Err(GltfChangeMaterialAlphaModeRejection { code: "gltf.mutation.invalid-touched-paths".into(), path: "diff/touchedPaths".into(), detail: "touched paths must equal the concrete material alpha-mode path".into() });
        }
        let material = snapshot.document.materials.get_mut(self.material).ok_or_else(|| GltfChangeMaterialAlphaModeRejection {
            code: "gltf.mutation.index-out-of-range".into(),
            path: "document/materials".into(),
            detail: "the addressed index must exist".into(),
        })?;
        if material.alpha_mode != self.expected_alpha_mode {
            return Err(GltfChangeMaterialAlphaModeRejection { code: "gltf.mutation.stale-diff".into(), path: format!("document/materials/{}/alphaMode", self.material), detail: "current alpha mode does not equal the planned pre-state".into() });
        }
        material.alpha_mode = self.alpha_mode;
        Ok(())
    }
}
pub fn derive(payload: &GltfChangeMaterialAlphaModePayload, base: &GltfSnapshot) -> Result<GltfChangeMaterialAlphaModeDiff, GltfChangeMaterialAlphaModeRejection> {
    validate(payload, base)?;
    let touched_paths = vec![format!("document/materials/{}/alphaMode", payload.material)];
    Ok(GltfChangeMaterialAlphaModeDiff { material: payload.material, expected_alpha_mode: base.document.materials[payload.material].alpha_mode, alpha_mode: payload.alpha_mode, touched_paths })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_diff_applies_the_canonical_forward_vector() {
        let mut snapshot = GltfSnapshot::default();
        snapshot.document.materials.push(Default::default());
        let payload = GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Mask };
        let diff = derive(&payload, &snapshot).unwrap();
        assert_eq!(diff.material, 0);
        assert_eq!(diff.expected_alpha_mode, GltfAlphaMode::Opaque);
        assert_eq!(diff.alpha_mode, GltfAlphaMode::Mask);
        diff.apply(&mut snapshot).unwrap();
        assert_eq!(snapshot.document.materials[0].alpha_mode, GltfAlphaMode::Mask);
        assert_eq!(diff.apply(&mut snapshot).unwrap_err().code, "gltf.mutation.stale-diff");
    }
}
