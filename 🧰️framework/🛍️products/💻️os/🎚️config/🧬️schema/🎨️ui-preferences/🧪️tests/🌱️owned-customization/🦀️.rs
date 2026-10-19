use crate::os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use super::*;
use semio_framework_test_fixtures::fixture_path;
use semio_framework_value::{DslValue,FromValue,ToValue};

#[test]
fn config_owned_customization_semantics_match_independent_json_without_projection_bridge(){
    let fixture:serde_json::Value=serde_json::from_str(std::fs::read_to_string(fixture_path!("../../../🧫️fixtures/🎨️ui-preferences/🌱️owned-customization/🔣️.json")).expect("fixture").as_str()).unwrap();
    for sample in fixture["samples"].as_array().unwrap(){
        let value:DslValue=semio_framework_pack_json::from_json_str(&serde_json::to_string(sample).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let driver=UiDriver{driver_id:fixture["driverId"].as_str().unwrap().into(),label:fixture["label"].as_str().unwrap().into(),config:value.clone()};
        let theme=UiTheme{theme_id:fixture["themeId"].as_str().unwrap().into(),label:fixture["label"].as_str().unwrap().into(),config:value.clone()};
        let layout=UserNamedLayout{label:fixture["label"].as_str().unwrap().into(),layout:value.clone()};
        assert_eq!(UiDriver::from_value(driver.to_value()).unwrap(),driver);
        assert_eq!(UiTheme::from_value(theme.to_value()).unwrap(),theme);
        assert_eq!(UserNamedLayout::from_value(layout.to_value()).unwrap(),layout);
        let mut hostile=driver.to_value();if let DslValue::Object(entries)=&mut hostile{entries.push(("physicalCodec".into(),DslValue::String("foreign".into())));}assert!(UiDriver::from_value(hostile).is_err());
        let(driver_value,theme_value,layout_value)=(driver.to_value(),theme.to_value(),layout.to_value());
        for actual in [driver_value.get("config").unwrap(),theme_value.get("config").unwrap(),layout_value.get("layout").unwrap()]{
            assert_eq!(actual,&value);assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(actual)).unwrap(),*sample);
        }
        let mut preferences=UiPreferences::default();
        preferences.custom_drivers.insert(driver.driver_id.clone(),driver.clone());
        preferences.custom_themes.insert(theme.theme_id.clone(),theme);
        preferences.named_layouts.insert("app".into(),HashMap::from([("layout".into(),layout)]));
        assert_eq!(UiPreferences::from_value(preferences.to_value()).unwrap(),preferences);
        let edit=crate::opening_config::mutations::set_custom_driver(driver.driver_id.clone(),None);
        use protocol::Mutation;
        let inverse=edit.inverse(&preferences).unwrap();let base=preferences.clone();
        apply_ui_preferences_config_mutation(&mut preferences,&edit).unwrap();
        assert!(!preferences.custom_drivers.contains_key(&driver.driver_id));
        for undo in inverse{apply_ui_preferences_config_mutation(&mut preferences,&undo).unwrap();}
        assert_eq!(preferences,base);
    }
    for original in [DslValue::Bytes(vec![0,255]),DslValue::Object(vec![("same".into(),DslValue::uint(1)),("same".into(),DslValue::uint(2))])]{
        let driver=UiDriver{driver_id:"semantic".into(),label:"Owned".into(),config:original.clone()};
        assert_eq!(UiDriver::from_value(driver.to_value()).unwrap().config,original);
    }
}

#[test]
fn config_owned_customization_native_external_facets_read_authored_schema(){
    let driver=UiDriver{driver_id:"native".into(),label:"Owned".into(),config:DslValue::object([("axis".into(),DslValue::String("hover".into()))])};
    let external=serde_json::to_value(&driver).unwrap();assert_eq!(external,serde_json::json!({"driverId":"native","label":"Owned","config":{"axis":"hover"}}));
    assert_eq!(serde_json::from_value::<UiDriver>(external).unwrap(),driver);
    let schema=serde_json::to_value(schemars::schema_for!(UiPreferences)).unwrap();
    assert_eq!(schema["additionalProperties"],false);
    assert_eq!(schema["definitions"]["OsConfigUiUiDriver"]["required"],serde_json::json!(["driverId","label","config"]));
    assert_eq!(schema["definitions"]["OsConfigUiUserNamedLayout"]["properties"]["layout"]["$ref"],"#/definitions/OsConfigUiWindowLayout");
}
