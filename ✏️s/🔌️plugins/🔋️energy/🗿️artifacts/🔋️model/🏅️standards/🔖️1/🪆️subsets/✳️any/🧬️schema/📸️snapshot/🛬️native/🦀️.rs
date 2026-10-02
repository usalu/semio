//! 🛬️ Explicit Energy native record construction under cumulative input and output controls.
use super::{EnergyModelSnapshot,EnergyModelPackRecord};
use semio_framework_value::{FromValue,ToValue};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
use dsl::{DslField,FieldValue,DslValue,RecordValue,TextError,NativeDecodeControl,NativeEncodeControl};
fn error(message:impl Into<String>)->TextError{TextError::new(message.into(),dsl::TextSpan::at(1,1))}
pub(super)fn rows(snapshot:&EnergyModelSnapshot,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,String>{
 let m=&snapshot.model;let mut count=15usize;let mut visits=0usize;c.check_rows(count)?;c.checkpoint(phase,0,0)?;
 let mut add=|n:usize|->Result<(),String>{count=count.checked_add(n).ok_or("Energy row count overflow")?;c.check_rows(count)?;visits=visits.checked_add(1).ok_or("Energy traversal overflow")?;if visits%256==0{c.checkpoint(phase,visits,0)?;}Ok(())};
 for n in[m.zones.len(),m.spaces.len(),m.surfaces.len(),m.fenestrations.len(),m.materials.len(),m.glazing_materials.len(),m.gas_materials.len(),m.constructions.len(),m.people.len(),m.lighting.len(),m.equipment.len(),m.thermostats.len(),m.humidistats.len(),m.setpoint_managers.len(),m.ideal_loads.len(),m.zone_equipment.len(),m.air_loops.len(),m.plant_loops.len(),m.outdoor_air_systems.len(),m.infiltrations.len(),m.mechanical_ventilations.len(),m.shading_surfaces.len(),m.space_lists.len(),m.thermal_enclosures.len(),m.adjacency_pairs.len(),m.electrical_load_centers.len(),m.pv_systems.len(),m.battery_storage.len(),m.shw_systems.len(),m.solar_thermal_systems.len(),m.refrigeration_systems.len(),m.water_systems.len(),m.faults.len(),m.output_variables.len(),m.sizing_objects.len(),m.daylight_zones.len(),m.room_air_models.len(),m.schedules.constants.len(),m.schedules.daily.len(),m.schedules.weekly.len(),m.schedules.annual.len(),m.schedules.time_series.len()]{add(n)?;}
 if snapshot.referenced_model.is_some(){add(1)?;}if snapshot.weather_link.is_some(){add(1)?;}
 for v in &m.surfaces{add(v.vertices_m.len())?;}for v in &m.fenestrations{add(v.vertices_m.len())?;}for v in &m.constructions{add(v.layer_material_ids.len())?;}for v in &m.setpoint_managers{if matches!(v.kind,crate::model::SetpointManagerKind::OutdoorAirReset{..}){add(1)?;}}
 for v in &m.air_loops{add(v.terminal_zone_ids.len())?;}for v in &m.plant_loops{add(v.equipment_ids.len())?;}for v in &m.shading_surfaces{add(v.vertices_m.len())?;}for v in &m.space_lists{add(v.space_ids.len())?;}for v in &m.thermal_enclosures{add(v.zone_ids.len())?;}
 if let Some(v)=&m.airflow_network{add(1)?;add(v.zone_node_ids.len())?;add(v.link_ids.len())?;}
 for v in &m.electrical_load_centers{add(v.generator_ids.len())?;add(v.pv_ids.len())?;add(v.battery_ids.len())?;}
 for v in &m.schedules.daily{add(24)?;if v.limits.is_some(){add(1)?;}}for _ in &m.schedules.weekly{add(7)?;}for v in &m.schedules.annual{add(v.rules.len())?;add(v.holiday_dates.len())?;}for v in &m.schedules.time_series{add(v.values.len())?;}
 c.checkpoint(phase,0,count)?;Ok(count)
}
fn borrowed<'a>(v:&'a DslValue,name:&str,c:&mut NativeDecodeControl<'_>)->Result<&'a DslValue,String>{let DslValue::Object(fields)=v else{return Err("Energy native model must be a literal object".into())};let mut found=None;for(key,value)in fields{c.step()?;if key==name{if found.is_some(){return Err("Energy native model has duplicate fields".into())}found=Some(value)}}found.ok_or_else(||"Energy native model is missing a persisted field".into())}
fn items(v:&DslValue)->Result<&[DslValue],String>{match v{DslValue::Array(items)=>Ok(items),_=>Err("Energy native collection shape differs".into())}}
fn admit_record(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<(),String>{
 c.scoped_stage(|c|{c.begin_stage(0)?;let mut rows=15usize;let mut add=|n:usize|->Result<(),String>{rows=rows.checked_add(n).filter(|n|*n<=maximum).ok_or("Energy native rows exceed caller limit")?;Ok(())};add(0)?;
 for id in[4,5]{if record.get(id).is_some_and(|value|!matches!(value,FieldValue::Absent)){add(1)?;}}
 let Some(FieldValue::Value(model))=record.get(1)else{return Err("Energy native model field differs".into())};
 for name in["zones","spaces","surfaces","fenestrations","materials","glazing_materials","gas_materials","constructions","people","lighting","equipment","thermostats","humidistats","setpoint_managers","ideal_loads","zone_equipment","air_loops","plant_loops","outdoor_air_systems","infiltrations","mechanical_ventilations","shading_surfaces","space_lists","thermal_enclosures","adjacency_pairs","electrical_load_centers","pv_systems","battery_storage","shw_systems","solar_thermal_systems","refrigeration_systems","water_systems","faults","output_variables","sizing_objects","daylight_zones","room_air_models"]{add(items(borrowed(model,name,c)?)?.len())?;}
 for(parent,child)in[("surfaces","vertices_m"),("fenestrations","vertices_m"),("constructions","layer_material_ids"),("air_loops","terminal_zone_ids"),("plant_loops","equipment_ids"),("shading_surfaces","vertices_m"),("space_lists","space_ids"),("thermal_enclosures","zone_ids")]{for v in items(borrowed(model,parent,c)?)?{add(items(borrowed(v,child,c)?)?.len())?;}}
 for v in items(borrowed(model,"setpoint_managers",c)?)?{if let DslValue::Object(fields)=borrowed(v,"kind",c)?{if fields.iter().any(|(key,_)|key=="OutdoorAirReset"){add(1)?;}}}
 let network=borrowed(model,"airflow_network",c)?;if !matches!(network,DslValue::Null){add(1)?;add(items(borrowed(network,"zone_node_ids",c)?)?.len())?;add(items(borrowed(network,"link_ids",c)?)?.len())?;}
 for v in items(borrowed(model,"electrical_load_centers",c)?)?{for name in["generator_ids","pv_ids","battery_ids"]{add(items(borrowed(v,name,c)?)?.len())?;}}
 let schedules=borrowed(model,"schedules",c)?;for name in["constants","daily","weekly","annual","time_series"]{add(items(borrowed(schedules,name,c)?)?.len())?;}
 for v in items(borrowed(schedules,"daily",c)?)?{add(items(borrowed(v,"hourly_values",c)?)?.len())?;if !matches!(borrowed(v,"limits",c)?,DslValue::Null){add(1)?;}}
 for v in items(borrowed(schedules,"weekly",c)?)?{add(items(borrowed(v,"daily_schedule_ids",c)?)?.len())?;}
 for v in items(borrowed(schedules,"annual",c)?)?{add(items(borrowed(v,"rules",c)?)?.len())?;add(items(borrowed(v,"holiday_dates",c)?)?.len())?;}
 for v in items(borrowed(schedules,"time_series",c)?)?{add(items(borrowed(v,"values",c)?)?.len())?;}c.checkpoint()
 })
}
fn construct(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<EnergyModelSnapshot,TextError>{
 admit_record(record,c,maximum).map_err(error)?;let record=EnergyModelPackRecord::__dsl_from_record_controlled(record,c)?;let model=crate::model::Model::from_value_controlled(&record.model,c).map_err(|e|error(e.to_string()))?;
 Ok(EnergyModelSnapshot{schema:record.schema,model,structure:record.structure,zones:record.zones,referenced_model:record.referenced_model,weather_link:record.weather_link})
}
pub(super)fn decode(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<EnergyModelSnapshot,String>{let maximum=c.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<EnergyModelSnapshot as store::ArtifactDsl>::envelope_id(),EnergyModelPackRecord::__dsl_spec_producer(),|record,native|construct(record,native,maximum),c)}
fn field<T:DslField>(value:&T,c:&mut NativeEncodeControl<'_>)->Result<FieldValue,String>{c.scoped_stage(|c|{c.begin_stage(0)?;value.to_value_controlled(c)})}
fn optional<T:DslField>(value:&Option<T>,c:&mut NativeEncodeControl<'_>)->Result<FieldValue,String>{match value{Some(value)=>field(value,c),None=>{c.checkpoint()?;Ok(FieldValue::Absent)}}}
fn project(snapshot:&EnergyModelSnapshot,c:&mut NativeEncodeControl<'_>)->Result<RecordValue,TextError>{
 c.scoped_stage(|c|->Result<RecordValue,String>{c.begin_stage(6)?;let mut out=dsl::native_encoding::EncodedRecord::new(6,c)?;
 out.insert(0,field(&snapshot.schema,c)?);c.step()?;
 let model=c.scoped_stage(|c|{c.begin_stage(0)?;snapshot.model.to_value_controlled(c).map_err(|e|e.to_string())})?;out.insert(1,FieldValue::Value(model));c.step()?;
 out.insert(2,field(&snapshot.structure,c)?);c.step()?;out.insert(3,field(&snapshot.zones,c)?);c.step()?;out.insert(4,optional(&snapshot.referenced_model,c)?);c.step()?;out.insert(5,optional(&snapshot.weather_link,c)?);c.step()?;Ok(out.take())
 }).map_err(error)
}
pub(super)fn encode(snapshot:&EnergyModelSnapshot,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{rows(snapshot,c,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<EnergyModelSnapshot as store::ArtifactDsl>::envelope_id(),EnergyModelPackRecord::__dsl_spec_producer(),|native|project(snapshot,native),c)}
