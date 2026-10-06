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
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "home.config")]
#[artifact(extension = "homecfg")]
#[dsl(layout = "lines")]
pub struct HomeConfig {
    /// 🪦️ Tombstones of the local-only studios the human retired from Home (`deleteVirtualFileSystemNode`), sorted and
    /// unique. Retiring never erases the studio's catalog document or its history: Home stops listing it, and undoing
    /// the retirement lists it again. Written only by `HomeConfigMutation::RetireLocalStudio`/`RestoreLocalStudio`.
    #[value(default)]
    pub retired_local_studio_ids: Vec<String>,
}

impl HomeConfig {
    /// 🪦️ Whether the human retired the local-only studio `space_id` from Home.
    pub fn is_local_studio_retired(&self, space_id: &str) -> bool {
        self.retired_local_studio_ids.binary_search_by(|retired| retired.as_str().cmp(space_id)).is_ok()
    }

    /// 🪦️ The config with `space_id` retired (`true`) or restored (`false`); `None` when that is already its state.
    pub fn with_local_studio_retired(&self, space_id: &str, retired: bool) -> Option<Self> {
        let position = self.retired_local_studio_ids.binary_search_by(|entry| entry.as_str().cmp(space_id));
        let mut next = self.clone();
        match (position, retired) {
            (Err(index), true) => next.retired_local_studio_ids.insert(index, space_id.to_owned()),
            (Ok(index), false) => {
                next.retired_local_studio_ids.remove(index);
            }
            _ => return None,
        }
        Some(next)
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

store::impl_whole_record_config!(HomeConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `HomeConfig`'s operation enum — mirrors `engine::space::config::SpaceConfigMutation`'s
/// whole-record-diff design (see its doc comment for the full rationale).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum HomeConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: HomeConfig,
    },
    /// 🪦️ Retires one local-only studio from Home — a tombstone event; the studio's catalog document is never erased.
    #[dsl(key = "retire-local-studio")]
    RetireLocalStudio {
        space_id: String,
    },
    /// ♻️ Lists one retired local-only studio in Home again — the exact inverse of `RetireLocalStudio`.
    #[dsl(key = "restore-local-studio")]
    RestoreLocalStudio {
        space_id: String,
    },
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
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set", semantic_kind: "set-snapshot", display_name: "Set Snapshot", emoji: "⚙️", aggregate_variant: "Snapshot", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🪦️retire-local-studio", semantic_kind: "retire-local-studio", display_name: "Retire Local Studio", emoji: "🪦️", aggregate_variant: "RetireLocalStudio", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp, protocol::MutationOutcomeClass::Rejected], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/♻️restore-local-studio", semantic_kind: "restore-local-studio", display_name: "Restore Local Studio", emoji: "♻️", aggregate_variant: "RestoreLocalStudio", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp, protocol::MutationOutcomeClass::Rejected], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            HomeConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            HomeConfigMutation::RetireLocalStudio { .. } => &Self::DESCRIPTORS[1],
            HomeConfigMutation::RestoreLocalStudio { .. } => &Self::DESCRIPTORS[2],
        }
    }

    type Diff = HomeConfig;

    fn diff(&self, base: &HomeConfig) -> protocol::MutationOutcome<HomeConfig> {
        match self {
            HomeConfigMutation::Snapshot { config } => protocol::MutationOutcome::new(config.clone()),
            HomeConfigMutation::RetireLocalStudio { space_id } | HomeConfigMutation::RestoreLocalStudio { space_id } => {
                let retired = matches!(self, HomeConfigMutation::RetireLocalStudio { .. });
                match base.with_local_studio_retired(space_id, retired) {
                    Some(candidate) if local_studio_tombstones_are_admissible(&candidate) => protocol::MutationOutcome::new(candidate),
                    Some(_) if !local_studio_id_is_admissible(space_id) => protocol::MutationOutcome::new(base.clone()).absorb_messages([protocol::MutationMessage::fatal("mutation.invariant", format!("Local studio id {space_id:?} is not admissible.")).at(["retiredLocalStudioIds"])]),
                    Some(_) => protocol::MutationOutcome::new(base.clone()).absorb_messages([protocol::MutationMessage::error("mutation.target-mismatch", format!("Local studio {space_id} cannot be retired: {HOME_RETIRED_LOCAL_STUDIOS_MAXIMUM} studios are retired already.")).at(["retiredLocalStudioIds"])]),
                    None => protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", format!("Local studio {space_id} is already {}.", if retired { "retired" } else { "listed" })),
                }
            }
        }
    }

    fn inverse(&self, base: &HomeConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        match self {
            HomeConfigMutation::RetireLocalStudio { space_id } | HomeConfigMutation::RestoreLocalStudio { space_id } => {
                let space_id = space_id.clone();
                if base.is_local_studio_retired(&space_id) {
                    vec![HomeConfigMutation::RetireLocalStudio { space_id }]
                } else {
                    vec![HomeConfigMutation::RestoreLocalStudio { space_id }]
                }
            }
            HomeConfigMutation::Snapshot { .. } => vec![HomeConfigMutation::Snapshot { config: base.clone() }],
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
pub struct HomeConfigPreparationFactory;

struct HomeConfigPreparation {
    base: Option<store::SnapshotRead<HomeConfig>>,
    mutation: Option<HomeConfigMutation>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(HomeConfig, HomeConfigMutation, HomeConfigMutation)>,
    sealed_candidate: Option<(HomeConfig, protocol::Edit<HomeConfigMutation>)>,
    serialized_bytes: Option<usize>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
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

fn home_config_edit_bytes(edit: &protocol::Edit<HomeConfigMutation>) -> Result<usize, String> {
    let bytes = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(edit)).len();
    if bytes > HOME_CONFIG_STEP_BYTES {
        return Err("Space Home config edit exceeds its serialized byte envelope".to_string());
    }
    Ok(bytes)
}

/// 🛂️ The byte extent and ceiling of the config mutations the retained lane admits: one local-studio tombstone. Every
/// other mutation (a whole-record `Snapshot`) never travels the retained lane.
fn home_config_retained_admission(mutation: &HomeConfigMutation) -> Option<(usize, usize)> {
    match mutation {
        HomeConfigMutation::RetireLocalStudio { space_id } | HomeConfigMutation::RestoreLocalStudio { space_id } if local_studio_id_is_admissible(space_id) => Some((space_id.len(), HOME_RETIRED_LOCAL_STUDIO_ID_BYTES)),
        _ => None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<HomeConfig, HomeConfigMutation> for HomeConfigPreparationFactory {
    fn preflight(&self, mutation: &HomeConfigMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let Some((mutation_bytes, maximum_bytes)) = home_config_retained_admission(mutation) else {
            return Err("Space Home config preparation rejects non-retained mutations".into());
        };
        if lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes {
            return Err("Space Home config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, HOME_CONFIG_STEP_BYTES))
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>> {
        let Some((mutation_bytes, maximum_bytes)) = home_config_retained_admission(&request.mutation) else {
            return Err(request);
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(HomeConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), authority: Some(request.authority), candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation> for HomeConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        // 🎟️ The grant is a PAGE, not the owner's whole envelope: `ArtifactStoreOneItemGrant`'s own
        // contract is "consume at most one semantic unit", and every framework pump that drives this
        // preparation grants `TYPED_OPERATION_RESULT_PAGE_BYTES` (4 KiB) — the typed-operation
        // publication ladder hard-codes it (`🔌️plugin/🦀️.rs`'s `ArtifactStoreOneItemGrant { maximum_items: 1,
        // maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }`). Demanding `HOME_CONFIG_STEP_BYTES`
        // (1 MiB) therefore answered `Blocked` on EVERY unit for ever, and `Blocked` is a silent
        // non-advance: the operation stayed in `Publishing`, the actor stayed in `MoreWork` with no
        // effect, no patch and no fault, and the signed-in Home listed 0 spaces while the host's drain
        // polled it for the whole session (ticket 26/09/18 S8, measured on serve 6190 → hub 7611).
        if !grant.permits_one() || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() && self.sealed_candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Space Home config preparation lost its exact base root".to_string())?.get();
            let base_bytes = home_config_retained_bytes(base);
            if base_bytes > HOME_CONFIG_BASE_BYTES { return Err("Space Home config base exceeds retained byte capacity".into()); }
            let mutation = self.mutation.take().ok_or_else(|| "Space Home config preparation lost its mutation owner".to_string())?;
            let (post, inverse) = match &mutation {
                HomeConfigMutation::RetireLocalStudio { space_id } | HomeConfigMutation::RestoreLocalStudio { space_id } => {
                    let post = base
                        .with_local_studio_retired(space_id, matches!(mutation, HomeConfigMutation::RetireLocalStudio { .. }))
                        .filter(|candidate| local_studio_tombstones_are_admissible(candidate))
                        .ok_or_else(|| format!("Space Home config preparation refuses the tombstone of {space_id}: it changes nothing or exceeds its ceiling"))?;
                    let inverse = <HomeConfigMutation as protocol::Mutation<HomeConfig>>::inverse(&mutation, base).map_err(semio_framework_value::ValueError::into_message)?.into_iter().next().ok_or_else(|| "Space Home config preparation lost its tombstone inverse".to_string())?;
                    (post, inverse)
                }
                _ => return Err("Space Home config preparation received a non-retained mutation".into()),
            };
            self.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if self.sealed_candidate.is_none() {
            let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Space Home config preparation lost its candidate".to_string())?;
            let authority = self.authority.as_ref().ok_or_else(|| "Space Home config preparation lost its Store authority".to_string())?;
            self.sealed_candidate = Some((post, authority.next_edit(forward, vec![inverse])));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.sealed_candidate.as_ref().ok_or_else(|| "Space Home config preparation lost its semantic edit".to_string())?;
            let bytes = home_config_edit_bytes(edit)?;
            if bytes.saturating_add(home_config_retained_bytes(post)).saturating_add(512) > HOME_CONFIG_STEP_BYTES {
                return Err("Space Home config publication exceeds its complete retained envelope".into());
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        let (post, edit) = self.sealed_candidate.take().ok_or_else(|| "Space Home config preparation lost its validated edit".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Space Home config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }
    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || !grant.permits_one() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        // 🧹️ One retained owner per granted page, never more bytes than the page granted — the same
        // reasoning as `advance` above: an owner that answers `Blocked` until it is handed its whole
        // declared envelope never closes under the framework's 4 KiB pumps.
        if self.prepared.take().is_some() || self.sealed_candidate.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: grant.maximum_bytes }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Space Home config preparation could not return its exact base root")); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            let bytes = authority.actor().len();
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.candidate.is_none() && self.sealed_candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigStorePreparation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
