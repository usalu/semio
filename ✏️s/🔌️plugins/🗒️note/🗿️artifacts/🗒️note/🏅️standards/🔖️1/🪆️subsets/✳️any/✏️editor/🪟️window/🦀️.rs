//! 🪟️ Exact-instance Note composite-window configuration and transient owners.

use crate::editor::note::NOTE_PLAY_WINDOW_COMPOSITE;
use crate::NoteCamera;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, Default, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.note.note.compositewindowconfig", extension = "notecompositewindowcfg")]
pub struct NoteCompositeWindowConfig {
    #[dsl(block)]
    pub camera: NoteCamera,
}


#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct NoteCompositeWindowTransient {
    pub engagement_input: String,
    /// 🛠️ The window's in-flight ink gesture (its open tool transaction), ridden from and back to the window transient
    /// — never config, never history.
    #[value(default)]
    #[cfg_attr(test, serde(skip))]
    pub ink_tool: Option<crate::editor::note::commands::ink_apply_events::NoteInkToolState>,
}

#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "🫧️transient/🦀️.rs"]
mod transient;
pub use diff::{NoteCompositeWindowConfigDiff, NoteCompositeWindowTransientDiff};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum NoteCompositeWindowConfigMutation {
    SetCamera(NoteCamera),
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum NoteCompositeWindowTransientMutation {
    Snapshot { transient: NoteCompositeWindowTransient },
}

impl protocol::Mutation<NoteCompositeWindowConfig> for NoteCompositeWindowConfigMutation {
    type Diff = NoteCompositeWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-camera",
        display_name: "Set Note Composite Window Camera",
        emoji: "🪟️",
        aggregate_variant: "SetCamera",
        payload_schema: "note.compositewindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &NoteCompositeWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::SetCamera(camera) => match &base.camera == camera {
                true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Window camera is unchanged."),
                false => protocol::MutationOutcome::new(NoteCompositeWindowConfigDiff { camera: Some(camera.clone()) }),
            },
        }
    }
    fn inverse(&self, base: &NoteCompositeWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::SetCamera(base.camera.clone())])
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

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for NoteCompositeWindowConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Note composite window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for NoteCompositeWindowConfig {
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
impl store::ConfigRecord for NoteCompositeWindowConfig {}
json_store!(NoteCompositeWindowTransient, "notecompositewindowtransient", "s.note.note.compositewindowtransient");

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

mutation_wire!(NoteCompositeWindowConfigMutation);
mutation_wire!(NoteCompositeWindowTransientMutation);

semio_framework_value::artifact_retire_struct!(NoteCompositeWindowTransient { engagement_input, ink_tool });

impl semio_framework_value::retirement::RetireOwned for NoteCompositeWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn note_composite_window_transient_preflight(mutation: &NoteCompositeWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let NoteCompositeWindowTransientMutation::Snapshot { transient } = mutation;
    let ink_tool = transient.ink_tool.as_ref().map_or(0, |state| semio_framework_pack_json::to_json_string(state).len());
    let retained_bytes = size_of::<NoteCompositeWindowTransient>().checked_add(transient.engagement_input.capacity()).and_then(|bytes| bytes.checked_add(ink_tool)).ok_or_else(|| "Note composite window transient footprint overflowed".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint::for_ephemeral_item(retained_bytes))
}

fn note_composite_window_transient_transfer(mutation: NoteCompositeWindowTransientMutation) -> NoteCompositeWindowTransient {
    match mutation {
        NoteCompositeWindowTransientMutation::Snapshot { transient } => transient,
    }
}

impl semio_framework_plugin::app::WindowConfigApplyMutation<NoteCompositeWindowConfig> for NoteCompositeWindowConfigMutation {
    fn exchange(self, post: &mut NoteCompositeWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetCamera(camera) => Self::SetCamera(std::mem::replace(&mut post.camera, camera)),
        })
    }
}

pub struct NoteCompositeWindowConfigOwner;
impl semio_framework_plugin::WindowConfigOwner for NoteCompositeWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = NOTE_PLAY_WINDOW_COMPOSITE;
    const SCHEMA: &'static str = "note.compositewindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = NoteCompositeWindowConfig;
    type Mutation = NoteCompositeWindowConfigMutation;
    type Edit = semio_framework_plugin::app::WindowConfigApplyEdit<NoteCompositeWindowConfig, NoteCompositeWindowConfigMutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;
    fn build_retained_edit() -> std::sync::Arc<Self::Edit> { std::sync::Arc::new(semio_framework_plugin::app::WindowConfigApplyEdit::new()) }
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub struct NoteCompositeWindowTransientOwner;
impl semio_framework_plugin::WindowTransientOwner for NoteCompositeWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = NOTE_PLAY_WINDOW_COMPOSITE;
    type State = NoteCompositeWindowTransient;
    type Mutation = NoteCompositeWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
            note_composite_window_transient_preflight,
            note_composite_window_transient_transfer,
            state.clone(),
            mutation.clone(),
        ));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> { registry.register::<NoteCompositeWindowConfigOwner>() }
pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> { registry.register::<NoteCompositeWindowTransientOwner>() }

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> NoteCompositeWindowConfig {
    view.window::<NoteCompositeWindowConfigOwner>().cloned().unwrap_or_default()
}
pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> NoteCompositeWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<NoteCompositeWindowConfigOwner>()).cloned().unwrap_or_default()
}
pub fn transient_from_view(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> NoteCompositeWindowTransient {
    view.window::<NoteCompositeWindowTransientOwner>().cloned().unwrap_or_default()
}
pub fn transient_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> NoteCompositeWindowTransient {
    snapshot.and_then(|snapshot| snapshot.get::<NoteCompositeWindowTransientOwner>()).cloned().unwrap_or_default()
}
pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: NoteCompositeWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("note-composite-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str());
    if kind != Some(NOTE_PLAY_WINDOW_COMPOSITE) { return Err(semio_framework_plugin::Fault::from("note-composite-window-kind-required")); }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<NoteCompositeWindowConfigOwner>(id, NoteCompositeWindowConfigMutation::SetCamera(config.camera)))
}
pub fn addressed_transient(snapshot: &semio_framework_plugin::WindowTransientSnapshot, transient: NoteCompositeWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    if snapshot.window_kind_id() != NOTE_PLAY_WINDOW_COMPOSITE { return Err(semio_framework_plugin::Fault::from("note-composite-window-kind-required")); }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<NoteCompositeWindowTransientOwner>(snapshot.window_id(), NoteCompositeWindowTransientMutation::Snapshot { transient }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
