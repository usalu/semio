
use super::*;
use crate::model::{EntityId, Model, Zone};

fn zone(id: u32) -> Zone {
    Zone { id: EntityId(id), name: format!("Zone {id}"), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

/// 🧪️ The minting LAW, stated as the three cases that actually differ: an empty collection
/// starts at 1 (never 0, the "no entity" sentinel), a populated one continues past its maximum
/// rather than past its length, and a collection with a hole left by a delete does NOT refill
/// that hole — reusing a deleted id is exactly how a dangling reference silently re-points at a
/// different entity.
#[semio_framework_async_macros::async_test]
async fn next_entity_id_never_reuses_and_never_mints_zero() {
    assert_eq!(next_entity_id(std::iter::empty()), EntityId(1));
    assert_eq!(next_entity_id([EntityId(1), EntityId(2), EntityId(3)].into_iter()), EntityId(4));
    assert_eq!(next_entity_id([EntityId(1), EntityId(7)].into_iter()), EntityId(8), "the hole at 2..=6 must stay a hole");
    assert_eq!(next_entity_id([EntityId(9), EntityId(2)].into_iter()), EntityId(10), "order must not matter");
    assert_eq!(next_entity_id([EntityId(u32::MAX)].into_iter()), EntityId(u32::MAX), "saturating, so exhaustion is a collision the caller can detect, not a wrap to 0");
}

/// 🧪️ Minting is per-COLLECTION, and repeated minting only advances once the previous id has
/// actually been inserted — the property every `create-*` kind's caller relies on.
#[semio_framework_async_macros::async_test]
async fn next_entity_id_advances_only_as_the_collection_grows() {
    let mut model = Model::default();
    for expected in 1..=4u32 {
        let id = next_entity_id(model.zones.iter().map(|zone| zone.id));
        assert_eq!(id, EntityId(expected));
        assert_eq!(next_entity_id(model.zones.iter().map(|zone| zone.id)), id, "minting is pure — it does not advance until the entity lands");
        model.zones.push(zone(id.0));
    }
    model.zones.retain(|zone| zone.id != EntityId(2));
    assert_eq!(next_entity_id(model.zones.iter().map(|zone| zone.id)), EntityId(5), "deleting zone 2 must not hand 2 back out");
}

/// 🧪️ A whole-document load keeps both link slots when the caller carries them, and clears both
/// when it does not — the regression that `energy_snapshot_with_state` alone cannot express,
/// since it hardcodes `weather_link: None`.
#[semio_framework_async_macros::async_test]
async fn a_whole_document_load_carries_both_link_slots() {
    let model = Model { name: "BESTEST 600".into(), version: "1".into(), ..Model::default() };
    let links = EnergyModelLinkSlots {
        referenced_model: Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("doc-2!s.stdio.semio@v1/model").expect("valid link ref uri"), pin: store::LinkPin::Head, role: "model".into() }),
        weather_link: Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("denver-tmy!s.stdio.semio@v1/value").expect("valid link ref uri"), pin: store::LinkPin::Head, role: "weather".into() }),
    };
    let carried = energy_snapshot_with_links(&model, &links);
    assert_eq!(EnergyModelLinkSlots::of(&carried), links);
    assert_eq!(EnergyModelLinkSlots::of(&energy_snapshot_with_links(&model, &EnergyModelLinkSlots::default())), EnergyModelLinkSlots::default());
}

/// 🧪️ The load effect is a genesis document, not an edit: its pack decodes back to exactly the
/// snapshot that went in, so the host's `ArtifactStore::reset` installs the loaded model itself
/// rather than a projection of it.
#[semio_framework_async_macros::async_test]
async fn the_load_document_effect_carries_the_model_it_was_given() {
    let mut model = Model { name: "BESTEST 900".into(), version: "1".into(), ..Model::default() };
    model.zones.push(zone(1));
    let effect = energy_model_load_document_effect("model", &model, &EnergyModelLinkSlots::default());
    let semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr } = effect else { panic!("a whole-document load must be a LoadDocument effect") };
    assert!(!spr.is_empty(), "the genesis envelope must still print an spr");
    let loaded = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("the emitted pack decodes");
    assert_eq!(loaded.model, model);
    assert_eq!(loaded.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
}
