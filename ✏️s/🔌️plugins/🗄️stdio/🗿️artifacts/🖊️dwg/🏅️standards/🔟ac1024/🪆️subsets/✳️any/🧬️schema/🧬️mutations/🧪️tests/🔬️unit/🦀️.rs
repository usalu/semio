
use super::*;
use protocol::MutationDiff;
use protocol::OpBinary;
use protocol::OpText;
use protocol::command::DiffAlgebra;

/// 🧪️ Keeps the declaration honest, which nothing else can: the framework never parses Rust, so
/// the CATALOGS are what the contract gate counts against, and this is the only check that ties
/// them to the enum. BOTH are read, because AC1018 re-exports this vocabulary rather than
/// declaring one — a kind added here and catalogued under only one standard fails here.
#[test]
fn kinds_matches_every_variant_and_both_catalogs() {
    let from_variants: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(DwgMutation::kind).collect();
    let from_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
    assert_eq!(from_variants, from_kinds, "KINDS must equal every DwgMutation variant's kind()");
    assert_eq!(KINDS.len(), 2, "KINDS must list exactly the declared 2 kinds");
    for manifest in [include_str!("../../../../🔮️oracle/🔣️.json"), include_str!("../../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🔮️oracle/🔣️.json")] {
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "a committed DWG catalog is missing kind {kind:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn logical_mutations_obey_diff_and_inverse_laws() {
    let base = DwgSnapshot { version: "AC1024".into(), maintenance_version: 2, codepage: 30, ..Default::default() };
    for mutation in demo_mutation_cases() {
        let mut applied = base.clone();
        let diff = apply_dwg_mutation(&mut applied, &mutation);
        assert_eq!(diff.diff().apply(&base).expect("diff must apply to base"), applied);
        for inverse in mutation.inverse(&base) {
            apply_dwg_mutation(&mut applied, &inverse);
        }
        assert_eq!(applied, base);
    }
    assert!(DwgDiff::between(&base, &base).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn operation_codecs_retain_logical_mutations() {
    for mutation in demo_mutation_cases() {
        assert_eq!(DwgMutation::parse_op(&mutation.print_op()).expect("text mutation"), mutation);
        assert_eq!(DwgMutation::decode_op(&mutation.encode_op().expect("binary mutation")).expect("binary mutation"), mutation);
    }
}
