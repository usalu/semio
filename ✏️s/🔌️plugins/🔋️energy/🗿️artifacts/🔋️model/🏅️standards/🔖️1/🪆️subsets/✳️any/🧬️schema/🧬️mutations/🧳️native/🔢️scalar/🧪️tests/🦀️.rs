use super::*;
#[path="../../../../📸️snapshot/🧳️state/🧪️tests/🧰️helpers/🦀️.rs"]mod helpers;
use helpers::{corpus,receive,source,grant,observe,drain,central_fragment,project_record,denied_axes};
use crate::snapshot::state::*;
macro_rules! path{($name:ident,$row:ident,$branchtype:ident,$branch:ident,$collection:ident,$field:ident)=>{
 struct $name;
 impl ScalarRowPath for $name{
 type Scalar=f64;type Storage=DirectScalar<f64>;type Row=$row;type Branch=$branchtype;type Output=crate::row::DirectBranchOutput<Self::Branch>;
 fn branch(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{source.project(0,|s|s.model.$branch.get())}
 fn rows(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{source.project(1,|s|&s.model.$branch.get().$collection)}
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>{&mut branch.$collection}
 fn replace(row:&mut Self::Row,value:<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored)-><Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored{::std::mem::replace(&mut row.$field,value)}fn read(row:&Self::Row)->Self::Scalar{<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::value(&row.$field)}
 fn valid(value:f64)->bool{value.is_finite()&&value>0.0}
 fn output()->Self::Output{crate::row::DirectBranchOutput::new(|branch|{let mut value=EnergyModelDiff::default();value.model.$branch=Some(branch);value})}
 }
};}
path!(WindowArea,StateFenestration,ModelGeometryState,geometry,fenestrations,area_m2);
path!(GlazingThickness,StateGlazingMaterial,ModelMaterialState,materials,glazing_materials,thickness_m);
path!(Occupancy,StatePeopleGain,ModelLoadState,loads,people,people_per_area);
path!(HeatingThrottle,StateThermostat,ModelControlState,controls,thermostats,heating_throttle_range_k);
path!(OutdoorFlow,StateOutdoorAirSystem,ModelSystemState,systems,outdoor_air_systems,min_oa_flow_m3_s);
path!(BatteryCapacity,StateBatteryAssignment,ModelGenerationState,generation,battery_storage,capacity_kwh);
struct GasType;
impl ScalarRowPath for GasType{
 type Scalar=crate::model::GasKind;type Storage=SharedScalar<Self::Scalar>;type Row=StateGasMaterial;type Branch=ModelMaterialState;type Output=crate::row::DirectBranchOutput<Self::Branch>;
 fn branch(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{source.project(0,|s|s.model.materials.get())}
 fn rows(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{source.project(1,|s|&s.model.materials.get().gas_materials)}
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>{&mut branch.gas_materials}
 fn replace(row:&mut Self::Row,value:<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored)-><Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored{::std::mem::replace(&mut row.gas,value)}fn read(row:&Self::Row)->Self::Scalar{<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::value(&row.gas)}
 fn valid(_:Self::Scalar)->bool{true}
 fn output()->Self::Output{crate::row::DirectBranchOutput::new(|branch|{let mut value=EnergyModelDiff::default();value.model.materials=Some(branch);value})}
}
struct SpaceZone;
impl ScalarRowPath for SpaceZone{
 type Scalar=crate::model::EntityId;type Storage=DirectScalar<Self::Scalar>;type Row=StateSpace;type Branch=ModelGeometryState;type Output=crate::row::DirectBranchOutput<Self::Branch>;
 fn branch(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{source.project(0,|s|s.model.geometry.get())}
 fn rows(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{source.project(1,|s|&s.model.geometry.get().spaces)}
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>{&mut branch.spaces}
 fn replace(row:&mut Self::Row,value:<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored)-><Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored{::std::mem::replace(&mut row.zone_id,value)}fn read(row:&Self::Row)->Self::Scalar{<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::value(&row.zone_id)}
 fn valid(_:Self::Scalar)->bool{true}
 fn reference_collections()->&'static[ReferenceCollection]{&[ReferenceCollection::Zones]}
 fn reference_key(value:RetainedCloneRef<'_,Self::Scalar>)->Result<RetainedCloneRef<'_,u32>,ValueError>{Ok(value.project(0,|v|&v.0))}
 fn output()->Self::Output{crate::row::DirectBranchOutput::new(|branch|{let mut value=EnergyModelDiff::default();value.model.geometry=Some(branch);value})}
}
struct PeopleSchedule;
impl ScalarRowPath for PeopleSchedule{
 type Scalar=crate::schedule::ScheduleId;type Storage=DirectScalar<Self::Scalar>;type Row=StatePeopleGain;type Branch=ModelLoadState;type Output=crate::row::DirectBranchOutput<Self::Branch>;
 fn branch(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{source.project(0,|s|s.model.loads.get())}
 fn rows(source:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{source.project(1,|s|&s.model.loads.get().people)}
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>{&mut branch.people}
 fn replace(row:&mut Self::Row,value:<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored)-><Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored{::std::mem::replace(&mut row.schedule_id,value)}fn read(row:&Self::Row)->Self::Scalar{<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::value(&row.schedule_id)}
 fn valid(_:Self::Scalar)->bool{true}
 fn reference_collections()->&'static[ReferenceCollection]{&[ReferenceCollection::ConstantSchedules,ReferenceCollection::DailySchedules,ReferenceCollection::WeeklySchedules,ReferenceCollection::AnnualSchedules,ReferenceCollection::TimeSeriesSchedules]}
 fn reference_key(value:RetainedCloneRef<'_,Self::Scalar>)->Result<RetainedCloneRef<'_,u32>,ValueError>{Ok(value.project(0,|v|&v.0))}
 fn output()->Self::Output{crate::row::DirectBranchOutput::new(|branch|{let mut value=EnergyModelDiff::default();value.model.loads=Some(branch);value})}
}
#[path="🧰️law/🦀️.rs"]mod scalar_law;use scalar_law::run;

#[test]
fn scalar_paths_across_six_original_branches_preserve_receipts_and_cancel_bounded_wide_rows(){let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let cuts=law["cancelAt"].as_array().unwrap();for case in law["cases"].as_array().unwrap(){if case.get("references").is_some(){continue;}match case["collection"].as_str().unwrap(){"fenestrations"=>run::<WindowArea>(case,cuts),"glazing_materials"=>run::<GlazingThickness>(case,cuts),"people"=>run::<Occupancy>(case,cuts),"thermostats"=>run::<HeatingThrottle>(case,cuts),"outdoor_air_systems"=>run::<OutdoorFlow>(case,cuts),"battery_storage"=>run::<BatteryCapacity>(case,cuts),"gas_materials"=>run::<GasType>(case,cuts),_=>panic!("authored native branch law")}}}

#[test]
fn retained_reference_validation_searches_only_original_foreign_rows_and_every_schedule_kind(){let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let cuts=law["cancelAt"].as_array().unwrap();for case in law["cases"].as_array().unwrap().iter().filter(|v|v.get("references").is_some()){match case["collection"].as_str().unwrap(){"spaces"=>run::<SpaceZone>(case,cuts),"people"=>run::<PeopleSchedule>(case,cuts),_=>panic!("authored original reference law")}}}

struct ConstantValue;
impl ScalarRowPath for ConstantValue{
 type Scalar=f64;type Storage=DirectScalar<f64>;type Row=StateConstantSchedule;type Branch=StateScheduleSet;type Output=crate::row::ScheduleBranchOutput;
 fn branch(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{o.project(0,|s|s.model.outputs.get().schedules.get())}
 fn rows(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{o.project(1,|s|&s.model.outputs.get().schedules.get().constants)}
 fn rows_mut(b:&mut Self::Branch)->&mut PersistentList<Self::Row>{&mut b.constants}
 fn read(r:&Self::Row)->Self::Scalar{<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::value(&r.value)}fn replace(r:&mut Self::Row,value:<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored)-><Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored{::std::mem::replace(&mut r.value,value)}
 fn valid(v:f64)->bool{v.is_finite()}
 fn output()->Self::Output{crate::row::ScheduleBranchOutput::new()}
}
#[test]
fn nested_schedule_row_adopts_only_original_output_branch_and_pays_every_cancelled_parent_prefix(){let law:serde_json::Value=serde_json::from_str(include_str!("📅️schedule.json")).unwrap();for case in law["cases"].as_array().unwrap(){run::<ConstantValue>(case,law["cancelAt"].as_array().unwrap());}}

#[derive(Debug,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
struct ScheduleOperation{id:u32,value:crate::schedule::ScheduleId}
impl protocol::Mutation<EnergyModelSnapshot> for ScheduleOperation{type Diff=EnergyModelDiff;const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[];fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{panic!("fixture exercises original native leaf")}}
impl ScalarLeafSpec<ScheduleOperation,ScheduleOperation> for PeopleSchedule{
 const INVALID:protocol::mutation::retained::RetainedDiagnosticSpec=protocol::mutation::retained::RetainedDiagnosticSpec{level:store::__diagnostic::Severity::Fatal,code:"mutation.invariant",message:"Invalid schedule reference."};
 const INVALID_REFERENCE:protocol::mutation::retained::RetainedDiagnosticSpec=protocol::mutation::retained::RetainedDiagnosticSpec{level:store::__diagnostic::Severity::Error,code:"mutation.target-missing",message:"The referenced schedule does not exist."};
 const MISSING:protocol::mutation::retained::RetainedDiagnosticSpec=Self::INVALID_REFERENCE;
 const UNCHANGED:protocol::mutation::retained::RetainedDiagnosticSpec=protocol::mutation::retained::RetainedDiagnosticSpec{level:store::__diagnostic::Severity::Warning,code:"mutation.no-op",message:"The schedule reference is unchanged."};
 fn input(p:RetainedCloneRef<'_,ScheduleOperation>)->RetainedCloneRef<'_,Self::Scalar>{p.project(0,|p|&p.value)}
 fn key(p:RetainedCloneRef<'_,ScheduleOperation>)->RetainedCloneRef<'_,u32>{p.project(1,|p|&p.id)}
 fn reference_target(p:RetainedCloneRef<'_,ScheduleOperation>)->RetainedCloneRef<'_,u32>{p.project(2,|p|&p.value.0)}
 fn inverse(p:RetainedCloneRef<'_,ScheduleOperation>,value:Self::Scalar)->ScheduleOperation{ScheduleOperation{id:p.get().id,value}}
}
#[test]
fn native_reference_diagnostic_targets_the_original_missing_foreign_identifier(){
 use protocol::mutation::retained::{RetainedMutationLeafPreparation,RetainedMutationAdvance,RetainedMutationDecision,RetainedMutationCloseContext};
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let case=law["cases"].as_array().unwrap().iter().find(|c|c["collection"]=="people"&&c["outcome"]=="invalid-reference").unwrap();let mut value:serde_json::Value=serde_json::from_str(include_str!("../../../../📸️snapshot/🧬️original/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();value=value["document"].take();value["model"]=corpus()["model"].clone();value["model"]["people"]=serde_json::json!([case["row"]]);if let Some(schedules)=case["model"]["schedules"].as_object(){for(kind,rows)in schedules{value["model"]["schedules"][kind]=rows.clone();}}let original=source(receive::<EnergyModelSnapshot>(&value));let payload=source(ScheduleOperation{id:7,value:crate::schedule::ScheduleId(case["value"].as_u64().unwrap()as u32)});let mut owner=ScalarLeafPreparation::<ScheduleOperation,ScheduleOperation,PeopleSchedule>::new();let mut ready=false;for _ in 0..10000{let g=grant(owner.decision_demand(payload.borrow(),original.borrow(),4096).unwrap());let(s,h)=observe(||owner.decide(payload.borrow(),original.borrow(),4096,g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));if matches!(s,RetainedMutationAdvance::Ready(_)){ready=true;break;}}assert!(ready);let RetainedMutationDecision::Refused(messages)=owner.take_decision()else{panic!("missing foreign source must refuse")};assert_eq!(messages[0].target,[case["value"].as_u64().unwrap().to_string()]);let mut messages=protocol::MutationMessageLedgerRetirement::new(String::new(),messages);for _ in 0..10000{if messages.terminal_is_empty(){break;}let d=RetirementDemand{copy_bytes:messages.next_copy_byte_demand(),capacity_bytes:messages.next_capacity_byte_demand(4096).unwrap(),release_bytes:messages.next_release_byte_demand().unwrap(),depth:messages.next_depth_demand().unwrap()};let g=grant(d);let(s,h)=observe(||messages.close_step(g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));}assert!(messages.terminal_is_empty());let factory=semio_framework_value::factory::OwnedValueRetirementFactory::<ScheduleOperation>::default();let context=RetainedMutationCloseContext{mutations:&factory};owner.begin_close();for _ in 0..100000{if owner.terminal_is_empty(){break;}let g=grant(owner.close_demand(context,4096).unwrap());let(s,h)=observe(||owner.close_step(context,g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));}assert!(owner.terminal_is_empty());drain(payload);drain(original);
}
