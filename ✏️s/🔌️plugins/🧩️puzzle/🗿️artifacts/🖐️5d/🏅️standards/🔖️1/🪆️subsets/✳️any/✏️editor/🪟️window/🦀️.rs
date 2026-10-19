//! 🪟️ Exact-instance Puzzle 5D window configuration and transient interaction owners.

use crate::editor::puzzle5d::config::Puzzle5dCamera2d;

use crate::editor::puzzle5d::config::Puzzle5dCamera3d;

use crate::editor::puzzle5d::config::Puzzle5dRuntime;

use crate::editor::puzzle5d::config::Puzzle5dSelectableKinds;
use crate::editor::puzzle5d::entities::{PUZZLE5D_BOARD_WINDOW_KIND_ID, PUZZLE5D_WORLD_WINDOW_KIND_ID};
use crate::editor::puzzle5d::modes::edit::windows::world3d;
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
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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
    ($mutation:ident, $set:ident, $state:ty, $diff:ident, $display:literal, $schema:literal) => {
        /// 🧩️ Payload of the mutation's `Set`: the absolute value of exactly the diff's named fields.
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
        #[canonical_json(owner = semio_framework_pack_json)]
        pub struct $set {
            pub patch: $diff,
        }

        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
        #[canonical_json(owner = semio_framework_pack_json)]
        pub enum $mutation {
            Set($set),
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

        }

impl protocol::ColdMutation<$state> for $mutation {
    type ReferenceDiff = $diff;
            fn diff(&self, base: &$state) -> protocol::MutationOutcome<$diff> {
                let Self::Set($set { patch }) = self;
                let diff = patch.changed(base);
                if protocol::DiffAlgebra::<$state>::is_empty(&diff) {
                    return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds these values.");
                }
                protocol::MutationOutcome::new(diff)
            }
            fn inverse(&self, base: &$state) -> Result<Vec<Self>, semio_framework_value::ValueError> {
                let Self::Set($set { patch }) = self;
                Ok(vec![Self::Set($set { patch: patch.restoring(base) })])
            }
}
    };
}

config_mutation!(Puzzle5dBoardWindowConfigMutation, Puzzle5dBoardWindowConfigMutationSet, Puzzle5dBoardWindowConfig, Puzzle5dBoardWindowConfigDiff, "Set Puzzle 5D Board Window Configuration", "puzzle.5dboardwindowconfig");
config_mutation!(Puzzle5dWorldWindowConfigMutation, Puzzle5dWorldWindowConfigMutationSet, Puzzle5dWorldWindowConfig, Puzzle5dWorldWindowConfigDiff, "Set Puzzle 5D World Window Configuration", "puzzle.5dworldwindowconfig");

impl semio_framework_plugin::WindowConfigApplyMutation<Puzzle5dBoardWindowConfig> for Puzzle5dBoardWindowConfigMutation {
    fn exchange(self, post: &mut Puzzle5dBoardWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let Self::Set(Puzzle5dBoardWindowConfigMutationSet { mut patch }) = self;
        if let Some(value) = patch.camera2d.as_mut() {
            std::mem::swap(value, &mut post.camera2d);
        }
        if let Some(value) = patch.lod_mode.as_mut() {
            std::mem::swap(value, &mut post.lod_mode);
        }
        if let Some(value) = patch.suggestion_offset.as_mut() {
            std::mem::swap(value, &mut post.suggestion_offset);
        }
        if let Some(value) = patch.grid_snap_enabled.as_mut() {
            std::mem::swap(value, &mut post.grid_snap_enabled);
        }
        if let Some(value) = patch.grid_factor.as_mut() {
            std::mem::swap(value, &mut post.grid_factor);
        }
        if let Some(value) = patch.grid_visible.as_mut() {
            std::mem::swap(value, &mut post.grid_visible);
        }
        if let Some(value) = patch.selectable_kinds.as_mut() {
            std::mem::swap(value, &mut post.selectable_kinds);
        }
        Ok(Self::Set(Puzzle5dBoardWindowConfigMutationSet { patch }))
    }

    fn payload_bytes(&self) -> usize {
        let Self::Set(Puzzle5dBoardWindowConfigMutationSet { patch }) = self;
        0 + patch.lod_mode.as_ref().map_or(0, String::len)
    }
}

impl semio_framework_plugin::WindowConfigApplyMutation<Puzzle5dWorldWindowConfig> for Puzzle5dWorldWindowConfigMutation {
    fn exchange(self, post: &mut Puzzle5dWorldWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let Self::Set(Puzzle5dWorldWindowConfigMutationSet { mut patch }) = self;
        if let Some(value) = patch.camera3d.as_mut() {
            std::mem::swap(value, &mut post.camera3d);
        }
        if let Some(value) = patch.sun.as_mut() {
            std::mem::swap(value, &mut post.sun);
        }
        if let Some(value) = patch.grid_visible.as_mut() {
            std::mem::swap(value, &mut post.grid_visible);
        }
        if let Some(value) = patch.grid_snap_enabled.as_mut() {
            std::mem::swap(value, &mut post.grid_snap_enabled);
        }
        if let Some(value) = patch.grid_spacing.as_mut() {
            std::mem::swap(value, &mut post.grid_spacing);
        }
        if let Some(value) = patch.lod_automatic.as_mut() {
            std::mem::swap(value, &mut post.lod_automatic);
        }
        if let Some(value) = patch.lod_depth_variable.as_mut() {
            std::mem::swap(value, &mut post.lod_depth_variable);
        }
        if let Some(value) = patch.lod_manual.as_mut() {
            std::mem::swap(value, &mut post.lod_manual);
        }
        if let Some(value) = patch.selectable_kinds.as_mut() {
            std::mem::swap(value, &mut post.selectable_kinds);
        }
        if let Some(value) = patch.grip_show.as_mut() {
            std::mem::swap(value, &mut post.grip_show);
        }
        if let Some(value) = patch.grip_direction.as_mut() {
            std::mem::swap(value, &mut post.grip_direction);
        }
        if let Some(value) = patch.transform_move.as_mut() {
            std::mem::swap(value, &mut post.transform_move);
        }
        if let Some(value) = patch.transform_rotate.as_mut() {
            std::mem::swap(value, &mut post.transform_rotate);
        }
        if let Some(value) = patch.voxel_dims.as_mut() {
            std::mem::swap(value, &mut post.voxel_dims);
        }
        Ok(Self::Set(Puzzle5dWorldWindowConfigMutationSet { patch }))
    }

    fn payload_bytes(&self) -> usize {
        let Self::Set(Puzzle5dWorldWindowConfigMutationSet { patch }) = self;
        0 + patch.grip_show.as_ref().map_or(0, String::len) + patch.grip_direction.as_ref().map_or(0, String::len)
    }
}

/// 🎣️ The grip suggestion menu one window has open — puzzle 3d's record verbatim, because the world host's
/// suggestion protocol (`suggestionMenu` on the interaction lane, `openVortexSuggestions`) is shared.
pub use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dSuggestionMenu as Puzzle5dSuggestionMenu;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dWindowTransient {
    #[value(default)]
    pub suggestion_menu: Option<Puzzle5dSuggestionMenu>,
    pub engagement_input: String,
    pub brush_candidate_index: usize,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum Puzzle5dWindowTransientMutation {
    Snapshot { transient: Puzzle5dWindowTransient },
}

impl protocol::Mutation<Puzzle5dWindowTransient> for Puzzle5dWindowTransientMutation {
    type Diff = Puzzle5dWindowTransientDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-transient", display_name: "Set Puzzle 5D Window Transient", emoji: "🫧️", aggregate_variant: "Snapshot", payload_schema: "puzzle.5dwindowtransient", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }

}

impl protocol::ColdMutation<Puzzle5dWindowTransient> for Puzzle5dWindowTransientMutation {
    type ReferenceDiff = Puzzle5dWindowTransientDiff;
    fn diff(&self, base: &Puzzle5dWindowTransient) -> protocol::MutationOutcome<Self::ReferenceDiff> {
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
        impl pack::snapshot::ArtifactDsl for $state {
            const EXTENSION: &'static str = $extension;
            fn envelope_id() -> &'static str { $envelope }
            fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1))) }
            fn print_dsl(&self) -> String { semio_framework_pack_json::to_json_string(self) }
        }
        impl pack::snapshot::ArtifactPack for $state {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> { semio_framework_value::ToValue::to_value(self).encode_pack_with(options) }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> { let value = semio_framework_value::DslValue::decode_pack_with(bytes, options)?; semio_framework_value::FromValue::from_value(value).map_err(|error| store::PackError::from(error)) }
        }
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for $state {

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
        impl pack::snapshot::ArtifactDsl for $state {
            const EXTENSION: &'static str = Self::__DSL_EXTENSION;
            fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
            fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
                let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
                let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
                Self::__dsl_from_record(&record)
            }
            fn print_dsl(&self) -> String {
                let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect($envelope_label);
                store::semio_format::wrap_text(&envelope, &body)
            }
        }
        impl pack::snapshot::ArtifactPack for $state {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
                Ok(store::semio_format::wrap_binary(&envelope, &body))
            }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
                if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
                    return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
                }
                let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
            }
            fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Self::__dsl_spec()) }
        }
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for $state {

}
    };
}

record_store!(Puzzle5dBoardWindowConfig, "valid Puzzle 5D board-window envelope");
impl store::ConfigRecord for Puzzle5dBoardWindowConfig {}

#[path = "🍓️config-diffs/🦀️.rs"]
mod config_diffs;
pub use config_diffs::{Puzzle5dBoardWindowConfigDiff, Puzzle5dWorldWindowConfigDiff};

#[path = "🥦️transient-diff/🦀️.rs"]
mod transient_diff;
pub use transient_diff::{puzzle5d_window_transient_preflight, Puzzle5dWindowTransientDiff};
use transient_diff::puzzle5d_window_transient_transfer;

#[path = "🌽️resolution/🦀️.rs"]
mod resolution;
pub use resolution::{
    addressed_config, addressed_transient, config_from_runtime, config_from_snapshot,
    config_from_view, kind_for_view, runtime, shared, transient_from_runtime,
    transient_from_snapshot, transient_from_view,
};

record_store!(Puzzle5dWorldWindowConfig, "valid Puzzle 5D world-window envelope");
impl store::ConfigRecord for Puzzle5dWorldWindowConfig {}

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

macro_rules! owners {
    ($config:ident, $state:ty, $mutation:ty, $schema:literal, $transient:ident, $kind:expr) => {
        pub struct $config;
        impl semio_framework_plugin::WindowConfigOwner for $config {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = $schema;
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = $state;
            type Mutation = $mutation;
            type Edit = semio_framework_plugin::WindowConfigApplyEdit<$state, $mutation>;
            const MAXIMUM_PREPARATION_DEPTH: usize = 64;
            fn build_retained_edit() -> std::sync::Arc<Self::Edit> { std::sync::Arc::new(semio_framework_plugin::WindowConfigApplyEdit::new()) }
            fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_plugin::bounded_window_config_preparation_source_birth::<Self>()}
            fn build_store_owners(grant:semio_framework_value::RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State,Self::Mutation>,semio_framework_value::RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State,Self::Mutation>>{semio_framework_plugin::bounded_window_config_store_owners::<Self>(grant)}
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
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

owners!(Puzzle5dBoardWindowConfigOwner, Puzzle5dBoardWindowConfig, Puzzle5dBoardWindowConfigMutation, "puzzle.5dboardwindowconfig", Puzzle5dBoardWindowTransientOwner, PUZZLE5D_BOARD_WINDOW_KIND_ID);
owners!(Puzzle5dWorldWindowConfigOwner, Puzzle5dWorldWindowConfig, Puzzle5dWorldWindowConfigMutation, "puzzle.5dworldwindowconfig", Puzzle5dWorldWindowTransientOwner, PUZZLE5D_WORLD_WINDOW_KIND_ID);

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle5dBoardWindowConfigOwner>()?;
    registry.register::<Puzzle5dWorldWindowConfigOwner>()
}

pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle5dBoardWindowTransientOwner>()?;
    registry.register::<Puzzle5dWorldWindowTransientOwner>()
}

//#region 🪢️TaxonomyMounts

//#endregion 🪢️TaxonomyMounts
