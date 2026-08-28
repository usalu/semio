//! 🧪️ Counter domain and direct operations used by the law helper's own tests.

use crate::os_spr::{DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔺️StructuralDiff
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterDiff {
    pub deltas: Vec<i64>,
}

impl CounterDiff {
    pub fn delta(delta: i64) -> Self {
        if delta == 0 { Self::default() } else { Self { deltas: vec![delta] } }
    }

    pub fn from_wide(mut delta: i128) -> Self {
        let mut deltas = Vec::new();
        while delta != 0 {
            let step = i64::try_from(delta.clamp(i128::from(i64::MIN), i128::from(i64::MAX))).expect("clamped i64 delta");
            deltas.push(step);
            delta -= i128::from(step);
        }
        Self { deltas }
    }
}

impl MutationDiff<i64> for CounterDiff {
    fn apply(&self, base: &i64) -> MutationApplyResult<i64> {
        self.deltas.iter().try_fold(*base, |value, delta| value.checked_add(*delta).ok_or_else(|| MutationApplyError::new("mutation.apply.invariant", "counter addition overflowed")))
    }

    fn absorb(&mut self, other: Self) { self.deltas.extend(other.deltas); }
}

impl DiffAlgebra<i64> for CounterDiff {
    fn inverse(&self, _base: &i64) -> Self {
        Self { deltas: self.deltas.iter().rev().flat_map(|delta| Self::from_wide(-i128::from(*delta)).deltas).collect() }
    }

    fn between(base: &i64, other: &i64) -> Self { Self::from_wide(i128::from(*other) - i128::from(*base)) }

    fn is_empty(&self) -> bool { self.deltas.iter().all(|delta| *delta == 0) }
}
//#endregion 🔺️StructuralDiff

//#region 🧬️Mutations
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;
//#endregion 🧬️Mutations

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{Mutation, MutationLeaf, OpText};

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).expect("law fixture") }

    pub(crate) fn assert_leaf<T>(index: usize, wrap: fn(T) -> CounterMutation, descriptor: &str)
    where T: MutationLeaf + OpText + serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug {
        let fixtures = fixture();
        let row = &fixtures["mutations"][index];
        let value = serde_json::from_value::<T>(row["payload"].clone()).expect("direct payload");
        assert_eq!(serde_json::to_value(&value).unwrap(), row["payload"]);
        assert_eq!(value.print_op(), row["text"].as_str().unwrap());
        assert_eq!(T::parse_op(&value.print_op()).unwrap(), value);
        assert!(T::parse_op("unknown").is_err());
        let mut unknown = row["payload"].clone();
        unknown["unknown"] = serde_json::json!(true);
        assert!(serde_json::from_value::<T>(unknown).is_err());
        let mutation = wrap(value);
        assert_eq!(mutation.descriptor(), &T::DESCRIPTOR);
        assert_eq!(serde_json::to_value(T::DESCRIPTOR).unwrap(), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
        let envelope = serde_json::json!({"operation": row["operation"], "payload": row["payload"]});
        assert_eq!(serde_json::to_value(&mutation).unwrap(), envelope);
        assert_eq!(serde_json::from_value::<CounterMutation>(envelope).unwrap(), mutation);
        assert_eq!(CounterMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        let base = row["base"].as_i64().unwrap();
        let outcome = mutation.diff(&base);
        let level = match outcome.worst_level() {
            None => "applied",
            Some(crate::os_dsl::Severity::Error) => "error",
            Some(crate::os_dsl::Severity::Fatal) => "fatal",
            other => panic!("unexpected fixture outcome {other:?}"),
        };
        assert_eq!(level, row["outcome"].as_str().unwrap());
        assert_eq!(outcome.diff().apply(&base).unwrap(), row["next"].as_i64().unwrap());
        assert_eq!(mutation.inverse(&base).len(), usize::try_from(row["inverseCount"].as_u64().unwrap()).unwrap());
    }

    #[test]
    fn ordered_counter_algebra_matches_exact_neutral_boundaries() {
        let fixture = fixture();
        for row in fixture["addition"].as_array().unwrap() {
            let base = row["base"].as_str().unwrap().parse::<i64>().unwrap();
            let diff = CounterDiff { deltas: row["deltas"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().parse().unwrap()).collect() };
            match row["expected"].as_str() {
                Some(expected) => {
                    let after = diff.apply(&base).unwrap();
                    assert_eq!(after, expected.parse::<i64>().unwrap());
                    assert_eq!(diff.inverse(&base).apply(&after).unwrap(), base);
                }
                None => assert!(diff.apply(&base).is_err()),
            }
        }
        for row in fixture["between"].as_array().unwrap() {
            let base = row["base"].as_str().unwrap().parse::<i64>().unwrap();
            let other = row["other"].as_str().unwrap().parse::<i64>().unwrap();
            let diff = CounterDiff::between(&base, &other);
            assert_eq!(diff.apply(&base).unwrap(), other);
            assert_eq!(diff.inverse(&base).apply(&other).unwrap(), base);
            assert_eq!(diff.is_empty(), base == other);
        }
        let first = CounterDiff::delta(2);
        let second = CounterDiff::delta(3);
        let third = CounterDiff::delta(4);
        let mut left = first.clone();
        left.absorb(second.clone());
        left.absorb(third.clone());
        let mut tail = second;
        tail.absorb(third);
        let mut right = first;
        right.absorb(tail);
        assert_eq!(left, right);
        assert_eq!(left.apply(&10), Ok(19));
    }

    #[test]
    fn minimum_add_inverse_obeys_store_reverse_order() {
        let mutation = CounterMutation::AddCounter(AddCounter { delta: i64::MIN });
        let before = i64::MAX;
        let after = mutation.diff(&before).diff().apply(&before).unwrap();
        let inverse = mutation.inverse(&before);
        assert_eq!(inverse, vec![AddCounter { delta: 1 }.into(), AddCounter { delta: i64::MAX }.into()]);
        let restored = inverse.into_iter().rev().fold(after, |state, operation| operation.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, before);
    }
}
//#endregion 🧪️Contracts
