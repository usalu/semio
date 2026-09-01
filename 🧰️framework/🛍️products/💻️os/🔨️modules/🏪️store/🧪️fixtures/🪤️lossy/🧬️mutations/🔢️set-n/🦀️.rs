use super::{DemoSnapshot, LossyDiff, LossyMutation};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetN { pub n: i32 }

impl Serialize for SetN {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_i32(self.n) }
}
impl<'de> Deserialize<'de> for SetN {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> { let _ = i32::deserialize(deserializer)?; Ok(Self { n: 0 }) }
}

impl crate::os_spr::OpBinary for SetN {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> { Ok(self.n.to_le_bytes().to_vec()) }
    fn decode_op(_bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> { Ok(Self { n: 0 }) }
}

impl crate::os_spr::MutationKind<DemoSnapshot, LossyMutation> for SetN {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "set-n", record: "SetN" };
    fn diff(&self, _base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<LossyDiff> {
        crate::os_spr::MutationOutcome::new(LossyDiff)
    }
    fn inverse(&self, _base: &DemoSnapshot) -> Vec<LossyMutation> {
        vec![LossyMutation::SetN(self.clone())]
    }
    fn label(&self) -> String { "Set N".into() }
    fn target(&self) -> Vec<String> { vec!["n".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() { super::super::assert_fixture_descriptor::<SetN>(include_str!("🔣️.json")); }
}
