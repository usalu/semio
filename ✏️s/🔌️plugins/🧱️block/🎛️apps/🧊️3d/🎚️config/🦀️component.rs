//! 🧮️ Block 3D play app — the view-state config artifact and its operation enum, plus the per-window
//! view record (`Block3dWindowView`) and transient brush-preview pose (`Block3dBrushPreview`) nested
//! inside it. Session-only but real, undoable config: it round-trips through the config `ArtifactStore`
//! exactly like document content, with a true `backwards` per operation. Nothing here is document
//! state — the object kind's identity/representations/vortices live in `crate::artifacts::block3d`.

use crate::artifacts::block3d::{Block3dBrushPreview, Block3dWindowView};
use crate::BlockCamera3d;
use protocol::Mutation;
use serde::{Deserialize, Serialize};


//#region 🔖️Config
/// 🧮️ `Block3dPlayApp`'s real `ArtifactApp::Config` — B1 pure-trait conversion. Absorbs every former
/// `Block3dPlayApp` `RefCell` runtime field (`selected_ids`/`active_representation_id`) plus the
/// locale this app resolves itself. `wanted_tags` is ready for whenever a later wave threads `cfg`
/// into `export_media` (see that fn's doc for why it's currently unused there).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "block3dcfg")]
#[dsl(id = "block3d.config")]
#[dsl(layout = "lines")]
pub struct Block3dConfig {
    /// 👁️ Multi-selected row ids in the document tree — was `Block3dPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ The representation shown in the inspector's representation select — was
    /// `Block3dPlayApp::active_representation_id`.
    pub active_representation_id: Option<String>,
    /// 🏷️ Tag filter for `puzzle3d_catalog_fragment`'s active-representation resolution. Empty means
    /// "all tags".
    pub wanted_tags: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewModel.locale`.
    pub locale: String,
    #[serde(default)]
    #[dsl(table)]
    pub windows: Vec<Block3dWindowView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_vortex_kind_id: Option<String>,
    #[serde(default = "default_brush_radius")]
    pub brush_radius: f64,
    #[serde(default)]
    pub brush_flip: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview: Option<Block3dBrushPreview>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<BlockCamera3d>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_vortex_full_id: Option<String>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Block3dConfig {
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
impl store::ArtifactPack for Block3dConfig {
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


fn default_brush_radius() -> f64 {
    0.3
}

impl Default for Block3dConfig {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            active_representation_id: None,
            wanted_tags: Vec::new(),
            locale: "en-US".into(),
            windows: Vec::new(),
            brush_vortex_kind_id: None,
            brush_radius: default_brush_radius(),
            brush_flip: false,
            brush_preview: None,
            camera: None,
            hovered_vortex_full_id: None,
        }
    }
}

store::impl_whole_record_config!(Block3dConfig);

//#region 🔖️Accessors
pub fn block3d_window_view(config: &Block3dConfig, window_id: &str) -> Block3dWindowView {
    config.windows.iter().find(|row| row.window_id == window_id).cloned().unwrap_or_else(|| Block3dWindowView::for_window(window_id))
}

pub fn block3d_active_utility(config: &Block3dConfig, window_id: &str) -> String {
    block3d_window_view(config, window_id).active_utility
}

pub fn upsert_window_view_index(windows: &mut Vec<Block3dWindowView>, window_id: &str) -> usize {
    if let Some(index) = windows.iter().position(|row| row.window_id == window_id) {
        return index;
    }
    windows.push(Block3dWindowView::for_window(window_id));
    windows.len() - 1
}
//#endregion 🔖️Accessors
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Block3dConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Block3dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns.
// 🧯️ `large_enum_variant`: `Snapshot` deliberately carries the WHOLE `Block3dConfig` while every other
// row carries one or two scalars — that whole-config snapshot IS the inverse mechanism every variant's
// `backwards()` returns. Boxing it would change the derived `dsl::DslOps` wire encoding, which this
// migration must preserve byte-for-byte, so the size skew is accepted by design (same tradeoff as gis's
// `Gis2dConfigMutation`).
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block3dConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Block3dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "active-representation")]
    SetActiveRepresentation { representation_id: Option<String> },
    #[dsl(key = "wanted-tags")]
    SetWantedTags { tags: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "window-representations")]
    SetWindowRepresentations { window_id: String, representation_ids: Vec<String> },
    #[dsl(key = "toggle-window-representation")]
    ToggleWindowRepresentation { window_id: String, representation_id: String, visible: bool },
    #[dsl(key = "window-arrangement")]
    SetWindowArrangement { window_id: String, arrangement: String },
    #[dsl(key = "window-spacing")]
    SetWindowSpacing { window_id: String, spacing: f64 },
    #[dsl(key = "active-utility")]
    SetActiveUtility { window_id: String, utility_id: String },
    #[dsl(key = "brush-vortex-kind")]
    SetBrushVortexKind { vortex_kind_id: Option<String> },
    #[dsl(key = "brush-radius")]
    SetBrushRadius { radius: f64 },
    #[dsl(key = "brush-flip")]
    SetBrushFlip { flip: bool },
    #[dsl(key = "brush-preview")]
    SetBrushPreview { preview: Option<Block3dBrushPreview> },
    #[dsl(key = "camera")]
    SetCamera { camera: BlockCamera3d },
    #[dsl(key = "hovered-vortex")]
    SetHoveredVortexFullId { full_id: Option<String> },
}

//#region 🔖️OpCodec
impl protocol::OpText for Block3dConfigMutation {
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
impl protocol::OpBinary for Block3dConfigMutation {
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


impl Mutation<Block3dConfig> for Block3dConfigMutation {
    type Diff = Block3dConfig;

    fn diff(&self, base: &Block3dConfig) -> Block3dConfig {
        let mut next = base.clone();
        match self {
            Block3dConfigMutation::Snapshot { config } => return config.clone(),
            Block3dConfigMutation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Block3dConfigMutation::SetActiveRepresentation { representation_id } => next.active_representation_id = representation_id.clone(),
            Block3dConfigMutation::SetWantedTags { tags } => next.wanted_tags = tags.clone(),
            Block3dConfigMutation::SetLocale { value } => next.locale = value.clone(),
            Block3dConfigMutation::SetWindowRepresentations { window_id, representation_ids } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].representation_ids = representation_ids.clone();
            }
            Block3dConfigMutation::ToggleWindowRepresentation { window_id, representation_id, visible } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                let row = &mut next.windows[index];
                if *visible {
                    if !row.representation_ids.contains(representation_id) {
                        row.representation_ids.push(representation_id.clone());
                    }
                } else {
                    row.representation_ids.retain(|id| id != representation_id);
                }
            }
            Block3dConfigMutation::SetWindowArrangement { window_id, arrangement } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].arrangement = arrangement.clone();
            }
            Block3dConfigMutation::SetWindowSpacing { window_id, spacing } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].spacing = *spacing;
            }
            Block3dConfigMutation::SetActiveUtility { window_id, utility_id } => {
                let index = upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].active_utility = utility_id.clone();
            }
            Block3dConfigMutation::SetBrushVortexKind { vortex_kind_id } => next.brush_vortex_kind_id = vortex_kind_id.clone(),
            Block3dConfigMutation::SetBrushRadius { radius } => next.brush_radius = *radius,
            Block3dConfigMutation::SetBrushFlip { flip } => next.brush_flip = *flip,
            Block3dConfigMutation::SetBrushPreview { preview } => next.brush_preview = preview.clone(),
            Block3dConfigMutation::SetCamera { camera } => next.camera = Some(camera.clone()),
            Block3dConfigMutation::SetHoveredVortexFullId { full_id } => next.hovered_vortex_full_id = full_id.clone(),
        }
        next
    }

    fn inverse(&self, base: &Block3dConfig) -> Vec<Self> {
        vec![Block3dConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block3d_config_default_has_no_selection_and_all_tags() {
        let config = Block3dConfig::default();
        assert!(config.selected_ids.is_empty());
        assert!(config.active_representation_id.is_none());
        assert!(config.wanted_tags.is_empty());
        assert_eq!(config.locale, "en-US");
        assert!(config.windows.is_empty());
        assert_eq!(config.brush_radius, 0.3);
    }

    #[test]
    fn config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Block3dConfig::default();
        let operation = Block3dConfigMutation::SetSelection { ids: vec!["r0".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["r0".to_string()]);
        let inverse = operation.inverse(&base);
        assert_eq!(inverse, vec![Block3dConfigMutation::Snapshot { config: base.clone() }]);
        let restored = inverse[0].diff(&next);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
