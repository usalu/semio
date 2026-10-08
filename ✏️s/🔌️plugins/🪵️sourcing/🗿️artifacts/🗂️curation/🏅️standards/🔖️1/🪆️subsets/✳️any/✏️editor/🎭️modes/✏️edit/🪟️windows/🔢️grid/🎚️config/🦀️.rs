//! 🎚️ Persisted local configuration for one exact sourcing curation Grid window.

use crate::editor::sourcing::modes::edit::windows::grid::SOURCING_CURATION_WINDOW_GRID;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧮️ How the 3D grid expands each curated row's count into world instances.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.sourcing.curation.gridwindowconfig", extension = "sourcinggridwindowcfg")]
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
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Grid window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GridWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1", <Self as store::ArtifactDsl>::envelope_id()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ConfigRecord for GridWindowConfig {}

/// 🔺️ Sparse field delta over [`GridWindowConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct GridWindowConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub instance_display: Option<String>,
}

impl protocol::MutationDiff<GridWindowConfig> for GridWindowConfigDiff {
    fn apply(&self, base: &GridWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GridWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.instance_display {
            next.instance_display = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.instance_display.is_some() {
            self.instance_display = other.instance_display;
        }
    }
}

impl protocol::DiffAlgebra<GridWindowConfig> for GridWindowConfigDiff {
    fn inverse(&self, base: &GridWindowConfig) -> Self {
        Self {
            instance_display: self.instance_display.as_ref().map(|_| base.instance_display.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.instance_display.is_none()
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub enum GridWindowConfigMutation {
    SetInstanceDisplay { instance_display: String },
}

impl protocol::Mutation<GridWindowConfig> for GridWindowConfigMutation {
    type Diff = GridWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/⚙️config",
        semantic_kind: "set-instance-display",
        display_name: "Set Sourcing Grid Instance Display",
        emoji: "🔢️",
        aggregate_variant: "SetInstanceDisplay",
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
    fn diff(&self, base: &GridWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::SetInstanceDisplay { instance_display } => protocol::MutationOutcome::new(GridWindowConfigDiff { instance_display: (base.instance_display != *instance_display).then(|| instance_display.clone()) }),
        }
    }
    fn inverse(&self, base: &GridWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::SetInstanceDisplay { instance_display: base.instance_display.clone() }])
    }
}

impl protocol::OpText for GridWindowConfigMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for GridWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
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

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = GridWindowConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&GridWindowConfigMutation::SetInstanceDisplay { instance_display: GRID_INSTANCE_DISPLAY_REPRESENTATIVE.into() }, &base).await;
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&GridWindowConfigMutation::SetInstanceDisplay { instance_display: GRID_INSTANCE_DISPLAY_LINE_BEHIND.into() }, &base).await;
    }
}
