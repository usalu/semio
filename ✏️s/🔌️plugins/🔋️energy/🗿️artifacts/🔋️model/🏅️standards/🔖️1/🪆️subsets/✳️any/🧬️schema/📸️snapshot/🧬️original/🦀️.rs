//! 🧬️ Original Energy document fields retain immutable metadata and path-local model authority.
use super::state::EnergyModelState;
use semio_framework_value::shared_issuance::SealedShared;

/// 🔋️ Native Energy document authority contains the sole persistent model and its original composition headers.
#[derive(Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree,framework_schema::ArtifactSchema)]
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

/// 🌳️ The fixed immutable root aliases each of its eleven sealed authorities without traversing a row.
impl semio_framework_value_snapshot_projection::snapshot_projection::ImmutableRootProjection for EnergyModelSnapshot{
 fn root_projection_demand(_:semio_framework_value::RetainedCloneRef<'_,Self>)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of::<Self>(),depth:1,..Default::default()})}
 fn project_root(original:semio_framework_value::RetainedCloneRef<'_,Self>,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Self,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{
  let demand=Self::root_projection_demand(original)?;
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(None);}
  let child=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-demand.copy_bytes,..grant};
  let s=original.get();let m=&s.model;
  macro_rules! alias{($field:expr)=>{$field.try_duplicate(child)?.0}}
  let root=Self{schema:alias!(s.schema),model:EnergyModelState{identity:alias!(m.identity),geometry:alias!(m.geometry),materials:alias!(m.materials),loads:alias!(m.loads),controls:alias!(m.controls),systems:alias!(m.systems),generation:alias!(m.generation),outputs:alias!(m.outputs)},referenced_model:alias!(s.referenced_model),weather_link:alias!(s.weather_link)};
  Ok(Some((root,semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
 }
}


#[path="🌱️genesis/🦀️.rs"]pub mod genesis;

#[path="📦️pack/🦀️.rs"]pub mod pack;


#[path="📜️carrier/🦀️.rs"]mod carrier;
