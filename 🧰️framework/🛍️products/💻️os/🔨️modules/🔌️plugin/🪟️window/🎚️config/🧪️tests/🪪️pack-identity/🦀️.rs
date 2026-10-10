//! 🪪️ Persisted window configuration is admitted only to its exact generated partition identity.

use super::*;

pub(super) struct IdentityWindowOwner;

impl WindowConfigOwner for IdentityWindowOwner {
    const WINDOW_KIND_ID: &'static str = "identity-window";
    const SCHEMA: &'static str = "test.window.identity";
    const MAXIMUM_PUBLICATION_BYTES: usize = 16_384;
    type State = retained_pack_load_tests::RetainedLoadCameraConfig;
    type Mutation = retained_pack_load_tests::RetainedLoadCameraConfigMutation;
    type Edit=retained_pack_load_tests::CameraRetainedEdit;
    const MAXIMUM_PREPARATION_DEPTH:usize=64;
    fn build_retained_edit()->Arc<Self::Edit>{Arc::new(retained_pack_load_tests::CameraRetainedEdit)}

    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
        bounded_window_config_store_owners::<Self>()
    }

    fn build_one_item_preparation_factory() -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        bounded_window_config_preparation_factory::<Self>()
    }

    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        bounded_window_config_store_disposer::<Self>()
    }
}

#[test]
fn window_config_pack_identity_rejects_foreign_inner_window_without_changing_or_materializing_target() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️pack-identity/🔣️.json")).unwrap();
    let observed = semio_framework_async::block_on(async {
        let mut observed = Vec::new();
        for row in fixture["cases"].as_array().unwrap() {
            let source = row["sourceWindowId"].as_str().unwrap();
            let target = row["targetWindowId"].as_str().unwrap();
            let target_exists = row["targetExists"].as_bool().unwrap();
            let mut registry = WindowConfigOwnerRegistry::new(protocol::ActorId(fixture["openedActor"].as_str().unwrap().to_owned().into()));
            registry.register::<IdentityWindowOwner>().unwrap();
            let original=registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(source).await.unwrap(); crate::component::window_mutation::close_test_original_with_pump(original,||{let demand=registry.maintenance_retirements_demands(4096).unwrap();registry.maintenance_retirements_step(retained_pack_load_tests::quoted(demand,4096)).unwrap();});
            let target_before = if target_exists {
                let before = registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(target).await.unwrap();
                let witness = (before.generation, before.revision, before.snapshot.get::<IdentityWindowOwner>().unwrap() as *const _ as *const ());
                crate::component::window_mutation::close_test_original_with_pump(before,||{let demand=registry.maintenance_retirements_demands(4096).unwrap();registry.maintenance_retirements_step(retained_pack_load_tests::quoted(demand,4096)).unwrap();});
                Some(witness)
            } else {
                None
            };
            let before_packs = registry.packs().await.unwrap();
            let before_ids = before_packs.iter().map(|pack| (pack.window_kind_id.clone(), pack.window_id.clone())).collect::<Vec<_>>();
            let mut pack = before_packs.into_iter().find(|pack| pack.window_id == source).unwrap();
            pack.window_id = target.to_owned();
            let admitted = registry.load(pack).await.is_ok();
            let after_ids = registry.packs().await.unwrap().into_iter().map(|pack| (pack.window_kind_id, pack.window_id)).collect::<Vec<_>>();
            let unchanged = match target_before {
                Some((generation, revision, pointer)) if !row["accepted"].as_bool().unwrap() => {
                    let after = registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(target).await.unwrap();
                    let unchanged = after.generation == generation && after.revision == revision && after.snapshot.get::<IdentityWindowOwner>().unwrap() as *const _ as *const () == pointer && after_ids == before_ids;
                    crate::component::window_mutation::close_test_original_with_pump(after,||{let demand=registry.maintenance_retirements_demands(4096).unwrap();registry.maintenance_retirements_step(retained_pack_load_tests::quoted(demand,4096)).unwrap();});
                    unchanged
                }
                None => after_ids == before_ids && !after_ids.iter().any(|(kind, id)| kind == IdentityWindowOwner::WINDOW_KIND_ID && id == target),
                Some(_) => true,
            };
            while !registry.direct_ingress_terminal_is_empty(){let grant=retained_pack_load_tests::quoted(registry.direct_ingress_demands(4096).unwrap(),4096);registry.close_direct_ingress(grant).unwrap();}
            let mut closed = false;
            for _ in 0..65_536 {
                if matches!(registry.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 64 }).unwrap(), PluginLifecycleStep::Complete(_)) {
                    closed = true;
                    break;
                }
            }
            let id = row["id"].as_str().unwrap().to_owned();
            let terminal_empty = closed && registry.terminal_is_empty();
            eprintln!("[DEBUG] Window Pack identity {id}: admitted={admitted}, target_exists={target_exists}, target_unchanged_or_absent={unchanged}, terminal_empty={terminal_empty}");
            observed.push((id, admitted, row["accepted"].as_bool().unwrap(), unchanged, terminal_empty));
        }
        observed
    });
    for (id, admitted, expected, unchanged, closed) in observed {
        assert!(closed, "{id}: exact window registry cleanup");
        assert_eq!(admitted, expected, "{id}: inner Pack identity must match the addressed window");
        if !expected {
            assert!(unchanged, "{id}: refusal preserves an existing target authority or leaves an absent target unmaterialized");
        }
    }
}
