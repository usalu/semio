use crate::standards::v1::subsets::cad::io::binary::mutations::wire_tag;
use crate::standards::v1::subsets::cad::io::text::mutations::print_cad_mutation;
use super::*;

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling the binary op
/// frame's `tag` ordinal and the text grammar's keyword both use, and every one of those
/// spellings must also appear in the committed oracle manifest's catalog. The framework never
/// parses Rust, so this is what makes the declaration honest.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    assert_eq!(KINDS.len(), 16, "KINDS must name exactly one entry per declared SemioCadMutation variant");
    let mut seen = vec![false; KINDS.len()];
    for m in demo_mutation_cases() {
        let keyword = print_cad_mutation(&m).split(' ').next().expect("printed op is never empty").to_string();
        let ordinal = wire_tag(&m) as usize;
        assert_eq!(KINDS[ordinal], keyword, "KINDS must match the declaration order and spelling for {m:?}");
        seen[ordinal] = true;
    }
    assert!(seen.iter().all(|hit| *hit), "demo_mutation_cases must reach every KINDS entry, missing {:?}", KINDS.iter().zip(seen.iter()).filter(|(_, hit)| !**hit).map(|(kind, _)| *kind).collect::<Vec<_>>());
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

//#region 🧪️Law1_MutationDiffLaw
/// ⚖️ Law 1 — `mutation_diff_law`: for every variant, `apply_semio_cad_mutation`'s returned
/// diff equals `m.diff(base)`, and applying it matches `diff.diff().apply(base)`.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = fixture();
    for m in demo_mutation_cases() {
        let mut snap = base.clone();
        let (__next, returned) = crate::applied(&snap, &m);
        snap = __next;
        let expected_diff = m.diff(&base);
        assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
        assert_eq!(snap, protocol::apply_diff(expected_diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), "apply mismatch for {m:?}");
    }
}
//#endregion

//#region 🧪️Law2_InverseLaw
/// ⚖️ Law 2 — `inverse_law`: every mutation round-trips (mutation-level) and every diff
/// round-trips (diff-level `d.diff().inverse(base).apply(&d.diff().apply(base)) == base`).
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    use protocol::command::DiffAlgebra;
    let base = fixture();
    for m in demo_mutation_cases() {
        let mut snap = base.clone();
        snap = crate::applied(&snap, &m).0;
        for inv in m.inverse(&base).expect("valid retained mutation inverse fixture") {
            let mut undone = snap.clone();
            undone = crate::applied(&undone, &inv).0;
            assert_eq!(undone, base, "mutation-level inverse mismatch for {m:?}");
        }

        let d = m.diff(&base);
        let after = protocol::apply_diff(d.diff(), &base).expect("apply must succeed for a well-formed fixture");
        let d_inv = d.diff().inverse(&base);
        assert_eq!(protocol::apply_diff(&d_inv, &after).expect("apply must succeed for a well-formed fixture"), base, "diff-level inverse mismatch for {m:?}");
    }
}
//#endregion

//#region 🧪️Law7_OpTextBinaryRoundtripLaw
/// ⚖️ Law 7 — `op_text_binary_roundtrip_law`: `OpText`/`OpBinary` round-trip for the
/// hand-rolled `SemioCadMutation` grammar, covering every variant via [`demo_mutation_cases`].
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for m in demo_mutation_cases() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioCadMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioCadMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion

/// 🎯️ Position law: removing ANY layer, block, entity or block entity (first, middle, last) is undone at its original index.
#[semio_framework_async_macros::async_test]
async fn removals_invert_at_every_position() {
    let base = crate::standards::v1::subsets::cad::schema::snapshot::demo_cad_snapshot();
    for item in &base.layers {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: item.name.clone() }), &base).await;
    }
    for item in &base.blocks {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name: item.name.clone() }), &base).await;
        for entity in &item.entities {
            protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name: item.name.clone(), handle: entity.handle.clone() }), &base).await;
        }
    }
    for item in &base.entities {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle: item.handle.clone() }), &base).await;
    }
}

//#region ↩️LeafInverseLaws
#[path = "../../✂️remove-block-entity/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_block_entity;
#[path = "../../🎚️set-layer/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_layer;
#[path = "../../🏳️set-entity-layer/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_entity_layer;
#[path = "../../📍set-block-base-point/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_block_base_point;
#[path = "../../📐set-entity-geometry/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_entity_geometry;
#[path = "../../🔷add-entity/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_add_entity;
#[path = "../../🔺set-block-entity-geometry/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_block_entity_geometry;
#[path = "../../🗂️add-layer/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_add_layer;
#[path = "../../🗑️remove-entity/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_entity;
#[path = "../../🚫remove-block/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_block;
#[path = "../../🧩add-block-entity/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_add_block_entity;
#[path = "../../🧱add-block/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_add_block;
#[path = "../../🧷️set-block-entity-layer/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_block_entity_layer;
#[path = "../../🧹remove-layer/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_layer;
//#endregion ↩️LeafInverseLaws
