//! 🧷 Remodeling mutation — `CreateAsset`: upserts one key-addressed `ImageAsset` (the only asset
//! write path in the app — import handlers always call this, overwriting is intentional so a retried
//! import with the same key lands cleanly).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{ImageAsset, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧷 `create-asset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-asset")]
pub struct CreateAsset {
    pub key: String,
    #[dsl(block)]
    pub asset: ImageAsset,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_asset(key: String, asset: ImageAsset) -> RemodelingMutation {
    RemodelingMutation::CreateAsset(CreateAsset { key, asset })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for CreateAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "asset", kind: "create-asset", record: "CreatedAsset" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create asset \"{}\"", self.key), &format!("Asset \"{}\" erstellen", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
