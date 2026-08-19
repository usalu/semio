//! 🧮️ Trinity Rewrite app — view-state config + config operations.

use crate::artifacts::jack::Camera;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 🧮️ Rewrite's `ArtifactApp::Config` — node selection, the Before pane's live viewport camera
/// (seeded once from the initial before-fixture's seed-only `camera` field, then only ever written by
/// `nodeGraphViewport`), the reorganize epoch, the hover/select var focus + their epochs, the
/// per-window LOD mode, and the BCP-47 locale tag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.rewritecfg")]
#[dsl(layout = "lines")]
pub struct RewriteConfig {
    #[dsl(block)]
    pub before_pane_camera: Camera,
    pub reorganize_epoch: u64,
    pub lod_mode_by_window: BTreeMap<String, String>,
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for RewriteConfig {
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
impl store::ArtifactPack for RewriteConfig {
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


impl Default for RewriteConfig {
    fn default() -> Self {
        Self {
            before_pane_camera: Camera::default(),
            reorganize_epoch: 0,
            lod_mode_by_window: BTreeMap::new(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(RewriteConfig);

/// @emoji 🧮️ Rewrite's `RewriteConfig` operation enum — one variant per settled interaction, plus a
/// generic `Snapshot` every variant's `backwards()` returns. See `JackConfigMutation`'s doc comment
/// for why `Snapshot`'s size is allowed rather than boxed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[allow(clippy::large_enum_variant)]
pub enum RewriteConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RewriteConfig,
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
impl protocol::OpText for RewriteConfigMutation {
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
impl protocol::OpBinary for RewriteConfigMutation {
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


impl protocol::Mutation<RewriteConfig> for RewriteConfigMutation {
    type Diff = RewriteConfig;

    async fn diff(&self, base: &RewriteConfig) -> protocol::MutationOutcome<RewriteConfig> {
        let mut next = base.clone();
        match self {
            RewriteConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            RewriteConfigMutation::SetBeforePaneCamera { camera } => next.before_pane_camera = camera.clone(),
            RewriteConfigMutation::SetReorganizeEpoch { value } => next.reorganize_epoch = *value,
            RewriteConfigMutation::SetLodMode { window_id, value } => {
                next.lod_mode_by_window.insert(window_id.clone(), value.clone());
            }
            RewriteConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &RewriteConfig) -> Vec<Self> {
        vec![RewriteConfigMutation::Snapshot { config: base.clone() }]
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use protocol::Mutation;
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn rewrite_config_default_has_default_locale() {
        let config = RewriteConfig::default();
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.before_pane_camera, Camera::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn rewrite_config_dsl_round_trips() {
        let mut config = RewriteConfig { reorganize_epoch: 3, ..RewriteConfig::default() };
        config.lod_mode_by_window.insert("trinity-rewrite-before".into(), "compact".into());
        ::store::os_store::test_support::assert_dsl_round_trip(&config);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn rewrite_config_operation_backwards_restores_prior_snapshot() {
        let base = RewriteConfig::default();
        let operation = RewriteConfigMutation::SetReorganizeEpoch { value: 7 };
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.reorganize_epoch, 7);
        let backwards = operation.inverse(&base);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn rewrite_config_operation_text_round_trips() {
        ::store::os_store::test_support::assert_op_line_round_trip(&RewriteConfigMutation::SetLodMode { window_id: "trinity-rewrite-before".into(), value: "compact".into() });
        ::store::os_store::test_support::assert_op_line_round_trip(&RewriteConfigMutation::SetReorganizeEpoch { value: 4 });
    }
}
//#endregion 🧪️Tests
