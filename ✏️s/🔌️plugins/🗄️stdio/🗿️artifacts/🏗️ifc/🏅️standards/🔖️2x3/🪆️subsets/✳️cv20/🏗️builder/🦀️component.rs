//! 🏗️ Ifc2x3Cv20Builder (2x3/✳️cv20) — a typed builder whose `new()` entry point can only produce
//! a Coordination View 2.0-conforming `Ifc2x3Snapshot` BY CONSTRUCTION: it seeds `FILE_SCHEMA
//! IFC2X3`, a `ViewDefinition [CoordinationView]` `FILE_DESCRIPTION`, and a single `IFCPROJECT`
//! with a real `IFCUNITASSIGNMENT` reference. `add_product` is the only content-mutating method,
//! and it always wires a real `IFCLOCALPLACEMENT` reference -- there is no way to reach a built
//! snapshot with an unplaced product via this builder's own vocabulary. `build()` still re-runs
//! the SAME `check_cv20_conformance` `Ifc2x3Cv20Composer` uses, unconditionally, so a hard
//! violation injected via the generic `mutate`/`SetSnapshot` escape hatch is still caught.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ifc::standards::v2x3::subsets::cv20::analyzer::check_cv20_conformance;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

//#region 🔖️Seed
const PLACEMENT_ID: u64 = 10;
const UNITS_ID: u64 = 20;
const PROJECT_ID: u64 = 1;

fn seeded_document() -> Part21Document {
    let header = Part21Header {
        file_description: vec![
            Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView]".into())]),
            Part21Value::Str("2;1".into()),
        ],
        file_name: vec![],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
    };
    let placement = Part21Instance { id: PLACEMENT_ID, entities: vec![("IFCLOCALPLACEMENT".into(), vec![])] };
    let units = Part21Instance { id: UNITS_ID, entities: vec![("IFCUNITASSIGNMENT".into(), vec![])] };
    let project = Part21Instance {
        id: PROJECT_ID,
        entities: vec![(
            "IFCPROJECT".into(),
            vec![
                Part21Value::Str("0000000000000000000000".into()),
                Part21Value::Unset,
                Part21Value::Str("Project".into()),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Ref(UNITS_ID),
            ],
        )],
    };
    Part21Document { header, instances: vec![placement, units, project] }
}
//#endregion 🔖️Seed

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct Ifc2x3Cv20Builder {
    snapshot: Ifc2x3Snapshot,
}

impl Ifc2x3Cv20Builder {
    /// ➕ The recommended entry point: always produces a document with `IFC2X3`/`CoordinationView`
    /// header and a real project+units pair.
    pub fn new() -> Self {
        Self { snapshot: Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: seeded_document() } }
    }

    /// 🧱️ Adds a product instance of `type_name` (must be one of the geometry-bearing product
    /// types this MVD checks), always wiring `ObjectPlacement` (attribute index 5) to the seeded
    /// `IFCLOCALPLACEMENT`.
    pub fn add_product(mut self, id: u64, type_name: &str, name: &str) -> Self {
        let instance = Part21Instance {
            id,
            entities: vec![(
                type_name.to_string(),
                vec![
                    Part21Value::Str(format!("guid-{id}")),
                    Part21Value::Unset,
                    Part21Value::Str(name.to_string()),
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Ref(PLACEMENT_ID),
                ],
            )],
        };
        apply_ifc2x3_mutation(&mut self.snapshot, &Ifc2x3Mutation::UpsertInstance { instance });
        self
    }
}

impl Default for Ifc2x3Cv20Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactBuilder for Ifc2x3Cv20Builder {
    type Snapshot = Ifc2x3Snapshot;
    type Mutation = Ifc2x3Mutation;
    type Diff = Ifc2x3Diff;

    /// ⚠️ `ArtifactBuilder::empty()` is mandated no-arg by the SDK trait -- falls back to
    /// `Ifc2x3Cv20Builder::new()`'s seeded document rather than a truly empty (non-conforming)
    /// one, since `build()` requires conformance regardless.
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

    /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard CV2.0 violation
    /// fails `build()`; soft diagnostics pass through as advisory (the `Err` path is not taken).
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_cv20_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() { Ok(self.snapshot) } else { Err(hard) }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_builds_clean() {
        let snapshot = Ifc2x3Cv20Builder::new().add_product(2, "IFCWALL", "Wall 1").build().expect("conforming construction must build");
        assert_eq!(snapshot.document.instances.len(), 4);
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let violating = Part21Instance { id: 99, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] };
        let mut snapshot = Ifc2x3Cv20Builder::new().build().unwrap();
        snapshot.document.instances.push(violating);
        let (mutated, _diff) = Ifc2x3Cv20Builder::from_snapshot(Ifc2x3Snapshot::default()).mutate(Ifc2x3Mutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a structural entity must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::ifc::standards::v2x3::subsets::cv20::analyzer::CODE_STRUCTURAL_ENTITY));
    }
}
