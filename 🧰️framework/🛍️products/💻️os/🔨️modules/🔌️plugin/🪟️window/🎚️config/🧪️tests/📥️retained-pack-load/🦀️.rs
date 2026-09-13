//! 📥️ Current exact registry load/reopen baseline for the retained load lifecycle cutover.

use super::*;

fn block_on_retained_window_load<F: std::future::Future>(mut future: std::pin::Pin<Box<F>>) -> F::Output {
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

struct RetainedLoadOwnerA;
struct RetainedLoadOwnerB;

macro_rules! retained_load_owner {
    ($owner:ty, $kind:literal, $schema:literal) => {
        impl WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = $schema;
            const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
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
    };
}

retained_load_owner!(RetainedLoadOwnerA, "retained-load-a", "test.window.retained-load.a");
retained_load_owner!(RetainedLoadOwnerB, "retained-load-b", "test.window.retained-load.b");

fn register_retained_load_owners(registry: &mut WindowConfigOwnerRegistry) {
    registry.register::<RetainedLoadOwnerA>().expect("register retained-load A");
    registry.register::<RetainedLoadOwnerB>().expect("register retained-load B");
}

fn close_retained_load_registry(registry: &mut WindowConfigOwnerRegistry) {
    for _ in 0..65_536 {
        match registry.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("retained-load registry close") {
            PluginCloseStep::Complete if registry.terminal_is_empty() => return,
            PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            PluginCloseStep::AwaitingInput { .. } | PluginCloseStep::Blocked { .. } | PluginCloseStep::Complete => {}
        }
    }
    panic!("retained-load registry did not reach terminal emptiness");
}

#[test]
fn window_config_retained_pack_load_current_registry_identity_and_reopen_baseline() {
    std::thread::Builder::new()
        .name("window-config-retained-load-baseline".into())
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📥️retained-pack-load/🔣️.json")).expect("retained-load fixture");
            assert_eq!(fixture["budgets"]["typedMutationBytes"], 4_096);
            assert_eq!(fixture["requiredScenarios"].as_array().expect("required scenarios").len(), 9);
            block_on_retained_window_load(Box::pin(async {
                let mut source = WindowConfigOwnerRegistry::default();
                register_retained_load_owners(&mut source);
                for (kind, id) in [("retained-load-a", "left"), ("retained-load-a", "right"), ("retained-load-b", "other")] {
                    drop(source.owners.get_mut(kind).expect("registered source owner").capture(id).await.expect("materialize baseline source"));
                }
                let expected = source.packs().await.expect("source packs");
                assert_eq!(expected.len(), 3);
                let expected_bytes = expected.iter().map(|pack| ((pack.window_kind_id.clone(), pack.window_id.clone()), pack.files.clone())).collect::<std::collections::BTreeMap<_, _>>();
                close_retained_load_registry(&mut source);

                let mut reopened = WindowConfigOwnerRegistry::default();
                register_retained_load_owners(&mut reopened);
                for pack in expected {
                    reopened.load(pack).await.expect("current exact registry load");
                }
                let restored = reopened.packs().await.expect("reopened packs");
                let restored_bytes = restored.iter().map(|pack| ((pack.window_kind_id.clone(), pack.window_id.clone()), pack.files.clone())).collect::<std::collections::BTreeMap<_, _>>();
                assert_eq!(restored_bytes, expected_bytes, "two same-kind and one cross-kind Pack+SPR identities survive a new registry lifetime");

                let first = restored.first().expect("one restored Pack");
                let invalid = WindowConfigPack { window_id: "malformed-absent".into(), window_kind_id: first.window_kind_id.clone(), files: store::ArtifactPackFiles { pack: vec![1], ..first.files.clone() } };
                assert!(reopened.load(invalid).await.is_err(), "malformed state Pack is rejected");
                let after_invalid = reopened.packs().await.expect("packs after malformed load").into_iter().map(|pack| ((pack.window_kind_id, pack.window_id), pack.files)).collect::<std::collections::BTreeMap<_, _>>();
                assert_eq!(after_invalid, restored_bytes, "malformed load leaves existing owners unchanged and the absent target unmaterialized");
                close_retained_load_registry(&mut reopened);
            }));
            eprintln!("[DEBUG] Window retained-load baseline: owners=2 packs=3 sameKind=2 crossKind=1 malformedAtomic=1 stackBytes=2097152");
        })
        .expect("spawn retained-load baseline")
        .join()
        .expect("retained-load baseline thread");
}
