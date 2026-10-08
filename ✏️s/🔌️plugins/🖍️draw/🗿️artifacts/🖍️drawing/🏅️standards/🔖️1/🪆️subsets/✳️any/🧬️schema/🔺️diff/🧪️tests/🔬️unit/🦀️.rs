use super::*;
use crate::schema::{create_drawing_group_layer, create_drawing_shape_layer_rect, default_drawing_document};
use protocol::{DiffAlgebra, MutationDiff};

fn id_of(node: &DrawingLayerNode) -> String {
    layer_id(node).to_string_owner()
}

fn tree() -> DrawingSnapshot {
    let mut group = create_drawing_group_layer("G");
    if let DrawingLayerNode::Group(inner) = &mut group {
        inner.children.push(create_drawing_shape_layer_rect("B"));
        inner.children.push(create_drawing_shape_layer_rect("C"));
    }
    let mut document = default_drawing_document("tree", None);
    document.layers.push(create_drawing_shape_layer_rect("A"));
    document.layers.push(group);
    document.layers.push(create_drawing_shape_layer_rect("D"));
    document
}

fn ids(layers: &PagedList<DrawingLayerNode, {usize::MAX}>) -> Vec<String> {
    layers.iter().flat_map(|node| std::iter::once(id_of(node)).chain(match node {
        DrawingLayerNode::Group(group) => ids(&group.children),
        _ => Vec::new(),
    })).collect()
}

fn apply(diff: &DrawingDiff, base: &DrawingSnapshot) -> DrawingSnapshot {
    protocol::apply_diff(diff, base).expect("diff applies")
}

fn sum(first: &DrawingDiff, later: &DrawingDiff) -> DrawingDiff {
    let mut total = first.clone();
    total.absorb(later.clone());
    total
}

fn address(base: &DrawingSnapshot, name: &str) -> (Option<String>, usize) {
    let id = ids(&base.layers).into_iter().find(|id| find_node(&base.layers, id).is_some_and(|node| crate::schema::layer_base(node).name == name)).expect("layer exists");
    address_of(&base.layers, &id).expect("layer has an address")
}

fn named(base: &DrawingSnapshot, name: &str) -> String {
    ids(&base.layers).into_iter().find(|id| find_node(&base.layers, id).is_some_and(|node| crate::schema::layer_base(node).name == name)).expect("layer exists")
}

#[semio_framework_async_macros::async_test]
async fn a_reorder_is_one_move_row_and_never_carries_the_layer() {
    let base = tree();
    let a = named(&base, "A");
    let diff = diff_reorder_layer(&base.layers, &a, None, 2);
    let delta = diff.layers.clone().expect("layers delta");
    assert!(delta.removed.is_empty() && delta.inserted.is_empty() && delta.modified.is_empty(), "a reorder is only a move");
    assert_eq!(delta.moved, vec![DrawingLayerRelocation { id: a.clone(), from: DrawingLayerAddress { parent_id: None, index: 0 }, to: DrawingLayerAddress { parent_id: None, index: 2 } }]);
    let after = apply(&diff, &base);
    assert_eq!(after.layers.iter().map(id_of).collect::<Vec<_>>(), vec![named(&base, "G"), named(&base, "D"), a], "the layer lands at the final index");
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_move_moves_back_and_the_sum_is_empty() {
    let base = tree();
    let b = named(&base, "B");
    let diff = diff_reorder_layer(&base.layers, &b, None, 0);
    let after = apply(&diff, &base);
    let inverse = diff.inverse(&base);
    let delta = inverse.layers.clone().expect("layers delta");
    assert_eq!(delta.moved, vec![DrawingLayerRelocation { id: b.clone(), from: DrawingLayerAddress { parent_id: None, index: 0 }, to: DrawingLayerAddress { parent_id: Some(named(&base, "G")), index: 0 } }], "the inverse is the same move run backwards across containers");
    assert_eq!(apply(&inverse, &after), base, "the inverse restores the base");
    assert!(DiffAlgebra::is_empty(&sum(&diff, &inverse)), "a move and its inverse sum to nothing");
}

#[semio_framework_async_macros::async_test]
async fn removal_inverse_reinserts_the_base_subtree_at_its_base_address() {
    let base = tree();
    let group = named(&base, "G");
    let diff = diff_remove_layer(&base.layers, &group);
    let inverse = diff.inverse(&base);
    let delta = inverse.layers.clone().expect("layers delta");
    assert_eq!((delta.inserted.len(), delta.inserted[0].index, id_of(&delta.inserted[0].layer)), (1, 1, group.clone()), "the inverse inserts the base subtree at its base index");
    assert_eq!(apply(&inverse, &apply(&diff, &base)), base);
    assert!(DiffAlgebra::is_empty(&sum(&diff, &inverse)), "remove then reinsert is nothing");
}

#[semio_framework_async_macros::async_test]
async fn every_pair_of_moves_absorbs_to_the_sequential_result_and_inverts() {
    let base = tree();
    let group = named(&base, "G");
    let moves = |state: &DrawingSnapshot| -> Vec<DrawingDiff> {
        let mut found = Vec::new();
        for id in ids(&state.layers) {
            for parent in [None, Some(group.clone())] {
                if parent.as_deref() == Some(id.as_str()) {
                    continue;
                }
                for index in 0..4 {
                    let parent_id = parent.as_ref().map(|parent| PagedUtf8::<{usize::MAX}>::from(parent.as_str()));
                    let diff = diff_reorder_layer(&state.layers, &id, parent_id.as_ref(), index);
                    if !DiffAlgebra::is_empty(&diff) {
                        found.push(diff);
                    }
                }
            }
        }
        found
    };
    let mut checked = 0;
    for first in moves(&base) {
        let middle = apply(&first, &base);
        for second in moves(&middle) {
            let after = apply(&second, &middle);
            let total = sum(&first, &second);
            assert_eq!(apply(&total, &base), after, "absorb(first, second) equals the sequential application");
            assert_eq!(apply(&total.inverse(&base), &after), base, "the inverse of the absorbed diff restores the base");
            checked += 1;
        }
    }
    assert!(checked > 1000, "the sweep covers every move pair, got {checked}");
}

#[semio_framework_async_macros::async_test]
async fn a_middle_move_is_one_row_over_three_moves_of_one_layer() {
    let base = tree();
    let a = named(&base, "A");
    let group = named(&base, "G");
    let first = diff_reorder_layer(&base.layers, &a, None, 2);
    let one = apply(&first, &base);
    let second = diff_reorder_layer(&one.layers, &a, Some(&PagedUtf8::<{usize::MAX}>::from(group.as_str())), 1);
    let two = apply(&second, &one);
    let third = diff_reorder_layer(&two.layers, &a, None, 1);
    let three = apply(&third, &two);
    let total = sum(&sum(&first, &second), &third);
    let delta = total.layers.clone().expect("layers delta");
    assert_eq!(delta.moved.len(), 1, "three moves of one layer are one move row");
    assert_eq!((delta.moved[0].from.index, delta.moved[0].to.index), (0, 1));
    assert_eq!(apply(&total, &base), three);
    assert_eq!(apply(&total.inverse(&base), &three), base);
}

#[semio_framework_async_macros::async_test]
async fn insert_then_move_lands_at_the_final_slot_and_insert_then_remove_cancels() {
    let base = tree();
    let group = named(&base, "G");
    let fresh = create_drawing_shape_layer_rect("X");
    let fresh_id = id_of(&fresh);
    let created = diff_create_layer(&base.layers, None, 1, fresh);
    let one = apply(&created, &base);
    let moved = diff_reorder_layer(&one.layers, &fresh_id, Some(&PagedUtf8::<{usize::MAX}>::from(group.as_str())), 0);
    let total = sum(&created, &moved);
    let delta = total.layers.clone().expect("layers delta");
    assert!(delta.moved.is_empty() && delta.removed.is_empty() && delta.inserted.len() == 1, "insert then move is one insertion at the final address");
    assert_eq!((delta.inserted[0].parent_id.clone(), delta.inserted[0].index), (Some(group.clone()), 0));
    assert_eq!(apply(&total, &base), apply(&moved, &one));
    let removed = diff_remove_layer(&one.layers, &fresh_id);
    assert!(DiffAlgebra::is_empty(&sum(&created, &removed)), "insert then remove cancels");
}

#[semio_framework_async_macros::async_test]
async fn move_then_remove_removes_at_the_base_address_and_a_patch_of_an_inserted_layer_folds_in() {
    let base = tree();
    let group = named(&base, "G");
    let a = named(&base, "A");
    let moved = diff_reorder_layer(&base.layers, &a, Some(&PagedUtf8::<{usize::MAX}>::from(group.as_str())), 1);
    let one = apply(&moved, &base);
    let removed = diff_remove_layer(&one.layers, &a);
    let total = sum(&moved, &removed);
    let delta = total.layers.clone().expect("layers delta");
    assert_eq!(delta.removed, vec![DrawingLayerRemoval { id: a, parent_id: None, index: 0 }], "move then remove removes at the base address");
    assert!(delta.moved.is_empty() && delta.inserted.is_empty());
    assert_eq!(apply(&total, &base), apply(&removed, &one));
    let fresh = create_drawing_shape_layer_rect("X");
    let fresh_id = id_of(&fresh);
    let created = diff_create_layer(&base.layers, None, 0, fresh);
    let renamed = diff_set_layer_name(&fresh_id, "Renamed");
    let folded = sum(&created, &renamed);
    let delta = folded.layers.clone().expect("layers delta");
    assert!(delta.modified.is_empty() && delta.inserted.len() == 1, "a patch of an inserted layer folds into its insertion");
    assert_eq!(apply(&folded, &base), apply(&renamed, &apply(&created, &base)));
}

#[semio_framework_async_macros::async_test]
async fn a_stale_base_address_is_refused() {
    let base = tree();
    let a = named(&base, "A");
    let stale = DrawingDiff { layers: Some(DrawingLayersDelta { moved: vec![DrawingLayerRelocation { id: a, from: DrawingLayerAddress { parent_id: None, index: 2 }, to: DrawingLayerAddress { parent_id: None, index: 0 } }], ..Default::default() }), ..Default::default() };
    assert!(protocol::apply_diff(&stale, &base).is_err(), "a move whose layer is not at its base address does not apply");
    assert_eq!(address(&base, "A"), (None, 0));
}

#[semio_framework_async_macros::async_test]
async fn several_edits_inside_an_inserted_group_fold_by_mid_coordinates() {
    let base = tree();
    let (p, q, r, s) = (create_drawing_shape_layer_rect("P"), create_drawing_shape_layer_rect("Q"), create_drawing_shape_layer_rect("R"), create_drawing_shape_layer_rect("S"));
    let (p_id, q_id, r_id, s_id) = (id_of(&p), id_of(&q), id_of(&r), id_of(&s));
    let mut group = create_drawing_group_layer("X");
    if let DrawingLayerNode::Group(inner) = &mut group {
        inner.children.push(p);
        inner.children.push(q);
        inner.children.push(r);
    }
    let group_id = id_of(&group);
    let created = diff_create_layer(&base.layers, None, 0, group);
    let one = apply(&created, &base);
    let later = DrawingDiff {
        layers: Some(DrawingLayersDelta {
            removed: vec![
                DrawingLayerRemoval { id: p_id, parent_id: Some(group_id.clone()), index: 0 },
                DrawingLayerRemoval { id: r_id, parent_id: Some(group_id.clone()), index: 2 },
            ],
            inserted: vec![DrawingLayerInsertion { parent_id: Some(group_id.clone()), index: 0, layer: s }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let after = apply(&later, &one);
    let total = sum(&created, &later);
    let delta = total.layers.clone().expect("layers delta");
    assert!(delta.removed.is_empty() && delta.moved.is_empty() && delta.inserted.len() == 1, "everything folds into the one insertion");
    assert_eq!(apply(&total, &base), after, "the folded insertion equals the sequential application");
    let inside = find_node(&after.layers, &group_id).and_then(|node| match node {
        DrawingLayerNode::Group(group) => Some(group.children.iter().map(id_of).collect::<Vec<_>>()),
        _ => None,
    });
    assert_eq!(inside, Some(vec![s_id, q_id]), "two removals and one insertion landed by mid coordinates");
}
