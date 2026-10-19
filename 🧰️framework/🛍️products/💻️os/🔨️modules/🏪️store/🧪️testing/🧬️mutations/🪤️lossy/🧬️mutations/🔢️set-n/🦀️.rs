use super::{DemoSnapshot, LossyDiff, LossyMutation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, semio_framework_os_dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetN {
    pub n: i32,
}

impl protocol::ToValue for SetN {
    fn to_value(&self) -> protocol::DslValue {
        protocol::ToValue::to_value(&self.n)
    }
}

impl protocol::FromValue for SetN {
    fn from_value(value: protocol::DslValue) -> Result<Self, protocol::ValueError> {
        let _ = <i32 as protocol::FromValue>::from_value(value)?;
        Ok(Self { n: 0 })
    }
}

impl Serialize for SetN {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(self.n)
    }
}
impl<'de> Deserialize<'de> for SetN {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _ = i32::deserialize(deserializer)?;
        Ok(Self { n: 0 })
    }
}

impl semio_framework_os_spr::OpBinary for SetN {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        Ok(self.n.to_le_bytes().to_vec())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        Ok(Self { n: 0 })
    }
}

impl semio_framework_os_spr::MutationKind<DemoSnapshot, LossyMutation> for SetN {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "set", entity: "n", kind: "set-n", record: "SetN" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set N", "N setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["n".into()]
    }
}

impl semio_framework_os_spr::ColdMutationKind<DemoSnapshot, LossyMutation> for SetN {
    fn diff(&self, _base: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<LossyDiff> {
        semio_framework_os_spr::MutationOutcome::new(LossyDiff {})
    }
    fn inverse(&self, _base: &DemoSnapshot) -> Result<Vec<LossyMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![LossyMutation::SetN(self.clone())]
    
    })())
}
}
