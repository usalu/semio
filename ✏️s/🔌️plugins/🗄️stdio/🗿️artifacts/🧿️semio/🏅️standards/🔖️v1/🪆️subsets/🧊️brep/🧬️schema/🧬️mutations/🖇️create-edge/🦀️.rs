//! 🏗️ `create-edge` — brings a new id-keyed edge into existence between two vertices (referential integrity across `start_vertex`/`end_vertex` is the subset validator's job, not this diff constructor's), carrying its own curve. A duplicate `id` already present in `base` is a no-op.

use crate::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, SemioBrepSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateEdge {
    pub id: String,
    pub start_vertex: String,
    pub end_vertex: String,
    pub curve: BrepCurve,
    pub tol: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for CreateEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "edge", kind: "create-edge", record: "CreatedEdge" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create edge \"{}\"", self.id), &format!("Kante \"{}\" erstellen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
