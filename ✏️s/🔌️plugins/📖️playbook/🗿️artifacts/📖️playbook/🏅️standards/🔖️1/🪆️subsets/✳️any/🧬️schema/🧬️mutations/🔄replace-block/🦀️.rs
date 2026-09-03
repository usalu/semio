//! 🔄 Playbook mutation — `ReplaceBlock`: whole-value swap of a block's configuration. `PlaybookBlock`
//! carries ~18 kind-dependent optional fields (see the artifact's `🦀️.rs`) edited together
//! as one property-panel form, never one field at a time — this fails `update`'s "all fields
//! required, cohesive facet" restriction (most fields are `Option` and only a kind-dependent subset
//! applies at once), so it takes taxonomy's `replace` verb ("whole-value swap of a large structured
//! sub-payload") instead of `update`. Was `PlaybookMutation::UpdateBlock` pre-migration.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookBlock, PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
// 🔬️ `Serialize`/`Deserialize` survive ONLY as a `#[cfg(test)]` differential oracle — committed
// `🧪️tests/<fixture>/🦀️.rs` fixture vectors decode/re-encode through them — never a production
// dependency of this crate.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-block")]
pub struct ReplaceBlock {
    pub step_id: String,
    #[dsl(block)]
    pub block: PlaybookBlock,
}

/// 🏗️ Builder.
pub fn replace_block_operation(step_id: &str, block: PlaybookBlock) -> PlaybookMutation {
    PlaybookMutation::ReplaceBlock(ReplaceBlock { step_id: step_id.into(), block })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for ReplaceBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "block", kind: "replace-block", record: "ReplacedBlock" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace block \"{}\"", self.block.label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block.id.clone()]
    }
}
//#endregion 🔖️Mutation
