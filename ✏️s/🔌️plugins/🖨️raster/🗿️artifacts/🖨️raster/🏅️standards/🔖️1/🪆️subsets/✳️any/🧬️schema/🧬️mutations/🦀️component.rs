//! 🧬️ Raster artifact — closed semantic mutation dispatch enum (constitutional: op). Derived from
//! `RasterSnapshot`'s recursive layer-tree shape per `📓️derivation-rules.md`: the five old
//! option-bag/whole-tree variants (`AddLayer`, `RemoveLayer`, `PatchLayer`, `MoveLayer`, and the old
//! whole-document-replace variant) are gone, replaced by ten real verbs (`create-layer`, `delete-layer`,
//! `reorder-layers`, `rename-layer`, `change-layer-visible`, `change-layer-opacity`,
//! `change-layer-blend-mode`, `move-layer`, `resize-layer`, `change-layer-adjustment-kind`) plus two
//! justified additions for the `assets` id-keyed root collection (`add-layer-asset`,
//! `remove-layer-asset` — see that leaf's docstring). The old whole-document-replace variant dies
//! with NO replacement: whole-document replace goes through `store::ArtifactStore::reset`, entirely
//! outside this enum.
//!
//! All twelve triads are mounted directly as `mutations`-sibling modules in `📦️glue.rs`, each with
//! its own unique emoji-prefixed directory — no inline `#[path = "."]` self-wiring.

use crate::artifacts::raster::diff::RasterDiff;
use crate::artifacts::raster::RasterSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::add_layer_asset;
use super::change_layer_adjustment_kind;
use super::change_layer_blend_mode;
use super::change_layer_opacity;
use super::change_layer_visible;
use super::create_layer;
use super::delete_layer;
use super::move_layer;
use super::remove_layer_asset;
use super::rename_layer;
use super::reorder_layers;
use super::resize_layer;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the raster document, derived per
/// `📓️derivation-rules.md` from `RasterLayerNode`'s recursive tree shape and the `assets` root
/// collection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RasterSnapshot, diff = RasterDiff, schema = "raster.raster")]
pub enum RasterMutation {
    CreateLayer(create_layer::mutation::CreateLayer),
    DeleteLayer(delete_layer::mutation::DeleteLayer),
    ReorderLayers(reorder_layers::mutation::ReorderLayers),
    RenameLayer(rename_layer::mutation::RenameLayer),
    ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible),
    ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity),
    ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode),
    MoveLayer(move_layer::mutation::MoveLayer),
    ResizeLayer(resize_layer::mutation::ResizeLayer),
    ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind),
    AddLayerAsset(add_layer_asset::mutation::AddLayerAsset),
    RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset),
}

/// ⚡️ Convenience wrapper kept for existing in-plugin callers (`RasterBuilderConstruction::mutate`,
/// the WASM bridge) — `diff().apply()` in one call, now delegating to the derive's real
/// `Mutation`/`MutationDiff` impls instead of a hand-written match.
pub fn apply_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> RasterSnapshot {
    protocol::MutationDiff::apply(&protocol::Mutation::diff(mutation, snapshot), snapshot)
}

/// ⚡️ Convenience wrapper mirroring `apply_raster_mutation` — forwards to the derive's real
/// `Mutation::inverse`.
pub fn inverse_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> Vec<RasterMutation> {
    protocol::Mutation::inverse(mutation, snapshot)
}

pub type RasterEnvelope = store::ArtifactEnvelope<RasterSnapshot, RasterMutation>;
pub type RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>;
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    use crate::artifacts::raster::schema::{empty_raster_snapshot, layer_name, layer_visible};
    use crate::artifacts::raster::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};
    use protocol::Mutation;
    use std::collections::BTreeMap;
    use store::{create_document_envelope, ArtifactCommand};

    fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
        RasterLayerNode::Pixel { id: id.into(), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(512), height: Some(512), image_key: None }
    }

    fn round_trip(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> RasterSnapshot {
        let forward = vcs::apply_mutation(snapshot, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "inverse(base) must restore the pre-mutation snapshot");
        forward
    }

    /// ⚖️ One value per `RasterMutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring `din16798`'s own `every_mutation()` fixture.
    fn every_mutation() -> Vec<RasterMutation> {
        vec![
            RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }),
            RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: "l1".into() }),
            RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id: "l1".into(), parent_id: None, index: 0 }),
            RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "l1".into(), new_name: "Renamed".into() }),
            RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: "l1".into(), new_visible: false }),
            RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id: "l1".into(), new_opacity: 0.4 }),
            RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id: "l1".into(), new_blend_mode: "multiply".into() }),
            RasterMutation::MoveLayer(move_layer::mutation::MoveLayer { layer_id: "l1".into(), new_x: 10.0, new_y: 20.0 }),
            RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: "l1".into(), new_width: 256, new_height: 256 }),
            RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id: "adjust-1".into(), new_adjustment_kind: "curves".into() }),
            RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: "asset-1".into(), asset: RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() } }),
            RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: "asset-1".into() }),
        ]
    }

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<RasterMutation as protocol::SemanticMutation<RasterSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let mut base = empty_raster_snapshot();
        base.layers.push(pixel_layer("l1", "Base"));
        base.layers.push(RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "brightnessContrast".into(), params: BTreeMap::new() });
        base.assets.insert("asset-1".into(), RasterImageAsset { mime: "image/png".into(), data: b"seed".to_vec() });
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    #[test]
    fn add_remove_layer_round_trip() {
        let snapshot = empty_raster_snapshot();
        let added = round_trip(&snapshot, &RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }));
        assert_eq!(added.layers.len(), 1);
        let removed = round_trip(&added, &RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: "l1".into() }));
        assert!(removed.layers.is_empty());
    }

    #[test]
    fn rename_and_change_layer_visible_round_trip() {
        let snapshot = empty_raster_snapshot();
        let added = round_trip(&snapshot, &RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }));
        let renamed = round_trip(&added, &RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "l1".into(), new_name: "Renamed".into() }));
        assert_eq!(layer_name(&renamed.layers[0]), "Renamed");
        let hidden = round_trip(&renamed, &RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: "l1".into(), new_visible: false }));
        assert!(!layer_visible(&hidden.layers[0]));
    }

    #[test]
    fn reorder_layer_into_group_round_trip() {
        let mut snapshot = empty_raster_snapshot();
        snapshot.layers.push(RasterLayerNode::Group { id: "g1".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() });
        snapshot.layers.push(pixel_layer("l1", "Base"));
        let moved = round_trip(&snapshot, &RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id: "l1".into(), parent_id: Some("g1".into()), index: 0 }));
        let RasterLayerNode::Group { children, .. } = &moved.layers[0] else { panic!("expected group") };
        assert_eq!(children.len(), 1);
        assert_eq!(crate::artifacts::raster::schema::layer_node_id(&children[0]), "l1");
    }

    #[test]
    fn resize_layer_is_a_graceful_no_op_on_a_group() {
        let mut snapshot = empty_raster_snapshot();
        snapshot.layers.push(RasterLayerNode::Group { id: "g1".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() });
        let mutation = RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: "g1".into(), new_width: 10, new_height: 10 });
        let diff = mutation.diff(&snapshot);
        assert_eq!(diff, RasterDiff::default());
        assert!(mutation.inverse(&snapshot).is_empty());
    }

    #[test]
    fn store_applies_layer_create() {
        let mut store = RasterStore::new(create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster", empty_raster_snapshot(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) })], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").layers.len(), 1);
    }

    //#region 🔖️OpText
    fn representative_raster_document() -> RasterSnapshot {
        let mut assets = BTreeMap::new();
        assets.insert("asset-1".into(), RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() });
        let mut params = BTreeMap::new();
        params.insert("brightness".into(), dsl::to_dsl_value(&serde_json::json!(0.06)).expect("dsl value"));
        params.insert("label".into(), dsl::to_dsl_value(&serde_json::json!("Warm \"Curve\"")).expect("dsl value"));
        params.insert("enabled".into(), dsl::to_dsl_value(&serde_json::json!(true)).expect("dsl value"));
        params.insert("fallback".into(), dsl::DslValue::Null);
        params.insert("curves".into(), dsl::to_dsl_value(&serde_json::json!([[0.0, 0.0], [0.25, 0.2], [1.0, 1.0]])).expect("dsl value"));
        params.insert("nested".into(), dsl::to_dsl_value(&serde_json::json!({ "inner": 1.5 })).expect("dsl value"));
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

    #[test]
    fn raster_op_text_round_trips_every_variant() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
        let _ = representative_raster_document();
    }
    //#endregion 🔖️OpText

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`,
    /// exercised against three structurally distinct kinds: `create-layer` (id-keyed collection
    /// insert), `change-layer-opacity` (typical `f32` scalar), and `reorder-layers` (tree
    /// reposition).
    #[test]
    fn create_layer_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_raster_snapshot();
        let mutation = RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: 1, layer: Box::new(pixel_layer("l2", "Second")) }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn change_layer_opacity_satisfies_the_inverse_and_absorb_laws() {
        let mut base = empty_raster_snapshot();
        base.layers.push(pixel_layer("l1", "Base"));
        let mutation = RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id: "l1".into(), new_opacity: 0.4 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: "l1".into(), new_visible: false }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn reorder_layers_satisfies_the_inverse_and_absorb_laws() {
        let mut base = empty_raster_snapshot();
        base.layers.push(pixel_layer("l1", "Base"));
        base.layers.push(pixel_layer("l2", "Second"));
        let mutation = RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id: "l1".into(), parent_id: None, index: 1 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: "l2".into() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
