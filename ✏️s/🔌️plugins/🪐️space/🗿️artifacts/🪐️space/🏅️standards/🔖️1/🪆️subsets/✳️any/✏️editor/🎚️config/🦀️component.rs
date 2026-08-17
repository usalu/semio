//! 🎚️ SpaceIndexEditor view-state — the folded-directory read model slice (members/visibility) plus
//! per-artifact live presence, both host-pushed via the `fold-directory-events`/`presence-heartbeat`
//! commands (never duplicated into the shared `SSpaceSnapshot` document — contract §C4: "space
//! name/kind/visibility/members are directory-owned ... never duplicated into this document"). Local
//! view state only, mirrors `DrawConfig`'s handcrafted DSL/pack codec shape.

use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Member
/// 🧑️ One space member, projected from `semio_framework_os::os_directory::MemberView` into the
/// space app's own local view-state vocabulary (`role` kept as the wire string `"author"`/
/// `"spectator"` rather than re-importing the directory crate's enum into render code).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct SpaceIndexMember {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub role: String,
}
//#endregion 🔖️Member

//#region 🔖️Presence
/// 👥️ Live peers on one artifact's documents (all surfaces/documents of that artifact, folded to a
/// flat actor-id list) — `actors_csv` avoids nesting `Vec<String>` inside a `#[dsl(table)]` row
/// (unproven by any existing facet in this tree); split on `,` for display.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct SpaceIndexArtifactPresence {
    pub artifact_id: String,
    pub actors_csv: String,
}

impl SpaceIndexArtifactPresence {
    /// 🪪️ The live actor ids for this artifact, empty-string-safe.
    pub fn actor_ids(&self) -> Vec<&str> {
        if self.actors_csv.is_empty() {
            Vec::new()
        } else {
            self.actors_csv.split(',').collect()
        }
    }
}
//#endregion 🔖️Presence

//#region 🔖️Config
/// 🎚️ `SpaceIndexEditor`'s real `ArtifactApp::Config` — whole-record, DSL/pack codec handcrafted
/// (mirrors `SSpaceSnapshot`'s own handcrafted pair, `🧬️schema/📸️snapshot/🦀️component.rs`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "sspacecfg")]
#[dsl(layout = "lines")]
pub struct SpaceIndexConfig {
    pub visibility: String,
    #[dsl(table)]
    pub members: Vec<SpaceIndexMember>,
    #[dsl(table)]
    pub presence: Vec<SpaceIndexArtifactPresence>,
}

impl Default for SpaceIndexConfig {
    fn default() -> Self {
        Self { visibility: "private".into(), members: Vec::new(), presence: Vec::new() }
    }
}

impl SpaceIndexConfig {
    /// 👥️ The live actor ids on `artifact_id`'s documents, empty when nothing is folded in yet.
    pub fn presence_for(&self, artifact_id: &str) -> Vec<&str> {
        self.presence.iter().find(|row| row.artifact_id == artifact_id).map(SpaceIndexArtifactPresence::actor_ids).unwrap_or_default()
    }
}

//#region 🔖️ArtifactCodec
impl store::ArtifactDsl for SpaceIndexConfig {
    const EXTENSION: &'static str = "sspacecfg";
    fn envelope_id() -> &'static str {
        "s.space.config"
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

impl store::ArtifactPack for SpaceIndexConfig {
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

store::impl_whole_record_config!(SpaceIndexConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutation
/// 🧮️ Whole-record replace — the config is always folded/derived host-side (directory events,
/// presence heartbeats) and pushed down as one snapshot, mirrors `DrawConfigMutation::Snapshot`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SpaceIndexConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SpaceIndexConfig,
    },
}

impl Mutation<SpaceIndexConfig> for SpaceIndexConfigMutation {
    type Diff = SpaceIndexConfig;

    fn diff(&self, _base: &SpaceIndexConfig) -> protocol::MutationOutcome<SpaceIndexConfig> {
        let SpaceIndexConfigMutation::Snapshot { config } = self;
        protocol::MutationOutcome::new(config.clone())
    }

    fn inverse(&self, base: &SpaceIndexConfig) -> Vec<Self> {
        vec![SpaceIndexConfigMutation::Snapshot { config: base.clone() }]
    }
}

//#region 🔖️OpCodec
impl protocol::OpText for SpaceIndexConfigMutation {
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

impl protocol::OpBinary for SpaceIndexConfigMutation {
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
//#endregion 🔖️ConfigMutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_private_with_no_members_or_presence() {
        let config = SpaceIndexConfig::default();
        assert_eq!(config.visibility, "private");
        assert!(config.members.is_empty());
        assert!(config.presence.is_empty());
    }

    #[test]
    fn presence_for_splits_the_csv() {
        let config = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1,user:2".into() }], ..Default::default() };
        assert_eq!(config.presence_for("artifact-1"), vec!["user:1", "user:2"]);
        assert!(config.presence_for("ghost").is_empty());
    }

    #[test]
    fn config_dsl_round_trips() {
        let config = SpaceIndexConfig {
            visibility: "public".into(),
            members: vec![SpaceIndexMember { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into(), role: "author".into() }],
            presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }],
        };
        store::os_store::test_support::assert_dsl_round_trip(&config);
    }

    #[test]
    fn config_mutation_snapshot_replaces_wholesale_and_inverse_restores() {
        let base = SpaceIndexConfig::default();
        let next = SpaceIndexConfig { visibility: "public".into(), ..Default::default() };
        let mutation = SpaceIndexConfigMutation::Snapshot { config: next.clone() };
        let forward = mutation.diff(&base).diff().clone();
        assert_eq!(forward, next);
        let backwards = mutation.inverse(&base);
        assert_eq!(backwards, vec![SpaceIndexConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward).diff().clone();
        assert_eq!(restored, base);
    }

    #[test]
    fn config_mutation_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceIndexConfigMutation::Snapshot { config: SpaceIndexConfig::default() });
    }
}
//#endregion 🧪️Tests
