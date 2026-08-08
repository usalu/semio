//! 🧬️ raster artifact — document mutation dispatch.

//! ⚡️ Raster artifact — operation enum + laws (constitutional: op).


use crate::artifacts::raster::diff::{patch_layer_in_tree, step_diff, RasterDiff};
use crate::artifacts::raster::engine::{find_layer, layer_node_id, locate_layer};
use crate::artifacts::raster::RasterLayerNode;
use crate::artifacts::raster::RasterProjection;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
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
    ReplaceDocument {
        #[dsl(block)]
        document: RasterProjection,
    },
}
impl Mutation<RasterProjection> for RasterMutation {
    type Diff = RasterDiff;

    fn diff(&self, _projection: &RasterProjection) -> RasterDiff {
        match self {
            RasterMutation::AddLayer { parent_id, index, layer } => step_diff(crate::artifacts::raster::diff::RasterStep::AddLayer { parent_id: parent_id.clone(), index: *index, layer: (**layer).clone() }),
            RasterMutation::RemoveLayer { layer_id } => step_diff(crate::artifacts::raster::diff::RasterStep::RemoveLayer { layer_id: layer_id.clone() }),
            RasterMutation::PatchLayer { layer_id, patch } => step_diff(crate::artifacts::raster::diff::RasterStep::PatchLayer { layer_id: layer_id.clone(), patch: patch.clone() }),
            RasterMutation::MoveLayer { layer_id, parent_id, index } => step_diff(crate::artifacts::raster::diff::RasterStep::MoveLayer { layer_id: layer_id.clone(), parent_id: parent_id.clone(), index: *index }),
            RasterMutation::ReplaceDocument { document } => RasterDiff { replace: Some(Box::new(document.clone())), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &RasterProjection) -> Vec<Self> {
        match self {
            RasterMutation::AddLayer { layer, .. } => vec![RasterMutation::RemoveLayer { layer_id: layer_node_id(layer).to_string() }],
            RasterMutation::RemoveLayer { layer_id } => match (locate_layer(&projection.layers, layer_id), find_layer(&projection.layers, layer_id)) {
                (Some((parent_id, index)), Some(layer)) => vec![RasterMutation::AddLayer { parent_id, index, layer: Box::new(layer.clone()) }],
                _ => Vec::new(),
            },
            RasterMutation::PatchLayer { layer_id, patch } => {
                let mut probe = projection.layers.clone();
                match patch_layer_in_tree(&mut probe, layer_id, patch) {
                    Some(inverse) => vec![RasterMutation::PatchLayer { layer_id: layer_id.clone(), patch: inverse }],
                    None => Vec::new(),
                }
            }
            RasterMutation::MoveLayer { layer_id, .. } => match locate_layer(&projection.layers, layer_id) {
                Some((parent_id, index)) => vec![RasterMutation::MoveLayer { layer_id: layer_id.clone(), parent_id, index }],
                None => Vec::new(),
            },
            RasterMutation::ReplaceDocument { .. } => vec![RasterMutation::ReplaceDocument { document: projection.clone() }],
        }
    }
}

pub type RasterEnvelope = store::DocumentEnvelope<RasterProjection, RasterMutation>;
pub type RasterStore = store::DocumentStore<RasterProjection, RasterMutation>;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::raster::{RasterImageAsset, RasterLayerMask, RasterLayerPatch, RasterTransform, RASTER_DOCUMENT_SCHEMA};
    use crate::artifacts::raster::engine::{empty_raster_projection, layer_name, layer_visible};
    use std::collections::BTreeMap;
    use store::{create_document_envelope, DocumentCommand};
    use vcs::apply_mutation;

    fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
        RasterLayerNode::Pixel { id: id.into(), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(512), height: Some(512), image_key: None }
    }

    fn round_trip(projection: &RasterProjection, operation: &RasterMutation) -> RasterProjection {
        let forward = vcs::apply_mutation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(projection) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation projection");
        forward
    }

    #[test]
    fn add_remove_patch_layer_round_trip() {
        let projection = empty_raster_projection();
        let added = round_trip(&projection, &RasterMutation::AddLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) });
        assert_eq!(added.layers.len(), 1);
        let patched = round_trip(&added, &RasterMutation::PatchLayer { layer_id: "l1".into(), patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() } });
        assert_eq!(layer_name(&patched.layers[0]), "Renamed");
        assert!(!layer_visible(&patched.layers[0]));
        let removed = round_trip(&patched, &RasterMutation::RemoveLayer { layer_id: "l1".into() });
        assert!(removed.layers.is_empty());
    }

    #[test]
    fn move_layer_into_group_round_trip() {
        let mut projection = empty_raster_projection();
        projection.layers.push(RasterLayerNode::Group { id: "g1".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() });
        projection.layers.push(pixel_layer("l1", "Base"));
        let moved = round_trip(&projection, &RasterMutation::MoveLayer { layer_id: "l1".into(), parent_id: Some("g1".into()), index: 0 });
        let RasterLayerNode::Group { children, .. } = &moved.layers[0] else { panic!("expected group") };
        assert_eq!(children.len(), 1);
        assert_eq!(layer_node_id(&children[0]), "l1");
    }

    #[test]
    fn replace_document_round_trip() {
        let projection = empty_raster_projection();
        let mut replacement = empty_raster_projection();
        replacement.layers.push(pixel_layer("l9", "Replaced"));
        let replaced = round_trip(&projection, &RasterMutation::ReplaceDocument { document: replacement.clone() });
        assert_eq!(replaced, replacement);
    }

    #[test]
    fn store_applies_layer_add() {
        let mut store = RasterStore::new(create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster", empty_raster_projection(), None));
        store.dispatch(DocumentCommand::Apply { mutations: vec![RasterMutation::AddLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").layers.len(), 1);
    }

    //#region 🔖️OpText
    /// 📄️ Handcrafted document exercising every layer kind/field — this node's own private copy (crate
    /// boundaries no longer apply, but each taxonomy node still keeps its own copy for test isolation).
    fn representative_raster_document() -> RasterProjection {
        let mut assets = BTreeMap::new();
        assets.insert("asset-1".into(), RasterImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into() });
        let mut params = BTreeMap::new();
        params.insert("brightness".into(), dsl::to_dsl_value(&serde_json::json!(0.06)).expect("dsl value"));
        params.insert("label".into(), dsl::to_dsl_value(&serde_json::json!("Warm \"Curve\"")).expect("dsl value"));
        params.insert("enabled".into(), dsl::to_dsl_value(&serde_json::json!(true)).expect("dsl value"));
        params.insert("fallback".into(), dsl::DslValue::Null);
        params.insert("curves".into(), dsl::to_dsl_value(&serde_json::json!([[0.0, 0.0], [0.25, 0.2], [1.0, 1.0]])).expect("dsl value"));
        params.insert("nested".into(), dsl::to_dsl_value(&serde_json::json!({ "inner": 1.5 })).expect("dsl value"));
        RasterProjection {
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
        store::test_support::assert_op_line_round_trip(&RasterMutation::AddLayer {
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
        store::test_support::assert_op_line_round_trip(&RasterMutation::AddLayer {
            parent_id: Some("group-1".into()),
            index: 3,
            layer: Box::new(RasterLayerNode::Group {
                id: "g2".into(),
                name: "Nested".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: true, invert: false, width: Some(10), height: Some(20) }),
                children: vec![],
            }),
        });
        store::test_support::assert_op_line_round_trip(&RasterMutation::RemoveLayer { layer_id: "l1".into() });
        store::test_support::assert_op_line_round_trip(&RasterMutation::PatchLayer { layer_id: "l1".into(), patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() } });
        store::test_support::assert_op_line_round_trip(&RasterMutation::PatchLayer { layer_id: "adjust-1".into(), patch: RasterLayerPatch::default() });
        store::test_support::assert_op_line_round_trip(&RasterMutation::MoveLayer { layer_id: "l1".into(), parent_id: Some("g2".into()), index: 1 });
        store::test_support::assert_op_line_round_trip(&RasterMutation::MoveLayer { layer_id: "l1".into(), parent_id: None, index: 0 });
        store::test_support::assert_op_line_round_trip(&RasterMutation::ReplaceDocument { document: representative_raster_document() });
    }
    //#endregion 🔖️OpText
}
//#endregion 🧪️Tests

/// ▶️ Applies `mutation` via its diff.
pub fn apply_raster_mutation(projection: &RasterProjection, mutation: &RasterMutation) -> RasterProjection {
    protocol::MutationDiff::apply(&mutation.diff(projection), projection)
}

pub fn inverse_raster_mutation(projection: &RasterProjection, mutation: &RasterMutation) -> Vec<RasterMutation> {
    mutation.inverse(projection)
}
