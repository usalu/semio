//! 🏡️ Zone schema paths supply native scalar semantics to the common original row pipeline.
use crate::{snapshot::{original::EnergyModelSnapshot,state::{StateZone,ModelGeometryState}},diff::original::EnergyModelDiff};
pub use semio_s_artifact_energy_model_native_mutations::scalar::{NativeScalar,ScalarOutcome,ScalarRowPath,ScalarRowPreparation};
use semio_framework_value::{RetainedCloneRef,shared_issuance::SealedShared};
use semio_framework_value_persistent::list::persistent::PersistentList;
macro_rules! zone_path{($kind:ident,$value:ty,$field:ident,$valid:expr)=>{
 pub struct $kind;
 impl ScalarRowPath for $kind{
  type Scalar=$value;type Storage=semio_s_artifact_energy_model_native_mutations::scalar::DirectScalar<$value>;type Row=StateZone;type Branch=ModelGeometryState;type Output=semio_s_artifact_energy_model_native_mutations::row::DirectBranchOutput<Self::Branch>;
  fn branch(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{original.project(0,|s|s.model.geometry.get())}
  fn rows(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{original.project(1,|s|&s.model.geometry.get().zones)}
  fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>{&mut branch.zones}
  fn replace(row:&mut Self::Row,value:<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored)-><Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored{::std::mem::replace(&mut row.$field,value)}
  fn read(row:&Self::Row)->Self::Scalar{<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::value(&row.$field)}
  fn valid(value:Self::Scalar)->bool{($valid)(value)}
  fn output()->Self::Output{semio_s_artifact_energy_model_native_mutations::row::DirectBranchOutput::new(|branch|{let mut diff=EnergyModelDiff::default();diff.model.geometry=Some(branch);diff})}
 }
};}
zone_path!(Volume,f64,volume_m3,|v:f64|v.is_finite()&&v>0.0);
zone_path!(Multiplier,u32,multiplier,|v:u32|v>0);
zone_path!(Conditioned,bool,conditioned,|_:bool|true);
zone_path!(FloorArea,bool,part_of_total_floor_area,|_:bool|true);
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
