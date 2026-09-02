//! 🧪️ Rust executor for the shared double-sided canonical vector.
#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::change_material_double_sided::{diff, inverse, mutation};
    use crate::artifacts::gltf::GltfSnapshot;
    use std::collections::BTreeMap;
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct State {
        material: usize,
        double_sided: bool,
    }
    #[derive(value_derive::FromValue)]
    struct Vector {
        base: State,
        mutation: mutation::GltfChangeMaterialDoubleSidedPayload,
        diff: diff::GltfChangeMaterialDoubleSidedDiff,
        inverse: inverse::GltfChangeMaterialDoubleSidedInverse,
        after: State,
        undo: State,
        rejections: BTreeMap<String, String>,
    }
    #[derive(value_derive::FromValue)]
    struct Contract {
        vectors: Vec<Vector>,
    }
    #[semio_framework_async_macros::async_test]
    async fn canonical_vector_executes_forward_inverse_stale_and_path_laws() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let vector = &contract.vectors[0];
        assert_eq!(vector.base.material, 0);
        let mut base = GltfSnapshot::default();
        base.document.materials.push(Default::default());
        base.document.materials[0].double_sided = vector.base.double_sided;
        let planned = diff::derive(&vector.mutation, &base).unwrap();
        let inverted = inverse::reconstruct(&vector.mutation, &base).unwrap();
        assert_eq!(planned, vector.diff);
        assert_eq!(inverted, vector.inverse);
        let mut mutation_state = base.clone();
        mutation::apply(&mut mutation_state, &vector.mutation).unwrap();
        assert_eq!(mutation_state.document.materials[0].double_sided, vector.after.double_sided);
        let mut diff_state = base.clone();
        planned.apply(&mut diff_state).unwrap();
        assert_eq!(diff_state.document.materials[0].double_sided, vector.after.double_sided);
        assert_eq!(planned.apply(&mut diff_state).unwrap_err().code, vector.rejections["staleDiff"]);
        let mut forged = planned.clone();
        forged.touched_paths = vec!["document/materials/9/doubleSided".into()];
        assert_eq!(forged.apply(&mut base.clone()).unwrap_err().code, vector.rejections["forgedPath"]);
        inverted.apply(&mut diff_state).unwrap();
        assert_eq!(diff_state.document.materials[0].double_sided, vector.undo.double_sided);
        assert_eq!(inverted.apply(&mut diff_state).unwrap_err().code, vector.rejections["staleInverse"]);
        assert_eq!(serde_json::to_value(&planned).unwrap(), serde_json::to_value(&vector.diff).unwrap());
        assert_eq!(serde_json::to_value(&inverted).unwrap(), serde_json::to_value(&vector.inverse).unwrap());
        assert_eq!(vector.after.material, vector.undo.material);
    }
}
