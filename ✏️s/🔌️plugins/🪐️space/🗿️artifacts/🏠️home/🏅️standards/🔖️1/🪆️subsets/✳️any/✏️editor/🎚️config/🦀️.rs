//! ⚙️ S Home launcher editor — `ArtifactEditor::Config` + its operation enum (constitutional: engine + op,
//! merged at app level per the per-app recipe: `Config`/`ConfigMutation` are inherently app-scoped,
//! never artifact-scoped).
//!
//! 🕳️ `SHomeSnapshot` is a two-field counter document (`schema` + `catalog_generation`) with no tree
//! structure, id generation, or media import/export of its own, so this app has no document-side `⚙️engine` node under
//! `🗿️artifacts/🏠️home`. What this file owns is `HomeConfig` — the Home launcher's real `ArtifactEditor::Config`: the
//! human's own, undoable Home choices, today the local-studio tombstones. The folded hub directory is DERIVED hub state
//! and lives in the app transient lane instead (`🫧️transient`), never in this history.

use semio_framework_plugin::ToolExecutionContract;

//#region 🔖️Config
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "home.config")]
#[artifact(extension = "homecfg")]
#[dsl(layout = "lines")]
pub struct HomeConfig {
    /// 🪦️ Tombstones of the local-only studios the human retired from Home (`deleteVirtualFileSystemNode`), sorted and
    /// unique. Retiring never erases the studio's catalog document or its history: Home stops listing it, and undoing
    /// the retirement lists it again. Written only by `HomeConfigMutation::RetireLocalStudio`/`ListLocalStudio`.
    #[value(default)]
    pub retired_local_studio_ids: Vec<String>,
}

impl HomeConfig {
    /// 🪦️ Whether the human retired the local-only studio `space_id` from Home.
    pub fn is_local_studio_retired(&self, space_id: &str) -> bool {
        self.retired_local_studio_ids.binary_search_by(|retired| retired.as_str().cmp(space_id)).is_ok()
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for HomeConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "home.config"
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
impl store::ArtifactPack for HomeConfig {
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

impl Default for HomeConfig {
    fn default() -> Self {
        Self { retired_local_studio_ids: Vec::new() }
    }
}

impl store::ConfigRecord for HomeConfig {}

/// 🪦️ Set delta over the sorted tombstone ids: the ids that join and the ids that leave.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct HomeTombstoneDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

/// 🔺️ Sparse field delta over [`HomeConfig`]: the tombstone set delta, when the set changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct HomeConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub retired_local_studio_ids: Option<HomeTombstoneDelta>,
}

impl HomeTombstoneDelta {
    fn canonical(mut self) -> Self {
        self.added.sort();
        self.added.dedup();
        self.removed.sort();
        self.removed.dedup();
        self
    }

    /// ➕️ Composes `self` then `later` as set algebra: join∘leave and leave∘join cancel.
    fn absorb(&self, later: &Self) -> Self {
        Self {
            added: self.added.iter().filter(|id| !later.removed.contains(id)).chain(later.added.iter().filter(|id| !self.removed.contains(id))).cloned().collect(),
            removed: self.removed.iter().filter(|id| !later.added.contains(id)).chain(later.removed.iter().filter(|id| !self.added.contains(id))).cloned().collect(),
        }
        .canonical()
    }

    fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}

impl protocol::MutationDiff<HomeConfig> for HomeConfigDiff {
    fn apply(&self, base: &HomeConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<HomeConfig> {
        let mut next = base.clone();
        if let Some(delta) = &self.retired_local_studio_ids {
            if let Some(missing) = delta.removed.iter().find(|id| !base.is_local_studio_retired(id)) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", format!("tombstone {missing} does not exist")).at(["retiredLocalStudioIds", "removed"]));
            }
            if let Some(present) = delta.added.iter().find(|id| base.is_local_studio_retired(id) && !delta.removed.contains(id)) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", format!("tombstone {present} already exists")).at(["retiredLocalStudioIds", "added"]));
            }
            next.retired_local_studio_ids.retain(|id| !delta.removed.contains(id));
            next.retired_local_studio_ids.extend(delta.added.iter().cloned());
            next.retired_local_studio_ids.sort();
            next.retired_local_studio_ids.dedup();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        self.retired_local_studio_ids = match (self.retired_local_studio_ids.take(), other.retired_local_studio_ids) {
            (Some(first), Some(later)) => Some(first.absorb(&later)).filter(|delta| !delta.is_empty()),
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<HomeConfig> for HomeConfigDiff {
    fn inverse(&self, base: &HomeConfig) -> Self {
        Self {
            retired_local_studio_ids: self.retired_local_studio_ids.as_ref().map(|delta| HomeTombstoneDelta { added: delta.removed.iter().filter(|id| base.is_local_studio_retired(id)).cloned().collect(), removed: delta.added.clone() }.canonical()),
        }
    }
    fn is_empty(&self) -> bool {
        self.retired_local_studio_ids.as_ref().is_none_or(HomeTombstoneDelta::is_empty)
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `HomeConfig`'s operation enum — two tombstone verbs (retire / list again) whose diff is the sparse
/// `HomeTombstoneDelta` and whose inverse is the opposite verb on the same studio id.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum HomeConfigMutation {
    /// 🪦️ Retires one local-only studio from Home — a tombstone event; the studio's catalog document is never erased.
    #[dsl(key = "retire-local-studio")]
    RetireLocalStudio(RetireLocalStudio),
    /// 📋️ Lists one retired local-only studio in Home again — the exact inverse of `RetireLocalStudio`.
    #[dsl(key = "list-local-studio")]
    ListLocalStudio(ListLocalStudio),
}

/// 🪦️ The one studio a retire tombstone names; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "retire-local-studio")]
pub struct RetireLocalStudio {
    pub space_id: String,
}

/// 📋️ The one studio a list-again names; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "list-local-studio")]
pub struct ListLocalStudio {
    pub space_id: String,
}

//#region 🔖️OpCodec
impl protocol::OpText for HomeConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for HomeConfigMutation {
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

impl protocol::Mutation<HomeConfig> for HomeConfigMutation {
    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate — one
    /// entry per variant, in declaration order. ⚠️ PROVISIONAL: mirrors the sibling `🪐️space` config
    /// aggregate's own provisional descriptors (`⚙️engine/🪐️space/🎚️config/🦀️.rs`) — no
    /// variant below has an authored leaf directory on disk yet.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🪦️retire-local-studio", semantic_kind: "retire-local-studio", display_name: "Retire Local Studio", emoji: "🪦️", aggregate_variant: "RetireLocalStudio", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp, protocol::MutationOutcomeClass::Rejected], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📋️list-local-studio", semantic_kind: "list-local-studio", display_name: "List Local Studio", emoji: "📋️", aggregate_variant: "ListLocalStudio", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp, protocol::MutationOutcomeClass::Rejected], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            HomeConfigMutation::RetireLocalStudio(_) => &Self::DESCRIPTORS[0],
            HomeConfigMutation::ListLocalStudio(_) => &Self::DESCRIPTORS[1],
        }
    }

    type Diff = HomeConfigDiff;

    fn diff(&self, base: &HomeConfig) -> protocol::MutationOutcome<HomeConfigDiff> {
        let (HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id }) | HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id })) = self;
        let retired = matches!(self, HomeConfigMutation::RetireLocalStudio(_));
        if base.is_local_studio_retired(space_id) == retired {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Local studio {space_id} is already {}.", if retired { "retired" } else { "listed" }));
        }
        if retired && !local_studio_id_is_admissible(space_id) {
            return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::fatal("mutation.invariant", format!("Local studio id {space_id:?} is not admissible.")).at(["retiredLocalStudioIds"])]);
        }
        if retired && base.retired_local_studio_ids.len() >= HOME_RETIRED_LOCAL_STUDIOS_MAXIMUM {
            return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::error("mutation.target-mismatch", format!("Local studio {space_id} cannot be retired: {HOME_RETIRED_LOCAL_STUDIOS_MAXIMUM} studios are retired already.")).at(["retiredLocalStudioIds"])]);
        }
        let delta = if retired { HomeTombstoneDelta { added: vec![space_id.clone()], removed: Vec::new() } } else { HomeTombstoneDelta { added: Vec::new(), removed: vec![space_id.clone()] } };
        protocol::MutationOutcome::new(HomeConfigDiff { retired_local_studio_ids: Some(delta) })
    }

    fn inverse(&self, base: &HomeConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        match self {
            HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id }) | HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id }) => {
                let space_id = space_id.clone();
                if base.is_local_studio_retired(&space_id) {
                    vec![HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id })]
                } else {
                    vec![HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id })]
                }
            }
        }
    
    })())
}
}
//#endregion 🔖️ConfigOperations

//#region 📏️RetainedLimits
/// 📏️ One sealed directory page is a `HostOnly` machine payload whose only real bound is the retained wire budget —
/// the hub pages the directory with `hasMore`, so a page is bounded by construction, and the 4 KiB public scalar cap
/// would refuse an ordinary page of a dozen spaces (ticket 26/09/18 S4). Shared by both Home surfaces.
pub const HOME_DIRECTORY_PAGE_BYTES: usize = 128 * 1024;
/// 📏️ Home's config lane is a ONE-ITEM retained lane, and `ArtifactStoreOneItemFootprint::is_admissible`
/// refuses any item declaring more than [`store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES`] (1 MiB). The config holds the
/// local-studio tombstones only — at most [`HOME_RETIRED_LOCAL_STUDIOS_MAXIMUM`] ids of at most
/// [`HOME_RETIRED_LOCAL_STUDIO_ID_BYTES`] each, which is exactly that ceiling — so the store's own bound is the honest
/// base and step budget. The hub directory never travels this lane (`🫧️transient`).
pub const HOME_CONFIG_BASE_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
pub const HOME_CONFIG_STEP_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
/// 📏️ The largest config value one retained Home config mutation carries: one local-studio tombstone id.
pub const HOME_CONFIG_VALUE_BYTES: usize = HOME_RETIRED_LOCAL_STUDIO_ID_BYTES;
/// 🪦️ The most local-only studios Home keeps retired at once, and the longest studio id one tombstone names (the public
/// invocation scalar ceiling every Home route admits).
pub const HOME_RETIRED_LOCAL_STUDIOS_MAXIMUM: usize = 256;
pub const HOME_RETIRED_LOCAL_STUDIO_ID_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES;

/// 🪦️ Whether every tombstone of `config` names an admissible studio id and the set stays within its ceiling.
pub fn local_studio_tombstones_are_admissible(config: &HomeConfig) -> bool {
    config.retired_local_studio_ids.len() <= HOME_RETIRED_LOCAL_STUDIOS_MAXIMUM && config.retired_local_studio_ids.iter().all(|space_id| local_studio_id_is_admissible(space_id))
}

/// 🪪️ A local studio id one tombstone may name: non-empty, no control characters, within the scalar ceiling.
pub fn local_studio_id_is_admissible(space_id: &str) -> bool {
    !space_id.is_empty() && space_id.len() <= HOME_RETIRED_LOCAL_STUDIO_ID_BYTES && !space_id.chars().any(char::is_control)
}

/// ⏱️ The execution contract of every Home retained route, editor and viewer alike: one page or one gesture per operation,
/// the config store's own step budget, resumable so work that outlives one turn continues from its checkpoint.
pub fn home_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(HOME_DIRECTORY_PAGE_BYTES, 256, 1, HOME_CONFIG_STEP_BYTES, 7_500, 1, 1)
}
//#endregion 📏️RetainedLimits

//#region 📬️ConfigStorePreparation
/// 📬️ The ONE retained one-item preparation of the Home config lane, shared by BOTH Home surfaces (they share `HomeConfig`):
/// it seals one local-studio tombstone as one point-invertible edit.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct HomeConfigPreparationFactory;

struct HomeConfigPreparation {
    owners: store::OneItemOwners<HomeConfig, HomeConfigMutation>,
    serialized_bytes: Option<usize>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

fn home_config_refusal(kind: semio_framework_value::ValueRefusalKind, message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(kind, message)
}

fn home_config_retained_bytes(config: &HomeConfig) -> usize {
    config.retired_local_studio_ids.iter().map(String::len).sum::<usize>()
}

#[cfg(test)]
struct HomeConfigByteCounter { bytes: usize }

#[cfg(test)]
impl std::io::Write for HomeConfigByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.saturating_add(bytes.len()) > HOME_CONFIG_STEP_BYTES { return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)); }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn home_config_edit_bytes(edit: &protocol::Edit<HomeConfigMutation>) -> Result<usize, semio_framework_value::ValueError> {
    let bytes = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(edit)).len();
    if bytes > HOME_CONFIG_STEP_BYTES {
        return Err(home_config_refusal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Space Home config edit exceeds its serialized byte envelope"));
    }
    Ok(bytes)
}

/// 🛂️ The byte extent and ceiling of the config mutations the retained lane admits: one local-studio tombstone. Every
/// other mutation never travels the retained lane.
fn home_config_retained_admission(mutation: &HomeConfigMutation) -> Option<(usize, usize)> {
    match mutation {
        HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id }) | HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id }) if local_studio_id_is_admissible(space_id) => Some((space_id.len(), HOME_RETIRED_LOCAL_STUDIO_ID_BYTES)),
        _ => None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<HomeConfig, HomeConfigMutation> for HomeConfigPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<HomeConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<HomeConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &HomeConfigMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let Some((mutation_bytes, maximum_bytes)) = home_config_retained_admission(mutation) else {
            return Err("Space Home config preparation rejects non-retained mutations".into());
        };
        if lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes {
            return Err("Space Home config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, HOME_CONFIG_STEP_BYTES))
    }

    fn begin_demand(&self, _mutation: &HomeConfigMutation, _lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<HomeConfigPreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation, HomeConfigMutation>, grant: store::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation, HomeConfigMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        let Some((mutation_bytes, maximum_bytes)) = home_config_retained_admission(&request.mutation) else {
            return Err((home_config_refusal(semio_framework_value::ValueRefusalKind::InvariantViolated, "home preparation rejected original mutation or publication authority"), request));
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((home_config_refusal(semio_framework_value::ValueRefusalKind::InvariantViolated, "home preparation rejected original mutation or publication authority"), request));
        }
        Ok((Box::new(HomeConfigPreparation { owners: store::OneItemOwners::from_request(request), serialized_bytes: None, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false }), progress))
    }
}

impl HomeConfigPreparation {
    fn progress() -> semio_framework_value::retained_clone::RetainedCloneProgress {
        semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }
    }
}

impl store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation> for HomeConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::ValueRefusalKind::{InvalidValue, InvariantViolated, OwnershipLimit};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(home_config_refusal(InvalidValue, "Space Home config preparation retains its original refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        if self.owners.candidate.is_none() && self.owners.sealed.is_none() {
            let base = self.owners.base.as_ref().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its exact base root"))?.get();
            let base_bytes = home_config_retained_bytes(base);
            if base_bytes > HOME_CONFIG_BASE_BYTES { return Err(home_config_refusal(OwnershipLimit, "Space Home config base exceeds retained byte capacity")); }
            let mutation = self.owners.mutation.as_ref().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its mutation owner"))?;
            let (post, inverse) = match mutation {
                HomeConfigMutation::RetireLocalStudio(_) | HomeConfigMutation::ListLocalStudio(_) => {
                    let outcome = <HomeConfigMutation as protocol::Mutation<HomeConfig>>::diff(mutation, base);
                    let post = (!protocol::DiffAlgebra::<HomeConfig>::is_empty(outcome.diff()))
                        .then(|| protocol::apply_diff(outcome.diff(), base).ok())
                        .flatten()
                        .filter(local_studio_tombstones_are_admissible)
                        .ok_or_else(|| home_config_refusal(InvalidValue, "Space Home config preparation refuses a tombstone that changes nothing or exceeds its ceiling"))?;
                    let inverse = <HomeConfigMutation as protocol::Mutation<HomeConfig>>::inverse(mutation, base)?.into_iter().next().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its tombstone inverse"))?;
                    (post, inverse)
                }
            };
            let mutation = self.owners.mutation.take().expect("observed original mutation owner");
            *self.owners.candidate = Some((post, vec![inverse], mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, Self::progress()));
        }
        if self.owners.sealed.is_none() {
            let authority = self.owners.authority.as_ref().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its Store authority"))?;
            let (post, inverse, forward) = self.owners.candidate.take().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its candidate"))?;
            *self.owners.sealed = Some((post, authority.next_edit(forward, inverse)));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.owners.sealed.as_ref().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its semantic edit"))?;
            let bytes = home_config_edit_bytes(edit)?;
            if bytes.saturating_add(home_config_retained_bytes(post)).saturating_add(512) > HOME_CONFIG_STEP_BYTES {
                return Err(home_config_refusal(OwnershipLimit, "Space Home config publication exceeds its complete retained envelope"));
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, Self::progress()));
        }
        let authority = self.owners.authority.as_ref().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its Store authority"))?;
        let (post, edit) = self.owners.sealed.take().ok_or_else(|| home_config_refusal(InvariantViolated, "Space Home config preparation lost its validated edit"))?;
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Self::progress()))
    }
    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️ConfigStorePreparation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
