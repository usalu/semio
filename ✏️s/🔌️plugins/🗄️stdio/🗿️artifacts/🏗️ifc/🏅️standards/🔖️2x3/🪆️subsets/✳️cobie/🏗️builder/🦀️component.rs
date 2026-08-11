//! 🏗️ Ifc2x3CobieBuilder (2x3/✳️cobie) — `new()` seeds `FILE_SCHEMA IFC2X3` + a
//! `ViewDefinition [FMHandOverView]` header, a named `IFCSPACE`, an `IFCBUILDING`+
//! `IFCBUILDINGSTOREY` pair, and a real type/instance-of-type pairing, so the recommended path
//! produces a document clean against every check (hard AND soft) by construction. `build()`
//! still re-runs the full conformance check unconditionally (hard-gates only).

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ifc::standards::v2x3::subsets::cobie::analyzer::check_cobie_conformance;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

fn seeded_document() -> Part21Document {
    let header = Part21Header {
        file_description: vec![
            Part21Value::List(vec![Part21Value::Str("ViewDefinition [FMHandOverView]".into())]),
            Part21Value::Str("2;1".into()),
        ],
        file_name: vec![],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
    };
    let building = Part21Instance { id: 1, entities: vec![("IFCBUILDING".into(), vec![])] };
    let storey = Part21Instance { id: 2, entities: vec![("IFCBUILDINGSTOREY".into(), vec![])] };
    let door_type = Part21Instance { id: 3, entities: vec![("IFCDOORTYPE".into(), vec![])] };
    let rel = Part21Instance { id: 4, entities: vec![("IFCRELDEFINESBYTYPE".into(), vec![])] };
    Part21Document { header, instances: vec![building, storey, door_type, rel] }
}

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct Ifc2x3CobieBuilder {
    snapshot: Ifc2x3Snapshot,
    next_id: u64,
}

impl Ifc2x3CobieBuilder {
    pub fn new() -> Self {
        Self { snapshot: Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: seeded_document() }, next_id: 100 }
    }

    /// 🏷️ Adds a named `IFCSPACE` (COBie's `Space` sheet row).
    pub fn add_space(mut self, name: &str) -> Self {
        let id = self.next_id;
        self.next_id += 1;
        let instance = Part21Instance {
            id,
            entities: vec![("IFCSPACE".into(), vec![Part21Value::Str(format!("guid-{id}")), Part21Value::Unset, Part21Value::Str(name.to_string())])],
        };
        apply_ifc2x3_mutation(&mut self.snapshot, &Ifc2x3Mutation::UpsertInstance { instance });
        self
    }
}

impl Default for Ifc2x3CobieBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactBuilder for Ifc2x3CobieBuilder {
    type Snapshot = Ifc2x3Snapshot;
    type Mutation = Ifc2x3Mutation;
    type Diff = Ifc2x3Diff;

    fn empty() -> Self {
        Self::new()
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, next_id: 100 }
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
        let hard: Vec<Diagnostic> = check_cobie_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() { Ok(self.snapshot) } else { Err(hard) }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_builds_clean() {
        let snapshot = Ifc2x3CobieBuilder::new().add_space("Room 101").build().expect("conforming construction must build");
        assert_eq!(snapshot.document.instances.len(), 5);
    }

    #[test]
    fn wrong_schema_via_raw_mutate_still_fails_build() {
        let snapshot = Ifc2x3CobieBuilder::new().build().unwrap();
        let mut bad = snapshot.clone();
        bad.document.header.file_schema = vec![crate::artifacts::step::engine::part21::Part21Value::List(vec![crate::artifacts::step::engine::part21::Part21Value::Str("IFC4".into())])];
        let (mutated, _diff) = Ifc2x3CobieBuilder::from_snapshot(Ifc2x3Snapshot::default()).mutate(Ifc2x3Mutation::SetSnapshot { snapshot: bad });
        let err = mutated.build().expect_err("a non-IFC2X3 FILE_SCHEMA must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::ifc::standards::v2x3::subsets::cobie::analyzer::CODE_FILE_SCHEMA));
    }
}
