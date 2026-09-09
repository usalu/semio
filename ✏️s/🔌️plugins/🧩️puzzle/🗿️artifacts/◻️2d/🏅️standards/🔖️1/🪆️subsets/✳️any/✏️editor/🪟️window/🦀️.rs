//! 🪟️ Exact-instance persisted and ephemeral ownership for Puzzle 2D panes.

use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dWindowConfig {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
    pub lod_mode: String,
    pub fill_count: u32,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub suggestion_offset: f64,
}

impl Default for Puzzle2dWindowConfig {
    fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            lod_mode: crate::editor::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.into(),
            fill_count: 0,
            grid_snap_enabled: false,
            grid_factor: 1.0,
            suggestion_offset: crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_SUGGESTION_OFFSET,
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle2dWindowConfigMutation {
    Snapshot { config: Puzzle2dWindowConfig },
}

impl protocol::Mutation<Puzzle2dWindowConfig> for Puzzle2dWindowConfigMutation {
    type Diff = Puzzle2dWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-config",
        display_name: "Set Puzzle 2D Window Configuration",
        emoji: "🪟️",
        aggregate_variant: "Snapshot",
        payload_schema: "puzzle.2dwindowconfig",
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
    fn diff(&self, _base: &Puzzle2dWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()),
        }
    }
    fn inverse(&self, base: &Puzzle2dWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}

macro_rules! json_store {
    ($state:ty, $extension:literal, $envelope:literal) => {
        impl store::ArtifactDsl for $state {
            const EXTENSION: &'static str = $extension;
            fn envelope_id() -> &'static str {
                $envelope
            }
            fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
                dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
            }
            fn print_dsl(&self) -> String {
                dsl::json::to_json_string(self)
            }
        }
        impl store::ArtifactPack for $state {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
            }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                let value = dsl::DslValue::decode_pack_with(bytes, options)?;
                dsl::from_dsl_value(value).map_err(store::PackError::Schema)
            }
        }
    };
}

json_store!(Puzzle2dWindowConfig, "puzzle2dwindowcfg", "s.puzzle.puzzle2d.windowconfig");
store::impl_whole_record_config!(Puzzle2dWindowConfig);

impl protocol::OpText for Puzzle2dWindowConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
impl protocol::OpBinary for Puzzle2dWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dWindowTransient {
    pub engagement_input: String,
    pub brush_candidate_index: usize,
    pub brush_candidates: Vec<dsl::DslValue>,
    pub brush_candidate_source_handle_id: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle2dWindowTransientMutation {
    Snapshot { transient: Puzzle2dWindowTransient },
}

impl protocol::Mutation<Puzzle2dWindowTransient> for Puzzle2dWindowTransientMutation {
    type Diff = Puzzle2dWindowTransient;
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
    fn diff(&self, _base: &Puzzle2dWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()),
        }
    }
    fn inverse(&self, base: &Puzzle2dWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<Puzzle2dWindowTransient> for Puzzle2dWindowTransient {
    fn apply(&self, _base: &Puzzle2dWindowTransient) -> protocol::MutationApplyResult<Puzzle2dWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
json_store!(Puzzle2dWindowTransient, "puzzle2dwindowtransient", "s.puzzle.puzzle2d.windowtransient");
impl protocol::OpText for Puzzle2dWindowTransientMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
impl protocol::OpBinary for Puzzle2dWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
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
            fn build_store_owners() -> store::MemberStoreOwners<Self::State, Self::Mutation> {
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
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::State, Self::Mutation>> {
                semio_framework_plugin::bounded_window_transient_preparation_factory::<Self>()
            }
            fn build_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::State>> {
                semio_framework_plugin::bounded_window_transient_root_retirement_factory::<Self>()
            }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::State, Self::Mutation>>> {
                semio_framework_plugin::bounded_window_transient_store_disposer::<Self>()
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

pub fn document_seed(document: &serde_json::Value) -> Puzzle2dWindowConfig {
    let mut seed = Puzzle2dWindowConfig::default();
    if let Some(camera) = document.get("camera") {
        seed.camera_x = camera.get("x").and_then(serde_json::Value::as_f64).unwrap_or(seed.camera_x);
        seed.camera_y = camera.get("y").and_then(serde_json::Value::as_f64).unwrap_or(seed.camera_y);
        seed.camera_zoom = camera.get("zoom").and_then(serde_json::Value::as_f64).filter(|zoom| zoom.is_finite() && *zoom > 0.0).unwrap_or(seed.camera_zoom);
    }
    seed
}

pub fn config_from_view_or_document(view: &semio_framework_plugin::ConfigView<'_, crate::editor::puzzle2d::config::Puzzle2dConfig>, document: &serde_json::Value) -> Puzzle2dWindowConfig {
    let config = config_from_view(view);
    if view.window.is_some_and(|snapshot| snapshot.generation() == 0) && config == Puzzle2dWindowConfig::default() {
        document_seed(document)
    } else {
        config
    }
}

pub fn config_from_snapshot_or_document(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>, document: &serde_json::Value) -> Puzzle2dWindowConfig {
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
    let mutation = Puzzle2dWindowConfigMutation::Snapshot { config };
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
        fill_count: window.fill_count,
        grid_snap_enabled: window.grid_snap_enabled,
        grid_factor: window.grid_factor,
        suggestion_offset: window.suggestion_offset,
        node_kind_weights: config.node_kind_weights.clone(),
        handle_kind_weights: config.handle_kind_weights.clone(),
        ..Default::default()
    }
}

pub fn split(runtime: &crate::editor::puzzle2d::config::Puzzle2dPlayRuntime, window_kind: &str) -> (crate::editor::puzzle2d::config::Puzzle2dConfig, Puzzle2dWindowConfig, Puzzle2dWindowTransient) {
    (
        crate::editor::puzzle2d::config::Puzzle2dConfig { node_kind_weights: runtime.node_kind_weights.clone(), handle_kind_weights: runtime.handle_kind_weights.clone() },
        Puzzle2dWindowConfig {
            camera_x: runtime.camera_x,
            camera_y: runtime.camera_y,
            camera_zoom: runtime.camera_zoom,
            lod_mode: runtime.lod_mode_by_pane.get(window_kind).cloned().unwrap_or_else(|| crate::editor::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.into()),
            fill_count: runtime.fill_count,
            grid_snap_enabled: runtime.grid_snap_enabled,
            grid_factor: runtime.grid_factor,
            suggestion_offset: runtime.suggestion_offset,
        },
        Puzzle2dWindowTransient {
            engagement_input: runtime.engagement_input_by_pane.get(window_kind).cloned().unwrap_or_default(),
            brush_candidate_index: runtime.brush_candidate_index,
            brush_candidates: runtime.brush_candidates.clone(),
            brush_candidate_source_handle_id: runtime.brush_candidate_source_handle_id.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_kind_windows_compose_isolated_runtime() {
        let shared = crate::editor::puzzle2d::config::Puzzle2dConfig::default();
        let first = Puzzle2dWindowConfig { camera_x: 12.0, grid_factor: 4.0, ..Default::default() };
        let second = Puzzle2dWindowConfig { camera_x: -7.0, grid_factor: 0.5, ..Default::default() };
        let first_runtime = runtime(&shared, &first, &Puzzle2dWindowTransient::default(), Some(overview::WINDOW_KIND_ID));
        let second_runtime = runtime(&shared, &second, &Puzzle2dWindowTransient::default(), Some(overview::WINDOW_KIND_ID));
        assert_eq!(first_runtime.camera_x, 12.0);
        assert_eq!(second_runtime.camera_x, -7.0);
        assert_eq!(first_runtime.grid_factor, 4.0);
        assert_eq!(second_runtime.grid_factor, 0.5);
    }

    #[test]
    fn app_pack_and_spr_exclude_window_and_transient_fields() {
        let shared = crate::editor::puzzle2d::config::Puzzle2dConfig::default();
        let spr = dsl::json::to_json_string(&shared);
        let oracle: serde_json::Value = serde_json::from_str(&spr).expect("serde_json oracle accepts the neutral config");
        assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(2));
        let pack = store::ArtifactPack::encode_pack(&shared);
        for forbidden in ["cameraX", "engagementInput", "brushCandidates", "fillJobCheckpointSequence"] {
            assert!(!spr.contains(forbidden));
            assert!(!pack.windows(forbidden.len()).any(|bytes| bytes == forbidden.as_bytes()));
        }
    }

    #[test]
    fn document_camera_is_only_the_initial_window_seed() {
        let document = serde_json::json!({ "camera": { "x": 8.0, "y": -3.0, "zoom": 2.5 } });
        let seed = document_seed(&document);
        assert_eq!((seed.camera_x, seed.camera_y, seed.camera_zoom), (8.0, -3.0, 2.5));
        let live = Puzzle2dWindowConfig { camera_x: 21.0, ..seed.clone() };
        assert_ne!(live.camera_x, document_seed(&document).camera_x);
    }
}
