//! 🪟️ Exact-instance Note composite-window configuration and transient owners.

use crate::editor::note::NOTE_PLAY_WINDOW_COMPOSITE;
use crate::NoteCamera;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, Default, dsl::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.note.note.compositewindowconfig", extension = "notecompositewindowcfg", layout = "lines")]
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
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum NoteCompositeWindowConfigMutation {
    Snapshot { config: NoteCompositeWindowConfig },
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum NoteCompositeWindowTransientMutation {
    Snapshot { transient: NoteCompositeWindowTransient },
}

impl protocol::Mutation<NoteCompositeWindowConfig> for NoteCompositeWindowConfigMutation {
    type Diff = NoteCompositeWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-config",
        display_name: "Set Note Composite Window Configuration",
        emoji: "🪟️",
        aggregate_variant: "Snapshot",
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
    fn diff(&self, _base: &NoteCompositeWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()) }
    }
    fn inverse(&self, base: &NoteCompositeWindowConfig) -> Vec<Self> { vec![Self::Snapshot { config: base.clone() }] }
}

impl protocol::Mutation<NoteCompositeWindowTransient> for NoteCompositeWindowTransientMutation {
    type Diff = NoteCompositeWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-transient",
        display_name: "Set Note Composite Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "note.compositewindowtransient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &NoteCompositeWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()) }
    }
    fn inverse(&self, base: &NoteCompositeWindowTransient) -> Vec<Self> { vec![Self::Snapshot { transient: base.clone() }] }
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

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for NoteCompositeWindowConfig {
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
store::impl_whole_record_config!(NoteCompositeWindowConfig);
json_store!(NoteCompositeWindowTransient, "notecompositewindowtransient", "s.note.note.compositewindowtransient");

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

mutation_wire!(NoteCompositeWindowConfigMutation);
mutation_wire!(NoteCompositeWindowTransientMutation);

impl protocol::MutationDiff<NoteCompositeWindowTransient> for NoteCompositeWindowTransient {
    fn apply(&self, _base: &NoteCompositeWindowTransient) -> protocol::MutationApplyResult<NoteCompositeWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

store::artifact_retire_struct!(NoteCompositeWindowTransient { engagement_input });

impl store::retirement::RetireOwned for NoteCompositeWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn note_composite_window_transient_preflight(mutation: &NoteCompositeWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let NoteCompositeWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = size_of::<NoteCompositeWindowTransient>().checked_add(transient.engagement_input.capacity()).ok_or_else(|| "Note composite window transient footprint overflowed".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn note_composite_window_transient_transfer(mutation: NoteCompositeWindowTransientMutation) -> NoteCompositeWindowTransient {
    match mutation {
        NoteCompositeWindowTransientMutation::Snapshot { transient } => transient,
    }
}

pub struct NoteCompositeWindowConfigOwner;
impl semio_framework_plugin::WindowConfigOwner for NoteCompositeWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = NOTE_PLAY_WINDOW_COMPOSITE;
    const SCHEMA: &'static str = "note.compositewindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = NoteCompositeWindowConfig;
    type Mutation = NoteCompositeWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub struct NoteCompositeWindowTransientOwner;
impl semio_framework_plugin::WindowTransientOwner for NoteCompositeWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = NOTE_PLAY_WINDOW_COMPOSITE;
    type State = NoteCompositeWindowTransient;
    type Mutation = NoteCompositeWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
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
    Ok(semio_framework_plugin::WindowConfigMutation::of::<NoteCompositeWindowConfigOwner>(id, NoteCompositeWindowConfigMutation::Snapshot { config }))
}
pub fn addressed_transient(snapshot: &semio_framework_plugin::WindowTransientSnapshot, transient: NoteCompositeWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    if snapshot.window_kind_id() != NOTE_PLAY_WINDOW_COMPOSITE { return Err(semio_framework_plugin::Fault::from("note-composite-window-kind-required")); }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<NoteCompositeWindowTransientOwner>(snapshot.window_id(), NoteCompositeWindowTransientMutation::Snapshot { transient }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
