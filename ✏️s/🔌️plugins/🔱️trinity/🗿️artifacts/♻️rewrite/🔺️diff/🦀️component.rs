//! 🔺️ `trinity.rewrite.rule` artifact — diff structs + `OperationDiff` impl (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::rewrite::RewriteRuleModel;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-state snapshot diff: the rule document is one small unit, so history stores full pre/post states rather than field-level patches.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewriteRuleDiff {
    pub next: Option<RewriteRuleModel>,
}

impl OperationDiff<RewriteRuleModel> for RewriteRuleDiff {
    fn apply(&self, projection: &RewriteRuleModel) -> RewriteRuleModel {
        self.next.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.next.is_some() {
            self.next = other.next;
        }
    }
}
