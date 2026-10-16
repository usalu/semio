//! 🌀️ Value-codec round trips of the world-3d scene, dense and bare.

use super::*;

#[test]
fn world3d_scene_round_trips_dense_and_bare_and_keeps_integers_as_integers() {
    let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    assert_eq!(World3dScene::from_value(bare.to_value()), Ok(bare.clone()));

    let mut dense = bare.clone();
    dense.domain_id = Some("cad".into());
    dense.domain_granularity_id = Some("handle".into());
    dense.snapshot = Some(World3dSnapshotLease { slot: 2, epoch: 3, revision: 4, generation: 5, page_count: 6, item_count: 7, byte_count: 8 });
    let encoded = dense.to_value();
    assert_eq!(World3dScene::from_value(encoded.clone()), Ok(dense));
    let DslValue::Object(entries) = &encoded else { panic!("expected an object") };
    let snapshot_entries = match entries.iter().find(|(k, _)| k == "snapshot").map(|(_, v)| v) {
        Some(DslValue::Object(entries)) => entries,
        other => panic!("expected the nested snapshot lease to encode as an object, found {other:?}"),
    };
    let slot = snapshot_entries.iter().find(|(k, _)| k == "slot").map(|(_, v)| v.clone());
    assert!(matches!(slot, Some(DslValue::Number(protocol::value::Number::UInt(2)))), "u8 field must stay an integer, found {slot:?}");
}
