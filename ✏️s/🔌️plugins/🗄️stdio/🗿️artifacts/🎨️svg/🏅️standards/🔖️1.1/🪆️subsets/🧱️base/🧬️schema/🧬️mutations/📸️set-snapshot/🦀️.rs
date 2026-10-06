//! 📸️ Whole SvgSnapshot replacement for schema-driven editing.

use super::SvgMutation;
use crate::schema::diff::SvgDiff;
use crate::SvgSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SvgSnapshot,
}


impl protocol::MutationKind<SvgSnapshot, SvgMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgMutation as Mutation<SvgSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SvgDiff as DiffAlgebra<SvgSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SvgMutation::SetSnapshot(Self { snapshot: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
