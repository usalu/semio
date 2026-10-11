//! 🏗️ `create-vertex` — brings a new id-keyed vertex into existence at `point`. A duplicate `id` already present in `base` is a no-op (never a duplicate id).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateVertex {
    pub id: String,
    pub point: SemioPoint3,
    pub tol: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for CreateVertex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vertex", kind: "create-vertex", record: "CreatedVertex" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create vertex \"{}\"", self.id), &format!("Vertex \"{}\" erstellen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
