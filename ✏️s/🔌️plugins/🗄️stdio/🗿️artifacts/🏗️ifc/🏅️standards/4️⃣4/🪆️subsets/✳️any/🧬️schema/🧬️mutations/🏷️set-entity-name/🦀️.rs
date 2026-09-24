//! 🏷️ `set-entity-name` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntityName {
    pub id: u64,
    pub name: String,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for SetEntityName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-name", kind: "set-entity-name", record: "SetEntityName" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<<IfcMutation as Mutation<IfcSnapshot>>::Diff> {
        agg_diff(&IfcMutation::SetEntityName(self.clone()), base)
    }
    fn inverse(&self, base: &IfcSnapshot) -> Vec<IfcMutation> {
        agg_inverse(&IfcMutation::SetEntityName(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Set entity name", "Entitätsname setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
