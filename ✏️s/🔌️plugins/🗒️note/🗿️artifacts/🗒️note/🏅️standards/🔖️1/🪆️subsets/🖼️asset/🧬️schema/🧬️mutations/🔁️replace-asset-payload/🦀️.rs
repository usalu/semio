//! 🔁 Note mutation — `ReplaceAssetPayload`: whole-value swap of an existing asset's image payload.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-asset-payload` payload — whole-value swap of an existing asset's image payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-asset-payload")]
pub struct ReplaceAssetPayload {
    pub key: String,
    #[dsl(block)]
    pub new_asset: crate::NoteImageAsset,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_asset_payload<Op>(key: String, new_asset: crate::NoteImageAsset) -> Op
where
    Op: From<crate::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::ReplaceAssetPayload>,
{
    Op::from(ReplaceAssetPayload { key, new_asset })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ReplaceAssetPayload
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ReplaceAssetPayload>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "asset", kind: "replace-asset-payload", record: "ReplacedAsset" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace asset \"{}\"", self.key), &format!("Asset \"{}\" ersetzen", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for ReplaceAssetPayload
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<ReplaceAssetPayload>,
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
