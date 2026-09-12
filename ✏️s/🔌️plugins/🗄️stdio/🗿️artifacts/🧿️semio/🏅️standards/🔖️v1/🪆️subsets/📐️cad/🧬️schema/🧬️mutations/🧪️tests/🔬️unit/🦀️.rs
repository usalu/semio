use super::*;

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling the binary op
/// frame's `tag` ordinal and the text grammar's keyword both use, and every one of those
/// spellings must also appear in the committed oracle manifest's catalog. The framework never
/// parses Rust, so this is what makes the declaration honest.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    assert_eq!(KINDS.len(), 15, "KINDS must name exactly one entry per declared SemioCadMutation variant");
    let mut seen = vec![false; KINDS.len()];
    for m in demo_mutation_cases() {
        let keyword = print_cad_mutation(&m).split(' ').next().expect("printed op is never empty").to_string();
        let ordinal = variant_ordinal(&m) as usize;
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
        let returned = apply_semio_cad_mutation(&mut snap, &m);
        let expected_diff = m.diff(&base);
        assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
        assert_eq!(snap, protocol::MutationDiff::apply(expected_diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), "apply mismatch for {m:?}");
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
        apply_semio_cad_mutation(&mut snap, &m);
        for inv in m.inverse(&base) {
            let mut undone = snap.clone();
            apply_semio_cad_mutation(&mut undone, &inv);
            assert_eq!(undone, base, "mutation-level inverse mismatch for {m:?}");
        }

        let d = m.diff(&base);
        let after = protocol::MutationDiff::apply(d.diff(), &base).expect("apply must succeed for a well-formed fixture");
        let d_inv = d.diff().inverse(&base);
        assert_eq!(protocol::MutationDiff::apply(&d_inv, &after).expect("apply must succeed for a well-formed fixture"), base, "diff-level inverse mismatch for {m:?}");
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
