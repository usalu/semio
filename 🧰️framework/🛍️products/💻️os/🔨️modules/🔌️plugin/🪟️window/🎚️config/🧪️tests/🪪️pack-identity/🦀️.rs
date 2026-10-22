//! 🪪️ Persisted window configuration is admitted only to its exact generated partition identity.

use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigOwnerRegistry};
use semio_framework_os_kernel as protocol;
use crate::window_config_tests::retained_pack_load_tests;

pub(super) use super::camera_owner::IdentityWindowOwner;

#[test]
fn window_config_pack_identity_rejects_foreign_inner_window_without_changing_or_materializing_target() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪪️pack-identity/🔣️.json")).unwrap();
    let observed = semio_framework_async::block_on(async {
        let mut observed = Vec::new();
        for row in fixture["cases"].as_array().unwrap() {
            let source = row["sourceWindowId"].as_str().unwrap();
            let target = row["targetWindowId"].as_str().unwrap();
            let target_exists = row["targetExists"].as_bool().unwrap();
            let mut registry = WindowConfigOwnerRegistry::new(protocol::ActorId(fixture["openedActor"].as_str().unwrap().to_owned().into()));
            registry.register::<IdentityWindowOwner>().unwrap();
            let original=registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(source).await.unwrap(); crate::window_mutation_tests::tests::close_with(original,||{let demand=registry.maintenance_retirements_demands(4096).unwrap();registry.maintenance_retirements_step(retained_pack_load_tests::quoted(demand,4096)).unwrap();});
            let target_before = if target_exists {
                let before = registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(target).await.unwrap();
                let witness = (before.generation, before.revision, before.snapshot.get::<IdentityWindowOwner>().unwrap() as *const _ as *const ());
                crate::window_mutation_tests::tests::close_with(before,||{let demand=registry.maintenance_retirements_demands(4096).unwrap();registry.maintenance_retirements_step(retained_pack_load_tests::quoted(demand,4096)).unwrap();});
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
                    crate::window_mutation_tests::tests::close_with(after,||{let demand=registry.maintenance_retirements_demands(4096).unwrap();registry.maintenance_retirements_step(retained_pack_load_tests::quoted(demand,4096)).unwrap();});
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
