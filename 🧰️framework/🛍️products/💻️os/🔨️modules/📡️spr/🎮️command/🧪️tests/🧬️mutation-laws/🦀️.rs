use crate::os_spr::{DiffRegions, ForeignStep, ForeignTarget, MutationApplyError, MutationApplyResult, MutationDiff, TouchedPaths};
use serde::{Deserialize, Serialize};

pub type Counter = i64;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterDiff { pub deltas: Vec<i64> }

impl MutationDiff<Counter> for CounterDiff {
    fn apply(&self, base: &Counter) -> MutationApplyResult<Counter> {
        self.deltas.iter().try_fold(*base, |value, delta| value.checked_add(*delta).ok_or_else(|| MutationApplyError::new("mutation.apply.invariant", "counter addition overflowed")))
    }
    fn absorb(&mut self, other: Self) { self.deltas.extend(other.deltas); }
}

impl DiffRegions for CounterDiff {
    fn touches(&self) -> TouchedPaths {
        if self.deltas.iter().any(|delta| *delta != 0) { TouchedPaths::new(["value"]) } else { TouchedPaths::default() }
    }
}

#[path = "🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub(super) fn foreign_step_fixture(n: u8) -> ForeignStep {
    ForeignStep {
        target: ForeignTarget { artifact_id: format!("artifact-{n}"), artifact_kind: "s.demo.widget".into(), dialect: None },
        mutation_id: crate::os_spr::ids::SchemaId("widget.doc#set-color".into()),
        payload: vec![n],
        label: format!("Recolor widget {n}"),
    }
}

fn assert_counter_leaf_descriptor<T: crate::os_spr::MutationLeaf>(descriptor: &str) {
    assert_eq!(serde_json::to_value(T::DESCRIPTOR).unwrap(), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
    assert!(T::DESCRIPTOR.validate().is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{fold_plan_diff, fold_plan_inverse, Mutation, OpBinary, OpText};

    fn cases() -> serde_json::Value { serde_json::from_str(include_str!("🔣️cases.json")).unwrap() }

    #[test]
    fn counter_fixture_codecs_and_descriptors() {
        assert_eq!(<CounterMutation as Mutation<Counter>>::DESCRIPTORS.len(), 5);
        for row in cases()["cases"].as_array().unwrap() {
            let mut wire = row["payload"].clone();
            wire["operation"] = row["operation"].clone();
            let op = serde_json::from_value::<CounterMutation>(wire.clone()).unwrap();
            assert!(op.descriptor().validate().is_ok());
            assert_eq!(serde_json::from_value::<CounterMutation>(serde_json::to_value(&op).unwrap()).unwrap(), op);
            assert_eq!(CounterMutation::parse_op(&op.print_op()).unwrap(), op);
            assert_eq!(CounterMutation::decode_op(&op.encode_op().unwrap()).unwrap(), op);
            assert_eq!(op.timestamp(), None);
            let base = row["before"].as_i64().unwrap();
            let mut current = op.diff(&base).diff().apply(&base).unwrap();
            assert_eq!(current, row["after"].as_i64().unwrap());
            for inverse in op.inverse(&base).iter().rev() { current = inverse.diff(&current).diff().apply(&current).unwrap(); }
            assert_eq!(current, base);
            let mut unknown = wire.clone();
            unknown["unknown"] = serde_json::json!(true);
            assert!(serde_json::from_value::<CounterMutation>(unknown).is_err());
            for key in row["payload"].as_object().unwrap().keys() {
                let mut missing = wire.clone();
                missing.as_object_mut().unwrap().remove(key);
                assert!(serde_json::from_value::<CounterMutation>(missing).is_err());
                for value in [serde_json::json!(null), serde_json::json!(true), serde_json::json!("1"), serde_json::json!(1e21)] {
                    let mut invalid = wire.clone(); invalid[key] = value;
                    assert!(serde_json::from_value::<CounterMutation>(invalid).is_err());
                }
            }
        }
    }

    #[test]
    fn counter_fixture_checked_add_and_structural_diff() {
        for row in cases()["arithmetic"].as_array().unwrap() {
            let base = row["before"].as_str().unwrap().parse::<i64>().unwrap();
            let deltas = row["deltas"].as_array().unwrap().iter().map(|delta| delta.as_str().unwrap().parse::<i64>().unwrap()).collect::<Vec<_>>();
            let diff = CounterDiff { deltas };
            let result = diff.apply(&base);
            if row["error"] == true { assert_eq!(result.unwrap_err().code, "mutation.apply.invariant"); continue; }
            let expected = row["after"].as_str().unwrap().parse::<i64>().unwrap();
            assert_eq!(result, Ok(expected));
            assert_eq!(serde_json::from_value::<CounterDiff>(serde_json::to_value(&diff).unwrap()).unwrap().apply(&base), Ok(expected));
            let mut joined = CounterDiff::default();
            for delta in &diff.deltas { joined.absorb(CounterDiff { deltas: vec![*delta] }); }
            assert_eq!(joined, diff);
        }
    }

    #[test]
    fn counter_fixture_mixed_inverse_stored_order() {
        for row in cases()["arithmetic"].as_array().unwrap().iter().filter(|row| row.get("storedInverse").is_some()) {
            let base = row["before"].as_str().unwrap().parse::<i64>().unwrap();
            let kind = AddCounterSequence { deltas: row["deltas"].as_array().unwrap().iter().map(|delta| delta.as_str().unwrap().parse::<i64>().unwrap()).collect() };
            let mut current = fold_plan_diff(&kind, &base).diff().apply(&base).unwrap();
            assert_eq!(current, row["after"].as_str().unwrap().parse::<i64>().unwrap());
            let stored = fold_plan_inverse(&kind, &base);
            let deltas = stored.iter().map(|op| match op { CounterMutation::AddCounter(add) => add.delta.to_string(), _ => panic!("inverse must be direct addition") }).collect::<Vec<_>>();
            assert_eq!(serde_json::to_value(deltas).unwrap(), row["storedInverse"]);
            for inverse in stored.iter().rev() { current = inverse.diff(&current).diff().apply(&current).unwrap(); }
            assert_eq!(current, base);
        }
    }

    #[test]
    fn counter_fixture_exact_i64_codecs() {
        for row in cases()["decimalI64"].as_array().unwrap() {
            let text = format!("{{\"operation\":\"addCounter\",\"delta\":{}}}", row["value"].as_str().unwrap());
            let decoded = serde_json::from_str::<CounterMutation>(&text);
            assert_eq!(decoded.is_ok(), row["valid"].as_bool().unwrap());
            if let Ok(op) = decoded {
                assert_eq!(CounterMutation::parse_op(&op.print_op()).unwrap(), op);
                assert_eq!(CounterMutation::decode_op(&op.encode_op().unwrap()).unwrap(), op);
            }
        }
        for bytes in [vec![], vec![255], vec![1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1]] { assert!(CounterMutation::decode_op(&bytes).is_err()); }
    }

    #[test]
    fn ordered_diff_preserves_step_admission_and_associativity() {
        let mut left = CounterDiff { deltas: vec![i64::MAX] };
        left.absorb(CounterDiff { deltas: vec![1] });
        assert_eq!(left.apply(&i64::MIN), Ok(0));
        let mut joined = CounterDiff { deltas: vec![2] };
        joined.absorb(CounterDiff { deltas: vec![3] });
        joined.absorb(CounterDiff { deltas: vec![4] });
        let mut suffix = CounterDiff { deltas: vec![3] };
        suffix.absorb(CounterDiff { deltas: vec![4] });
        let mut right_grouped = CounterDiff { deltas: vec![2] };
        right_grouped.absorb(suffix);
        assert_eq!(joined, right_grouped);
        assert_eq!(joined.apply(&0), Ok(9));
        assert!(CounterDiff { deltas: vec![i64::MAX, 1, -1] }.apply(&0).is_err());
    }
}
