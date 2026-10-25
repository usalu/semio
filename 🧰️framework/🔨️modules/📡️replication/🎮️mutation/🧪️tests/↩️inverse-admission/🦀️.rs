use super::{ApplyCapability, DiffAlgebra, ColdDiffAlgebra, MutationApplyResult, MutationDiff, MutationMessage, MutationOutcome};

thread_local! {
    static RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[derive(Clone, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct Delta { empty: bool }

impl MutationDiff<()> for Delta {


    fn retire_cold(self) { RETIRED.with(|count| count.set(count.get() + 1)); }
}

impl DiffAlgebra<()> for Delta {

    fn is_empty(&self) -> bool { self.empty }
}

impl ColdDiffAlgebra<()> for Delta {
    fn inverse(&self, _: &()) -> Self { Self { empty: self.empty } }
}

/// 🛂️ Neutral admission vectors gate inverse candidates and retire every evaluated diff once.
#[test]
fn inverse_admission_follows_portable_outcomes_and_retires_each_diff() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/↩️inverse-admission/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        RETIRED.with(|count| count.set(0));
        let outcome = MutationOutcome::new(Delta { empty: row["empty"].as_bool().unwrap() });
        let message = match row["level"].as_str() {
            None => None,
            Some("info") => Some(MutationMessage::info("inverse.probe", "info")),
            Some("warning") => Some(MutationMessage::warning("inverse.probe", "warning")),
            Some("error") => Some(MutationMessage::error("inverse.probe", "error")),
            Some("fatal") => Some(MutationMessage::fatal("inverse.probe", "fatal")),
            _ => panic!("unknown fixture level"),
        };
        let mut discovery = false;
        let admitted = outcome.absorb_messages(message).admit_inverse::<()>(|| { discovery = true; Ok(row["foreignSteps"].as_bool().unwrap()) }).unwrap();
        assert_eq!(admitted, row["inverseAdmitted"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(discovery, row["empty"] == true && !matches!(row["level"].as_str(), Some("error" | "fatal")), "{}", row["id"]);
        assert_eq!(RETIRED.with(|count| count.get()), 1, "{}", row["id"]);
    }
    RETIRED.with(|count| count.set(0));
    let refused = MutationOutcome::new(Delta { empty: true }).admit_inverse::<()>(|| Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "foreign source refusal")));
    assert!(refused.is_err());
    assert_eq!(RETIRED.with(|count| count.get()), 1);
}
