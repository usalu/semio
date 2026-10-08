use crate::apply_mutation;
use super::*;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn base() -> BinarySnapshot {
    BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let b = base();
    for m in demo_mutation_cases() {
        let mut via_apply = b.clone();
        let returned = apply_mutation(&mut via_apply, &m);
        let expected_diff = m.diff(&b);
        assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
        assert_eq!(via_apply, protocol::apply_diff(expected_diff.diff(), &b).unwrap(), "apply mismatch for {m:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let b = base();
    for m in demo_mutation_cases() {
        let mut mutated = b.clone();
        apply_mutation(&mut mutated, &m);
        for undo in m.inverse(&b).expect("valid retained mutation inverse fixture").into_iter().rev() {
            apply_mutation(&mut mutated, &undo);
        }
        assert_eq!(mutated, b, "mutation-level inverse round-trip failed for {m:?}");
    }
    for m in demo_mutation_cases() {
        let d = m.diff(&b);
        let next = protocol::apply_diff(d.diff(), &b).unwrap();
        let inv = d.diff().inverse(&b);
        assert_eq!(protocol::apply_diff(&inv, &next).unwrap(), b, "diff-level inverse round-trip failed for {m:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_cartesian() {
    let b = base();
    let variants = demo_mutation_cases();
    let mut valid_compositions = 0;
    let mut rejected_compositions = 0;
    for m1 in &variants {
        let d1 = m1.diff(&b);
        let mid = protocol::apply_diff(d1.diff(), &b).unwrap();
        for m2 in &variants {
            let d2 = m2.diff(&mid);
            let Ok(after) = protocol::apply_diff(d2.diff(), &mid) else {
                rejected_compositions += 1;
                continue;
            };
            let mut merged = d1.diff().clone();
            merged.absorb(d2.diff().clone());
            assert_eq!(protocol::apply_diff(&merged, &b).unwrap(), after, "absorb({m1:?}, {m2:?}) mismatch");
            valid_compositions += 1;
        }
    }
    assert_eq!(valid_compositions, 9, "the representative matrix must exercise every valid sequential composition");
    assert_eq!(rejected_compositions, 0, "no representative pair leaves the algebra domain");
}

/// 🧪️ F6-PILOT: `OpText`/`OpBinary` round-trip laws (handcrafted impls over the
/// `semio_framework_dsl_record_derive::DslEnum`-derived `DslVariants`).
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
    assert!(BinaryMutation::parse_op("splice offset=1 remove-len=2 insert=\"qrvM\"").is_err());
    assert!(BinaryMutation::parse_op("replace-byte-range-extra offset=1 remove-len=2 insert=\"qrvM\"").is_err());
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
            BinaryMutation::ReplaceByteRange(_) => "replace-byte-range",
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
    assert_eq!(KINDS.len(), 3, "binary-raw-any declares 3 BinaryMutation variants");
}
//#endregion 🔖️KindsCoverageLaw

/// ⚖️ `mutation_inverse_sum_law`: for every leaf the inverse diffs sum to the negative forward diff.
#[semio_framework_async_macros::async_test]
async fn mutation_inverse_sum_law_holds_for_every_leaf() {
    let b = base();
    for mutation in demo_mutation_cases() {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &b).await;
    }
}
