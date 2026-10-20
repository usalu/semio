use super::{ArtifactBuilder, SnapshotBuilder};
use protocol::{DiffAlgebra, ColdDiffAlgebra, MutationDiff, ColdMutationDiff};
use crate::store::{os_store::serialization::{}, PackDecodeOptions, PackEncodeOptions, PackError};
use pack::snapshot::ArtifactDsl;
use pack::snapshot::ArtifactPack;

thread_local! { static RETIRED: std::cell::Cell<[usize; 3]> = const { std::cell::Cell::new([0; 3]) }; }
fn retire(axis: usize) { RETIRED.with(|counts| { let mut next = counts.get(); next[axis] += 1; counts.set(next); }); }

#[derive(Clone, Default, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct State { value: i64 }
impl ArtifactDsl for State {
    const EXTENSION: &'static str = "builder-ownership-law";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { serde_json::from_str(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1))) }
    fn print_dsl(&self) -> String { serde_json::to_string(self).unwrap() }
}
impl ArtifactPack for State {
    fn encode_pack_with(&self, _: &PackEncodeOptions) -> Result<Vec<u8>, PackError> { Ok(self.value.to_le_bytes().to_vec()) }
    fn decode_pack_with(bytes: &[u8], _: &PackDecodeOptions) -> Result<Self, PackError> { let bytes = bytes.try_into().map_err(|_| PackError::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "builder law needs eight bytes")))?; Ok(Self { value: i64::from_le_bytes(bytes) }) }
}
#[derive(Clone, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct Delta { value: Option<i64>, refuse: bool }
impl DiffAlgebra<State> for Delta {

    fn is_empty(&self) -> bool { self.value.is_none() }
}

impl ColdDiffAlgebra<State> for Delta {
    fn inverse(&self, base: &State) -> Self { Self { value: self.value.map(|_| base.value), refuse: false } }
}
impl MutationDiff<State> for Delta {


    fn retire_cold(self) { retire(1); }
    fn retire_projection(_: State) { retire(2); }
}

impl ColdMutationDiff<State> for Delta {
    fn absorb(&mut self, other: Self) { if other.value.is_some() { self.value = other.value; } self.refuse |= other.refuse; }

    fn apply(&self, state: &mut State, _: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> { if self.refuse { return Err(protocol::MutationApplyError::new("mutation.apply.refused", "builder fixture refusal")); } if let Some(value) = self.value { state.value = value; } Ok(()) }
}
#[derive(semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct Operation { value: i64, semantic_refusal: bool, apply_refusal: bool }
impl protocol::ColdMutation<State> for Operation {
    type ReferenceDiff = Delta;
    fn diff(&self, _: &State) -> protocol::MutationOutcome<Delta> { let outcome = protocol::MutationOutcome::new(Delta { value: Some(self.value), refuse: self.apply_refusal }); if self.semantic_refusal { outcome.absorb_messages([protocol::MutationMessage::error("mutation.refused", "semantic fixture refusal")]) } else { outcome } }
    fn inverse(&self, base: &State) -> Result<Vec<Self>, semio_framework_value::ValueError> { Ok(vec![Self { value: base.value, semantic_refusal: false, apply_refusal: false }]) }
}

impl Drop for State { fn drop(&mut self) { retire(2); } }
impl Drop for Delta { fn drop(&mut self) { retire(1); } }
impl Drop for Operation { fn drop(&mut self) { retire(0); } }

/// ❄️ The independent cold builder disposes owned wire values once without a native Mutation facet.
#[test]
fn snapshot_builder_retires_original_owners_on_success_and_refusal() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(corpus["disposal"], "owned-wire");
    assert_eq!(corpus["nativeOperation"], false);
    for row in corpus["cases"].as_array().unwrap() {
        RETIRED.with(|counts| counts.set([0; 3]));
        let builder = SnapshotBuilder::<State, Operation>::from_snapshot(State { value: corpus["base"].as_i64().unwrap() });
        let value = corpus["value"].as_i64().unwrap();
        let apply_refusal = row["applyRefusal"].as_bool().unwrap();
        let (result, returned_diff) = if row["entry"] == "mutate" {
            let (builder, outcome) = builder.mutate(Operation { value, semantic_refusal: row["semanticRefusal"].as_bool().unwrap(), apply_refusal });
            assert_eq!(outcome.worst_level().is_some(), row["semanticRefusal"] == true || apply_refusal);
            (Ok(builder), Some(outcome.into_parts().0))
        } else { (builder.absorb(Delta { value: Some(value), refuse: apply_refusal }), None) };
        assert_eq!(result.is_err(), row["error"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(RETIRED.with(|counts| counts.get()), [row["mutationsRetired"].as_u64().unwrap() as usize, row["diffsRetired"].as_u64().unwrap() as usize, row["projectionsRetired"].as_u64().unwrap() as usize], "{}", row["id"]);
        if let Ok(builder) = result { let snapshot = builder.build().unwrap(); assert_eq!(snapshot.value, row["result"].as_i64().unwrap()); Delta::retire_projection(snapshot); }
        if let Some(diff) = returned_diff { Delta::retire_cold(diff); }
    }
}
