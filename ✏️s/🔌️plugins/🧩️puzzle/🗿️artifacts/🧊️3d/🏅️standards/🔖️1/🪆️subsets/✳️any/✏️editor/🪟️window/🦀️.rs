//! 🪟️ Exact-instance Puzzle 3D window configuration and transient interaction owners.

use crate::editor::puzzle3d::config::{Puzzle3dCamera, Puzzle3dConfig, Puzzle3dRuntime, Puzzle3dSelectableKinds, Puzzle3dSuggestionMenu};
use crate::editor::puzzle3d::modes::edit::windows::main;
use semio_framework_plugin::WorldSunConfig;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dWindowConfig {
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub grid_visible: bool,
    pub lod_manual: f64,
    pub grid_snap_enabled: bool,
    pub grid_spacing: f64,
    pub selectable_kinds: Puzzle3dSelectableKinds,
    pub proximity_radius: f64,
    pub chunk_size: f64,
    pub voxel_dims: [u32; 3],
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub vortex_show: String,
    pub vortex_direction: String,
    pub sun: WorldSunConfig,
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
            sun: runtime.sun.clone(),
            camera: runtime.camera.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle3dWindowConfigMutation { Snapshot { config: Puzzle3dWindowConfig } }

impl protocol::Mutation<Puzzle3dWindowConfig> for Puzzle3dWindowConfigMutation {
    type Diff = Puzzle3dWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-config", display_name: "Set Puzzle 3D Window Configuration", emoji: "🪟️", aggregate_variant: "Snapshot", payload_schema: "puzzle.3dwindowconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &Puzzle3dWindowConfig) -> protocol::MutationOutcome<Self::Diff> { match self { Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()) } }
    fn inverse(&self, base: &Puzzle3dWindowConfig) -> Vec<Self> { vec![Self::Snapshot { config: base.clone() }] }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dWindowTransient {
    pub suggestion_menu: Option<Puzzle3dSuggestionMenu>,
    pub engagement_input: String,
    pub brush_candidate_index: usize,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle3dWindowTransientMutation { Snapshot { transient: Puzzle3dWindowTransient } }

impl protocol::Mutation<Puzzle3dWindowTransient> for Puzzle3dWindowTransientMutation {
    type Diff = Puzzle3dWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window", semantic_kind: "set-window-transient", display_name: "Set Puzzle 3D Window Transient", emoji: "🫧️", aggregate_variant: "Snapshot", payload_schema: "puzzle.3dwindowtransient", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &Puzzle3dWindowTransient) -> protocol::MutationOutcome<Self::Diff> { match self { Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()) } }
    fn inverse(&self, base: &Puzzle3dWindowTransient) -> Vec<Self> { vec![Self::Snapshot { transient: base.clone() }] }
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

json_store!(Puzzle3dWindowConfig, "puzzle3dwindowcfg", "s.puzzle.puzzle3d.windowconfig");
store::impl_whole_record_config!(Puzzle3dWindowConfig);
json_store!(Puzzle3dWindowTransient, "puzzle3dwindowtransient", "s.puzzle.puzzle3d.windowtransient");
mutation_wire!(Puzzle3dWindowConfigMutation);
mutation_wire!(Puzzle3dWindowTransientMutation);
impl protocol::MutationDiff<Puzzle3dWindowTransient> for Puzzle3dWindowTransient {
    fn apply(&self, _base: &Puzzle3dWindowTransient) -> protocol::MutationApplyResult<Puzzle3dWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

pub struct Puzzle3dWindowConfigOwner;
impl semio_framework_plugin::WindowConfigOwner for Puzzle3dWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = main::WINDOW_KIND_ID;
    const SCHEMA: &'static str = "puzzle.3dwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = Puzzle3dWindowConfig;
    type Mutation = Puzzle3dWindowConfigMutation;
    fn build_store_owners() -> store::MemberStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub struct Puzzle3dWindowTransientOwner;
impl semio_framework_plugin::WindowTransientOwner for Puzzle3dWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = main::WINDOW_KIND_ID;
    type State = Puzzle3dWindowTransient;
    type Mutation = Puzzle3dWindowTransientMutation;
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_transient_preparation_factory::<Self>() }
    fn build_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::State>> { semio_framework_plugin::bounded_window_transient_root_retirement_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_transient_store_disposer::<Self>() }
}

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle3dWindowConfigOwner>()
}

pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Puzzle3dWindowTransientOwner>()
}

pub fn runtime(shared: &Puzzle3dConfig, window: &Puzzle3dWindowConfig, transient: &Puzzle3dWindowTransient, view: Option<&semio_framework_plugin::ViewModel>) -> Puzzle3dRuntime {
    let mut runtime = Puzzle3dRuntime::default();
    runtime.fill_count = shared.fill_count;
    runtime.overlap_budget = shared.overlap_budget;
    runtime.object_kind_weights = shared.object_kind_weights.clone();
    runtime.vortex_kind_weights = shared.vortex_kind_weights.clone();
    runtime.lod_automatic = window.lod_automatic;
    runtime.lod_depth_variable = window.lod_depth_variable;
    runtime.grid_visible = window.grid_visible;
    runtime.lod_manual = window.lod_manual;
    runtime.grid_snap_enabled = window.grid_snap_enabled;
    runtime.grid_spacing = window.grid_spacing;
    runtime.selectable_kinds = window.selectable_kinds.clone();
    runtime.proximity_radius = window.proximity_radius;
    runtime.chunk_size = window.chunk_size;
    runtime.voxel_dims = window.voxel_dims;
    runtime.transform_move = window.transform_move;
    runtime.transform_rotate = window.transform_rotate;
    runtime.vortex_show = window.vortex_show.clone();
    runtime.vortex_direction = window.vortex_direction.clone();
    runtime.sun = window.sun.clone();
    runtime.camera = window.camera.clone();
    runtime.suggestion_menu = transient.suggestion_menu.clone();
    runtime.engagement_input = transient.engagement_input.clone();
    runtime.brush_candidate_index = transient.brush_candidate_index;
    runtime.active_tool_id = view.and_then(|value| value.active_tool_id.clone());
    runtime.window_ids = view.map(|value| value.window_instances.iter().map(|window| window.id.clone()).collect()).unwrap_or_else(|| vec![main::WINDOW_KIND_ID.into()]);
    runtime
}

pub fn shared(runtime: &Puzzle3dRuntime) -> Puzzle3dConfig {
    Puzzle3dConfig { fill_count: runtime.fill_count, overlap_budget: runtime.overlap_budget, object_kind_weights: runtime.object_kind_weights.clone(), vortex_kind_weights: runtime.vortex_kind_weights.clone() }
}

pub fn transient(runtime: &Puzzle3dRuntime) -> Puzzle3dWindowTransient {
    Puzzle3dWindowTransient { suggestion_menu: runtime.suggestion_menu.clone(), engagement_input: runtime.engagement_input.clone(), brush_candidate_index: runtime.brush_candidate_index }
}

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, Puzzle3dConfig>) -> Puzzle3dWindowConfig { view.window::<Puzzle3dWindowConfigOwner>().cloned().unwrap_or_default() }
pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Puzzle3dWindowConfig { snapshot.and_then(|value| value.get::<Puzzle3dWindowConfigOwner>()).cloned().unwrap_or_default() }
pub fn transient_from_view(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> Puzzle3dWindowTransient { view.window::<Puzzle3dWindowTransientOwner>().cloned().unwrap_or_default() }
pub fn transient_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> Puzzle3dWindowTransient { snapshot.and_then(|value| value.get::<Puzzle3dWindowTransientOwner>()).cloned().unwrap_or_default() }

pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Puzzle3dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle3d-window-required"))?;
    Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle3dWindowConfigOwner>(id, Puzzle3dWindowConfigMutation::Snapshot { config }))
}

pub fn addressed_transient(view: &semio_framework_plugin::ViewModel, transient: Puzzle3dWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle3d-window-required"))?;
    Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle3dWindowTransientOwner>(id, Puzzle3dWindowTransientMutation::Snapshot { transient }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_kind_windows_compose_independently() {
        let shared = Puzzle3dConfig::default();
        let first = Puzzle3dWindowConfig { grid_spacing: 2.0, camera: Puzzle3dCamera { zoom: 4.0, ..Default::default() }, ..Default::default() };
        let second = Puzzle3dWindowConfig { grid_spacing: 40.0, camera: Puzzle3dCamera { zoom: 0.5, ..Default::default() }, ..Default::default() };
        let first_runtime = runtime(&shared, &first, &Puzzle3dWindowTransient::default(), None);
        let second_runtime = runtime(&shared, &second, &Puzzle3dWindowTransient::default(), None);
        assert_eq!((first_runtime.grid_spacing, first_runtime.camera.zoom), (2.0, 4.0));
        assert_eq!((second_runtime.grid_spacing, second_runtime.camera.zoom), (40.0, 0.5));
    }

    #[test]
    fn app_pack_and_spr_exclude_window_transient_and_operation_fields() {
        let shared = Puzzle3dConfig::default();
        let spr = dsl::json::to_json_string(&shared);
        let oracle: serde_json::Value = serde_json::from_str(&spr).expect("serde_json oracle accepts the neutral config");
        assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(4));
        let pack = store::ArtifactPack::encode_pack(&shared);
        for forbidden in ["camera", "windowOptions", "engagementInput", "suggestionMenu", "fillCheckpoint", "fillApplyGeneration"] {
            assert!(!spr.contains(forbidden));
            assert!(!pack.windows(forbidden.len()).any(|bytes| bytes == forbidden.as_bytes()));
        }
    }
}
