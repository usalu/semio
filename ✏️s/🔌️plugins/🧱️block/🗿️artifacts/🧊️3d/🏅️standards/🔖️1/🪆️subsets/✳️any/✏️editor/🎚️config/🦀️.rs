//! 🧮️ Block 3D play app — the view-state config artifact and its operation enum, plus the per-window
//! view record (`Block3dWindowView`). Session-only but real, undoable config: it round-trips through the config `ArtifactStore`
//! exactly like document content, with a true `backwards` per operation. Nothing here is document
//! state — the object kind's identity/representations/vortices live in `crate`.

use crate::BlockCamera3d;
use crate::Block3dWindowView;
use protocol::Mutation;
use semio_s_plugin_block::{BlockOptionalText, BlockPatch};

//#region 🔖️Config
/// 🧮️ `Block3dPlayApp`'s real `ArtifactEditor::Config` — B1 pure-trait conversion. Absorbs every former
/// `Block3dPlayApp` `RefCell` runtime field (`selected_ids`/`active_representation_id`).
/// `wanted_tags` is ready for whenever a later wave threads `cfg`
/// into `export_media` (see that fn's doc for why it's currently unused there).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "block3dcfg")]
#[artifact(id = "block3d.config")]
#[dsl(layout = "lines")]
pub struct Block3dConfig {
    /// 👁️ The representation shown in the inspector's representation select — was
    /// `Block3dPlayApp::active_representation_id`.
    pub active_representation_id: Option<String>,
    /// 🏷️ Tag filter for `puzzle3d_catalog_fragment`'s active-representation resolution. Empty means
    /// "all tags".
    pub wanted_tags: Vec<String>,
    /// 🪟️ Per-window view state.
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    pub windows: Vec<Block3dWindowView>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub brush_vortex_kind_id: Option<String>,
    #[value(default = "default_brush_radius")]
    #[cfg_attr(test, serde(default = "default_brush_radius"))]
    pub brush_radius: f64,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub brush_flip: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub camera: Option<BlockCamera3d>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Block3dConfig {
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
impl store::ArtifactPack for Block3dConfig {
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

fn default_brush_radius() -> f64 {
    0.3
}

impl Default for Block3dConfig {
    fn default() -> Self {
        Self { active_representation_id: None, wanted_tags: Vec::new(), windows: Vec::new(), brush_vortex_kind_id: None, brush_radius: default_brush_radius(), brush_flip: false, camera: None }
    }
}

impl store::ConfigRecord for Block3dConfig {}

//#region 🔖️ConfigDiff
semio_s_plugin_block::block_optional!(test; /// 🎥 The optional camera set to a value or cleared.
    Block3dOptionalCamera(BlockCamera3d));
semio_s_plugin_block::block_patch!(test; /// 🪟 Field patch over one window view (its window id is the row identity).
    Block3dWindowViewPatch for Block3dWindowView { plain { representation_ids: Vec<String>, arrangement: String, spacing: f64 } optional { } });
semio_framework_value::artifact_retire_struct!(Block3dWindowViewPatch { representation_ids, arrangement, spacing });
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📂 Row delta over the per-window views.
    pub Block3dWindowsDelta { removal: Block3dWindowsRemoval, insertion: Block3dWindowsInsertion, relocation: Block3dWindowsRelocation, modification: Block3dWindowsPatchEntry, row: Block3dWindowView, patch: Block3dWindowViewPatch, key: window_id }
}

/// 🔺️ Field-sparse diff of [`Block3dConfig`]: each field is an optional absolute value, window views are id-keyed rows.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block3dConfigDiff {
    pub active_representation_id: Option<BlockOptionalText>,
    pub wanted_tags: Option<Vec<String>>,
    pub windows: Block3dWindowsDelta,
    pub brush_vortex_kind_id: Option<BlockOptionalText>,
    pub brush_radius: Option<f64>,
    pub brush_flip: Option<bool>,
    pub camera: Option<Block3dOptionalCamera>,
}

/// ✏️ The window delta that sets `patch` on `window_id`. Rows equal to the default view are never stored and rows stay sorted by window id, so every edit and its inverse land on one canonical document.
fn window_edit(base: &Block3dConfig, window_id: &str, patch: Block3dWindowViewPatch) -> Block3dConfigDiff {
    let current = base.windows.iter().find(|row| row.window_id == window_id);
    let default = Block3dWindowView::for_window(window_id);
    let next = patch.patched(current.unwrap_or(&default)).unwrap_or_else(|_| default.clone());
    let windows = match (current, next == default) {
        (None, true) => Block3dWindowsDelta::default(),
        (None, false) => Block3dWindowsDelta::insertion(base.windows.partition_point(|row| row.window_id.as_str() < window_id), next),
        (Some(_), true) => Block3dWindowsDelta::removal(&base.windows, base.windows.iter().position(|row| row.window_id == window_id).unwrap_or(usize::MAX)),
        (Some(_), false) => Block3dWindowsDelta { modified: vec![Block3dWindowsPatchEntry { id: window_id.to_string(), patch }], ..Default::default() },
    };
    Block3dConfigDiff { windows, ..Default::default() }
}

fn lift(error: semio_s_plugin_block::BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

impl protocol::MutationDiff<Block3dConfig> for Block3dConfigDiff {
    fn apply(&self, base: &Block3dConfig, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block3dConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.active_representation_id {
            next.active_representation_id.clone_from(&value.value);
        }
        if let Some(tags) = &self.wanted_tags {
            next.wanted_tags.clone_from(tags);
        }
        next.windows = self.windows.commit_onto(&base.windows, capability).map_err(|error| error.under(["windows"]))?;
        if let Some(value) = &self.brush_vortex_kind_id {
            next.brush_vortex_kind_id.clone_from(&value.value);
        }
        if let Some(radius) = self.brush_radius {
            next.brush_radius = radius;
        }
        if let Some(flip) = self.brush_flip {
            next.brush_flip = flip;
        }
        if let Some(value) = &self.camera {
            next.camera.clone_from(&value.value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.active_representation_id.is_some() {
            self.active_representation_id = later.active_representation_id;
        }
        if later.wanted_tags.is_some() {
            self.wanted_tags = later.wanted_tags;
        }
        self.windows.absorb(later.windows);
        if later.brush_vortex_kind_id.is_some() {
            self.brush_vortex_kind_id = later.brush_vortex_kind_id;
        }
        if later.brush_radius.is_some() {
            self.brush_radius = later.brush_radius;
        }
        if later.brush_flip.is_some() {
            self.brush_flip = later.brush_flip;
        }
        if later.camera.is_some() {
            self.camera = later.camera;
        }
    }
}

impl protocol::DiffAlgebra<Block3dConfig> for Block3dConfigDiff {
    fn inverse(&self, base: &Block3dConfig) -> Self {
        Self {
            active_representation_id: self.active_representation_id.as_ref().map(|_| BlockOptionalText { value: base.active_representation_id.clone() }),
            wanted_tags: self.wanted_tags.as_ref().map(|_| base.wanted_tags.clone()),
            windows: self.windows.inverse(&base.windows),
            brush_vortex_kind_id: self.brush_vortex_kind_id.as_ref().map(|_| BlockOptionalText { value: base.brush_vortex_kind_id.clone() }),
            brush_radius: self.brush_radius.map(|_| base.brush_radius),
            brush_flip: self.brush_flip.map(|_| base.brush_flip),
            camera: self.camera.as_ref().map(|_| Block3dOptionalCamera { value: base.camera.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.active_representation_id.is_none() && self.wanted_tags.is_none() && self.windows.is_empty() && self.brush_vortex_kind_id.is_none() && self.brush_radius.is_none() && self.brush_flip.is_none() && self.camera.is_none()
    }
}
//#endregion 🔖️ConfigDiff


//#region 🔖️Accessors
pub fn block3d_window_view(config: &Block3dConfig, window_id: &str) -> Block3dWindowView {
    config.windows.iter().find(|row| row.window_id == window_id).cloned().unwrap_or_else(|| Block3dWindowView::for_window(window_id))
}
//#endregion 🔖️Accessors
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Block3dConfig`'s operation enum — one variant per settled interaction; each lowers to a field-sparse [`Block3dConfigDiff`].
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "kebab-case")]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Block3dConfigMutation {
    #[dsl(key = "active-representation")]
    SetActiveRepresentation { representation_id: Option<String> },
    #[dsl(key = "wanted-tags")]
    SetWantedTags { tags: Vec<String> },
    #[dsl(key = "window-representations")]
    SetWindowRepresentations { window_id: String, representation_ids: Vec<String> },
    #[dsl(key = "toggle-window-representation")]
    ToggleWindowRepresentation { window_id: String, representation_id: String, visible: bool },
    #[dsl(key = "window-arrangement")]
    SetWindowArrangement { window_id: String, arrangement: String },
    #[dsl(key = "window-spacing")]
    SetWindowSpacing { window_id: String, spacing: f64 },
    #[dsl(key = "brush-vortex-kind")]
    SetBrushVortexKind { vortex_kind_id: Option<String> },
    #[dsl(key = "brush-radius")]
    SetBrushRadius { radius: f64 },
    #[dsl(key = "brush-flip")]
    SetBrushFlip { flip: bool },
    #[dsl(key = "camera")]
    SetCamera { camera: BlockCamera3d },
    #[dsl(key = "clear-camera")]
    ClearCamera,
}

//#region 🔖️OpCodec
impl protocol::OpText for Block3dConfigMutation {
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
impl protocol::OpBinary for Block3dConfigMutation {
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

impl Mutation<Block3dConfig> for Block3dConfigMutation {
    type Diff = Block3dConfigDiff;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: this enum's `diff`/`inverse` dispatch is a
    /// plain `match`, not the derive's per-leaf `MutationKind` shape). One entry per variant, in
    /// declaration order. ⚠️ PROVISIONAL: none of these variants has an authored leaf directory on
    /// disk yet — every `owner` below names a path that does not exist, matching puzzle3d's own
    /// `Puzzle3dConfigMutation` precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧱set-active-representation",
            semantic_kind: "set-active-representation",
            display_name: "Set Active Representation",
            emoji: "🧱",
            aggregate_variant: "SetActiveRepresentation",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔖set-wanted-tags",
            semantic_kind: "set-wanted-tags",
            display_name: "Set Wanted Tags",
            emoji: "🔖",
            aggregate_variant: "SetWantedTags",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🪟set-window-representations",
            semantic_kind: "set-window-representations",
            display_name: "Set Window Representations",
            emoji: "🪟",
            aggregate_variant: "SetWindowRepresentations",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔁toggle-window-representation",
            semantic_kind: "toggle-window-representation",
            display_name: "Toggle Window Representation",
            emoji: "🔁",
            aggregate_variant: "ToggleWindowRepresentation",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/↔️set-window-arrangement",
            semantic_kind: "set-window-arrangement",
            display_name: "Set Window Arrangement",
            emoji: "↔️",
            aggregate_variant: "SetWindowArrangement",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📏set-window-spacing",
            semantic_kind: "set-window-spacing",
            display_name: "Set Window Spacing",
            emoji: "📏",
            aggregate_variant: "SetWindowSpacing",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌀set-brush-vortex-kind",
            semantic_kind: "set-brush-vortex-kind",
            display_name: "Set Brush Vortex Kind",
            emoji: "🌀",
            aggregate_variant: "SetBrushVortexKind",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-brush-radius",
            semantic_kind: "set-brush-radius",
            display_name: "Set Brush Radius",
            emoji: "🖌️",
            aggregate_variant: "SetBrushRadius",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔃set-brush-flip",
            semantic_kind: "set-brush-flip",
            display_name: "Set Brush Flip",
            emoji: "🔃",
            aggregate_variant: "SetBrushFlip",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📷set-camera",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "📷",
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
            owner: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🈚clear-camera",
            semantic_kind: "clear-camera",
            display_name: "Clear Camera",
            emoji: "🈚",
            aggregate_variant: "ClearCamera",
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
            Block3dConfigMutation::SetActiveRepresentation { .. } => &Self::DESCRIPTORS[0],
            Block3dConfigMutation::SetWantedTags { .. } => &Self::DESCRIPTORS[1],
            Block3dConfigMutation::SetWindowRepresentations { .. } => &Self::DESCRIPTORS[2],
            Block3dConfigMutation::ToggleWindowRepresentation { .. } => &Self::DESCRIPTORS[3],
            Block3dConfigMutation::SetWindowArrangement { .. } => &Self::DESCRIPTORS[4],
            Block3dConfigMutation::SetWindowSpacing { .. } => &Self::DESCRIPTORS[5],
            Block3dConfigMutation::SetBrushVortexKind { .. } => &Self::DESCRIPTORS[6],
            Block3dConfigMutation::SetBrushRadius { .. } => &Self::DESCRIPTORS[7],
            Block3dConfigMutation::SetBrushFlip { .. } => &Self::DESCRIPTORS[8],
            Block3dConfigMutation::SetCamera { .. } => &Self::DESCRIPTORS[9],
            Block3dConfigMutation::ClearCamera => &Self::DESCRIPTORS[10],
        }
    }

    fn diff(&self, base: &Block3dConfig) -> protocol::MutationOutcome<Block3dConfigDiff> {
        let unchanged = || protocol::MutationOutcome::empty().warning("mutation.no-op", "The config already holds that value.");
        let outcome = |diff: Block3dConfigDiff| protocol::MutationOutcome::new(diff);
        match self {
            Block3dConfigMutation::SetActiveRepresentation { representation_id } if *representation_id == base.active_representation_id => unchanged(),
            Block3dConfigMutation::SetActiveRepresentation { representation_id } => outcome(Block3dConfigDiff { active_representation_id: Some(BlockOptionalText { value: representation_id.clone() }), ..Default::default() }),
            Block3dConfigMutation::SetWantedTags { tags } if *tags == base.wanted_tags => unchanged(),
            Block3dConfigMutation::SetWantedTags { tags } => outcome(Block3dConfigDiff { wanted_tags: Some(tags.clone()), ..Default::default() }),
            Block3dConfigMutation::SetWindowRepresentations { window_id, representation_ids } if block3d_window_view(base, window_id).representation_ids == *representation_ids => unchanged(),
            Block3dConfigMutation::SetWindowRepresentations { window_id, representation_ids } => outcome(window_edit(base, window_id, Block3dWindowViewPatch { representation_ids: Some(representation_ids.clone()), ..Default::default() })),
            Block3dConfigMutation::ToggleWindowRepresentation { window_id, representation_id, visible } => {
                let current = block3d_window_view(base, window_id).representation_ids;
                let toggled: Vec<String> = match *visible {
                    true if current.contains(representation_id) => current.clone(),
                    true => current.iter().cloned().chain([representation_id.clone()]).collect(),
                    false => current.iter().filter(|id| *id != representation_id).cloned().collect(),
                };
                if toggled == current {
                    return unchanged();
                }
                outcome(window_edit(base, window_id, Block3dWindowViewPatch { representation_ids: Some(toggled), ..Default::default() }))
            }
            Block3dConfigMutation::SetWindowArrangement { window_id, arrangement } if block3d_window_view(base, window_id).arrangement == *arrangement => unchanged(),
            Block3dConfigMutation::SetWindowArrangement { window_id, arrangement } => outcome(window_edit(base, window_id, Block3dWindowViewPatch { arrangement: Some(arrangement.clone()), ..Default::default() })),
            Block3dConfigMutation::SetWindowSpacing { window_id, spacing } if block3d_window_view(base, window_id).spacing == *spacing => unchanged(),
            Block3dConfigMutation::SetWindowSpacing { window_id, spacing } => outcome(window_edit(base, window_id, Block3dWindowViewPatch { spacing: Some(*spacing), ..Default::default() })),
            Block3dConfigMutation::SetBrushVortexKind { vortex_kind_id } if *vortex_kind_id == base.brush_vortex_kind_id => unchanged(),
            Block3dConfigMutation::SetBrushVortexKind { vortex_kind_id } => outcome(Block3dConfigDiff { brush_vortex_kind_id: Some(BlockOptionalText { value: vortex_kind_id.clone() }), ..Default::default() }),
            Block3dConfigMutation::SetBrushRadius { radius } if *radius == base.brush_radius => unchanged(),
            Block3dConfigMutation::SetBrushRadius { radius } => outcome(Block3dConfigDiff { brush_radius: Some(*radius), ..Default::default() }),
            Block3dConfigMutation::SetBrushFlip { flip } if *flip == base.brush_flip => unchanged(),
            Block3dConfigMutation::SetBrushFlip { flip } => outcome(Block3dConfigDiff { brush_flip: Some(*flip), ..Default::default() }),
            Block3dConfigMutation::SetCamera { camera } if base.camera.as_ref() == Some(camera) => unchanged(),
            Block3dConfigMutation::SetCamera { camera } => outcome(Block3dConfigDiff { camera: Some(Block3dOptionalCamera { value: Some(camera.clone()) }), ..Default::default() }),
            Block3dConfigMutation::ClearCamera if base.camera.is_none() => unchanged(),
            Block3dConfigMutation::ClearCamera => outcome(Block3dConfigDiff { camera: Some(Block3dOptionalCamera { value: None }), ..Default::default() }),
        }
    }

    fn inverse(&self, base: &Block3dConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Block3dConfigMutation::SetActiveRepresentation { .. } => Block3dConfigMutation::SetActiveRepresentation { representation_id: base.active_representation_id.clone() },
            Block3dConfigMutation::SetWantedTags { .. } => Block3dConfigMutation::SetWantedTags { tags: base.wanted_tags.clone() },
            Block3dConfigMutation::SetWindowRepresentations { window_id, .. } | Block3dConfigMutation::ToggleWindowRepresentation { window_id, .. } => Block3dConfigMutation::SetWindowRepresentations { window_id: window_id.clone(), representation_ids: block3d_window_view(base, window_id).representation_ids },
            Block3dConfigMutation::SetWindowArrangement { window_id, .. } => Block3dConfigMutation::SetWindowArrangement { window_id: window_id.clone(), arrangement: block3d_window_view(base, window_id).arrangement },
            Block3dConfigMutation::SetWindowSpacing { window_id, .. } => Block3dConfigMutation::SetWindowSpacing { window_id: window_id.clone(), spacing: block3d_window_view(base, window_id).spacing },
            Block3dConfigMutation::SetBrushVortexKind { .. } => Block3dConfigMutation::SetBrushVortexKind { vortex_kind_id: base.brush_vortex_kind_id.clone() },
            Block3dConfigMutation::SetBrushRadius { .. } => Block3dConfigMutation::SetBrushRadius { radius: base.brush_radius },
            Block3dConfigMutation::SetBrushFlip { .. } => Block3dConfigMutation::SetBrushFlip { flip: base.brush_flip },
            Block3dConfigMutation::SetCamera { .. } | Block3dConfigMutation::ClearCamera => match &base.camera {
                Some(camera) => Block3dConfigMutation::SetCamera { camera: camera.clone() },
                None => Block3dConfigMutation::ClearCamera,
            },
        }])
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
