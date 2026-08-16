//! 🧮️ Raster app — view-state configuration (constitutional: general/config). B1: this absorbs every
//! former `RasterPlayRuntime` (`ui`-crate `RefCell`) field (brush size/opacity, navigator
//! composite-viewport size, the session-only free camera) plus the two former `ViewModel`-driven
//! fields the raster UI actually reads (`active_utility_id`/`locale` — mirrors
//! `shooting_engine::ShootingConfig`'s identical B1 migration). `RasterConfigMutation` lives here too,
//! next to the `RasterConfig` it patches (TEMPLATE.md §4).
//!
//! 🕹️ `selected_ids`/`hovered_id` deleted (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
//! layer selection/hover is the framework-owned `"layers"` interaction domain now (granularity
//! `"layer"`, `HierarchyProvider::Flat`), read via `InteractionView::selection("layers")` instead of
//! this config.

use crate::artifacts::raster::RasterCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(id = "raster.config")]
#[dsl(extension = "rastercfg")]
#[dsl(layout = "lines")]
pub struct RasterConfig {
    /// 🖌️ Brush diameter (px) — was `RasterPlayRuntime::brush_size`.
    pub brush_size: f64,
    /// 🖌️ Brush opacity (0..1) — was `RasterPlayRuntime::brush_opacity`.
    pub brush_opacity: f64,
    /// 🔭️ Navigator's last-known composite-window viewport size — was
    /// `RasterPlayRuntime::composite_viewport`.
    #[dsl(block)]
    pub composite_viewport: Option<RasterConfigViewportSize>,
    /// 🎥️ The free/live composite camera — session-only, never a document field. Was
    /// `RasterPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: RasterCamera,
    /// 🧰️ The active composite-window utility — was read off `view_state.active_utility_id`
    /// (host-pushed `ViewModel`, deleted by B1). Default mirrors the app's `RASTER_DEFAULT_UTILITY`.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for RasterConfig {
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
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RasterConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
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

//#endregion 🔖️ArtifactCodec


pub type RasterConfigViewportSize = crate::artifacts::raster::RasterViewportSize;

impl Default for RasterConfig {
    fn default() -> Self {
        Self { brush_size: 24.0, brush_opacity: 1.0, composite_viewport: None, camera: RasterCamera::default(), active_utility_id: "selectMarquee".into(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(RasterConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// @emoji 🧮️ B1: `RasterConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `RasterPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `shooting_op::ShootingConfigMutation`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum RasterConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RasterConfig,
    },
    #[dsl(key = "brush-size")]
    SetBrushSize { value: f64 },
    #[dsl(key = "brush-opacity")]
    SetBrushOpacity { value: f64 },
    #[dsl(key = "composite-viewport")]
    SetCompositeViewport {
        #[dsl(block)]
        viewport: Option<RasterConfigViewportSize>,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: RasterCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for RasterConfigMutation {
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
impl protocol::OpBinary for RasterConfigMutation {
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


impl Mutation<RasterConfig> for RasterConfigMutation {
    type Diff = RasterConfig;

    fn diff(&self, base: &RasterConfig) -> protocol::MutationOutcome<RasterConfig> {
        let mut next = base.clone();
        match self {
            RasterConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            RasterConfigMutation::SetBrushSize { value } => next.brush_size = *value,
            RasterConfigMutation::SetBrushOpacity { value } => next.brush_opacity = value.clamp(0.0, 1.0),
            RasterConfigMutation::SetCompositeViewport { viewport } => next.composite_viewport = viewport.clone(),
            RasterConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            RasterConfigMutation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            RasterConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &RasterConfig) -> Vec<Self> {
        vec![RasterConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_config_operation_round_trips_and_backwards_restores_snapshot() {
        let base = RasterConfig { brush_size: 24.0, ..Default::default() };
        let operation = RasterConfigMutation::SetBrushSize { value: 40.0 };
        let forward = operation.diff(&base).diff().clone();
        assert_eq!(forward.brush_size, 40.0);
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![RasterConfigMutation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&forward).diff().clone(), base);
    }

    #[test]
    fn raster_config_operation_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::Snapshot { config: RasterConfig::default() });
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushSize { value: 40.0 });
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetBrushOpacity { value: 0.5 });
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport { viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }) });
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCompositeViewport { viewport: None });
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetCamera { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetActiveUtility { utility_id: "paintBrush".into() });
        store::os_store::test_support::assert_op_line_round_trip(&RasterConfigMutation::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn raster_config_default_matches_ui_selectmarquee_utility() {
        let config = RasterConfig::default();
        assert_eq!(config.active_utility_id, "selectMarquee");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.brush_size, 24.0);
        assert_eq!(config.brush_opacity, 1.0);
    }

    #[test]
    fn raster_config_dsl_round_trips() {
        let config = RasterConfig {
            brush_size: 40.0,
            brush_opacity: 0.5,
            composite_viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }),
            camera: RasterCamera { x: 5.0, y: -3.0, zoom: 2.0 },
            active_utility_id: "paintBrush".into(),
            locale: "de-DE".into(),
        };
        store::os_store::test_support::assert_dsl_round_trip(&config);
    }
}
//#endregion 🧪️Tests
