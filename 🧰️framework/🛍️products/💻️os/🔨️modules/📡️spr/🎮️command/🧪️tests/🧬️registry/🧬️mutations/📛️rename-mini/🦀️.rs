//! 📛️ Direct rename payload and behavior for the miniature registry fixture.

use super::super::{MiniDiff, MiniDoc, MiniMutation};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RenameMini {
    pub new_name: String,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<MiniDoc, MiniMutation> for RenameMini {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "mini", kind: "rename-mini", record: "RenamedMini" };
    fn diff(&self, _base: &MiniDoc) -> MutationOutcome<MiniDiff> {
        MutationOutcome::new(MiniDiff { name: Some(self.new_name.clone()) })
    }
    fn inverse(&self, base: &MiniDoc) -> Vec<MiniMutation> {
        vec![Self { new_name: base.name.clone() }.into()]
    }
    fn label(&self) -> String {
        format!("Rename mini to \"{}\"", self.new_name)
    }
}
//#endregion ⚙️Behavior
