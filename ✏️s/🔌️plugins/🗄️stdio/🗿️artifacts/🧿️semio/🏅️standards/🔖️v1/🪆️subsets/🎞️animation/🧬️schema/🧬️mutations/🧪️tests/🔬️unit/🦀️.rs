use super::*;

/// 🧪️ `kinds_match_the_enum_and_the_catalog`: `KINDS` names every declared variant, at the
/// position `variant_ordinal` assigns it, and every one of those names also appears in the
/// committed oracle manifest's catalog. The bijection against `demo_mutation_cases` is what
/// makes a newly added variant fail here instead of silently shrinking the vocabulary
/// `🎞️mutate-semio-animation` claims to cover.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    assert_eq!(KINDS.len(), OP_KEYWORDS.len(), "KINDS must name exactly one kind per declared variant, same length as the op tag table");
    let mut seen = vec![false; KINDS.len()];
    for mutation in demo_mutation_cases() {
        let ordinal = variant_ordinal(&mutation) as usize;
        assert!(!seen[ordinal], "ordinal {ordinal} is represented twice — demo_mutation_cases must carry exactly one case per declared variant");
        seen[ordinal] = true;
    }
    assert!(seen.iter().all(|hit| *hit), "every declared variant must be represented in demo_mutation_cases");
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}

/// 🧪️ mutation_diff_law: `m.diff(base).diff().apply(base) == { apply_x_mutation(&mut s, m); s }`.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_covers_every_variant() {
    let base = fixture();
    for m in demo_mutation_cases() {
        let diff = <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(&m, &base);
        let via_diff = diff.diff().apply(&base).expect("apply must succeed for a well-formed fixture");

        let mut applied = base.clone();
        let returned_diff = apply_semio_animation_mutation(&mut applied, &m);

        assert_eq!(via_diff, applied, "diff().apply(base) must match apply_semio_animation_mutation's result for {m:?}");
        assert_eq!(returned_diff, diff, "apply_semio_animation_mutation must return the same diff as Mutation::diff for {m:?}");
    }
}

/// 🧪️ inverse_law: every variant's inverse restores `base` when applied after the mutation.
#[semio_framework_async_macros::async_test]
async fn inverse_law_covers_every_variant() {
    let base = fixture();
    for m in demo_mutation_cases() {
        let mut mutated = base.clone();
        let _ = apply_semio_animation_mutation(&mut mutated, &m);
        let inv = <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(&m, &base);
        let mut restored = mutated.clone();
        for step in &inv {
            let _ = apply_semio_animation_mutation(&mut restored, step);
        }
        assert_eq!(restored, base, "inverse must restore base for {m:?}");
    }
}

/// 🧪️ op_text_binary_roundtrip_law: handcrafted `OpText`/`OpBinary` round trip for every
/// variant.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let _base = fixture();
    for m in demo_mutation_cases() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioAnimationMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioAnimationMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
