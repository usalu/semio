
use super::*;

fn sample_with_composition() -> EnergyModelSnapshot {
    let mut snapshot = energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model { name: "Demo".into(), version: "1".into(), ..crate::model::Model::default() }, None);
    snapshot.referenced_model = Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("doc-2!s.stdio.semio@v1/model").expect("valid link ref uri"), pin: store::LinkPin::Head, role: "model".into() });
    snapshot.weather_link = Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("denver-tmy!s.stdio.semio@v1/value").expect("valid link ref uri"), pin: store::LinkPin::Head, role: "weather".into() });
    snapshot
}

/// 🧪️ Every field on `EnergyModelSnapshot` — the two composition slots and BOTH link slots —
/// must survive both hand-rolled codecs (text and binary), independently. Codec completeness is
/// not caught by `cargo check`; this is the real round-trip proof the migration recipe requires,
/// and since neither slot is on `serde` any more it is also the proof that the
/// `ToValue`/`FromValue` + `pack::json` route carries an `ArtifactLink` losslessly.
#[semio_framework_async_macros::async_test]
async fn structure_zones_and_both_link_slots_round_trip_through_text_and_binary() {
    let snapshot = sample_with_composition();
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let from_text = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse round-tripped text");
    assert_eq!(from_text, snapshot);

    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let from_binary = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode round-tripped binary");
    assert_eq!(from_binary, snapshot);
}

/// 🧪️ An absent link slot must come back absent, not as a decode error and not as a present
/// link — checked on each slot on its own AND on both at once, because a codec that reads the
/// two slots in the wrong order still round-trips whenever they happen to agree.
#[semio_framework_async_macros::async_test]
async fn absent_link_slots_round_trip_as_none() {
    let full = sample_with_composition();
    let mut without_referenced_model = full.clone();
    without_referenced_model.referenced_model = None;
    let mut without_weather = full.clone();
    without_weather.weather_link = None;
    let mut without_either = full.clone();
    without_either.referenced_model = None;
    without_either.weather_link = None;
    for snapshot in [without_referenced_model, without_weather, without_either] {
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        assert_eq!(<EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snapshot);
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        assert_eq!(<EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snapshot);
    }
}

/// 🧪️ The two link slots are distinguishable on the wire: swapping their contents must produce a
/// different document. Without this, a codec that wrote one slot twice would pass every test
/// above.
#[semio_framework_async_macros::async_test]
async fn the_two_link_slots_are_not_interchangeable_on_the_wire() {
    let snapshot = sample_with_composition();
    let mut swapped = snapshot.clone();
    std::mem::swap(&mut swapped.referenced_model, &mut swapped.weather_link);
    assert_ne!(store::ArtifactDsl::print_dsl(&swapped), store::ArtifactDsl::print_dsl(&snapshot));
    assert_ne!(store::ArtifactPack::encode_pack(&swapped), store::ArtifactPack::encode_pack(&snapshot));
    assert_eq!(<EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&swapped)).expect("decode"), swapped);
}

/// 🧪️ `energy_structure_from_model`/`energy_model_from_structure` round-trip the whole `Model`
/// losslessly through the generic JSON<->`SemioValue` bridge — the real codec-completeness proof
/// for the artifact root's `🔖️Converters` region.
#[semio_framework_async_macros::async_test]
async fn model_round_trips_through_the_structure_child_content() {
    let model = crate::model::Model {
        name: "Demo".into(),
        version: "1".into(),
        zones: vec![crate::model::Zone { id: crate::model::EntityId(1), name: "Zone1".into(), volume_m3: 100.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
        ..crate::model::Model::default()
    };
    let structure = crate::energy_structure_from_model(&model);
    let restored = crate::energy_model_from_structure(&structure);
    assert_eq!(restored, model);
}
