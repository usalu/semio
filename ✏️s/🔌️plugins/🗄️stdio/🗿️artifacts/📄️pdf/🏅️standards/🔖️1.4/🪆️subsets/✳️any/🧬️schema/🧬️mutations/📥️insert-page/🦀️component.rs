//! 📥️ Direct insert-page payload, sparse diff, concrete inverse, and laws.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{
    diff::{PdfDiff, PdfPageAdded, PdfPagesDiff},
    snapshot::{PageDoc, PdfSnapshot},
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertPage {
    pub index: usize,
    #[serde(deserialize_with = "deserialize_page")]
    pub page: PageDoc,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PagePayload {
    width: f64,
    height: f64,
    text: String,
}

fn deserialize_page<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<PageDoc, D::Error> {
    let PagePayload { width, height, text } = PagePayload::deserialize(deserializer)?;
    Ok(PageDoc { width, height, text })
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl InsertPage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty() && self.index <= base.pages.len() && self.page.width.is_finite() && self.page.height.is_finite()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertPage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "InsertedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.insert-page.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { added: vec![PdfPageAdded { index: self.index, page: self.page.clone() }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfMutation::RemovePage(super::RemovePage { index: self.index })]
    }

    fn label(&self) -> String {
        "insert page".into()
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

    #[test]
    fn inserted_page_wire_requires_exact_declared_fields() {
        let missing = serde_json::json!({"mutation":"insert-page","payload":{"index":0,"page":{"width":1,"height":2}}});
        let extra = serde_json::json!({"mutation":"insert-page","payload":{"index":0,"page":{"width":1,"height":2,"text":"","extra":true}}});
        assert!(serde_json::from_value::<PdfMutation>(missing).is_err());
        assert!(serde_json::from_value::<PdfMutation>(extra).is_err());
    }
}
//#endregion 🧪️Tests
