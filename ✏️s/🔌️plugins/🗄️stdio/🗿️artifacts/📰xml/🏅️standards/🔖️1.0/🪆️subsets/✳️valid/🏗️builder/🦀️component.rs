//! 🏗️ XmlValidBuilder (1.0/✳️valid) — `ArtifactBuilder` wrapper whose `build()` re-runs the SAME
//! `check_valid_conformance` used by `XmlValidComposer`, unconditionally, regardless of which path
//! (`from_snapshot`/`from_text`/`from_binary`/`mutate`) produced the in-flight snapshot -- so a
//! hard XML 1.0 §5.1 validity violation (missing doctype, root-name mismatch) can never leave this
//! builder as an `Ok(XmlSnapshot)`.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xml::standards::v1_0::subsets::any::builder::XmlBuilder as XmlAnyBuilder;
use crate::artifacts::xml::standards::v1_0::subsets::any::schema::diff::XmlDiff;
use crate::artifacts::xml::standards::v1_0::subsets::any::schema::mutations::XmlMutation;
use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::XmlSnapshot;
use crate::artifacts::xml::standards::v1_0::subsets::valid::analyzer::check_valid_conformance;

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct XmlValidBuilder(XmlAnyBuilder);

impl ArtifactBuilder for XmlValidBuilder {
    type Snapshot = XmlSnapshot;
    type Mutation = XmlMutation;
    type Diff = XmlDiff;

    fn empty() -> Self {
        Self(XmlAnyBuilder::empty())
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self(XmlAnyBuilder::from_snapshot(snapshot))
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self(XmlAnyBuilder::from_text(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self(XmlAnyBuilder::from_binary(bytes)?))
    }

    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let (inner, diff) = self.0.mutate(mutation);
        (Self(inner), diff)
    }

    fn absorb(self, diff: Self::Diff) -> Self {
        Self(self.0.absorb(diff))
    }

    /// 🛡️ The real construction gate: however `self.0`'s inner snapshot got here, a hard XML 1.0
    /// §5.1 validity violation fails `build()` -- soft/advisory diagnostics pass through as `Ok`.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let snapshot = self.0.build()?;
        let hard: Vec<Diagnostic> = check_valid_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
        let snapshot = XmlValidBuilder::from_text("<!DOCTYPE root>\n<root/>").expect("parses").build().expect("conforming construction must build");
        assert_eq!(snapshot.doc.doctype.as_deref(), Some("<!DOCTYPE root>"));
    }

    #[test]
    fn missing_doctype_fails_build() {
        let err = XmlValidBuilder::from_text("<root/>").expect("parses").build().expect_err("a document without a doctype must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing"));
    }

    #[test]
    fn root_name_mismatch_injected_via_raw_mutate_still_fails_build() {
        let built = XmlValidBuilder::from_text("<!DOCTYPE root>\n<root/>").expect("parses").build().expect("clean build");
        let mut mismatched = built;
        mismatched.doc.doctype = Some("<!DOCTYPE somethingElse>".into());
        let (mutated, _diff) = XmlValidBuilder::from_snapshot(XmlSnapshot::default()).mutate(XmlMutation::SetSnapshot { snapshot: mismatched });
        let err = mutated.build().expect_err("a doctype/root name mismatch must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.xml.valid.root-name-mismatch"));
    }
}
