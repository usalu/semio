//! 🧬️ Raster artifact — document mutation dispatch enum + apply helpers.

use crate::artifacts::raster::diff::{
    diff_add_layer, diff_from_snapshot, diff_move_layer, diff_patch_layer, diff_remove_layer, diff_set_snapshot, RasterDiff,
};
use crate::artifacts::raster::engine::{find_layer, layer_node_id, locate_layer};
use crate::artifacts::raster::diff::{insert_layer, patch_layer_in_tree, remove_layer_from_tree};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum RasterMutation {
    AddLayer {
        parent_id: Option<String>,
        index: usize,
        #[dsl(statements)]
        layer: Box<RasterLayerNode>,
    },
    RemoveLayer {
        #[dsl(key = "id")]
        layer_id: String,
    },
    PatchLayer {
        #[dsl(key = "id")]
        layer_id: String,
        #[dsl(block)]
        patch: crate::artifacts::raster::RasterLayerPatch,
    },
    MoveLayer {
        #[dsl(key = "id")]
        layer_id: String,
        #[dsl(key = "parent")]
        parent_id: Option<String>,
        index: usize,
    },
    SetSnapshot {
        #[dsl(block)]
        snapshot: RasterSnapshot,
    },
}

impl Mutation<RasterSnapshot> for RasterMutation {
    type Diff = RasterDiff;

    fn diff(&self, snapshot: &RasterSnapshot) -> RasterDiff {
        match self {
            RasterMutation::AddLayer { parent_id, index, layer } => {
                if parent_id.is_none() {
                    diff_add_layer((**layer).clone())
                } else {
                    diff_from_snapshot(apply_raster_mutation(snapshot, self))
                }
            }
            RasterMutation::RemoveLayer { layer_id } => diff_remove_layer(layer_id),
            RasterMutation::PatchLayer { layer_id, patch } => diff_patch_layer(layer_id, patch.clone()),
            RasterMutation::MoveLayer { layer_id, parent_id, index } => diff_move_layer(snapshot, layer_id, parent_id.clone(), *index),
            RasterMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &RasterSnapshot) -> Vec<Self> {
        match self {
            RasterMutation::AddLayer { layer, .. } => vec![RasterMutation::RemoveLayer { layer_id: layer_node_id(layer).to_string() }],
            RasterMutation::RemoveLayer { layer_id } => match (locate_layer(&snapshot.layers, layer_id), find_layer(&snapshot.layers, layer_id)) {
                (Some((parent_id, index)), Some(layer)) => vec![RasterMutation::AddLayer { parent_id, index, layer: Box::new(layer.clone()) }],
                _ => Vec::new(),
            },
            RasterMutation::PatchLayer { layer_id, patch } => {
                let mut probe = snapshot.layers.clone();
                match patch_layer_in_tree(&mut probe, layer_id, patch) {
                    Some(inverse) => vec![RasterMutation::PatchLayer { layer_id: layer_id.clone(), patch: inverse }],
                    None => Vec::new(),
                }
            }
            RasterMutation::MoveLayer { layer_id, .. } => match locate_layer(&snapshot.layers, layer_id) {
                Some((parent_id, index)) => vec![RasterMutation::MoveLayer { layer_id: layer_id.clone(), parent_id, index }],
                None => Vec::new(),
            },
            RasterMutation::SetSnapshot { .. } => vec![RasterMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

pub fn apply_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> RasterSnapshot {
    match mutation {
        RasterMutation::SetSnapshot { snapshot } => snapshot.clone(),
        RasterMutation::AddLayer { parent_id, index, layer } => {
            let mut next = snapshot.clone();
            insert_layer(&mut next.layers, parent_id.as_deref(), *index, layer.as_ref().clone());
            next
        }
        RasterMutation::RemoveLayer { layer_id } => {
            let mut next = snapshot.clone();
            remove_layer_from_tree(&mut next.layers, layer_id);
            next
        }
        RasterMutation::PatchLayer { layer_id, patch } => {
            let mut next = snapshot.clone();
            patch_layer_in_tree(&mut next.layers, layer_id, patch);
            next
        }
        RasterMutation::MoveLayer { layer_id, parent_id, index } => {
            let mut next = snapshot.clone();
            if let Some(node) = remove_layer_from_tree(&mut next.layers, layer_id) {
                insert_layer(&mut next.layers, parent_id.as_deref(), *index, node);
            }
            next
        }
    }
}

pub fn inverse_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> Vec<RasterMutation> {
    mutation.inverse(snapshot)
}

pub type RasterEnvelope = store::ArtifactEnvelope<RasterSnapshot, RasterMutation>;
pub type RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>;

pub use super::set_snapshot::mutation::{set_snapshot, SetSnapshot};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::raster::{RasterImageAsset, RasterLayerMask, RasterLayerPatch, RasterTransform, RASTER_DOCUMENT_SCHEMA};
    use crate::artifacts::raster::engine::{empty_raster_snapshot, layer_name, layer_visible};
    use std::collections::BTreeMap;
    use store::{create_document_envelope, ArtifactCommand};
    use vcs::apply_mutation;

    fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
        RasterLayerNode::Pixel { id: id.into(), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(512), height: Some(512), image_key: None }
    }

    fn round_trip(snapshot: &RasterSnapshot, operation: &RasterMutation) -> RasterSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "backwards() must restore the pre-operation snapshot");
        forward
    }

    #[test]
    fn add_remove_patch_layer_round_trip() {
        let snapshot = empty_raster_snapshot();
        let added = round_trip(&snapshot, &RasterMutation::AddLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) });
        assert_eq!(added.layers.len(), 1);
        let patched = round_trip(&added, &RasterMutation::PatchLayer { layer_id: "l1".into(), patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() } });
        assert_eq!(layer_name(&patched.layers[0]), "Renamed");
        assert!(!layer_visible(&patched.layers[0]));
        let removed = round_trip(&patched, &RasterMutation::RemoveLayer { layer_id: "l1".into() });
        assert!(removed.layers.is_empty());
    }

    #[test]
    fn move_layer_into_group_round_trip() {
        let mut snapshot = empty_raster_snapshot();
        snapshot.layers.push(RasterLayerNode::Group { id: "g1".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() });
        snapshot.layers.push(pixel_layer("l1", "Base"));
        let moved = round_trip(&snapshot, &RasterMutation::MoveLayer { layer_id: "l1".into(), parent_id: Some("g1".into()), index: 0 });
        let RasterLayerNode::Group { children, .. } = &moved.layers[0] else { panic!("expected group") };
        assert_eq!(children.len(), 1);
        assert_eq!(layer_node_id(&children[0]), "l1");
    }

    #[test]
    fn set_snapshot_round_trip() {
        let snapshot = empty_raster_snapshot();
        let mut replacement = empty_raster_snapshot();
        replacement.layers.push(pixel_layer("l9", "Replaced"));
        let replaced = round_trip(&snapshot, &RasterMutation::SetSnapshot { snapshot: replacement.clone() });
        assert_eq!(replaced, replacement);
    }

    #[test]
    fn store_applies_layer_add() {
        let mut store = RasterStore::new(create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster", empty_raster_snapshot(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![RasterMutation::AddLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }], description: None }).expect("apply");
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
        store::os_store::test_support::assert_op_line_round_trip(&RasterMutation::AddLayer {
            parent_id: None,
            index: 0,
            layer: Box::new(RasterLayerNode::Pixel {
                id: "l1".into(),
                name: "Base".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(512),
                height: Some(512),
                image_key: None,
            }),
        });
        store::os_store::test_support::assert_op_line_round_trip(&RasterMutation::SetSnapshot { snapshot: representative_raster_document() });
    }
    //#endregion 🔖️OpText
}
//#endregion 🧪️Tests
