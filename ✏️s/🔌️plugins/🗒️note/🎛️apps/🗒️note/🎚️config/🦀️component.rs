//! 🧮️ Note play app — view state (`NoteConfig`) and its operation enum (`NoteConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.note` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/camera/utility edits are VCS'd exactly like
//! document content.

use crate::artifacts::note::NoteCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ Note's real `DocumentApp::Config` — mirrors `shooting_engine::ShootingConfig`'s pilot shape.
/// Absorbs every field that used to live on the old ui crate's `NotePlayRuntime` (selection, hover, the
/// in-progress engagement-rename input, and the free/live canvas camera) plus the two `ViewModel`
/// fields the note UI actually reads (`locale`/`active_utility_id`) — see
/// `crate::apps::note::NotePlayApp::render`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(id = "note.config", layout = "lines")]
pub struct NoteConfig {
    /// 👁️ Selected block ids — was `NotePlayRuntime::selected_ids`.
    pub selected_block_ids: Vec<String>,
    /// 👁️ Hovered block id — was `NotePlayRuntime::hovered_id`.
    pub hovered_block_id: Option<String>,
    /// ✏️ In-progress engagement-rename input — was `NotePlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 📷️ The free/live canvas camera — session-only, never a document field. Was
    /// `NotePlayRuntime::camera`.
    #[dsl(block)]
    pub camera: NoteCamera,
    /// 🧰️ The active canvas utility (select/pencil/eraser/…) — was read off
    /// `view_state.active_utility_id` (host-pushed `ViewModel`, deleted by the pure-trait migration).
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for NoteConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for NoteConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️DocumentCodec


impl Default for NoteConfig {
    fn default() -> Self {
        Self { selected_block_ids: Vec::new(), hovered_block_id: None, engagement_input: String::new(), camera: NoteCamera::default(), active_utility_id: "selectDirect".into(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(NoteConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// @emoji 🧮️ `NoteConfig`'s operation enum — mirrors `shooting_op::ShootingConfigMutation`'s pilot shape
/// exactly: one variant per settled interaction (the pre-migration `NotePlayRuntime` field writes), plus
/// a generic `Snapshot` every variant's `backwards()` returns — since a config-only "View" dispatch is a
/// plain `Apply` (not an `AmendLast`), each tick is its own distinct, real config edit, and "undo this
/// tick" is exactly "restore the whole-config snapshot from just before it".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum NoteConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: NoteConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { block_ids: Vec<String> },
    #[dsl(key = "hovered-block")]
    SetHoveredBlock { block_id: Option<String> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: NoteCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for NoteConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for NoteConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
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
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


impl Mutation<NoteConfig> for NoteConfigMutation {
    type Diff = NoteConfig;

    fn diff(&self, base: &NoteConfig) -> NoteConfig {
        let mut next = base.clone();
        match self {
            NoteConfigMutation::Snapshot { config } => return config.clone(),
            NoteConfigMutation::SetSelection { block_ids } => next.selected_block_ids = block_ids.clone(),
            NoteConfigMutation::SetHoveredBlock { block_id } => next.hovered_block_id = block_id.clone(),
            NoteConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            NoteConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            NoteConfigMutation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            NoteConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &NoteConfig) -> Vec<Self> {
        vec![NoteConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_config_default_matches_the_pre_migration_runtime_defaults() {
        let config = NoteConfig::default();
        assert!(config.selected_block_ids.is_empty());
        assert!(config.hovered_block_id.is_none());
        assert_eq!(config.active_utility_id, "selectDirect");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.camera, NoteCamera::default());
    }

    /// 🧮️ B1 Config dsl/pack round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE).
    #[test]
    fn note_config_dsl_pack_round_trips() {
        let config = NoteConfig {
            selected_block_ids: vec!["text-1".into(), "table-2".into()],
            hovered_block_id: Some("image-3".into()),
            engagement_input: "Renaming…".into(),
            camera: NoteCamera { x: 12.5, y: -4.0, zoom: 2.5 },
            active_utility_id: "pencil".into(),
            locale: "de-DE".into(),
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn note_config_operation_text_and_binary_round_trip_every_variant() {
        let config = NoteConfig {
            selected_block_ids: vec!["text-1".into()],
            hovered_block_id: Some("image-2".into()),
            engagement_input: "Renaming…".into(),
            camera: NoteCamera { x: 3.0, y: -1.5, zoom: 1.75 },
            active_utility_id: "pencil".into(),
            locale: "de-DE".into(),
        };
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::Snapshot { config });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetSelection { block_ids: vec!["text-1".into(), "table-2".into()] });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetHoveredBlock { block_id: Some("image-2".into()) });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetHoveredBlock { block_id: None });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetEngagementInput { value: "Renaming…".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetActiveUtility { utility_id: "eraserStroke".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetLocale { value: "de-DE".into() });
    }

    /// 🧮️ Every `NoteConfigMutation`'s `backwards()` is the whole-config snapshot from just before it —
    /// mirrors `shooting_op`'s analogous coverage.
    #[test]
    fn note_config_operation_backwards_is_always_a_snapshot_of_the_prior_config() {
        let base = NoteConfig::default();
        let operation = NoteConfigMutation::SetActiveUtility { utility_id: "pencil".into() };
        assert_eq!(operation.inverse(&base), vec![NoteConfigMutation::Snapshot { config: base.clone() }]);
        let next = operation.diff(&base);
        assert_eq!(next.active_utility_id, "pencil");
        let restored = NoteConfigMutation::Snapshot { config: base.clone() }.diff(&next);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
