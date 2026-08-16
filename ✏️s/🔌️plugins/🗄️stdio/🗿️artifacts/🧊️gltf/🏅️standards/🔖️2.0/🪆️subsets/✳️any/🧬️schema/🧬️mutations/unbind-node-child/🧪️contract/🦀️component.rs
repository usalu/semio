//! 🧪️ Rust executor for the shared unbind-node-child vector.
//!
#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::unbind_node_child::{diff, inverse, mutation};
    use crate::artifacts::gltf::GltfSnapshot;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Vector {
        base: GltfSnapshot,
        mutation: mutation::GltfUnbindNodeChildPayload,
        diff: diff::GltfUnbindNodeChildDiff,
        inverse: inverse::GltfUnbindNodeChildInverse,
        after: GltfSnapshot,
        stale: GltfSnapshot,
        rejected_payload: mutation::GltfUnbindNodeChildPayload,
    }

    #[derive(Deserialize)]
    struct Contract {
        vectors: Vec<Vector>,
    }

    #[test]
    fn canonical_vector_plans_removes_replays_rejects_stale_and_undoes_the_exact_edge() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️component.json")).unwrap();
        let vector = &contract.vectors[0];
        let planned = diff::derive(&vector.mutation, &vector.base).unwrap();
        let inverted = inverse::derive(&vector.mutation, &vector.base).unwrap();
        assert_eq!(planned, vector.diff);
        assert_eq!(inverted, vector.inverse);
        assert!(mutation::validate(&vector.rejected_payload, &vector.base).is_err());
        let forward = mutation::apply(&vector.mutation, &vector.base).unwrap();
        let applied = diff::apply(&vector.base, &planned).unwrap();
        let mut forged_diff = planned.clone();
        forged_diff.touched_paths = vec!["document/forged".into()];
        let mut forged_inverse = inverted.clone();
        forged_inverse.touched_paths = vec!["document/forged".into()];
        assert_eq!(forward, vector.after);
        assert_eq!(applied, vector.after);
        assert_eq!(diff::apply(&vector.base, &planned).unwrap(), applied);
        assert!(diff::apply(&vector.base, &forged_diff).is_err());
        assert!(inverse::apply(&applied, &forged_inverse).is_err());
        assert!(diff::apply(&vector.stale, &planned).is_err());
        assert!(inverse::apply(&vector.base, &inverted).is_err());
        assert_eq!(inverse::apply(&applied, &inverted).unwrap(), vector.base);
    }
}
