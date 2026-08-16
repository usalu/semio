//! 🧮️ Imperative play app — view state (`ImperativeConfig`) and its operation enum
//! (`ImperativeConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.imperative` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so run-output/locale edits are VCS'd exactly like document
//! content — absorbing the former app-struct `RefCell` (`ImperativePlayRuntime`'s `run_output_json`) plus
//! the locale the UI used to read off the deleted `ViewModel`. Step selection is no longer here: it is the
//! framework-owned `steps` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "imperative.config")]
#[dsl(id = "imperative.config")]
#[dsl(layout = "lines")]
pub struct ImperativeConfig {
    /// 📤️ Last `run` output, JSON-encoded scope — was `ImperativePlayRuntime::run_output_json`.
    pub run_output_json: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `imperative.module` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for ImperativeConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
    fn print_dsl(&self) -> String {
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
impl store::ArtifactPack for ImperativeConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


fn default_contributions_json() -> String {
    crate::artifacts::imperative::io::default_imperative_contributions_json()
}

impl Default for ImperativeConfig {
    fn default() -> Self {
        Self { run_output_json: String::new(), locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

store::impl_whole_record_config!(ImperativeConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// @emoji 🧮️ `ImperativeConfig`'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns — mirrors `shooting_op::ShootingConfigMutation`'s
/// "undo this tick is exactly restore the whole-config snapshot from just before it" pattern:
/// `Mutation::Diff` is the WHOLE `ImperativeConfig` (not a granular patch type), `diff()` returns "the
/// full config after this op", and `store::impl_whole_record_config!` supplies the
/// `MutationDiff<ImperativeConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum ImperativeConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: ImperativeConfig,
    },
    #[dsl(key = "run-output")]
    SetRunOutput { json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for ImperativeConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for ImperativeConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
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
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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


impl protocol::Mutation<ImperativeConfig> for ImperativeConfigMutation {
    type Diff = ImperativeConfig;

    fn diff(&self, base: &ImperativeConfig) -> protocol::MutationOutcome<ImperativeConfig> {
        let mut next = base.clone();
        match self {
            ImperativeConfigMutation::Snapshot { config } => {
                if base == config {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", "Config snapshot is already identical to the requested replacement.");
                }
                return protocol::MutationOutcome::new(config.clone());
            }
            ImperativeConfigMutation::SetRunOutput { json } => {
                if &base.run_output_json == json {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", "Run output is already identical to the requested replacement.");
                }
                next.run_output_json = json.clone();
            }
            ImperativeConfigMutation::SetLocale { value } => {
                if &base.locale == value {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Locale is already \"{}\".", value));
                }
                next.locale = value.clone();
            }
            ImperativeConfigMutation::SetContributions { json } => {
                imperative_engine::sync_imperative_module_contributions(json);
                if &base.contributions_json == json {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", "Contributions are already identical to the requested replacement.");
                }
                next.contributions_json = json.clone();
            }
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &ImperativeConfig) -> Vec<Self> {
        vec![ImperativeConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imperative_config_default_is_empty_english() {
        let config = ImperativeConfig::default();
        assert!(config.run_output_json.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn imperative_config_dsl_round_trips() {
        let config = ImperativeConfig { run_output_json: r#"{"counter":1}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::os_store::test_support::assert_dsl_round_trip(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn config_operation_snapshot_diff_ignores_base() {
        let base = ImperativeConfig::default();
        let mut snapshot = base.clone();
        snapshot.run_output_json = r#"{"counter":1}"#.into();
        let operation = ImperativeConfigMutation::Snapshot { config: snapshot.clone() };
        assert_eq!(protocol::Mutation::diff(&operation, &base).diff(), &snapshot);
    }

    #[test]
    fn config_operation_set_run_output_and_locale_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&ImperativeConfigMutation::SetRunOutput { json: r#"{"counter":1}"#.into() });
        store::os_store::test_support::assert_op_line_round_trip(&ImperativeConfigMutation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
