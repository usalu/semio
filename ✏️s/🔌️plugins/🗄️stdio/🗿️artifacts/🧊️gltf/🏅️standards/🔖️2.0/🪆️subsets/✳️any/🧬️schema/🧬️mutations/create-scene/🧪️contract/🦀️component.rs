//! 🧪️ Executes create-scene laws from the canonical JSON vector.
#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::create_scene::{diff, inverse, mutation};
    use crate::artifacts::gltf::schema::snapshot::GltfScene;
    use crate::artifacts::gltf::GltfSnapshot;
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Contract {
        vectors: Vec<Vector>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Vector {
        base: SceneState,
        payload: mutation::GltfCreateScenePayload,
        after: SceneState,
        undo: SceneState,
        diff: Value,
        inverse: Value,
        rejections: Vec<Rejection>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SceneState {
        scene: usize,
        scenes: Vec<GltfScene>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Rejection {
        payload: Option<mutation::GltfCreateScenePayload>,
        scene: Option<GltfScene>,
        code: String,
    }
    fn snapshot(state: &SceneState) -> GltfSnapshot {
        let mut snapshot = GltfSnapshot::default();
        snapshot.schema = "gltf/2.0".into();
        snapshot.document.scene = Some(state.scene);
        snapshot.document.scenes = state.scenes.clone();
        snapshot
    }

    #[test]
    fn create_scene_shared_vector_executes_all_laws() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️component.json")).expect("canonical vector decodes");
        let vector = &contract.vectors[0];
        let base = snapshot(&vector.base);
        let after = mutation::apply(&vector.payload, &base).expect("mutation accepts vector");
        assert_eq!(after.document.scene, Some(vector.after.scene));
        assert_eq!(after.document.scenes, vector.after.scenes);
        let range = mutation::apply(vector.rejections[0].payload.as_ref().expect("range payload"), &base).expect_err("range payload rejects");
        assert_eq!(range.code, vector.rejections[0].code);
        let forward: diff::GltfCreateSceneDiff = serde_json::from_value(vector.diff.clone()).expect("diff decodes");
        let planned = diff::derive(&base, vector.payload.position).expect("diff derives");
        assert_eq!(planned, forward);
        assert_eq!(planned.touched_paths, vec!["document/scenes/0", "document/scene"]);
        let replay = diff::apply(&planned, &base).expect("diff applies");
        assert_eq!(replay, after);
        let stale_forward = diff::apply(&planned, &after).expect_err("post-state replay rejects");
        assert_eq!(stale_forward.code, vector.rejections[1].code);
        let mut stale_default = base.clone();
        stale_default.document.scene = None;
        let stale_default = diff::apply(&planned, &stale_default).expect_err("default-scene precondition rejects");
        assert_eq!(stale_default.code, vector.rejections[2].code);
        let mut stale_anchor = base.clone();
        stale_anchor.document.scenes[0] = vector.rejections[3].scene.clone().expect("anchor scene");
        let stale_anchor = diff::apply(&planned, &stale_anchor).expect_err("insertion anchor rejects");
        assert_eq!(stale_anchor.code, vector.rejections[3].code);
        assert_eq!(serde_json::from_slice::<Value>(&diff::encode(&planned).expect("diff encodes")).expect("encoded diff decodes"), vector.diff);
        let undo: inverse::GltfCreateSceneInverse = serde_json::from_value(vector.inverse.clone()).expect("inverse decodes");
        let inverted = inverse::derive(&base, vector.payload.position).expect("inverse derives");
        assert_eq!(inverted, undo);
        let restored = inverse::apply(&inverted, &after).expect("inverse applies");
        assert_eq!(restored.document.scene, Some(vector.undo.scene));
        assert_eq!(restored.document.scenes, vector.undo.scenes);
        assert_eq!(serde_json::from_slice::<Value>(&inverse::encode(&inverted).expect("inverse encodes")).expect("encoded inverse decodes"), vector.inverse);
        let mut stale_after = after.clone();
        stale_after.document.scenes[0] = vector.rejections[4].scene.clone().expect("stale scene");
        let stale = inverse::apply(&inverted, &stale_after).expect_err("stale inverse rejects");
        assert_eq!(stale.code, vector.rejections[4].code);
    }
}
