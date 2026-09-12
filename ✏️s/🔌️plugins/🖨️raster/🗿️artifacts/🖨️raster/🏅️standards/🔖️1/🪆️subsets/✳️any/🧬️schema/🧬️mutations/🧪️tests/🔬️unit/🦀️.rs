use super::*;
use crate::standards::v1::subsets::any::schema::{empty_raster_snapshot, layer_name, layer_visible};
use crate::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterOwnedMap, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use protocol::Mutation;

use semio_framework_os_kernel as vcs;
use store::{create_document_envelope, ArtifactCommand};

fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: id.into(), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(512), height: Some(512), image_key: None }
}

/// 🖼️ Real, decodable 1x1 RGBA PNGs (not arbitrary placeholder bytes) — `add-layer-asset` now
/// routes through the real `s.stdio.semio/v1/image` png codec bridge
/// (`crate::mint_raster_asset_child`), so `AddLayerAsset`'s inverse can only
/// recover a faithful prior asset from the working-scene cache if the payload actually decodes.
const SEED_ASSET_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 218, 99, 224, 18, 145, 251, 15, 0, 1, 164, 1, 60, 76, 213, 28, 167, 0, 0, 0, 0, 73, 69, 78,
    68, 174, 66, 96, 130,
];
const ABC_ASSET_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 218, 99, 56, 49, 45, 229, 63, 0, 6, 174, 2, 194, 232, 197, 127, 29, 0, 0, 0, 0, 73, 69, 78,
    68, 174, 66, 96, 130,
];

fn round_trip(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> RasterSnapshot {
    let (forward, _messages) = vcs::apply_mutation(snapshot, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in mutation.inverse(snapshot) {
        let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, snapshot, "inverse(base) must restore the pre-mutation snapshot");
    forward
}

/// ⚖️ One value per `RasterMutation` variant — the closed set the semantics/round-trip tests
/// iterate, mirroring `din16798`'s own `every_mutation()` fixture.
fn every_mutation() -> Vec<RasterMutation> {
    vec![
        RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }),
        RasterMutation::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l1".into() }),
        RasterMutation::ReorderLayers(reorder_layers::ReorderLayers { layer_id: "l1".into(), parent_id: None, index: 0 }),
        RasterMutation::RenameLayer(rename_layer::RenameLayer { layer_id: "l1".into(), new_name: "Renamed".into() }),
        RasterMutation::ChangeLayerVisible(change_layer_visible::ChangeLayerVisible { layer_id: "l1".into(), new_visible: false }),
        RasterMutation::ChangeLayerOpacity(change_layer_opacity::ChangeLayerOpacity { layer_id: "l1".into(), new_opacity: 0.4 }),
        RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::ChangeLayerBlendMode { layer_id: "l1".into(), new_blend_mode: "multiply".into() }),
        RasterMutation::MoveLayer(move_layer::MoveLayer { layer_id: "l1".into(), new_x: 10.0, new_y: 20.0 }),
        RasterMutation::ResizeLayer(resize_layer::ResizeLayer { layer_id: "l1".into(), new_width: 256, new_height: 256 }),
        RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::ChangeLayerAdjustmentKind { layer_id: "adjust-1".into(), new_adjustment_kind: "curves".into() }),
        RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset { asset_id: "asset-1".into(), asset: RasterImageAsset { mime: "image/png".into(), data: ABC_ASSET_PNG.to_vec() } }),
        RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset { asset_id: "asset-1".into() }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for mutation in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
    }
    assert_eq!(<RasterMutation as protocol::SemanticMutation<RasterSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let mut base = empty_raster_snapshot();
    base.layers.push(pixel_layer("l1", "Base"));
    base.layers.push(RasterLayerNode::Adjustment {
        id: "adjust-1".into(),
        name: "Curves".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        adjustment_kind: "brightnessContrast".into(),
        params: RasterOwnedMap::new(),
    });
    let seed_asset = RasterImageAsset { mime: "image/png".into(), data: SEED_ASSET_PNG.to_vec() };
    base.assets.insert("asset-1".into(), crate::mint_raster_asset_child("asset-1", &seed_asset)).expect("bounded fixture operation succeeds");
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn add_remove_layer_round_trip() {
    let snapshot = empty_raster_snapshot();
    let added = round_trip(&snapshot, &RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }));
    assert_eq!(added.layers.len(), 1);
    let removed = round_trip(&added, &RasterMutation::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l1".into() }));
    assert!(removed.layers.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn rename_and_change_layer_visible_round_trip() {
    let snapshot = empty_raster_snapshot();
    let added = round_trip(&snapshot, &RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }));
    let renamed = round_trip(&added, &RasterMutation::RenameLayer(rename_layer::RenameLayer { layer_id: "l1".into(), new_name: "Renamed".into() }));
    assert_eq!(layer_name(&renamed.layers[0]), "Renamed");
    let hidden = round_trip(&renamed, &RasterMutation::ChangeLayerVisible(change_layer_visible::ChangeLayerVisible { layer_id: "l1".into(), new_visible: false }));
    assert!(!layer_visible(&hidden.layers[0]));
}

#[semio_framework_async_macros::async_test]
async fn reorder_layer_into_group_round_trip() {
    let mut snapshot = empty_raster_snapshot();
    snapshot.layers.push(RasterLayerNode::Group { id: "g1".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() });
    snapshot.layers.push(pixel_layer("l1", "Base"));
    let moved = round_trip(&snapshot, &RasterMutation::ReorderLayers(reorder_layers::ReorderLayers { layer_id: "l1".into(), parent_id: Some("g1".into()), index: 0 }));
    let RasterLayerNode::Group { children, .. } = &moved.layers[0] else { panic!("expected group") };
    assert_eq!(children.len(), 1);
    assert_eq!(crate::standards::v1::subsets::any::schema::layer_node_id(&children[0]), "l1");
}

#[semio_framework_async_macros::async_test]
async fn resize_layer_is_a_graceful_no_op_on_a_group() {
    let mut snapshot = empty_raster_snapshot();
    snapshot.layers.push(RasterLayerNode::Group { id: "g1".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() });
    let mutation = RasterMutation::ResizeLayer(resize_layer::ResizeLayer { layer_id: "g1".into(), new_width: 10, new_height: 10 });
    let outcome = mutation.diff(&snapshot);
    assert_eq!(outcome.diff(), &RasterDiff::default());
    assert_eq!(outcome.worst_level(), Some(protocol::os_dsl::Severity::Error));
    assert!(mutation.inverse(&snapshot).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn store_applies_layer_create() {
    let mut store = RasterStore::new(create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster", empty_raster_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) })], description: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot").layers.len(), 1);
}

//#region 🔖️OpText
fn representative_raster_document() -> RasterSnapshot {
    let mut assets = RasterOwnedMap::new();
    assets.insert("asset-1".into(), crate::image_asset_child_handle("asset-1", &RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() })).expect("bounded fixture operation succeeds");
    let mut params = RasterOwnedMap::new();
    params.insert("brightness".into(), dsl::DslValue::float(0.06)).expect("bounded fixture operation succeeds");
    params.insert("label".into(), dsl::DslValue::String("Warm \"Curve\"".to_string())).expect("bounded fixture operation succeeds");
    params.insert("enabled".into(), dsl::DslValue::Bool(true)).expect("bounded fixture operation succeeds");
    params.insert("fallback".into(), dsl::DslValue::Null).expect("bounded fixture operation succeeds");
    params
        .insert(
            "curves".into(),
            dsl::DslValue::Array(vec![
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.0), dsl::DslValue::float(0.0)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.25), dsl::DslValue::float(0.2)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(1.0), dsl::DslValue::float(1.0)]),
            ]),
        )
        .expect("bounded fixture operation succeeds");
    params.insert("nested".into(), dsl::DslValue::Object(vec![("inner".to_string(), dsl::DslValue::float(1.5))])).expect("bounded fixture operation succeeds");
    RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "doc-1".into(),
        title: Some("Representative \"Doc\"".into()),
        assets,
        layers: vec![
            RasterLayerNode::Pixel {
                id: "pixel-1".into(),
                name: "Pixel One".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                width: Some(256),
                height: Some(256),
                image_key: Some("asset-1".into()),
            },
            RasterLayerNode::Group {
                id: "group-1".into(),
                name: "Group / Nested".into(),
                visible: false,
                opacity: 0.5,
                blend_mode: "screen".into(),
                transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                mask: None,
                children: vec![
                    RasterLayerNode::Pixel {
                        id: "pixel-2".into(),
                        name: "Child Pixel".into(),
                        visible: true,
                        opacity: 0.75,
                        blend_mode: "multiply".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        width: None,
                        height: None,
                        image_key: None,
                    },
                    RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                ],
            },
            RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn raster_op_text_round_trips_every_variant() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
    let _ = representative_raster_document();
}
//#endregion 🔖️OpText

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`,
/// exercised against three structurally distinct kinds: `create-layer` (id-keyed collection
/// insert), `change-layer-opacity` (typical `f32` scalar), and `reorder-layers` (tree
/// reposition).
#[semio_framework_async_macros::async_test]
async fn create_layer_satisfies_the_inverse_and_absorb_laws() {
    let base = empty_raster_snapshot();
    let mutation = RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 1, layer: Box::new(pixel_layer("l2", "Second")) }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_layer_opacity_satisfies_the_inverse_and_absorb_laws() {
    let mut base = empty_raster_snapshot();
    base.layers.push(pixel_layer("l1", "Base"));
    let mutation = RasterMutation::ChangeLayerOpacity(change_layer_opacity::ChangeLayerOpacity { layer_id: "l1".into(), new_opacity: 0.4 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = RasterMutation::ChangeLayerVisible(change_layer_visible::ChangeLayerVisible { layer_id: "l1".into(), new_visible: false }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_layers_satisfies_the_inverse_and_absorb_laws() {
    let mut base = empty_raster_snapshot();
    base.layers.push(pixel_layer("l1", "Base"));
    base.layers.push(pixel_layer("l2", "Second"));
    let mutation = RasterMutation::ReorderLayers(reorder_layers::ReorderLayers { layer_id: "l1".into(), parent_id: None, index: 1 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = RasterMutation::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l2".into() }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🧪️OutcomeLaws
/// ⚖️ `📋️contract-freeze.md` §C2 laws, per verb family: `assert_missing_target_is_error`/
/// `assert_fatal_never_applies` below, `assert_outcome_policy_matrix` cases further down (delete,
/// rename, create).
#[semio_framework_async_macros::async_test]
async fn delete_missing_layer_is_a_target_missing_error() {
    let base = empty_raster_snapshot();
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &RasterMutation::DeleteLayer(delete_layer::DeleteLayer { layer_id: "does-not-exist".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_missing_layer_is_a_target_missing_error() {
    let base = empty_raster_snapshot();
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &RasterMutation::RenameLayer(rename_layer::RenameLayer { layer_id: "does-not-exist".into(), new_name: "New".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_layer_duplicate_id_never_applies() {
    let mut base = empty_raster_snapshot();
    base.layers.push(pixel_layer("l1", "Base"));
    let duplicate = RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) });
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&duplicate.diff(&base)).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_layer_outcome_obeys_the_policy_matrix() {
    let mut base = empty_raster_snapshot();
    base.layers.push(pixel_layer("l1", "Base"));
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &RasterMutation::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l1".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_layer_outcome_obeys_the_policy_matrix() {
    let mut base = empty_raster_snapshot();
    base.layers.push(pixel_layer("l1", "Base"));
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &RasterMutation::RenameLayer(rename_layer::RenameLayer { layer_id: "l1".into(), new_name: "New".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_layer_outcome_obeys_the_policy_matrix() {
    let base = empty_raster_snapshot();
    let mutation = RasterMutation::CreateLayer(create_layer::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}
//#endregion 🧪️OutcomeLaws
