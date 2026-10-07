//! 🕸️ Canonical Record identity and direct mutation ownership for actual Graph fields.
use semio_framework_dsl_record::DslField;
use crate::manifest::{PropertyBag,PropertyDef,PropertyKind,PropertyValue,PortDirection};

#[test]
fn graph_fields_implement_the_canonical_record_identity() {
    fn bound<T:DslField>(){}
    let semio_framework_dsl_record::BorrowedShape::Map(inner)=<PropertyBag as semio_framework_dsl_record::BorrowedDslField>::SHAPE else{panic!("borrowed property map")};assert!(matches!(inner(),semio_framework_dsl_record::BorrowedShape::Value));
    bound::<PropertyBag>();bound::<PropertyDef>();bound::<PropertyKind>();bound::<PropertyValue>();bound::<PortDirection>();bound::<Vec<PropertyDef>>();
}

#[test]
fn graph_property_declaration_round_trips_through_the_canonical_record_algebra() {
    let value=PropertyDef{name:"label".into(),kind:PropertyKind::Data,value_type:semio_framework_value::ValueType::Text,expr:None};
    let encoded=<PropertyDef as DslField>::to_value(&value);
    let decoded=<PropertyDef as DslField>::from_value(&encoded).expect("canonical Graph property declaration");
    assert_eq!(value,decoded);
}

#[test]
fn graph_property_delta_implements_the_direct_replication_owner() {
    fn bound<T:protocol::mutation::MapDeltaTarget<PropertyValue>>(){}
    bound::<PropertyBag>();
}
