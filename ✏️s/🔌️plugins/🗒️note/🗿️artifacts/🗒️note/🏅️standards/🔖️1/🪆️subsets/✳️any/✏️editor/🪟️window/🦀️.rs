//! 🪟️ Exact-instance Note composite-window configuration and transient owners.

use crate::editor::note::NOTE_PLAY_WINDOW_COMPOSITE;
use crate::NoteCamera;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct NoteCompositeWindowConfig {
    pub camera: NoteCamera,
}

impl Default for NoteCompositeWindowConfig {
    fn default() -> Self {
        Self { camera: NoteCamera::default() }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct NoteCompositeWindowTransient {
    pub engagement_input: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum NoteCompositeWindowConfigMutation {
    Snapshot { config: NoteCompositeWindowConfig },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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

json_store!(NoteCompositeWindowConfig, "notecompositewindowcfg", "s.note.note.compositewindowconfig");
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

pub struct NoteCompositeWindowConfigOwner;
impl semio_framework_plugin::WindowConfigOwner for NoteCompositeWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = NOTE_PLAY_WINDOW_COMPOSITE;
    const SCHEMA: &'static str = "note.compositewindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = NoteCompositeWindowConfig;
    type Mutation = NoteCompositeWindowConfigMutation;
    fn build_store_owners() -> store::MemberStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub struct NoteCompositeWindowTransientOwner;
impl semio_framework_plugin::WindowTransientOwner for NoteCompositeWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = NOTE_PLAY_WINDOW_COMPOSITE;
    type State = NoteCompositeWindowTransient;
    type Mutation = NoteCompositeWindowTransientMutation;
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_transient_preparation_factory::<Self>() }
    fn build_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::State>> { semio_framework_plugin::bounded_window_transient_root_retirement_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_transient_store_disposer::<Self>() }
}

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> { registry.register::<NoteCompositeWindowConfigOwner>() }
pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> { registry.register::<NoteCompositeWindowTransientOwner>() }

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, crate::editor::note::config::NoteConfig>) -> NoteCompositeWindowConfig {
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
mod tests {
    use super::*;

    #[test]
    fn window_schema_round_trips_match_the_serde_json_oracle() {
        let config = NoteCompositeWindowConfig { camera: NoteCamera { x: 3.0, y: -2.0, zoom: 1.5 } };
        let transient = NoteCompositeWindowTransient { engagement_input: "Änderung".into() };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&config)).expect("independent config oracle"), serde_json::json!({"camera":{"x":3.0,"y":-2.0,"zoom":1.5}}));
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&transient)).expect("independent transient oracle"), serde_json::json!({"engagementInput":"Änderung"}));
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&transient);
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCompositeWindowConfigMutation::Snapshot { config });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCompositeWindowTransientMutation::Snapshot { transient });
    }
}
