//! 🧬️ Original Energy document fields retain immutable metadata and path-local model authority.
use super::state::EnergyModelState;
use semio_framework_value::shared_issuance::SealedShared;

/// 🔋️ Native Energy document authority contains the sole persistent model and its original composition headers.
#[derive(Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree,framework_schema::ArtifactSchema)]
#[artifact_schema(id="s.energy.model")]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all="camelCase")]
pub struct EnergyModelSnapshot{
    #[state(artifact)]
    pub schema:SealedShared<String>,
    #[state(artifact)]
    pub model:EnergyModelState,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    pub referenced_model:SealedShared<Option<store::ArtifactLink>>,
    #[state(artifact)]
    #[link_slot(roles("weather"))]
    pub weather_link:SealedShared<Option<store::ArtifactLink>>,
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🌱️genesis/🦀️.rs"]pub mod genesis;

#[path="📦️pack/🦀️.rs"]pub mod pack;
