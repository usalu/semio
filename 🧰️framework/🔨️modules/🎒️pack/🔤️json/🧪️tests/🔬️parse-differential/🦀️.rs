//! 🧪️ Differential law: the owned JSON parser accepts and rejects exactly what serde_json does and yields the same value tree.
use semio_framework_pack_json::{parse, JsonMemberPolicy};

#[test]
fn owned_parser_agrees_with_serde_json_on_the_portable_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("fixture");
    for text in corpus["valid"].as_array().expect("valid") {
        let text = text.as_str().expect("text");
        let oracle: serde_json::Value = serde_json::from_str(text).expect("oracle accepts");
        let value = parse(text, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("owned parser refused {text:?}: {error:?}"));
        assert_eq!(serde_json::to_value(&value).expect("project"), oracle, "{text:?}");
    }
    for text in corpus["invalid"].as_array().expect("invalid") {
        let text = text.as_str().expect("text");
        assert!(serde_json::from_str::<serde_json::Value>(text).is_err(), "oracle must refuse {text:?}");
        assert!(parse(text, JsonMemberPolicy::Reject).is_err(), "owned parser accepted {text:?}");
    }
}
