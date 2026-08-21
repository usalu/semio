//! 🧮️ Sourcing curate app — view state (`SourcingCurateConfig`) and its operation enum
//! (`SourcingCurateConfigMutation`).
//!
//! This is APP state, not document state: `filters` (search/sort) used to live on `CurateSnapshot`
//! itself (`Filters`/`CurateRuntime`) but is session-only view state, not VCS'd content — moved here so
//! it round-trips through its own real `ArtifactStore` (with a real `backwards`) instead of polluting
//! the VCS'd document. The former `selected_object_id` field/`SetSelectedObject` mutation dissolved
//! into the framework-owned "rows" interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — see `SourcingCurateApp::create_sourcing_curate_app`'s
//! `.interaction(...)` declaration. `locale` is the config-derived
//! counterpart to a host-pushed `ViewModel.locale` — `ArtifactApp::render`/`handle` no longer receive a
//! `ViewModel` at all, so locale-aware label resolution reads it off here (see
//! `crate::editor::sourcing::terminology::sourcing_curate_labels`).

use crate::artifacts::curate::{Filters, TableSort};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "sourcingcuratecfg")]
#[dsl(id = "curate.config")]
#[dsl(layout = "lines")]
pub struct SourcingCurateConfig {
    /// 🔍️ The pool table's active filter/search/sort state.
    #[dsl(block)]
    pub filters: Filters,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `sourcing.module` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for SourcingCurateConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for SourcingCurateConfig {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

async fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for SourcingCurateConfig {
    fn default() -> Self {
        Self { filters: Filters::default(), locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

store::impl_whole_record_config!(SourcingCurateConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`SourcingCurateConfig`]'s operation enum — one variant per settled interaction (search query,
/// module/typology/availability filters, sort, locale), plus a generic `Snapshot` every
/// variant's `backwards()` returns. Since a config-only dispatch is a plain `Apply` (never an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it" — no per-field reverse-patch bookkeeping
/// needed. `Mutation::Diff` is the WHOLE `SourcingCurateConfig` (not a granular patch type): `diff()`
/// returns "the full config after this op", and `store::impl_whole_record_config!` supplies the
/// `MutationDiff<SourcingCurateConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SourcingCurateConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SourcingCurateConfig,
    },
    #[dsl(key = "filter-query")]
    SetFilterQuery { value: String },
    #[dsl(key = "filter-modules")]
    SetFilterModules { module_ids: Vec<String> },
    #[dsl(key = "filter-typology")]
    SetFilterTypology { path: Vec<String> },
    #[dsl(key = "filter-min-availability")]
    SetFilterMinAvailability { value: u32 },
    #[dsl(key = "sort")]
    SetSort {
        #[dsl(block)]
        sort: Option<TableSort>,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for SourcingCurateConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for SourcingCurateConfigMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
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
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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

impl Mutation<SourcingCurateConfig> for SourcingCurateConfigMutation {
    type Diff = SourcingCurateConfig;

    /// 📦️ Whole-config field-setter/snapshot — every variant addresses the single always-present
    /// `SourcingCurateConfig` by value, so there is no target to be missing; message-free outcome
    /// per the contract's root-scoped shrink-only allowlist.
    async fn diff(&self, base: &SourcingCurateConfig) -> protocol::MutationOutcome<SourcingCurateConfig> {
        let mut next = base.clone();
        match self {
            SourcingCurateConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            SourcingCurateConfigMutation::SetFilterQuery { value } => next.filters.query = value.clone(),
            SourcingCurateConfigMutation::SetFilterModules { module_ids } => next.filters.module_ids = module_ids.clone(),
            SourcingCurateConfigMutation::SetFilterTypology { path } => next.filters.typology_path = path.clone(),
            SourcingCurateConfigMutation::SetFilterMinAvailability { value } => next.filters.min_availability = *value,
            SourcingCurateConfigMutation::SetSort { sort } => next.filters.sort = sort.clone(),
            SourcingCurateConfigMutation::SetLocale { value } => next.locale = value.clone(),
            SourcingCurateConfigMutation::SetContributions { json } => {
                next.contributions_json = json.clone();
                crate::artifacts::curate::schema::sync_sourcing_module_contributions(json);
            }
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &SourcingCurateConfig) -> Vec<Self> {
        vec![SourcingCurateConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::curate::SortDirection;

    #[semio_framework_async_macros::async_test]
    async fn sourcing_curate_config_default_matches_the_prior_document_defaults() {
        let config = SourcingCurateConfig::default();
        assert_eq!(config.filters, Filters::default());
        assert_eq!(config.locale, "en-US");
    }

    async fn sample_config() -> SourcingCurateConfig {
        SourcingCurateConfig {
            filters: Filters {
                query: "glulam".into(),
                module_ids: vec!["beams".into()],
                typology_path: vec!["beams".into(), "steel".into()],
                min_availability: 5,
                sort: Some(TableSort { column_id: "availability".into(), direction: SortDirection::Desc }),
            },
            locale: "de-DE".into(),
            contributions_json: "[]".into(),
        }
    }

    /// 🎞️ Every variant's `backwards()` must exactly restore the pre-operation config.
    async fn round_trip(config: &SourcingCurateConfig, operation: &SourcingCurateConfigMutation) -> SourcingCurateConfig {
        let forward = operation.diff(config).into_parts().0;
        let backwards = operation.inverse(config);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored).into_parts().0;
        }
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn config_mutations_round_trip_every_variant() {
        let config = sample_config();
        round_trip(&config, &SourcingCurateConfigMutation::SetFilterQuery { value: "kvh".into() });
        round_trip(&config, &SourcingCurateConfigMutation::SetFilterModules { module_ids: vec!["windows".into(), "slabs".into()] });
        round_trip(&config, &SourcingCurateConfigMutation::SetFilterTypology { path: vec!["slabs".into()] });
        round_trip(&config, &SourcingCurateConfigMutation::SetFilterMinAvailability { value: 12 });
        round_trip(&config, &SourcingCurateConfigMutation::SetSort { sort: None });
        round_trip(&config, &SourcingCurateConfigMutation::SetLocale { value: "en-US".into() });
        round_trip(&config, &SourcingCurateConfigMutation::SetContributions { json: "[]".into() });
        let snapshot = round_trip(&config, &SourcingCurateConfigMutation::Snapshot { config: SourcingCurateConfig::default() });
        assert_eq!(snapshot, SourcingCurateConfig::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::Snapshot { config: sample_config() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::SetFilterQuery { value: "kvh".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::SetFilterModules { module_ids: vec!["beams".into(), "slabs".into()] });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::SetFilterTypology { path: vec!["beams".into(), "steel".into()] });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::SetFilterMinAvailability { value: 7 });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::SetSort { sort: Some(TableSort { column_id: "name".into(), direction: SortDirection::Asc }) });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::SetSort { sort: None });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigMutation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
