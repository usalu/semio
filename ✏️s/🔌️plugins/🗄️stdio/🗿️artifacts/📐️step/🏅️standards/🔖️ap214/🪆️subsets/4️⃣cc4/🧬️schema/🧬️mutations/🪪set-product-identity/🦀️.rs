//! 🪪️ `set-product-identity` — one axis of this conformance class, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file names the axis and
//! routes to it, so each rule has ONE implementation and every class calls it.

use crate::StepSnapshot;
use crate::standards::v_ap214::engine::ladder::ProductIdentity;
use crate::standards::v_ap214::engine::ladder::ClassEdit;
use crate::standards::v_ap214::subsets::cc4::schema::mutations::{class_diff, class_inverse};
use crate::standards::v_ap214::subsets::cc4::schema::mutations::{StepCc4Mutation};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetProductIdentity {
    pub identity: Option<ProductIdentity>,
}

impl protocol::MutationKind<StepSnapshot, StepCc4Mutation> for SetProductIdentity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "product-identity", kind: "set-product-identity", record: "SetProductIdentity" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepCc4Mutation as protocol::Mutation<StepSnapshot>>::Diff> {
        class_diff(base, &ClassEdit::ProductIdentity { identity: self.identity.clone() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepCc4Mutation> {
        class_inverse(base, &ClassEdit::ProductIdentity { identity: self.identity.clone() })
    }
    fn label(&self) -> String {
        "Set the PRODUCT identity chain".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
