//! 🧿️ Value-codec round trips of the canvas, board and tiled-map scenes.

use super::*;

#[test]
fn canvas2d_scene_round_trips_with_and_without_snapshot() {
    let bare = Canvas2dScene::base(1.5, -2.0, 1.0, "[]".into());
    assert_eq!(Canvas2dScene::from_value(bare.to_value()), Ok(bare.clone()));
    assert!(matches!(bare.to_value(), DslValue::Object(entries) if !entries.iter().any(|(k, _)| k == "snapshot")));

    let leased = Canvas2dScene { snapshot: Some(Canvas2dSnapshotLease { slot: 1, epoch: 2, revision: 3, generation: 4, page_count: 1, byte_count: 16 }), ..bare };
    assert_eq!(Canvas2dScene::from_value(leased.to_value()), Ok(leased.clone()));

    let traced = Canvas2dScene { framing: Some(Canvas2dFraming { revision: 7,bounds: [-100.0,-50.0,300.0,150.0],padding: 40.0 }), tool_run_trace: Some("AAQB".into()), lanes: vec![SceneLaneRef { lane: "toolRunTrace".into(), bytes: 4, hash: scene_lane_hash("AAQB") }], ..leased };
    assert_eq!(Canvas2dScene::from_value(traced.to_value()), Ok(traced.clone()));
    assert_eq!(Canvas2dScene::decode_pack(&traced.encode_pack().expect("canvas-2d packs")).expect("canvas-2d unpacks"), traced);
}

#[test]
fn tiled_map_scene_round_trips() {
    let tiled = TiledMapScene::base("{}".into(), "{}".into());
    assert_eq!(TiledMapScene::from_value(tiled.to_value()), Ok(tiled.clone()));
    assert_eq!(TiledMapScene::from_value(DslValue::object([("mapDescriptorJson".to_string(), DslValue::String("{}".into())), ("cameraJson".to_string(), DslValue::String("{}".into()))])), Ok(tiled));
}

#[test]
fn board2d_scene_round_trips() {
    let board = Board2dScene::base("{}".into(), "{}".into(), true);
    assert_eq!(Board2dScene::from_value(board.to_value()), Ok(board));
}
