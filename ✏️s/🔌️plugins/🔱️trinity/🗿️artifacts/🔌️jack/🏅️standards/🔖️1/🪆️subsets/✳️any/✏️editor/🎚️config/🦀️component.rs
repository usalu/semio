//! 🧮️ Trinity Jack app — view-state config + config operations.

use crate::artifacts::jack::Camera;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 🎯️ Ephemeral editor selection range (offsets into the jack query text).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct JackEditorSelection {
    pub start: u64,
    pub end: u64,
}

/// 🧮️ Jack's `ArtifactApp::Config` — node selection, the live node-graph viewport camera (seeded once
/// from the initial fixture's seed-only `camera` field, then only ever written by
/// `nodeGraphViewport`), the active fixture/example id, the jack query draft + its last result, the
/// three engagement-input drafts, the reorganize epoch, the editor's text selection, the per-window
/// LOD mode, a completion-request revision counter, and the BCP-47 locale tag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.jackcfg")]
#[dsl(layout = "lines")]
pub struct JackConfig {
    #[dsl(block)]
    pub camera: Camera,
    pub active_fixture_id: String,
    pub jack_query: String,
    pub jack_result_json: String,
    pub editor_engagement_input: String,
    pub graph_engagement_input: String,
    pub results_engagement_input: String,
    pub reorganize_epoch: u64,
    #[dsl(block)]
    pub editor_selection: Option<JackEditorSelection>,
    pub lod_mode_by_window: BTreeMap<String, String>,
    pub revision: u64,
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for JackConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for JackConfig {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


impl Default for JackConfig {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            active_fixture_id: String::new(),
            jack_query: String::new(),
            jack_result_json: String::new(),
            editor_engagement_input: String::new(),
            graph_engagement_input: String::new(),
            results_engagement_input: String::new(),
            reorganize_epoch: 0,
            editor_selection: None,
            lod_mode_by_window: BTreeMap::new(),
            revision: 0,
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(JackConfig);

/// @emoji 🧮️ Jack's `JackConfig` operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns. `Snapshot`'s whole-`JackConfig` payload is
/// inherent to the "backwards restores a full prior snapshot" design (mirrors `RewriteConfigMutation`
/// and `shooting_op::ShootingConfigMutation`) — boxing it would perturb the `#[dsl(block)]` wire
/// shape for no behavioral gain, so the size lint is silenced instead of restructuring the type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[allow(clippy::large_enum_variant)]
pub enum JackConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: JackConfig,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: Camera,
    },
    #[dsl(key = "active-fixture")]
    SetActiveFixture { value: String },
    #[dsl(key = "query")]
    SetQuery { value: String },
    #[dsl(key = "result")]
    SetResult { value: String },
    #[dsl(key = "editor-input")]
    SetEditorEngagementInput { value: String },
    #[dsl(key = "graph-input")]
    SetGraphEngagementInput { value: String },
    #[dsl(key = "results-input")]
    SetResultsEngagementInput { value: String },
    #[dsl(key = "reorganize-epoch")]
    SetReorganizeEpoch { value: u64 },
    #[dsl(key = "editor-selection")]
    SetEditorSelection {
        #[dsl(block)]
        selection: Option<JackEditorSelection>,
    },
    #[dsl(key = "lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "revision")]
    SetRevision { value: u64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for JackConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for JackConfigMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


impl protocol::Mutation<JackConfig> for JackConfigMutation {
    type Diff = JackConfig;

    async fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        match self {
            JackConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            JackConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            JackConfigMutation::SetActiveFixture { value } => next.active_fixture_id = value.clone(),
            JackConfigMutation::SetQuery { value } => next.jack_query = value.clone(),
            JackConfigMutation::SetResult { value } => next.jack_result_json = value.clone(),
            JackConfigMutation::SetEditorEngagementInput { value } => next.editor_engagement_input = value.clone(),
            JackConfigMutation::SetGraphEngagementInput { value } => next.graph_engagement_input = value.clone(),
            JackConfigMutation::SetResultsEngagementInput { value } => next.results_engagement_input = value.clone(),
            JackConfigMutation::SetReorganizeEpoch { value } => next.reorganize_epoch = *value,
            JackConfigMutation::SetEditorSelection { selection } => next.editor_selection = selection.clone(),
            JackConfigMutation::SetLodMode { window_id, value } => {
                next.lod_mode_by_window.insert(window_id.clone(), value.clone());
            }
            JackConfigMutation::SetRevision { value } => next.revision = *value,
            JackConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &JackConfig) -> Vec<Self> {
        vec![JackConfigMutation::Snapshot { config: base.clone() }]
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use protocol::Mutation;
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn jack_config_default_has_default_locale() {
        let config = JackConfig::default();
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.camera, Camera::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_config_dsl_round_trips() {
        let mut config = JackConfig {
            jack_query: "MATCH (a:Piece) RETURN a".into(),
            editor_selection: Some(JackEditorSelection { start: 3, end: 9 }),
            ..JackConfig::default()
        };
        config.lod_mode_by_window.insert("trinity-jack-graph".into(), "compact".into());
        ::store::os_store::test_support::assert_dsl_round_trip(&config);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_config_operation_backwards_restores_prior_snapshot() {
        let base = JackConfig::default();
        let operation = JackConfigMutation::SetActiveFixture { value: "nakagin".into() };
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.active_fixture_id, "nakagin".to_string());
        let backwards = operation.inverse(&base);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_config_operation_text_round_trips() {
        ::store::os_store::test_support::assert_op_line_round_trip(&JackConfigMutation::SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() });
        ::store::os_store::test_support::assert_op_line_round_trip(&JackConfigMutation::SetActiveFixture { value: "nakagin".into() });
    }
}
//#endregion 🧪️Tests
