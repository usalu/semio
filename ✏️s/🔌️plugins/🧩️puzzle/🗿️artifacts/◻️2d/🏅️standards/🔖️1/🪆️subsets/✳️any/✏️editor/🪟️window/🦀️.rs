//! 🪟️ Exact-instance persisted and ephemeral ownership for Puzzle 2D panes.

use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use std::collections::BTreeMap;

/// 🎚️ ONE exact Puzzle 2D pane's persisted-local options. `WindowConfigOwner::State` requires
/// `dsl::DslField`, which `#[derive(dsl::DslArtifact)]` emits alongside the `__dsl_*` helpers the
/// record-backed `ArtifactDsl`/`ArtifactPack` below are written against; `id`/`extension` are stated
/// explicitly so the derived `__DSL_ENVELOPE_ID`/`__DSL_EXTENSION` reproduce the envelope identity
/// the three panes already shared.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.puzzle.puzzle2d.windowconfig", extension = "puzzle2dwindowcfg", layout = "lines")]
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

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for Puzzle2dWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
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
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

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

store::artifact_retire_struct!(Puzzle2dWindowTransient { engagement_input, brush_candidate_index, brush_candidates, brush_candidate_source_handle_id });

impl store::retirement::RetireOwned for Puzzle2dWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
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
    charge(&mut bytes, transient.brush_candidates.capacity().checked_mul(std::mem::size_of::<dsl::DslValue>())?)?;
    let mut pending = Vec::new();
    pending.try_reserve(transient.brush_candidates.len()).ok()?;
    pending.extend(transient.brush_candidates.iter());
    while let Some(value) = pending.pop() {
        match value {
            dsl::DslValue::String(value) => charge(&mut bytes, value.capacity())?,
            dsl::DslValue::Array(values) => {
                charge(&mut bytes, values.capacity().checked_mul(std::mem::size_of::<dsl::DslValue>())?)?;
                pending.try_reserve(values.len()).ok()?;
                pending.extend(values);
            }
            dsl::DslValue::Object(entries) => {
                charge(&mut bytes, entries.capacity().checked_mul(std::mem::size_of::<(String, dsl::DslValue)>())?)?;
                pending.try_reserve(entries.len()).ok()?;
                for (key, value) in entries {
                    charge(&mut bytes, key.capacity())?;
                    pending.push(value);
                }
            }
            dsl::DslValue::Null | dsl::DslValue::Bool(_) | dsl::DslValue::Number(_) => {}
        }
    }
    Some(bytes)
}

fn puzzle2d_window_transient_preflight(mutation: &Puzzle2dWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Puzzle2dWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = puzzle2d_window_transient_retained_bytes(transient).ok_or_else(|| "Puzzle 2D window transient exceeds its exact retained publication envelope".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn puzzle2d_window_transient_transfer(mutation: Puzzle2dWindowTransientMutation) -> Puzzle2dWindowTransient {
    match mutation {
        Puzzle2dWindowTransientMutation::Snapshot { transient } => transient,
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
            fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
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
                let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
