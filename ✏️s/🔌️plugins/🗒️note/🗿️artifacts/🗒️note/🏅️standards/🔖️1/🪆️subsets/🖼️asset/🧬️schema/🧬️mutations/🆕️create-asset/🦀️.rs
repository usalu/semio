//! 🆕 Note mutation — `CreateAsset`: brings a new id-keyed image asset into existence.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🆕 `create-asset` payload — brings a new id-keyed image asset into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-asset")]
pub struct CreateAsset {
    pub key: String,
    #[dsl(block)]
    pub asset: crate::NoteImageAsset,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_asset<Op>(key: String, asset: crate::NoteImageAsset) -> Op
where
    Op: From<crate::standards::v1::subsets::asset::schema::mutations::create_asset::CreateAsset>,
{
    Op::from(CreateAsset { key, asset })
}

impl<Op> MutationKind<NoteSnapshot, Op> for CreateAsset
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<CreateAsset> + From<crate::standards::v1::subsets::asset::schema::mutations::delete_asset::DeleteAsset>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "asset", kind: "create-asset", record: "CreatedAsset" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create asset \"{}\"", self.key), &format!("Asset \"{}\" erstellen", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for CreateAsset
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<CreateAsset> + From<crate::standards::v1::subsets::asset::schema::mutations::delete_asset::DeleteAsset>,
{
    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
