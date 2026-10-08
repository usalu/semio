//! 🧪️ `Block2dDiff` — apply, absorb and inverse laws over the positional attribute rows and the field-patched kind identity.

use super::*;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_plugin_block::{BlockAttribute, BlockAttributePatch, BlockAttributesInsertion, BlockAttributesPatchEntry, BlockAttributesRelocation, BlockAttributesRemoval};

fn attribute(key: &str, value: &str) -> BlockAttribute {
    BlockAttribute { key: key.into(), value: value.into(), definition: None }
}

fn list(keys: &[&str]) -> Vec<BlockAttribute> {
    keys.iter().map(|key| attribute(key, "1")).collect()
}

fn base() -> Block2dSnapshot {
    Block2dSnapshot { attributes: vec![attribute("a", "1"), attribute("b", "2")], ..Default::default() }
}

fn rows(delta: BlockAttributesDelta) -> Block2dDiff {
    Block2dDiff { attributes: delta, ..Default::default() }
}

fn patch(key: &str, value: &str) -> BlockAttributesPatchEntry {
    BlockAttributesPatchEntry { id: key.into(), patch: BlockAttributePatch { value: Some(value.into()), ..Default::default() } }
}

fn absorbed(first: Block2dDiff, second: Block2dDiff) -> Block2dDiff {
    let mut sigma = first;
    MutationDiff::<Block2dSnapshot>::absorb(&mut sigma, second);
    sigma
}

fn assert_absorb_law(base: &Block2dSnapshot, first: &Block2dDiff, second: &Block2dDiff) -> Block2dDiff {
    let sigma = absorbed(first.clone(), second.clone());
    let mid = protocol::apply_diff(first, base).expect("first applies");
    let sequential = protocol::apply_diff(second, &mid).expect("second applies");
    assert_eq!(protocol::apply_diff(&sigma, base).expect("absorbed applies"), sequential, "absorb must equal sequential application");
    sigma
}

/// 🕳️ The identity delta leaves the document untouched.
#[test]
fn empty_diff_is_the_identity() {
    let base = base();
    assert_eq!(protocol::apply_diff(&Block2dDiff::default(), &base).expect("identity applies"), base);
    assert!(DiffAlgebra::<Block2dSnapshot>::is_empty(&Block2dDiff::default()));
}

/// 🔀 A row created and then deleted cancels out: nothing remains of it.
#[test]
fn create_then_delete_cancels() {
    let base = base();
    let created = attribute("c", "3");
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta::insertion(2, created.clone())), &rows(BlockAttributesDelta::removal(&[attribute("a", "1"), attribute("b", "2"), created], 2)));
    assert!(DiffAlgebra::<Block2dSnapshot>::is_empty(&sigma));
}

/// 🔀 A base row patched and then deleted leaves only the deletion at its base index.
#[test]
fn patch_then_delete_leaves_the_deletion() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta { modified: vec![patch("a", "9")], ..Default::default() }), &rows(BlockAttributesDelta::removal(&[attribute("a", "9"), attribute("b", "2")], 0)));
    let delta = sigma.attributes;
    assert_eq!(delta.removed, vec![BlockAttributesRemoval { id: "a".into(), index: 0 }]);
    assert!(delta.modified.is_empty() && delta.inserted.is_empty());
}

/// 🔀 Two patches of one row coalesce into one patch holding the later values.
#[test]
fn patch_then_patch_coalesces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta { modified: vec![patch("a", "8")], ..Default::default() }), &rows(BlockAttributesDelta { modified: vec![patch("a", "9")], ..Default::default() }));
    assert_eq!(sigma.attributes.modified, vec![patch("a", "9")]);
}

/// 🔀 A row deleted and created again is one replacement: removed at its base index and inserted at its after index.
#[test]
fn delete_then_create_replaces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta::removal(&base.attributes, 0)), &rows(BlockAttributesDelta::insertion(0, attribute("a", "9"))));
    let delta = sigma.attributes;
    assert_eq!((delta.removed, delta.inserted), (vec![BlockAttributesRemoval { id: "a".into(), index: 0 }], vec![BlockAttributesInsertion { index: 0, row: attribute("a", "9") }]));
}

/// 🔀 A row created and then patched is created already patched.
#[test]
fn create_then_patch_folds_into_the_row() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta::insertion(2, attribute("c", "3"))), &rows(BlockAttributesDelta { modified: vec![patch("c", "4")], ..Default::default() }));
    let delta = sigma.attributes;
    assert_eq!(delta.inserted, vec![BlockAttributesInsertion { index: 2, row: attribute("c", "4") }]);
    assert!(delta.modified.is_empty());
}

/// ↕️ Two moves of one row coalesce into one move from its base index to its final index.
#[test]
fn move_then_move_is_one_move() {
    let base = Block2dSnapshot { attributes: list(&["a", "b", "c", "d"]), ..Default::default() };
    let first = rows(BlockAttributesDelta { moved: vec![BlockAttributesRelocation { id: "a".into(), from: 0, to: 2 }], ..Default::default() });
    let second = rows(BlockAttributesDelta { moved: vec![BlockAttributesRelocation { id: "a".into(), from: 2, to: 3 }], ..Default::default() });
    let sigma = assert_absorb_law(&base, &first, &second);
    assert_eq!(sigma.attributes.moved, vec![BlockAttributesRelocation { id: "a".into(), from: 0, to: 3 }]);
}

/// 🪪️ Two identity patches coalesce per field.
#[test]
fn kind_patches_coalesce_per_field() {
    let base = base();
    let first = Block2dDiff { node_kind: Some(BlockKindIdentityPatch { name: Some("n1".into()), label: Some("l1".into()), ..Default::default() }), ..Default::default() };
    let second = Block2dDiff { node_kind: Some(BlockKindIdentityPatch { name: Some("n2".into()), ..Default::default() }), ..Default::default() };
    let sigma = assert_absorb_law(&base, &first, &second);
    assert_eq!(sigma.node_kind, Some(BlockKindIdentityPatch { name: Some("n2".into()), label: Some("l1".into()), ..Default::default() }));
}

/// 🔁️ The inverse diff restores the base exactly — rows, positions and fields.
#[test]
fn inverse_restores_the_base() {
    let base = base();
    let diff = Block2dDiff {
        node_kind: Some(BlockKindIdentityPatch { name: Some("renamed".into()), ..Default::default() }),
        attributes: BlockAttributesDelta {
            removed: vec![BlockAttributesRemoval { id: "a".into(), index: 0 }],
            inserted: vec![BlockAttributesInsertion { index: 1, row: attribute("c", "3") }],
            modified: vec![patch("b", "7")],
            ..Default::default()
        },
        ..Default::default()
    };
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<Block2dSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🎯️ Removing, inserting and moving a MIDDLE row are each undone by the inverse at the row's original position.
#[test]
fn inverse_restores_middle_rows() {
    let base = Block2dSnapshot { attributes: list(&["a", "b", "c", "d"]), ..Default::default() };
    for delta in [
        BlockAttributesDelta::removal(&base.attributes, 1),
        BlockAttributesDelta::insertion(2, attribute("x", "1")),
        BlockAttributesDelta { moved: vec![BlockAttributesRelocation { id: "b".into(), from: 1, to: 3 }], ..Default::default() },
    ] {
        let diff = rows(delta);
        let after = protocol::apply_diff(&diff, &base).expect("diff applies");
        let inverse = DiffAlgebra::<Block2dSnapshot>::inverse(&diff, &base);
        assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
    }
}

/// 🚫️ Removing a row that is not at its stated base index is refused, not silently ignored.
#[test]
fn apply_refuses_a_missing_removal() {
    let diff = rows(BlockAttributesDelta { removed: vec![BlockAttributesRemoval { id: "ghost".into(), index: 0 }], ..Default::default() });
    assert!(protocol::apply_diff(&diff, &base()).is_err());
}
