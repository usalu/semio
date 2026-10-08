use crate::apply_mutation;
use super::*;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use protocol::OpBinary;
use protocol::OpText;

/// 🧪️ Keeps the declaration honest, which nothing else can: the framework never parses Rust, so
/// the CATALOGS are what the contract gate counts against, and this is the only check that ties
/// them to the enum. BOTH are read, because AC1018 re-exports this vocabulary rather than
/// declaring one — a kind added here and catalogued under only one standard fails here.
#[test]
fn kinds_matches_every_variant_and_both_catalogs() {
    let from_variants: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(DwgMutation::kind).collect();
    let from_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
    assert_eq!(from_variants, from_kinds, "KINDS must equal every DwgMutation variant's kind()");
    assert_eq!(KINDS.len(), 12, "KINDS must list exactly the declared 12 kinds");
    for manifest in [include_str!("../../../../🔮️oracles/🔣️.json"), include_str!("../../../../../../../4️⃣ac1018/🪆️subsets/✳️any/🔮️oracles/🔣️.json")] {
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
        let diff = apply_mutation(&mut applied, &mutation);
        assert_eq!(protocol::apply_diff(diff.diff(), &base).expect("diff must apply to base"), applied);
        for inverse in mutation.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
            apply_mutation(&mut applied, &inverse);
        }
        assert_eq!(applied, base);
    }
    assert!(DwgDiff::default().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn operation_codecs_retain_logical_mutations() {
    for mutation in demo_mutation_cases() {
        assert_eq!(DwgMutation::parse_op(&mutation.print_op()).expect("text mutation"), mutation);
        assert_eq!(DwgMutation::decode_op(&mutation.encode_op().expect("binary mutation")).expect("binary mutation"), mutation);
    }
}

/// ⚖️ `dwg_mutation_inverse_sum_law`: `set-version-info` on a contentful AC1024 drawing (stamp pinned, preamble fields moved) and on a
/// preamble-only drawing (any AC10xx stamp), plus a no-op, all satisfy the inverse-sum law.
#[semio_framework_async_macros::async_test]
async fn dwg_mutation_inverse_sum_law_holds_for_every_leaf() {
    let contentful = crate::standards::v_ac1024::engine::demo_dwg_snapshot();
    let preamble_only = DwgSnapshot { version: "AC1018".into(), maintenance_version: 2, codepage: 30, ..Default::default() };
    let leaf = |version: &str, maintenance_version: u8, codepage: u16| DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: version.into(), maintenance_version, codepage });
    for (mutation, base) in [
        (leaf("AC1024", 9, 65001), &contentful),
        (leaf(&contentful.version, contentful.maintenance_version, 28), &contentful),
        (leaf("AC1032", 4, 28), &preamble_only),
        (leaf("AC1018", 2, 30), &preamble_only),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, base).await;
    }
    for mutation in demo_mutation_cases() {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &contentful).await;
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &preamble_only).await;
    }
}
