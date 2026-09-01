//! 🫧️ Real local-ephemeral publication snapshot fixture.

use crate::store::{ArtifactDsl, ArtifactPack, PackDecodeOptions, PackEncodeOptions, PackError, TextError, TextSpan};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicationTransient {
    pub revision: u64,
}

impl ArtifactDsl for PublicationTransient {
    const EXTENSION: &'static str = "publication-transient";

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        serde_json::from_str(text).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string(self).expect("publication transient serializes")
    }
}

impl ArtifactPack for PublicationTransient {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        serde_json::to_vec(self).map_err(|error| PackError::Schema(error.to_string()))
    }

    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        serde_json::from_slice(bytes).map_err(|error| PackError::Schema(error.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicationTransientDiff {
    pub revision: Option<u64>,
}

impl protocol::MutationDiff<PublicationTransient> for PublicationTransientDiff {
    fn apply(&self, base: &PublicationTransient) -> protocol::MutationApplyResult<PublicationTransient> {
        Ok(PublicationTransient { revision: self.revision.unwrap_or(base.revision) })
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
