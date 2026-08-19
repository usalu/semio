//! 🔺️ change-material-double-sided owns a sparse render-state diff.
use crate::artifacts::gltf::schema::mutations::change_material_double_sided::mutation::{validate, GltfChangeMaterialDoubleSidedPayload, GltfChangeMaterialDoubleSidedRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedDiff {
    pub material: usize,
    pub expected_double_sided: bool,
    pub double_sided: bool,
    pub touched_paths: Vec<String>,
}
impl GltfChangeMaterialDoubleSidedDiff {
    pub async fn expected_touched_paths(&self) -> Vec<String> {
        vec![format!("document/materials/{}/doubleSided", self.material)]
    }
    pub async fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfChangeMaterialDoubleSidedRejection> {
        if self.touched_paths != self.expected_touched_paths() {
            return Err(GltfChangeMaterialDoubleSidedRejection { code: "gltf.mutation.invalid-touched-paths".into(), path: "diff/touchedPaths".into(), detail: "touched paths must equal the concrete material double-sided path".into() });
        }
        let material = snapshot.document.materials.get_mut(self.material).ok_or_else(|| GltfChangeMaterialDoubleSidedRejection {
            code: "gltf.mutation.index-out-of-range".into(),
            path: "document/materials".into(),
            detail: "the addressed index must exist".into(),
        })?;
        if material.double_sided != self.expected_double_sided {
            return Err(GltfChangeMaterialDoubleSidedRejection {
                code: "gltf.mutation.stale-diff".into(),
                path: format!("document/materials/{}/doubleSided", self.material),
                detail: "current double-sided value does not equal the planned pre-state".into(),
            });
        }
        material.double_sided = self.double_sided;
        Ok(())
    }
}
pub async fn derive(payload: &GltfChangeMaterialDoubleSidedPayload, base: &GltfSnapshot) -> Result<GltfChangeMaterialDoubleSidedDiff, GltfChangeMaterialDoubleSidedRejection> {
    validate(payload, base)?;
    Ok(GltfChangeMaterialDoubleSidedDiff {
        material: payload.material,
        expected_double_sided: base.document.materials[payload.material].double_sided,
        double_sided: payload.double_sided,
        touched_paths: vec![format!("document/materials/{}/doubleSided", payload.material)],
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn direct_diff_applies_and_rejects_stale_state() {
        let mut snapshot = GltfSnapshot::default();
        snapshot.document.materials.push(Default::default());
        let payload = GltfChangeMaterialDoubleSidedPayload { material: 0, double_sided: true };
        let diff = derive(&payload, &snapshot).unwrap();
        diff.apply(&mut snapshot).unwrap();
        assert!(snapshot.document.materials[0].double_sided);
        assert_eq!(diff.apply(&mut snapshot).unwrap_err().code, "gltf.mutation.stale-diff");
    }
}
