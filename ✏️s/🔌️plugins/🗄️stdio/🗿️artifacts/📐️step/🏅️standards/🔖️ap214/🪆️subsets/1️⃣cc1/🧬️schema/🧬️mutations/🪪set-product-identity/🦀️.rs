//! 🪪 `set-product-identity` -- replaces every product identity chain rung with the given three, or removes the chain; the base chain is restored rung by rung at its exact positions.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::engine::ladder;
use crate::standards::v_ap214::engine::ladder::ProductIdentity;
use crate::standards::v_ap214::subsets::cc1::schema::mutations::{rejected, restored, StepCc1Mutation, CLASS};
use crate::StepSnapshot;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetProductIdentity {
    pub identity: Option<ProductIdentity>,
}

impl protocol::MutationKind<StepSnapshot, StepCc1Mutation> for SetProductIdentity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "product-identity", kind: "set-product-identity", record: "SetProductIdentity" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        protocol::MutationOutcome::new(ladder::product_identity_diff(base, self.identity.as_ref()))
    }

    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepCc1Mutation>, semio_framework_value::ValueError> {
        Ok(restored(ladder::chain_restore_rows(base, self.identity.as_ref())))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set the PRODUCT identity chain", "Produktidentitätskette setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
