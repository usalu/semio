//! 🪟️ Exact-instance Puzzle 3D window configuration and transient interaction owners.

use crate::editor::puzzle3d::config::{Puzzle3dCamera, Puzzle3dConfig, Puzzle3dRuntime, Puzzle3dSelectableKinds, Puzzle3dSuggestionMenu};
use crate::editor::puzzle3d::modes::edit::windows::main;
use semio_framework_plugin::WorldSunConfig;

/// 🎚️ ONE exact Puzzle 3D window instance's persisted-local options. `WindowConfigOwner::State`
/// requires `dsl::DslField`, which `#[derive(dsl::DslArtifact)]` emits alongside the `__dsl_*`
/// helpers `ArtifactDsl`/`ArtifactPack` below are written against; `id`/`extension` are stated
/// explicitly so the derived `__DSL_ENVELOPE_ID`/`__DSL_EXTENSION` reproduce the envelope identity
/// this window kind already carried.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.puzzle.puzzle3d.windowconfig", extension = "puzzle3dwindowcfg")]
pub struct Puzzle3dWindowConfig {
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub grid_visible: bool,
    pub lod_manual: f64,
    pub grid_snap_enabled: bool,
    pub grid_spacing: f64,
    #[dsl(block)]
    pub selectable_kinds: Puzzle3dSelectableKinds,
    pub proximity_radius: f64,
    pub chunk_size: f64,
    pub voxel_dims: [u32; 3],
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub vortex_show: String,
    pub vortex_direction: String,
    /// 🖱️ How a viewport drag sweeps a selection — `PUZZLE3D_SELECTION_METHOD_PICK`/`…_RECTANGLE`/
    /// `…_LASSO`. A window option like `vortex_show`, not activation scratch: switching utility or
    /// tool must not silently put the marquee back to a shape the user did not ask for.
    pub selection_method: String,
    #[dsl(block)]
    pub sun: WorldSunConfig,
    #[dsl(block)]
    pub camera: Puzzle3dCamera,
}

impl Default for Puzzle3dWindowConfig {
    fn default() -> Self {
        let runtime = Puzzle3dRuntime::default();
        Self::from_runtime(&runtime)
    }
}

impl Puzzle3dWindowConfig {
    pub fn from_runtime(runtime: &Puzzle3dRuntime) -> Self {
        Self {
            lod_automatic: runtime.lod_automatic,
            lod_depth_variable: runtime.lod_depth_variable,
            grid_visible: runtime.grid_visible,
            lod_manual: runtime.lod_manual,
            grid_snap_enabled: runtime.grid_snap_enabled,
            grid_spacing: runtime.grid_spacing,
            selectable_kinds: runtime.selectable_kinds.clone(),
            proximity_radius: runtime.proximity_radius,
            chunk_size: runtime.chunk_size,
            voxel_dims: runtime.voxel_dims,
            transform_move: runtime.transform_move,
            transform_rotate: runtime.transform_rotate,
            vortex_show: runtime.vortex_show.clone(),
            vortex_direction: runtime.vortex_direction.clone(),
            selection_method: runtime.selection_method.clone(),
            sun: runtime.sun.clone(),
            camera: runtime.camera.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle3dWindowConfigMutation {
    Set { patch: Puzzle3dWindowConfigDiff },
}

impl protocol::Mutation<Puzzle3dWindowConfig> for Puzzle3dWindowConfigMutation {
    type Diff = Puzzle3dWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-config", display_name: "Set Puzzle 3D Window Configuration", emoji: "🪟️", aggregate_variant: "Set", payload_schema: "puzzle.3dwindowconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, base: &Puzzle3dWindowConfig) -> protocol::MutationOutcome<Puzzle3dWindowConfigDiff> {
        let Self::Set { patch } = self;
        let diff = patch.changed(base);
        if protocol::DiffAlgebra::<Puzzle3dWindowConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds these values.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Puzzle3dWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        let Self::Set { patch } = self;
        Ok(vec![Self::Set { patch: patch.restoring(base) }])
    }
}

/// 🫧️ One window instance's interaction scratch: the one-shot suggestion popup, the engagement input
/// line and the brush candidate the popup is hovering. All three belong to ONE host activation — the
/// mode-wide active tool, or that window's active utility — and none of them may outlive it.
///
/// 🏛️ [`Self::activation`] is the id the scratch was captured under, and NOT a plugin-side copy of the
/// host's session state: the app never reads it to answer "what is active", only to answer "is what I
/// am holding still mine". The host stays the sole authority (`ViewModel::active_tool_id` /
/// `active_utility_id`, `📓️2026-09-09-peer-config-runtime-split.md` §1(d)); [`runtime`] compares the
/// two on every `handle`/`render`/`window_measures` call and drops scratch whose activation has moved
/// on. This replaces the clearing the `setActiveTool` reducer used to do: the framework dispatches
/// `setActiveTool`/`setActiveUtility` as an empty `Emit` (`🔌️plugin/🦀️.rs` `dispatch_action`), so no
/// app reducer ever runs for them, and a push-based framework hook would both re-introduce that
/// duplicate and still miss every activation change that arrives as a plain refresh.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dWindowTransient {
    pub suggestion_menu: Option<Puzzle3dSuggestionMenu>,
    pub engagement_input: String,
    pub brush_candidate_index: usize,
    pub activation: String,
}

/// 🏛️ The host activation this window is under: the mode-wide active tool wins, then the window's own
/// active utility (`ViewModel::for_window_instance` stamps `active_utility_id` from the per-window
/// map), else nothing. The two are mutually exclusive by the shell's own rule.
pub fn host_activation(view: Option<&semio_framework_plugin::ViewModel>) -> String {
    view.and_then(|view| view.active_tool_id.clone().or_else(|| view.active_utility_id.clone())).unwrap_or_default()
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle3dWindowTransientMutation { Snapshot { transient: Puzzle3dWindowTransient } }

impl protocol::Mutation<Puzzle3dWindowTransient> for Puzzle3dWindowTransientMutation {
    type Diff = Puzzle3dWindowTransientDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-transient", display_name: "Set Puzzle 3D Window Transient", emoji: "🫧️", aggregate_variant: "Snapshot", payload_schema: "puzzle.3dwindowtransient", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &Puzzle3dWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        let Self::Snapshot { transient } = self;
        let diff = Puzzle3dWindowTransientDiff::of(transient).changed(base);
        if protocol::DiffAlgebra::<Puzzle3dWindowTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window transient already holds this state.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Puzzle3dWindowTransient) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| { vec![Self::Snapshot { transient: base.clone() }] 
    })())
}
}

macro_rules! json_store {
    ($state:ty, $extension:literal, $envelope:literal) => {
        impl store::ArtifactDsl for $state {
            const EXTENSION: &'static str = $extension;
            fn envelope_id() -> &'static str { $envelope }
            fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1))) }
            fn print_dsl(&self) -> String { semio_framework_pack_json::to_json_string(self) }
        }
        impl store::ArtifactPack for $state {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> { semio_framework_value::ToValue::to_value(self).encode_pack_with(options) }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> { let value = semio_framework_value::DslValue::decode_pack_with(bytes, options)?; semio_framework_value::FromValue::from_value(value).map_err(|error| store::PackError::from(error)) }
        }
    };
}

macro_rules! mutation_wire {
    ($mutation:ty) => {
        impl protocol::OpText for $mutation {
            fn print_op(&self) -> String { semio_framework_pack_json::to_json_string(self) }
            fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> { semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1))) }
        }
        impl protocol::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
            fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
                semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
            }
        }
    };
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for Puzzle3dWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Puzzle 3D window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for Puzzle3dWindowConfig {
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
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Self::__dsl_spec()) }
}

impl store::ConfigRecord for Puzzle3dWindowConfig {}

/// 🔺️ Sparse typed delta of one Puzzle 3D window instance's persisted-local options: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dWindowConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_automatic: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_depth_variable: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_manual: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_snap_enabled: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_kinds: Option<Puzzle3dSelectableKinds>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub proximity_radius: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub voxel_dims: Option<[u32; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_move: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_rotate: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vortex_show: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vortex_direction: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selection_method: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sun: Option<WorldSunConfig>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Puzzle3dCamera>,
}

impl Puzzle3dWindowConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle3dWindowConfig) -> Self {
        Self { lod_automatic: Some(state.lod_automatic), lod_depth_variable: Some(state.lod_depth_variable), grid_visible: Some(state.grid_visible), lod_manual: Some(state.lod_manual), grid_snap_enabled: Some(state.grid_snap_enabled), grid_spacing: Some(state.grid_spacing), selectable_kinds: Some(state.selectable_kinds.clone()), proximity_radius: Some(state.proximity_radius), chunk_size: Some(state.chunk_size), voxel_dims: Some(state.voxel_dims), transform_move: Some(state.transform_move), transform_rotate: Some(state.transform_rotate), vortex_show: Some(state.vortex_show.clone()), vortex_direction: Some(state.vortex_direction.clone()), selection_method: Some(state.selection_method.clone()), sun: Some(state.sun.clone()), camera: Some(state.camera.clone()) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle3dWindowConfig) -> Self {
        Self { lod_automatic: self.lod_automatic.as_ref().filter(|value| **value != base.lod_automatic).cloned(), lod_depth_variable: self.lod_depth_variable.as_ref().filter(|value| **value != base.lod_depth_variable).cloned(), grid_visible: self.grid_visible.as_ref().filter(|value| **value != base.grid_visible).cloned(), lod_manual: self.lod_manual.as_ref().filter(|value| **value != base.lod_manual).cloned(), grid_snap_enabled: self.grid_snap_enabled.as_ref().filter(|value| **value != base.grid_snap_enabled).cloned(), grid_spacing: self.grid_spacing.as_ref().filter(|value| **value != base.grid_spacing).cloned(), selectable_kinds: self.selectable_kinds.as_ref().filter(|value| **value != base.selectable_kinds).cloned(), proximity_radius: self.proximity_radius.as_ref().filter(|value| **value != base.proximity_radius).cloned(), chunk_size: self.chunk_size.as_ref().filter(|value| **value != base.chunk_size).cloned(), voxel_dims: self.voxel_dims.as_ref().filter(|value| **value != base.voxel_dims).cloned(), transform_move: self.transform_move.as_ref().filter(|value| **value != base.transform_move).cloned(), transform_rotate: self.transform_rotate.as_ref().filter(|value| **value != base.transform_rotate).cloned(), vortex_show: self.vortex_show.as_ref().filter(|value| **value != base.vortex_show).cloned(), vortex_direction: self.vortex_direction.as_ref().filter(|value| **value != base.vortex_direction).cloned(), selection_method: self.selection_method.as_ref().filter(|value| **value != base.selection_method).cloned(), sun: self.sun.as_ref().filter(|value| **value != base.sun).cloned(), camera: self.camera.as_ref().filter(|value| **value != base.camera).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle3dWindowConfig) -> Self {
        Self { lod_automatic: self.lod_automatic.as_ref().map(|_| base.lod_automatic), lod_depth_variable: self.lod_depth_variable.as_ref().map(|_| base.lod_depth_variable), grid_visible: self.grid_visible.as_ref().map(|_| base.grid_visible), lod_manual: self.lod_manual.as_ref().map(|_| base.lod_manual), grid_snap_enabled: self.grid_snap_enabled.as_ref().map(|_| base.grid_snap_enabled), grid_spacing: self.grid_spacing.as_ref().map(|_| base.grid_spacing), selectable_kinds: self.selectable_kinds.as_ref().map(|_| base.selectable_kinds.clone()), proximity_radius: self.proximity_radius.as_ref().map(|_| base.proximity_radius), chunk_size: self.chunk_size.as_ref().map(|_| base.chunk_size), voxel_dims: self.voxel_dims.as_ref().map(|_| base.voxel_dims), transform_move: self.transform_move.as_ref().map(|_| base.transform_move), transform_rotate: self.transform_rotate.as_ref().map(|_| base.transform_rotate), vortex_show: self.vortex_show.as_ref().map(|_| base.vortex_show.clone()), vortex_direction: self.vortex_direction.as_ref().map(|_| base.vortex_direction.clone()), selection_method: self.selection_method.as_ref().map(|_| base.selection_method.clone()), sun: self.sun.as_ref().map(|_| base.sun.clone()), camera: self.camera.as_ref().map(|_| base.camera.clone()) }
    }
}

impl protocol::MutationDiff<Puzzle3dWindowConfig> for Puzzle3dWindowConfigDiff {
    fn apply(&self, base: &Puzzle3dWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle3dWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.lod_automatic {
            next.lod_automatic = *value;
        }
        if let Some(value) = &self.lod_depth_variable {
            next.lod_depth_variable = *value;
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.lod_manual {
            next.lod_manual = *value;
        }
        if let Some(value) = &self.grid_snap_enabled {
            next.grid_snap_enabled = *value;
        }
        if let Some(value) = &self.grid_spacing {
            next.grid_spacing = *value;
        }
        if let Some(value) = &self.selectable_kinds {
            next.selectable_kinds = value.clone();
        }
        if let Some(value) = &self.proximity_radius {
            next.proximity_radius = *value;
        }
        if let Some(value) = &self.chunk_size {
            next.chunk_size = *value;
        }
        if let Some(value) = &self.voxel_dims {
            next.voxel_dims = *value;
        }
        if let Some(value) = &self.transform_move {
            next.transform_move = *value;
        }
        if let Some(value) = &self.transform_rotate {
            next.transform_rotate = *value;
        }
        if let Some(value) = &self.vortex_show {
            next.vortex_show = value.clone();
        }
        if let Some(value) = &self.vortex_direction {
            next.vortex_direction = value.clone();
        }
        if let Some(value) = &self.selection_method {
            next.selection_method = value.clone();
        }
        if let Some(value) = &self.sun {
            next.sun = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.lod_automatic.is_some() {
            self.lod_automatic = other.lod_automatic;
        }
        if other.lod_depth_variable.is_some() {
            self.lod_depth_variable = other.lod_depth_variable;
        }
        if other.grid_visible.is_some() {
            self.grid_visible = other.grid_visible;
        }
        if other.lod_manual.is_some() {
            self.lod_manual = other.lod_manual;
        }
        if other.grid_snap_enabled.is_some() {
            self.grid_snap_enabled = other.grid_snap_enabled;
        }
        if other.grid_spacing.is_some() {
            self.grid_spacing = other.grid_spacing;
        }
        if other.selectable_kinds.is_some() {
            self.selectable_kinds = other.selectable_kinds;
        }
        if other.proximity_radius.is_some() {
            self.proximity_radius = other.proximity_radius;
        }
        if other.chunk_size.is_some() {
            self.chunk_size = other.chunk_size;
        }
        if other.voxel_dims.is_some() {
            self.voxel_dims = other.voxel_dims;
        }
        if other.transform_move.is_some() {
            self.transform_move = other.transform_move;
        }
        if other.transform_rotate.is_some() {
            self.transform_rotate = other.transform_rotate;
        }
        if other.vortex_show.is_some() {
            self.vortex_show = other.vortex_show;
        }
        if other.vortex_direction.is_some() {
            self.vortex_direction = other.vortex_direction;
        }
        if other.selection_method.is_some() {
            self.selection_method = other.selection_method;
        }
        if other.sun.is_some() {
            self.sun = other.sun;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle3dWindowConfig> for Puzzle3dWindowConfigDiff {
    fn inverse(&self, base: &Puzzle3dWindowConfig) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.lod_automatic.is_none() && self.lod_depth_variable.is_none() && self.grid_visible.is_none() && self.lod_manual.is_none() && self.grid_snap_enabled.is_none() && self.grid_spacing.is_none() && self.selectable_kinds.is_none() && self.proximity_radius.is_none() && self.chunk_size.is_none() && self.voxel_dims.is_none() && self.transform_move.is_none() && self.transform_rotate.is_none() && self.vortex_show.is_none() && self.vortex_direction.is_none() && self.selection_method.is_none() && self.sun.is_none() && self.camera.is_none()
    }
}

json_store!(Puzzle3dWindowTransient, "puzzle3dwindowtransient", "s.puzzle.puzzle3d.windowtransient");
mutation_wire!(Puzzle3dWindowConfigMutation);
mutation_wire!(Puzzle3dWindowTransientMutation);
/// 🕳️ Tri-state decode of every `Option<Option<T>>` diff slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🔺️ Sparse typed delta of one Puzzle 3D window instance's ephemeral interaction state: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dWindowTransientDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub suggestion_menu: Option<Option<Puzzle3dSuggestionMenu>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brush_candidate_index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
}

impl Puzzle3dWindowTransientDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle3dWindowTransient) -> Self {
        Self { suggestion_menu: Some(state.suggestion_menu.clone()), engagement_input: Some(state.engagement_input.clone()), brush_candidate_index: Some(state.brush_candidate_index), activation: Some(state.activation.clone()) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle3dWindowTransient) -> Self {
        Self { suggestion_menu: self.suggestion_menu.as_ref().filter(|value| **value != base.suggestion_menu).cloned(), engagement_input: self.engagement_input.as_ref().filter(|value| **value != base.engagement_input).cloned(), brush_candidate_index: self.brush_candidate_index.as_ref().filter(|value| **value != base.brush_candidate_index).cloned(), activation: self.activation.as_ref().filter(|value| **value != base.activation).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle3dWindowTransient) -> Self {
        Self { suggestion_menu: self.suggestion_menu.as_ref().map(|_| base.suggestion_menu.clone()), engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), brush_candidate_index: self.brush_candidate_index.as_ref().map(|_| base.brush_candidate_index), activation: self.activation.as_ref().map(|_| base.activation.clone()) }
    }
}

impl protocol::MutationDiff<Puzzle3dWindowTransient> for Puzzle3dWindowTransientDiff {
    fn apply(&self, base: &Puzzle3dWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle3dWindowTransient> {
        let mut next = base.clone();
        if let Some(value) = &self.suggestion_menu {
            next.suggestion_menu = value.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.brush_candidate_index {
            next.brush_candidate_index = *value;
        }
        if let Some(value) = &self.activation {
            next.activation = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.suggestion_menu.is_some() {
            self.suggestion_menu = other.suggestion_menu;
        }
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.brush_candidate_index.is_some() {
            self.brush_candidate_index = other.brush_candidate_index;
        }
        if other.activation.is_some() {
            self.activation = other.activation;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle3dWindowTransient> for Puzzle3dWindowTransientDiff {
    fn inverse(&self, base: &Puzzle3dWindowTransient) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.suggestion_menu.is_none() && self.engagement_input.is_none() && self.brush_candidate_index.is_none() && self.activation.is_none()
    }
}

semio_framework_value::artifact_retire_struct!(Puzzle3dSuggestionMenu { x, y, window_id, vortex_full_id, submenu });
semio_framework_value::artifact_retire_struct!(Puzzle3dWindowTransient { suggestion_menu, engagement_input, brush_candidate_index, activation });

impl semio_framework_value::retirement::RetireOwned for Puzzle3dWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

/// 📏️ The exact heap bytes ONE window transient retains: the suggestion popup's two owned ids when
/// it is open, plus the engagement input line. Every other field is a fixed-width scalar the
/// enclosing record already accounts for.
fn puzzle3d_window_transient_retained_bytes(transient: &Puzzle3dWindowTransient) -> Option<usize> {
    let menu = transient.suggestion_menu.as_ref().map_or(Some(0), |menu| menu.window_id.capacity().checked_add(menu.vortex_full_id.capacity()))?;
    menu.checked_add(transient.engagement_input.capacity())?.checked_add(transient.activation.capacity())
}

fn puzzle3d_window_transient_preflight(mutation: &Puzzle3dWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Puzzle3dWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = puzzle3d_window_transient_retained_bytes(transient).ok_or_else(|| "Puzzle 3D window transient footprint overflowed".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint::for_ephemeral_item(retained_bytes))
}

fn puzzle3d_window_transient_transfer(mutation: Puzzle3dWindowTransientMutation) -> Puzzle3dWindowTransient {
    match mutation {
        Puzzle3dWindowTransientMutation::Snapshot { transient } => transient,
    }
}

pub struct Puzzle3dWindowConfigOwner;
impl semio_framework_plugin::WindowConfigOwner for Puzzle3dWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = main::WINDOW_KIND_ID;
    const SCHEMA: &'static str = "puzzle.3dwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = Puzzle3dWindowConfig;
    type Mutation = Puzzle3dWindowConfigMutation;
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub struct Puzzle3dWindowTransientOwner;
impl semio_framework_plugin::WindowTransientOwner for Puzzle3dWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = main::WINDOW_KIND_ID;
    type State = Puzzle3dWindowTransient;
    type Mutation = Puzzle3dWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
            puzzle3d_window_transient_preflight,
            puzzle3d_window_transient_transfer,
            state.clone(),
            mutation.clone(),
        ));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle3dWindowConfigOwner>()
}

pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle3dWindowTransientOwner>()
}

/// 🫧️ Retires interaction scratch the host has already moved past: a transient captured under a
/// different activation than the one this call carries is not this activation's scratch, so the
/// suggestion popup, the engagement input and the brush candidate index all read as empty. Fixed
/// cost — one string comparison per call, no allocation on the matching path.
pub fn live_transient(transient: &Puzzle3dWindowTransient, activation: &str) -> Puzzle3dWindowTransient {
    if transient.activation == activation {
        return transient.clone();
    }
    Puzzle3dWindowTransient { activation: activation.to_string(), ..Puzzle3dWindowTransient::default() }
}

pub fn runtime(shared: &Puzzle3dConfig, window: &Puzzle3dWindowConfig, transient: &Puzzle3dWindowTransient, view: Option<&semio_framework_plugin::ViewModel>) -> Puzzle3dRuntime {
    let transient = &live_transient(transient, &host_activation(view));
    Puzzle3dRuntime {
        fill_count: shared.fill_count,
        contact_tolerance: shared.contact_tolerance,
        object_kind_weights: shared.object_kind_weights.clone(),
        vortex_kind_weights: shared.vortex_kind_weights.clone(),
        lod_automatic: window.lod_automatic,
        lod_depth_variable: window.lod_depth_variable,
        grid_visible: window.grid_visible,
        lod_manual: window.lod_manual,
        grid_snap_enabled: window.grid_snap_enabled,
        grid_spacing: window.grid_spacing,
        selectable_kinds: window.selectable_kinds.clone(),
        proximity_radius: window.proximity_radius,
        chunk_size: window.chunk_size,
        voxel_dims: window.voxel_dims,
        transform_move: window.transform_move,
        transform_rotate: window.transform_rotate,
        vortex_show: window.vortex_show.clone(),
        vortex_direction: window.vortex_direction.clone(),
        selection_method: window.selection_method.clone(),
        sun: window.sun.clone(),
        camera: window.camera.clone(),
        active_example_id: shared.active_example_id.clone(),
        suggestion_menu: transient.suggestion_menu.clone(),
        engagement_input: transient.engagement_input.clone(),
        brush_candidate_index: transient.brush_candidate_index,
        active_tool_id: view.and_then(|value| value.active_tool_id.clone()),
        window_ids: view.map_or_else(|| vec![main::WINDOW_KIND_ID.into()], |value| value.window_instances.iter().map(|window| window.id.clone()).collect()),
    }
}

pub fn shared(runtime: &Puzzle3dRuntime) -> Puzzle3dConfig {
    Puzzle3dConfig { fill_count: runtime.fill_count, contact_tolerance: runtime.contact_tolerance, object_kind_weights: runtime.object_kind_weights.clone(), vortex_kind_weights: runtime.vortex_kind_weights.clone(), active_example_id: runtime.active_example_id.clone() }
}

/// 🫧️ The scratch this turn wants to retain, stamped with the activation it belongs to — read back by
/// [`live_transient`] on every later call.
pub fn transient(runtime: &Puzzle3dRuntime, view: Option<&semio_framework_plugin::ViewModel>) -> Puzzle3dWindowTransient {
    Puzzle3dWindowTransient { suggestion_menu: runtime.suggestion_menu.clone(), engagement_input: runtime.engagement_input.clone(), brush_candidate_index: runtime.brush_candidate_index, activation: host_activation(view) }
}

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, Puzzle3dConfig>) -> Puzzle3dWindowConfig { view.window::<Puzzle3dWindowConfigOwner>().cloned().unwrap_or_default() }
pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Puzzle3dWindowConfig { snapshot.and_then(|value| value.get::<Puzzle3dWindowConfigOwner>()).cloned().unwrap_or_default() }
pub fn transient_from_view(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> Puzzle3dWindowTransient { view.window::<Puzzle3dWindowTransientOwner>().cloned().unwrap_or_default() }
pub fn transient_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> Puzzle3dWindowTransient { snapshot.and_then(|value| value.get::<Puzzle3dWindowTransientOwner>()).cloned().unwrap_or_default() }

pub fn addressed_config_for(window_id: &str, config: Puzzle3dWindowConfig) -> semio_framework_plugin::WindowConfigMutation {
    semio_framework_plugin::WindowConfigMutation::of::<Puzzle3dWindowConfigOwner>(window_id, Puzzle3dWindowConfigMutation::Set { patch: Puzzle3dWindowConfigDiff::of(&config) })
}

pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Puzzle3dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle3d-window-required"))?;
    Ok(addressed_config_for(id, config))
}

pub fn addressed_transient_for(window_id: &str, transient: Puzzle3dWindowTransient) -> semio_framework_plugin::WindowTransientMutation {
    semio_framework_plugin::WindowTransientMutation::of::<Puzzle3dWindowTransientOwner>(window_id, Puzzle3dWindowTransientMutation::Snapshot { transient })
}

pub fn addressed_transient(view: &semio_framework_plugin::ViewModel, transient: Puzzle3dWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle3d-window-required"))?;
    Ok(addressed_transient_for(id, transient))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🪢️TaxonomyMounts
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
//#endregion 🪢️TaxonomyMounts
