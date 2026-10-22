//! 🧿 Remodeling mutation — `CreateGcp`: brings a new id-keyed ground control point into existence
//! (id minted by the caller via `mint_remodeling_id(operation, "gcp")`).

use crate::standards::v1::subsets::any::schema::mutations::delete_gcp::DeleteGcp;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GroundControlPoint;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧿 `create-gcp` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-gcp")]
pub struct CreateGcp {
    #[dsl(block)]
    pub gcp: GroundControlPoint,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_gcp<Op: From<CreateGcp>>(gcp: GroundControlPoint) -> Op {
    Op::from(CreateGcp { gcp })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for CreateGcp
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<DeleteGcp>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "gcp", kind: "create-gcp", record: "CreatedGcp" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create GCP \"{}\"", self.gcp.id), &format!("GCP \"{}\" erstellen", self.gcp.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.gcp.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for CreateGcp
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<DeleteGcp>,
{
    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
