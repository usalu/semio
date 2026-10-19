//! 🧪️ Neutral optical mutation intent reaches the existing central applier without replacing peer fields.
use super::*;
use crate::window_layer_mutations::{change_glazing_material_solar_reflectance_front,change_glazing_material_solar_reflectance_back,change_glazing_material_visible_reflectance_front,change_glazing_material_visible_reflectance_back,change_glazing_material_infrared_transmittance};
use protocol::{ColdMutation, Mutation,SemanticMutation};
use semio_framework_value::ToValue;

#[test]
fn optical_fields_match_neutral_intent_and_preserve_independent_peer_edits(){
 let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🪟️optics/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let pane:crate::model::GlazingMaterial=semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_json_string(&fixture["base"]),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let base=crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model{glazing_materials:vec![pane],..Default::default()}, None);
 for row in fixture["cases"].as_array().unwrap(){
  let id=crate::model::EntityId(row["payload"]["id"].as_u64().unwrap()as u32);let field=row["field"].as_str().unwrap();let value=row["payload"].as_object().unwrap().iter().find(|(key,_)|key.starts_with("new")).unwrap().1.as_f64().unwrap_or(f64::NAN);
  let mutation:EnergyModelMutation=match field{"solar_reflectance_front"=>change_glazing_material_solar_reflectance_front(id,value),"solar_reflectance_back"=>change_glazing_material_solar_reflectance_back(id,value),"visible_reflectance_front"=>change_glazing_material_visible_reflectance_front(id,value),"visible_reflectance_back"=>change_glazing_material_visible_reflectance_back(id,value),"infrared_transmittance"=>change_glazing_material_infrared_transmittance(id,value),_=>panic!("neutral optical field has no mutation")};
  let outcome=mutation.diff(&base);let status=if outcome.worst_level().is_some_and(|level|level>=semio_framework_diagnostic::Severity::Error){"rejected"}else if outcome.messages().iter().any(|message|message.code.0=="mutation.no-op"){"no-op"}else{"applied"};assert_eq!(row["outcome"].as_str(),Some(status));let expected:crate::diff::EnergyModelDiff=semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_json_string(&row["diff"]),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(outcome.diff(),&expected);
  let inverse=mutation.inverse(&base).unwrap();assert_eq!(inverse.len(),row["inverse"].as_array().unwrap().len());
  let after=protocol::apply_diff_cold(outcome.diff(),&base).unwrap();let expected:crate::model::GlazingMaterial=semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_json_string(&row["after"]),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(after.model.glazing_materials[0],expected);
  if let Some(undo)=inverse.first(){let mut peer=after;peer.model.glazing_materials[0].conductivity_w_m_k=fixture["peer"]["value"].as_f64().unwrap();let undone=protocol::apply_diff_cold(undo.diff(&peer).diff(),&peer).unwrap();let mut wanted=base.model.glazing_materials[0].clone();wanted.conductivity_w_m_k=fixture["peer"]["value"].as_f64().unwrap();assert_eq!(undone.model.glazing_materials[0],wanted);let wire=undo.to_value();let expected=semio_framework_pack_json::from_json_str::<EnergyModelMutation>(&semio_framework_pack_json::to_json_string(&row["inverse"][0]),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(wire,expected.to_value());}
 }
 eprintln!("[DEBUG] energy optical40 neutralvectors fields5 centralatomicapplier baseinverse independentpeerpreserved");
}

/// 📡️ The declared optical tags and text terminals reach the shared canonical codecs.
#[test]
fn optical_wire_cases_match_the_neutral_protocol_and_round_trip(){
 use protocol::{OpBinary,OpText};
 let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🪟️optics/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for row in fixture["wireCases"].as_array().unwrap(){
  let operation=EnergyModelMutation::parse_op(row["text"].as_str().unwrap()).unwrap();assert_eq!(operation.kind(),row["kind"].as_str().unwrap());assert_eq!(operation.print_op(),row["text"].as_str().unwrap());
  let bytes=operation.encode_op().unwrap();let prefix:Vec<u8>=row["binaryPrefix"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();assert_eq!(&bytes[..prefix.len()],prefix.as_slice());assert_eq!(EnergyModelMutation::decode_op(&bytes).unwrap(),operation);
  let mut trailing=bytes;trailing.push(0);assert!(EnergyModelMutation::decode_op(&trailing).is_err());assert!(EnergyModelMutation::parse_op(&format!("{} unknown-field=1",row["text"].as_str().unwrap())).is_err());
 }
 eprintln!("[DEBUG] energy optical wire cases5 normative tags300to304 shared text/binary canonical roundtrips");
}

#[test]
fn pv_authored_thermal_mutations_preserve_original_rows_inverse_and_refusal() {
    use protocol::ColdMutation as _;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🧫️fixtures/🔆️pv-plane/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut model=crate::model::test_model_full_topology();let original=model.pv_systems[0].clone();let thermal=&row["thermal"];
        let mounting=match thermal["mounting"].as_str().unwrap(){"OpenRack"=>crate::model::PhotovoltaicMounting::OpenRack,"CloseRoof"=>crate::model::PhotovoltaicMounting::CloseRoof,"InsulatedBack"=>crate::model::PhotovoltaicMounting::InsulatedBack,_=>panic!("unknown neutral mounting")};
        let coefficient=thermal["coefficient"].as_f64().unwrap();let constant=thermal["constant"].as_f64().unwrap();let wind=thermal["wind"].as_f64().unwrap();
        let base=crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA,&model,None);
        let commands:[EnergyModelMutation;4]=[crate::pv_system_mutations::change_pv_system_temperature_coefficient(original.id,coefficient),crate::pv_system_mutations::change_pv_system_faiman_heat_loss_constant(original.id,constant),crate::pv_system_mutations::change_pv_system_faiman_heat_loss_wind(original.id,wind),crate::pv_system_mutations::change_pv_system_mounting(original.id,mounting)];
        let mut current=base.clone();
        for mutation in commands {
            let inverse=mutation.inverse(&current).unwrap();let before=current.clone();current=protocol::apply_diff_cold(mutation.diff(&current).diff(),&current).unwrap();let mut restored=current.clone();
            for undo in inverse {restored=protocol::apply_diff_cold(undo.diff(&restored).diff(),&restored).unwrap();}
            assert_eq!(restored,before);assert!(mutation.inverse(&current).unwrap().is_empty());
        }
        model.pv_systems[0].temperature_coefficient=coefficient;model.pv_systems[0].faiman_heat_loss_constant_w_m2k=constant;model.pv_systems[0].faiman_heat_loss_wind_w_s_m3k=wind;model.pv_systems[0].mounting=mounting;
        assert_eq!(current.model,model);assert_eq!(base.model.pv_systems[0],original);assert!(current.model.validate().is_ok());
        for mutation in <[EnergyModelMutation;6]>::from([crate::pv_system_mutations::change_pv_system_temperature_coefficient(original.id,f64::NAN),crate::pv_system_mutations::change_pv_system_faiman_heat_loss_constant(original.id,0.0),crate::pv_system_mutations::change_pv_system_faiman_heat_loss_constant(original.id,f64::INFINITY),crate::pv_system_mutations::change_pv_system_faiman_heat_loss_wind(original.id,-1.0),crate::pv_system_mutations::change_pv_system_faiman_heat_loss_wind(original.id,f64::NAN),crate::pv_system_mutations::change_pv_system_mounting(EntityId(u32::MAX),mounting)]) {
            assert!(mutation.inverse(&base).unwrap().is_empty());assert_eq!(protocol::apply_diff_cold(mutation.diff(&base).diff(),&base).unwrap(),base);
        }
        let invalid:EnergyModelMutation=crate::pv_system_mutations::create_pv_system(1,EntityId(999),5000.0,25.0,30.0,180.0,0.2,0.96,-0.004,0.0,6.84,mounting);assert!(invalid.inverse(&base).unwrap().is_empty());assert_eq!(protocol::apply_diff_cold(invalid.diff(&base).diff(),&base).unwrap(),base);
        let deleted:EnergyModelMutation=crate::pv_system_mutations::delete_pv_system(original.id);let inverse=deleted.inverse(&current).unwrap();let mut restored=protocol::apply_diff_cold(deleted.diff(&current).diff(),&current).unwrap();for undo in inverse {restored=protocol::apply_diff_cold(undo.diff(&restored).diff(),&restored).unwrap();}assert_eq!(restored,current);
    }
    println!("[DEBUG] original PV thermal intents preserve all13 neutral authored coefficient/mounting cases, sparse inverse, no-op, delete/recreate and physical refusal");
}
