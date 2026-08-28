//! 📝️ Direct publication-transient replacement payload and inverse behavior.

use super::super::{PublicationTransient, PublicationTransientMutation};
use protocol::{MutationKind, MutationOutcome, OpBinary, OpText, ProtocolError, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangePublicationTransient {
    pub revision: u64,
}

impl ChangePublicationTransient {
    pub const TEXT_OPCODE: &'static str = "change-publication-transient";
    pub const BINARY_TAG: u8 = 0x52;

    fn parse_revision(line: &str) -> Result<u64, crate::store::TextError> {
        let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unknown publication transient op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
        if revision.is_empty() || !revision.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(crate::store::TextError::new("publication transient revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
        }
        revision.parse().map_err(|_| crate::store::TextError::new("publication transient revision is outside u64", crate::store::TextSpan::at(1, 1)))
    }
}

impl OpText for ChangePublicationTransient {
    fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
        Ok(Self { revision: Self::parse_revision(line)? })
    }

    fn print_op(&self) -> String {
        format!("{} {}", Self::TEXT_OPCODE, self.revision)
    }
}

impl OpBinary for ChangePublicationTransient {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::with_capacity(9);
        bytes.push(Self::BINARY_TAG);
        bytes.extend_from_slice(&self.revision.to_be_bytes());
        Ok(bytes)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() != 9 || bytes.first() != Some(&Self::BINARY_TAG) {
            return Err(ProtocolError::Malformed { what: "change-publication-transient", offset: 0, detail: "expected tag 0x52 and eight revision bytes".into() });
        }
        Ok(Self { revision: u64::from_be_bytes(bytes[1..].try_into().expect("exact binary revision width")) })
    }
}

impl MutationKind<PublicationTransient, PublicationTransientMutation> for ChangePublicationTransient {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "publication-transient", kind: "change-publication-transient", record: "ChangedPublicationTransient" };

    fn diff(&self, _base: &PublicationTransient) -> MutationOutcome<super::super::PublicationTransientDiff> {
        MutationOutcome::new(super::super::PublicationTransientDiff { revision: Some(self.revision) })
    }

    fn inverse(&self, base: &PublicationTransient) -> Vec<PublicationTransientMutation> {
        vec![Self { revision: base.revision }.into()]
    }

    fn label(&self) -> String {
        format!("Change publication transient to {}", self.revision)
    }
}
