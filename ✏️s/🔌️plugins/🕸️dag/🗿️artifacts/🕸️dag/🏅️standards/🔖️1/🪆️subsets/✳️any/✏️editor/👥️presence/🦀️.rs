//! 👥️ DAG play presence — shareable live ephemeral state + mutations.

use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live node-graph view state — viewport camera only; peer selection/hover now ride the
/// framework's own typed `PresenceInteraction` for the `graph` domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), not this app-opaque facet.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "dag.presence")]
#[dsl(layout = "lines")]
pub struct DagPresence {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
}

impl store::ArtifactPresenceSnapshot for DagPresence {}

impl Default for DagPresence {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0 }
    }
}

/// 🔺️ Sparse field delta over [`DagPresence`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct DagPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_x: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_y: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_zoom: Option<f64>,
}

impl protocol::MutationDiff<DagPresence> for DagPresenceDiff {
    fn apply(&self, base: &DagPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<DagPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_x {
            next.camera_x = value.clone();
        }
        if let Some(value) = &self.camera_y {
            next.camera_y = value.clone();
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera_x.is_some() {
            self.camera_x = other.camera_x;
        }
        if other.camera_y.is_some() {
            self.camera_y = other.camera_y;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
    }
}

impl protocol::DiffAlgebra<DagPresence> for DagPresenceDiff {
    fn inverse(&self, base: &DagPresence) -> Self {
        Self {
            camera_x: self.camera_x.as_ref().map(|_| base.camera_x.clone()),
            camera_y: self.camera_y.as_ref().map(|_| base.camera_y.clone()),
            camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_zoom.is_none()
    }
}

impl store::ArtifactDsl for DagPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DagPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-vectors/🦀️.rs"]
mod mutation_vectors;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
