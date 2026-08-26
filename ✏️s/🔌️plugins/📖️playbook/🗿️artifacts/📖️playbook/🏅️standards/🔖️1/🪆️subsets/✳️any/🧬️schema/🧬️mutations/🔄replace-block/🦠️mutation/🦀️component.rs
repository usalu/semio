//! 🔄 Playbook mutation — `ReplaceBlock`: whole-value swap of a block's configuration. `PlaybookBlock`
//! carries ~18 kind-dependent optional fields (see the artifact's `🦀️component.rs`) edited together
//! as one property-panel form, never one field at a time — this fails `update`'s "all fields
//! required, cohesive facet" restriction (most fields are `Option` and only a kind-dependent subset
//! applies at once), so it takes taxonomy's `replace` verb ("whole-value swap of a large structured
//! sub-payload") instead of `update`. Was `PlaybookMutation::UpdateBlock` pre-migration.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::{PlaybookBlock, PlaybookSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
