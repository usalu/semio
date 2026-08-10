//! 👥️ Home presence — shareable live ephemeral state + mutations.
//!
//! Empty: the home launcher keeps panel tab and locale in [`crate::apps::home::config::HomeConfig`];
//! there is no multi-user shareable live surface on the launcher.

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ No shareable live launcher state — chrome stays in `HomeConfig`.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomePresence {}

impl protocol::MutationDiff<HomePresence> for HomePresence {
    fn apply(&self, base: &HomePresence) -> HomePresence {
        base.clone()
    }
    fn absorb(&mut self, _other: Self) {}
}

impl store::ArtifactDsl for HomePresence {
    const EXTENSION: &'static str = "home.presence";
    fn envelope_id() -> &'static str {
        "home.presence"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(store::TextError::new("home presence", store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for HomePresence {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        if !inner.is_empty() {
            return Err(store::PackError::Schema("home presence pack must be empty".into()));
        }
        Ok(Self::default())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum HomePresenceMutation {
    Noop,
}

impl Mutation<HomePresence> for HomePresenceMutation {
    type Diff = HomePresence;

    fn diff(&self, _base: &HomePresence) -> HomePresence {
        HomePresence::default()
    }

    fn inverse(&self, _base: &HomePresence) -> Vec<Self> {
        vec![HomePresenceMutation::Noop]
    }
}

impl protocol::OpText for HomePresenceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() {
                    line[keyword.len()..].trim_start()
                } else {
                    ""
                };
                let record = dsl::parse(
                    body,
                    &spec_fn(),
                    &dsl::ParseOptions {
                        limits: dsl::Limits::default(),
                        mode: dsl::SourceMode::Inline,
                    },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants
            .iter()
            .find(|(k, _)| k == &keyword)
            .map(|(_, s)| *s)
            .expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for HomePresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
