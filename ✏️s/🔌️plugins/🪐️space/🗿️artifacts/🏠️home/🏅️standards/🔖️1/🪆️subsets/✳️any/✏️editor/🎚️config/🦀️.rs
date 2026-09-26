//! ⚙️ S Home launcher editor — `ArtifactEditor::Config` + its operation enum (constitutional: engine + op,
//! merged at app level per the per-app recipe: `Config`/`ConfigMutation` are inherently app-scoped,
//! never artifact-scoped).
//!
//! 🕳️ `SHomeSnapshot` is a two-field counter document (`schema` + `catalog_generation`) with no tree
//! structure, id generation, or media import/export of its own — the original monolith never factored
//! out a pure `empty_home_document()`/compute helper (every call site builds the literal
//! `SHomeSnapshot { schema: "s.home".into(), catalog_generation: N }` directly), so this app has no
//! document-side `⚙️engine` node under `🗿️artifacts/🏠️home`. What this file owns is `HomeConfig` — the
//! Home launcher's real `ArtifactEditor::Config`: the folded hub directory read model
//! (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C1/§C6).

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use semio_framework_plugin::{AppEvent, Emit, Fault, NoDraftMutation, ToolExecutionContract};
use std::collections::BTreeMap;

/// 🧾️ Exact terminal proof that one authenticated directory frontier is the retained Home config.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryProjectionReceiptV1 {
    pub schema: String,
    pub session_binding_sha256: String,
    pub authorization_generation: u64,
    pub through_seq_inclusive: u64,
    pub receipt_sha256: String,
}

impl DirectoryProjectionReceiptV1 {
    pub const SCHEMA: &'static str = "semio.space.home.directory-projection-receipt.v1";

    /// 🛡️ Validates the complete browser-visible receipt without admitting resume-only fields.
    pub fn validate(&self) -> bool {
        self.schema == Self::SCHEMA
            && directory_sha256_is_valid(&self.session_binding_sha256)
            && self.authorization_generation > 0
            && self.authorization_generation <= store::os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && self.through_seq_inclusive <= store::os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && directory_sha256_is_valid(&self.receipt_sha256)
    }
}

//#region 🔖️DirectoryJson
/// 📇️ `store::os_directory::DirectoryReadModel`/`DirectorySpace` carry no `Serialize`/`Deserialize`
/// derive of their own (framework-owned, `🧰️framework/**` is outside this lease) and the `dsl` derive
/// has no opaque/json escape hatch for a nested non-`DslField` type (checked: `#[dsl(...)]` recognizes
/// `key/positional/list/tuple/statements/block/base64/flatten/table/unit/angle/refs/defines/lang/
/// lang_from/coord/dir` only — no `json`/`opaque`/`blob`, and `base64` is documented as `Vec<u8>`-only).
/// Mirrors the sibling `🔱️trinity/🔌️jack` plugin's own `JackConfig.jack_result_json: String`
/// convention instead: the DSL-layer field stays a plain `String`, the rich-type round trip happens by
/// hand, entirely inside this file. `SpaceView`/`MemberView`/`UserView` (the read model's own leaves)
/// already derive `Serialize`/`Deserialize`; only the two WRAPPER structs need a hand-written wire shape.
#[derive(value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct DirectorySpaceWire {
    view: store::os_directory::SpaceView,
    members: Vec<store::os_directory::MemberView>,
    documents: Vec<store::os_directory::DocumentDescriptor>,
    indexed_documents: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>,
}

#[derive(value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct DirectoryReadModelWire {
    spaces: BTreeMap<String, DirectorySpaceWire>,
    cursor: u64,
    users: BTreeMap<String, store::os_directory::UserView>,
}

/// 📇️ Encodes a `DirectoryReadModel` as the `directory_json` DSL field's wire value.
pub(crate) fn directory_to_json(model: &store::os_directory::DirectoryReadModel) -> String {
    let wire = DirectoryReadModelWire {
        spaces: model
            .spaces
            .iter()
            .map(|(id, space)| (id.clone(), DirectorySpaceWire { view: space.view.clone(), members: space.members.clone(), documents: space.documents.clone(), indexed_documents: space.indexed_documents.clone() }))
            .collect(),
        cursor: model.cursor,
        users: model.users.clone(),
    };
    pack::to_json_string(&wire)
}

/// 📇️ Decodes `directory_json` without converting persisted corruption into an empty projection.
fn directory_from_json(json: &str) -> Result<store::os_directory::DirectoryReadModel, Fault> {
    let wire: DirectoryReadModelWire = pack::from_json_str(json).map_err(|_| Fault::from("s.home.directory-projection-malformed"))?;
    Ok(store::os_directory::DirectoryReadModel {
        spaces: wire
            .spaces
            .into_iter()
            .map(|(id, space)| (id, store::os_directory::DirectorySpace { view: space.view, members: space.members, documents: space.documents, indexed_documents: space.indexed_documents }))
            .collect(),
        cursor: wire.cursor,
        users: wire.users,
    })
}

fn directory_sha256_is_valid(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

/// 🛡️ Validates the complete persisted projection authority, including the intentional unbound initial state.
pub(crate) fn directory_projection_state_is_valid(directory_json: &str, session_binding_sha256: &str, authorization_generation: u64, receipt_sha256: &str) -> bool {
    if directory_from_json(directory_json).is_err() {
        return false;
    }
    if session_binding_sha256.is_empty() && authorization_generation == 0 && receipt_sha256.is_empty() {
        return true;
    }
    authorization_generation > 0
        && authorization_generation <= store::os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER
        && directory_sha256_is_valid(session_binding_sha256)
        && directory_sha256_is_valid(receipt_sha256)
}
//#endregion 🔖️DirectoryJson

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "home.config")]
#[dsl(extension = "homecfg")]
#[dsl(layout = "lines")]
pub struct HomeConfig {
    /// 📇️ JSON-serialized `DirectoryReadModel` (see `🔖️DirectoryJson` above), read via `directory()`.
    /// No optimistic mutation (contract §C6): the ONLY writer is `HomeConfigMutation::ReplaceDirectoryProjection`,
    /// sealed from one authenticated `DirectoryEventPageV1` together with the three receipt fields below.
    pub directory_json: String,
    /// 🔐️ Opaque digest binding the accepted page frontier to one authenticated hub session.
    pub directory_session_binding_sha256: String,
    /// 🛂️ Authorization generation under which the current projection was filtered.
    pub directory_authorization_generation: u64,
    /// 🧾️ Receipt of the last durably accepted directory page.
    pub directory_receipt_sha256: String,
}

impl HomeConfig {
    /// 📇️ Decodes the folded hub directory read model.
    pub fn directory(&self) -> Result<store::os_directory::DirectoryReadModel, Fault> {
        directory_from_json(&self.directory_json)
    }

    /// 🧾️ Projects the retained config's exact terminal acknowledgement authority.
    pub fn directory_projection_receipt(&self) -> Option<DirectoryProjectionReceiptV1> {
        let through_seq_inclusive = self.directory().ok()?.cursor;
        let receipt = DirectoryProjectionReceiptV1 {
            schema: DirectoryProjectionReceiptV1::SCHEMA.into(),
            session_binding_sha256: self.directory_session_binding_sha256.clone(),
            authorization_generation: self.directory_authorization_generation,
            through_seq_inclusive,
            receipt_sha256: self.directory_receipt_sha256.clone(),
        };
        receipt.validate().then_some(receipt)
    }

    /// 📄️ Applies one authenticated page to a replacement config without exposing partial folds. A page from the origin
    /// (`after_seq_exclusive == 0`) rebuilds the projection under any authority: the reader's visible set can change
    /// retroactively (a human added to an existing space sees that space's earlier events), so the worker re-reads the
    /// directory from the origin on the hub's `access-changed` signal; a later page must continue the held frontier.
    pub fn apply_directory_event_page(&self, page: &store::os_directory::DirectoryEventPageV1) -> Result<Self, Fault> {
        page.validate().map_err(|_| Fault::from("s.home.directory-event-page-invalid"))?;
        let current = self.directory()?;
        let same_authority = self.directory_session_binding_sha256 == page.session_binding_sha256
            && self.directory_authorization_generation == page.authorization_generation;
        if same_authority && current.cursor == page.through_seq_inclusive && self.directory_receipt_sha256 == page.receipt_sha256 {
            return Ok(self.clone());
        }
        let mut directory = if page.after_seq_exclusive == 0 {
            store::os_directory::DirectoryReadModel::default()
        } else if !same_authority {
            return Err(Fault::from("s.home.directory-event-page-rebootstrap-required"));
        } else if page.after_seq_exclusive != current.cursor {
            return Err(Fault::from("s.home.directory-event-page-frontier-race"));
        } else {
            current
        };
        for event in &page.events {
            directory = store::os_directory::fold(directory, event);
        }
        directory.cursor = page.through_seq_inclusive;
        let mut next = self.clone();
        next.directory_json = directory_to_json(&directory);
        next.directory_session_binding_sha256 = page.session_binding_sha256.clone();
        next.directory_authorization_generation = page.authorization_generation;
        next.directory_receipt_sha256 = page.receipt_sha256.clone();
        Ok(next)
    }

    /// 📬️ The whole config-lane answer to one sealed page, for BOTH Home surfaces: the replacement projection (none
    /// when the page is the frontier already held) and the typed terminal receipt the host acknowledges the page by.
    /// The editor's `applyDirectoryEventPage` and the viewer's retained twin answer through this one function, so the
    /// read-only Home lists exactly the rows the editor lists (ticket 26/09/23 S16).
    pub fn directory_event_page_emit(&self, page_json: &str) -> Result<Emit<SHomeMutation, HomeConfigMutation, NoDraftMutation>, Fault> {
        let page = store::os_directory::DirectoryEventPageV1::parse_canonical_json(page_json).map_err(|_| Fault::from("s.home.directory-event-page-invalid"))?;
        let next = self.apply_directory_event_page(&page)?;
        let receipt = next.directory_projection_receipt().ok_or_else(|| Fault::from("s.home.directory-projection-receipt-invalid"))?;
        let event = AppEvent { kind: DirectoryProjectionReceiptV1::SCHEMA.into(), payload: protocol::ToValue::to_value(&receipt) };
        if next == *self {
            return Ok(Emit { events: vec![event], ..Default::default() });
        }
        Ok(Emit {
            config_mutations: vec![HomeConfigMutation::ReplaceDirectoryProjection {
                directory_json: next.directory_json,
                session_binding_sha256: next.directory_session_binding_sha256,
                authorization_generation: next.directory_authorization_generation,
                receipt_sha256: next.directory_receipt_sha256,
            }],
            events: vec![event],
            ..Default::default()
        })
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for HomeConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "home.config"
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
impl store::ArtifactPack for HomeConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

impl Default for HomeConfig {
    fn default() -> Self {
        Self {
            directory_json: directory_to_json(&store::os_directory::DirectoryReadModel::default()),
            directory_session_binding_sha256: String::new(),
            directory_authorization_generation: 0,
            directory_receipt_sha256: String::new(),
        }
    }
}

store::impl_whole_record_config!(HomeConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ `HomeConfig`'s operation enum — mirrors `engine::space::config::SpaceConfigMutation`'s
/// whole-record-diff design (see its doc comment for the full rationale).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
pub enum HomeConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: HomeConfig,
    },
    /// 📄️ Atomically replaces the page-derived projection and its authenticated resume authority.
    #[dsl(key = "replace-directory-projection")]
    ReplaceDirectoryProjection {
        directory_json: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        receipt_sha256: String,
    },
}

//#region 🔖️OpCodec
impl protocol::OpText for HomeConfigMutation {
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
impl protocol::OpBinary for HomeConfigMutation {
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

impl protocol::Mutation<HomeConfig> for HomeConfigMutation {
    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate — one
    /// entry per variant, in declaration order. ⚠️ PROVISIONAL: mirrors the sibling `🪐️space` config
    /// aggregate's own provisional descriptors (`⚙️engine/🪐️space/🎚️config/🦀️.rs`) — no
    /// variant below has an authored leaf directory on disk yet.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-snapshot", semantic_kind: "set-snapshot", display_name: "Set Snapshot", emoji: "⚙️", aggregate_variant: "Snapshot", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️replace-directory-projection", semantic_kind: "replace-directory-projection", display_name: "Replace Directory Projection", emoji: "📄️", aggregate_variant: "ReplaceDirectoryProjection", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            HomeConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            HomeConfigMutation::ReplaceDirectoryProjection { .. } => &Self::DESCRIPTORS[1],
        }
    }

    type Diff = HomeConfig;

    fn diff(&self, base: &HomeConfig) -> protocol::MutationOutcome<HomeConfig> {
        let mut next = base.clone();
        match self {
            HomeConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 } => {
                if directory_projection_state_is_valid(directory_json, session_binding_sha256, *authorization_generation, receipt_sha256) {
                    next.directory_json = directory_json.clone();
                    next.directory_session_binding_sha256 = session_binding_sha256.clone();
                    next.directory_authorization_generation = *authorization_generation;
                    next.directory_receipt_sha256 = receipt_sha256.clone();
                }
            }
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &HomeConfig) -> Vec<Self> {
        vec![HomeConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 📏️RetainedLimits
/// 📏️ One sealed directory page is a `HostOnly` machine payload whose only real bound is the retained wire budget —
/// the hub pages the directory with `hasMore`, so a page is bounded by construction, and the 4 KiB public scalar cap
/// would refuse an ordinary page of a dozen spaces (ticket 26/09/18 S4). Shared by both Home surfaces.
pub const HOME_DIRECTORY_PAGE_BYTES: usize = 128 * 1024;
/// 📏️ Home's config lane is a ONE-ITEM retained lane, and `ArtifactStoreOneItemFootprint::is_admissible`
/// refuses any item declaring more than [`store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES`] (1 MiB). These
/// two constants were 4 MiB and 16 MiB, so `HomeConfigPreparationFactory::preflight`'s footprint could
/// never be admitted and every retained config gesture died with "one-item preparation footprint exceeds
/// its fixed item or byte capacity" — invisible while `applyDirectoryEventPage` was
/// `BatchOnlyPendingRewrite` and therefore never reached this lane at all (ticket 26/09/18 S4).
/// The directory projection they carry is a few KiB for an ordinary hub, and the hub pages it, so the
/// store's own ceiling is the honest budget rather than an aspirational one.
pub const HOME_CONFIG_BASE_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
pub const HOME_CONFIG_STEP_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;

/// ⏱️ The execution contract of every Home retained route, editor and viewer alike: one page per operation, the config
/// store's own step budget, resumable so a page that outlives one turn continues from its checkpoint.
pub fn home_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(HOME_DIRECTORY_PAGE_BYTES, 256, 1, HOME_CONFIG_STEP_BYTES, 7_500, 1, 1)
}
//#endregion 📏️RetainedLimits

//#region 📬️ConfigStorePreparation
/// 📬️ The ONE retained one-item preparation of the Home config lane, shared by BOTH Home surfaces: the editor and
/// the read-only viewer each fold sealed directory pages into their own config store through it, so a viewer
/// lists the same hub rows as the editor (ticket 26/09/23 S16).
pub struct HomeConfigPreparationFactory;

struct HomeConfigPreparation {
    base: Option<store::SnapshotRead<HomeConfig>>,
    mutation: Option<HomeConfigMutation>,
    description: Option<String>,
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
    config
        .directory_json
        .len()
        .saturating_add(config.directory_session_binding_sha256.len())
        .saturating_add(config.directory_receipt_sha256.len())
        .saturating_add(size_of_val(&config.directory_authorization_generation))
}

fn home_config_edit(forward: HomeConfigMutation, inverse: HomeConfigMutation, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<HomeConfigMutation> {
    let id = format!("space-home-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse: vec![inverse],
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
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
    let bytes = pack::to_json_string(&dsl::ToValue::to_value(edit)).len();
    if bytes > HOME_CONFIG_STEP_BYTES {
        return Err("Space Home config edit exceeds its serialized byte envelope".to_string());
    }
    Ok(bytes)
}

impl store::ArtifactStoreOneItemPreparationFactory<HomeConfig, HomeConfigMutation> for HomeConfigPreparationFactory {
    fn preflight(&self, mutation: &HomeConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let (mutation_bytes, maximum_bytes) = match mutation {
            HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 }
                if *authorization_generation > 0
                    && directory_json.len() <= HOME_CONFIG_BASE_BYTES
                    && directory_projection_state_is_valid(directory_json, session_binding_sha256, *authorization_generation, receipt_sha256) =>
            {
                (directory_json.len().saturating_add(session_binding_sha256.len()).saturating_add(receipt_sha256.len()).saturating_add(8), HOME_CONFIG_BASE_BYTES + 136)
            }
            _ => return Err("Space Home config preparation rejects non-retained mutations".into()),
        };
        if lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Space Home config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 3, retained_bytes: HOME_CONFIG_STEP_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>> {
        let (mutation_bytes, maximum_bytes) = match &request.mutation {
            HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 }
                if *authorization_generation > 0
                    && directory_json.len() <= HOME_CONFIG_BASE_BYTES
                    && directory_projection_state_is_valid(directory_json, session_binding_sha256, *authorization_generation, receipt_sha256) =>
            {
                (directory_json.len().saturating_add(session_binding_sha256.len()).saturating_add(receipt_sha256.len()).saturating_add(8), HOME_CONFIG_BASE_BYTES + 136)
            }
            _ => return Err(request),
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes || request.description.as_ref().is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(HomeConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
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
            let mut post = base.clone();
            let inverse = match &mutation {
                HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 } => HomeConfigMutation::ReplaceDirectoryProjection {
                    directory_json: std::mem::replace(&mut post.directory_json, directory_json.clone()),
                    session_binding_sha256: std::mem::replace(&mut post.directory_session_binding_sha256, session_binding_sha256.clone()),
                    authorization_generation: std::mem::replace(&mut post.directory_authorization_generation, *authorization_generation),
                    receipt_sha256: std::mem::replace(&mut post.directory_receipt_sha256, receipt_sha256.clone()),
                },
                _ => return Err("Space Home config preparation received a non-retained mutation".into()),
            };
            self.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if self.sealed_candidate.is_none() {
            let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Space Home config preparation lost its candidate".to_string())?;
            let authority = self.authority.as_ref().ok_or_else(|| "Space Home config preparation lost its Store authority".to_string())?;
            self.sealed_candidate = Some((post, home_config_edit(forward, inverse, self.description.take(), authority)));
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
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        // 🧹️ One retained owner per granted page, never more bytes than the page granted — the same
        // reasoning as `advance` above: an owner that answers `Blocked` until it is handed its whole
        // declared envelope never closes under the framework's 4 KiB pumps.
        if self.prepared.take().is_some() || self.sealed_candidate.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: grant.maximum_bytes }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Space Home config preparation could not return its exact base root".into()); }
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
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.sealed_candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigStorePreparation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
