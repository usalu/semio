//#region ➕️ContributedWireAddValue
//! ➕️ Direct value addition for contributed mutation wire planning.

use super::super::{WireTestDiff, WireTestMutation, WireTestSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

// 🌱️ `Serialize`/`Deserialize` stay for `MutationKind`'s own (untouched) supertrait bound below;
// `ToValue`/`FromValue` are the newer `CompositeMutationKind` supertrait bound (see that trait's
// own doc) — this fixture implements both traits, so both derive pairs coexist here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct AddValue {
    pub(crate) delta: i32,
}

impl protocol::MutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };

    fn diff(&self, _base: &WireTestSnapshot) -> protocol::MutationOutcome<WireTestDiff> {
        protocol::MutationOutcome::new(WireTestDiff { deltas: vec![self.delta] })
    }

    fn inverse(&self, _base: &WireTestSnapshot) -> Vec<WireTestMutation> {
        if self.delta == i32::MIN {
            vec![WireTestMutation::AddValue(Self { delta: 1 }), WireTestMutation::AddValue(Self { delta: i32::MAX })]
        } else {
            vec![WireTestMutation::AddValue(Self { delta: -self.delta })]
        }
    }

    fn label(&self) -> String {
        format!("Add {} to value", self.delta)
    }

    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

impl protocol::CompositeMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
    const SEMANTICS: protocol::SemanticDescriptor = <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::SEMANTICS;

    fn plan(&self, _base: &WireTestSnapshot, planner: &mut protocol::Planner<WireTestSnapshot, WireTestMutation>) -> Result<(), protocol::PlanError> {
        planner.call(WireTestMutation::AddValue(self.clone()))
    }

    fn label(&self) -> String {
        <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::label(self)
    }

    fn target(&self) -> Vec<String> {
        <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::target(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation,MutationDiff,MutationLeaf};

    #[test]
    fn direct_leaf_descriptor_and_inverse_law() {
        assert!(AddValue::DESCRIPTOR.validate().is_ok());
        assert_eq!(AddValue::DESCRIPTOR.semantic_kind,"add-value");
        let provenance=AddValue::PROVENANCE;
        assert_eq!(provenance.owner,AddValue::DESCRIPTOR.owner);
        let scope=protocol::MutationLeafSourceScope{workspace_token:provenance.workspace_token,mutation_root:provenance.mutation_root,taxonomy_path:provenance.taxonomy_path,source_filename:"🦀️.rs",descriptor_filename:"🔣️.json"};
        assert!(protocol::validate_mutation_leaf_source(&AddValue::DESCRIPTOR,&provenance,&scope).is_ok());
        let base=WireTestSnapshot{value:0};
        let mutation=WireTestMutation::AddValue(AddValue{delta:i32::MIN});
        let current=mutation.diff(&base).diff().apply(&base).expect("minimum applies");
        let inverse=mutation.inverse(&base);
        assert_eq!(inverse,vec![WireTestMutation::AddValue(AddValue{delta:1}),WireTestMutation::AddValue(AddValue{delta:i32::MAX})]);
        let restored=inverse.iter().rev().try_fold(current,|snapshot,next|next.diff(&snapshot).diff().apply(&snapshot)).expect("stored reverse inverse");
        assert_eq!(restored,base);
    }
}
//#endregion ➕️ContributedWireAddValue
