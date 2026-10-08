//! 🌱 Puzzle2d mutation — `CreateNode`: brings a new id-keyed node into existence.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::{Puzzle2dNode, Puzzle2dSnapshot};

//#region 🔖️Mutation
/// 🌱 `create-node` payload — full initial payload at an optional FINAL-state `index` (`None`
/// appends). A duplicate `node.id` refuses creation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-node")]
pub struct CreateNode {
    #[dsl(block)]
    pub node: Puzzle2dNode,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_node(node: Puzzle2dNode, index: Option<usize>) -> Puzzle2dMutation {
    Puzzle2dMutation::CreateNode(CreateNode { node, index })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create node \"{}\"", self.node.id), &format!("Knoten \"{}\" erstellen", self.node.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node.id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation

#[path = "🎮️prepare/🦀️.rs"]
pub mod preparation;
