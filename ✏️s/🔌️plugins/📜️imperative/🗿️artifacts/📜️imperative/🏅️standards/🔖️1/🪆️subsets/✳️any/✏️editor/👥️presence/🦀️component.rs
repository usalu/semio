//! 👥️ Imperative presence — shareable live ephemeral state + mutations.
//!
//! Empty: imperative has no multi-user shareable live state of its own; step selection is now the
//! framework-owned `steps` interaction domain, broadcast via its own typed `PresencePeer.interaction`
//! (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of imperative view state (none yet — step selection lives in the `steps`
/// interaction domain).
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativePresence {}

impl store::ArtifactDsl for ImperativePresence {
    const EXTENSION: &'static str = "imperativepres";
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(store::TextError::new("no imperative presence", store::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for ImperativePresence {
    async fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    async fn decode_pack_with(_bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        Ok(Self::default())
    }
}

impl protocol::MutationDiff<ImperativePresence> for ImperativePresence {
    async fn apply(&self, base: &ImperativePresence) -> protocol::MutationApplyResult<ImperativePresence> {
        Ok({ base.clone() })
    }
    async fn absorb(&mut self, _other: Self) {}
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum ImperativePresenceMutation {
    Noop,
}

impl Mutation<ImperativePresence> for ImperativePresenceMutation {
    type Diff = ImperativePresence;

    async fn diff(&self, _base: &ImperativePresence) -> protocol::MutationOutcome<ImperativePresence> {
        protocol::MutationOutcome::new(ImperativePresence::default())
    }

    async fn inverse(&self, _base: &ImperativePresence) -> Vec<Self> {
        vec![ImperativePresenceMutation::Noop]
    }
}

impl protocol::OpText for ImperativePresenceMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for ImperativePresenceMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
