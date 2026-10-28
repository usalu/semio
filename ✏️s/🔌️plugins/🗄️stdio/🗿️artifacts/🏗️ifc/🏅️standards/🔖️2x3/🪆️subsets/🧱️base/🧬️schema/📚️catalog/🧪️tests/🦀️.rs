use super::*;
#[semio_framework_async_macros::async_test]
async fn normative_ifc2x3_catalog_preserves_its_own_domains() {
    let vectors = semio_framework_pack_json::parse(include_str!("🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let catalog = ifc2x3_catalog();
    assert_eq!(catalog.schema, "IFC2X3");
    assert_eq!(catalog.entities.len() as u64, vectors["counts"]["entities"].as_u64().unwrap());
    assert_eq!(catalog.types.len() as u64, vectors["counts"]["types"].as_u64().unwrap());
    assert_eq!(catalog.functions.len() as u64, vectors["counts"]["functions"].as_u64().unwrap());
    assert_eq!(catalog.rules.len() as u64, vectors["counts"]["rules"].as_u64().unwrap());
    for row in vectors["subtypes"].as_array().unwrap() {
        assert_eq!(ifc2x3_is_subtype(row[0].as_str().unwrap(), row[1].as_str().unwrap()), row[2].as_bool().unwrap());
    }
    for row in vectors["attributes"].as_array().unwrap() {
        let attribute = ifc2x3_attribute(row["entity"].as_str().unwrap(), row["index"].as_u64().unwrap() as usize).unwrap();
        assert_eq!(attribute.name, row["name"].as_str().unwrap());
        assert_eq!(attribute.optional, row["optional"].as_bool().unwrap());
        assert_eq!(attribute.derived, row["derived"].as_bool().unwrap());
    }
    assert_eq!(ifc2x3_attribute_declarations("IfcWall").unwrap().len(), 8);
    assert!(!ifc2x3_is_subtype("IfcSpatialZone", "IfcRoot"));
}
