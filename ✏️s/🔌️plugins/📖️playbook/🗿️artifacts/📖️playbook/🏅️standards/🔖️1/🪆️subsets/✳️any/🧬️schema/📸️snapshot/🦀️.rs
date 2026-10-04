//! 🧬️ Playbook snapshot schema — artifact-lane fields only.
//!
//! The steps live in the composed `flow` child (see the artifact root's `🔖️ContentBridge`), never on this struct: the parent
//! carries the child's coordinate, its text and pack carry nothing of the child's content, and every reader composes parent +
//! child on read (design §20.15 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted playbook document snapshot (persistent fields of the artifact). `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: crate::PlaybookFlowChild,
}

impl Default for PlaybookSnapshot {
    fn default() -> Self {
        let kernel = crate::playbook::empty_playbook_snapshot();
        Self { schema: kernel.schema, id: kernel.id, version: kernel.version, title: kernel.title, flow: crate::playbook_flow_child(crate::PLAYBOOK_GENESIS_FLOW_ID) }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived — mirrors the sibling `PlaybookArtifact` impl one region up
/// (`../🦀️.rs`'s `🔖️ValueCodec`): `flow` is a `store::ArtifactChild<S>` composed-artifact handle,
/// bridged through the pre-existing `to_dsl_value`/`from_dsl_value` seam instead of widening the
/// derive macro to understand child-slot handles.
impl semio_framework_value::ToValue for PlaybookSnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)),
            ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)),
            ("version".to_string(), semio_framework_value::ToValue::to_value(&self.version)),
            ("title".to_string(), semio_framework_value::ToValue::to_value(&self.title)),
            ("flow".to_string(), semio_framework_value::ToValue::to_value(&self.flow)),
        ])
    }
}
impl semio_framework_value::FromValue for PlaybookSnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            schema: semio_framework_value::FromValue::from_value(field("schema")?)?,
            id: semio_framework_value::FromValue::from_value(field("id")?)?,
            version: semio_framework_value::FromValue::from_value(field("version")?)?,
            title: semio_framework_value::FromValue::from_value(field("title")?)?,
            flow: semio_framework_value::FromValue::from_value(field("flow")?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️PackRecord
/// 📦️ Derived pack and text record of a `PlaybookSnapshot`: its scalars and the `flow` child's coordinate — no child content.
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension = "playbook")]
struct PlaybookPackRecord {
    schema: String,
    id: String,
    version: String,
    title: Option<String>,
    flow: crate::PlaybookFlowChild,
}

impl PlaybookPackRecord {
    fn from_snapshot(snapshot: &PlaybookSnapshot) -> Self {
        Self { schema: snapshot.schema.clone(), id: snapshot.id.clone(), version: snapshot.version.clone(), title: snapshot.title.clone(), flow: snapshot.flow.clone() }
    }

    fn into_snapshot(self) -> PlaybookSnapshot {
        PlaybookSnapshot { schema: self.schema, id: self.id, version: self.version, title: self.title, flow: self.flow }
    }
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PlaybookSnapshot {
    const EXTENSION: &'static str = "playbook";
    fn envelope_id() -> &'static str {
        "playbook.playbook"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &PlaybookPackRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Ok(PlaybookPackRecord::__dsl_from_record(&record)?.into_snapshot())
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&PlaybookPackRecord::from_snapshot(self).__dsl_to_record(), &PlaybookPackRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PlaybookSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&PlaybookPackRecord::__dsl_spec(), &PlaybookPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &PlaybookPackRecord::__dsl_spec(), options)?;
        Ok(PlaybookPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot())
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(PlaybookPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ExternalBridges
/// 📖️ Parses `.playbook` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_playbook_dsl(text: &str) -> Result<PlaybookSnapshot, String> {
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`PlaybookSnapshot`] back to `.playbook` DSL text under a name an external caller can reach, paired
/// with [`parse_playbook_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_playbook_dsl(snapshot: &PlaybookSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;
