
use super::*;
use crate::{Board2dScene, TableScene, World3dScene};

#[test]
fn table_scene_round_trips_byte_identical() {
    let scene = TableScene::base("[{\"id\":\"name\"}]", "[]");
    let props = encode(SurfaceKind::Table, &scene).expect("bounded fixture");
    assert_eq!(props.doc_schema.as_str(), "table@1");
    assert_eq!(
        props.doc.bytes.as_slice(),
        &[13, 2, 6, 11, 99, 111, 108, 117, 109, 110, 115, 74, 115, 111, 110, 6, 15, 91, 123, 34, 105, 100, 34, 58, 34, 110, 97, 109, 101, 34, 125, 93, 6, 8, 114, 111, 119, 115, 74, 115, 111, 110, 6, 2, 91, 93,],
        "the language-neutral scene packet keeps its camelCase field names so every renderer can decode it without a Rust schema mirror",
    );
    let back: TableScene = decode(&props).expect("decode");
    assert_eq!(scene, back);
}

#[test]
fn text_editor_scene_round_trips_with_absent_optional_fields() {
    let scene = crate::TextEditorScene::base("playground.custom".into(), Some("playground".into()), None);
    let props = encode(SurfaceKind::TextEditor, &scene).expect("bounded fixture");
    let back: crate::TextEditorScene = decode(&props).expect("decode");
    assert_eq!(scene, back);
}

#[test]
fn board_scene_round_trips_with_absent_optional_fields() {
    let scene = Board2dScene::base("{}".into(), "{}".into(), true);
    let props = encode(SurfaceKind::Board2d, &scene).expect("bounded fixture");
    let back: Board2dScene = decode(&props).expect("decode");
    assert_eq!(scene, back);
}

#[test]
fn world_scene_round_trips_with_absent_optional_fields() {
    let scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    let props = encode(SurfaceKind::World3d, &scene).expect("bounded fixture");
    let back: World3dScene = decode(&props).expect("decode");
    assert_eq!(scene, back);
}

#[test]
fn wrong_schema_errs_never_panics() {
    let scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    let mut props = encode(SurfaceKind::World3d, &scene).expect("bounded fixture");
    props.doc_schema = ui_contract::UiText::try_from_str("world3d@99").expect("bounded fixture");
    let result = decode::<World3dScene>(&props);
    assert!(matches!(result, Err(SurfaceDocError::SchemaMismatch { .. })));
}
