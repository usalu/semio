//! 🗑️ Direct remove-page payload, sparse diff, concrete inverse, and laws.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{
    diff::{PdfDiff, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemovePage {
    pub index: usize,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl RemovePage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        base.pages.len() > 1 && self.index < base.pages.len()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemovePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "page", kind: "remove-page", record: "RemovedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.remove-page.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { removed: vec![self.index], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfMutation::InsertPage(super::InsertPage { index: self.index, page: base.pages[self.index].clone() })]
    }

    fn label(&self) -> String {
        "remove page".into()
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Behavior

//#region 🔖️Codecs
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Codecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;
    use protocol::{OpBinary, OpText};

    #[test]
    fn language_neutral_forward_and_concrete_inverse() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/round-trips-the-concrete-inverse/🔣️component.json")).unwrap();
        fn assert_json_shape(actual: &serde_json::Value, expected: &serde_json::Value) {
            match (actual, expected) {
                (serde_json::Value::Object(actual), serde_json::Value::Object(expected)) => {
                    assert_eq!(actual.len(), expected.len());
                    for (key, expected) in expected {
                        assert_json_shape(actual.get(key).unwrap_or_else(|| panic!("missing JSON field {key:?}")), expected);
                    }
                }
                (serde_json::Value::Array(actual), serde_json::Value::Array(expected)) => {
                    assert_eq!(actual.len(), expected.len());
                    for (actual, expected) in actual.iter().zip(expected) {
                        assert_json_shape(actual, expected);
                    }
                }
                (serde_json::Value::Number(_), serde_json::Value::Number(_)) => {}
                _ => assert_eq!(actual, expected),
            }
        }
        let base: PdfSnapshot = serde_json::from_value(fixture["base"].clone()).unwrap();
        let mutation: PdfMutation = serde_json::from_value(fixture["mutation"].clone()).unwrap();
        assert_json_shape(&serde_json::to_value(&mutation).unwrap(), &fixture["mutation"]);
        let mut state = base.clone();
        let outcome = mutation.diff(&state).apply_to(&mut state);
        assert!(outcome.messages().is_empty());
        let expected: PdfSnapshot = serde_json::from_value(fixture["expected"].clone()).unwrap();
        assert_eq!(state, expected);
        assert_json_shape(&serde_json::to_value(&state).unwrap(), &fixture["expected"]);
        let inverse = mutation.inverse(&base);
        let expected_inverse: Vec<PdfMutation> = serde_json::from_value(fixture["inverse"].clone()).unwrap();
        assert_eq!(inverse, expected_inverse);
        assert_json_shape(&serde_json::to_value(&inverse).unwrap(), &fixture["inverse"]);
        for step in std::iter::once(mutation.clone()).chain(inverse.iter().cloned()) {
            assert_eq!(PdfMutation::parse_op(&step.print_op()).unwrap(), step);
            assert_eq!(PdfMutation::decode_op(&step.encode_op().unwrap()).unwrap(), step);
            assert_eq!(serde_json::from_value::<PdfMutation>(serde_json::to_value(&step).unwrap()).unwrap(), step);
        }
        for step in inverse {
            assert!(step.diff(&state).apply_to(&mut state).messages().is_empty());
        }
        assert_eq!(state, base);
    }

    #[test]
    fn missing_page_refuses_without_inverse_or_state_change() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/round-trips-the-concrete-inverse/🔣️component.json")).unwrap();
        let mutation: PdfMutation = serde_json::from_value(fixture["mutation"].clone()).unwrap();
        let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
        let mut state = base.clone();
        assert!(!mutation.diff(&state).apply_to(&mut state).messages().is_empty());
        assert_eq!(state, base);
        assert!(mutation.inverse(&base).is_empty());
    }
}
//#endregion 🧪️Tests
