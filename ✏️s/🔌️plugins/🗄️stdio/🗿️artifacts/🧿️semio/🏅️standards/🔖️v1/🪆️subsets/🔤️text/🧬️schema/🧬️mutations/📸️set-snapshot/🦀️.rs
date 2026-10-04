//! 📸️ Whole SemioTextSnapshot replacement for schema-driven editing.

use super::SemioTextMutation;
use crate::standards::v1::subsets::text::schema::diff::SemioTextDiff;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioTextSnapshot,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioTextSnapshot) -> protocol::MutationOutcome<<SemioTextMutation as Mutation<SemioTextSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioTextDiff as DiffAlgebra<SemioTextSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Result<Vec<SemioTextMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SemioTextMutation::SetSnapshot(Self { snapshot: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
