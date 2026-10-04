//! 🔎️ TrinityGraph mutation — `SetQuery`: replaces the document's Jack query text.
use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::JackSnapshot;

//#region 🔖️Mutation
/// 🔎️ `set-query` payload — the whole query text the document holds afterwards.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetQuery {
    pub value: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_query(value: String) -> TrinityGraphMutation {
    TrinityGraphMutation::SetQuery(SetQuery { value })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for SetQuery {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "query", kind: "set-query", record: "SetQuery" };

    fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &JackSnapshot) -> Result<Vec<TrinityGraphMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit Jack query", "Jack-Abfrage bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        vec!["query".to_string()]
    }
}
//#endregion 🔖️Mutation
