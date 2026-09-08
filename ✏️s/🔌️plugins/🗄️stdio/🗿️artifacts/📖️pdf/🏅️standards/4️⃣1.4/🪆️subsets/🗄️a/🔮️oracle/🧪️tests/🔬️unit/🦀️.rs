
use super::*;

#[test]
fn every_real_document_feature_row_is_observable_and_invertible() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🖼️assets/🎓️bachelor-thesis.pdf");
    let base = std::fs::read(path).unwrap();
    let feature = include_str!("../../../🧪️tests/🗄️mutate-pdf-1-4-a/🥒️.feature");
    let rows = crate::law::feature_rows(feature);
    assert_eq!(rows.len(), KINDS.len());
    for (kind, params) in rows {
        assert!(KINDS.contains(&kind.as_str()));
        let forward = Json::Object(vec![("kind".into(), Json::String(kind.clone())), ("params".into(), params)]);
        let mutated = oracle_apply_mutation(&base, &forward).unwrap();
        crate::law::mutation_is_observable_within(&kind, &project_conformance(&mutated).unwrap(), &project_conformance(&base).unwrap(), &[], &[], 0.001).unwrap();
        let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&base, &forward).unwrap()).unwrap();
        crate::law::inverse_restores_within(&kind, &project_conformance(&restored).unwrap(), &project_conformance(&base).unwrap(), &[], 0.001).unwrap();
    }
    let rewritten = oracle_round_trip(&base).unwrap();
    assert_ne!(rewritten, base);
    assert_eq!(project_conformance(&rewritten).unwrap(), project_conformance(&base).unwrap());
}

#[test]
fn language_neutral_direct_vectors_match_independent_lopdf() {
    use crate::standards::v1_4::subsets::base::{OraclePage, build_document, independent_pages};
    use semio_repo_test_host::parse_json;
    let vectors = [include_str!("../../../🧬️schema/🧬️mutations/📝️set-page-text/🧪️tests/🔄️round-trips-the-concrete-inverse/🔣️.json"), include_str!("../../../🧬️schema/🧬️mutations/🧹️clear-page-text/🧪️tests/🔄️round-trips-the-concrete-inverse/🔣️.json")];
    for text in vectors {
        let fixture = parse_json(text).unwrap();
        let to_page = |p: &Json| OraclePage {
            width: match p.get("width").unwrap() {
                Json::Number(v) => *v,
                _ => panic!("width"),
            },
            height: match p.get("height").unwrap() {
                Json::Number(v) => *v,
                _ => panic!("height"),
            },
            text: p.str("text"),
        };
        let pages: Vec<_> = fixture.get("base").unwrap().array("pages").iter().map(to_page).collect();
        let base = build_document(&pages).unwrap();
        let wire = fixture.get("mutation").unwrap();
        let forward = Json::Object(vec![("kind".into(), Json::String(wire.str("mutation"))), ("params".into(), wire.get("payload").unwrap().clone())]);
        let mutated = oracle_apply_mutation(&base, &forward).unwrap();
        let expected: Vec<_> = fixture.get("expected").unwrap().array("pages").iter().map(to_page).collect();
        let actual = independent_pages(&mutated).unwrap();
        assert_eq!(actual.len(), expected.len());
        for (a, b) in actual.iter().zip(&expected) {
            assert!((a.width - b.width).abs() < 0.001 && (a.height - b.height).abs() < 0.001);
            assert_eq!(a.text, b.text);
        }
        let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&base, &forward).unwrap()).unwrap();
        assert_eq!(independent_pages(&restored).unwrap(), independent_pages(&base).unwrap());
    }
}
