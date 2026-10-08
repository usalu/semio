use super::*;
use crate::RasterTransform;

fn pixel_layer(id: &str) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: id.into(), name: id.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: None, height: None, image_key: None }
}

fn group_layer(id: &str, children: Vec<RasterLayerNode>) -> RasterLayerNode {
    RasterLayerNode::Group { id: id.into(), name: id.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children }
}

/// 🌳️ A: root [a, g[b, c], d].
fn tree() -> Vec<RasterLayerNode> {
    vec![pixel_layer("a"), group_layer("g", vec![pixel_layer("b"), pixel_layer("c")]), pixel_layer("d")]
}

fn ids(layers: &[RasterLayerNode]) -> Vec<String> {
    layers.iter().flat_map(|node| std::iter::once(layer_node_id(node).to_string()).chain(match node {
        RasterLayerNode::Group { children, .. } => ids(children),
        _ => Vec::new(),
    })).collect()
}

fn delta_of(diff: RasterDiff) -> RasterLayersDelta {
    diff.layers.expect("layers delta")
}

fn apply(delta: &RasterLayersDelta, base: &[RasterLayerNode]) -> Vec<RasterLayerNode> {
    apply_layers_delta(base, delta).expect("delta applies")
}

fn same(left: Vec<RasterLayerNode>, right: Vec<RasterLayerNode>, message: &str) {
    let equal = left == right;
    crate::retire_raster_layers(left);
    crate::retire_raster_layers(right);
    assert!(equal, "{message}");
}

fn retire_delta(delta: RasterLayersDelta) {
    crate::retire_raster_layers(delta.inserted.into_iter().map(|row| row.layer).collect());
}

fn sum(first: &RasterLayersDelta, later: &RasterLayersDelta) -> RasterLayersDelta {
    let mut total = first.clone();
    absorb_layers_delta(&mut total, later.clone());
    total
}

fn shape_of(layers: &[RasterLayerNode]) -> Vec<String> {
    ids(layers)
}

#[semio_framework_async_macros::async_test]
async fn a_reorder_is_one_move_row_and_never_carries_the_layer() {
    let base = tree();
    let delta = delta_of(diff_move_layer(&base, "a", None, 2));
    assert!(delta.removed.is_empty() && delta.inserted.is_empty() && delta.modified.is_empty(), "a reorder is only a move");
    assert_eq!(delta.moved, vec![RasterLayerRelocation { id: "a".into(), from: RasterLayerAddress { parent_id: None, index: 0 }, to: RasterLayerAddress { parent_id: None, index: 2 } }]);
    let after = apply(&delta, &base);
    assert_eq!(after.iter().map(|node| layer_node_id(node).to_string()).collect::<Vec<_>>(), vec!["g", "d", "a"], "the layer lands at the final index");
    crate::retire_raster_layers(after);
    crate::retire_raster_layers(base);
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_move_moves_back_and_the_sum_is_empty() {
    let base = tree();
    let delta = delta_of(diff_move_layer(&base, "b", None, 0));
    let after = apply(&delta, &base);
    let inverse = inverse_layers(&delta, &base);
    assert_eq!(inverse.moved, vec![RasterLayerRelocation { id: "b".into(), from: RasterLayerAddress { parent_id: None, index: 0 }, to: RasterLayerAddress { parent_id: Some("g".into()), index: 0 } }], "the inverse is the same move run backwards across containers");
    same(apply(&inverse, &after), tree(), "the inverse restores the base");
    assert!(sum(&delta, &inverse).is_empty(), "a move and its inverse sum to nothing");
    crate::retire_raster_layers(after);
    crate::retire_raster_layers(base);
}

#[semio_framework_async_macros::async_test]
async fn removal_inverse_reinserts_the_base_subtree_at_its_base_address() {
    let base = tree();
    let delta = delta_of(diff_remove_layer(&base, "g"));
    assert_eq!(delta.removed, vec![RasterLayerRemoval { id: "g".into(), parent_id: None, index: 1 }]);
    let after = apply(&delta, &base);
    let inverse = inverse_layers(&delta, &base);
    assert_eq!((inverse.inserted.len(), inverse.inserted[0].index, layer_node_id(&inverse.inserted[0].layer).to_string()), (1, 1, "g".to_string()));
    same(apply(&inverse, &after), tree(), "the inverse restores the removed subtree");
    assert!(sum(&delta, &inverse).is_empty(), "remove then reinsert is nothing");
    retire_delta(inverse);
    crate::retire_raster_layers(after);
    crate::retire_raster_layers(base);
}

#[semio_framework_async_macros::async_test]
async fn every_pair_of_moves_absorbs_to_the_sequential_result_and_inverts() {
    let moves = |state: &[RasterLayerNode]| -> Vec<RasterLayersDelta> {
        let mut found = Vec::new();
        for id in ids(state) {
            for parent in [None, Some("g".to_string())] {
                if parent.as_deref() == Some(id.as_str()) {
                    continue;
                }
                for index in 0..4 {
                    let delta = delta_of(diff_move_layer(state, &id, parent.clone(), index));
                    if !delta.is_empty() {
                        found.push(delta);
                    }
                }
            }
        }
        found
    };
    let base = tree();
    let mut checked = 0;
    for first in moves(&base) {
        let middle = apply(&first, &base);
        for second in moves(&middle) {
            let after = apply(&second, &middle);
            let total = sum(&first, &second);
            same(apply(&total, &base), after.iter().cloned().collect(), "absorb(first, second) equals the sequential application");
            same(apply(&inverse_layers(&total, &base), &after), tree(), "the inverse of the absorbed delta restores the base");
            crate::retire_raster_layers(after);
            checked += 1;
        }
        crate::retire_raster_layers(middle);
    }
    crate::retire_raster_layers(base);
    assert!(checked > 1000, "the sweep covers every move pair, got {checked}");
}

#[semio_framework_async_macros::async_test]
async fn three_moves_of_one_layer_are_one_middle_row() {
    let base = tree();
    let first = delta_of(diff_move_layer(&base, "a", None, 2));
    let one = apply(&first, &base);
    let second = delta_of(diff_move_layer(&one, "a", Some("g".into()), 1));
    let two = apply(&second, &one);
    let third = delta_of(diff_move_layer(&two, "a", None, 1));
    let three = apply(&third, &two);
    let total = sum(&sum(&first, &second), &third);
    assert_eq!(total.moved.len(), 1, "three moves of one layer are one move row");
    assert_eq!((total.moved[0].from.index, total.moved[0].to.index), (0, 1));
    same(apply(&total, &base), three.iter().cloned().collect(), "the single row reaches the same tree");
    same(apply(&inverse_layers(&total, &base), &three), tree(), "and inverts to the base");
    for layers in [one, two, three, base] {
        crate::retire_raster_layers(layers);
    }
}

#[semio_framework_async_macros::async_test]
async fn insert_then_move_lands_at_the_final_slot_and_insert_then_remove_cancels() {
    let base = tree();
    let created = delta_of(diff_add_layer(None, 1, pixel_layer("x")));
    let one = apply(&created, &base);
    let moved = delta_of(diff_move_layer(&one, "x", Some("g".into()), 0));
    let total = sum(&created, &moved);
    assert!(total.moved.is_empty() && total.removed.is_empty() && total.inserted.len() == 1, "insert then move is one insertion at the final address");
    assert_eq!((total.inserted[0].parent_id.clone(), total.inserted[0].index), (Some("g".to_string()), 0));
    let after = apply(&moved, &one);
    same(apply(&total, &base), after, "same tree as the sequential application");
    let removed = delta_of(diff_remove_layer(&one, "x"));
    assert!(sum(&created, &removed).is_empty(), "insert then remove cancels");
    retire_delta(total);
    retire_delta(created);
    for layers in [one, base] {
        crate::retire_raster_layers(layers);
    }
}

#[semio_framework_async_macros::async_test]
async fn move_then_remove_removes_at_the_base_address_and_a_patch_of_an_inserted_layer_folds_in() {
    let base = tree();
    let moved = delta_of(diff_move_layer(&base, "a", Some("g".into()), 1));
    let one = apply(&moved, &base);
    let removed = delta_of(diff_remove_layer(&one, "a"));
    let total = sum(&moved, &removed);
    assert_eq!(total.removed, vec![RasterLayerRemoval { id: "a".into(), parent_id: None, index: 0 }], "move then remove removes at the base address");
    assert!(total.moved.is_empty() && total.inserted.is_empty());
    let after = apply(&removed, &one);
    same(apply(&total, &base), after, "same tree as the sequential application");
    let created = delta_of(diff_add_layer(None, 0, pixel_layer("x")));
    let renamed = delta_of(diff_patch_layer("x", RasterLayerPatch { name: Some("Renamed".into()), ..Default::default() }));
    let folded = sum(&created, &renamed);
    assert!(folded.modified.is_empty() && folded.inserted.len() == 1, "a patch of an inserted layer folds into its insertion");
    assert!(matches!(&folded.inserted[0].layer, RasterLayerNode::Pixel { name, .. } if name == "Renamed"));
    retire_delta(folded);
    retire_delta(created);
    for layers in [one, base] {
        crate::retire_raster_layers(layers);
    }
}

#[semio_framework_async_macros::async_test]
async fn a_stale_base_address_is_refused() {
    let base = tree();
    let stale = RasterLayersDelta { moved: vec![RasterLayerRelocation { id: "a".into(), from: RasterLayerAddress { parent_id: None, index: 2 }, to: RasterLayerAddress { parent_id: None, index: 0 } }], ..Default::default() };
    assert!(apply_layers_delta(&base, &stale).is_err(), "a move whose layer is not at its base address does not apply");
    assert_eq!(shape_of(&base), vec!["a", "g", "b", "c", "d"]);
    crate::retire_raster_layers(base);
}
