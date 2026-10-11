//! 🪟️ Exact-instance persisted and ephemeral ownership for Puzzle 2D panes.

use crate::editor::puzzle2d::config::Puzzle2dSuggestionMenu;
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use std::collections::BTreeMap;

/// 🎚️ ONE exact Puzzle 2D pane's persisted-local options. `WindowConfigOwner::State` requires
/// `dsl::DslField`, which `#[derive(dsl::DslArtifact)]` emits alongside the `__dsl_*` helpers the
/// record-backed `ArtifactDsl`/`ArtifactPack` below are written against; `id`/`extension` are stated
/// explicitly so the derived `__DSL_ENVELOPE_ID`/`__DSL_EXTENSION` reproduce the envelope identity
/// the three panes already shared.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.puzzle.puzzle2d.windowconfig", extension = "puzzle2dwindowcfg")]
pub struct Puzzle2dWindowConfig {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
    pub lod_mode: String,
    pub grid_visible: bool,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub suggestion_offset: f64,
    pub proximity_radius: f64,
    pub area_brush_width: f64,
    pub area_brush_height: f64,
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub selectable_nodes: bool,
    pub selectable_handles: bool,
    pub selectable_edges: bool,
}

impl Default for Puzzle2dWindowConfig {
    fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            lod_mode: crate::editor::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.into(),
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: 1.0,
            suggestion_offset: crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_SUGGESTION_OFFSET,
            proximity_radius: crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_PROXIMITY_RADIUS,
            area_brush_width: crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_AREA_BRUSH_EXTENT,
            area_brush_height: crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_AREA_BRUSH_EXTENT,
            transform_move: true,
            transform_rotate: true,
            selectable_nodes: true,
            selectable_handles: true,
            selectable_edges: true,
        }
    }
}

/// 🧩️ Payload of [`Puzzle2dWindowConfigMutation::Set`]: the absolute value of exactly the diff's named fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Puzzle2dWindowConfigMutationSet {
    pub patch: Puzzle2dWindowConfigDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum Puzzle2dWindowConfigMutation {
    Set(Puzzle2dWindowConfigMutationSet),
}

impl semio_framework_plugin::WindowConfigApplyMutation<Puzzle2dWindowConfig> for Puzzle2dWindowConfigMutation {
    fn exchange(self, post: &mut Puzzle2dWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let Self::Set(Puzzle2dWindowConfigMutationSet { mut patch }) = self;
        if let Some(value) = patch.camera_x.as_mut() {
            std::mem::swap(value, &mut post.camera_x);
        }
        if let Some(value) = patch.camera_y.as_mut() {
            std::mem::swap(value, &mut post.camera_y);
        }
        if let Some(value) = patch.camera_zoom.as_mut() {
            std::mem::swap(value, &mut post.camera_zoom);
        }
        if let Some(value) = patch.lod_mode.as_mut() {
            std::mem::swap(value, &mut post.lod_mode);
        }
        if let Some(value) = patch.grid_visible.as_mut() {
            std::mem::swap(value, &mut post.grid_visible);
        }
        if let Some(value) = patch.grid_snap_enabled.as_mut() {
            std::mem::swap(value, &mut post.grid_snap_enabled);
        }
        if let Some(value) = patch.grid_factor.as_mut() {
            std::mem::swap(value, &mut post.grid_factor);
        }
        if let Some(value) = patch.suggestion_offset.as_mut() {
            std::mem::swap(value, &mut post.suggestion_offset);
        }
        if let Some(value) = patch.proximity_radius.as_mut() {
            std::mem::swap(value, &mut post.proximity_radius);
        }
        if let Some(value) = patch.area_brush_width.as_mut() {
            std::mem::swap(value, &mut post.area_brush_width);
        }
        if let Some(value) = patch.area_brush_height.as_mut() {
            std::mem::swap(value, &mut post.area_brush_height);
        }
        if let Some(value) = patch.transform_move.as_mut() {
            std::mem::swap(value, &mut post.transform_move);
        }
        if let Some(value) = patch.transform_rotate.as_mut() {
            std::mem::swap(value, &mut post.transform_rotate);
        }
        if let Some(value) = patch.selectable_nodes.as_mut() {
            std::mem::swap(value, &mut post.selectable_nodes);
        }
        if let Some(value) = patch.selectable_handles.as_mut() {
            std::mem::swap(value, &mut post.selectable_handles);
        }
        if let Some(value) = patch.selectable_edges.as_mut() {
            std::mem::swap(value, &mut post.selectable_edges);
        }
        Ok(Self::Set(Puzzle2dWindowConfigMutationSet { patch }))
    }

    fn payload_bytes(&self) -> usize {
        let Self::Set(Puzzle2dWindowConfigMutationSet { patch }) = self;
        0 + patch.lod_mode.as_ref().map_or(0, String::len)
    }
}

impl protocol::Mutation<Puzzle2dWindowConfig> for Puzzle2dWindowConfigMutation {
    type Diff = Puzzle2dWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-config", display_name: "Set Puzzle 2D Window Configuration", emoji: "🪟️", aggregate_variant: "Set", payload_schema: "puzzle.2dwindowconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, base: &Puzzle2dWindowConfig) -> protocol::MutationOutcome<Puzzle2dWindowConfigDiff> {
        let Self::Set(Puzzle2dWindowConfigMutationSet { patch }) = self;
        let diff = patch.changed(base);
        if protocol::DiffAlgebra::<Puzzle2dWindowConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds these values.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Puzzle2dWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        let Self::Set(Puzzle2dWindowConfigMutationSet { patch }) = self;
        Ok(vec![Self::Set(Puzzle2dWindowConfigMutationSet { patch: patch.restoring(base) })])
    }
}

macro_rules! json_store {
    ($state:ty, $extension:literal, $envelope:literal) => {
        impl store::ArtifactDsl for $state {
            const EXTENSION: &'static str = $extension;
            fn envelope_id() -> &'static str {
                $envelope
            }
            fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
                semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
            }
            fn print_dsl(&self) -> String {
                semio_framework_pack_json::to_json_string(self)
            }
        }
        impl store::ArtifactPack for $state {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                semio_framework_value::ToValue::to_value(self).encode_pack_with(options)
            }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                let value = semio_framework_value::DslValue::decode_pack_with(bytes, options)?;
                semio_framework_value::FromValue::from_value(value).map_err(|error| store::PackError::from(error))
            }
        }
    };
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for Puzzle2dWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Puzzle 2D window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// these window kinds with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for Puzzle2dWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ConfigRecord for Puzzle2dWindowConfig {}

/// 🔺️ Sparse typed delta of one Puzzle 2D pane's persisted-local options: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dWindowConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_zoom: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_mode: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_snap_enabled: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_factor: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub proximity_radius: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub area_brush_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub area_brush_height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_move: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_rotate: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_nodes: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_handles: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_edges: Option<bool>,
}

impl Puzzle2dWindowConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle2dWindowConfig) -> Self {
        Self { camera_x: Some(state.camera_x), camera_y: Some(state.camera_y), camera_zoom: Some(state.camera_zoom), lod_mode: Some(state.lod_mode.clone()), grid_visible: Some(state.grid_visible), grid_snap_enabled: Some(state.grid_snap_enabled), grid_factor: Some(state.grid_factor), suggestion_offset: Some(state.suggestion_offset), proximity_radius: Some(state.proximity_radius), area_brush_width: Some(state.area_brush_width), area_brush_height: Some(state.area_brush_height), transform_move: Some(state.transform_move), transform_rotate: Some(state.transform_rotate), selectable_nodes: Some(state.selectable_nodes), selectable_handles: Some(state.selectable_handles), selectable_edges: Some(state.selectable_edges) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle2dWindowConfig) -> Self {
        Self { camera_x: self.camera_x.as_ref().filter(|value| **value != base.camera_x).cloned(), camera_y: self.camera_y.as_ref().filter(|value| **value != base.camera_y).cloned(), camera_zoom: self.camera_zoom.as_ref().filter(|value| **value != base.camera_zoom).cloned(), lod_mode: self.lod_mode.as_ref().filter(|value| **value != base.lod_mode).cloned(), grid_visible: self.grid_visible.as_ref().filter(|value| **value != base.grid_visible).cloned(), grid_snap_enabled: self.grid_snap_enabled.as_ref().filter(|value| **value != base.grid_snap_enabled).cloned(), grid_factor: self.grid_factor.as_ref().filter(|value| **value != base.grid_factor).cloned(), suggestion_offset: self.suggestion_offset.as_ref().filter(|value| **value != base.suggestion_offset).cloned(), proximity_radius: self.proximity_radius.as_ref().filter(|value| **value != base.proximity_radius).cloned(), area_brush_width: self.area_brush_width.as_ref().filter(|value| **value != base.area_brush_width).cloned(), area_brush_height: self.area_brush_height.as_ref().filter(|value| **value != base.area_brush_height).cloned(), transform_move: self.transform_move.as_ref().filter(|value| **value != base.transform_move).cloned(), transform_rotate: self.transform_rotate.as_ref().filter(|value| **value != base.transform_rotate).cloned(), selectable_nodes: self.selectable_nodes.as_ref().filter(|value| **value != base.selectable_nodes).cloned(), selectable_handles: self.selectable_handles.as_ref().filter(|value| **value != base.selectable_handles).cloned(), selectable_edges: self.selectable_edges.as_ref().filter(|value| **value != base.selectable_edges).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle2dWindowConfig) -> Self {
        Self { camera_x: self.camera_x.as_ref().map(|_| base.camera_x), camera_y: self.camera_y.as_ref().map(|_| base.camera_y), camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom), lod_mode: self.lod_mode.as_ref().map(|_| base.lod_mode.clone()), grid_visible: self.grid_visible.as_ref().map(|_| base.grid_visible), grid_snap_enabled: self.grid_snap_enabled.as_ref().map(|_| base.grid_snap_enabled), grid_factor: self.grid_factor.as_ref().map(|_| base.grid_factor), suggestion_offset: self.suggestion_offset.as_ref().map(|_| base.suggestion_offset), proximity_radius: self.proximity_radius.as_ref().map(|_| base.proximity_radius), area_brush_width: self.area_brush_width.as_ref().map(|_| base.area_brush_width), area_brush_height: self.area_brush_height.as_ref().map(|_| base.area_brush_height), transform_move: self.transform_move.as_ref().map(|_| base.transform_move), transform_rotate: self.transform_rotate.as_ref().map(|_| base.transform_rotate), selectable_nodes: self.selectable_nodes.as_ref().map(|_| base.selectable_nodes), selectable_handles: self.selectable_handles.as_ref().map(|_| base.selectable_handles), selectable_edges: self.selectable_edges.as_ref().map(|_| base.selectable_edges) }
    }
}

impl protocol::MutationDiff<Puzzle2dWindowConfig> for Puzzle2dWindowConfigDiff {
    fn apply(&self, base: &Puzzle2dWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_x {
            next.camera_x = *value;
        }
        if let Some(value) = &self.camera_y {
            next.camera_y = *value;
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom = *value;
        }
        if let Some(value) = &self.lod_mode {
            next.lod_mode = value.clone();
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.grid_snap_enabled {
            next.grid_snap_enabled = *value;
        }
        if let Some(value) = &self.grid_factor {
            next.grid_factor = *value;
        }
        if let Some(value) = &self.suggestion_offset {
            next.suggestion_offset = *value;
        }
        if let Some(value) = &self.proximity_radius {
            next.proximity_radius = *value;
        }
        if let Some(value) = &self.area_brush_width {
            next.area_brush_width = *value;
        }
        if let Some(value) = &self.area_brush_height {
            next.area_brush_height = *value;
        }
        if let Some(value) = &self.transform_move {
            next.transform_move = *value;
        }
        if let Some(value) = &self.transform_rotate {
            next.transform_rotate = *value;
        }
        if let Some(value) = &self.selectable_nodes {
            next.selectable_nodes = *value;
        }
        if let Some(value) = &self.selectable_handles {
            next.selectable_handles = *value;
        }
        if let Some(value) = &self.selectable_edges {
            next.selectable_edges = *value;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera_x.is_some() {
            self.camera_x = other.camera_x;
        }
        if other.camera_y.is_some() {
            self.camera_y = other.camera_y;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
        if other.lod_mode.is_some() {
            self.lod_mode = other.lod_mode;
        }
        if other.grid_visible.is_some() {
            self.grid_visible = other.grid_visible;
        }
        if other.grid_snap_enabled.is_some() {
            self.grid_snap_enabled = other.grid_snap_enabled;
        }
        if other.grid_factor.is_some() {
            self.grid_factor = other.grid_factor;
        }
        if other.suggestion_offset.is_some() {
            self.suggestion_offset = other.suggestion_offset;
        }
        if other.proximity_radius.is_some() {
            self.proximity_radius = other.proximity_radius;
        }
        if other.area_brush_width.is_some() {
            self.area_brush_width = other.area_brush_width;
        }
        if other.area_brush_height.is_some() {
            self.area_brush_height = other.area_brush_height;
        }
        if other.transform_move.is_some() {
            self.transform_move = other.transform_move;
        }
        if other.transform_rotate.is_some() {
            self.transform_rotate = other.transform_rotate;
        }
        if other.selectable_nodes.is_some() {
            self.selectable_nodes = other.selectable_nodes;
        }
        if other.selectable_handles.is_some() {
            self.selectable_handles = other.selectable_handles;
        }
        if other.selectable_edges.is_some() {
            self.selectable_edges = other.selectable_edges;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle2dWindowConfig> for Puzzle2dWindowConfigDiff {
    fn inverse(&self, base: &Puzzle2dWindowConfig) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_zoom.is_none() && self.lod_mode.is_none() && self.grid_visible.is_none() && self.grid_snap_enabled.is_none() && self.grid_factor.is_none() && self.suggestion_offset.is_none() && self.proximity_radius.is_none() && self.area_brush_width.is_none() && self.area_brush_height.is_none() && self.transform_move.is_none() && self.transform_rotate.is_none() && self.selectable_nodes.is_none() && self.selectable_handles.is_none() && self.selectable_edges.is_none()
    }
}


impl protocol::OpText for Puzzle2dWindowConfigMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
impl protocol::OpBinary for Puzzle2dWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dWindowTransient {
    pub engagement_input: String,
    pub brush_candidate_index: usize,
    pub brush_candidates: Vec<semio_framework_value::DslValue>,
    pub brush_candidate_source_handle_id: String,
    /// 💡️ The one-shot handle-suggestions popup this exact window has open — per-gesture scratch,
    /// never a persisted option, and never shared with a sibling pane.
    pub suggestion_menu: Option<Puzzle2dSuggestionMenu>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle2dWindowTransientMutation {
    Snapshot { transient: Puzzle2dWindowTransient },
}

impl protocol::Mutation<Puzzle2dWindowTransient> for Puzzle2dWindowTransientMutation {
    type Diff = Puzzle2dWindowTransientDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-transient",
        display_name: "Set Puzzle 2D Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "puzzle.2dwindowtransient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, base: &Puzzle2dWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        let Self::Snapshot { transient } = self;
        let diff = Puzzle2dWindowTransientDiff::of(transient).changed(base);
        if protocol::DiffAlgebra::<Puzzle2dWindowTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window transient already holds this state.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Puzzle2dWindowTransient) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::Snapshot { transient: base.clone() }]
    
    })())
}
}

/// 🕳️ Tri-state decode of every `Option<Option<T>>` diff slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🔺️ Sparse typed delta of one Puzzle 2D pane's ephemeral interaction state: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dWindowTransientDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brush_candidate_index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brush_candidates: Option<Vec<semio_framework_value::DslValue>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brush_candidate_source_handle_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub suggestion_menu: Option<Option<Puzzle2dSuggestionMenu>>,
}

impl Puzzle2dWindowTransientDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle2dWindowTransient) -> Self {
        Self { engagement_input: Some(state.engagement_input.clone()), brush_candidate_index: Some(state.brush_candidate_index), brush_candidates: Some(state.brush_candidates.clone()), brush_candidate_source_handle_id: Some(state.brush_candidate_source_handle_id.clone()), suggestion_menu: Some(state.suggestion_menu.clone()) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle2dWindowTransient) -> Self {
        Self { engagement_input: self.engagement_input.as_ref().filter(|value| **value != base.engagement_input).cloned(), brush_candidate_index: self.brush_candidate_index.as_ref().filter(|value| **value != base.brush_candidate_index).cloned(), brush_candidates: self.brush_candidates.as_ref().filter(|value| **value != base.brush_candidates).cloned(), brush_candidate_source_handle_id: self.brush_candidate_source_handle_id.as_ref().filter(|value| **value != base.brush_candidate_source_handle_id).cloned(), suggestion_menu: self.suggestion_menu.as_ref().filter(|value| **value != base.suggestion_menu).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle2dWindowTransient) -> Self {
        Self { engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), brush_candidate_index: self.brush_candidate_index.as_ref().map(|_| base.brush_candidate_index), brush_candidates: self.brush_candidates.as_ref().map(|_| base.brush_candidates.clone()), brush_candidate_source_handle_id: self.brush_candidate_source_handle_id.as_ref().map(|_| base.brush_candidate_source_handle_id.clone()), suggestion_menu: self.suggestion_menu.as_ref().map(|_| base.suggestion_menu.clone()) }
    }
}

impl protocol::MutationDiff<Puzzle2dWindowTransient> for Puzzle2dWindowTransientDiff {
    fn apply(&self, base: &Puzzle2dWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dWindowTransient> {
        let mut next = base.clone();
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.brush_candidate_index {
            next.brush_candidate_index = *value;
        }
        if let Some(value) = &self.brush_candidates {
            next.brush_candidates = value.clone();
        }
        if let Some(value) = &self.brush_candidate_source_handle_id {
            next.brush_candidate_source_handle_id = value.clone();
        }
        if let Some(value) = &self.suggestion_menu {
            next.suggestion_menu = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.brush_candidate_index.is_some() {
            self.brush_candidate_index = other.brush_candidate_index;
        }
        if other.brush_candidates.is_some() {
            self.brush_candidates = other.brush_candidates;
        }
        if other.brush_candidate_source_handle_id.is_some() {
            self.brush_candidate_source_handle_id = other.brush_candidate_source_handle_id;
        }
        if other.suggestion_menu.is_some() {
            self.suggestion_menu = other.suggestion_menu;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle2dWindowTransient> for Puzzle2dWindowTransientDiff {
    fn inverse(&self, base: &Puzzle2dWindowTransient) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none() && self.brush_candidate_index.is_none() && self.brush_candidates.is_none() && self.brush_candidate_source_handle_id.is_none() && self.suggestion_menu.is_none()
    }
}

semio_framework_value::artifact_retire_struct!(Puzzle2dSuggestionMenu { x, y, window_id, handle_id });
semio_framework_value::artifact_retire_struct!(Puzzle2dWindowTransient { engagement_input, brush_candidate_index, brush_candidates, brush_candidate_source_handle_id, suggestion_menu });

impl semio_framework_value::retirement::RetireOwned for Puzzle2dWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn puzzle2d_window_transient_retained_bytes(transient: &Puzzle2dWindowTransient) -> Option<usize> {
    fn charge(bytes: &mut usize, increment: usize) -> Option<()> {
        *bytes = bytes.checked_add(increment)?;
        (*bytes <= store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).then_some(())
    }

    let mut bytes = 0;
    charge(&mut bytes, std::mem::size_of::<Puzzle2dWindowTransient>())?;
    charge(&mut bytes, transient.engagement_input.capacity())?;
    charge(&mut bytes, transient.brush_candidate_source_handle_id.capacity())?;
    if let Some(menu) = transient.suggestion_menu.as_ref() {
        charge(&mut bytes, menu.window_id.capacity())?;
        charge(&mut bytes, menu.handle_id.capacity())?;
    }
    charge(&mut bytes, transient.brush_candidates.capacity().checked_mul(std::mem::size_of::<semio_framework_value::DslValue>())?)?;
    let mut pending = Vec::new();
    pending.try_reserve(transient.brush_candidates.len()).ok()?;
    pending.extend(transient.brush_candidates.iter());
    while let Some(value) = pending.pop() {
        match value {
            semio_framework_value::DslValue::String(value) => charge(&mut bytes, value.capacity())?,
            semio_framework_value::DslValue::Bytes(value) => charge(&mut bytes, value.capacity())?,
            semio_framework_value::DslValue::Array(values) => {
                charge(&mut bytes, values.capacity().checked_mul(std::mem::size_of::<semio_framework_value::DslValue>())?)?;
                pending.try_reserve(values.len()).ok()?;
                pending.extend(values);
            }
            semio_framework_value::DslValue::Object(entries) => {
                charge(&mut bytes, entries.capacity().checked_mul(std::mem::size_of::<(String, semio_framework_value::DslValue)>())?)?;
                pending.try_reserve(entries.len()).ok()?;
                for (key, value) in entries {
                    charge(&mut bytes, key.capacity())?;
                    pending.push(value);
                }
            }
            semio_framework_value::DslValue::Null | semio_framework_value::DslValue::Bool(_) | semio_framework_value::DslValue::Number(_) => {}
        }
    }
    Some(bytes)
}

fn puzzle2d_window_transient_preflight(mutation: &Puzzle2dWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Puzzle2dWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = puzzle2d_window_transient_retained_bytes(transient).ok_or_else(|| "Puzzle 2D window transient exceeds its exact retained publication envelope".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint::for_ephemeral_item(retained_bytes))
}

fn puzzle2d_window_transient_transfer(mutation: Puzzle2dWindowTransientMutation) -> Puzzle2dWindowTransient {
    match mutation {
        Puzzle2dWindowTransientMutation::Snapshot { transient } => transient,
    }
}

json_store!(Puzzle2dWindowTransient, "puzzle2dwindowtransient", "s.puzzle.puzzle2d.windowtransient");
impl protocol::OpText for Puzzle2dWindowTransientMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
impl protocol::OpBinary for Puzzle2dWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

macro_rules! owners {
    ($config:ident, $transient:ident, $kind:expr) => {
        pub struct $config;
        impl semio_framework_plugin::WindowConfigOwner for $config {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = "puzzle.2dwindowconfig";
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = Puzzle2dWindowConfig;
            type Mutation = Puzzle2dWindowConfigMutation;
            type Edit = semio_framework_plugin::WindowConfigApplyEdit<Puzzle2dWindowConfig, Puzzle2dWindowConfigMutation>;
            const MAXIMUM_PREPARATION_DEPTH: usize = 64;
            fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
                std::sync::Arc::new(semio_framework_plugin::WindowConfigApplyEdit::new())
            }
            fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
                semio_framework_plugin::bounded_window_config_store_owners::<Self>()
            }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
            }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
            }
        }
        pub struct $transient;
        impl semio_framework_plugin::WindowTransientOwner for $transient {
            const WINDOW_KIND_ID: &'static str = $kind;
            type State = Puzzle2dWindowTransient;
            type Mutation = Puzzle2dWindowTransientMutation;
            fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
                    puzzle2d_window_transient_preflight,
                    puzzle2d_window_transient_transfer,
                    state.clone(),
                    mutation.clone(),
                ));
                semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
            }
        }
    };
}

owners!(Puzzle2dOverviewWindowConfigOwner, Puzzle2dOverviewWindowTransientOwner, overview::WINDOW_KIND_ID);
owners!(Puzzle2dDetailWindowConfigOwner, Puzzle2dDetailWindowTransientOwner, detail::WINDOW_KIND_ID);
owners!(Puzzle2dSelectionWindowConfigOwner, Puzzle2dSelectionWindowTransientOwner, selection::WINDOW_KIND_ID);

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle2dOverviewWindowConfigOwner>()?;
    registry.register::<Puzzle2dDetailWindowConfigOwner>()?;
    registry.register::<Puzzle2dSelectionWindowConfigOwner>()
}

pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle2dOverviewWindowTransientOwner>()?;
    registry.register::<Puzzle2dDetailWindowTransientOwner>()?;
    registry.register::<Puzzle2dSelectionWindowTransientOwner>()
}

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, crate::editor::puzzle2d::config::Puzzle2dConfig>) -> Puzzle2dWindowConfig {
    view.window::<Puzzle2dOverviewWindowConfigOwner>().or_else(|| view.window::<Puzzle2dDetailWindowConfigOwner>()).or_else(|| view.window::<Puzzle2dSelectionWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Puzzle2dWindowConfig {
    snapshot.and_then(|value| value.get::<Puzzle2dOverviewWindowConfigOwner>().or_else(|| value.get::<Puzzle2dDetailWindowConfigOwner>()).or_else(|| value.get::<Puzzle2dSelectionWindowConfigOwner>())).cloned().unwrap_or_default()
}

pub fn document_seed(document: &semio_framework_pack_json::Value) -> Puzzle2dWindowConfig {
    let mut seed = Puzzle2dWindowConfig::default();
    if let Some(camera) = document.get("camera") {
        seed.camera_x = camera.get("x").and_then(semio_framework_pack_json::Value::as_f64).unwrap_or(seed.camera_x);
        seed.camera_y = camera.get("y").and_then(semio_framework_pack_json::Value::as_f64).unwrap_or(seed.camera_y);
        seed.camera_zoom = camera.get("zoom").and_then(semio_framework_pack_json::Value::as_f64).filter(|zoom| zoom.is_finite() && *zoom > 0.0).unwrap_or(seed.camera_zoom);
    }
    seed
}

pub fn config_from_view_or_document(view: &semio_framework_plugin::ConfigView<'_, crate::editor::puzzle2d::config::Puzzle2dConfig>, document: &semio_framework_pack_json::Value) -> Puzzle2dWindowConfig {
    let config = config_from_view(view);
    if view.window.is_some_and(|snapshot| snapshot.generation() == 0) && config == Puzzle2dWindowConfig::default() {
        document_seed(document)
    } else {
        config
    }
}

pub fn config_from_snapshot_or_document(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>, document: &semio_framework_pack_json::Value) -> Puzzle2dWindowConfig {
    let config = config_from_snapshot(snapshot);
    if snapshot.is_some_and(|snapshot| snapshot.generation() == 0) && config == Puzzle2dWindowConfig::default() {
        document_seed(document)
    } else {
        config
    }
}

pub fn transient_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> Puzzle2dWindowTransient {
    snapshot.and_then(|value| value.get::<Puzzle2dOverviewWindowTransientOwner>().or_else(|| value.get::<Puzzle2dDetailWindowTransientOwner>()).or_else(|| value.get::<Puzzle2dSelectionWindowTransientOwner>())).cloned().unwrap_or_default()
}

pub fn transient_from_view(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> Puzzle2dWindowTransient {
    view.window::<Puzzle2dOverviewWindowTransientOwner>().or_else(|| view.window::<Puzzle2dDetailWindowTransientOwner>()).or_else(|| view.window::<Puzzle2dSelectionWindowTransientOwner>()).cloned().unwrap_or_default()
}

fn kind(view: &semio_framework_plugin::ViewModel) -> Option<(&str, &str)> {
    let id = view.window_id.as_deref()?;
    let kind = view.window_instances.iter().find(|window| window.id == id)?.window_kind_id.as_str();
    Some((id, kind))
}

pub fn kind_for_view(view: &semio_framework_plugin::ViewModel) -> Option<&str> {
    kind(view).map(|(_, kind)| kind)
}

pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Puzzle2dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let (id, kind) = kind(view).ok_or_else(|| semio_framework_plugin::Fault::from("puzzle2d-window-required"))?;
    let mutation = Puzzle2dWindowConfigMutation::Set(Puzzle2dWindowConfigMutationSet { patch: Puzzle2dWindowConfigDiff::of(&config) });
    match kind {
        overview::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle2dOverviewWindowConfigOwner>(id, mutation)),
        detail::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle2dDetailWindowConfigOwner>(id, mutation)),
        selection::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle2dSelectionWindowConfigOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("puzzle2d-window-kind-required")),
    }
}

pub fn addressed_transient(view: &semio_framework_plugin::ViewModel, transient: Puzzle2dWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let (id, kind) = kind(view).ok_or_else(|| semio_framework_plugin::Fault::from("puzzle2d-window-required"))?;
    let mutation = Puzzle2dWindowTransientMutation::Snapshot { transient };
    match kind {
        overview::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle2dOverviewWindowTransientOwner>(id, mutation)),
        detail::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle2dDetailWindowTransientOwner>(id, mutation)),
        selection::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle2dSelectionWindowTransientOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("puzzle2d-window-kind-required")),
    }
}

pub fn runtime(config: &crate::editor::puzzle2d::config::Puzzle2dConfig, window: &Puzzle2dWindowConfig, transient: &Puzzle2dWindowTransient, window_kind: Option<&str>) -> crate::editor::puzzle2d::config::Puzzle2dPlayRuntime {
    let kind = window_kind.unwrap_or(overview::WINDOW_KIND_ID);
    let mut lod = BTreeMap::new();
    lod.insert(kind.to_string(), window.lod_mode.clone());
    let mut engagement = BTreeMap::new();
    engagement.insert(kind.to_string(), transient.engagement_input.clone());
    crate::editor::puzzle2d::config::Puzzle2dPlayRuntime {
        camera_x: window.camera_x,
        camera_y: window.camera_y,
        camera_zoom: window.camera_zoom,
        lod_mode_by_pane: lod,
        engagement_input_by_pane: engagement,
        brush_candidate_index: transient.brush_candidate_index,
        brush_candidates: transient.brush_candidates.clone(),
        brush_candidate_source_handle_id: transient.brush_candidate_source_handle_id.clone(),
        suggestion_menu: transient.suggestion_menu.clone(),
        fill_count: config.fill_count,
        grid_snap_enabled: window.grid_snap_enabled,
        grid_factor: window.grid_factor,
        suggestion_offset: window.suggestion_offset,
        proximity_radius: window.proximity_radius,
        area_brush_width: window.area_brush_width,
        area_brush_height: window.area_brush_height,
        grid_visible: window.grid_visible,
        transform_move: window.transform_move,
        transform_rotate: window.transform_rotate,
        selectable_kinds: crate::editor::puzzle2d::config::Puzzle2dSelectableKinds { nodes: window.selectable_nodes, handles: window.selectable_handles, edges: window.selectable_edges },
        contact_tolerance: config.contact_tolerance,
        brush_placement_overlap_budget: config.brush_placement_overlap_budget,
        node_kind_weights: config.node_kind_weights.clone(),
        handle_kind_weights: config.handle_kind_weights.clone(),
        ..Default::default()
    }
}

pub fn split(runtime: &crate::editor::puzzle2d::config::Puzzle2dPlayRuntime, window_kind: &str) -> (crate::editor::puzzle2d::config::Puzzle2dConfig, Puzzle2dWindowConfig, Puzzle2dWindowTransient) {
    (
        crate::editor::puzzle2d::config::Puzzle2dConfig {
            node_kind_weights: runtime.node_kind_weights.clone(),
            handle_kind_weights: runtime.handle_kind_weights.clone(),
            fill_count: runtime.fill_count,
            contact_tolerance: runtime.contact_tolerance,
            brush_placement_overlap_budget: runtime.brush_placement_overlap_budget,
        },
        Puzzle2dWindowConfig {
            camera_x: runtime.camera_x,
            camera_y: runtime.camera_y,
            camera_zoom: runtime.camera_zoom,
            lod_mode: runtime.lod_mode_by_pane.get(window_kind).cloned().unwrap_or_else(|| crate::editor::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.into()),
            grid_visible: runtime.grid_visible,
            grid_snap_enabled: runtime.grid_snap_enabled,
            grid_factor: runtime.grid_factor,
            suggestion_offset: runtime.suggestion_offset,
            proximity_radius: runtime.proximity_radius,
            area_brush_width: runtime.area_brush_width,
            area_brush_height: runtime.area_brush_height,
            transform_move: runtime.transform_move,
            transform_rotate: runtime.transform_rotate,
            selectable_nodes: runtime.selectable_kinds.nodes,
            selectable_handles: runtime.selectable_kinds.handles,
            selectable_edges: runtime.selectable_kinds.edges,
        },
        Puzzle2dWindowTransient {
            engagement_input: runtime.engagement_input_by_pane.get(window_kind).cloned().unwrap_or_default(),
            brush_candidate_index: runtime.brush_candidate_index,
            brush_candidates: runtime.brush_candidates.clone(),
            brush_candidate_source_handle_id: runtime.brush_candidate_source_handle_id.clone(),
            suggestion_menu: runtime.suggestion_menu.clone(),
        },
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🪢️TaxonomyMounts
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
//#endregion 🪢️TaxonomyMounts
