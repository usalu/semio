
use super::*;
use semio_repo_test_host::parse_json;

#[test]
fn direct_language_neutral_vectors_match_lopdf_and_concrete_inverse() {
    macro_rules! vector {
        ($mutation:literal) => {
            (
                include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/🎯️outcome/🔣️.json")),
                include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/📸️snapshot/⬅️before/🔣️.json")),
                include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/📸️snapshot/➡️after/🔣️.json")),
                include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/🔺️diff/🔣️.json")),
                include_str!(concat!("../🧬️schema/🧬️mutations/", $mutation, "/🧪️tests/round-trips-the-concrete-inverse/🦠️mutation/🔣️.json")),
            )
        };
    }
    let vectors = [vector!("📥️insert-page"), vector!("🗑️remove-page"), vector!("🔀️move-page"), vector!("📐️resize-page"), vector!("♻️replace-page-text")];
    for (outcome, before, after, diff, mutation) in vectors {
        assert_eq!(parse_json(outcome).unwrap().str("status"), "applied");
        let _ = parse_json(diff).unwrap();
        let base = parse_json(before).unwrap();
        let mutation = parse_json(mutation).unwrap();
        let expected = parse_json(after).unwrap();
        let pages = base.array("pages").iter().map(page).collect::<Result<Vec<_>, _>>().unwrap();
        let base = build_document(&pages).unwrap();
        let forward = spec(&mutation.str("mutation"), mutation.get("payload").unwrap().clone());
        let mutated = oracle_apply_mutation(&base, &forward).unwrap();
        let expected = expected.array("pages").iter().map(page).collect::<Result<Vec<_>, _>>().unwrap();
        let actual = independent_pages(&mutated).unwrap();
        assert_eq!(actual.len(), expected.len());
        for (left, right) in actual.iter().zip(&expected) {
            assert!((left.width - right.width).abs() < 0.001 && (left.height - right.height).abs() < 0.001);
            assert_eq!(left.text, right.text);
        }
        let inverse = oracle_inverse_spec(&base, &forward).unwrap();
        let restored = oracle_apply_mutation(&mutated, &inverse).unwrap();
        crate::law::inverse_restores_within(&forward.str("kind"), &project_pdf_1_4(&restored).unwrap(), &project_pdf_1_4(&base).unwrap(), &[], 0.001).unwrap();
    }
}

#[test]
fn every_real_document_feature_row_is_observable_and_invertible() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🖼️assets/🎓️bachelor-thesis.pdf");
    let base = std::fs::read(path).unwrap();
    assert_eq!(independent_pages(&base).unwrap().len(), 65);
    let feature = include_str!("../../../🧪️tests/📑️mutate-pdf-1-4/🥒️.feature");
    let rows = crate::law::feature_rows(feature);
    assert_eq!(rows.len(), KINDS.len());
    for (kind, params) in rows {
        assert!(KINDS.contains(&kind.as_str()));
        let forward = spec(&kind, params);
        let mutated = oracle_apply_mutation(&base, &forward).unwrap();
        crate::law::mutation_is_observable_within(&kind, &project_pdf_1_4(&mutated).unwrap(), &project_pdf_1_4(&base).unwrap(), &[], &[], 0.001).unwrap();
        let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&base, &forward).unwrap()).unwrap();
        crate::law::inverse_restores_within(&kind, &project_pdf_1_4(&restored).unwrap(), &project_pdf_1_4(&base).unwrap(), &[], 0.001).unwrap();
    }
    let rewritten = oracle_round_trip(&base).unwrap();
    assert_ne!(rewritten, base);
    assert_eq!(project_pdf_1_4(&rewritten).unwrap(), project_pdf_1_4(&base).unwrap());
}
