//! 🪟️ Exact-instance Puzzle 3D window configuration and transient interaction owners.

use crate::editor::puzzle3d::config::{Puzzle3dCamera, Puzzle3dConfig, Puzzle3dRuntime, Puzzle3dSelectableKinds, Puzzle3dSuggestionMenu};
use std::collections::BTreeMap;
use crate::editor::puzzle3d::modes::edit::windows::main;
use semio_framework_plugin::WorldSunConfig;

/// 🎚️ ONE exact Puzzle 3D window instance's persisted-local options. `WindowConfigOwner::State`
/// requires `dsl::DslField`, which `#[derive(dsl::DslArtifact)]` emits alongside the `__dsl_*`
/// helpers `ArtifactDsl`/`ArtifactPack` below are written against; `id`/`extension` are stated
/// explicitly so the derived `__DSL_ENVELOPE_ID`/`__DSL_EXTENSION` reproduce the envelope identity
/// this window kind already carried.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.puzzle.puzzle3d.windowconfig", extension = "puzzle3dwindowcfg", layout = "lines")]
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
    pub panel_pages: BTreeMap<String, u32>,
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
            panel_pages: runtime.panel_pages.clone(),
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

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for Puzzle3dWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
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

store::impl_whole_record_config!(Puzzle3dWindowConfig);
json_store!(Puzzle3dWindowTransient, "puzzle3dwindowtransient", "s.puzzle.puzzle3d.windowtransient");
mutation_wire!(Puzzle3dWindowConfigMutation);
mutation_wire!(Puzzle3dWindowTransientMutation);
impl protocol::MutationDiff<Puzzle3dWindowTransient> for Puzzle3dWindowTransient {
    fn apply(&self, _base: &Puzzle3dWindowTransient) -> protocol::MutationApplyResult<Puzzle3dWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

store::artifact_retire_struct!(Puzzle3dSuggestionMenu { x, y, window_id, vortex_full_id });
store::artifact_retire_struct!(Puzzle3dWindowTransient { suggestion_menu, engagement_input, brush_candidate_index, activation });

impl store::retirement::RetireOwned for Puzzle3dWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
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
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
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
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub struct Puzzle3dWindowTransientOwner;
impl semio_framework_plugin::WindowTransientOwner for Puzzle3dWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = main::WINDOW_KIND_ID;
    type State = Puzzle3dWindowTransient;
    type Mutation = Puzzle3dWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
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
        overlap_budget: shared.overlap_budget,
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
        panel_pages: window.panel_pages.clone(),
        active_example_id: shared.active_example_id.clone(),
        suggestion_menu: transient.suggestion_menu.clone(),
        engagement_input: transient.engagement_input.clone(),
        brush_candidate_index: transient.brush_candidate_index,
        active_tool_id: view.and_then(|value| value.active_tool_id.clone()),
        window_ids: view.map_or_else(|| vec![main::WINDOW_KIND_ID.into()], |value| value.window_instances.iter().map(|window| window.id.clone()).collect()),
    }
}

pub fn shared(runtime: &Puzzle3dRuntime) -> Puzzle3dConfig {
    Puzzle3dConfig { fill_count: runtime.fill_count, overlap_budget: runtime.overlap_budget, object_kind_weights: runtime.object_kind_weights.clone(), vortex_kind_weights: runtime.vortex_kind_weights.clone(), active_example_id: runtime.active_example_id.clone() }
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
    semio_framework_plugin::WindowConfigMutation::of::<Puzzle3dWindowConfigOwner>(window_id, Puzzle3dWindowConfigMutation::Snapshot { config })
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
