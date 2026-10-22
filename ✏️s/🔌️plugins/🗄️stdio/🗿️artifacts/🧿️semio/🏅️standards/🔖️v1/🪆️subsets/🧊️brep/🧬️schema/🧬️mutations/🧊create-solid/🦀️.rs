//! 🏗️ `create-solid` — brings a new id-keyed solid into existence with its full initial `shells` membership list (each flagged void/non-void, referencing already-existing shells). A duplicate `id` already present in `base` is a no-op.

use crate::schema::snapshot::{BrepSolidShell, SemioBrepSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSolid {
    pub id: String,
    #[value(default)]
    pub shells: Vec<BrepSolidShell>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for CreateSolid
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<CreateSolid> + From<crate::schema::mutations::delete_solid::DeleteSolid>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "solid", kind: "create-solid", record: "CreatedSolid" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create solid \"{}\"", self.id), &format!("Körper \"{}\" erstellen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for CreateSolid
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<CreateSolid> + From<crate::schema::mutations::delete_solid::DeleteSolid>,
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
