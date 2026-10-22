//! 🗑️ `delete-shell` — removes an id-keyed shell. Does NOT cascade into `solid.shells` membership, for the same reason `delete-face` does not cascade into `shell.faces` — flagged, not invented. Absent `id` is a no-op.

use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteShell {
    pub id: String,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for DeleteShell
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<DeleteShell> + From<crate::schema::mutations::create_shell::CreateShell>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "shell", kind: "delete-shell", record: "DeletedShell" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete shell \"{}\"", self.id), &format!("Schale \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for DeleteShell
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<DeleteShell> + From<crate::schema::mutations::create_shell::CreateShell>,
{
    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioBrepSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
