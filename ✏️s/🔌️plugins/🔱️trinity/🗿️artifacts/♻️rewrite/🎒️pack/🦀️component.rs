//! 📦️ `trinity.rewrite.rule` artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::rewrite::RewriteRuleState;
use store::PackError;

/// 📦️ Encodes a `RewriteRuleState` to its binary pack form.
pub fn encode(document: &RewriteRuleState) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `RewriteRuleState` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<RewriteRuleState, PackError> {
    <RewriteRuleState as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::PropertyValue;
    use crate::artifacts::rewrite::LayoutPoint;
    use std::collections::BTreeMap;
    use store::test_support::assert_dsl_pack_equivalence;

    fn sample_rule_state() -> RewriteRuleState {
        let mut parameter_bindings = BTreeMap::new();
        parameter_bindings.insert("label".to_string(), PropertyValue::String("nakagin-core".into()));
        parameter_bindings.insert("count".to_string(), PropertyValue::Number(3.0));
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint::from((10.5, -20.25)));
        RewriteRuleState {
            before_fixture_json: "{\"schema\":\"trinity.graph\",\"name\":\"x \\\"quoted\\\"\\nline\"}".to_string(),
            lhs_json: r#"{"pattern":{"leftVar":"a","leftKind":"Piece"}}"#.to_string(),
            rhs_json: r#"{"set":[{"var":"a","prop":"label","value":"$label"}]}"#.to_string(),
            parameter_bindings,
            rule_layout,
        }
    }

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = sample_rule_state();
        assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
