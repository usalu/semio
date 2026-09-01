//! 🎯️ Direct active space-alternative restoration mutation.
use super::super::SpaceHistoryMutation;
use super::super::{SpaceHistoryDiff, SpaceHistorySnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestoreActiveSpaceAlternative {
    #[serde(deserialize_with = "required_option")]
    pub alternative_id: Option<String>,
}
//#endregion 🔖️Payload

//#region ⚙️Serde
fn required_option<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
}
//#endregion ⚙️Serde

//#region ⚙️Semantics
impl crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for RestoreActiveSpaceAlternative {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "restore", entity: "active-space-alternative", kind: "restore-active-space-alternative", record: "RestoredActiveSpaceAlternative" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> { crate::os_spr::MutationOutcome::new(SpaceHistoryDiff { set_active_alternative_id: Some(self.alternative_id.clone()), ..Default::default() }) }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Vec<SpaceHistoryMutation> { vec![SpaceHistoryMutation::RestoreActiveSpaceAlternative(Self { alternative_id: base.active_alternative_id.clone() })] }
    fn label(&self) -> String { "Restore active space alternative".into() }
    fn target(&self) -> Vec<String> { vec!["activeAlternativeId".into()] }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{MutationLeaf, OpBinary, OpText};

    #[test]
    fn metadata_and_explicit_nullable_wire_are_canonical() {
        assert_eq!(<RestoreActiveSpaceAlternative as MutationLeaf>::DESCRIPTOR.semantic_kind, "restore-active-space-alternative");
        assert!(<RestoreActiveSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🎯️restore-active-space-alternative"));
        let mutation = SpaceHistoryMutation::RestoreActiveSpaceAlternative(RestoreActiveSpaceAlternative { alternative_id: None });
        let json = serde_json::to_string(&mutation).expect("serialize");
        assert_eq!(json, r#"{"operation":"restoreActiveSpaceAlternative","payload":{"alternativeId":null}}"#);
        assert!(serde_json::from_str::<SpaceHistoryMutation>(r#"{"operation":"restoreActiveSpaceAlternative","payload":{}}"#).is_err());
        assert_eq!(SpaceHistoryMutation::parse_op(&json).expect("text"), mutation);
        assert_eq!(SpaceHistoryMutation::decode_op(&mutation.encode_op().expect("binary")).expect("binary decode"), mutation);
    }
}
//#endregion 🧪️Tests
