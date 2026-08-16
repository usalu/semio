//! 🧪️ Rust executor for the shared bind-node-child vector.
//!
#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::bind_node_child::{diff, inverse, mutation};
    use crate::artifacts::gltf::GltfSnapshot;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Vector {
        base: GltfSnapshot,
        mutation: mutation::GltfBindNodeChildPayload,
        diff: diff::GltfBindNodeChildDiff,
        inverse: inverse::GltfBindNodeChildInverse,
        after: GltfSnapshot,
    }

    #[derive(Deserialize)]
    struct Contract {
        vectors: Vec<Vector>,
    }

    #[test]
    fn canonical_vector_plans_applies_replays_and_undoes_the_exact_edge() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️component.json")).unwrap();
        let vector = &contract.vectors[0];
        let planned = diff::derive(&vector.mutation, &vector.base).unwrap();
        let inverted = inverse::derive(&vector.mutation, &vector.base).unwrap();
        assert_eq!(planned, vector.diff);
        assert_eq!(inverted, vector.inverse);
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
        assert!(inverse::apply(&vector.base, &inverted).is_err());
        assert_eq!(inverse::apply(&applied, &inverted).unwrap(), vector.base);
        assert!(diff::apply(&applied, &planned).is_err());
    }
}
