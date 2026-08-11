//! 🏗️ Ifc2x3SavBuilder (2x3/✳️sav) — `new()` seeds `FILE_SCHEMA IFC2X3` + a
//! `ViewDefinition [StructuralAnalysisView]` header plus a real `IFCSTRUCTURALANALYSISMODEL`
//! instance, so the recommended path can never reach a built snapshot missing the one HARD
//! requirement (`check_sav_conformance`'s `CODE_NO_ANALYSIS_MODEL`). `build()` still re-runs the
//! full conformance check unconditionally.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ifc::standards::v2x3::subsets::sav::analyzer::check_sav_conformance;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

fn seeded_document() -> Part21Document {
    let header = Part21Header {
        file_description: vec![
            Part21Value::List(vec![Part21Value::Str("ViewDefinition [StructuralAnalysisView]".into())]),
            Part21Value::Str("2;1".into()),
        ],
        file_name: vec![],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
    };
    let model = Part21Instance { id: 1, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] };
    Part21Document { header, instances: vec![model] }
}

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct Ifc2x3SavBuilder {
    snapshot: Ifc2x3Snapshot,
}

impl Ifc2x3SavBuilder {
    pub fn new() -> Self {
        Self { snapshot: Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: seeded_document() } }
    }

    /// ⚖️ Adds a load group (`IFCSTRUCTURALLOADGROUP`) instance.
    pub fn add_load_group(mut self, id: u64) -> Self {
        apply_ifc2x3_mutation(&mut self.snapshot, &Ifc2x3Mutation::UpsertInstance { instance: Part21Instance { id, entities: vec![("IFCSTRUCTURALLOADGROUP".into(), vec![])] } });
        self
    }
}

impl Default for Ifc2x3SavBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactBuilder for Ifc2x3SavBuilder {
    type Snapshot = Ifc2x3Snapshot;
    type Mutation = Ifc2x3Mutation;
    type Diff = Ifc2x3Diff;

    fn empty() -> Self {
        Self::new()
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_sav_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() { Ok(self.snapshot) } else { Err(hard) }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_builds_clean() {
        let snapshot = Ifc2x3SavBuilder::new().add_load_group(2).build().expect("conforming construction must build");
        assert_eq!(snapshot.document.instances.len(), 2);
    }

    #[test]
    fn removing_the_analysis_model_via_raw_mutate_still_fails_build() {
        let snapshot = Ifc2x3SavBuilder::new().build().unwrap();
        let (mutated, _diff) = Ifc2x3SavBuilder::from_snapshot(snapshot).mutate(Ifc2x3Mutation::RemoveInstance { id: 1 });
        let err = mutated.build().expect_err("removing the only analysis model must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::ifc::standards::v2x3::subsets::sav::analyzer::CODE_NO_ANALYSIS_MODEL));
    }
}
