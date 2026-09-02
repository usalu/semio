//! 🧪️ Rust executor for the shared unbind-scene-root-node vectors.
#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::unbind_scene_root_node::{diff, inverse, mutation};
    use crate::artifacts::gltf::GltfSnapshot;

    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Rejected {
        index: mutation::GltfUnbindSceneRootNodePayload,
        reference: mutation::GltfUnbindSceneRootNodePayload,
    }
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Wire {
        malformed_payload: String,
        mutation: String,
        diff: String,
        inverse: String,
    }
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Vector {
        base: GltfSnapshot,
        mutation: mutation::GltfUnbindSceneRootNodePayload,
        diff: diff::GltfUnbindSceneRootNodeDiff,
        inverse: inverse::GltfUnbindSceneRootNodeInverse,
        after: GltfSnapshot,
        stale_diff: GltfSnapshot,
        stale_inverse: GltfSnapshot,
        rejected: Rejected,
        wire: Wire,
    }
    #[derive(value_derive::FromValue)]
    struct Contract {
        vectors: Vec<Vector>,
    }

    #[semio_framework_async_macros::async_test]
    async fn canonical_vector_enforces_forward_inverse_reference_stale_and_wire_laws() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let vector = &contract.vectors[0];
        assert_eq!(serde_json::to_string(&vector.mutation).unwrap(), vector.wire.mutation);
        assert_eq!(serde_json::to_string(&vector.diff).unwrap(), vector.wire.diff);
        assert_eq!(serde_json::to_string(&vector.inverse).unwrap(), vector.wire.inverse);
        assert!(serde_json::from_str::<mutation::GltfUnbindSceneRootNodePayload>(&vector.wire.malformed_payload).is_err());
        assert!((super::super::DESCRIPTOR.plan)(vector.wire.malformed_payload.as_bytes(), &vector.base).is_err());
        assert_eq!(mutation::validate(&vector.rejected.index, &vector.base).unwrap_err().code, "gltf.mutation.index-out-of-range");
        assert_eq!(mutation::validate(&vector.rejected.reference, &vector.base).unwrap_err().code, "gltf.mutation.index-out-of-range");
        let planned = diff::derive(&vector.mutation, &vector.base).unwrap();
        let inverted = inverse::derive(&vector.mutation, &vector.base).unwrap();
        assert_eq!(planned, vector.diff);
        assert_eq!(inverted, vector.inverse);
        let forward = mutation::apply(&vector.mutation, &vector.base).unwrap();
        let applied = diff::apply(&vector.base, &planned).unwrap();
        assert_eq!(forward, vector.after);
        assert_eq!(applied, vector.after);
        let inverse_plan = (super::super::DESCRIPTOR.plan_inverse)(vector.wire.inverse.as_bytes(), &applied).unwrap();
        assert_eq!(inverse_plan.diff_payload, vector.wire.inverse.as_bytes());
        assert!(inverse_plan.inverse_payload.is_empty());
        let mut forged_diff = planned.clone();
        forged_diff.touched_paths = vec!["document/forged".into()];
        let mut forged_inverse = inverted.clone();
        forged_inverse.touched_paths = vec!["document/forged".into()];
        assert_eq!(diff::apply(&vector.stale_diff, &planned).unwrap_err().code, "gltf.mutation.stale-diff");
        assert_eq!(inverse::apply(&vector.stale_inverse, &inverted).unwrap_err().code, "gltf.mutation.stale-inverse");
        assert!(diff::apply(&vector.base, &forged_diff).is_err());
        assert!(inverse::apply(&applied, &forged_inverse).is_err());
        assert!(inverse::apply(&vector.base, &inverted).is_err());
        assert_eq!(inverse::apply(&applied, &inverted).unwrap(), vector.base);
        assert!(diff::apply(&applied, &planned).is_err());
    }
}
