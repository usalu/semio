//! 📸️ Whole SemioTableSnapshot replacement for schema-driven editing.

use super::SemioTableMutation;
use crate::standards::v1::subsets::table::schema::diff::SemioTableDiff;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioTableSnapshot,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as Mutation<SemioTableSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioTableDiff as DiffAlgebra<SemioTableSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Result<Vec<SemioTableMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SemioTableMutation::SetSnapshot(Self { snapshot: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
