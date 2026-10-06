use super::*;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::epjson::v25_2::any::encode_model;

fn round_trip(case: &str) -> (String, String) {
    let model = crate::bestest::model(case).expect("bestest case");
    let first = semio_framework_pack_json::to_string_pretty(&encode_model(&model));
    let imported = decode_model(&semio_framework_pack_json::parse(&first, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("parse")).expect("decode");
    let second = semio_framework_pack_json::to_string_pretty(&encode_model(&imported.model));
    (first, second)
}

#[semio_framework_async_macros::async_test]
async fn round_trip_of_every_bestest_case_is_byte_identical() {
    for case in ["600", "600FF", "610", "620", "640", "900", "900FF", "910", "920", "940"] {
        let (first, second) = round_trip(case);
        assert_eq!(first, second, "case {case}: export -> import -> export must be byte-identical");
    }
}

#[semio_framework_async_macros::async_test]
async fn round_trip_preserves_the_counted_entities_of_case_600() {
    let model = crate::bestest::model("600").expect("case 600");
    let imported = decode_model(&encode_model(&model)).expect("decode").model;
    assert_eq!(imported.zones.len(), model.zones.len(), "zones");
    assert_eq!(imported.surfaces.len(), model.surfaces.len(), "surfaces");
    assert_eq!(imported.materials.len(), model.materials.len(), "materials");
    assert_eq!(imported.constructions.len(), model.constructions.len(), "constructions");
    assert_eq!(imported.fenestrations.len(), model.fenestrations.len(), "fenestrations");
    assert_eq!(imported.infiltrations.len(), model.infiltrations.len(), "infiltrations");
    assert_eq!(imported.equipment.len(), model.equipment.len(), "equipment");
    assert_eq!(imported.thermostats.len(), model.thermostats.len(), "thermostats");
    assert_eq!(imported.ideal_loads.len(), model.ideal_loads.len(), "ideal loads");
    assert_eq!(imported.schedules.constants.len(), model.schedules.constants.len(), "constant schedules");
    assert!(imported.validate().is_ok(), "a re-imported case 600 must still validate: {:?}", imported.validate().err());
}

#[semio_framework_async_macros::async_test]
async fn round_trip_preserves_the_ashrae_140_window_and_envelope_numbers() {
    let model = crate::bestest::model("600").expect("case 600");
    let imported = decode_model(&encode_model(&model)).expect("decode").model;
    let window = imported.fenestrations.first().expect("window");
    assert!((window.area_m2 - 6.0).abs() < 1e-9, "aperture area {}", window.area_m2);
    assert!((window.u_value_w_m2k - model.fenestrations[0].u_value_w_m2k).abs() < 1e-12, "u value");
    assert!((window.shgc - model.fenestrations[0].shgc).abs() < 1e-12, "shgc");
    assert!((window.height_m - 2.0).abs() < 1e-9, "aperture height {} (the ring starts at its upper-left corner)", window.height_m);
    assert!((window.sill_height_m - 0.2).abs() < 1e-9, "aperture sill {}", window.sill_height_m);
    let stack = window.glazing_construction_id.and_then(|id| imported.constructions.iter().find(|construction| construction.id == id)).expect("the window stays bound to its layered glazing");
    assert_eq!(stack.name, "Double Clear Glazing");
    let zone = imported.zones.first().expect("zone");
    assert!((zone.volume_m3 - 129.6).abs() < 1e-9, "zone volume {}", zone.volume_m3);
    let infiltration = imported.infiltrations.first().expect("infiltration");
    assert_eq!(infiltration.method, InfiltrationMethod::ScheduledAch);
    assert!((infiltration.design_flow_ach - 0.5).abs() < 1e-12, "ach {}", infiltration.design_flow_ach);
}

/// 💨️ A semio `ScheduledAch` infiltration is a constant design flow; an EnergyPlus document that scales its
/// design flow with temperature or wind says something the model cannot simulate, and the import says so.
#[semio_framework_async_macros::async_test]
async fn infiltration_coefficients_a_constant_design_flow_cannot_carry_are_reported() {
    let model = crate::bestest::model("600").expect("case 600");
    let Value::Object(mut root) = encode_model(&model) else { panic!("object") };
    assert!(!decode_model(&Value::Object(root.clone())).expect("decode").diagnostics.iter().any(|diagnostic| diagnostic.code == "epjson.infiltration.coefficients-dropped"), "the codec's own constant design flow is carried whole");
    let Some(Value::Object(group)) = root.get_mut("ZoneInfiltration:DesignFlowRate") else { panic!("infiltration") };
    for (_, value) in group.iter_mut() {
        let Value::Object(entry) = value else { panic!("object") };
        entry.insert("temperature_term_coefficient", Value::from(0.03));
    }
    let import = decode_model(&Value::Object(root)).expect("decode");
    assert!(import.diagnostics.iter().any(|diagnostic| diagnostic.code == "epjson.infiltration.coefficients-dropped"), "expected a coefficients-dropped diagnostic, got {:?}", import.diagnostics);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_object_type_is_reported_not_merged() {
    let model = crate::bestest::model("600").expect("case 600");
    let Value::Object(mut root) = encode_model(&model) else { panic!("object") };
    root.insert("Boiler:HotWater", Value::Object(Object::from_iter([("Main Boiler".to_string(), Value::Object(Object::new()))])));
    let import = decode_model(&Value::Object(root)).expect("decode");
    assert!(import.diagnostics.iter().any(|diagnostic| diagnostic.code == "epjson.object.unknown-type" && diagnostic.subject == "Boiler:HotWater"), "expected an unknown-type diagnostic, got {:?}", import.diagnostics);
}

#[semio_framework_async_macros::async_test]
async fn a_document_without_a_building_is_refused() {
    assert!(decode_model(&Value::Object(Object::new())).is_err(), "EnergyPlus itself requires a Building object");
}
