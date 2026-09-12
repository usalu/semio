//! 🪪️ Persisted window configuration is admitted only to its exact generated partition identity.

use super::*;

struct IdentityWindowOwner;

impl WindowConfigOwner for IdentityWindowOwner {
    const WINDOW_KIND_ID: &'static str = "identity-window";
    const SCHEMA: &'static str = "test.window.identity";
    const MAXIMUM_PUBLICATION_BYTES: usize = 16_384;
    type State = crate::app::NoConfig;
    type Mutation = crate::app::NoConfigMutation;

    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
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
fn window_config_pack_identity_rejects_foreign_inner_window_and_preserves_target() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️pack-identity/🔣️.json")).unwrap();
    let observed = semio_framework_async::block_on(async {
        let mut observed = Vec::new();
        for row in fixture["cases"].as_array().unwrap() {
            let source = row["sourceWindowId"].as_str().unwrap();
            let target = row["targetWindowId"].as_str().unwrap();
            let mut registry = WindowConfigOwnerRegistry::default();
            registry.register::<IdentityWindowOwner>().unwrap();
            drop(registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(source).await.unwrap());
            let before = registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(target).await.unwrap();
            let generation = before.generation;
            let revision = before.revision;
            let pointer = Arc::as_ptr(&before.snapshot.snapshot) as *const ();
            drop(before);
            let mut pack = registry.packs().await.unwrap().into_iter().find(|pack| pack.window_id == source).unwrap();
            pack.window_id = target.to_owned();
            let admitted = registry.load(pack).await.is_ok();
            let after = registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(target).await.unwrap();
            let unchanged = after.generation == generation && after.revision == revision && Arc::as_ptr(&after.snapshot.snapshot) as *const () == pointer;
            drop(after);
            let mut closed = false;
            for _ in 0..65_536 {
                if registry.close_step(1, 16_384).unwrap() == PluginCloseStep::Complete {
                    closed = true;
                    break;
                }
            }
            observed.push((row["id"].as_str().unwrap().to_owned(), admitted, row["accepted"].as_bool().unwrap(), unchanged, closed && registry.terminal_is_empty()));
        }
        observed
    });
    for (id, admitted, expected, unchanged, closed) in observed {
        assert!(closed, "{id}: exact window registry cleanup");
        assert_eq!(admitted, expected, "{id}: inner Pack identity must match the addressed window");
        if !expected {
            assert!(unchanged, "{id}: refusal preserves target generation, revision and snapshot backing");
        }
        eprintln!("[DEBUG] Window Pack identity {id}: admitted={admitted}, target_unchanged={unchanged}, terminal_empty={closed}");
    }
}
