//! 🧮️ DAG play app — view state (`DagConfig`) and its operation enum (`DagConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.dag` document. It absorbs everything that used to live in the old
//! ui crate's `DagPlayRuntime` (an app-struct `RefCell`) AND the two fields the dag UI actually read off
//! the deleted host-pushed `ViewModel` (`locale`, via `dag_play_labels`/`app_labels`/`context_menu`): the
//! selected node ids, the free/live node-graph viewport camera, and the BCP-47 locale tag — session-only
//! view state round-trips through the config `ArtifactStore` exactly like document content, with a real
//! `backwards` per `DagConfigMutation` instead of never being VCS'd at all.

use infinite_board_port_directed_dag::DagCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `DagPlayApp::Config` — the pure-trait `ArtifactApp::Config` for the dag app.
///
/// The camera is flattened to its three scalar fields (`camera_x`/`camera_y`/`camera_zoom`) rather than
/// embedding `infinite_board_port_directed_dag::DagCamera` as a `#[dsl(block)]`: that kernel type is
/// explicitly out of scope for this crate and doesn't derive `dsl::DslRecord` (only
/// `Clone`/`Debug`/`PartialEq`/`Serialize`/`Deserialize`), so it can't satisfy a nested-block field —
/// three plain `f64` fields need no such support at all. See `dag_config_camera` below for the seam back
/// to the real `DagCamera` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "dagcfg")]
#[dsl(id = "dag.config")]
#[dsl(layout = "lines")]
pub struct DagConfig {
    /// 🎥️ Viewport camera x — was `DagPlayRuntime::camera.x`.
    pub camera_x: f64,
    /// 🎥️ Viewport camera y — was `DagPlayRuntime::camera.y`.
    pub camera_y: f64,
    /// 🎥️ Viewport camera zoom — was `DagPlayRuntime::camera.zoom`.
    pub camera_zoom: f64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for DagConfig {
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
impl store::ArtifactPack for DagConfig {
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


impl Default for DagConfig {
    fn default() -> Self {
        // 🎥️ Matches `DagCamera`'s own implicit default (`x: 0.0, y: 0.0, zoom: 1.0`, see `DagFixture`'s
        // `Default` impl in the kernel crate) without needing to parse the bundled demo document just to
        // read a trivial camera default.
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(DagConfig);

/// 🎥️ Reassembles the kernel's `DagCamera` from `DagConfig`'s flattened scalar fields — the seam
/// `crate::apps::dag` uses wherever the old `DagPlayRuntime::camera` field was read.
pub fn dag_config_camera(config: &DagConfig) -> DagCamera {
    DagCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom }
}
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ `DagConfig`'s operation enum — one variant per settled interaction (mirrors the pre-migration
/// `DagPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()` returns: since
/// a config-only "View" dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own distinct,
/// real config edit, and "undo this tick" is exactly "restore the whole-config snapshot from just before
/// it" — the simplest correct inverse, needing no per-field reverse-patch bookkeeping. Mirrors
/// `shooting_op::ShootingConfigMutation` exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum DagConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: DagConfig,
    },
    #[dsl(key = "camera")]
    SetCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for DagConfigMutation {
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
impl protocol::OpBinary for DagConfigMutation {
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


impl Mutation<DagConfig> for DagConfigMutation {
    type Diff = DagConfig;

    fn diff(&self, base: &DagConfig) -> DagConfig {
        let mut next = base.clone();
        match self {
            DagConfigMutation::Snapshot { config } => return config.clone(),
            DagConfigMutation::SetCamera { x, y, zoom } => {
                next.camera_x = *x;
                next.camera_y = *y;
                next.camera_zoom = *zoom;
            }
            DagConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &DagConfig) -> Vec<Self> {
        vec![DagConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dag_config_default_matches_dag_camera_implicit_default() {
        let config = DagConfig::default();
        assert_eq!((config.camera_x, config.camera_y, config.camera_zoom), (0.0, 0.0, 1.0));
        assert_eq!(dag_config_camera(&config), DagCamera { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(config.locale, "en-US");
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `DagConfig`.
    #[test]
    fn dag_config_dsl_pack_round_trip() {
        let config = DagConfig { camera_x: 12.5, camera_y: -3.0, camera_zoom: 2.25, locale: "de-DE".into() };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn dag_config_operation_text_binary_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&DagConfigMutation::Snapshot { config: DagConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, locale: "de-DE".into() } });
        store::os_store::test_support::assert_op_line_round_trip(&DagConfigMutation::SetCamera { x: 12.5, y: -3.0, zoom: 2.25 });
        store::os_store::test_support::assert_op_line_round_trip(&DagConfigMutation::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn dag_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = DagConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, locale: "en-US".into() };
        let operation = DagConfigMutation::SetCamera { x: 9.0, y: 8.0, zoom: 7.0 };
        let forward = operation.diff(&base);
        assert_eq!((forward.camera_x, forward.camera_y, forward.camera_zoom), (9.0, 8.0, 7.0));
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![DagConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
