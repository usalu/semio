//! 🧫️ One committed quintet of a mutation vector: its raw texts read from the fixture mirror, and the typed views the laws start from.

use semio_framework_os_kernel as protocol;
use semio_framework_pack_json::{from_json_str, parse, JsonMemberPolicy, Value};
use semio_framework_test_fixtures::fixture_path;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use std::path::{Path, PathBuf};

//#region 🧫️Vector
/// 🧫️ One committed quintet, read from disk.
pub struct Vector {
    pub(super) label: String,
    pub(super) before: String,
    pub(super) after: String,
    pub(super) mutation: String,
    outcome: String,
    pub(super) diff: Option<String>,
    pub(super) absent_marker: Option<String>,
}

/// 📂️ The fixture mirror of this subset's `🧬️mutations` facet, seen from a mutation group package.
fn fixture_root() -> PathBuf {
    fixture_path!("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("committed vector file {} is readable: {error}", path.display()))
}

impl Vector {
    /// 📖️ Reads the vector `<leaf>/<case>` of the fixture mirror.
    pub fn load(leaf: &str, case: &str) -> Self {
        let root = fixture_root().join(leaf).join(case);
        assert!(root.is_dir(), "the committed vector {} must exist", root.display());
        let diff = root.join("🔺️diff");
        Self {
            label: format!("{leaf}/{case}"),
            before: read(&root.join("📸️snapshot/⬅️before/🔣️.json")),
            after: read(&root.join("📸️snapshot/➡️after/🔣️.json")),
            mutation: read(&root.join("🦠️mutation/🔣️.json")),
            outcome: read(&root.join("🎯️outcome/🔣️.json")),
            diff: diff.join("🔣️.json").is_file().then(|| read(&diff.join("🔣️.json"))),
            absent_marker: diff.join("🚫️.absent").is_file().then(|| read(&diff.join("🚫️.absent"))),
        }
    }

    /// 📸️ The committed before-document.
    pub fn before(&self) -> RemodelingSnapshot {
        from_json_str(&self.before, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}: before snapshot decodes: {error:?}", self.label))
    }

    pub(super) fn expected_after(&self) -> RemodelingSnapshot {
        from_json_str(&self.after, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}: after snapshot decodes: {error:?}", self.label))
    }

    /// 🦠️ The committed mutation.
    pub fn mutation(&self) -> RemodelingMutation {
        from_json_str(&self.mutation, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}: mutation decodes: {error:?}", self.label))
    }

    pub(super) fn declared(&self) -> Value {
        parse(&self.outcome, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}: outcome decodes: {error:?}", self.label))
    }

    pub(super) fn status(&self) -> String {
        self.declared().get("status").and_then(|status| status.as_str()).map(str::to_string).unwrap_or_else(|| panic!("{}: the outcome declares a status", self.label))
    }

    pub(super) fn produced(&self) -> protocol::MutationOutcome<RemodelingDiff> {
        <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(&self.mutation(), &self.before())
    }

    pub(super) fn committed_diff(&self) -> Value {
        parse(self.diff.as_deref().unwrap_or_else(|| panic!("{}: a committed 🔺️diff/🔣️.json", self.label)), JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}: committed diff decodes: {error:?}", self.label))
    }
}

pub(super) fn json_of<T: semio_framework_value::ToValue>(value: &T) -> Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(value))
}

/// 🚦️ The level every refusal code of this vocabulary is raised at: a broken document invariant or a duplicate id is
/// Fatal, a missing, mismatched or still-referenced target is an Error.
pub(super) fn refusal_level(code: &str) -> semio_framework_diagnostic::Severity {
    match code {
        "mutation.duplicate-id" | "mutation.invariant" => semio_framework_diagnostic::Severity::Fatal,
        "mutation.target-missing" | "mutation.target-mismatch" | "mutation.target-referenced" => semio_framework_diagnostic::Severity::Error,
        other => panic!("refusal code {other:?} has no declared level"),
    }
}
//#endregion 🧫️Vector
