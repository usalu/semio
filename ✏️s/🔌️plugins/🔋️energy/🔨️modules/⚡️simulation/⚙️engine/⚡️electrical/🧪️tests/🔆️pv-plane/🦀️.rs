use super::*;

#[test]
fn pv_kernel_preserves_independent_plane_components_weather_and_authored_efficiency() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔆️pv-plane/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let numbers=|name:&str|row[name].as_array().unwrap().iter().map(|value|value.as_f64().unwrap()).collect::<Vec<_>>();
        let sun=numbers("sun");let collector=numbers("collector");let irradiance=numbers("irradiance");let outdoor=numbers("weather");let pv=numbers("pv");let expected=numbers("expected");
        let thermal=&row["thermal"];let mounting=match thermal["mounting"].as_str().unwrap(){"OpenRack"=>crate::model::PhotovoltaicMounting::OpenRack,"CloseRoof"=>crate::model::PhotovoltaicMounting::CloseRoof,"InsulatedBack"=>crate::model::PhotovoltaicMounting::InsulatedBack,_=>panic!("unknown neutral mounting")};
        let mut model=crate::model::test_model_single_zone();model.site.elevation_m=row["elevation"].as_f64().unwrap();
        model.pv_systems.push(crate::model::PvSystemAssignment{id:EntityId(900),dc_capacity_w:pv[0],area_m2:pv[1],module_efficiency:pv[2],inverter_efficiency:pv[3],tilt_deg:collector[0],azimuth_deg:collector[1], temperature_coefficient:thermal["coefficient"].as_f64().unwrap(),faiman_heat_loss_constant_w_m2k:thermal["constant"].as_f64().unwrap(),faiman_heat_loss_wind_w_s_m3k:thermal["wind"].as_f64().unwrap(),mounting});
        let pre=PrecomputedModel::build(&model,60,60);let mut weather=default_weather(12);weather.direct_normal_irradiance_w_m2=irradiance[0];weather.diffuse_horizontal_irradiance_w_m2=irradiance[1];weather.dry_bulb_c=outdoor[0];weather.wind_speed_m_s=outdoor[1];
        let mut state=SimulationKernel::initialize(&model,&pre,&weather);let mut work=TimestepWork::new(&model,&pre,HourWeather{previous:weather,current:weather,next:weather},SimDate::new(2026,6,21),12.0);work.sun_alt=sun[0];work.sun_az=sun[1];
        let(altitude,azimuth)=(sun[0].to_radians(),sun[1].to_radians());let(tilt,direction)=(collector[0].to_radians(),collector[1].to_radians());
        let sky=SkyState::new([altitude.cos()*azimuth.sin(),altitude.cos()*azimuth.cos(),altitude.sin()],irradiance[0],irradiance[1],model.site.elevation_m,row["reflectance"].as_f64().unwrap());
        let incident=sky.incident([tilt.sin()*direction.sin(),tilt.sin()*direction.cos(),tilt.cos()],1.0,1.0,1.0);
        for(actual,expected)in [incident.beam_w_m2,incident.sky_diffuse_w_m2,incident.ground_diffuse_w_m2].iter().zip(&expected){assert!((actual-expected).abs()<=fixture["tolerance"].as_f64().unwrap(),"{} original solar component {actual} != {expected}",row["id"]);}
        let assigned=&model.pv_systems[0];let system=crate::electrical::PvSystem{dc_capacity_w:assigned.dc_capacity_w,module_efficiency:assigned.module_efficiency,area_m2:assigned.area_m2,inverter_efficiency:assigned.inverter_efficiency,temperature_coefficient:assigned.temperature_coefficient,faiman_heat_loss_constant_w_m2k:assigned.faiman_heat_loss_constant_w_m2k,faiman_heat_loss_wind_w_s_m3k:assigned.faiman_heat_loss_wind_w_s_m3k,mounting:assigned.mounting,tilt_deg:assigned.tilt_deg,azimuth_deg:assigned.azimuth_deg};let cell=system.cell_temperature_c(incident.total_w_m2(),weather.dry_bulb_c,weather.wind_speed_m_s);assert!((cell-expected[3]).abs()<=fixture["tolerance"].as_f64().unwrap(),"{} original collector temperature",row["id"]);
        work.step_pv(&model,&mut state);assert!((work.pv_generation_w-expected[4]).abs()<=fixture["tolerance"].as_f64().unwrap(),"{} original PV power {} != {}",row["id"],work.pv_generation_w,expected[4]);
        work.step_pv(&model,&mut state);assert_eq!(state.delivered_total.pv_generation_w,work.pv_generation_w);
    }
    println!("[DEBUG] thirteen original PV kernel planes preserve direct/diffuse/ground irradiation, wind cooling, nighttime, DC clipping and authored inverter efficiency");
}
