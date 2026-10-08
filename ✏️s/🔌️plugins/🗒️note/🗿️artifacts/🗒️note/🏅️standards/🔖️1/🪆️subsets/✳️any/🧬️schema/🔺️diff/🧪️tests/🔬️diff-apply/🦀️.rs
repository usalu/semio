use super::*;
use crate::schema::{block_id, create_block_by_kind, NoteIdOwner};
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};

fn text(owner: &mut NoteIdOwner) -> NoteBlockNode {
    create_block_by_kind(owner, "text", 0.0, 0.0)
}

fn group(owner: &mut NoteIdOwner) -> NoteBlockNode {
    create_block_by_kind(owner, "group", 0.0, 0.0)
}

fn snapshot_with(blocks: Vec<NoteBlockNode>) -> NoteSnapshot {
    NoteSnapshot { blocks, ..NoteSnapshot::default() }
}

fn ids(snapshot: &NoteSnapshot) -> Vec<String> {
    crate::schema::flatten_blocks(&snapshot.blocks).into_iter().map(|block| block_id(block).to_string()).collect()
}

fn rename(id: &str, name: &str) -> NoteDiff {
    NoteDiff::block_patches([(id.to_string(), NoteBlockPatch { name: Some(name.into()), ..Default::default() })])
}

#[semio_framework_async_macros::async_test]
async fn malformed_nested_parent_rejects_without_changing_the_base() {
    let base = NoteSnapshot::default();
    let block = text(&mut NoteIdOwner::new("diff-hostile-test", 0));
    let diff = NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: Some("missing-group".into()), index: 0, block }]);
    let error = protocol::apply_diff(&diff, &base).expect_err("missing nested parent must reject");
    assert_eq!(error.code, "mutation.apply.missing-target");
    assert_eq!(error.target, ["blocks", "rows", "0", "parentId"]);
    assert!(base.blocks.is_empty());
}

/// 🚫️ Malformed rows are rejected with a typed error: an insertion past the container, a duplicate identity, a missing patch or move target, a move into the block's own subtree, a patch slot the block kind lacks.
#[semio_framework_async_macros::async_test]
async fn malformed_rows_are_rejected() {
    let mut owner = NoteIdOwner::new("diff-malformed", 0);
    let a = text(&mut owner);
    let g = group(&mut owner);
    let g_id = block_id(&g).to_string();
    let base = snapshot_with(vec![a.clone(), g]);
    for diff in [
        NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: None, index: 5, block: text(&mut owner) }]),
        NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: None, index: 0, block: a.clone() }]),
        NoteDiff::block_rows(vec![NoteBlockRow::Move { id: "ghost".into(), parent_id: None, index: 0 }]),
        NoteDiff::block_rows(vec![NoteBlockRow::Move { id: g_id.clone(), parent_id: Some(g_id.clone()), index: 0 }]),
        rename("ghost", "x"),
        NoteDiff::block_patches([(block_id(&a).to_string(), NoteBlockPatch { tex: Some("x".into()), ..Default::default() })]),
    ] {
        assert!(protocol::apply_diff(&diff, &base).is_err(), "{diff:?} must be rejected");
    }
}

/// ➕️ Absorb coalesces same-key rows: add∘remove cancels, add∘move keeps the last destination, patch∘patch merges per slot across other blocks' patches, move∘remove keeps the remove, and asset rows follow the insert/replace/remove table.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_rows_of_one_key() {
    let mut owner = NoteIdOwner::new("diff-absorb", 0);
    let block = text(&mut owner);
    let id = block_id(&block).to_string();
    let mut created = NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: None, index: 0, block: block.clone() }]);
    created.absorb(NoteDiff::block_rows(vec![NoteBlockRow::Remove { id: id.clone() }]));
    assert_eq!(created.blocks, Some(NoteBlocksDelta::default()), "create∘delete leaves nothing");

    let mut placed = NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: None, index: 0, block }]);
    placed.absorb(NoteDiff::block_rows(vec![NoteBlockRow::Move { id: id.clone(), parent_id: None, index: 3 }]));
    assert!(matches!(placed.blocks.expect("blocks delta").rows.as_slice(), [NoteBlockRow::Add { index: 3, .. }]), "add∘move is an add at the last destination");

    let mut patched = rename("a", "First");
    patched.absorb(rename("b", "Other"));
    patched.absorb(NoteDiff::block_patches([("a".to_string(), NoteBlockPatch { x: Some(4.0), ..Default::default() })]));
    let rows = patched.blocks.expect("blocks delta").rows;
    assert_eq!(rows.len(), 2, "patch∘patch of one block is one row");
    assert!(matches!(&rows[0], NoteBlockRow::Patch { id, patch } if id == "a" && patch.name.as_deref() == Some("First") && patch.x == Some(4.0)));

    let mut dropped = NoteDiff::block_rows(vec![NoteBlockRow::Move { id: "a".into(), parent_id: None, index: 1 }]);
    dropped.absorb(NoteDiff::block_rows(vec![NoteBlockRow::Remove { id: "a".into() }]));
    assert_eq!(dropped.blocks.expect("blocks delta").rows, vec![NoteBlockRow::Remove { id: "a".into() }], "move∘remove keeps the remove");

    let asset = NoteImageAsset { mime: "image/png".into(), data: "d".into(), width: None, height: None };
    let other = NoteImageAsset { mime: "image/jpeg".into(), data: "e".into(), width: None, height: None };
    let mut assets = NoteDiff::asset_rows(vec![NoteAssetRow::Insert { key: "k".into(), asset: asset.clone() }]);
    assets.absorb(NoteDiff::asset_rows(vec![NoteAssetRow::Replace { key: "k".into(), asset: other.clone() }]));
    assert_eq!(assets.assets.as_ref().expect("assets delta").rows, vec![NoteAssetRow::Insert { key: "k".into(), asset: other.clone() }], "insert∘replace is an insert of the last value");
    assets.absorb(NoteDiff::asset_rows(vec![NoteAssetRow::Remove { key: "k".into() }]));
    assert_eq!(assets.assets, Some(NoteAssetsDelta::default()), "insert∘remove cancels");
    let mut replaced = NoteDiff::asset_rows(vec![NoteAssetRow::Remove { key: "k".into() }]);
    replaced.absorb(NoteDiff::asset_rows(vec![NoteAssetRow::Insert { key: "k".into(), asset }]));
    assert!(matches!(replaced.assets.expect("assets delta").rows.as_slice(), [NoteAssetRow::Replace { .. }]), "remove∘insert is a replace");
}

/// ➕️ LAW: `absorb(d1, d2)` applies like `d1` then `d2`, over structural and patch rows including a group's subtree.
#[semio_framework_async_macros::async_test]
async fn absorb_equals_sequential_application() {
    let mut owner = NoteIdOwner::new("diff-sequential", 0);
    let (a, b, g) = (text(&mut owner), text(&mut owner), group(&mut owner));
    let (a_id, b_id, g_id) = (block_id(&a).to_string(), block_id(&b).to_string(), block_id(&g).to_string());
    let base = snapshot_with(vec![a, b, g]);
    let fresh = text(&mut owner);
    let fresh_id = block_id(&fresh).to_string();
    let firsts = [
        rename(&a_id, "Renamed"),
        NoteDiff::block_rows(vec![NoteBlockRow::Move { id: b_id.clone(), parent_id: Some(g_id.clone()), index: 0 }]),
        NoteDiff::block_rows(vec![NoteBlockRow::Remove { id: a_id.clone() }]),
        NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: Some(g_id.clone()), index: 0, block: fresh }]),
    ];
    let seconds = [rename(&a_id, "Again"), rename(&fresh_id, "Fresh"), NoteDiff::block_rows(vec![NoteBlockRow::Move { id: a_id.clone(), parent_id: None, index: 0 }]), NoteDiff::block_rows(vec![NoteBlockRow::Remove { id: b_id.clone() }])];
    for d1 in &firsts {
        let mid = protocol::apply_diff(d1, &base).expect("first diff applies");
        for d2 in &seconds {
            if protocol::apply_diff(d2, &mid).is_ok() {
                assert_mutation_diff_absorb_law(&base, d1.clone(), d2.clone()).await;
            }
        }
    }
}

/// 🔁️ LAW: the negative delta applied after the diff restores the base, for adds, removals, moves across containers, field patches and table edits.
#[semio_framework_async_macros::async_test]
async fn the_negative_delta_restores_the_base() {
    let mut owner = NoteIdOwner::new("diff-negative", 0);
    let (a, g, table) = (text(&mut owner), group(&mut owner), create_block_by_kind(&mut owner, "table", 0.0, 0.0));
    let (a_id, g_id, table_id) = (block_id(&a).to_string(), block_id(&g).to_string(), block_id(&table).to_string());
    let base = snapshot_with(vec![a, g, table]);
    let fresh = text(&mut owner);
    let mut diff = NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: Some(g_id.clone()), index: 0, block: fresh }]);
    diff.absorb(NoteDiff::block_rows(vec![NoteBlockRow::Move { id: a_id.clone(), parent_id: Some(g_id), index: 1 }]));
    diff.absorb(NoteDiff::block_patches([(table_id, NoteBlockPatch { table: Some(vec![NoteTableEdit::RemoveRow { index: 1 }, NoteTableEdit::InsertColumn { index: 0, name: "Z".into(), cells: vec![NoteTableCell { content: "q".into() }] }]), x: Some(9.0), ..Default::default() })]));
    diff.absorb(NoteDiff { title: Some(NoteAssigned::new(Some("Titled".into()))), grid_opacity: Some(NoteAssigned::new(None)), ..Default::default() });
    assert_diff_algebra_inverse_law::<NoteSnapshot, NoteDiff>(&base, &diff).await;
    assert_eq!(ids(&protocol::apply_diff(&diff, &base).expect("diff applies")).len(), 4);
}

/// 🧭️ LAW: `between(a, b)` carries `a` to `b` and `between(a, a)` is empty.
#[semio_framework_async_macros::async_test]
async fn between_reaches_the_other_snapshot() {
    let mut owner = NoteIdOwner::new("diff-between", 0);
    let (a, b, c) = (text(&mut owner), text(&mut owner), text(&mut owner));
    let mut renamed = b.clone();
    if let NoteBlockNode::Text { name, .. } = &mut renamed {
        *name = "Renamed".into();
    }
    let from = snapshot_with(vec![a.clone(), b]);
    let mut to = snapshot_with(vec![c, renamed, a]);
    to.title = Some("Title".into());
    to.assets.insert("k".into(), NoteImageAsset { mime: "image/png".into(), data: "d".into(), width: None, height: None });
    assert_diff_algebra_between_law::<NoteSnapshot, NoteDiff>(&from, &to).await;
}
