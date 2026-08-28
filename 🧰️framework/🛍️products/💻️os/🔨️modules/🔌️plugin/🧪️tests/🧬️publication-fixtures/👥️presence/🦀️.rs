//! 👥️ Real shared-ephemeral publication snapshot fixture.

use crate::store::{ArtifactDsl, ArtifactPack, PackDecodeOptions, PackEncodeOptions, PackError, TextError, TextSpan};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicationPresence {
    pub revision: u64,
}

impl ArtifactDsl for PublicationPresence {
    const EXTENSION: &'static str = "publication-presence";

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        serde_json::from_str(text).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string(self).expect("publication presence serializes")
    }
}

impl ArtifactPack for PublicationPresence {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        serde_json::to_vec(self).map_err(|error| PackError::Schema(error.to_string()))
    }

    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        serde_json::from_slice(bytes).map_err(|error| PackError::Schema(error.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicationPresenceDiff {
    pub revision: Option<u64>,
}

impl protocol::MutationDiff<PublicationPresence> for PublicationPresenceDiff {
    fn apply(&self, base: &PublicationPresence) -> protocol::MutationApplyResult<PublicationPresence> {
        Ok(PublicationPresence { revision: self.revision.unwrap_or(base.revision) })
    }

    fn absorb(&mut self, other: Self) {
        if other.revision.is_some() {
            self.revision = other.revision;
        }
    }
}

#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;
