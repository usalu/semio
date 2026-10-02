//! 🩹️ Relative rewriting mutation — `PatchWorkingNodes`: one field (`name` or `kind`) of a set of working-graph nodes set to
//! one value. The rail's patch verb yields it; history edits which nodes, which field and the value, and replays them on any
//! graph; its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🩹️ `patch-working-nodes` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "patch-working-nodes")]
pub struct PatchWorkingNodes {
    pub targets: Vec<String>,
    pub field: String,
    pub value: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn patch_working_nodes(targets: Vec<String>, field: String, value: String) -> RewriteRuleMutation {
    RewriteRuleMutation::PatchWorkingNodes(PatchWorkingNodes { targets, field, value })
}

impl PatchWorkingNodes {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice, the `name` or `kind` field, a non-blank value.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) && matches!(self.field.as_str(), "name" | "kind") && !self.value.trim().is_empty()
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for PatchWorkingNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "patch", entity: "working-nodes", kind: "patch-working-nodes", record: "PatchedWorkingNodes" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 node".to_string(), "1 Knoten".to_string()),
            count => (format!("{count} nodes"), format!("{count} Knoten")),
        };
        match self.field.as_str() {
            "kind" => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the kind of {items_en} to “{}”", self.value), &format!("Art von {items_de} auf „{}“ setzen", self.value)),
            _ => semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename {items_en} to “{}”", self.value), &format!("{items_de} in „{}“ umbenennen", self.value)),
        }
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
