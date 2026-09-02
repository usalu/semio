//! 🧮️ Drawing play app — view state (constitutional: was `engine`'s `Config` struct + `op`'s
//! `ConfigOperation`, split out per the taxonomy recipe: view state lives at app level, not artifact).

use crate::artifacts::drawing::DrawingCamera;
use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ B1: drawing's real `ArtifactApp::Config` — absorbs every former `DrawingInteractionState`
/// (`ui`-crate `RefCell`) field (selection, hover, in-progress engagement-input text, the
/// session-only free viewport camera) plus the two former `ViewModel`-driven fields the drawing UI
/// actually reads (`active_utility_id`/`locale` — mirrors `shooting_engine::ShootingConfig`'s
/// identical B1 migration) — session view state now round-trips through the config `ArtifactStore`
/// exactly like document content, with a real `backwards` per `DrawingConfigMutation` instead of
/// never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "drawingcfg")]
#[dsl(id = "drawing.config")]
#[dsl(layout = "lines")]
pub struct DrawingConfig {
    /// 👁️ In-progress rename/engagement input text — was `DrawingInteractionState::engagement_input`.
    pub engagement_input: String,
    /// 🎥️ The free/live canvas camera — session-only, never a document field. Was
    /// `DrawingInteractionState::camera`.
    #[dsl(block)]
    pub camera: DrawingCamera,
    /// 🧰️ The active canvas utility — was read off `view_state.active_utility_id` (host-pushed
    /// `ViewModel`, deleted by B1). Default mirrors the pre-migration `DRAWING_DEFAULT_UTILITY`
    /// (`"selectDirect"`).
    pub active_utility_id: String,
    /// 🧵️ Latest trace-pointer generation; zero means no live trace continuation.
    pub trace_pointer_generation: u64,
    /// ⏱️ Completed bounded trace work units for observable progress.
    pub trace_pointer_completed_work: u64,
    /// ⏳ Pending bounded trace work units for observable progress.
    pub trace_pointer_pending_work: u64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for DrawingConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
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
impl store::ArtifactPack for DrawingConfig {
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

impl Default for DrawingConfig {
    fn default() -> Self {
        Self {
            engagement_input: String::new(),
            camera: DrawingCamera::default(),
            active_utility_id: "selectDirect".into(),
            trace_pointer_generation: 0,
            trace_pointer_completed_work: 0,
            trace_pointer_pending_work: 0,
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(DrawingConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `DrawingConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `DrawingInteractionState` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns: since a config-only "View" dispatch is a plain `Apply` (not an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it" — mirrors
/// `shooting_op::ShootingConfigOperation`'s identical shape.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps)]
pub enum DrawingConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: DrawingConfig,
    },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: DrawingCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "trace-pointer-progress")]
    SetTracePointerProgress { generation: u64, completed_work: u64, pending_work: u64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for DrawingConfigMutation {
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
impl protocol::OpBinary for DrawingConfigMutation {
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

impl Mutation<DrawingConfig> for DrawingConfigMutation {
    type Diff = DrawingConfig;

    fn diff(&self, base: &DrawingConfig) -> protocol::MutationOutcome<DrawingConfig> {
        let mut next = base.clone();
        match self {
            DrawingConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            DrawingConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            DrawingConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            DrawingConfigMutation::SetActiveUtility { utility_id } => {
                next.active_utility_id = utility_id.clone();
                next.trace_pointer_generation = 0;
                next.trace_pointer_completed_work = 0;
                next.trace_pointer_pending_work = 0;
            }
            DrawingConfigMutation::SetTracePointerProgress { generation, completed_work, pending_work } => {
                next.trace_pointer_generation = *generation;
                next.trace_pointer_completed_work = *completed_work;
                next.trace_pointer_pending_work = *pending_work;
            }
            DrawingConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &DrawingConfig) -> Vec<Self> {
        vec![DrawingConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn drawing_config_default_matches_ui_selectdirect_utility() {
        let config = DrawingConfig::default();
        assert_eq!(config.active_utility_id, "selectDirect");
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn drawing_config_dsl_round_trips() {
        let config = DrawingConfig { engagement_input: "Renaming \"layer\"".into(), camera: DrawingCamera { x: 12.0, y: -4.0, zoom: 1.5 }, active_utility_id: "pen".into(), locale: "de-DE".into(), ..Default::default() };
        store::os_store::test_support::assert_dsl_round_trip(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn drawing_config_operation_round_trips_and_backwards_restores_snapshot() {
        let base = DrawingConfig { active_utility_id: "selectDirect".into(), ..Default::default() };
        let operation = DrawingConfigMutation::SetActiveUtility { utility_id: "pen".into() };
        let forward = operation.diff(&base).diff().clone();
        assert_eq!(forward.active_utility_id, "pen");
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![DrawingConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward).diff().clone();
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn drawing_config_operation_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::Snapshot { config: DrawingConfig::default() });
        store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetEngagementInput { value: "New \"Name\"".into() });
        store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetCamera { camera: DrawingCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
        store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetActiveUtility { utility_id: "pen".into() });
        store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetTracePointerProgress { generation: 7, completed_work: 32, pending_work: 4 });
        store::os_store::test_support::assert_op_line_round_trip(&DrawingConfigMutation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
