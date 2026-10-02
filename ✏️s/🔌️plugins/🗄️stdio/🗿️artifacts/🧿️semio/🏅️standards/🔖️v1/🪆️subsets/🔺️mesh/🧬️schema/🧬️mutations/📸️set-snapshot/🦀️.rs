//! 📸️ Whole SemioMeshSnapshot replacement for schema-driven editing.

use super::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::diff::{
    dec_list, dec_material, dec_mesh, dec_point3, dec_primitive, dec_rgba, dec_str, dec_texture, dec_topology, dec_uv, decode_option, enc_list, enc_material, enc_mesh, enc_point3, enc_primitive, enc_rgba, enc_str, enc_texture, enc_topology, enc_uv,
    encode_option, hex_decode, hex_encode, SemioMeshDiff,
};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioMeshSnapshot,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as Mutation<SemioMeshSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        vec![SemioMeshMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
