//! 🏗️ `create-solid` — brings a new id-keyed solid into existence with its full initial `shells` membership list (each flagged void/non-void, referencing already-existing shells). A duplicate `id` already present in `base` is a no-op.

use crate::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::standards::v1::subsets::brep::schema::snapshot::{BrepSolidShell, SemioBrepSnapshot};

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

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for CreateSolid {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "solid", kind: "create-solid", record: "CreatedSolid" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create solid \"{}\"", self.id), &format!("Körper \"{}\" erstellen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
