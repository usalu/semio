//! 👥️ Jack presence — shareable live ephemeral state + mutations.

use crate::artifacts::jack::Camera;
use std::collections::BTreeMap;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of jack view state (fixture, query draft, viewport camera, LOD) — node
/// selection broadcasts automatically via the framework's typed `PresenceInteraction` (ticket
/// `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`), no longer mirrored here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.jack.presence")]
#[dsl(layout = "lines")]
#[derive(Default)]
pub struct JackPresence {
    pub active_fixture_id: String,
    pub jack_query: String,
    #[dsl(block)]
    pub camera: Camera,
    pub lod_mode_by_window: BTreeMap<String, String>,
}


impl protocol::MutationDiff<JackPresence> for JackPresence {
    fn apply(&self, _base: &JackPresence) -> protocol::MutationApplyResult<JackPresence> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for JackPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for JackPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence


#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn configuration_and_presence_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).unwrap();
        let base: JackPresence = pack::from_json_str(&vectors["base"].to_string()).unwrap();
        assert_eq!(<JackPresenceMutation as Mutation<JackPresence>>::DESCRIPTORS.len(), vectors["cases"].as_array().unwrap().len());
        for vector in vectors["cases"].as_array().unwrap() {
            let mutation: JackPresenceMutation = pack::from_json_str(&vector["mutation"].to_string()).unwrap();
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).unwrap(), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
            let next = mutation.diff(&base).diff().apply(&base).unwrap();
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).unwrap(), vector["expected"]);
            assert_eq!(JackPresenceMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            assert_eq!(JackPresenceMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
            assert_eq!(restored, base);
        }
    }
}
