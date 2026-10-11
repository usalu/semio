//! 🗑️ Note mutation — `DeleteAsset`: removes an id-keyed image asset.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-asset` payload — removes an id-keyed image asset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-asset")]
pub struct DeleteAsset {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_asset(key: String) -> NoteMutation {
    NoteMutation::DeleteAsset(DeleteAsset { key })
}

impl MutationKind<NoteSnapshot, NoteMutation> for DeleteAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "asset", kind: "delete-asset", record: "DeletedAsset" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete asset \"{}\"", self.key), &format!("Asset \"{}\" löschen", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
