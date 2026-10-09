//! 🪟️ Exact-instance Puzzle 5D window configuration and transient interaction owners.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d, Puzzle5dRuntime, Puzzle5dSelectableKinds};
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use semio_framework_plugin::WorldSunConfig;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dWindowConfig {
    pub camera2d: Puzzle5dCamera2d,
    pub camera3d: Puzzle5dCamera3d,
    pub lod_mode: String,
    pub suggestion_offset: f64,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub grid_visible: bool,
    pub grid_spacing: f64,
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub lod_manual: f64,
    pub selectable_kinds: Puzzle5dSelectableKinds,
    pub grip_show: String,
    pub grip_direction: String,
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub sun: WorldSunConfig,
    pub voxel_dims: [u32; 3],
}

impl Default for Puzzle5dWindowConfig {
    /// 🎚️ Every field mirrors [`Puzzle5dRuntime::default`] so a freshly-opened pane and a
    /// Rust-constructed runtime never disagree about a boot value.
    fn default() -> Self {
        let runtime = Puzzle5dRuntime::default();
        Self {
            camera2d: runtime.camera2d,
            camera3d: runtime.camera3d,
            lod_mode: runtime.lod_mode,
            suggestion_offset: runtime.suggestion_offset,
            grid_snap_enabled: runtime.grid_snap_enabled,
            grid_factor: runtime.grid_factor,
            grid_visible: runtime.grid_visible,
            grid_spacing: runtime.grid_spacing,
            lod_automatic: runtime.lod_automatic,
            lod_depth_variable: runtime.lod_depth_variable,
            lod_manual: runtime.lod_manual,
            selectable_kinds: runtime.selectable_kinds,
            grip_show: runtime.grip_show,
            grip_direction: runtime.grip_direction,
            transform_move: runtime.transform_move,
            transform_rotate: runtime.transform_rotate,
            sun: runtime.sun,
            voxel_dims: runtime.voxel_dims,
        }
    }
}

/// 🎚️ ONE exact Puzzle 5D board pane's persisted-local options — `WindowConfigOwner::State`
/// requires `dsl::DslField`, emitted by `#[derive(dsl::DslArtifact)]` together with the `__dsl_*`
/// helpers the record-backed `ArtifactDsl`/`ArtifactPack` below call; `id`/`extension` are stated
/// explicitly so the derived constants reproduce the envelope identity this kind already carried.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.puzzle.puzzle5d.boardwindowconfig", extension = "puzzle5dboardwindowcfg")]
pub struct Puzzle5dBoardWindowConfig {
    #[dsl(block)]
    pub camera2d: Puzzle5dCamera2d,
    pub lod_mode: String,
    pub suggestion_offset: f64,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub grid_visible: bool,
    #[dsl(block)]
    pub selectable_kinds: Puzzle5dSelectableKinds,
}

impl Default for Puzzle5dBoardWindowConfig {
    fn default() -> Self {
        let value = Puzzle5dWindowConfig::default();
        Self {
            camera2d: value.camera2d,
            lod_mode: value.lod_mode,
            suggestion_offset: value.suggestion_offset,
            grid_snap_enabled: value.grid_snap_enabled,
            grid_factor: value.grid_factor,
            grid_visible: value.grid_visible,
            selectable_kinds: value.selectable_kinds,
        }
    }
}

/// 🎚️ ONE exact Puzzle 5D world pane's persisted-local options — same `dsl::DslArtifact` completion
/// as [`Puzzle5dBoardWindowConfig`], with its own envelope identity.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.puzzle.puzzle5d.worldwindowconfig", extension = "puzzle5dworldwindowcfg")]
pub struct Puzzle5dWorldWindowConfig {
    #[dsl(block)]
    pub camera3d: Puzzle5dCamera3d,
    #[dsl(block)]
    pub sun: WorldSunConfig,
    pub grid_visible: bool,
    pub grid_snap_enabled: bool,
    pub grid_spacing: f64,
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub lod_manual: f64,
    #[dsl(block)]
    pub selectable_kinds: Puzzle5dSelectableKinds,
    pub grip_show: String,
    pub grip_direction: String,
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub voxel_dims: [u32; 3],
}

impl Default for Puzzle5dWorldWindowConfig {
    fn default() -> Self {
        let value = Puzzle5dWindowConfig::default();
        Self {
            camera3d: value.camera3d,
            sun: value.sun,
            grid_visible: value.grid_visible,
            grid_snap_enabled: value.grid_snap_enabled,
            grid_spacing: value.grid_spacing,
            lod_automatic: value.lod_automatic,
            lod_depth_variable: value.lod_depth_variable,
            lod_manual: value.lod_manual,
            selectable_kinds: value.selectable_kinds,
            grip_show: value.grip_show,
            grip_direction: value.grip_direction,
            transform_move: value.transform_move,
            transform_rotate: value.transform_rotate,
            voxel_dims: value.voxel_dims,
        }
    }
}

macro_rules! config_mutation {
    ($mutation:ident, $state:ty, $diff:ident, $display:literal, $schema:literal) => {
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        pub enum $mutation {
            Set { patch: $diff },
        }
        impl protocol::Mutation<$state> for $mutation {
            type Diff = $diff;
            const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
                schema_version: 1,
                owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
                semantic_kind: "set-window-config",
                display_name: $display,
                emoji: "🪟️",
                aggregate_variant: "Set",
                payload_schema: $schema,
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
            fn diff(&self, base: &$state) -> protocol::MutationOutcome<$diff> {
                let Self::Set { patch } = self;
                let diff = patch.changed(base);
                if protocol::DiffAlgebra::<$state>::is_empty(&diff) {
                    return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds these values.");
                }
                protocol::MutationOutcome::new(diff)
            }
            fn inverse(&self, base: &$state) -> Result<Vec<Self>, semio_framework_value::ValueError> {
                let Self::Set { patch } = self;
                Ok(vec![Self::Set { patch: patch.restoring(base) }])
            }
        }
    };
}

config_mutation!(Puzzle5dBoardWindowConfigMutation, Puzzle5dBoardWindowConfig, Puzzle5dBoardWindowConfigDiff, "Set Puzzle 5D Board Window Configuration", "puzzle.5dboardwindowconfig");
config_mutation!(Puzzle5dWorldWindowConfigMutation, Puzzle5dWorldWindowConfig, Puzzle5dWorldWindowConfigDiff, "Set Puzzle 5D World Window Configuration", "puzzle.5dworldwindowconfig");

/// 🎣️ The grip suggestion menu one window has open — puzzle 3d's record verbatim, because the world host's
/// suggestion protocol (`suggestionMenu` on the interaction lane, `openVortexSuggestions`) is shared.
pub use semio_s_artifact_puzzle_3d::editor::puzzle3d::config::Puzzle3dSuggestionMenu as Puzzle5dSuggestionMenu;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dWindowTransient {
    #[value(default)]
    pub suggestion_menu: Option<Puzzle5dSuggestionMenu>,
    pub engagement_input: String,
    pub brush_candidate_index: usize,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle5dWindowTransientMutation {
    Snapshot { transient: Puzzle5dWindowTransient },
}

impl protocol::Mutation<Puzzle5dWindowTransient> for Puzzle5dWindowTransientMutation {
    type Diff = Puzzle5dWindowTransientDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-transient", display_name: "Set Puzzle 5D Window Transient", emoji: "🫧️", aggregate_variant: "Snapshot", payload_schema: "puzzle.5dwindowtransient", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &Puzzle5dWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        let Self::Snapshot { transient } = self;
        let diff = Puzzle5dWindowTransientDiff::of(transient).changed(base);
        if protocol::DiffAlgebra::<Puzzle5dWindowTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window transient already holds this state.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Puzzle5dWindowTransient) -> Result<Vec<Self>, semio_framework_value::ValueError> {
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

/// 📜️ Record-backed text form for one Puzzle 5D window config — the derived `__dsl_spec` grammar
/// inside that kind's semio text envelope.
///
/// 🎒️ The pack half's `record_spec` is what the retained window-config loader reads to decode a
/// mounted pack field-by-field; `None` there would fail every retained load of the kind with
/// `WindowConfigPackLoadDiagnostic::TypedState`.
macro_rules! record_store {
    ($state:ty, $envelope_label:literal) => {
        impl store::ArtifactDsl for $state {
            const EXTENSION: &'static str = Self::__DSL_EXTENSION;
            fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
            fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
                let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
                let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
                Self::__dsl_from_record(&record)
            }
            fn print_dsl(&self) -> String {
                let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect($envelope_label);
                store::semio_format::wrap_text(&envelope, &body)
            }
        }
        impl store::ArtifactPack for $state {
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
    };
}

record_store!(Puzzle5dBoardWindowConfig, "valid Puzzle 5D board-window envelope");
impl store::ConfigRecord for Puzzle5dBoardWindowConfig {}

/// 🔺️ Sparse typed delta of one Puzzle 5D board window's persisted-local options: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dBoardWindowConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera2d: Option<Puzzle5dCamera2d>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_mode: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_snap_enabled: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_factor: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_kinds: Option<Puzzle5dSelectableKinds>,
}

impl Puzzle5dBoardWindowConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dBoardWindowConfig) -> Self {
        Self { camera2d: Some(state.camera2d.clone()), lod_mode: Some(state.lod_mode.clone()), suggestion_offset: Some(state.suggestion_offset), grid_snap_enabled: Some(state.grid_snap_enabled), grid_factor: Some(state.grid_factor), grid_visible: Some(state.grid_visible), selectable_kinds: Some(state.selectable_kinds.clone()) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dBoardWindowConfig) -> Self {
        Self { camera2d: self.camera2d.as_ref().filter(|value| **value != base.camera2d).cloned(), lod_mode: self.lod_mode.as_ref().filter(|value| **value != base.lod_mode).cloned(), suggestion_offset: self.suggestion_offset.as_ref().filter(|value| **value != base.suggestion_offset).cloned(), grid_snap_enabled: self.grid_snap_enabled.as_ref().filter(|value| **value != base.grid_snap_enabled).cloned(), grid_factor: self.grid_factor.as_ref().filter(|value| **value != base.grid_factor).cloned(), grid_visible: self.grid_visible.as_ref().filter(|value| **value != base.grid_visible).cloned(), selectable_kinds: self.selectable_kinds.as_ref().filter(|value| **value != base.selectable_kinds).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dBoardWindowConfig) -> Self {
        Self { camera2d: self.camera2d.as_ref().map(|_| base.camera2d.clone()), lod_mode: self.lod_mode.as_ref().map(|_| base.lod_mode.clone()), suggestion_offset: self.suggestion_offset.as_ref().map(|_| base.suggestion_offset), grid_snap_enabled: self.grid_snap_enabled.as_ref().map(|_| base.grid_snap_enabled), grid_factor: self.grid_factor.as_ref().map(|_| base.grid_factor), grid_visible: self.grid_visible.as_ref().map(|_| base.grid_visible), selectable_kinds: self.selectable_kinds.as_ref().map(|_| base.selectable_kinds.clone()) }
    }
}

impl protocol::MutationDiff<Puzzle5dBoardWindowConfig> for Puzzle5dBoardWindowConfigDiff {
    fn apply(&self, base: &Puzzle5dBoardWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dBoardWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera2d {
            next.camera2d = value.clone();
        }
        if let Some(value) = &self.lod_mode {
            next.lod_mode = value.clone();
        }
        if let Some(value) = &self.suggestion_offset {
            next.suggestion_offset = *value;
        }
        if let Some(value) = &self.grid_snap_enabled {
            next.grid_snap_enabled = *value;
        }
        if let Some(value) = &self.grid_factor {
            next.grid_factor = *value;
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.selectable_kinds {
            next.selectable_kinds = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera2d.is_some() {
            self.camera2d = other.camera2d;
        }
        if other.lod_mode.is_some() {
            self.lod_mode = other.lod_mode;
        }
        if other.suggestion_offset.is_some() {
            self.suggestion_offset = other.suggestion_offset;
        }
        if other.grid_snap_enabled.is_some() {
            self.grid_snap_enabled = other.grid_snap_enabled;
        }
        if other.grid_factor.is_some() {
            self.grid_factor = other.grid_factor;
        }
        if other.grid_visible.is_some() {
            self.grid_visible = other.grid_visible;
        }
        if other.selectable_kinds.is_some() {
            self.selectable_kinds = other.selectable_kinds;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle5dBoardWindowConfig> for Puzzle5dBoardWindowConfigDiff {
    fn inverse(&self, base: &Puzzle5dBoardWindowConfig) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.camera2d.is_none() && self.lod_mode.is_none() && self.suggestion_offset.is_none() && self.grid_snap_enabled.is_none() && self.grid_factor.is_none() && self.grid_visible.is_none() && self.selectable_kinds.is_none()
    }
}

record_store!(Puzzle5dWorldWindowConfig, "valid Puzzle 5D world-window envelope");
impl store::ConfigRecord for Puzzle5dWorldWindowConfig {}

/// 🔺️ Sparse typed delta of one Puzzle 5D world window's persisted-local options: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dWorldWindowConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera3d: Option<Puzzle5dCamera3d>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sun: Option<WorldSunConfig>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_snap_enabled: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_automatic: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_depth_variable: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_manual: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_kinds: Option<Puzzle5dSelectableKinds>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grip_show: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grip_direction: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_move: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_rotate: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub voxel_dims: Option<[u32; 3]>,
}

impl Puzzle5dWorldWindowConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dWorldWindowConfig) -> Self {
        Self { camera3d: Some(state.camera3d.clone()), sun: Some(state.sun.clone()), grid_visible: Some(state.grid_visible), grid_snap_enabled: Some(state.grid_snap_enabled), grid_spacing: Some(state.grid_spacing), lod_automatic: Some(state.lod_automatic), lod_depth_variable: Some(state.lod_depth_variable), lod_manual: Some(state.lod_manual), selectable_kinds: Some(state.selectable_kinds.clone()), grip_show: Some(state.grip_show.clone()), grip_direction: Some(state.grip_direction.clone()), transform_move: Some(state.transform_move), transform_rotate: Some(state.transform_rotate), voxel_dims: Some(state.voxel_dims) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dWorldWindowConfig) -> Self {
        Self { camera3d: self.camera3d.as_ref().filter(|value| **value != base.camera3d).cloned(), sun: self.sun.as_ref().filter(|value| **value != base.sun).cloned(), grid_visible: self.grid_visible.as_ref().filter(|value| **value != base.grid_visible).cloned(), grid_snap_enabled: self.grid_snap_enabled.as_ref().filter(|value| **value != base.grid_snap_enabled).cloned(), grid_spacing: self.grid_spacing.as_ref().filter(|value| **value != base.grid_spacing).cloned(), lod_automatic: self.lod_automatic.as_ref().filter(|value| **value != base.lod_automatic).cloned(), lod_depth_variable: self.lod_depth_variable.as_ref().filter(|value| **value != base.lod_depth_variable).cloned(), lod_manual: self.lod_manual.as_ref().filter(|value| **value != base.lod_manual).cloned(), selectable_kinds: self.selectable_kinds.as_ref().filter(|value| **value != base.selectable_kinds).cloned(), grip_show: self.grip_show.as_ref().filter(|value| **value != base.grip_show).cloned(), grip_direction: self.grip_direction.as_ref().filter(|value| **value != base.grip_direction).cloned(), transform_move: self.transform_move.as_ref().filter(|value| **value != base.transform_move).cloned(), transform_rotate: self.transform_rotate.as_ref().filter(|value| **value != base.transform_rotate).cloned(), voxel_dims: self.voxel_dims.as_ref().filter(|value| **value != base.voxel_dims).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dWorldWindowConfig) -> Self {
        Self { camera3d: self.camera3d.as_ref().map(|_| base.camera3d.clone()), sun: self.sun.as_ref().map(|_| base.sun.clone()), grid_visible: self.grid_visible.as_ref().map(|_| base.grid_visible), grid_snap_enabled: self.grid_snap_enabled.as_ref().map(|_| base.grid_snap_enabled), grid_spacing: self.grid_spacing.as_ref().map(|_| base.grid_spacing), lod_automatic: self.lod_automatic.as_ref().map(|_| base.lod_automatic), lod_depth_variable: self.lod_depth_variable.as_ref().map(|_| base.lod_depth_variable), lod_manual: self.lod_manual.as_ref().map(|_| base.lod_manual), selectable_kinds: self.selectable_kinds.as_ref().map(|_| base.selectable_kinds.clone()), grip_show: self.grip_show.as_ref().map(|_| base.grip_show.clone()), grip_direction: self.grip_direction.as_ref().map(|_| base.grip_direction.clone()), transform_move: self.transform_move.as_ref().map(|_| base.transform_move), transform_rotate: self.transform_rotate.as_ref().map(|_| base.transform_rotate), voxel_dims: self.voxel_dims.as_ref().map(|_| base.voxel_dims) }
    }
}

impl protocol::MutationDiff<Puzzle5dWorldWindowConfig> for Puzzle5dWorldWindowConfigDiff {
    fn apply(&self, base: &Puzzle5dWorldWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dWorldWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera3d {
            next.camera3d = value.clone();
        }
        if let Some(value) = &self.sun {
            next.sun = value.clone();
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.grid_snap_enabled {
            next.grid_snap_enabled = *value;
        }
        if let Some(value) = &self.grid_spacing {
            next.grid_spacing = *value;
        }
        if let Some(value) = &self.lod_automatic {
            next.lod_automatic = *value;
        }
        if let Some(value) = &self.lod_depth_variable {
            next.lod_depth_variable = *value;
        }
        if let Some(value) = &self.lod_manual {
            next.lod_manual = *value;
        }
        if let Some(value) = &self.selectable_kinds {
            next.selectable_kinds = value.clone();
        }
        if let Some(value) = &self.grip_show {
            next.grip_show = value.clone();
        }
        if let Some(value) = &self.grip_direction {
            next.grip_direction = value.clone();
        }
        if let Some(value) = &self.transform_move {
            next.transform_move = *value;
        }
        if let Some(value) = &self.transform_rotate {
            next.transform_rotate = *value;
        }
        if let Some(value) = &self.voxel_dims {
            next.voxel_dims = *value;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera3d.is_some() {
            self.camera3d = other.camera3d;
        }
        if other.sun.is_some() {
            self.sun = other.sun;
        }
        if other.grid_visible.is_some() {
            self.grid_visible = other.grid_visible;
        }
        if other.grid_snap_enabled.is_some() {
            self.grid_snap_enabled = other.grid_snap_enabled;
        }
        if other.grid_spacing.is_some() {
            self.grid_spacing = other.grid_spacing;
        }
        if other.lod_automatic.is_some() {
            self.lod_automatic = other.lod_automatic;
        }
        if other.lod_depth_variable.is_some() {
            self.lod_depth_variable = other.lod_depth_variable;
        }
        if other.lod_manual.is_some() {
            self.lod_manual = other.lod_manual;
        }
        if other.selectable_kinds.is_some() {
            self.selectable_kinds = other.selectable_kinds;
        }
        if other.grip_show.is_some() {
            self.grip_show = other.grip_show;
        }
        if other.grip_direction.is_some() {
            self.grip_direction = other.grip_direction;
        }
        if other.transform_move.is_some() {
            self.transform_move = other.transform_move;
        }
        if other.transform_rotate.is_some() {
            self.transform_rotate = other.transform_rotate;
        }
        if other.voxel_dims.is_some() {
            self.voxel_dims = other.voxel_dims;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle5dWorldWindowConfig> for Puzzle5dWorldWindowConfigDiff {
    fn inverse(&self, base: &Puzzle5dWorldWindowConfig) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.camera3d.is_none() && self.sun.is_none() && self.grid_visible.is_none() && self.grid_snap_enabled.is_none() && self.grid_spacing.is_none() && self.lod_automatic.is_none() && self.lod_depth_variable.is_none() && self.lod_manual.is_none() && self.selectable_kinds.is_none() && self.grip_show.is_none() && self.grip_direction.is_none() && self.transform_move.is_none() && self.transform_rotate.is_none() && self.voxel_dims.is_none()
    }
}

json_store!(Puzzle5dWindowTransient, "puzzle5dwindowtransient", "s.puzzle.puzzle5d.windowtransient");

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

mutation_wire!(Puzzle5dBoardWindowConfigMutation);
mutation_wire!(Puzzle5dWorldWindowConfigMutation);
mutation_wire!(Puzzle5dWindowTransientMutation);

/// 🕳️ Tri-state decode of every `Option<Option<T>>` diff slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🔺️ Sparse typed delta of one Puzzle 5D window instance's ephemeral interaction state: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dWindowTransientDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub suggestion_menu: Option<Option<Puzzle5dSuggestionMenu>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brush_candidate_index: Option<usize>,
}

impl Puzzle5dWindowTransientDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dWindowTransient) -> Self {
        Self { suggestion_menu: Some(state.suggestion_menu.clone()), engagement_input: Some(state.engagement_input.clone()), brush_candidate_index: Some(state.brush_candidate_index) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dWindowTransient) -> Self {
        Self { suggestion_menu: self.suggestion_menu.as_ref().filter(|value| **value != base.suggestion_menu).cloned(), engagement_input: self.engagement_input.as_ref().filter(|value| **value != base.engagement_input).cloned(), brush_candidate_index: self.brush_candidate_index.as_ref().filter(|value| **value != base.brush_candidate_index).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dWindowTransient) -> Self {
        Self { suggestion_menu: self.suggestion_menu.as_ref().map(|_| base.suggestion_menu.clone()), engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), brush_candidate_index: self.brush_candidate_index.as_ref().map(|_| base.brush_candidate_index) }
    }
}

impl protocol::MutationDiff<Puzzle5dWindowTransient> for Puzzle5dWindowTransientDiff {
    fn apply(&self, base: &Puzzle5dWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dWindowTransient> {
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
    }
}

impl protocol::DiffAlgebra<Puzzle5dWindowTransient> for Puzzle5dWindowTransientDiff {
    fn inverse(&self, base: &Puzzle5dWindowTransient) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.suggestion_menu.is_none() && self.engagement_input.is_none() && self.brush_candidate_index.is_none()
    }
}

semio_framework_value::artifact_retire_struct!(Puzzle5dWindowTransient { suggestion_menu, engagement_input, brush_candidate_index });

impl semio_framework_value::retirement::RetireOwned for Puzzle5dWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn puzzle5d_window_transient_preflight(mutation: &Puzzle5dWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Puzzle5dWindowTransientMutation::Snapshot { transient } = mutation;
    let menu = transient.suggestion_menu.as_ref().map_or(Some(0), |menu| menu.window_id.capacity().checked_add(menu.vortex_full_id.capacity()));
    let retained_bytes = menu
        .and_then(|menu| std::mem::size_of::<Puzzle5dWindowTransient>().checked_add(transient.engagement_input.capacity())?.checked_add(menu))
        .ok_or_else(|| "Puzzle 5D window transient footprint overflowed".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint::for_ephemeral_item(retained_bytes))
}

fn puzzle5d_window_transient_transfer(mutation: Puzzle5dWindowTransientMutation) -> Puzzle5dWindowTransient {
    match mutation {
        Puzzle5dWindowTransientMutation::Snapshot { transient } => transient,
    }
}

macro_rules! owners {
    ($config:ident, $state:ty, $mutation:ty, $schema:literal, $transient:ident, $kind:expr) => {
        pub struct $config;
        impl semio_framework_plugin::WindowConfigOwner for $config {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = $schema;
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = $state;
            type Mutation = $mutation;
            fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
        }
        pub struct $transient;
        impl semio_framework_plugin::WindowTransientOwner for $transient {
            const WINDOW_KIND_ID: &'static str = $kind;
            type State = Puzzle5dWindowTransient;
            type Mutation = Puzzle5dWindowTransientMutation;
            fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
                    puzzle5d_window_transient_preflight,
                    puzzle5d_window_transient_transfer,
                    state.clone(),
                    mutation.clone(),
                ));
                semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
            }
        }
    };
}

owners!(Puzzle5dBoardWindowConfigOwner, Puzzle5dBoardWindowConfig, Puzzle5dBoardWindowConfigMutation, "puzzle.5dboardwindowconfig", Puzzle5dBoardWindowTransientOwner, board2d::WINDOW_KIND_ID);
owners!(Puzzle5dWorldWindowConfigOwner, Puzzle5dWorldWindowConfig, Puzzle5dWorldWindowConfigMutation, "puzzle.5dworldwindowconfig", Puzzle5dWorldWindowTransientOwner, world3d::WINDOW_KIND_ID);

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle5dBoardWindowConfigOwner>()?;
    registry.register::<Puzzle5dWorldWindowConfigOwner>()
}

pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle5dBoardWindowTransientOwner>()?;
    registry.register::<Puzzle5dWorldWindowTransientOwner>()
}

/// 🎚️ Lifts ONE board pane's persisted options into the merged shape the runtime reads.
fn window_config_from_board(value: &Puzzle5dBoardWindowConfig) -> Puzzle5dWindowConfig {
    Puzzle5dWindowConfig {
        camera2d: value.camera2d.clone(),
        lod_mode: value.lod_mode.clone(),
        suggestion_offset: value.suggestion_offset,
        grid_snap_enabled: value.grid_snap_enabled,
        grid_factor: value.grid_factor,
        grid_visible: value.grid_visible,
        selectable_kinds: value.selectable_kinds.clone(),
        ..Default::default()
    }
}

/// 🎚️ Lifts ONE world pane's persisted options into the merged shape the runtime reads.
fn window_config_from_world(value: &Puzzle5dWorldWindowConfig) -> Puzzle5dWindowConfig {
    Puzzle5dWindowConfig {
        camera3d: value.camera3d.clone(),
        sun: value.sun.clone(),
        voxel_dims: value.voxel_dims,
        grid_visible: value.grid_visible,
        grid_snap_enabled: value.grid_snap_enabled,
        grid_spacing: value.grid_spacing,
        lod_automatic: value.lod_automatic,
        lod_depth_variable: value.lod_depth_variable,
        lod_manual: value.lod_manual,
        selectable_kinds: value.selectable_kinds.clone(),
        grip_show: value.grip_show.clone(),
        grip_direction: value.grip_direction.clone(),
        transform_move: value.transform_move,
        transform_rotate: value.transform_rotate,
        ..Default::default()
    }
}

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, crate::editor::puzzle5d::config::Puzzle5dConfig>) -> Puzzle5dWindowConfig {
    if let Some(value) = view.window::<Puzzle5dBoardWindowConfigOwner>() {
        return window_config_from_board(value);
    }
    if let Some(value) = view.window::<Puzzle5dWorldWindowConfigOwner>() {
        return window_config_from_world(value);
    }
    Puzzle5dWindowConfig::default()
}

pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Puzzle5dWindowConfig {
    if let Some(value) = snapshot.and_then(|snapshot| snapshot.get::<Puzzle5dBoardWindowConfigOwner>()) {
        return window_config_from_board(value);
    }
    if let Some(value) = snapshot.and_then(|snapshot| snapshot.get::<Puzzle5dWorldWindowConfigOwner>()) {
        return window_config_from_world(value);
    }
    Puzzle5dWindowConfig::default()
}

pub fn transient_from_view(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> Puzzle5dWindowTransient {
    view.window::<Puzzle5dBoardWindowTransientOwner>().or_else(|| view.window::<Puzzle5dWorldWindowTransientOwner>()).cloned().unwrap_or_default()
}

pub fn transient_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> Puzzle5dWindowTransient {
    snapshot.and_then(|value| value.get::<Puzzle5dBoardWindowTransientOwner>().or_else(|| value.get::<Puzzle5dWorldWindowTransientOwner>())).cloned().unwrap_or_default()
}

pub fn kind_for_view(view: &semio_framework_plugin::ViewModel) -> Option<&str> {
    let id = view.window_id.as_deref()?;
    view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str())
}

pub fn runtime(
    shared: &crate::editor::puzzle5d::config::Puzzle5dConfig,
    window: &Puzzle5dWindowConfig,
    transient: &Puzzle5dWindowTransient,
    window_id: &str,
) -> Puzzle5dRuntime {
    let mut runtime = Puzzle5dRuntime::default();
    runtime.contact_tolerance = shared.contact_tolerance;
    runtime.proximity_radius = shared.proximity_radius;
    runtime.chunk_size = shared.chunk_size;
    runtime.object_kind_weights = shared.object_kind_weights.clone();
    runtime.vortex_kind_weights = shared.vortex_kind_weights.clone();
    runtime.camera2d = window.camera2d.clone();
    runtime.camera3d = window.camera3d.clone();
    runtime.fill_count = shared.fill_count;
    runtime.lod_mode = window.lod_mode.clone();
    runtime.suggestion_offset = window.suggestion_offset;
    runtime.grid_snap_enabled = window.grid_snap_enabled;
    runtime.grid_factor = window.grid_factor;
    runtime.grid_visible = window.grid_visible;
    runtime.grid_spacing = window.grid_spacing;
    runtime.lod_automatic = window.lod_automatic;
    runtime.lod_depth_variable = window.lod_depth_variable;
    runtime.lod_manual = window.lod_manual;
    runtime.selectable_kinds = window.selectable_kinds.clone();
    runtime.grip_show = window.grip_show.clone();
    runtime.grip_direction = window.grip_direction.clone();
    runtime.transform_move = window.transform_move;
    runtime.transform_rotate = window.transform_rotate;
    runtime.sun = window.sun.clone();
    runtime.voxel_dims = window.voxel_dims;
    runtime.engagement_input_by_window.clear();
    runtime.engagement_input_by_window.insert(window_id.to_string(), transient.engagement_input.clone());
    runtime.brush_candidate_index = transient.brush_candidate_index;
    runtime.suggestion_menu = transient.suggestion_menu.clone();
    runtime
}

pub fn shared(runtime: &Puzzle5dRuntime) -> crate::editor::puzzle5d::config::Puzzle5dConfig {
    let mut config = crate::editor::puzzle5d::config::Puzzle5dConfig::default();
    config.fill_count = runtime.fill_count;
    config.contact_tolerance = runtime.contact_tolerance;
    config.proximity_radius = runtime.proximity_radius;
    config.chunk_size = runtime.chunk_size;
    config.object_kind_weights = runtime.object_kind_weights.clone();
    config.vortex_kind_weights = runtime.vortex_kind_weights.clone();
    config
}

pub fn config_from_runtime(runtime: &Puzzle5dRuntime) -> Puzzle5dWindowConfig {
    Puzzle5dWindowConfig {
        camera2d: runtime.camera2d.clone(),
        camera3d: runtime.camera3d.clone(),
        lod_mode: runtime.lod_mode.clone(),
        suggestion_offset: runtime.suggestion_offset,
        grid_snap_enabled: runtime.grid_snap_enabled,
        grid_factor: runtime.grid_factor,
        grid_visible: runtime.grid_visible,
        grid_spacing: runtime.grid_spacing,
        lod_automatic: runtime.lod_automatic,
        lod_depth_variable: runtime.lod_depth_variable,
        lod_manual: runtime.lod_manual,
        selectable_kinds: runtime.selectable_kinds.clone(),
        grip_show: runtime.grip_show.clone(),
        grip_direction: runtime.grip_direction.clone(),
        transform_move: runtime.transform_move,
        transform_rotate: runtime.transform_rotate,
        sun: runtime.sun.clone(),
        voxel_dims: runtime.voxel_dims,
    }
}

pub fn transient_from_runtime(runtime: &Puzzle5dRuntime, window_id: &str) -> Puzzle5dWindowTransient {
    Puzzle5dWindowTransient { suggestion_menu: runtime.suggestion_menu.clone(), engagement_input: runtime.engagement_input_by_window.get(window_id).cloned().unwrap_or_default(), brush_candidate_index: runtime.brush_candidate_index }
}

pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Puzzle5dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle5d-window-required"))?;
    match kind_for_view(view) {
        Some(board2d::WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle5dBoardWindowConfigOwner>(
            id,
            Puzzle5dBoardWindowConfigMutation::Snapshot {
                config: Puzzle5dBoardWindowConfig {
                    camera2d: config.camera2d,
                    lod_mode: config.lod_mode,
                    suggestion_offset: config.suggestion_offset,
                    grid_snap_enabled: config.grid_snap_enabled,
                    grid_factor: config.grid_factor,
                    grid_visible: config.grid_visible,
                    selectable_kinds: config.selectable_kinds,
                },
            },
        )),
        Some(world3d::WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle5dWorldWindowConfigOwner>(
            id,
            Puzzle5dWorldWindowConfigMutation::Snapshot {
                config: Puzzle5dWorldWindowConfig {
                    camera3d: config.camera3d,
                    sun: config.sun,
                    voxel_dims: config.voxel_dims,
                    grid_visible: config.grid_visible,
                    grid_snap_enabled: config.grid_snap_enabled,
                    grid_spacing: config.grid_spacing,
                    lod_automatic: config.lod_automatic,
                    lod_depth_variable: config.lod_depth_variable,
                    lod_manual: config.lod_manual,
                    selectable_kinds: config.selectable_kinds,
                    grip_show: config.grip_show,
                    grip_direction: config.grip_direction,
                    transform_move: config.transform_move,
                    transform_rotate: config.transform_rotate,
                },
            },
        )),
        _ => Err(semio_framework_plugin::Fault::from("puzzle5d-window-kind-required")),
    }
}

pub fn addressed_transient(view: &semio_framework_plugin::ViewModel, transient: Puzzle5dWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle5d-window-required"))?;
    let mutation = Puzzle5dWindowTransientMutation::Snapshot { transient };
    match kind_for_view(view) {
        Some(board2d::WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle5dBoardWindowTransientOwner>(id, mutation)),
        Some(world3d::WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle5dWorldWindowTransientOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("puzzle5d-window-kind-required")),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🪢️TaxonomyMounts
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
//#endregion 🪢️TaxonomyMounts
