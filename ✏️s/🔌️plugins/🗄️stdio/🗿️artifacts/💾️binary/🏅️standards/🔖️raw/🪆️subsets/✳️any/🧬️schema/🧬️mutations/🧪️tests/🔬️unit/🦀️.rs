
use super::*;
use protocol::MutationDiff;
use protocol::os_spr::command::DiffAlgebra;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn base() -> BinarySnapshot {
    BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let b = base();
    for m in demo_mutation_cases() {
        let mut via_apply = b.clone();
        let returned = apply_binary_mutation(&mut via_apply, &m);
        let expected_diff = m.diff(&b);
        assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
        assert_eq!(via_apply, expected_diff.diff().apply(&b).unwrap(), "apply mismatch for {m:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let b = base();
    for m in demo_mutation_cases() {
        let mut mutated = b.clone();
        apply_binary_mutation(&mut mutated, &m);
        for undo in m.inverse(&b) {
            apply_binary_mutation(&mut mutated, &undo);
        }
        assert_eq!(mutated, b, "mutation-level inverse round-trip failed for {m:?}");
    }
    for m in demo_mutation_cases() {
        let d = m.diff(&b);
        let next = d.diff().apply(&b).unwrap();
        let inv = d.diff().inverse(&b);
        assert_eq!(inv.apply(&next).unwrap(), b, "diff-level inverse round-trip failed for {m:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_cartesian() {
    let b = base();
    let variants = demo_mutation_cases();
    for m1 in &variants {
        let d1 = m1.diff(&b);
        let mid = d1.diff().apply(&b).unwrap();
        for m2 in &variants {
            let d2 = m2.diff(&mid);
            let after = d2.diff().apply(&mid).unwrap();
            let mut merged = d1.diff().clone();
            merged.absorb(d2.diff().clone());
            assert_eq!(merged.apply(&b).unwrap(), after, "absorb({m1:?}, {m2:?}) mismatch");
        }
    }
}

/// 🧪️ F6-PILOT: `OpText`/`OpBinary` round-trip laws (handcrafted impls over the
/// `dsl::DslOps`-derived `DslVariants`).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for m in demo_mutation_cases() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = BinaryMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = BinaryMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}

//#region 🔖️KindsCoverageLaw
/// 🏷️ `KINDS` must name exactly the enum's variants (kebab-case), one entry each — an
/// exhaustive `match` so the compiler itself fails the moment a variant is added, renamed or
/// removed without this list being updated alongside it. The manifest side of the same claim
/// (`../../🔣️oracle.json`'s `binary-raw-any` catalog `kinds`) is checked by the
/// mutate/inverse test case's own contract gate, which fails if the two lists ever diverge.
#[semio_framework_async_macros::async_test]
async fn kinds_cover_every_variant() {
    fn kind_of(mutation: &BinaryMutation) -> &'static str {
        match mutation {
            BinaryMutation::SetSnapshot(_) => "set-snapshot",
            BinaryMutation::ReplaceByteRange(_) => "splice",
            BinaryMutation::AppendBytes(_) => "append-bytes",
            BinaryMutation::TruncateAt(_) => "truncate-at",
        }
    }
    let mut exercised: Vec<&str> = demo_mutation_cases().iter().map(kind_of).collect();
    exercised.sort_unstable();
    exercised.dedup();
    let mut declared: Vec<&str> = KINDS.to_vec();
    declared.sort_unstable();
    assert_eq!(exercised, declared, "KINDS must name exactly the variants demo_mutation_cases() exercises");
    assert_eq!(KINDS.len(), 4, "binary-raw-any declares 4 BinaryMutation variants");
}
//#endregion 🔖️KindsCoverageLaw
