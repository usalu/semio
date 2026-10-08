use super::*;
use semio_framework_os_kernel::os_directory::schema::DocumentScope;
use semio_framework_os_kernel::os_directory::client::InstalledServiceTurnV1;
use semio_framework_os_kernel::os_dsl::{DslValue,json};

#[test]
fn native_composition_installs_the_real_owner_and_matches_the_portable_oracle() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔌️service-composition/🔣️.json")).unwrap();
    let services=service_contributions_v1();
    assert_eq!(services.len(),fixture["services"].as_array().unwrap().len());
    for (service,row) in services.iter().zip(fixture["services"].as_array().unwrap()) {
        assert_eq!(service.owner,row["owner"].as_str().unwrap());
        assert_eq!(service.service_id,row["serviceId"].as_str().unwrap());
        let mut driver=(service.create)(DocumentScope::new("space-1","document-1"),true);
        let actual:serde_json::Value=serde_json::from_str(&json::to_json_string(&driver.status())).unwrap();
        assert_eq!(actual,fixture["initialStatus"]);
        driver.intend("propose",DslValue::Object(Vec::new())).unwrap();
        let InstalledServiceTurnV1::Call {action,payload}=driver.turn(0) else {panic!("installed owner did not submit")};
        assert_eq!(action,fixture["requestAction"].as_str().unwrap());
        let DslValue::Object(fields)=payload else {panic!("invalid request payload")};
        assert!(fields.iter().any(|(key,value)|key=="serviceId" && *value==DslValue::String(service.service_id.into())));
        assert!(matches!(driver.turn(1),InstalledServiceTurnV1::WaitUntil(_)));
    }
}

#[test]
fn installed_owner_cannot_execute_without_a_verified_document_lease() {
    for service in service_contributions_v1() {
        let mut driver=(service.create)(DocumentScope::new("space-1","document-1"),false);
        driver.intend("propose",DslValue::Object(Vec::new())).unwrap();
        assert!(matches!(driver.turn(0),InstalledServiceTurnV1::Terminal));
        assert!(driver.terminal());
        assert!(matches!(driver.turn(1),InstalledServiceTurnV1::Terminal));
    }
}
