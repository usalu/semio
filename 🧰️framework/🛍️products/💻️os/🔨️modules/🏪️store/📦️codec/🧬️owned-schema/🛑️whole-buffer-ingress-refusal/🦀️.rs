//! 🛑️ Whole buffer ingress refusal: the whole buffer artifact envelope ingress error and its rejection. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeWholeBufferIngressError;

impl std::fmt::Display for ArtifactEnvelopeWholeBufferIngressError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("artifact envelope JSON must be submitted as fixed pages to the app-owned persistent decoder")
    }
}

impl std::error::Error for ArtifactEnvelopeWholeBufferIngressError {}

/// 🚫️ Greenfield fail-closure for deleted whole-buffer envelope ingress; callers must use the app-owned paged submission handle.
pub fn reject_whole_buffer_artifact_envelope_ingress<P, Mutation>(_input: &str) -> Result<ArtifactEnvelope<P, Mutation>, ArtifactEnvelopeWholeBufferIngressError> {
    Err(ArtifactEnvelopeWholeBufferIngressError)
}
