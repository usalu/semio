use crate::DagHostSnapshot;

/// 📥️ The actual typed decoder owns unknown-kind refusal without a concrete manifest lookup.
#[test]
fn host_kind_admission_matches_language_neutral_vectors() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📥️host-kind-admission/🔣️.json")).expect("host admission corpus");
    let cases = corpus["cases"].as_array().expect("host admission cases");
    assert_eq!(cases.len(), 15);
    for case in cases {
        let text = serde_json::to_string(&case["snapshot"]).expect("independent JSON fixture encoding");
        let decoded = semio_framework_pack_json::from_json_str::<DagHostSnapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject);
        assert_eq!(decoded.is_ok(), case["accepted"].as_bool().expect("admission expectation"), "{}: {decoded:?}", case["id"]);
        if let Ok(snapshot) = decoded { assert_eq!(crate::dag_node_kind_tag(&snapshot.nodes[0].kind), case["id"].as_str().expect("exact typed discriminator")); }
    }
}
