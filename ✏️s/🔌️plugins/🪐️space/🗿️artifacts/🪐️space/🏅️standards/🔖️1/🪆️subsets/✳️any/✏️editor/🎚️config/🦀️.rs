//! 🎚️ SpaceIndexEditor view-state — the folded-directory read model slice (members/visibility) plus
//! per-artifact live presence, both host-pushed via the `fold-directory-events`/`presence-heartbeat`
//! commands (never duplicated into the shared `SSpaceSnapshot` document — contract §C4: "space
//! name/kind/visibility/members are directory-owned ... never duplicated into this document"). Local
//! view state only, mirrors `DrawConfig`'s handcrafted DSL/pack codec shape.

use protocol::Mutation;

use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
use semio_framework_os_kernel::os_directory::DirectoryIndexedDocumentViewV1;

//#region 🔖️Member
/// 🧑️ One space member, projected from `semio_framework_os::os_directory::MemberView` into the
/// space app's own local view-state vocabulary (`role` kept as the wire string `"author"`/
/// `"spectator"` rather than re-importing the directory crate's enum into render code).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
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
/// (mirrors `SSpaceSnapshot`'s own handcrafted pair, `🧬️schema/📸️snapshot/🦀️.rs`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "sspacecfg")]
#[dsl(layout = "lines")]
pub struct SpaceIndexConfig {
    pub visibility: String,
    #[dsl(table)]
    pub members: Vec<SpaceIndexMember>,
    #[dsl(table)]
    pub indexed_artifacts: Vec<SpaceArtifactRow>,
    #[dsl(table)]
    pub presence: Vec<SpaceIndexArtifactPresence>,
}

impl Default for SpaceIndexConfig {
    fn default() -> Self {
        Self { visibility: "private".into(), members: Vec::new(), indexed_artifacts: Vec::new(), presence: Vec::new() }
    }
}

impl SpaceIndexConfig {
    /// 📇️ Projects one Directory-owned indexed document into the Space app's bounded read-only row.
    pub fn indexed_artifact_from_directory(row: &DirectoryIndexedDocumentViewV1) -> Option<SpaceArtifactRow> {
        let created_at_ms = u64::try_from(row.created_at_ms).ok()?;
        Some(SpaceArtifactRow {
            id: row.descriptor.document_id.clone(),
            name: row.entry.name.clone(),
            kind_id: row.descriptor.artifact_kind.clone(),
            schema: row.descriptor.artifact_schema.clone(),
            dialect: SpaceArtifactDialect { artifact_kind: row.entry.dialect.artifact_kind.clone(), standard: row.entry.dialect.standard.clone(), subset: row.entry.dialect.subset.clone() },
            created_at_ms,
            created_by: row.created_by.clone(),
            updated_at_ms: created_at_ms,
            updated_by: row.created_by.clone(),
        })
    }

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

impl store::ArtifactPack for SpaceIndexConfig {
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

impl store::ConfigRecord for SpaceIndexConfig {}

/// 📇️ The directory-owned slice of the config (visibility, members, indexed documents): what a directory fold replaces as one entity.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexDirectoryProjection {
    pub visibility: String,
    #[dsl(table)]
    pub members: Vec<SpaceIndexMember>,
    #[dsl(table)]
    pub indexed_artifacts: Vec<SpaceArtifactRow>,
}

/// 🩹 Patch of one presence row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexPresencePatch {
    pub actors_csv: Option<String>,
}

protocol::list_delta! {
    /// 📍 Positional row delta of the live presence rows, which stay ordered by artifact id so a set inserts at its sorted slot and a clear's inverse restores exactly that slot.
    pub SpaceIndexPresenceDelta { removal: SpaceIndexPresenceRemoval, insertion: SpaceIndexPresenceInsertion, relocation: SpaceIndexPresenceRelocation, modification: SpaceIndexPresenceModification, row: SpaceIndexArtifactPresence, patch: SpaceIndexPresencePatch, key: artifact_id, values_only }
}

/// 🔺️ Sparse field delta over [`SpaceIndexConfig`]; the directory slots are whole-entity replacements (their kind replaces exactly that
/// slice), the presence slot is a positional row delta.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<SpaceIndexMember>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub indexed_artifacts: Option<Vec<SpaceArtifactRow>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub presence: Option<SpaceIndexPresenceDelta>,
}


impl protocol::list_delta::RowPatch<SpaceIndexArtifactPresence> for SpaceIndexPresencePatch {
    fn commit_into(&self, row: &mut SpaceIndexArtifactPresence, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        let patch = self;
        if let Some(actors_csv) = &patch.actors_csv {
            row.actors_csv = actors_csv.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let earlier = self.clone();
        let first = &earlier;
        let later = &later;
        *self = SpaceIndexPresencePatch { artifact_id: first.artifact_id.clone(), actors_csv: later.actors_csv.clone().or_else(|| first.actors_csv.clone()) };
    }
    fn inverse(&self, row: &SpaceIndexArtifactPresence) -> Self {
        let patch = self;
        let base = row;
        SpaceIndexPresencePatch { actors_csv: patch.actors_csv.as_ref().map(|_| base.actors_csv.clone()) }
    }
    fn is_empty(&self) -> bool {
        let patch = self;
        patch.actors_csv.is_none()
    }
}

impl protocol::MutationDiff<SpaceIndexConfig> for SpaceIndexConfigDiff {
    fn apply(&self, base: &SpaceIndexConfig, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SpaceIndexConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.visibility {
            next.visibility = value.clone();
        }
        if let Some(value) = &self.members {
            next.members = value.clone();
        }
        if let Some(value) = &self.indexed_artifacts {
            next.indexed_artifacts = value.clone();
        }
        if let Some(delta) = &self.presence {
            next.presence = delta.commit_onto(&next.presence, capability).map_err(|error| error.under(["presence"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.visibility.is_some() {
            self.visibility = other.visibility;
        }
        if other.members.is_some() {
            self.members = other.members;
        }
        if other.indexed_artifacts.is_some() {
            self.indexed_artifacts = other.indexed_artifacts;
        }
        self.presence = match (self.presence.take(), other.presence) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<SpaceIndexConfig> for SpaceIndexConfigDiff {
    fn inverse(&self, base: &SpaceIndexConfig) -> Self {
        Self {
            visibility: self.visibility.as_ref().map(|_| base.visibility.clone()),
            members: self.members.as_ref().map(|_| base.members.clone()),
            indexed_artifacts: self.indexed_artifacts.as_ref().map(|_| base.indexed_artifacts.clone()),
            presence: self.presence.as_ref().map(|delta| delta.inverse(&base.presence)),
        }
    }
    fn is_empty(&self) -> bool {
        self.visibility.is_none() && self.members.is_none() && self.indexed_artifacts.is_none() && self.presence.as_ref().is_none_or(SpaceIndexPresenceDelta::is_empty)
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigMutation
/// 🧮️ The config's mutation vocabulary: the host-folded directory slice replaces as one entity, and each presence heartbeat sets
/// (or clears) the live actors of one artifact.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum SpaceIndexConfigMutation {
    #[dsl(key = "directory-projection")]
    ReplaceDirectoryProjection {
        #[dsl(block)]
        projection: SpaceIndexDirectoryProjection,
    },
    #[dsl(key = "artifact-presence")]
    SetArtifactPresence { artifact_id: String, actors_csv: String },
    #[dsl(key = "clear-artifact-presence")]
    ClearArtifactPresence { artifact_id: String },
}

impl Mutation<SpaceIndexConfig> for SpaceIndexConfigMutation {
    type Diff = SpaceIndexConfigDiff;

    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate — one
    /// entry per variant, in declaration order.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️replace-directory-projection",
            semantic_kind: "replace-directory-projection",
            display_name: "Replace Directory Projection",
            emoji: "⚙️",
            aggregate_variant: "ReplaceDirectoryProjection",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-artifact-presence",
            semantic_kind: "set-artifact-presence",
            display_name: "Set Artifact Presence",
            emoji: "⚙️",
            aggregate_variant: "SetArtifactPresence",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️clear-artifact-presence",
            semantic_kind: "clear-artifact-presence",
            display_name: "Clear Artifact Presence",
            emoji: "⚙️",
            aggregate_variant: "ClearArtifactPresence",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::ReplaceDirectoryProjection { .. } => &Self::DESCRIPTORS[0],
            Self::SetArtifactPresence { .. } => &Self::DESCRIPTORS[1],
            Self::ClearArtifactPresence { .. } => &Self::DESCRIPTORS[2],
        }
    }

    fn diff(&self, base: &SpaceIndexConfig) -> protocol::MutationOutcome<SpaceIndexConfigDiff> {
        protocol::MutationOutcome::new(match self {
            Self::ReplaceDirectoryProjection { projection } => SpaceIndexConfigDiff {
                visibility: (base.visibility != projection.visibility).then(|| projection.visibility.clone()),
                members: (base.members != projection.members).then(|| projection.members.clone()),
                indexed_artifacts: (base.indexed_artifacts != projection.indexed_artifacts).then(|| projection.indexed_artifacts.clone()),
                presence: None,
            },
            Self::SetArtifactPresence { artifact_id, actors_csv } => {
                let delta = match base.presence.iter().find(|row| row.artifact_id == *artifact_id) {
                    Some(row) if row.actors_csv == *actors_csv => None,
                    Some(_) => Some(SpaceIndexPresenceDelta::modification(artifact_id.clone(), SpaceIndexPresencePatch { actors_csv: Some(actors_csv.clone()) })),
                    None => Some(SpaceIndexPresenceDelta::insertion(base.presence.partition_point(|row| row.artifact_id < *artifact_id), SpaceIndexArtifactPresence { artifact_id: artifact_id.clone(), actors_csv: actors_csv.clone() })),
                };
                SpaceIndexConfigDiff { presence: delta, ..Default::default() }
            }
            Self::ClearArtifactPresence { artifact_id } => SpaceIndexConfigDiff { presence: base.presence.iter().position(|row| row.artifact_id == *artifact_id).map(|index| SpaceIndexPresenceDelta::removal(&base.presence, index)), ..Default::default() },
        })
    }

    fn inverse(&self, base: &SpaceIndexConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(match self {
            Self::ReplaceDirectoryProjection { .. } => vec![Self::ReplaceDirectoryProjection { projection: SpaceIndexDirectoryProjection { visibility: base.visibility.clone(), members: base.members.clone(), indexed_artifacts: base.indexed_artifacts.clone() } }],
            Self::SetArtifactPresence { artifact_id, .. } | Self::ClearArtifactPresence { artifact_id } => match base.presence.iter().find(|row| row.artifact_id == *artifact_id) {
                Some(row) => vec![Self::SetArtifactPresence { artifact_id: artifact_id.clone(), actors_csv: row.actors_csv.clone() }],
                None if matches!(self, Self::SetArtifactPresence { .. }) => vec![Self::ClearArtifactPresence { artifact_id: artifact_id.clone() }],
                None => Vec::new(),
            },
        })
    }
}

//#region 🔖️OpCodec
impl protocol::OpText for SpaceIndexConfigMutation {
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

impl protocol::OpBinary for SpaceIndexConfigMutation {
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
//#endregion 🔖️ConfigMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
