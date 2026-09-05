//! ⚙️ S Home launcher editor — `ArtifactEditor::Config` + its operation enum (constitutional: engine + op,
//! merged at app level per the per-app recipe: `Config`/`ConfigMutation` are inherently app-scoped,
//! never artifact-scoped).
//!
//! 🕳️ `SHomeSnapshot` is a two-field counter document (`schema` + `catalog_generation`) with no tree
//! structure, id generation, or media import/export of its own — the original monolith never factored
//! out a pure `empty_home_document()`/compute helper (every call site builds the literal
//! `SHomeSnapshot { schema: "s.home".into(), catalog_generation: N }` directly), so this app has no
//! document-side `⚙️engine` node under `🗿️artifacts/🏠️home`. What this file owns is `HomeConfig` — the
//! Home launcher's real `ArtifactEditor::Config`: the one `view_state.locale` read the editor's home
//! labels actually need, plus the `active_panel_tab` action, the folded hub directory read model
//! (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C1/§C6) and the signed-in
//! client identity.

use std::collections::BTreeMap;
use semio_framework_plugin::Fault;

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
#[value(deny_unknown_fields)]
struct DirectorySpaceWire {
    view: store::os_directory::SpaceView,
    members: Vec<store::os_directory::MemberView>,
    documents: Vec<store::os_directory::DocumentDescriptor>,
}

#[derive(value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
struct DirectoryReadModelWire {
    spaces: BTreeMap<String, DirectorySpaceWire>,
    cursor: u64,
    users: BTreeMap<String, store::os_directory::UserView>,
}

/// 📇️ Encodes a `DirectoryReadModel` as the `directory_json` DSL field's wire value.
fn directory_to_json(model: &store::os_directory::DirectoryReadModel) -> String {
    let wire = DirectoryReadModelWire {
        spaces: model
            .spaces
            .iter()
            .map(|(id, space)| (id.clone(), DirectorySpaceWire { view: space.view.clone(), members: space.members.clone(), documents: space.documents.clone() }))
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
            .map(|(id, space)| (id, store::os_directory::DirectorySpace { view: space.view, members: space.members, documents: space.documents }))
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
    /// 👁️ Active launcher panel tab.
    pub active_panel_tab: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
    /// 📇️ JSON-serialized `DirectoryReadModel` (see `🔖️DirectoryJson` above) — folded here by
    /// `HomeConfigMutation::FoldDirectoryEvent` as `/directory/ws` events arrive; read via `directory()`.
    /// No optimistic mutation (contract §C6): the ONLY writer is the fold over hub-confirmed events.
    pub directory_json: String,
    /// 🔐️ Opaque digest binding the accepted page frontier to one authenticated hub session.
    pub directory_session_binding_sha256: String,
    /// 🛂️ Authorization generation under which the current projection was filtered.
    pub directory_authorization_generation: u64,
    /// 🧾️ Receipt of the last durably accepted directory page.
    pub directory_receipt_sha256: String,
    /// 🪪️ The signed-in client's directory user id (`os.config.identity`'s `userId`); empty while
    /// offline/no identity yet. Threads into `SpaceUser` ownership on space creation.
    pub client_id: String,
    /// 🪪️ The signed-in client's display name.
    pub client_name: String,
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

    /// 📄️ Applies one authenticated page to a replacement config without exposing partial folds.
    pub fn apply_directory_event_page(&self, page: &store::os_directory::DirectoryEventPageV1) -> Result<Self, Fault> {
        page.validate().map_err(|_| Fault::from("s.home.directory-event-page-invalid"))?;
        let current = self.directory()?;
        let same_authority = self.directory_session_binding_sha256 == page.session_binding_sha256
            && self.directory_authorization_generation == page.authorization_generation;
        if same_authority && current.cursor == page.through_seq_inclusive && self.directory_receipt_sha256 == page.receipt_sha256 {
            return Ok(self.clone());
        }
        let mut directory = if same_authority {
            if page.after_seq_exclusive != current.cursor {
                return Err(Fault::from("s.home.directory-event-page-frontier-race"));
            }
            current
        } else {
            if page.after_seq_exclusive != 0 {
                return Err(Fault::from("s.home.directory-event-page-rebootstrap-required"));
            }
            store::os_directory::DirectoryReadModel::default()
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

impl Default for HomeConfig {
    fn default() -> Self {
        Self {
            active_panel_tab: String::new(),
            locale: "en-US".into(),
            directory_json: directory_to_json(&store::os_directory::DirectoryReadModel::default()),
            directory_session_binding_sha256: String::new(),
            directory_authorization_generation: 0,
            directory_receipt_sha256: String::new(),
            client_id: String::new(),
            client_name: String::new(),
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
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    /// 📇️ Folds one hub-confirmed `DirectoryEvent` (JSON-encoded, contract §C1) into `directory_json`
    /// — the SOLE writer of the directory read model (contract §C6: no optimistic mutation).
    #[dsl(key = "fold-directory-event")]
    FoldDirectoryEvent { event_json: String },
    /// 📄️ Atomically replaces the page-derived projection and its authenticated resume authority.
    #[dsl(key = "replace-directory-projection")]
    ReplaceDirectoryProjection {
        directory_json: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        receipt_sha256: String,
    },
    /// 🪪️ Sets the signed-in client identity (contract §C3 identity bootstrap).
    #[dsl(key = "set-client")]
    SetClient { client_id: String, client_name: String },
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
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-snapshot", semantic_kind: "set-snapshot", display_name: "Set Snapshot", emoji: "⚙️", aggregate_variant: "Snapshot", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-active-panel-tab", semantic_kind: "set-active-panel-tab", display_name: "Set Active Panel Tab", emoji: "⚙️", aggregate_variant: "SetActivePanelTab", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-locale", semantic_kind: "set-locale", display_name: "Set Locale", emoji: "⚙️", aggregate_variant: "SetLocale", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️fold-directory-event", semantic_kind: "fold-directory-event", display_name: "Fold Directory Event", emoji: "⚙️", aggregate_variant: "FoldDirectoryEvent", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️replace-directory-projection", semantic_kind: "replace-directory-projection", display_name: "Replace Directory Projection", emoji: "📄️", aggregate_variant: "ReplaceDirectoryProjection", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-client", semantic_kind: "set-client", display_name: "Set Client", emoji: "⚙️", aggregate_variant: "SetClient", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            HomeConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            HomeConfigMutation::SetActivePanelTab { .. } => &Self::DESCRIPTORS[1],
            HomeConfigMutation::SetLocale { .. } => &Self::DESCRIPTORS[2],
            HomeConfigMutation::FoldDirectoryEvent { .. } => &Self::DESCRIPTORS[3],
            HomeConfigMutation::ReplaceDirectoryProjection { .. } => &Self::DESCRIPTORS[4],
            HomeConfigMutation::SetClient { .. } => &Self::DESCRIPTORS[5],
        }
    }

    type Diff = HomeConfig;

    fn diff(&self, base: &HomeConfig) -> protocol::MutationOutcome<HomeConfig> {
        let mut next = base.clone();
        match self {
            HomeConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            HomeConfigMutation::SetActivePanelTab { tab_id } => next.active_panel_tab = tab_id.clone(),
            HomeConfigMutation::SetLocale { value } => next.locale = value.clone(),
            HomeConfigMutation::FoldDirectoryEvent { event_json } => {
                if let (Ok(event), Ok(directory)) = (pack::from_json_str::<store::os_directory::DirectoryEvent>(event_json), next.directory()) {
                    next.directory_json = directory_to_json(&store::os_directory::fold(directory, &event));
                }
            }
            HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 } => {
                if directory_projection_state_is_valid(directory_json, session_binding_sha256, *authorization_generation, receipt_sha256) {
                    next.directory_json = directory_json.clone();
                    next.directory_session_binding_sha256 = session_binding_sha256.clone();
                    next.directory_authorization_generation = *authorization_generation;
                    next.directory_receipt_sha256 = receipt_sha256.clone();
                }
            }
            HomeConfigMutation::SetClient { client_id, client_name } => {
                next.client_id = client_id.clone();
                next.client_name = client_name.clone();
            }
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &HomeConfig) -> Vec<Self> {
        vec![HomeConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    #[semio_framework_async_macros::async_test]
    async fn home_config_default_locale_is_english() {
        let config = HomeConfig::default();
        assert_eq!(config.locale, "en-US");
        assert!(config.active_panel_tab.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn home_config_dsl_text_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&HomeConfig::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn home_config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::Snapshot { config: HomeConfig::default() });
        store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::SetActivePanelTab { tab_id: "tab-1".into() });
        store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::SetLocale { value: "de".into() });
        store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::FoldDirectoryEvent { event_json: "{}".into() });
        store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::ReplaceDirectoryProjection {
            directory_json: directory_to_json(&store::os_directory::DirectoryReadModel::default()),
            session_binding_sha256: "a".repeat(64),
            authorization_generation: 1,
            receipt_sha256: "b".repeat(64),
        });
        store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::SetClient { client_id: "u1".into(), client_name: "Ada".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn home_config_default_directory_is_empty() {
        let model = HomeConfig::default().directory().expect("default directory projection");
        assert!(model.spaces.is_empty());
        assert_eq!(model.cursor, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn fold_directory_event_updates_the_read_model() {
        let config = HomeConfig::default();
        let event_json = pack::json!({
            "seq": 1,
            "id": "evt-1",
            "hlc": { "physicalMs": 0, "logical": 0 },
            "actor": { "kind": "user", "id": "user:u1#s1" },
            "spaceId": "sp-1",
            "body": { "kind": "space.created", "spaceId": "sp-1", "name": "Atelier", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1" },
            "recordedAtMs": 1000
        })
        .to_string();
        let next = HomeConfigMutation::FoldDirectoryEvent { event_json }.diff(&config).diff().clone();
        let model = next.directory().expect("folded directory projection");
        assert_eq!(model.cursor, 1);
        let space = model.spaces.get("sp-1").expect("space folded");
        assert_eq!(space.view.name, "Atelier");
    }

    #[semio_framework_async_macros::async_test]
    async fn fold_directory_event_ignores_malformed_json() {
        let config = HomeConfig::default();
        let next = HomeConfigMutation::FoldDirectoryEvent { event_json: "not json".into() }.diff(&config).diff().clone();
        assert_eq!(next.directory_json, config.directory_json, "malformed events never panic and never change the model");
    }

    #[semio_framework_async_macros::async_test]
    async fn directory_projection_round_trip_preserves_documents_and_rejects_corruption() {
        let fixture: pack::JsonValue = pack::from_json_str(include_str!("🧪️fixtures/📇️projection-persistence-v1/🔣️.json")).expect("language-neutral projection fixture");
        let wire = fixture.get("wire").expect("fixture wire").to_string();
        let model = directory_from_json(&wire).expect("fixture directory projection");
        let document_ids = model.spaces.values().flat_map(|space| space.documents.iter().map(|document| document.document_id.as_str())).collect::<Vec<_>>();
        assert_eq!(document_ids, vec!["document-雪"]);
        let encoded: pack::JsonValue = pack::from_json_str(&directory_to_json(&model)).expect("encoded projection JSON");
        assert_eq!(encoded, fixture["wire"]);
        for malformed in fixture["malformed"].as_array().expect("malformed cases") {
            assert!(directory_from_json(malformed.as_str().expect("malformed text")).is_err());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn set_client_updates_identity_fields() {
        let config = HomeConfig::default();
        let next = HomeConfigMutation::SetClient { client_id: "u1".into(), client_name: "Ada".into() }.diff(&config).diff().clone();
        assert_eq!(next.client_id, "u1");
        assert_eq!(next.client_name, "Ada");
    }

    #[semio_framework_async_macros::async_test]
    async fn home_config_operation_round_trips_via_apply_and_backwards() {
        let config = HomeConfig::default();
        let operation = HomeConfigMutation::SetLocale { value: "de".into() };
        let next = operation.diff(&config).diff().clone();
        assert_eq!(next.locale, "de");
        let backwards = operation.inverse(&config);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, config);
    }
}
//#endregion 🧪️Tests
