use super::super::{DummyDiff, DummyMutation, DummySnapshot};
use protocol::{MutationKind, MutationOutcome, OpBinary, OpText, ProtocolError, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetDummyCount { pub value: i32 }
impl SetDummyCount { const OPCODE: &'static str = "set-dummy-count"; const TAG: u8 = 0x61; }
impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
impl OpBinary for SetDummyCount { fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> { let mut bytes = vec![Self::TAG]; bytes.extend_from_slice(&self.value.to_be_bytes()); Ok(bytes) } fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> { if bytes.len() != 5 || bytes.first() != Some(&Self::TAG) { return Err(ProtocolError::Malformed { what: "set-dummy-count", offset: 0, detail: "expected tag 0x61 and four i32 bytes".into() }); } Ok(Self { value: i32::from_be_bytes(bytes[1..].try_into().expect("exact i32 width")) }) } }
impl MutationKind<DummySnapshot, DummyMutation> for SetDummyCount { const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dummy-count", kind: "set-dummy-count", record: "SetDummyCount" }; fn diff(&self, _: &DummySnapshot) -> MutationOutcome<DummyDiff> { MutationOutcome::new(DummyDiff { count: Some(self.value) }) } fn inverse(&self, base: &DummySnapshot) -> Vec<DummyMutation> { vec![Self { value: base.count }.into()] } fn label(&self) -> String { format!("Set dummy count to {}", self.value) } }
