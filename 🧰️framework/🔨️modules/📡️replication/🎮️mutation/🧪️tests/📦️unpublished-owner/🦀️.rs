use super::{apply_diff_cold, apply_diff_cold_into, ApplyCapability, ColdDiffAlgebra, ColdMutationDiff, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff, PreparedField};

thread_local! {
    static CLONES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static PROJECTION_RETIREMENTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FIELD_RETIREMENTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[derive(Debug, PartialEq)]
struct Projection { first: i64, second: i64 }

impl Clone for Projection {
    fn clone(&self) -> Self {
        CLONES.with(|value| value.set(value.get() + 1));
        Self { first: self.first, second: self.second }
    }
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
struct Diff { first: Option<i64>, second: Option<i64>, refuse: bool }

fn retire_field(_: i64) { FIELD_RETIREMENTS.with(|value| value.set(value.get() + 1)); }

impl DiffAlgebra<Projection> for Diff {
    fn is_empty(&self) -> bool { self.first.is_none() && self.second.is_none() }
}

impl ColdDiffAlgebra<Projection> for Diff {
    fn inverse(&self, base: &Projection) -> Self { Self { first: self.first.map(|_| base.first), second: self.second.map(|_| base.second), refuse: false } }
}

impl ColdMutationDiff<Projection> for Diff {
    fn absorb(&mut self, other: Self) { self.first = other.first.or(self.first); self.second = other.second.or(self.second); self.refuse |= other.refuse; }

    fn apply(&self, unpublished: &mut Projection, capability: ApplyCapability) -> MutationApplyResult<()> {
        let first = self.first.map(|value| PreparedField::prepare(&unpublished.first, retire_field, capability, |candidate| { *candidate = value; Ok(()) })).transpose()?;
        let second = self.second.map(|value| PreparedField::prepare(&unpublished.second, retire_field, capability, |candidate| {
            if self.refuse { return Err(MutationApplyError { code: "mutation.apply.refused".into(), message: "later field refused".into(), target: vec!["second".into()] }); }
            *candidate = value;
            Ok(())
        })).transpose()?;
        if self.refuse && self.second.is_none() { return Err(MutationApplyError { code: "mutation.apply.refused".into(), message: "later field refused".into(), target: vec!["second".into()] }); }
        if let Some(first) = first { first.install(&mut unpublished.first); }
        if let Some(second) = second { second.install(&mut unpublished.second); }
        Ok(())
    }
}

impl MutationDiff<Projection> for Diff {

    fn retire_projection(_: Projection) { PROJECTION_RETIREMENTS.with(|value| value.set(value.get() + 1)); }
}

/// 🪶️ The explicit cold scratch owner changes fields without another projection clone.
#[test]
fn unpublished_owner_applies_without_projection_clones() {
    CLONES.with(|value| value.set(0));
    FIELD_RETIREMENTS.with(|value| value.set(0));
    let mut projection = Projection { first: 1, second: 2 };
    apply_diff_cold_into(&Diff { first: Some(7), second: Some(8), ..Default::default() }, &mut projection).expect("owner application");
    assert_eq!(projection, Projection { first: 7, second: 8 });
    assert_eq!(CLONES.with(|value| value.get()), 0);
    assert_eq!(FIELD_RETIREMENTS.with(|value| value.get()), 2);
}

/// 🛡️ A later refusal leaves the owner unchanged and retires the earlier prepared field.
#[test]
fn unpublished_owner_refusal_is_atomic_and_retires_prepared_fields() {
    FIELD_RETIREMENTS.with(|value| value.set(0));
    let mut projection = Projection { first: 1, second: 2 };
    assert!(apply_diff_cold_into(&Diff { first: Some(7), refuse: true, ..Default::default() }, &mut projection).is_err());
    assert_eq!(projection, Projection { first: 1, second: 2 });
    assert_eq!(FIELD_RETIREMENTS.with(|value| value.get()), 1);
}

/// ❄️ Cold evaluation owns one scratch clone and retires it on refusal while retaining its original.
#[test]
fn cold_owner_refusal_retires_scratch_and_preserves_original() {
    CLONES.with(|value| value.set(0));
    PROJECTION_RETIREMENTS.with(|value| value.set(0));
    let projection = Projection { first: 1, second: 2 };
    assert!(apply_diff_cold(&Diff { first: Some(7), refuse: true, ..Default::default() }, &projection).is_err());
    assert_eq!(projection, Projection { first: 1, second: 2 });
    assert_eq!(CLONES.with(|value| value.get()), 1);
    assert_eq!(PROJECTION_RETIREMENTS.with(|value| value.get()), 1);
}

/// 🧪️ Portable staged-field outcomes agree with an independent transactional projection.
#[test]
fn prepared_fields_follow_portable_atomic_outcomes() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️prepared-fields/🔣️.json")).expect("portable field corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        FIELD_RETIREMENTS.with(|value| value.set(0));
        let mut projection = Projection { first: corpus["base"]["first"].as_i64().unwrap(), second: corpus["base"]["second"].as_i64().unwrap() };
        let diff = Diff { first: case["diff"]["first"].as_i64(), second: case["diff"]["second"].as_i64(), refuse: case["diff"]["refuse"].as_bool().unwrap_or(false) };
        assert_eq!(apply_diff_cold_into(&diff, &mut projection).is_err(), case["refused"].as_bool().unwrap());
        assert_eq!(projection.first, case["expected"]["first"].as_i64().unwrap());
        assert_eq!(projection.second, case["expected"]["second"].as_i64().unwrap());
        assert_eq!(FIELD_RETIREMENTS.with(|value| value.get()), usize::from(diff.first.is_some()) + usize::from(diff.second.is_some()));
    }
    eprintln!("[DEBUG] Shared prepared fields preserve portable atomic projections and retire each staged/displaced field once");
}
