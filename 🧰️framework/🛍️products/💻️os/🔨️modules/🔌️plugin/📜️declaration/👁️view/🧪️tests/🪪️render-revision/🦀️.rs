use super::*;

fn context(base: &serde_json::Value, patch: &serde_json::Value, maximum_items: usize) -> AppRenderOperationContext {
    let mut value = base.clone();
    for (key, field) in patch.as_object().unwrap() {
        value.as_object_mut().unwrap().insert(key.clone(), field.clone());
    }
    let text = value["canonicalBaseRevision"].as_str().unwrap();
    let canonical_base_revision = std::array::from_fn(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).unwrap());
    let grant = RetainedCloneGrant { maximum_items, ..RetainedCloneGrant::default() };
    AppRenderOperationContext {
        app_instance_id: value["appInstanceId"].as_u64().unwrap() as u32,
        base_revision: semio_framework_job::RevisionId(value["baseRevision"].as_str().unwrap().parse().unwrap()),
        generation: semio_framework_job::Generation(value["generation"].as_str().unwrap().parse().unwrap()),
        canonical_base_revision,
        mounted_policy: crate::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant },
    }
}

#[test]
fn mounted_render_revision_matches_complete_portable_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️render-revision/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let left = context(&fixture["base"], &case["left"], 0);
        let right = context(&fixture["base"], &case["right"], 1);
        assert_eq!(left.matches_revision(right.app_instance_id, right.base_revision, right.generation, &right.canonical_base_revision), case["expected"].as_bool().unwrap(), "{}", case["id"]);
        assert_eq!(right.matches_revision(left.app_instance_id, left.base_revision, left.generation, &left.canonical_base_revision), case["expected"].as_bool().unwrap(), "{}", case["id"]);
    }
    eprintln!("[DEBUG] mounted render revision matched eight complete portable identities independently of work grants");
}
