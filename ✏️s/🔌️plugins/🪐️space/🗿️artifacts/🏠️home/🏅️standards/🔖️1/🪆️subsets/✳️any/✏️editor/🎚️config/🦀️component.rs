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

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
#[derive(Serialize, Deserialize)]
struct DirectorySpaceWire {
    view: store::os_directory::SpaceView,
    members: Vec<store::os_directory::MemberView>,
}

#[derive(Serialize, Deserialize, Default)]
struct DirectoryReadModelWire {
    #[serde(default)]
    spaces: BTreeMap<String, DirectorySpaceWire>,
    #[serde(default)]
    cursor: u64,
    #[serde(default)]
    users: BTreeMap<String, store::os_directory::UserView>,
}

/// 📇️ Encodes a `DirectoryReadModel` as the `directory_json` DSL field's wire value.
async fn directory_to_json(model: &store::os_directory::DirectoryReadModel) -> String {
    let wire = DirectoryReadModelWire { spaces: model.spaces.iter().map(|(id, space)| (id.clone(), DirectorySpaceWire { view: space.view.clone(), members: space.members.clone() })).collect(), cursor: model.cursor, users: model.users.clone() };
    serde_json::to_string(&wire).unwrap_or_else(|_| "{}".into())
}

/// 📇️ Decodes `directory_json` back into a `DirectoryReadModel` — malformed/empty input yields the
/// empty model (never panics: this reads persisted config text, which must never crash a boot).
async fn directory_from_json(json: &str) -> store::os_directory::DirectoryReadModel {
    let wire: DirectoryReadModelWire = serde_json::from_str(json).unwrap_or_default();
    store::os_directory::DirectoryReadModel { spaces: wire.spaces.into_iter().map(|(id, space)| (id, store::os_directory::DirectorySpace { view: space.view, members: space.members })).collect(), cursor: wire.cursor, users: wire.users }
}
//#endregion 🔖️DirectoryJson

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
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
    /// 🪪️ The signed-in client's directory user id (`os.config.identity`'s `userId`); empty while
    /// offline/no identity yet. Threads into `SpaceUser` ownership on space creation.
    pub client_id: String,
    /// 🪪️ The signed-in client's display name.
    pub client_name: String,
}

impl HomeConfig {
    /// 📇️ Decodes the folded hub directory read model.
    pub async fn directory(&self) -> store::os_directory::DirectoryReadModel {
        directory_from_json(&self.directory_json)
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for HomeConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        "home.config"
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
impl store::ArtifactPack for HomeConfig {
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

impl Default for HomeConfig {
    fn default() -> Self {
        Self { active_panel_tab: String::new(), locale: "en-US".into(), directory_json: directory_to_json(&store::os_directory::DirectoryReadModel::default()), client_id: String::new(), client_name: String::new() }
    }
}

store::impl_whole_record_config!(HomeConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ `HomeConfig`'s operation enum — mirrors `engine::space::config::SpaceConfigMutation`'s
/// whole-record-diff design (see its doc comment for the full rationale).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
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
    /// 🪪️ Sets the signed-in client identity (contract §C3 identity bootstrap).
    #[dsl(key = "set-client")]
    SetClient { client_id: String, client_name: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for HomeConfigMutation {
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
impl protocol::OpBinary for HomeConfigMutation {
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

impl protocol::Mutation<HomeConfig> for HomeConfigMutation {
    type Diff = HomeConfig;

    async fn diff(&self, base: &HomeConfig) -> protocol::MutationOutcome<HomeConfig> {
        let mut next = base.clone();
        match self {
            HomeConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            HomeConfigMutation::SetActivePanelTab { tab_id } => next.active_panel_tab = tab_id.clone(),
            HomeConfigMutation::SetLocale { value } => next.locale = value.clone(),
            HomeConfigMutation::FoldDirectoryEvent { event_json } => {
                if let Ok(event) = serde_json::from_str::<store::os_directory::DirectoryEvent>(event_json) {
                    next.directory_json = directory_to_json(&store::os_directory::fold(next.directory(), &event));
                }
            }
            HomeConfigMutation::SetClient { client_id, client_name } => {
                next.client_id = client_id.clone();
                next.client_name = client_name.clone();
            }
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &HomeConfig) -> Vec<Self> {
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
        store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::SetClient { client_id: "u1".into(), client_name: "Ada".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn home_config_default_directory_is_empty() {
        let model = HomeConfig::default().directory();
        assert!(model.spaces.is_empty());
        assert_eq!(model.cursor, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn fold_directory_event_updates_the_read_model() {
        let config = HomeConfig::default();
        let event_json = serde_json::json!({
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
        let model = next.directory();
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
