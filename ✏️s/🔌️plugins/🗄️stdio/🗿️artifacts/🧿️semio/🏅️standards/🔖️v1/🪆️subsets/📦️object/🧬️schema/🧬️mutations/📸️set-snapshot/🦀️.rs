//! 📸️ Whole SemioObjectSnapshot replacement for schema-driven editing.

use super::SemioObjectMutation;
use crate::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioObjectSnapshot,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as Mutation<SemioObjectSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioObjectDiff as DiffAlgebra<SemioObjectSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SemioObjectMutation::SetSnapshot(Self { snapshot: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
