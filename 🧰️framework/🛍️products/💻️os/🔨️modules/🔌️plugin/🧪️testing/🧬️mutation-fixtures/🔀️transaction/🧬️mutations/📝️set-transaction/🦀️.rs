use super::super::{TxnDiff, TxnMutation, TxnSnapshot};
use protocol::{MutationKind, MutationOutcome, OpBinary, OpText, ProtocolError, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetTransactionCount {
    pub value: i32,
}
impl SetTransactionCount {
    const OPCODE: &'static str = "set-transaction-count";
    const TAG: u8 = 0x62;
}
impl OpText for SetTransactionCount {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(Self {
            value: line
                .strip_prefix("set-transaction-count ")
                .ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected set-transaction-count", semio_framework_diagnostic::TextSpan::at(1, 1)))?
                .parse()
                .map_err(|_| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "transaction count must be i32", semio_framework_diagnostic::TextSpan::at(1, 1)))?,
        })
    }
    fn print_op(&self) -> String {
        format!("{} {}", Self::OPCODE, self.value)
    }
}
impl OpBinary for SetTransactionCount {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = vec![Self::TAG];
        bytes.extend_from_slice(&self.value.to_be_bytes());
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() != 5 || bytes.first() != Some(&Self::TAG) {
            return Err(ProtocolError::Malformed { what: "set-transaction-count", offset: 0, detail: "expected tag 0x62 and four i32 bytes".into() });
        }
        Ok(Self { value: i32::from_be_bytes(bytes[1..].try_into().expect("exact i32 width")) })
    }
}
impl MutationKind<TxnSnapshot, TxnMutation> for SetTransactionCount {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "transaction-count", kind: "set-transaction-count", record: "SetTransactionCount" };
    fn diff(&self, _: &TxnSnapshot) -> MutationOutcome<TxnDiff> {
        MutationOutcome::new(TxnDiff { count: Some(self.value) })
    }
    fn inverse(&self, base: &TxnSnapshot) -> Result<Vec<TxnMutation>, semio_framework_value::ValueError> {
        Ok((|| vec![Self { value: base.count }.into()])())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set transaction count to {}", self.value), &format!("Transaktionsanzahl auf {} setzen", self.value))
    }
}
