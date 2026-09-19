//! 🎚️ Persisted local configuration for one exact sourcing curation Grid window.

use crate::editor::sourcing::modes::edit::windows::grid::SOURCING_CURATION_WINDOW_GRID;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧮️ How the 3D grid expands each curated row's count into world instances.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.sourcing.curation.gridwindowconfig", extension = "sourcinggridwindowcfg", layout = "lines")]
pub struct GridWindowConfig {
    /// `line-behind` | `representative` | `representative-with-count`
    pub instance_display: String,
}

pub const GRID_INSTANCE_DISPLAY_LINE_BEHIND: &str = "line-behind";
pub const GRID_INSTANCE_DISPLAY_REPRESENTATIVE: &str = "representative";
pub const GRID_INSTANCE_DISPLAY_REPRESENTATIVE_WITH_COUNT: &str = "representative-with-count";

impl Default for GridWindowConfig {
    fn default() -> Self {
        Self { instance_display: GRID_INSTANCE_DISPLAY_LINE_BEHIND.into() }
    }
}

impl store::ArtifactDsl for GridWindowConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Grid window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GridWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1", <Self as store::ArtifactDsl>::envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ConfigRecord for GridWindowConfig {}

impl protocol::MutationDiff<GridWindowConfig> for GridWindowConfig {
    fn apply(&self, _base: &GridWindowConfig) -> protocol::MutationApplyResult<GridWindowConfig> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub enum GridWindowConfigMutation {
    Snapshot { config: GridWindowConfig },
}

impl protocol::Mutation<GridWindowConfig> for GridWindowConfigMutation {
    type Diff = GridWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/⚙️config",
        semantic_kind: "set-grid-window-config",
        display_name: "Set Sourcing Grid Window Configuration",
        emoji: "🔢️",
        aggregate_variant: "Snapshot",
        payload_schema: "s.sourcing.curation.gridwindowconfig",
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
    fn diff(&self, _base: &GridWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()),
        }
    }
    fn inverse(&self, base: &GridWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}

impl protocol::OpText for GridWindowConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for GridWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

pub struct GridWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for GridWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = SOURCING_CURATION_WINDOW_GRID;
    const SCHEMA: &'static str = "s.sourcing.curation.gridwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = GridWindowConfig;
    type Mutation = GridWindowConfigMutation;
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

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<GridWindowConfigOwner>()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> GridWindowConfig {
    snapshot
        .filter(|snapshot| snapshot.window_kind_id() == SOURCING_CURATION_WINDOW_GRID)
        .and_then(|snapshot| snapshot.get::<GridWindowConfigOwner>())
        .cloned()
        .unwrap_or_default()
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> GridWindowConfig {
    from_snapshot(view.window)
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: GridWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("sourcing-grid-window-required"))?;
    let kind = view
        .window_instances
        .iter()
        .find(|window| window.id == id)
        .map(|window| window.window_kind_id.as_str())
        .ok_or_else(|| semio_framework_plugin::Fault::from("sourcing-grid-window-stale"))?;
    if kind != SOURCING_CURATION_WINDOW_GRID {
        return Err(semio_framework_plugin::Fault::from("sourcing-grid-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<GridWindowConfigOwner>(id, mutation))
}

pub fn next_config(_base: &GridWindowConfig, instance_display: &str) -> GridWindowConfig {
    GridWindowConfig { instance_display: instance_display.to_string() }
}
