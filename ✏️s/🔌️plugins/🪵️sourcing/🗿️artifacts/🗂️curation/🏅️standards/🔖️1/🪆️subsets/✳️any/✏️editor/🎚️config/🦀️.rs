//! 🧮️ Sourcing curation app — view state (`SourcingCurationConfig`) and its operation enum
//! (`SourcingCurationConfigMutation`).
//!
//! This is APP state, not document state: `filters` (search/sort) used to live on `CurationSnapshot`
//! itself (`Filters`/`CurationRuntime`) but is session-only view state, not VCS'd content — moved here so
//! it round-trips through its own real `ArtifactStore` (with a real `backwards`) instead of polluting
//! the VCS'd document. The former `selected_object_id` field/`SetSelectedObject` mutation dissolved
//! into the framework-owned "rows" interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — see `SourcingCurationApp::create_sourcing_curation_app`'s
//! `.interaction(...)` declaration. `locale` is the config-derived
//! counterpart to a host-pushed `ViewModel.locale` — `ArtifactApp::render`/`handle` no longer receive a
//! `ViewModel` at all, so locale-aware label resolution reads it off here (see
//! `crate::editor::sourcing::terminology::sourcing_curation_labels`).

use crate::artifacts::curation::{Filters, TableSort};
use protocol::Mutation;

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "sourcingcurationcfg")]
#[dsl(id = "curation.config")]
#[dsl(layout = "lines")]
pub struct SourcingCurationConfig {
    /// 🔍️ The pool table's active filter/search/sort state.
    #[dsl(block)]
    pub filters: Filters,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `sourcing.module` hot-swap installs.
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for SourcingCurationConfig {
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
impl store::ArtifactPack for SourcingCurationConfig {
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

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for SourcingCurationConfig {
    fn default() -> Self {
        Self { filters: Filters::default(), locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

store::impl_whole_record_config!(SourcingCurationConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`SourcingCurationConfig`]'s operation enum — one variant per settled interaction (search query,
/// module/typology/availability filters, sort, locale), plus a generic `Snapshot` every
/// variant's `backwards()` returns. Since a config-only dispatch is a plain `Apply` (never an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it" — no per-field reverse-patch bookkeeping
/// needed. `Mutation::Diff` is the WHOLE `SourcingCurationConfig` (not a granular patch type): `diff()`
/// returns "the full config after this op", and `store::impl_whole_record_config!` supplies the
/// `MutationDiff<SourcingCurationConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps)]
pub enum SourcingCurationConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SourcingCurationConfig,
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
impl protocol::OpText for SourcingCurationConfigMutation {
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
impl protocol::OpBinary for SourcingCurationConfigMutation {
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

impl Mutation<SourcingCurationConfig> for SourcingCurationConfigMutation {
    type Diff = SourcingCurationConfig;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: this is a plain whole-config-record
    /// mutation enum, not a `dsl::Mutations`-eligible semantic-document vocabulary — see
    /// `🧬️schema/🧬️mutations/🦀️.rs`'s derive for the contrast). ⚠️ PROVISIONAL: none of the eight
    /// `owner` paths below name a directory that exists on disk — this enum has no
    /// `🧬️mutations/<slug>` leaf triads of its own (every field lives flat in `component.rs`), so
    /// every entry is a metadata placeholder to satisfy `protocol::Mutation`, matching puzzle's
    /// `🖐️5d` and stdio's `🔊️wav`/`🏗️ifc` precedent for enums in the same situation.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔍️set-filter-query", semantic_kind: "set-filter-query", display_name: "Set Filter Query", emoji: "🔍️", aggregate_variant: "SetFilterQuery", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧩️set-filter-modules", semantic_kind: "set-filter-modules", display_name: "Set Filter Modules", emoji: "🧩️", aggregate_variant: "SetFilterModules", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌳️set-filter-typology", semantic_kind: "set-filter-typology", display_name: "Set Filter Typology", emoji: "🌳️", aggregate_variant: "SetFilterTypology", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📦️set-filter-min-availability", semantic_kind: "set-filter-min-availability", display_name: "Set Filter Min Availability", emoji: "📦️", aggregate_variant: "SetFilterMinAvailability", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/↕️set-sort", semantic_kind: "set-sort", display_name: "Set Sort", emoji: "↕️", aggregate_variant: "SetSort", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🗣️set-locale", semantic_kind: "set-locale", display_name: "Set Locale", emoji: "🗣️", aggregate_variant: "SetLocale", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🤝️set-contributions", semantic_kind: "set-contributions", display_name: "Set Contributions", emoji: "🤝️", aggregate_variant: "SetContributions", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            SourcingCurationConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            SourcingCurationConfigMutation::SetFilterQuery { .. } => &Self::DESCRIPTORS[1],
            SourcingCurationConfigMutation::SetFilterModules { .. } => &Self::DESCRIPTORS[2],
            SourcingCurationConfigMutation::SetFilterTypology { .. } => &Self::DESCRIPTORS[3],
            SourcingCurationConfigMutation::SetFilterMinAvailability { .. } => &Self::DESCRIPTORS[4],
            SourcingCurationConfigMutation::SetSort { .. } => &Self::DESCRIPTORS[5],
            SourcingCurationConfigMutation::SetLocale { .. } => &Self::DESCRIPTORS[6],
            SourcingCurationConfigMutation::SetContributions { .. } => &Self::DESCRIPTORS[7],
        }
    }

    /// 📦️ Whole-config field-setter/snapshot — every variant addresses the single always-present
    /// `SourcingCurationConfig` by value, so there is no target to be missing; message-free outcome
    /// per the contract's root-scoped shrink-only allowlist.
    fn diff(&self, base: &SourcingCurationConfig) -> protocol::MutationOutcome<SourcingCurationConfig> {
        let mut next = base.clone();
        match self {
            SourcingCurationConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            SourcingCurationConfigMutation::SetFilterQuery { value } => next.filters.query = value.clone(),
            SourcingCurationConfigMutation::SetFilterModules { module_ids } => next.filters.module_ids = module_ids.clone(),
            SourcingCurationConfigMutation::SetFilterTypology { path } => next.filters.typology_path = path.clone(),
            SourcingCurationConfigMutation::SetFilterMinAvailability { value } => next.filters.min_availability = *value,
            SourcingCurationConfigMutation::SetSort { sort } => next.filters.sort = sort.clone(),
            SourcingCurationConfigMutation::SetLocale { value } => next.locale = value.clone(),
            SourcingCurationConfigMutation::SetContributions { json } => {
                next.contributions_json = json.clone();
            }
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &SourcingCurationConfig) -> Vec<Self> {
        vec![SourcingCurationConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::curation::SortDirection;

    #[semio_framework_async_macros::async_test]
    async fn sourcing_curation_config_default_matches_the_prior_document_defaults() {
        let config = SourcingCurationConfig::default();
        assert_eq!(config.filters, Filters::default());
        assert_eq!(config.locale, "en-US");
    }

    fn sample_config() -> SourcingCurationConfig {
        SourcingCurationConfig {
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
    fn round_trip(config: &SourcingCurationConfig, operation: &SourcingCurationConfigMutation) -> SourcingCurationConfig {
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
        round_trip(&config, &SourcingCurationConfigMutation::SetFilterQuery { value: "kvh".into() });
        round_trip(&config, &SourcingCurationConfigMutation::SetFilterModules { module_ids: vec!["windows".into(), "slabs".into()] });
        round_trip(&config, &SourcingCurationConfigMutation::SetFilterTypology { path: vec!["slabs".into()] });
        round_trip(&config, &SourcingCurationConfigMutation::SetFilterMinAvailability { value: 12 });
        round_trip(&config, &SourcingCurationConfigMutation::SetSort { sort: None });
        round_trip(&config, &SourcingCurationConfigMutation::SetLocale { value: "en-US".into() });
        round_trip(&config, &SourcingCurationConfigMutation::SetContributions { json: "[]".into() });
        let snapshot = round_trip(&config, &SourcingCurationConfigMutation::Snapshot { config: SourcingCurationConfig::default() });
        assert_eq!(snapshot, SourcingCurationConfig::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::Snapshot { config: sample_config() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterQuery { value: "kvh".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterModules { module_ids: vec!["beams".into(), "slabs".into()] });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterTypology { path: vec!["beams".into(), "steel".into()] });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetFilterMinAvailability { value: 7 });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort { sort: Some(TableSort { column_id: "name".into(), direction: SortDirection::Asc }) });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetSort { sort: None });
        store::os_store::test_support::assert_op_text_binary_equivalence(&SourcingCurationConfigMutation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
