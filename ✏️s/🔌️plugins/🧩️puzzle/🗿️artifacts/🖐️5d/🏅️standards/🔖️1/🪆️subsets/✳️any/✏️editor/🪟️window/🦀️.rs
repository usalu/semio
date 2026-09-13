//! 🪟️ Exact-instance Puzzle 5D window configuration and transient interaction owners.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d};
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use semio_framework_plugin::WorldSunConfig;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dWindowConfig {
    pub camera2d: Puzzle5dCamera2d,
    pub camera3d: Puzzle5dCamera3d,
    pub fill_count: u32,
    pub lod_mode: String,
    pub suggestion_offset: f64,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub sun: WorldSunConfig,
}

impl Default for Puzzle5dWindowConfig {
    fn default() -> Self {
        Self {
            camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
            camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
            fill_count: 0,
            lod_mode: crate::editor::puzzle5d::PUZZLE5D_LOD_MODE_AUTOMATIC.into(),
            suggestion_offset: crate::editor::puzzle5d::PUZZLE5D_DEFAULT_SUGGESTION_OFFSET,
            grid_snap_enabled: true,
            grid_factor: 1.0,
            sun: WorldSunConfig::default(),
        }
    }
}

/// 🎚️ ONE exact Puzzle 5D board pane's persisted-local options — `WindowConfigOwner::State`
/// requires `dsl::DslField`, emitted by `#[derive(dsl::DslArtifact)]` together with the `__dsl_*`
/// helpers the record-backed `ArtifactDsl`/`ArtifactPack` below call; `id`/`extension` are stated
/// explicitly so the derived constants reproduce the envelope identity this kind already carried.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.puzzle.puzzle5d.boardwindowconfig", extension = "puzzle5dboardwindowcfg", layout = "lines")]
pub struct Puzzle5dBoardWindowConfig {
    #[dsl(block)]
    pub camera2d: Puzzle5dCamera2d,
    pub fill_count: u32,
    pub lod_mode: String,
    pub suggestion_offset: f64,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
}

impl Default for Puzzle5dBoardWindowConfig {
    fn default() -> Self {
        let value = Puzzle5dWindowConfig::default();
        Self { camera2d: value.camera2d, fill_count: value.fill_count, lod_mode: value.lod_mode, suggestion_offset: value.suggestion_offset, grid_snap_enabled: value.grid_snap_enabled, grid_factor: value.grid_factor }
    }
}

/// 🎚️ ONE exact Puzzle 5D world pane's persisted-local options — same `dsl::DslArtifact` completion
/// as [`Puzzle5dBoardWindowConfig`], with its own envelope identity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.puzzle.puzzle5d.worldwindowconfig", extension = "puzzle5dworldwindowcfg", layout = "lines")]
pub struct Puzzle5dWorldWindowConfig {
    #[dsl(block)]
    pub camera3d: Puzzle5dCamera3d,
    #[dsl(block)]
    pub sun: WorldSunConfig,
}

impl Default for Puzzle5dWorldWindowConfig {
    fn default() -> Self {
        let value = Puzzle5dWindowConfig::default();
        Self { camera3d: value.camera3d, sun: value.sun }
    }
}

macro_rules! config_mutation {
    ($mutation:ident, $state:ty, $display:literal, $schema:literal) => {
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        pub enum $mutation { Snapshot { config: $state } }
        impl protocol::Mutation<$state> for $mutation {
            type Diff = $state;
            const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-config", display_name: $display, emoji: "🪟️", aggregate_variant: "Snapshot", payload_schema: $schema, text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] }];
            fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
            fn diff(&self, _base: &$state) -> protocol::MutationOutcome<Self::Diff> { match self { Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()) } }
            fn inverse(&self, base: &$state) -> Vec<Self> { vec![Self::Snapshot { config: base.clone() }] }
        }
    };
}

config_mutation!(Puzzle5dBoardWindowConfigMutation, Puzzle5dBoardWindowConfig, "Set Puzzle 5D Board Window Configuration", "puzzle.5dboardwindowconfig");
config_mutation!(Puzzle5dWorldWindowConfigMutation, Puzzle5dWorldWindowConfig, "Set Puzzle 5D World Window Configuration", "puzzle.5dworldwindowconfig");

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dWindowTransient {
    pub engagement_input: String,
    pub brush_candidate_index: usize,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle5dWindowTransientMutation {
    Snapshot { transient: Puzzle5dWindowTransient },
}

impl protocol::Mutation<Puzzle5dWindowTransient> for Puzzle5dWindowTransientMutation {
    type Diff = Puzzle5dWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-transient", display_name: "Set Puzzle 5D Window Transient", emoji: "🫧️", aggregate_variant: "Snapshot", payload_schema: "puzzle.5dwindowtransient", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &Puzzle5dWindowTransient) -> protocol::MutationOutcome<Self::Diff> { match self { Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()) } }
    fn inverse(&self, base: &Puzzle5dWindowTransient) -> Vec<Self> { vec![Self::Snapshot { transient: base.clone() }] }
}

macro_rules! json_store {
    ($state:ty, $extension:literal, $envelope:literal) => {
        impl store::ArtifactDsl for $state {
            const EXTENSION: &'static str = $extension;
            fn envelope_id() -> &'static str { $envelope }
            fn parse_dsl(text: &str) -> Result<Self, store::TextError> { dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1))) }
            fn print_dsl(&self) -> String { dsl::json::to_json_string(self) }
        }
        impl store::ArtifactPack for $state {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> { dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options) }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> { let value = dsl::DslValue::decode_pack_with(bytes, options)?; dsl::from_dsl_value(value).map_err(store::PackError::Schema) }
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
            fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
                let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
                let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
                Self::__dsl_from_record(&record)
            }
            fn print_dsl(&self) -> String {
                let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect($envelope_label);
                store::semio_format::wrap_text(&envelope, &body)
            }
        }
        impl store::ArtifactPack for $state {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
                Ok(store::semio_format::wrap_binary(&envelope, &body))
            }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
                if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
                    return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
                }
                let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
            }
            fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
        }
    };
}

record_store!(Puzzle5dBoardWindowConfig, "valid Puzzle 5D board-window envelope");
store::impl_whole_record_config!(Puzzle5dBoardWindowConfig);
record_store!(Puzzle5dWorldWindowConfig, "valid Puzzle 5D world-window envelope");
store::impl_whole_record_config!(Puzzle5dWorldWindowConfig);
json_store!(Puzzle5dWindowTransient, "puzzle5dwindowtransient", "s.puzzle.puzzle5d.windowtransient");

macro_rules! mutation_wire {
    ($mutation:ty) => {
        impl protocol::OpText for $mutation {
            fn print_op(&self) -> String { dsl::json::to_json_string(self) }
            fn parse_op(line: &str) -> Result<Self, store::TextError> { dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1))) }
        }
        impl protocol::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
            fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
                dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
            }
        }
    };
}

mutation_wire!(Puzzle5dBoardWindowConfigMutation);
mutation_wire!(Puzzle5dWorldWindowConfigMutation);
mutation_wire!(Puzzle5dWindowTransientMutation);

impl protocol::MutationDiff<Puzzle5dWindowTransient> for Puzzle5dWindowTransient {
    fn apply(&self, _base: &Puzzle5dWindowTransient) -> protocol::MutationApplyResult<Puzzle5dWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

store::artifact_retire_struct!(Puzzle5dWindowTransient { engagement_input, brush_candidate_index });

impl store::retirement::RetireOwned for Puzzle5dWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn puzzle5d_window_transient_preflight(mutation: &Puzzle5dWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Puzzle5dWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = std::mem::size_of::<Puzzle5dWindowTransient>().checked_add(transient.engagement_input.capacity()).ok_or_else(|| "Puzzle 5D window transient footprint overflowed".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
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
            fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
        }
        pub struct $transient;
        impl semio_framework_plugin::WindowTransientOwner for $transient {
            const WINDOW_KIND_ID: &'static str = $kind;
            type State = Puzzle5dWindowTransient;
            type Mutation = Puzzle5dWindowTransientMutation;
            fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
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

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, crate::editor::puzzle5d::config::Puzzle5dConfig>) -> Puzzle5dWindowConfig {
    if let Some(value) = view.window::<Puzzle5dBoardWindowConfigOwner>() {
        return Puzzle5dWindowConfig { camera2d: value.camera2d.clone(), fill_count: value.fill_count, lod_mode: value.lod_mode.clone(), suggestion_offset: value.suggestion_offset, grid_snap_enabled: value.grid_snap_enabled, grid_factor: value.grid_factor, ..Default::default() };
    }
    if let Some(value) = view.window::<Puzzle5dWorldWindowConfigOwner>() {
        return Puzzle5dWindowConfig { camera3d: value.camera3d.clone(), sun: value.sun.clone(), ..Default::default() };
    }
    Puzzle5dWindowConfig::default()
}

pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Puzzle5dWindowConfig {
    if let Some(value) = snapshot.and_then(|snapshot| snapshot.get::<Puzzle5dBoardWindowConfigOwner>()) {
        return Puzzle5dWindowConfig { camera2d: value.camera2d.clone(), fill_count: value.fill_count, lod_mode: value.lod_mode.clone(), suggestion_offset: value.suggestion_offset, grid_snap_enabled: value.grid_snap_enabled, grid_factor: value.grid_factor, ..Default::default() };
    }
    if let Some(value) = snapshot.and_then(|snapshot| snapshot.get::<Puzzle5dWorldWindowConfigOwner>()) {
        return Puzzle5dWindowConfig { camera3d: value.camera3d.clone(), sun: value.sun.clone(), ..Default::default() };
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
) -> crate::editor::puzzle5d::config::Puzzle5dRuntime {
    let mut runtime = crate::editor::puzzle5d::config::Puzzle5dRuntime::default();
    runtime.overlap_budget = shared.overlap_budget;
    runtime.object_kind_weights = shared.object_kind_weights.clone();
    runtime.vortex_kind_weights = shared.vortex_kind_weights.clone();
    runtime.camera2d = window.camera2d.clone();
    runtime.camera3d = window.camera3d.clone();
    runtime.fill_count = window.fill_count;
    runtime.lod_mode = window.lod_mode.clone();
    runtime.suggestion_offset = window.suggestion_offset;
    runtime.grid_snap_enabled = window.grid_snap_enabled;
    runtime.grid_factor = window.grid_factor;
    runtime.sun = window.sun.clone();
    runtime.engagement_input_by_window.clear();
    runtime.engagement_input_by_window.insert(window_id.to_string(), transient.engagement_input.clone());
    runtime.brush_candidate_index = transient.brush_candidate_index;
    runtime
}

pub fn shared(runtime: &crate::editor::puzzle5d::config::Puzzle5dRuntime) -> crate::editor::puzzle5d::config::Puzzle5dConfig {
    let mut config = crate::editor::puzzle5d::config::Puzzle5dConfig::default();
    config.overlap_budget = runtime.overlap_budget;
    config.object_kind_weights = runtime.object_kind_weights.clone();
    config.vortex_kind_weights = runtime.vortex_kind_weights.clone();
    config
}

pub fn config_from_runtime(runtime: &crate::editor::puzzle5d::config::Puzzle5dRuntime) -> Puzzle5dWindowConfig {
    Puzzle5dWindowConfig {
        camera2d: runtime.camera2d.clone(),
        camera3d: runtime.camera3d.clone(),
        fill_count: runtime.fill_count,
        lod_mode: runtime.lod_mode.clone(),
        suggestion_offset: runtime.suggestion_offset,
        grid_snap_enabled: runtime.grid_snap_enabled,
        grid_factor: runtime.grid_factor,
        sun: runtime.sun.clone(),
    }
}

pub fn transient_from_runtime(runtime: &crate::editor::puzzle5d::config::Puzzle5dRuntime, window_id: &str) -> Puzzle5dWindowTransient {
    Puzzle5dWindowTransient { engagement_input: runtime.engagement_input_by_window.get(window_id).cloned().unwrap_or_default(), brush_candidate_index: runtime.brush_candidate_index }
}

pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Puzzle5dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle5d-window-required"))?;
    match kind_for_view(view) {
        Some(board2d::WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle5dBoardWindowConfigOwner>(id, Puzzle5dBoardWindowConfigMutation::Snapshot { config: Puzzle5dBoardWindowConfig { camera2d: config.camera2d, fill_count: config.fill_count, lod_mode: config.lod_mode, suggestion_offset: config.suggestion_offset, grid_snap_enabled: config.grid_snap_enabled, grid_factor: config.grid_factor } })),
        Some(world3d::WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle5dWorldWindowConfigOwner>(id, Puzzle5dWorldWindowConfigMutation::Snapshot { config: Puzzle5dWorldWindowConfig { camera3d: config.camera3d, sun: config.sun } })),
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
