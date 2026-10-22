//! 🗑️ `delete-face` — removes an id-keyed face. Does NOT cascade into `shell.faces` membership: no modify-verb exists for a shell's face list (only `create-shell`/`delete-shell` govern the shell's own existence), so severing that membership here would be uninvertible within the approved vocabulary — flagged, not invented. Absent `id` is a no-op.

use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteFace {
    pub id: String,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for DeleteFace
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<DeleteFace> + From<crate::schema::mutations::create_face::CreateFace>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "face", kind: "delete-face", record: "DeletedFace" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete face \"{}\"", self.id), &format!("Fläche \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for DeleteFace
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<DeleteFace> + From<crate::schema::mutations::create_face::CreateFace>,
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
