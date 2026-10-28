//! 🧪️ Native retained declaration and original overlay contracts, declared before implementation.
use super::metadata;

fn expression(actual: &metadata::StaticTypeExpression, expected: &serde_json::Value) {
    assert_eq!(actual.kind, expected["kind"].as_str().unwrap());
    assert_eq!(actual.name, expected["name"].as_str());
    assert_eq!(actual.width, expected["width"].as_i64());
    assert_eq!(actual.fixed, expected["fixed"].as_bool().unwrap_or(false));
    assert_eq!(actual.aggregate, expected["aggregate"].as_str());
    assert_eq!(actual.lower, expected["lower"].as_i64());
    assert_eq!(actual.upper, expected["upper"].as_i64());
    assert_eq!(actual.lower_expression, expected["lowerExpression"].as_str());
    assert_eq!(actual.upper_expression, expected["upperExpression"].as_str());
    assert_eq!(actual.unique, expected["unique"].as_bool().unwrap_or(false));
    assert_eq!(actual.optional, expected["optional"].as_bool().unwrap_or(false));
    if let Some(item) = actual.item {
        expression(item, &expected["item"]);
    } else {
        assert!(expected["item"].is_null());
    }
}

#[test]
fn static_declarations_preserve_every_released_serialization_position() {
    let released: serde_json::Value = serde_json::from_str(include_str!("../📚️catalog/🔣️.json")).unwrap();
    assert_eq!(metadata::ENTITIES.len(), released["entities"].as_object().unwrap().len());
    assert_eq!(metadata::TYPES.len(), released["types"].as_object().unwrap().len());
    for (key, expected) in released["entities"].as_object().unwrap() {
        let actual = metadata::entity(key).unwrap();
        assert_eq!(actual.name, expected["name"].as_str().unwrap());
        assert_eq!(actual.supertype, expected["supertype"].as_str());
        assert_eq!(actual.abstract_entity, expected["abstractEntity"].as_bool().unwrap());
        for (actual, expected) in actual.all_attributes.iter().zip(expected["allAttributes"].as_array().unwrap()) {
            assert_eq!(actual.name, expected["name"].as_str().unwrap());
            assert_eq!(actual.optional, expected["optional"].as_bool().unwrap());
            assert_eq!(actual.derived, expected["derived"].as_bool().unwrap());
            assert_eq!(actual.declared_by, expected["declaredBy"].as_str().unwrap());
            expression(&actual.type_expr, &expected["typeExpr"]);
        }
        assert_eq!(actual.all_attributes.len(), expected["allAttributes"].as_array().unwrap().len());
    }
    for (key, expected) in released["types"].as_object().unwrap() {
        let actual = metadata::defined_type(key).unwrap();
        assert_eq!(actual.name, expected["name"].as_str().unwrap());
        assert_eq!(actual.kind, expected["kind"].as_str().unwrap());
        assert_eq!(actual.members, expected["members"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>());
        if let Some(value) = actual.type_expr {
            expression(value, &expected["typeExpr"]);
        } else {
            assert!(expected["typeExpr"].is_null());
        }
    }
    assert!(metadata::is_subtype("IfcWall", "IFCROOT"));
    assert!(metadata::entity("IFCSIUNIT").unwrap().all_attributes[0].derived);
    assert!(metadata::entity(&"X".repeat(129)).is_none());
}

#[test]
fn retained_contract_never_advertises_unexecuted_native_admission() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("retained-cases.json")).unwrap();
    assert_eq!(contract["result"]["nativeExecution"], false);
    assert_eq!(contract["limits"]["sourceClone"], false);
    assert_eq!(contract["limits"]["coldEvaluation"], false);
}

#[test]
fn static_lookup_has_no_request_time_heap_birth() {
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        assert!(metadata::is_subtype("IFCWALL", "IFCROOT"));
        assert_eq!(metadata::defined_type("IFCPOSITIVELENGTHMEASURE").unwrap().type_expr.unwrap().name, Some("IFCLENGTHMEASURE"));
        assert_eq!(metadata::entity("IFCSIUNIT").unwrap().all_attributes.len(), 4);
    });
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}
