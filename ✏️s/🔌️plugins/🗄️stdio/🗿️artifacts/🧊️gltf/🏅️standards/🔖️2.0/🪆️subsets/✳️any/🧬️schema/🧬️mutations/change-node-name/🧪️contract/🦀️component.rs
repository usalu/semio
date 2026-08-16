//! 🧪️ Rust executor for the shared change-node-name vector.
#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::change_node_name::{diff, inverse, mutation};
    use crate::artifacts::gltf::GltfSnapshot;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Vector { base: GltfSnapshot, mutation: mutation::GltfChangeNodeNamePayload, diff: diff::GltfChangeNodeNameDiff, inverse: inverse::GltfChangeNodeNameInverse, after: GltfSnapshot }
    #[derive(Deserialize)]
    struct Contract { vectors: Vec<Vector> }

    #[test]
    fn canonical_vector_plans_applies_replays_undoes_and_rejects_forged_paths() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️component.json")).unwrap();
        let vector = &contract.vectors[0];
        let planned = diff::derive(&vector.mutation, &vector.base).unwrap();
        let inverted = inverse::derive(&vector.mutation, &vector.base).unwrap();
        assert_eq!(planned, vector.diff);
        assert_eq!(inverted, vector.inverse);
        let forward = mutation::apply(&vector.mutation, &vector.base).unwrap();
        let applied = diff::apply(&vector.base, &planned).unwrap();
        let replay = diff::apply(&vector.base, &planned).unwrap();
        let mut forged_diff = planned.clone();
        forged_diff.touched_paths = vec!["document/forged".into()];
        let mut forged_inverse = inverted.clone();
        forged_inverse.touched_paths = vec!["document/forged".into()];
        let mut out_of_range = vector.mutation.clone();
        out_of_range.node = vector.base.document.nodes.len();
        assert_eq!(forward, vector.after);
        assert_eq!(applied, vector.after);
        assert_eq!(replay, applied);
        assert_eq!(serde_json::from_slice::<diff::GltfChangeNodeNameDiff>(&diff::encode(&planned).unwrap()).unwrap(), planned);
        assert_eq!(serde_json::from_slice::<inverse::GltfChangeNodeNameInverse>(&inverse::encode(&inverted).unwrap()).unwrap(), inverted);
        assert!(mutation::apply(&vector.mutation, &applied).is_err());
        assert!(mutation::apply(&out_of_range, &vector.base).is_err());
        assert!(diff::apply(&vector.base, &forged_diff).is_err());
        assert!(inverse::apply(&applied, &forged_inverse).is_err());
        assert_eq!(inverse::apply(&applied, &inverted).unwrap(), vector.base);
        assert!(diff::apply(&applied, &planned).is_err());
    }
}
