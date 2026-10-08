//! 🧮️ Sourcing curation app — view state (`SourcingCurationConfig`) and its operation enum
//! (`SourcingCurationConfigMutation`).
//!
//! This is APP state, not document state: `filters` (search/sort) used to live on `CurationSnapshot`
//! itself (`Filters`/`CurationRuntime`) but is session-only view state, not VCS'd content — moved here so
//! it round-trips through its own real `ArtifactStore` (with a real `backwards`) instead of polluting
//! the VCS'd document. The former `selected_object_id` field/`SetSelectedObject` mutation dissolved
//! into the framework-owned "rows" interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — see `SourcingCurationApp::create_sourcing_curation_app`'s
//! `.interaction(...)` declaration. Locale and terminology come from the shared host `ViewModel`
//! (see `crate::editor::sourcing::terminology::sourcing_curation_labels`).

use crate::{Filters, TableSort};
use protocol::Mutation;

//#region 🔖️Config
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "sourcingcurationcfg")]
#[artifact(id = "curation.config")]
#[dsl(layout = "lines")]
pub struct SourcingCurationConfig {
    /// 🔍️ The pool table's active filter/search/sort state.
    #[dsl(block)]
    pub filters: Filters,
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
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for SourcingCurationConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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

//#endregion 🔖️ArtifactCodec

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for SourcingCurationConfig {
    fn default() -> Self {
        Self { filters: Filters::default(), contributions_json: default_contributions_json() }
    }
}

impl store::ConfigRecord for SourcingCurationConfig {}

/// 🧱️ Carries the optional sort as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SourcingOptionalSort {
    pub value: Option<TableSort>,
}

/// 🔺️ Sparse field delta over [`SourcingCurationConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SourcingCurationConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub filters_query: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub filters_module_ids: Option<Vec<String>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub filters_typology_path: Option<Vec<String>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub filters_min_availability: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub filters_sort: Option<SourcingOptionalSort>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub contributions_json: Option<String>,
}

impl protocol::MutationDiff<SourcingCurationConfig> for SourcingCurationConfigDiff {
    fn apply(&self, base: &SourcingCurationConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SourcingCurationConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.filters_query {
            next.filters.query = value.clone();
        }
        if let Some(value) = &self.filters_module_ids {
            next.filters.module_ids = value.clone();
        }
        if let Some(value) = &self.filters_typology_path {
            next.filters.typology_path = value.clone();
        }
        if let Some(value) = self.filters_min_availability {
            next.filters.min_availability = value;
        }
        if let Some(value) = &self.filters_sort {
            next.filters.sort = value.value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.filters_query.is_some() {
            self.filters_query = other.filters_query;
        }
        if other.filters_module_ids.is_some() {
            self.filters_module_ids = other.filters_module_ids;
        }
        if other.filters_typology_path.is_some() {
            self.filters_typology_path = other.filters_typology_path;
        }
        if other.filters_min_availability.is_some() {
            self.filters_min_availability = other.filters_min_availability;
        }
        if other.filters_sort.is_some() {
            self.filters_sort = other.filters_sort;
        }
        if other.contributions_json.is_some() {
            self.contributions_json = other.contributions_json;
        }
    }
}

impl protocol::DiffAlgebra<SourcingCurationConfig> for SourcingCurationConfigDiff {
    fn inverse(&self, base: &SourcingCurationConfig) -> Self {
        Self {
            filters_query: self.filters_query.as_ref().map(|_| base.filters.query.clone()),
            filters_module_ids: self.filters_module_ids.as_ref().map(|_| base.filters.module_ids.clone()),
            filters_typology_path: self.filters_typology_path.as_ref().map(|_| base.filters.typology_path.clone()),
            filters_min_availability: self.filters_min_availability.map(|_| base.filters.min_availability),
            filters_sort: self.filters_sort.as_ref().map(|_| SourcingOptionalSort { value: base.filters.sort.clone() }),
            contributions_json: self.contributions_json.as_ref().map(|_| base.contributions_json.clone()),
        }
    }
    fn between(base: &SourcingCurationConfig, other: &SourcingCurationConfig) -> Self {
        Self {
            filters_query: (base.filters.query != other.filters.query).then(|| other.filters.query.clone()),
            filters_module_ids: (base.filters.module_ids != other.filters.module_ids).then(|| other.filters.module_ids.clone()),
            filters_typology_path: (base.filters.typology_path != other.filters.typology_path).then(|| other.filters.typology_path.clone()),
            filters_min_availability: (base.filters.min_availability != other.filters.min_availability).then_some(other.filters.min_availability),
            filters_sort: (base.filters.sort != other.filters.sort).then(|| SourcingOptionalSort { value: other.filters.sort.clone() }),
            contributions_json: (base.contributions_json != other.contributions_json).then(|| other.contributions_json.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.filters_query.is_none() && self.filters_module_ids.is_none() && self.filters_typology_path.is_none() && self.filters_min_availability.is_none() && self.filters_sort.is_none() && self.contributions_json.is_none()
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`SourcingCurationConfig`]'s operation enum — one variant per settled interaction (search query,
/// module/typology/availability filters, sort, locale), plus a generic `Snapshot` every
/// variant's `backwards()` returns. Since a config-only dispatch is a plain `Apply`, each tick is its own
/// distinct, real config edit, and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it" — no per-field reverse-patch bookkeeping
/// needed. `Mutation::Diff` is the sparse `SourcingCurationConfigDiff` (hand-written above): `diff()` sets the slots
/// where the requested config differs from the base.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum SourcingCurationConfigMutation {
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
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for SourcingCurationConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for SourcingCurationConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
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
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

impl Mutation<SourcingCurationConfig> for SourcingCurationConfigMutation {
    type Diff = SourcingCurationConfigDiff;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: this is a plain whole-config-record
    /// mutation enum, not a `dsl::Mutations`-eligible semantic-document vocabulary — see
    /// `🧬️schema/🧬️mutations/🦀️.rs`'s derive for the contrast). ⚠️ PROVISIONAL: none of the eight
    /// `owner` paths below name a directory that exists on disk — this enum has no
    /// `🧬️mutations/<slug>` leaf triads of its own (every field lives flat in `component.rs`), so
    /// every entry is a metadata placeholder to satisfy `protocol::Mutation`, matching puzzle's
    /// `🖐️5d` and stdio's `🔊️wav`/`🏗️ifc` precedent for enums in the same situation.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔍️set-filter-query", semantic_kind: "set-filter-query", display_name: "Set Filter Query", emoji: "🔍️", aggregate_variant: "SetFilterQuery", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧩️set-filter-modules", semantic_kind: "set-filter-modules", display_name: "Set Filter Modules", emoji: "🧩️", aggregate_variant: "SetFilterModules", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌳️set-filter-typology", semantic_kind: "set-filter-typology", display_name: "Set Filter Typology", emoji: "🌳️", aggregate_variant: "SetFilterTypology", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📦️set-filter-min-availability", semantic_kind: "set-filter-min-availability", display_name: "Set Filter Min Availability", emoji: "📦️", aggregate_variant: "SetFilterMinAvailability", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/↕️set-sort", semantic_kind: "set-sort", display_name: "Set Sort", emoji: "↕️", aggregate_variant: "SetSort", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🤝️set-contributions", semantic_kind: "set-contributions", display_name: "Set Contributions", emoji: "🤝️", aggregate_variant: "SetContributions", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            SourcingCurationConfigMutation::SetFilterQuery { .. } => &Self::DESCRIPTORS[0],
            SourcingCurationConfigMutation::SetFilterModules { .. } => &Self::DESCRIPTORS[1],
            SourcingCurationConfigMutation::SetFilterTypology { .. } => &Self::DESCRIPTORS[2],
            SourcingCurationConfigMutation::SetFilterMinAvailability { .. } => &Self::DESCRIPTORS[3],
            SourcingCurationConfigMutation::SetSort { .. } => &Self::DESCRIPTORS[4],
            SourcingCurationConfigMutation::SetContributions { .. } => &Self::DESCRIPTORS[5],
        }
    }

    fn diff(&self, base: &SourcingCurationConfig) -> protocol::MutationOutcome<SourcingCurationConfigDiff> {
        protocol::MutationOutcome::new(match self {
            Self::SetFilterQuery { value } => SourcingCurationConfigDiff { filters_query: (base.filters.query != *value).then(|| value.clone()), ..Default::default() },
            Self::SetFilterModules { module_ids } => SourcingCurationConfigDiff { filters_module_ids: (base.filters.module_ids != *module_ids).then(|| module_ids.clone()), ..Default::default() },
            Self::SetFilterTypology { path } => SourcingCurationConfigDiff { filters_typology_path: (base.filters.typology_path != *path).then(|| path.clone()), ..Default::default() },
            Self::SetFilterMinAvailability { value } => SourcingCurationConfigDiff { filters_min_availability: (base.filters.min_availability != *value).then_some(*value), ..Default::default() },
            Self::SetSort { sort } => SourcingCurationConfigDiff { filters_sort: (base.filters.sort != *sort).then(|| SourcingOptionalSort { value: sort.clone() }), ..Default::default() },
            Self::SetContributions { json } => SourcingCurationConfigDiff { contributions_json: (base.contributions_json != *json).then(|| json.clone()), ..Default::default() },
        })
    }

    fn inverse(&self, base: &SourcingCurationConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetFilterQuery { .. } => Self::SetFilterQuery { value: base.filters.query.clone() },
            Self::SetFilterModules { .. } => Self::SetFilterModules { module_ids: base.filters.module_ids.clone() },
            Self::SetFilterTypology { .. } => Self::SetFilterTypology { path: base.filters.typology_path.clone() },
            Self::SetFilterMinAvailability { .. } => Self::SetFilterMinAvailability { value: base.filters.min_availability },
            Self::SetSort { .. } => Self::SetSort { sort: base.filters.sort.clone() },
            Self::SetContributions { .. } => Self::SetContributions { json: base.contributions_json.clone() },
        }])
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
