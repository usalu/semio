//! 🏗️ JsonIJsonBuilder (rfc8259/✳️i-json) — `ArtifactBuilder` wrapper whose `build()` re-runs the
//! SAME `check_i_json_conformance` used by `JsonIJsonComposer`, unconditionally, regardless of
//! which path (`from_snapshot`/`from_text`/`from_binary`/`mutate`) produced the in-flight
//! snapshot -- so a hard RFC 7493 violation (duplicate member name, integer beyond ±(2^53-1)) can
//! never leave this builder as an `Ok(JsonSnapshot)`.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::json::standards::v_rfc8259::subsets::any::builder::JsonBuilder as JsonAnyBuilder;
use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::diff::JsonDiff;
use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::mutations::JsonMutation;
use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::JsonSnapshot;
use crate::artifacts::json::standards::v_rfc8259::subsets::i_json::analyzer::check_i_json_conformance;

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct JsonIJsonBuilder(JsonAnyBuilder);

impl ArtifactBuilder for JsonIJsonBuilder {
    type Snapshot = JsonSnapshot;
    type Mutation = JsonMutation;
    type Diff = JsonDiff;

    fn empty() -> Self {
        Self(JsonAnyBuilder::empty())
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self(JsonAnyBuilder::from_snapshot(snapshot))
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self(JsonAnyBuilder::from_text(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self(JsonAnyBuilder::from_binary(bytes)?))
    }

    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let (inner, diff) = self.0.mutate(mutation);
        (Self(inner), diff)
    }

    fn absorb(self, diff: Self::Diff) -> Self {
        Self(self.0.absorb(diff))
    }

    /// 🛡️ The real construction gate: however `self.0`'s inner snapshot got here, a hard RFC 7493
    /// violation fails `build()` -- soft/advisory diagnostics pass through as `Ok`.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let snapshot = self.0.build()?;
        let hard: Vec<Diagnostic> = check_i_json_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(snapshot)
        } else {
            Err(hard)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conforming_snapshot_builds_clean() {
        let snapshot = JsonIJsonBuilder::from_text("{\"a\":1}").expect("parses").build().expect("conforming construction must build");
        assert!(matches!(snapshot.value, crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::JsonValue::Object { .. }));
    }

    #[test]
    fn duplicate_member_name_fails_build() {
        let err = JsonIJsonBuilder::from_text("{\"a\":1,\"a\":2}").expect("parses").build().expect_err("a duplicate member name must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.json.i-json.duplicate-member-name"));
    }

    #[test]
    fn unsafe_integer_injected_via_raw_mutate_still_fails_build() {
        use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::{JsonMember, JsonValue};
        let violating = JsonSnapshot { value: JsonValue::Object { members: vec![JsonMember { key: "n".into(), value: JsonValue::Number { lexeme: "9007199254740993".into() } }] }, ..JsonSnapshot::default() };
        let (mutated, _diff) = JsonIJsonBuilder::from_snapshot(JsonSnapshot::default()).mutate(JsonMutation::SetSnapshot { snapshot: violating });
        let err = mutated.build().expect_err("an unsafe integer must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.json.i-json.unsafe-integer"));
    }
}
