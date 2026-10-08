//! 🧪️ `Block5dDiff` — apply, absorb, inverse and between laws over the id-keyed attribute rows and the field-patched kind identity.

use super::*;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_plugin_block::{BlockAttribute, BlockAttributePatch, BlockAttributesPatchEntry};

fn attribute(key: &str, value: &str) -> BlockAttribute {
    BlockAttribute { key: key.into(), value: value.into(), definition: None }
}

fn base() -> Block5dSnapshot {
    Block5dSnapshot { attributes: vec![attribute("a", "1"), attribute("b", "2")], ..Default::default() }
}

fn rows(delta: BlockAttributesDelta) -> Block5dDiff {
    Block5dDiff { attributes: Some(delta), ..Default::default() }
}

fn patch(key: &str, value: &str) -> BlockAttributesPatchEntry {
    BlockAttributesPatchEntry { id: key.into(), patch: BlockAttributePatch { value: Some(value.into()), ..Default::default() } }
}

fn absorbed(first: Block5dDiff, second: Block5dDiff) -> Block5dDiff {
    let mut sigma = first;
    MutationDiff::<Block5dSnapshot>::absorb(&mut sigma, second);
    sigma
}

fn assert_absorb_law(base: &Block5dSnapshot, first: &Block5dDiff, second: &Block5dDiff) -> Block5dDiff {
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
    assert_eq!(protocol::apply_diff(&Block5dDiff::default(), &base).expect("identity applies"), base);
    assert!(DiffAlgebra::<Block5dSnapshot>::is_empty(&Block5dDiff::default()));
}

/// 🔀 A row created and then deleted cancels out: nothing remains of it.
#[test]
fn create_then_delete_cancels() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta { added: vec![attribute("c", "3")], ..Default::default() }), &rows(BlockAttributesDelta { removed: vec!["c".into()], ..Default::default() }));
    assert!(DiffAlgebra::<Block5dSnapshot>::is_empty(&sigma));
}

/// 🔀 A base row patched and then deleted leaves only the deletion.
#[test]
fn patch_then_delete_leaves_the_deletion() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta { patched: vec![patch("a", "9")], ..Default::default() }), &rows(BlockAttributesDelta { removed: vec!["a".into()], ..Default::default() }));
    let delta = sigma.attributes.expect("rows survive");
    assert_eq!(delta.removed, vec!["a".to_string()]);
    assert!(delta.patched.is_empty() && delta.added.is_empty());
}

/// 🔀 Two patches of one row coalesce into one patch holding the later values.
#[test]
fn patch_then_patch_coalesces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta { patched: vec![patch("a", "8")], ..Default::default() }), &rows(BlockAttributesDelta { patched: vec![patch("a", "9")], ..Default::default() }));
    assert_eq!(sigma.attributes.expect("rows survive").patched, vec![patch("a", "9")]);
}

/// 🔀 A row deleted and created again is one replacement: removed and added together.
#[test]
fn delete_then_create_replaces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta { removed: vec!["a".into()], ..Default::default() }), &rows(BlockAttributesDelta { added: vec![attribute("a", "9")], ..Default::default() }));
    let delta = sigma.attributes.expect("rows survive");
    assert_eq!((delta.removed, delta.added), (vec!["a".to_string()], vec![attribute("a", "9")]));
}

/// 🔀 A row created and then patched is created already patched.
#[test]
fn create_then_patch_folds_into_the_row() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(BlockAttributesDelta { added: vec![attribute("c", "3")], ..Default::default() }), &rows(BlockAttributesDelta { patched: vec![patch("c", "4")], ..Default::default() }));
    let delta = sigma.attributes.expect("rows survive");
    assert_eq!(delta.added, vec![attribute("c", "4")]);
    assert!(delta.patched.is_empty());
}

/// 🪪️ Two identity patches coalesce per field.
#[test]
fn kind_patches_coalesce_per_field() {
    let base = base();
    let first = Block5dDiff { part_kind: Some(BlockKindIdentityPatch { name: Some("n1".into()), label: Some("l1".into()), ..Default::default() }), ..Default::default() };
    let second = Block5dDiff { part_kind: Some(BlockKindIdentityPatch { name: Some("n2".into()), ..Default::default() }), ..Default::default() };
    let sigma = assert_absorb_law(&base, &first, &second);
    assert_eq!(sigma.part_kind, Some(BlockKindIdentityPatch { name: Some("n2".into()), label: Some("l1".into()), ..Default::default() }));
}

/// 🔁️ The inverse diff restores the base exactly — rows, order and fields.
#[test]
fn inverse_restores_the_base() {
    let base = base();
    let diff = Block5dDiff {
        part_kind: Some(BlockKindIdentityPatch { name: Some("renamed".into()), ..Default::default() }),
        attributes: Some(BlockAttributesDelta { removed: vec!["a".into()], added: vec![attribute("c", "3")], patched: vec![patch("b", "7")], ..Default::default() }),
        ..Default::default()
    };
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<Block5dSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🧭️ `between` reaches the other document, and is empty between equal documents.
#[test]
fn between_reaches_the_other_document() {
    let base = base();
    let other = Block5dSnapshot { attributes: vec![attribute("b", "7"), attribute("c", "3"), attribute("a", "1")], ..Default::default() };
    let delta = <Block5dDiff as DiffAlgebra<Block5dSnapshot>>::between(&base, &other);
    assert_eq!(protocol::apply_diff(&delta, &base).expect("between applies"), other);
    assert!(DiffAlgebra::<Block5dSnapshot>::is_empty(&<Block5dDiff as DiffAlgebra<Block5dSnapshot>>::between(&base, &base)));
}

/// 🚫️ Removing a row the base never held is refused, not silently ignored.
#[test]
fn apply_refuses_a_missing_removal() {
    let diff = rows(BlockAttributesDelta { removed: vec!["ghost".into()], ..Default::default() });
    assert!(protocol::apply_diff(&diff, &base()).is_err());
}
