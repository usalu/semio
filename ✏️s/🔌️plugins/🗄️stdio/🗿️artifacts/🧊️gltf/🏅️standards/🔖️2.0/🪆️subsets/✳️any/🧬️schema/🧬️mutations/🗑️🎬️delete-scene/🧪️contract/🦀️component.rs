//! 🧪️ Executes delete-scene laws from the canonical JSON vector.
#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::delete_scene::{diff, inverse, mutation};
    use crate::artifacts::gltf::schema::snapshot::GltfScene;
    use crate::artifacts::gltf::GltfSnapshot;
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Contract {
        vectors: Vec<Vector>,
    }
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Vector {
        base: SceneState,
        payload: mutation::GltfDeleteScenePayload,
        after: SceneState,
        undo: SceneState,
        diff: Value,
        inverse: Value,
        rejections: Vec<Rejection>,
    }
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct SceneState {
        scene: usize,
        scenes: Vec<GltfScene>,
    }
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Rejection {
        payload: Option<mutation::GltfDeleteScenePayload>,
        base: Option<SceneState>,
        default_scene_after: Option<usize>,
        code: String,
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot(state: &SceneState) -> GltfSnapshot {
        let mut snapshot = GltfSnapshot::default();
        snapshot.schema = "gltf/2.0".into();
        snapshot.document.scene = Some(state.scene);
        snapshot.document.scenes = state.scenes.clone();
        snapshot
    }
    #[semio_framework_async_macros::async_test]
    async fn delete_scene_shared_vector_executes_all_laws() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️component.json")).expect("canonical vector decodes");
        let vector = &contract.vectors[0];
        let base = snapshot(&vector.base);
        let after = mutation::apply(&vector.payload, &base).expect("mutation accepts vector");
        assert_eq!(after.document.scene, Some(vector.after.scene));
        assert_eq!(after.document.scenes, vector.after.scenes);
        let range = mutation::apply(vector.rejections[0].payload.as_ref().expect("range payload"), &base).expect_err("range rejects");
        assert_eq!(range.code, vector.rejections[0].code);
        let dangling = mutation::apply(vector.rejections[1].payload.as_ref().expect("dangling payload"), &snapshot(vector.rejections[1].base.as_ref().expect("dangling base"))).expect_err("dangling default rejects");
        assert_eq!(dangling.code, vector.rejections[1].code);
        let forward: diff::GltfDeleteSceneDiff = serde_json::from_value(vector.diff.clone()).expect("diff decodes");
        let planned = diff::derive(&base, vector.payload.index).expect("diff derives");
        assert_eq!(planned, forward);
        assert_eq!(planned.touched_paths, vec!["document/scenes/0", "document/scene"]);
        assert_eq!(diff::apply(&planned, &base).expect("diff applies"), after);
        assert_eq!(serde_json::from_slice::<Value>(&diff::encode(&planned).expect("diff encodes")).expect("encoded diff decodes"), vector.diff);
        let undo: inverse::GltfDeleteSceneInverse = serde_json::from_value(vector.inverse.clone()).expect("inverse decodes");
        let inverted = inverse::derive(&base, vector.payload.index).expect("inverse derives");
        assert_eq!(inverted, undo);
        let restored = inverse::apply(&inverted, &after).expect("inverse applies");
        assert_eq!(restored.document.scene, Some(vector.undo.scene));
        assert_eq!(restored.document.scenes, vector.undo.scenes);
        assert_eq!(serde_json::from_slice::<Value>(&inverse::encode(&inverted).expect("inverse encodes")).expect("encoded inverse decodes"), vector.inverse);
        let mut stale_after = after.clone();
        stale_after.document.scene = vector.rejections[2].default_scene_after;
        let stale = inverse::apply(&inverted, &stale_after).expect_err("stale inverse rejects");
        assert_eq!(stale.code, vector.rejections[2].code);
    }
}
