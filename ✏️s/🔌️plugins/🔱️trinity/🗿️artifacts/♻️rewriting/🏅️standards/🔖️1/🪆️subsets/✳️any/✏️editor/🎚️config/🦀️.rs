//! 🧮️ Trinity Rewriting app — view-state config + config operations.

use crate::artifacts::jack::Camera;
use std::collections::BTreeMap;

/// 🧮️ Rewriting's `ArtifactApp::Config` — node selection, the Before pane's live viewport camera
/// (seeded once from the initial before-fixture's seed-only `camera` field, then only ever written by
/// `nodeGraphViewport`), the reorganize epoch, the hover/select var focus + their epochs, the
/// per-window LOD mode, and the BCP-47 locale tag.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.rewritingcfg")]
#[dsl(layout = "lines")]
pub struct RewritingConfig {
    #[dsl(block)]
    pub before_pane_camera: Camera,
    pub reorganize_epoch: u64,
    pub lod_mode_by_window: BTreeMap<String, String>,
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for RewritingConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RewritingConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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

//#endregion 🔖️ArtifactCodec

impl Default for RewritingConfig {
    fn default() -> Self {
        Self { before_pane_camera: Camera::default(), reorganize_epoch: 0, lod_mode_by_window: BTreeMap::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(RewritingConfig);

/// @emoji 🧮️ Rewriting's `RewritingConfig` operation enum — one variant per settled interaction, plus a
/// generic `Snapshot` every variant's `backwards()` returns. See `JackConfigMutation`'s doc comment
/// for why `Snapshot`'s size is allowed rather than boxed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
#[allow(clippy::large_enum_variant)]
pub enum RewritingConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RewritingConfig,
    },
    #[dsl(key = "before-pane-camera")]
    SetBeforePaneCamera {
        #[dsl(block)]
        camera: Camera,
    },
    #[dsl(key = "reorganize-epoch")]
    SetReorganizeEpoch { value: u64 },
    #[dsl(key = "lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for RewritingConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for RewritingConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

impl protocol::Mutation<RewritingConfig> for RewritingConfigMutation {
    type Diff = RewritingConfig;

    fn diff(&self, base: &RewritingConfig) -> protocol::MutationOutcome<RewritingConfig> {
        let mut next = base.clone();
        match self {
            RewritingConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            RewritingConfigMutation::SetBeforePaneCamera { camera } => next.before_pane_camera = camera.clone(),
            RewritingConfigMutation::SetReorganizeEpoch { value } => next.reorganize_epoch = *value,
            RewritingConfigMutation::SetLodMode { window_id, value } => {
                next.lod_mode_by_window.insert(window_id.clone(), value.clone());
            }
            RewritingConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &RewritingConfig) -> Vec<Self> {
        vec![RewritingConfigMutation::Snapshot { config: base.clone() }]
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    #[semio_framework_async_macros::async_test]
    async fn rewriting_config_default_has_default_locale() {
        let config = RewritingConfig::default();
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.before_pane_camera, Camera::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn rewriting_config_dsl_round_trips() {
        let mut config = RewritingConfig { reorganize_epoch: 3, ..RewritingConfig::default() };
        config.lod_mode_by_window.insert("trinity-rewriting-before".into(), "compact".into());
        ::store::os_store::test_support::assert_dsl_round_trip(&config);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn rewriting_config_operation_backwards_restores_prior_snapshot() {
        let base = RewritingConfig::default();
        let operation = RewritingConfigMutation::SetReorganizeEpoch { value: 7 };
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.reorganize_epoch, 7);
        let backwards = operation.inverse(&base);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn rewriting_config_operation_text_round_trips() {
        ::store::os_store::test_support::assert_op_line_round_trip(&RewritingConfigMutation::SetLodMode { window_id: "trinity-rewriting-before".into(), value: "compact".into() });
        ::store::os_store::test_support::assert_op_line_round_trip(&RewritingConfigMutation::SetReorganizeEpoch { value: 4 });
    }
}
//#endregion 🧪️Tests
