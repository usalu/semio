//! 📸️ Whole SemioBrepSnapshot replacement for schema-driven editing.

use super::SemioBrepMutation;
use crate::standards::v1::subsets::brep::schema::diff::{
    dec_curve, dec_list, dec_point3, dec_shell_face, dec_solid_shell, dec_str, dec_surface, enc_bool, enc_curve, enc_list, enc_point3, enc_shell_face, enc_solid_shell, enc_str, enc_surface, parse_f64, SemioBrepDiff,
};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioBrepSnapshot,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as Mutation<SemioBrepSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        vec![SemioBrepMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
