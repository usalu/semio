//! 🧮️ Raster app — view-state configuration (constitutional: general/config). B1: this absorbs every
//! former `RasterPlayRuntime` (`ui`-crate `RefCell`) field (brush size/opacity, navigator
//! composite-viewport size and the session-only free camera). `RasterConfigMutation` lives here too,
//! next to the `RasterConfig` it patches (TEMPLATE.md §4).
//!
//! 🕹️ `selected_ids`/`hovered_id` deleted (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
//! layer selection/hover is the framework-owned `"layers"` interaction domain now (granularity
//! `"layer"`, `HierarchyProvider::Topology`), read via `InteractionView::selection("layers")` instead of
//! this config.

use crate::RasterCamera;
pub use crate::editor::raster::selection::RasterPixelSelection;
use protocol::Mutation;

//#region 🔖️Config
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(id = "raster.config")]
#[artifact(extension = "rastercfg")]
#[dsl(layout = "lines")]
pub struct RasterConfig {
    /// 🖌️ Brush diameter (px) — was `RasterPlayRuntime::brush_size`.
    pub brush_size: f64,
    /// 🖌️ Brush opacity (0..1) — was `RasterPlayRuntime::brush_opacity`.
    pub brush_opacity: f64,
    /// 🎨️ Session foreground color in hexadecimal RGB.
    pub brush_color: String,
    /// 🖌️ Solid fraction of the brush radius.
    pub brush_hardness: f64,
    pub paint_target: String,
    pub mask_value: u32,
    /// 🪣️ The bucket's colour tolerance (0..255): how far a pixel may differ from the clicked pixel and still flood.
    pub fill_tolerance: u32,
    #[dsl(block)]
    pub pixel_selection:Option<RasterPixelSelection>,
    /// 🔭️ Navigator's last-known composite-window viewport size — was
    /// `RasterPlayRuntime::composite_viewport`.
    #[dsl(block)]
    pub composite_viewport: Option<RasterConfigViewportSize>,
    /// 🎥️ The free/live composite camera — session-only, never a document field. Was
    /// `RasterPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: RasterCamera,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for RasterConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
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

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RasterConfig {
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

pub type RasterConfigViewportSize = crate::RasterViewportSize;

impl Default for RasterConfig {
    fn default() -> Self {
        Self { brush_size: 24.0, brush_opacity: 1.0, brush_color: "#2878dc".into(), brush_hardness: 1.0, paint_target:"pixels".into(),mask_value:255,fill_tolerance:24,pixel_selection:None,composite_viewport: None, camera: RasterCamera::default() }
    }
}

/// 🎨️ Admits a complete RGB color without permissive CSS parsing.
pub fn valid_brush_color(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
}

impl RasterConfig {
    pub fn brush_rgba(&self) -> [u8; 4] {
        let value = u32::from_str_radix(self.brush_color.trim_start_matches('#'), 16).unwrap_or(0);
        [(value >> 16) as u8, (value >> 8) as u8, value as u8, 255]
    }
}

impl store::ConfigRecord for RasterConfig {}

/// 🧭️ Explicit set-or-clear of the optional pixel selection (an absent patch field leaves it untouched, `value: None` clears it).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterPixelSelectionSet {
    pub value: Option<RasterPixelSelection>,
}

/// 🧭️ Explicit set-or-clear of the optional composite viewport size.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterViewportSet {
    pub value: Option<RasterConfigViewportSize>,
}

/// 🔺️ Sparse delta of the session preferences: only the fields a mutation actually changes.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub brush_size: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub brush_opacity: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub brush_color: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub brush_hardness: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub paint_target: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub mask_value: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub fill_tolerance: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub pixel_selection: Option<RasterPixelSelectionSet>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub composite_viewport: Option<RasterViewportSet>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<RasterCamera>,
}

impl protocol::MutationDiff<RasterConfig> for RasterConfigDiff {
    fn apply(&self, base: &RasterConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RasterConfig> {
        Ok(RasterConfig {
            brush_size: self.brush_size.unwrap_or(base.brush_size),
            brush_opacity: self.brush_opacity.unwrap_or(base.brush_opacity),
            brush_color: self.brush_color.clone().unwrap_or_else(|| base.brush_color.clone()),
            brush_hardness: self.brush_hardness.unwrap_or(base.brush_hardness),
            paint_target: self.paint_target.clone().unwrap_or_else(|| base.paint_target.clone()),
            mask_value: self.mask_value.unwrap_or(base.mask_value),
            fill_tolerance: self.fill_tolerance.unwrap_or(base.fill_tolerance),
            pixel_selection: self.pixel_selection.as_ref().map_or_else(|| base.pixel_selection.clone(), |set| set.value.clone()),
            composite_viewport: self.composite_viewport.as_ref().map_or_else(|| base.composite_viewport.clone(), |set| set.value.clone()),
            camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()),
        })
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(brush_size);
        take!(brush_opacity);
        take!(brush_color);
        take!(brush_hardness);
        take!(paint_target);
        take!(mask_value);
        take!(fill_tolerance);
        take!(pixel_selection);
        take!(composite_viewport);
        take!(camera);
    }
}

impl protocol::DiffAlgebra<RasterConfig> for RasterConfigDiff {
    fn inverse(&self, base: &RasterConfig) -> Self {
        Self {
            brush_size: self.brush_size.map(|_| base.brush_size),
            brush_opacity: self.brush_opacity.map(|_| base.brush_opacity),
            brush_color: self.brush_color.as_ref().map(|_| base.brush_color.clone()),
            brush_hardness: self.brush_hardness.map(|_| base.brush_hardness),
            paint_target: self.paint_target.as_ref().map(|_| base.paint_target.clone()),
            mask_value: self.mask_value.map(|_| base.mask_value),
            fill_tolerance: self.fill_tolerance.map(|_| base.fill_tolerance),
            pixel_selection: self.pixel_selection.as_ref().map(|_| RasterPixelSelectionSet { value: base.pixel_selection.clone() }),
            composite_viewport: self.composite_viewport.as_ref().map(|_| RasterViewportSet { value: base.composite_viewport.clone() }),
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ `RasterConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1 `RasterPlayRuntime` field
/// writes); each variant's diff carries only the fields it changes and its inverse is the absolute setter of the base values.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum RasterConfigMutation {
    #[dsl(key = "brush-size")]
    SetBrushSize { value: f64 },
    #[dsl(key = "brush-opacity")]
    SetBrushOpacity { value: f64 },
    #[dsl(key = "brush-color")]
    SetBrushColor { value: String },
    #[dsl(key = "brush-hardness")]
    SetBrushHardness { value: f64 },
    #[dsl(key = "paint-target")]
    SetPaintTarget {value:String},
    #[dsl(key = "mask-value")]
    SetMaskValue {value:u32},
    #[dsl(key="pixel-selection")]
    SetPixelSelection {#[dsl(block)] selection:Option<RasterPixelSelection>},
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
    #[dsl(key = "fill-tolerance")]
    SetFillTolerance { value: u32 },
}

//#region 🔖️OpCodec
impl protocol::OpText for RasterConfigMutation {
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

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for RasterConfigMutation {
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

impl Mutation<RasterConfig> for RasterConfigMutation {
    type Diff = RasterConfigDiff;

    /// 🧷️ Hand-written (no `dsl::Mutations` derive on this enum) — config authorities are session
    /// state, not document leaves, so the `owner` paths are registry metadata only.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️brush-size",
            semantic_kind: "set-brush-size",
            display_name: "Set Brush Size",
            emoji: "🖌️",
            aggregate_variant: "SetBrushSize",
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
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌫️brush-opacity",
            semantic_kind: "set-brush-opacity",
            display_name: "Set Brush Opacity",
            emoji: "🌫️",
            aggregate_variant: "SetBrushOpacity",
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
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖼️composite-viewport",
            semantic_kind: "set-composite-viewport",
            display_name: "Set Composite Viewport",
            emoji: "🖼️",
            aggregate_variant: "SetCompositeViewport",
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
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎥️camera",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "🎥️",
            aggregate_variant: "SetCamera",
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
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎨️brush-color",
            semantic_kind: "set-brush-color",
            display_name: "Set Brush Color",
            emoji: "🎨️",
            aggregate_variant: "SetBrushColor",
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
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️brush-hardness",
            semantic_kind: "set-brush-hardness",
            display_name: "Set Brush Hardness",
            emoji: "🖌️",
            aggregate_variant: "SetBrushHardness",
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
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎭️paint-target",
            semantic_kind: "set-paint-target",
            display_name: "Set Paint Target",
            emoji: "🎭️",
            aggregate_variant: "SetPaintTarget",
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
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎭️mask-value",
            semantic_kind: "set-mask-value",
            display_name: "Set Mask Value",
            emoji: "🎭️",
            aggregate_variant: "SetMaskValue",
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
            schema_version:1,
            owner:"✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎯️pixel-selection",
            semantic_kind:"set-pixel-selection",
            display_name:"Set Pixel Selection",
            emoji:"🎯️",
            aggregate_variant:"SetPixelSelection",
            payload_schema:"🧬️schema/🔣️.json",
            text_opcode:None,binary_tag:None,
            invertibility:protocol::MutationInvertibility::ExplicitMutation,
            diff_participation:protocol::MutationDiffParticipation::Detect,
            outcome_classes:&[protocol::MutationOutcomeClass::Applied],
            composition:protocol::MutationComposition::Atomic,
            required_language_surfaces:&[protocol::MutationLanguageSurface::Rust,protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌊️fill-tolerance",
            semantic_kind: "set-fill-tolerance",
            display_name: "Set Fill Tolerance",
            emoji: "🌊️",
            aggregate_variant: "SetFillTolerance",
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
            Self::SetBrushSize { .. } => &Self::DESCRIPTORS[0],
            Self::SetBrushOpacity { .. } => &Self::DESCRIPTORS[1],
            Self::SetCompositeViewport { .. } => &Self::DESCRIPTORS[2],
            Self::SetCamera { .. } => &Self::DESCRIPTORS[3],
            Self::SetBrushColor { .. } => &Self::DESCRIPTORS[4],
            Self::SetBrushHardness { .. } => &Self::DESCRIPTORS[5],
            Self::SetPaintTarget {..}=> &Self::DESCRIPTORS[6],
            Self::SetMaskValue {..}=> &Self::DESCRIPTORS[7],
            Self::SetPixelSelection {..}=> &Self::DESCRIPTORS[8],
            Self::SetFillTolerance { .. } => &Self::DESCRIPTORS[9],
        }
    }

    fn diff(&self, base: &RasterConfig) -> protocol::MutationOutcome<RasterConfigDiff> {
        let mut diff = RasterConfigDiff::default();
        match self {
            RasterConfigMutation::SetBrushSize { value } => {
                if value.is_finite() && (1.0..=2048.0).contains(value) && base.brush_size != *value {
                    diff.brush_size = Some(*value);
                }
            }
            RasterConfigMutation::SetBrushOpacity { value } => {
                if value.is_finite() && (0.0..=1.0).contains(value) && base.brush_opacity != *value {
                    diff.brush_opacity = Some(*value);
                }
            }
            RasterConfigMutation::SetBrushColor { value } => {
                if valid_brush_color(value) && base.brush_color != *value {
                    diff.brush_color = Some(value.clone());
                }
            }
            RasterConfigMutation::SetBrushHardness { value } => {
                if value.is_finite() && (0.0..=1.0).contains(value) && base.brush_hardness != *value {
                    diff.brush_hardness = Some(*value);
                }
            }
            RasterConfigMutation::SetPaintTarget { value } => {
                if matches!(value.as_str(), "pixels" | "mask") && base.paint_target != *value {
                    diff.paint_target = Some(value.clone());
                    if base.pixel_selection.is_some() {
                        diff.pixel_selection = Some(RasterPixelSelectionSet { value: None });
                    }
                }
            }
            RasterConfigMutation::SetPixelSelection { selection } => {
                if selection.as_ref().is_none_or(|value| value.target == base.paint_target && value.validate().is_ok()) && base.pixel_selection != *selection {
                    diff.pixel_selection = Some(RasterPixelSelectionSet { value: selection.clone() });
                }
            }
            RasterConfigMutation::SetMaskValue { value } => {
                if *value <= 255 && base.mask_value != *value {
                    diff.mask_value = Some(*value);
                }
            }
            RasterConfigMutation::SetFillTolerance { value } => {
                if *value <= 255 && base.fill_tolerance != *value {
                    diff.fill_tolerance = Some(*value);
                }
            }
            RasterConfigMutation::SetCompositeViewport { viewport } => {
                if base.composite_viewport != *viewport {
                    diff.composite_viewport = Some(RasterViewportSet { value: viewport.clone() });
                }
            }
            RasterConfigMutation::SetCamera { camera } => {
                if base.camera != *camera {
                    diff.camera = Some(camera.clone());
                }
            }
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &RasterConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(match self {
            RasterConfigMutation::SetBrushSize { .. } => vec![RasterConfigMutation::SetBrushSize { value: base.brush_size }],
            RasterConfigMutation::SetBrushOpacity { .. } => vec![RasterConfigMutation::SetBrushOpacity { value: base.brush_opacity }],
            RasterConfigMutation::SetBrushColor { .. } => vec![RasterConfigMutation::SetBrushColor { value: base.brush_color.clone() }],
            RasterConfigMutation::SetBrushHardness { .. } => vec![RasterConfigMutation::SetBrushHardness { value: base.brush_hardness }],
            RasterConfigMutation::SetPaintTarget { .. } => vec![RasterConfigMutation::SetPaintTarget { value: base.paint_target.clone() }, RasterConfigMutation::SetPixelSelection { selection: base.pixel_selection.clone() }],
            RasterConfigMutation::SetPixelSelection { .. } => vec![RasterConfigMutation::SetPixelSelection { selection: base.pixel_selection.clone() }],
            RasterConfigMutation::SetMaskValue { .. } => vec![RasterConfigMutation::SetMaskValue { value: base.mask_value }],
            RasterConfigMutation::SetFillTolerance { .. } => vec![RasterConfigMutation::SetFillTolerance { value: base.fill_tolerance }],
            RasterConfigMutation::SetCompositeViewport { .. } => vec![RasterConfigMutation::SetCompositeViewport { viewport: base.composite_viewport.clone() }],
            RasterConfigMutation::SetCamera { .. } => vec![RasterConfigMutation::SetCamera { camera: base.camera.clone() }],
        })
    }

    fn inverse_rows(&self) -> usize {
        match self {
            RasterConfigMutation::SetPaintTarget { .. } => 2,
            _ => 1,
        }
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
