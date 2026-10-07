//! 📸️ Whole BMP snapshot replacement for schema-driven editing.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::BmpMutation;
use crate::BmpSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: BmpSnapshot,
}


impl protocol::MutationKind<BmpSnapshot, BmpMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<<BmpMutation as Mutation<BmpSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<BmpDiff as DiffAlgebra<BmpSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![BmpMutation::SetSnapshot(Self { snapshot: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen")
    }
    fn target(&self) -> Vec<String> { Vec::new() }
}
