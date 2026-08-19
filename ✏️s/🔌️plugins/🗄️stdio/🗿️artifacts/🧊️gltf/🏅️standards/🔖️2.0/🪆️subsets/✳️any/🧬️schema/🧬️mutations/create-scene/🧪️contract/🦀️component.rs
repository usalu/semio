//! 🧪️ Executes create-scene descriptor laws from the canonical JSON vector.

#[cfg(test)]
mod tests {
    use crate::artifacts::gltf::schema::mutations::create_scene::{diff, inverse, mutation, DESCRIPTOR};
    use crate::artifacts::gltf::schema::snapshot::GltfScene;
    use crate::artifacts::gltf::GltfSnapshot;
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Contract {
        id: String,
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
        malformed_payload: EncodedRejection,
        rejections: Rejections,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    struct SceneState {
        scene: Option<usize>,
        scenes: Vec<GltfScene>,
    }

    #[derive(Deserialize)]
    struct EncodedRejection {
        encoded: String,
        code: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Rejection {
        code: String,
        payload: Option<mutation::GltfCreateScenePayload>,
        base: Option<SceneState>,
        scene: Option<GltfScene>,
        touched_paths: Option<Vec<String>>,
        position: Option<u32>,
    }

    #[derive(Default, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Rejections {
        #[serde(default)]
        out_of_range_position: Option<Rejection>,
        #[serde(default)]
        invalid_default_reference: Option<Rejection>,
        #[serde(default)]
        stale_diff_replay: Option<Rejection>,
        #[serde(default)]
        stale_default_scene: Option<Rejection>,
        #[serde(default)]
        stale_insertion_anchor: Option<Rejection>,
        #[serde(default)]
        forged_diff_touched_paths: Option<Rejection>,
        #[serde(default)]
        inverse_index: Option<Rejection>,
        #[serde(default)]
        stale_inverse: Option<Rejection>,
        #[serde(default)]
        stale_inverse_anchor: Option<Rejection>,
        #[serde(default)]
        forged_inverse_touched_paths: Option<Rejection>,
        #[serde(default)]
        stale_distant_diff: Option<Rejection>,
        #[serde(default)]
        stale_distant_inverse: Option<Rejection>,
    }

    async fn snapshot(state: &SceneState) -> GltfSnapshot {
        let mut snapshot = GltfSnapshot::default();
        snapshot.schema = "gltf/2.0".into();
        snapshot.document.scene = state.scene;
        snapshot.document.scenes = state.scenes.clone();
        snapshot
    }

    async fn state(snapshot: &GltfSnapshot) -> SceneState {
        SceneState { scene: snapshot.document.scene, scenes: snapshot.document.scenes.clone() }
    }

    #[semio_framework_async_macros::async_test]
    async fn create_scene_shared_vector_executes_descriptor_and_phase_laws() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️component.json")).expect("canonical vector decodes");
        assert_eq!(contract.id, mutation::ID);
        for vector in &contract.vectors {
            let base = snapshot(&vector.base);
            let direct = mutation::apply(&vector.payload, &base).expect("mutation accepts each canonical vector");
            assert_eq!(state(&direct), vector.after);
            let derived = diff::derive(&base, vector.payload.position).expect("diff derives each canonical vector");
            assert_eq!(serde_json::to_value(&derived).expect("diff serializes"), vector.diff);
            assert_eq!(diff::apply(&derived, &base).expect("diff applies"), direct);
            let derived_inverse = inverse::derive(&base, vector.payload.position).expect("inverse derives each canonical vector");
            assert_eq!(serde_json::to_value(&derived_inverse).expect("inverse serializes"), vector.inverse);
            assert_eq!(state(&inverse::apply(&derived_inverse, &direct).expect("inverse applies")), vector.undo);
        }
        let vector = &contract.vectors[0];
        let base = snapshot(&vector.base);
        let payload = serde_json::to_vec(&vector.payload).expect("payload encodes");

        let malformed = (DESCRIPTOR.plan)(vector.malformed_payload.encoded.as_bytes(), &base).err().expect("malformed payload rejects");
        assert_eq!(malformed.code, vector.malformed_payload.code);

        let direct = mutation::apply(&vector.payload, &base).expect("mutation accepts canonical payload");
        assert_eq!(state(&direct), vector.after);
        let range_rejection = vector.rejections.out_of_range_position.as_ref().expect("range rejection");
        let range = mutation::apply(range_rejection.payload.as_ref().expect("range payload"), &base).expect_err("out-of-range payload rejects");
        assert_eq!(range.code, range_rejection.code);
        let invalid_reference_rejection = vector.rejections.invalid_default_reference.as_ref().expect("invalid reference rejection");
        let invalid_reference = mutation::apply(&vector.payload, &snapshot(invalid_reference_rejection.base.as_ref().expect("invalid reference base"))).expect_err("invalid default scene rejects");
        assert_eq!(invalid_reference.code, invalid_reference_rejection.code);

        let plan = (DESCRIPTOR.plan)(&payload, &base).expect("descriptor plans canonical payload");
        let planned: diff::GltfCreateSceneDiff = serde_json::from_slice(&plan.diff_payload).expect("planned diff decodes");
        let derived = diff::derive(&base, vector.payload.position).expect("direct diff derives");
        assert_eq!(planned, derived);
        assert_eq!(serde_json::to_value(&planned).expect("diff value"), vector.diff);
        assert_eq!(plan.touched_paths, vec!["document/scenes/0", "document/scene"]);
        assert_eq!(diff::touched_paths(&planned, &base).expect("paths recompute"), plan.touched_paths);

        let forward = diff::apply(&planned, &base).expect("direct diff applies");
        assert_eq!(forward, direct);
        let applied = (DESCRIPTOR.apply_diff)(&plan.diff_payload, &base).expect("descriptor applies canonical diff");
        assert_eq!(applied.snapshot, direct);
        assert_eq!(applied.touched_paths, plan.touched_paths);
        let replay = diff::apply(&planned, &direct).expect_err("diff replay rejects");
        assert_eq!(replay.code, vector.rejections.stale_diff_replay.as_ref().expect("replay rejection").code);
        let mut stale_default = base.clone();
        stale_default.document.scene = None;
        let stale_default = diff::apply(&planned, &stale_default).expect_err("changed default scene rejects");
        assert_eq!(stale_default.code, vector.rejections.stale_default_scene.as_ref().expect("default rejection").code);
        let mut stale_anchor = base.clone();
        stale_anchor.document.scenes[0] = vector.rejections.stale_insertion_anchor.as_ref().expect("anchor rejection").scene.clone().expect("anchor scene");
        let stale_anchor = diff::apply(&planned, &stale_anchor).expect_err("changed insertion anchor rejects");
        assert_eq!(stale_anchor.code, vector.rejections.stale_insertion_anchor.as_ref().expect("anchor rejection").code);
        let mut forged_diff = planned.clone();
        forged_diff.touched_paths = vector.rejections.forged_diff_touched_paths.as_ref().expect("forged diff rejection").touched_paths.clone().expect("forged diff paths");
        let forged_diff = (DESCRIPTOR.apply_diff)(&diff::encode(&forged_diff).expect("forged diff encodes"), &base).err().expect("forged diff paths reject");
        assert_eq!(forged_diff.code, vector.rejections.forged_diff_touched_paths.as_ref().expect("forged diff rejection").code);
        assert_eq!(serde_json::from_slice::<Value>(&diff::encode(&planned).expect("diff encodes")).expect("diff json decodes"), vector.diff);

        let planned_inverse: inverse::GltfCreateSceneInverse = serde_json::from_slice(&plan.inverse_payload).expect("planned inverse decodes");
        let derived_inverse = inverse::derive(&base, vector.payload.position).expect("direct inverse derives");
        assert_eq!(planned_inverse, derived_inverse);
        assert_eq!(serde_json::to_value(&planned_inverse).expect("inverse value"), vector.inverse);
        let restored = inverse::apply(&planned_inverse, &direct).expect("direct inverse applies");
        assert_eq!(state(&restored), vector.undo);
        let inverse_application = (DESCRIPTOR.apply_inverse)(&plan.inverse_payload, &direct).expect("descriptor applies inverse");
        assert_eq!(inverse_application.snapshot, restored);
        assert_eq!(inverse_application.touched_paths, plan.touched_paths);
        let inverse_plan = (DESCRIPTOR.plan_inverse)(&plan.inverse_payload, &direct).expect("descriptor plans inverse");
        assert_eq!(serde_json::from_slice::<Value>(&inverse_plan.diff_payload).expect("inverse plan decodes"), vector.inverse);
        assert_eq!(inverse_plan.touched_paths, plan.touched_paths);
        let mut invalid_index = planned_inverse.clone();
        invalid_index.position = vector.rejections.inverse_index.as_ref().expect("inverse index rejection").position.expect("inverse index");
        let invalid_index = inverse::apply(&invalid_index, &direct).expect_err("invalid inverse index rejects");
        assert_eq!(invalid_index.code, vector.rejections.inverse_index.as_ref().expect("inverse index rejection").code);
        let mut stale_inverse = direct.clone();
        stale_inverse.document.scenes[0] = vector.rejections.stale_inverse.as_ref().expect("stale inverse rejection").scene.clone().expect("stale created scene");
        let stale_inverse = inverse::apply(&planned_inverse, &stale_inverse).expect_err("changed created scene rejects");
        assert_eq!(stale_inverse.code, vector.rejections.stale_inverse.as_ref().expect("stale inverse rejection").code);
        let mut stale_inverse_anchor = direct.clone();
        stale_inverse_anchor.document.scenes[1] = vector.rejections.stale_inverse_anchor.as_ref().expect("stale inverse anchor rejection").scene.clone().expect("stale next scene");
        let stale_inverse_anchor = inverse::apply(&planned_inverse, &stale_inverse_anchor).expect_err("changed post-insertion anchor rejects");
        assert_eq!(stale_inverse_anchor.code, vector.rejections.stale_inverse_anchor.as_ref().expect("stale inverse anchor rejection").code);
        let mut forged_inverse = planned_inverse.clone();
        forged_inverse.touched_paths = vector.rejections.forged_inverse_touched_paths.as_ref().expect("forged inverse rejection").touched_paths.clone().expect("forged inverse paths");
        let forged_inverse = (DESCRIPTOR.apply_inverse)(&inverse::encode(&forged_inverse).expect("forged inverse encodes"), &direct).err().expect("forged inverse paths reject");
        assert_eq!(forged_inverse.code, vector.rejections.forged_inverse_touched_paths.as_ref().expect("forged inverse rejection").code);
        assert_eq!(serde_json::from_slice::<Value>(&inverse::encode(&planned_inverse).expect("inverse encodes")).expect("inverse json decodes"), vector.inverse);

        let distant = &contract.vectors[2];
        let distant_base = snapshot(&distant.base);
        let distant_diff = diff::derive(&distant_base, distant.payload.position).expect("distant diff derives");
        let mut stale_distant_base = distant_base.clone();
        *stale_distant_base.document.scenes.last_mut().expect("distant base scene") = distant.rejections.stale_distant_diff.as_ref().expect("distant diff rejection").scene.clone().expect("distant diff scene");
        let stale_distant_diff = diff::apply(&distant_diff, &stale_distant_base).expect_err("distant base edit rejects");
        assert_eq!(stale_distant_diff.code, distant.rejections.stale_distant_diff.as_ref().expect("distant diff rejection").code);
        let distant_after = mutation::apply(&distant.payload, &distant_base).expect("distant mutation applies");
        let distant_inverse = inverse::derive(&distant_base, distant.payload.position).expect("distant inverse derives");
        let mut stale_distant_after = distant_after.clone();
        *stale_distant_after.document.scenes.last_mut().expect("distant after scene") = distant.rejections.stale_distant_inverse.as_ref().expect("distant inverse rejection").scene.clone().expect("distant inverse scene");
        let stale_distant_inverse = inverse::apply(&distant_inverse, &stale_distant_after).expect_err("distant post-state edit rejects");
        assert_eq!(stale_distant_inverse.code, distant.rejections.stale_distant_inverse.as_ref().expect("distant inverse rejection").code);
    }
}
